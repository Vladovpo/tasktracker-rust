mod app;
mod cli;
mod models;
mod storage;

use clap::Parser;

use crate::app::App;
use crate::cli::Cli;

fn main() {
    let cli = Cli::parse();
    let mut app: App = App::new();

    app.run(cli);
}