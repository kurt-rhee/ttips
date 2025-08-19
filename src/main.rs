mod scraper;

use clap::Parser;
use chrono::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(serde::Deserialize, Debug)]
struct Tip {
    command: String,
    description: String,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    vim: bool,

    #[clap(short, long)]
    korean: bool,

    #[clap(long)]
    scrape: bool,
}

fn main() {
    let args = Args::parse();

    if args.scrape {
        println!("Scraping new tips...");
        scraper::scrape_and_save().unwrap();
        println!("Done.");
        return;
    }

    let vim_tips_json = include_str!("vim_tips.json");
    let vim_tips: Vec<Tip> = serde_json::from_str(vim_tips_json).unwrap();

    let korean_tips_json = include_str!("korean_tips.json");
    let korean_tips: Vec<Tip> = serde_json::from_str(korean_tips_json).unwrap();

    let now = Local::now();
    let seed = now.ordinal() as u64;
    let mut rng = StdRng::seed_from_u64(seed);

    if args.vim {
        let index = rng.gen_range(0..vim_tips.len());
        let tip = &vim_tips[index];
        println!("Vim tip of the day:\n{} - {}", tip.command, tip.description);
    } else if args.korean {
        let index = rng.gen_range(0..korean_tips.len());
        let tip = &korean_tips[index];
        println!("Korean word of the day:\n{} - {}", tip.command, tip.description);
    } else {
        println!("Please specify a tip type with -v for vim or -k for korean");
    }
}
