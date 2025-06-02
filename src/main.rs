mod utils; mod config_parse; mod parse;
mod ui;

use config_parse::Config;
use parse::App;

use anyhow;

const DEFAULT_PATH: &str = "config.toml";

fn main() -> anyhow::Result<()> {
    let mut conf = Config::new();
    conf.load(DEFAULT_PATH.to_string())?;
    
    let mut app = App::new(&conf);
    app.init(None)?;
    app.list_dir()?;
    
    Ok(())
}
