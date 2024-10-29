mod lib;

use clap::{Parser, Subcommand};
use std::process;

/// Command-line arguments parser
#[derive(Parser)]
#[command(name = "Drinks ETL-Query CLI")]
#[command(about = "ETL-Query CLI tool for the drinks dataset", long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Actions,
}

#[derive(Subcommand)]
enum Actions {
    Extract,
    TransformLoad,
    UpdateRecord {
        country: String,
        beer_servings: i32,
        spirit_servings: i32,
        wine_servings: i32,
        total_alcohol: f64,
    },
    DeleteRecord {
        country: String,
    },
    CreateRecord {
        country: String,
        beer_servings: i32,
        spirit_servings: i32,
        wine_servings: i32,
        total_alcohol: f64,
    },
    GeneralQuery {
        query: String,
    },
    ReadData,
}

fn main() {
    let cli = Cli::parse();

    match &cli.action {
        Actions::Extract => {
            println!("Extracting data...");
            if let Err(e) = lib::extract("https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/alcohol-consumption/drinks.csv", "data/drinks.csv") {
                eprintln!("Error in extract: {}", e);
                process::exit(1);
            }
        }
        Actions::TransformLoad => {
            println!("Transforming and loading data...");
            if let Err(e) = lib::load("data/drinks.csv") {
                eprintln!("Error in load: {}", e);
                process::exit(1);
            }
        }
        Actions::UpdateRecord {
            country,
            beer_servings,
            spirit_servings,
            wine_servings,
            total_alcohol,
        } => {
            if let Err(e) = lib::update_record(country, *beer_servings, *spirit_servings, *wine_servings, *total_alcohol) {
                eprintln!("Error in update_record: {}", e);
                process::exit(1);
            }
        }
        Actions::DeleteRecord { country } => {
            if let Err(e) = lib::delete_record(country) {
                eprintln!("Error in delete_record: {}", e);
                process::exit(1);
            }
        }
        Actions::CreateRecord {
            country,
            beer_servings,
            spirit_servings,
            wine_servings,
            total_alcohol,
        } => {
            if let Err(e) = lib::create_record(country, *beer_servings, *spirit_servings, *wine_servings, *total_alcohol) {
                eprintln!("Error in create_record: {}", e);
                process::exit(1);
            }
        }
        Actions::GeneralQuery { query } => {
            if let Err(e) = lib::log_query(query) {
                eprintln!("Error logging query: {}", e);
            }
            match lib::general_query(query) {
                Ok(data) => println!("Query results: {:?}", data),
                Err(e) => {
                    eprintln!("Error in general_query: {}", e);
                    process::exit(1);
                }
            }
        }
        Actions::ReadData => {
            match lib::read_data() {
                Ok(data) => {
                    println!("Data:");
                    for row in data {
                        println!("{:?}", row);
                    }
                }
                Err(e) => {
                    eprintln!("Error in read_data: {}", e);
                    process::exit(1);
                }
            }
        }
    }
}
