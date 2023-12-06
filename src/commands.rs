use clap::{Args, Parser, Subcommand};
use miette::miette;
use rusqlite::Connection;

use crate::data;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Info(InfoArgs),
    List(ListArgs),
    Create(CreateArgs),
    Edit(EditArgs),
    Remove(RemoveArgs),
}

pub trait Executable {
    fn execute(&self, conn: &Connection) -> Result<(), miette::ErrReport>;
}

#[derive(Args)]
pub struct InfoArgs {
    #[arg(help = "The name of the note.")]
    name: String,
}

impl Executable for InfoArgs {
    fn execute(&self, conn: &Connection) -> Result<(), miette::ErrReport> {
        let note = data::select(&conn, &self.name)?;
        println!("{}", note);

        Ok(())
    }
}

#[derive(Args)]
pub struct ListArgs {}

impl Executable for ListArgs {
    fn execute(&self, conn: &Connection) -> Result<(), miette::ErrReport> {
        let notes = data::select_all(&conn)?;
        for note in notes.into_iter() {
            println!("{}", note.name);
        }

        Ok(())
    }
}

#[derive(Args)]
pub struct CreateArgs {
    #[arg(help = "The name of the note.")]
    name: String,
}

impl Executable for CreateArgs {
    fn execute(&self, conn: &Connection) -> Result<(), miette::ErrReport> {
        let body = edit::edit("").map_err(|e| miette!("{}", e))?;
        let mut note = data::Note::default();
        note.name = self.name.clone();
        note.body = body;

        data::insert(&conn, &note)
    }
}

#[derive(Args)]
pub struct EditArgs {
    #[arg(help = "The name of the note.")]
    name: String,
}

impl Executable for EditArgs {
    fn execute(&self, conn: &Connection) -> Result<(), miette::ErrReport> {
        let mut note = data::select(&conn, &self.name)?;
        let body = edit::edit(note.body).map_err(|e| miette!("{}", e))?;
        note.body = body;

        data::update(&conn, &mut note)
    }
}

#[derive(Args)]
pub struct RemoveArgs {
    #[arg(help = "The name of the note.")]
    name: String,
}

impl Executable for RemoveArgs {
    fn execute(&self, conn: &Connection) -> Result<(), miette::ErrReport> {
        data::delete(&conn, &self.name)
    }
}
