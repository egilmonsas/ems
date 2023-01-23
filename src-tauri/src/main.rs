#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use emsdesign::crs::{CrossSectionLib, Variant};
use emsdesign::erc::NSEN_1993::BuckleCurve;
use emsdesign::mat::steel::{Class, Steel};
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
    let preset = Variant::get(crstype).ok_or_else(|| "Could not get preset".to_owned())?;
    let crs = CrossSectionLib::get(&preset, name);

    Ok(crs.json())
}
#[tauri::command]
fn get_capacity(crstype: &str, name: &str, material: &str) -> Result<Value, String> {
    let preset = Variant::get(crstype).ok_or_else(|| "Could not get preset".to_owned())?;
    let crs = CrossSectionLib::get(&preset, name);
    let matvariant = Class::get(material).ok_or_else(|| "Could not get material".to_owned())?;
    let mat = Steel::from(&matvariant);
    let cmb = ColumnBeam::new(crs, mat);
    Ok(cmb.json())
}

#[tauri::command]
fn get_section_names(crstype: &str) -> Result<Value, String> {
    let preset = Variant::get(crstype).ok_or_else(|| "Could not get preset".to_owned())?;
    Ok(json!(CrossSectionLib::sections(&preset)))
}
#[tauri::command]
fn get_steel_variants() -> Value {
    json!(Class::variants())
}
#[tauri::command]
fn get_material_properties(material: &str) -> Result<Value, String> {
    let matvariant = Class::get(material).ok_or_else(|| "Could not get material".to_owned())?;
    let mat = Steel::from(&matvariant);
    Ok(mat.json())
}
#[derive(Serialize, Deserialize)]
struct Capacity {
    L_k: f64,
    N_eu_y: f64,
    N_eu_z: f64,
    N_rd_y: f64,
    N_rd_z: f64,
    N_pl: f64,
}
#[tauri::command]
fn get_buckle_curve(
    crstype: &str,
    name: &str,
    material: &str,
    limitstate: &str,
    curve_y: &str,
    curve_z: &str,
) -> Result<Vec<Capacity>, String> {
    #[allow(non_snake_case)]
    let limitstate =
        LimitStateType::get(limitstate).ok_or_else(|| "Could not get limitstate".to_owned())?;
    let preset = Variant::get(crstype).ok_or_else(|| "Could not get preset".to_owned())?;
    let crs = CrossSectionLib::get(&preset, name);
    let matvariant = Class::get(material).ok_or_else(|| "Could not get material".to_owned())?;
    let mat = Steel::from(&matvariant);
    let cmb = ColumnBeam::new(crs, mat);

    let buckle_curve_y =
        BuckleCurve::get(curve_y).ok_or_else(|| "Could not get buckle curve y".to_owned())?;
    let buckle_curve_z =
        BuckleCurve::get(curve_z).ok_or_else(|| "Could not get buckle curve z".to_owned())?;

    let mut l: f64 = 0.0;
    let l_max = 100000.0; //mm
    let dl = 100.0; //mm

    let mut out: Vec<Capacity> = Vec::new();
    #[allow(non_snake_case)]
    let N_pl = cmb.N_pl(&limitstate);

    while l <= l_max {
        l += dl;
        #[allow(non_snake_case)]
        let L_k = l;
        #[allow(non_snake_case)]
        let N_eu_y = cmb.euler_load(l, Axis::Y);
        #[allow(non_snake_case)]
        let N_eu_z = cmb.euler_load(l, Axis::Z);
        #[allow(non_snake_case)]
        let N_rd_y = cmb.buckle_cap(l, Axis::Y, &buckle_curve_y, &limitstate);
        #[allow(non_snake_case)]
        let N_rd_z = cmb.buckle_cap(l, Axis::Z, &buckle_curve_z, &limitstate);
        out.push(Capacity {
            L_k,
            N_pl,
            N_eu_y,
            N_eu_z,
            N_rd_y,
            N_rd_z,
        });
        if N_rd_y.max(N_rd_z) <= 0.1 * N_pl {
            break;
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn lib_import_works() {
        let crs = CrossSectionLib::get(&Variant::HEB, "HEB 100");
        dbg!(crs.json());
    }

    #[test]
    fn sectionnamesworks() {
        dbg!(CrossSectionLib::sections(&Variant::HEB));
    }
    #[test]
    fn get_capacity() {
        let crs = CrossSectionLib::get(&Variant::HEB, "HEB 100");
        let matvariant = Class::get("S355").unwrap();
        let mat = Steel::from(&matvariant);
        let cmb = ColumnBeam::new(crs, mat);
        dbg!(cmb.json());
    }
}
