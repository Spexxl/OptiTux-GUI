use crate::core::models::Game;
use std::fs;
use std::path::Path;
use anyhow::Result;

pub struct Installer;

impl Installer {
    pub fn install(game: &Game, zip_path: &Path) -> Result<()> {

        Ok(())
    }

    pub fn uninstall(game: &Game) -> Result<()> {

        Ok(())
    }
}
