// Copyright 2024 David-Alexandre Yana
// Licensed under the terms in LICENSE.txt

use crate::profile::{ATMOSPHERIC_PRESSURE, WATER_PRESSURE_PER_10_METERS};

pub struct GasMix {
    dinitrogen: f64,
    dioxygen: f64,
    helium: f64,
}

#[derive(Clone)]
pub struct OperatingDepth {
    pub minimum: f64,
    pub maximum: f64,
}

#[derive(serde::Serialize)]
pub enum GasMixError {
    GasMixProportionsNotValid,
    NoDioxygenInGasMix,
}

#[derive(serde::Serialize)]
pub enum OperatingDepthError {
    NoMinimumOperatingPressure,
    NoMaximumOperatingPressure,
    NoValidRange,
}

pub fn new(dinitrogen: f64, dioxygen: f64, helium: f64) -> Result<GasMix, crate::Error> {
    let gas_mix = GasMix {
        dinitrogen: dinitrogen,
        dioxygen: dioxygen,
        helium: helium,
    };

    // Check if the gas mix proportions are valid and if the gas mix contains dioxygen.
    if gas_mix.are_proportions_valid() == false {
        return Err(crate::Error::GasMixError(GasMixError::GasMixProportionsNotValid));
    }
    if dioxygen == 0.0 {
        return Err(crate::Error::GasMixError(GasMixError::NoDioxygenInGasMix))
    }
    return Ok(gas_mix);
}

// Checks if the sum of the shares of the gas mix components is close to 100%.
impl GasMix {
    fn are_proportions_valid(&self) -> bool {
        let mut sum: f64 = 0.0;
        sum += self.dinitrogen;
        sum += self.dioxygen;
        sum += self.helium;
        if sum >= 0.98 && sum <= 1.0 {
            return true;
        } else {
            return false;
        }
    }
}

// The pressure_limit function returns a Result containing an Option containing
// the minimum or maximum pressure in bar at which a GasMix can be used
// according to the limits in the LIMITS constant in profile.rs.
// If there is no limit with the same extremum as the one passed to the
// function, the None variant of the enum will be returned.
impl GasMix {
    fn pressure_limit(&self, extremum: crate::Extremum) -> Result<Option<f64>, crate::Error> {
        let mut combined_depth_limit: Option<f64> = None;
        
        for limit in crate::profile::get_limits()? {
            if limit.extremum != extremum {
                continue;
            }
            let limit_gas_percentage_in_gas_mix: f64 = {
                match limit.gas {
                    crate::Gas::Dinitrogen => self.dinitrogen,
                    crate::Gas::Dioxygen => self.dioxygen,
                    crate::Gas::Helium => self.helium,
                }
            };
            if limit_gas_percentage_in_gas_mix == 0.0 {
                continue;
            }
            let depth_limit = {
                match limit.pressure {
                    crate::Pressure::Absolute(absolute) => absolute,
                    crate::Pressure::Partial(partial) => partial / limit_gas_percentage_in_gas_mix,
                }
            };
            if combined_depth_limit.is_some() {
                match extremum {
                    crate::Extremum::Minimum => {
                        if depth_limit > combined_depth_limit.unwrap() {
                            combined_depth_limit = Some(depth_limit);
                        }
                    },
                    crate::Extremum::Maximum => {
                        if depth_limit < combined_depth_limit.unwrap() {
                            combined_depth_limit = Some(depth_limit);
                        }
                    },
                }
            } else {
                combined_depth_limit = Some(depth_limit);
            }
        }
        return Ok(combined_depth_limit);
    }
}

// The operating_depth function returns the underwater operating depth range
// for the gas mix in the provided units system.
// It also checks that:
// - there are minimum and maximum limits (as the gas mix contains dioxygen,
//   there should at least be a minimum and a maximum limit for dioxygen).
// - the maximum depth is bellow the surface,
// - the maximum depth is bellow the minimum depth.
// The minimum and maximum depth values are rounded up or down (respectively)
// to one decimal place.
impl GasMix {
    pub fn operating_depth(&self, units_system: &crate::UnitsSystem) -> Result<OperatingDepth, crate::Error> {
        let minimum_pressure = self.pressure_limit(crate::Extremum::Minimum)?;
        let maximum_pressure = self.pressure_limit(crate::Extremum::Maximum)?;

        // Check if the gas mix has a minimum and maximum operating pressures.
        if minimum_pressure == None {
            return Err(crate::Error::OperatingDepthError(OperatingDepthError::NoMinimumOperatingPressure));
        }
        if maximum_pressure == None {
            return Err(crate::Error::OperatingDepthError(OperatingDepthError::NoMaximumOperatingPressure));
        }

        let mut minimum_depth: f64;
        let maximum_depth: f64;
        minimum_depth = (to_depth(minimum_pressure.unwrap(), &units_system) * 10.0 + 1.0 ).trunc() / 10.0;
        if minimum_depth < 0.0 {
            minimum_depth = 0.0;
        }
        maximum_depth = (to_depth(maximum_pressure.unwrap(), &units_system) * 10.0).trunc() / 10.0;

        // Check if maximum depth is bellow the surface and superior to minimum depth.
        if maximum_depth <= 0.0 {
            return Err(crate::Error::OperatingDepthError(OperatingDepthError::NoValidRange));
        }
        if minimum_depth >= maximum_depth {
            return Err(crate::Error::OperatingDepthError(OperatingDepthError::NoValidRange));
        }

        Ok(OperatingDepth {
            minimum: minimum_depth,
            maximum: maximum_depth,
        })
    }
}

fn to_depth(pressure: f64, units_system: &crate::UnitsSystem) -> f64 {
    let depth = (pressure - ATMOSPHERIC_PRESSURE) * WATER_PRESSURE_PER_10_METERS;
    match units_system {
        crate::UnitsSystem::Metric => depth,
        crate::UnitsSystem::Imperial => depth * crate::METER_TO_FEET,
    }
}
