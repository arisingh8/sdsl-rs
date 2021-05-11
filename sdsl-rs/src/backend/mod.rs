use anyhow::{format_err, Result};

mod analyse;
mod common;
pub mod sdsl_c;

pub fn build() -> Result<()> {
    simple_logger::SimpleLogger::new().init()?;

    if common::skip_build() {
        return Ok(());
    }

    let out_directory = std::env::var("OUT_DIR").map_err(|e| format_err!("{}", e))?;
    let out_directory = std::path::PathBuf::from(&out_directory);

    let crate_directory = get_crate_directory()?;
    let code_metadata = analyse::setup(&crate_directory, &out_directory)?;

    let specifications = if let Some(code_metadata) = code_metadata {
        analyse::analyse(&code_metadata)?
    } else {
        log::debug!("Failed to generate code metadata for analysis. Exiting SDSL build.");
        return Ok(());
    };

    let template_directory = sdsl_c::template::setup(&out_directory)?;
    let interface_directory =
        sdsl_c::specification::setup(&specifications, &template_directory, &out_directory)?;
    let lib_path = sdsl_c::specification::compile(&interface_directory)?;
    log::info!("Compilation complete. Library path: {}", lib_path.display());

    println!("cargo:rerun-if-changed=./src");
    Ok(())
}

fn get_crate_directory() -> Result<std::path::PathBuf> {
    let cargo_manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR").ok_or(format_err!(
        "Failed to read env variable: CARGO_MANIFEST_DIR"
    ))?;
    Ok(std::path::PathBuf::from(&cargo_manifest_dir))
}
