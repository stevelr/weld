// build.rs - build smithy models into rust sources at compile tile

// path to codegen.toml relative to location of build.rs
const CONFIG: &str = "../codegen.toml";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    wasmcloud_weld_codegen::rust_build(CONFIG)?;
    Ok(())
}