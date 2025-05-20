use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Serialize, Deserialize};
use mongodb::{Client, options::ClientOptions};
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Weapon {
    name: String,
    damage: i32,
    weight: f64,
    upgrade: String,
    perk: String,
    weapon_type: String,
    predicted_price: Option<f64>,
}

fn main() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();

    print!("Enter a base name for the weapon (e.g., Hilda): ");
    io::stdout().flush().unwrap();
    let mut base_name = String::new();
    io::stdin().read_line(&mut base_name).unwrap();
    let base_name = base_name.trim();

    Python::with_gil(|py| {

        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let sys = py.import("sys")?;
        sys.getattr("path")?
            .call_method1("append", (project_root.to_str().unwrap(),))?;

        println!("Using Python version: {}", sys.getattr("version")?.extract::<String>()?);

        let ai = PyModule::import(py, "ai_calls")?;
        let predictor = PyModule::import(py, "model_predictor")?;

        let generate_name = ai.getattr("generate_weapon_name")?;
        let full_name: String = generate_name.call1((base_name,))?.extract()?;
        println!("Generated name: {}", full_name);

        let generate_stats = ai.getattr("generate_weapon")?;
        let py_result = generate_stats.call1((full_name.clone(),))?;
        let py_dict = py_result.downcast::<PyDict>()?;

        let weapon = Weapon {
            name: extract_string(&py_dict, "Name")?,
            damage: extract_i32(&py_dict, "Damage")?,
            weight: extract_f64(&py_dict, "Weight")?,
            upgrade: extract_string(&py_dict, "Upgrade")?,
            perk: extract_string(&py_dict, "Perk")?,
            weapon_type: extract_string(&py_dict, "Type")?,
            predicted_price: None,
        };

        let predict = predictor.getattr("predict_price")?;
        let predicted: f64 = predict.call1((py_dict,))?.extract()?;
        let mut weapon = weapon;
        weapon.predicted_price = Some(predicted);

        println!("Final Weapon: {:?}", weapon);

        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        rt.block_on(async {
            let client_options = ClientOptions::parse("mongodb://localhost:27017").await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyConnectionError, _>(e.to_string()))?;
            let client = Client::with_options(client_options)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyConnectionError, _>(e.to_string()))?;
            let db = client.database("Skyrim_weapons");
            let coll = db.collection::<Weapon>("weapons");

            coll.insert_one(weapon, None).await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
            println!("Inserted into MongoDB!");
            Ok(())
        })
    })
}

fn extract_string(dict: &Bound<'_, PyDict>, key: &str) -> PyResult<String> {
    dict.get_item(key)?
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Missing key: {}", key)))?
        .extract()
}

fn extract_i32(dict: &Bound<'_, PyDict>, key: &str) -> PyResult<i32> {
    dict.get_item(key)?
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Missing key: {}", key)))?
        .extract()
}

fn extract_f64(dict: &Bound<'_, PyDict>, key: &str) -> PyResult<f64> {
    dict.get_item(key)?
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Missing key: {}", key)))?
        .extract()
}