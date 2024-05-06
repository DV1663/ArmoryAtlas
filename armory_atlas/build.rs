
use pyo3_bindgen::Codegen;
pub const DATABASE_HANDLER: &str = include_str!("ArmoryAtlasDBHandler.py");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // set the local module into PYTHONPATH
    //std::env::set_var("PYTHONPATH", format!("{}/pymodules/mysql-connector-python-8.4.0", std::env::var("CARGO_MANIFEST_DIR")?));

    Codegen::default()
        .module_names(["os"])?
        .module_from_str(DATABASE_HANDLER, "ArmoryAtlasDBHandler")?
        .build(format!("{}/bindings.rs", std::env::var("OUT_DIR")?))?;
    Ok(())
}