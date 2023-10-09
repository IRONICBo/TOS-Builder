use std::error::Error;

use crate::app::App;

pub fn export_config(app: &mut App, path: String) -> Result<(), Box<dyn Error>> {
    app.export.message = format!("Export config to {}", path);

    serde_json::to_writer_pretty(
        std::fs::File::create(path)?,
        app,
    ).expect("Unable dump config to file");
    
    app.export.message = format!("Export config ok");
    Ok(())
}

// Unimplemented
pub fn import_config() -> Result<(), Box<dyn Error>> {
    let mut app = crate::app::App::default();
    app = serde_json::from_reader(std::fs::File::open("config.json")?)?;
    Ok(())
}