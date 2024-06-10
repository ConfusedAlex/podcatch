//use color_eyre::eyre::Result;
use std::{io, path::PathBuf};

pub async fn get_input() -> Option<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let input = input.trim();

    if input.is_empty() {
        None
    } else {
        Some(input.to_string())
    }
}

pub async fn get_file() -> Option<PathBuf> {
    Some(
        xdg::BaseDirectories::with_prefix("podcatch")
            .ok()?
            .get_config_file("podcasts.json"),
    )
}
