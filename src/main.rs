mod vulkan_core;
mod render_app;

use anyhow::Result;


fn main() -> Result<()> {
    pretty_env_logger::init();
    render_app::run_app()
}