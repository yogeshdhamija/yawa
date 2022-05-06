use anyhow::Result;
use yawa::services::ports::{PersistenceAdapter, UserInputAdapter};
use yawa::{adapters, controllers};

fn main() -> Result<()> {
    let (file_system_adapter, tui_adapter) = initialize_dependencies()?;
    controllers::cli::execute_based_on_args(file_system_adapter, &tui_adapter)?;
    Ok(())
}

fn initialize_dependencies() -> Result<(impl PersistenceAdapter, impl UserInputAdapter)> {
    let file_system_adapter = adapters::filesystem::new()?;
    let tui_adapter = adapters::tui::new();
    Ok((file_system_adapter, tui_adapter))
}
