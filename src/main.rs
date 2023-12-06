use std::env;
use std::path::Path;

use clap::Parser;
use commands::{Cli, Commands, Executable};
use miette::miette;
use rusqlite::Connection;

pub mod commands;
pub mod data;

#[tokio::main]
async fn main() -> Result<(), miette::ErrReport> {
    let binding = env::var("JOT_PATH");
    let jot_path = match binding.as_ref() {
        Ok(value) => value.as_str(),
        Err(_) => "/",
    };
    let db_path = Path::new(jot_path).join("jot.sqlite");

    let conn = Connection::open(db_path).map_err(|e| miette!("{:?}", e))?;

    data::execute_schema(&conn)?;

    let cli = Cli::parse();

    let status = match cli.command {
        Commands::Note(args) => args.execute(&conn)?,
        Commands::List(args) => args.execute(&conn)?,
        Commands::Create(args) => args.execute(&conn)?,
        Commands::Edit(args) => args.execute(&conn)?,
        Commands::Remove(args) => args.execute(&conn)?,
    };

    Ok(status)
}
