mod args;
mod structs;
mod utils;

use args::{Commands, PodcatchArgs};
use clap::Parser;
use color_eyre::eyre::Ok;
use color_eyre::eyre::{OptionExt, Result};
use rss::Channel;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::usize;
use structs::{Episode, Podcast};
use tokio::{fs::File, io::AsyncWriteExt};
use utils::get_file;
use utils::get_input;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = PodcatchArgs::parse();
    dbg!(&cli);

    init().await?;

    match &cli.command {
        Commands::Search { query } => {
            search(query.as_ref().ok_or_eyre("")?).await?;
        }
        Commands::Add { url } => add(url.as_ref().ok_or_eyre("")?).await?,
        Commands::Remove {} => remove().await?,
        Commands::List => list().await?,
    }

    Ok(())
}

async fn init() -> Result<()> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("podcatch")?;
    let podcasts_file_path = xdg_dirs.place_config_file("podcasts.json")?;

    if !podcasts_file_path.exists() {
        let mut file = File::create(podcasts_file_path).await?;
        file.write_all(b"[]").await?;
        return Ok(());
    }
    Ok(())
}

async fn search(query: &str) -> Result<()> {
    let resp = podcast_search::search(query).await?;
    let vec = resp.results;

    let mut results = HashMap::new();

    if vec.is_empty() {
        println!("No podcasts found");
        return Ok(());
    }

    println!("Type the number of the Podcast you want to subscribe to");
    for (i, result) in vec.clone().into_iter().enumerate().rev() {
        println!(
            "{} {}",
            i + 1,
            result.track_name.ok_or_eyre("Can't get Podcast Name")?
        );
        results.insert(i + 1, result.feed_url.ok_or_eyre("Can't get feed url!")?);
    }

    let input = get_input().await;

    add(results
        .get(&input.ok_or_eyre("Invalid Input!")?.parse::<usize>()?)
        .ok_or_eyre("Invalid Input!")?)
    .await?;
    Ok(())
}

async fn add(target: &str) -> Result<()> {
    let resp = reqwest::get(target).await?.bytes().await?;
    let channel = Channel::read_from(&resp[..])?;
    let mut episodes: Vec<Episode> = vec![];

    for x in channel.items {
        let tmp = Episode {
            title: x.title.ok_or_eyre("Episode Title wasn't found!")?,
            guid: x
                .guid
                .ok_or_eyre("Episode ID isn't found!")?
                .value
                .to_string(),
            description: x
                .description
                .ok_or_eyre("Episode Description isn't found!")?,
            mp3: x.enclosure.ok_or_eyre("Episode URL isn't found!")?.url,
        };

        episodes.push(tmp);
    }

    let podcast = structs::Podcast {
        name: channel.title,
        url: target.to_owned(),
        episodes,
    };

    let file_path = xdg::BaseDirectories::with_prefix("podcatch")?.get_config_file("podcasts.json");

    let mut podcasts: Vec<Podcast> =
        serde_json::from_str(fs::read_to_string(&file_path)?.as_ref())?;

    for x in &podcasts {
        if podcast.name.eq(&x.name) {
            eprintln!("You have already subscribed to this podcast!");
            return Ok(());
        }
    }

    podcasts.push(podcast);

    let mut file = File::create(&file_path).await?;

    file.write_all(serde_json::to_string_pretty(&podcasts)?.as_bytes())
        .await?;

    Ok(())
}

async fn remove() -> Result<()> {
    let file_path = get_file().await.ok_or_eyre("Podcasts file not found!")?;
    let mut podcasts: Vec<Podcast> =
        serde_json::from_str(fs::read_to_string(&file_path)?.as_ref())?;

    println!("Type the number of the Podcast you want to unsubscribe to");
    for (i, result) in podcasts.clone().into_iter().enumerate().rev() {
        println!("{} {}", i + 1, result.name);
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    podcasts.remove(input.trim_end().parse::<usize>()?.saturating_sub(1));

    let mut file = File::create(&file_path).await?;

    file.write_all(serde_json::to_string_pretty(&podcasts)?.as_bytes())
        .await?;

    Ok(())
}

async fn list() -> Result<()> {
    let file = get_file().await.ok_or_eyre("Podcasts file not found!")?;

    let podcasts: Vec<Podcast> = serde_json::from_str(fs::read_to_string(&file)?.as_ref())?;

    println!("Your are currently subscribed to the following podcasts:");

    for p in podcasts {
        println!("{}", p.name);
    }

    Ok(())
}
