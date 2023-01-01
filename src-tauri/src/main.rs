#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use emsdesign::crs::standard::{CrsLib, PresetCrs, PRESETS};
use emsdesign::crs::CrossSection;
use emsdesign::mat::steel::{Steel, SteelVariant};
use emsdesign::mat::Material;
use emsdesign::mmb::columnbeam::ColumnBeam;
use emsdesign::{Axis, Gamma};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            compute,
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
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn compute(num: i64) -> i64 {
    num * num
}

#[tauri::command]
fn get_area(crstype: &str, name: &str) -> Value {
    let df = CrsLib::new(&PRESETS::get(crstype));
    let crs = PresetCrs::new(name, &df);

    crs.json()
}
#[tauri::command]
fn get_capacity(crstype: &str, name: &str, material: &str) -> Value {
    let df = CrsLib::new(&PRESETS::get(crstype));

    let crs = PresetCrs::new(name, &df);
    let matvariant = SteelVariant::get(material);
    let mat = Steel::from(matvariant);

    let cmb = ColumnBeam::new(Box::new(crs), mat);
    cmb.json()
}

#[tauri::command]
fn get_section_names(crstype: &str) -> Value {
    let df = CrsLib::new(&PRESETS::get(crstype));

    json!(df.sections())
}
#[tauri::command]
fn get_steel_variants() -> Value {
    json!(SteelVariant::variants())
}
#[tauri::command]
fn get_material_properties(material: &str) -> Value {
    let matvariant = SteelVariant::get(material);
    let mat = Steel::from(matvariant);
    mat.json()
}

#[tauri::command]
fn get_buckle_curve(crstype: &str, name: &str, material: &str) -> Value {
    #[derive(Serialize, Deserialize)]
    struct Capacity {
        L_k: f64,
        N_eu_y: f64,
        N_eu_z: f64,
        N_rd_y: f64,
        N_rd_z: f64,
        N_pl: f64,
    }
    let df = CrsLib::new(&PRESETS::get(crstype));

    let crs = PresetCrs::new(name, &df);
    let matvariant = SteelVariant::get(material);
    let mat = Steel::from(matvariant);

    let cmb = ColumnBeam::new(Box::new(crs), mat);

    let mut l: f64 = 0.0;
    let l_max = 20000.0; //mm
    let dl = 200.0; //mm

    let mut out: Vec<Capacity> = Vec::new();

    while l <= l_max {
        l += dl;
        out.push(Capacity {
            L_k: l,
            N_pl: cmb.N_pl(&Gamma::K),
            N_eu_y: cmb.euler_load(l, Axis::Y),
            N_eu_z: cmb.euler_load(l, Axis::Z),
            N_rd_y: cmb.buckle_cap(l, Axis::Y, &Gamma::K),
            N_rd_z: cmb.buckle_cap(l, Axis::Z, &Gamma::K),
        });
    }
    json![out]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn lib_import_works() {
        let df = CrsLib::new(&PRESETS::HEB);
        let crs = PresetCrs::new("HEB 100", &df);
        dbg!(crs.json());
    }

    #[test]
    fn sectionnamesworks() {
        let df = CrsLib::new(&PRESETS::HEB);
        dbg!(df.sections());
        dbg!(json!(df.sections()));
    }
    #[test]
    fn get_capacity() {
        let df = CrsLib::new(&PRESETS::get("HEB"));

        let crs = PresetCrs::new("HEB 100", &df);
        let matvariant = SteelVariant::get("S355");
        let mat = Steel::from(matvariant);

        let cmb = ColumnBeam::new(Box::new(crs), mat);
        dbg!(cmb.json());
    }

    #[test]
    fn get_buckle_curve() {
        #[derive(Serialize, Deserialize)]
        struct Capacity {
            L_k: f64,
            N_rd: f64,
            N_pl: f64,
        }

        let df = CrsLib::new(&PRESETS::get("HEB"));

        let crs = PresetCrs::new("HEB 100", &df);
        let matvariant = SteelVariant::get("S355");
        let mat = Steel::from(matvariant);

        let cmb = ColumnBeam::new(Box::new(crs), mat);

        let mut l = 1000.0;
        let l_max = 20000.0; //mm
        let dl = 1000.0; //mm

        let mut out: Vec<Capacity> = Vec::new();

        while l <= l_max {
            let n_rd = cmb.buckle_cap(5000.0, Axis::Z, &Gamma::K);
            let BuckleResponse = Capacity {
                L_k: l,
                N_rd: cmb.N_pl(&Gamma::K),
                N_pl: n_rd,
            };
            out.push(BuckleResponse);
            l += dl;
        }
        let json = json![out];
        use std::fs::File;
        serde_json::to_writer(
            &File::create("data.json").expect("Couldnt create file"),
            &json,
        )
        .expect("Couldnt write file");
    }
}
