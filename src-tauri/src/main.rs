#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use emsdesign::crs::CrossSection;
use emsdesign::crs::{CrsLib, PRESETS};
use emsdesign::erc::NSEN_1993::BuckleCurve;
use emsdesign::mat::steel::{Steel, Variant};
use emsdesign::mat::Material;
use emsdesign::mmb::columnbeam::ColumnBeam;
use emsdesign::{Axis, LimitStateType};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_area,
            get_section_names,
            get_capacity,
            get_steel_variants,
            get_material_properties,
            get_buckle_curve
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_area(crstype: &str, name: &str) -> Result<Value, String> {
    let preset = PRESETS::get(crstype).ok_or_else(|| "Could not get preset".to_owned())?;
    let crs = CrsLib::get(&preset, name);

    Ok(crs.json())
}
#[tauri::command]
fn get_capacity(crstype: &str, name: &str, material: &str) -> Result<Value, String> {
    let preset = PRESETS::get(crstype).ok_or_else(|| "Could not get preset".to_owned())?;
    let crs = CrsLib::get(&preset, name);
    let matvariant = Variant::get(material).ok_or_else(|| "Could not get material".to_owned())?;
    let mat = Steel::from(&matvariant);
    let cmb = ColumnBeam::new(crs, mat);
    Ok(cmb.json())
}

#[tauri::command]
fn get_section_names(crstype: &str) -> Result<Value, String> {
    let preset = PRESETS::get(crstype).ok_or_else(|| "Could not get preset".to_owned())?;
    Ok(json!(CrsLib::sections(&preset)))
}
#[tauri::command]
fn get_steel_variants() -> Value {
    json!(Variant::variants())
}
#[tauri::command]
fn get_material_properties(material: &str) -> Result<Value, String> {
    let matvariant = Variant::get(material).ok_or_else(|| "Could not get material".to_owned())?;
    let mat = Steel::from(&matvariant);
    Ok(mat.json())
}

#[tauri::command]
fn get_buckle_curve(
    crstype: &str,
    name: &str,
    material: &str,
    limitstate: &str,
    curve_y: &str,
    curve_z: &str,
) -> Result<Value, String> {
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Capacity {
        L_k: f64,
        N_eu_y: f64,
        N_eu_z: f64,
        N_rd_y: f64,
        N_rd_z: f64,
        N_pl: f64,
    }
    let limitstate =
        LimitStateType::get(limitstate).ok_or_else(|| "Could not get limitstate".to_owned())?;
    let preset = PRESETS::get(crstype).ok_or_else(|| "Could not get preset".to_owned())?;
    let crs = CrsLib::get(&preset, name);
    let matvariant = Variant::get(material).ok_or_else(|| "Could not get material".to_owned())?;
    let mat = Steel::from(&matvariant);
    let cmb = ColumnBeam::new(crs, mat);

    let buckle_curve_y =
        BuckleCurve::get(curve_y).ok_or_else(|| "Could not get buckle curve y".to_owned())?;
    let buckle_curve_z =
        BuckleCurve::get(curve_z).ok_or_else(|| "Could not get buckle curve z".to_owned())?;

    let mut l: f64 = 0.0;
    let l_max = 20000.0; //mm
    let dl = 200.0; //mm

    let mut out: Vec<Capacity> = Vec::new();

    while l <= l_max {
        l += dl;
        out.push(Capacity {
            L_k: l,
            N_pl: cmb.N_pl(&limitstate),
            N_eu_y: cmb.euler_load(l, Axis::Y),
            N_eu_z: cmb.euler_load(l, Axis::Z),
            N_rd_y: cmb.buckle_cap(l, Axis::Y, &buckle_curve_y, &limitstate),
            N_rd_z: cmb.buckle_cap(l, Axis::Z, &buckle_curve_z, &limitstate),
        });
    }
    Ok(json![out])
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn lib_import_works() {
        let crs = CrsLib::get(&PRESETS::HEB, "HEB 100");
        dbg!(crs.json());
    }

    #[test]
    fn sectionnamesworks() {
        dbg!(CrsLib::sections(&PRESETS::HEB));
    }
    #[test]
    fn get_capacity() {
        let crs = CrsLib::get(&PRESETS::HEB, "HEB 100");
        let matvariant = Variant::get("S355").unwrap();
        let mat = Steel::from(&matvariant);
        let cmb = ColumnBeam::new(crs, mat);
        dbg!(cmb.json());
    }
}
