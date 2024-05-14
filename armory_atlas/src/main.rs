use anyhow::Result;
use armory_atlas_lib::{run_cli};

#[tokio::main]
async fn main() -> Result<()> {
    run_cli(None)
}
