// Copyright 2024 David-Alexandre Yana
// Licensed under the terms in LICENSE.txt

use crate::gas_mix::{self, GasMix};
use crate::profile::{DINITROGEN_IN_AIR, DIOXYGEN_IN_AIR, HELIUM_IN_AIR};

#[derive(serde::Deserialize)]
pub enum GasMixer {
    Air,
    Heliox { dioxygen: f64 },
    Nitrox { dioxygen: f64 },
    Trimix { dinitrogen: f64, dioxygen: f64, helium: f64 },
}

#[derive(serde::Serialize)]
pub enum GasMixerError {
    NegativeProportion,
    ProportionAboveMaximum,
}

impl GasMixer {
    fn check_proportions(&self) -> Result<(), crate::Error> {
        match self {
            GasMixer::Air => {},
            GasMixer::Heliox { dioxygen } => is_proportion_valid(*dioxygen)?,
            GasMixer::Nitrox { dioxygen } => is_proportion_valid(*dioxygen)?,
            GasMixer::Trimix { dinitrogen, dioxygen, helium } => {
                is_proportion_valid(*dinitrogen)?;
                is_proportion_valid(*dioxygen)?;
                is_proportion_valid(*helium)?;
            },
        }
        Ok(())
    }
}

fn is_proportion_valid(proportion: f64) -> Result<(), crate::Error> {
    if proportion < 0.0 {
        Err(crate::Error::GasMixerError(GasMixerError::NegativeProportion))
    } else if proportion > 1.0 {
        Err(crate::Error::GasMixerError(GasMixerError::ProportionAboveMaximum))
    } else {
        Ok(())
    }
}

impl GasMixer {
    pub fn make_gas_mix(&self) -> Result<GasMix, crate::Error> {
        self.check_proportions()?;
        match self {
            GasMixer::Air => gas_mix::new(
                DINITROGEN_IN_AIR,
                DIOXYGEN_IN_AIR,
                HELIUM_IN_AIR,
            ),
            GasMixer::Heliox { dioxygen } => gas_mix::new(
                0.0,
                *dioxygen,
                1.0 - dioxygen,
            ),
            GasMixer::Nitrox { dioxygen } => gas_mix::new(
                air_gas_fraction_in_nitrox_mix(DINITROGEN_IN_AIR, *dioxygen),
                *dioxygen,
                air_gas_fraction_in_nitrox_mix(HELIUM_IN_AIR, *dioxygen),
            ),
            GasMixer::Trimix { dinitrogen, dioxygen, helium } => gas_mix::new(
                *dinitrogen,
                *dioxygen,
                *helium,
            ),
        }

    }
}

fn air_gas_fraction_in_nitrox_mix(gas_fraction_in_air: f64, dioxygen_fraction_in_nitrox_mix: f64) -> f64 {
    gas_fraction_in_air * (dioxygen_fraction_in_nitrox_mix - 1.0) / (DIOXYGEN_IN_AIR - 1.0)
}
