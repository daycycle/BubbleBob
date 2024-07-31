// Copyright 2024 David-Alexandre Yana
// Licensed under the terms in LICENSE.txt

// Pressures
pub const ATMOSPHERIC_PRESSURE: f64 = 1.0;
pub const WATER_PRESSURE_PER_10_METERS: f64 = 10.0;

// Air composition
pub const DINITROGEN_IN_AIR: f64 = 0.78;
pub const DIOXYGEN_IN_AIR: f64 = 0.21;
pub const HELIUM_IN_AIR: f64 = 0.000005;

// Gas limits
const LIMITS: [crate::Limit; 4] = [
    // Dioxygen partial pressure to sustain life
    crate::Limit {
        gas: crate::Gas::Dioxygen,
        extremum: crate::Extremum::Minimum,
        pressure: crate::Pressure::Partial(0.16),
    },

    // Central nervous system oxygen toxicity
    crate::Limit {
        gas: crate::Gas::Dioxygen,
        extremum: crate::Extremum::Maximum,
        pressure: crate::Pressure::Partial(1.3),
    },

    // Nitrogen narcosis
    crate::Limit {
        gas: crate::Gas::Dinitrogen,
        extremum: crate::Extremum::Maximum,
        pressure: crate::Pressure::Partial(3.12),
    },

    // Absolute pressure component of the high-pressure nervous syndrome (HPNS)
    crate::Limit {
        gas: crate::Gas::Helium,
        extremum: crate::Extremum::Maximum,
        pressure: crate::Pressure::Absolute(31.0),
    }
];

#[derive(serde::Serialize)]
pub enum LimitsError {
    NoMinimumDioxygenLimit,
    NoMaximumDiogygenLimit,
}

pub fn get_limits() -> Result<[crate::Limit; LIMITS.len()], crate::Error> {
    let mut minimum_dioxygen_limit_exists = false;
    let mut maximum_dioxygen_limit_exists = false;
    for limit in LIMITS {
        if limit.gas == crate::Gas::Dioxygen {
            if limit.extremum == crate::Extremum::Minimum {
                minimum_dioxygen_limit_exists = true;
            } else if limit.extremum == crate::Extremum::Maximum {
                maximum_dioxygen_limit_exists = true;
            }
        }
    }
    if minimum_dioxygen_limit_exists == false {
        return Err(crate::Error::LimitsError(LimitsError::NoMinimumDioxygenLimit));
    }
    if maximum_dioxygen_limit_exists == false {
        return Err(crate::Error::LimitsError(LimitsError::NoMaximumDiogygenLimit));
    }
    Ok(LIMITS)
}
