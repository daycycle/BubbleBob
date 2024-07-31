// Copyright 2024 David-Alexandre Yana
// Licensed under the terms in LICENSE.txt

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gas_mix;
mod gas_mixer;
mod profile;

use gas_mixer::GasMixer;

const METER_TO_FEET: f64 = 3.280839895;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_operating_depth, exit])
        .run(tauri::generate_context!())
        .expect("error while running application");
}

#[derive(serde::Deserialize)]
enum UnitsSystem {
    Metric,
    Imperial,
}

#[derive(Eq, PartialEq)]
enum Gas {
    Dinitrogen,
    Dioxygen,
    Helium,
}

#[derive(Eq, PartialEq)]
enum Extremum {
    Minimum,
    Maximum,
}

struct Limit {
    gas: Gas,
    extremum: Extremum,
    pressure: Pressure,
}

enum Pressure {
    Absolute(f64),
    Partial(f64),
}

#[derive(serde::Serialize)]
struct OperatingDepthString {
    minimum: String,
    maximum: String,
}

#[derive(serde::Serialize)]
enum Error {
    GasMixError(gas_mix::GasMixError),
    GasMixerError(gas_mixer::GasMixerError),
    OperatingDepthError(gas_mix::OperatingDepthError),
    LimitsError(profile::LimitsError),
}

#[tauri::command]
fn get_operating_depth(mixer: GasMixer, units_system: UnitsSystem) -> Result<OperatingDepthString, Error> {
    let operating_depth = &mixer.make_gas_mix()?.operating_depth(&units_system)?;
    Ok(OperatingDepthString {
        minimum: operating_depth.minimum.to_string(),
        maximum: operating_depth.maximum.to_string(),
    })
}

#[tauri::command]
fn exit(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
}
