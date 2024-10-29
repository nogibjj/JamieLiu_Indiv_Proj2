use sqlite::{
    create_record, delete_record, extract, general_query, load, read_data, read_record,
    update_record,
};

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
    Load,
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
    ReadRecord {
        country: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.action {
        Actions::Extract => {
            println!("Extracting data...");
            if let Err(e) = extract("https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/alcohol-consumption/drinks.csv", "data/drinks.csv") {
                eprintln!("Error in extract: {}", e);
                process::exit(1);
            }
        }
        Actions::Load => {
            println!("Transforming and loading data...");
            if let Err(e) = load("data/drinks.csv") {
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
            if let Err(e) = update_record(
                country,
                *beer_servings,
                *spirit_servings,
                *wine_servings,
                *total_alcohol,
            ) {
                eprintln!("Error in update_record: {}", e);
                process::exit(1);
            }
        }
        Actions::DeleteRecord { country } => {
            if let Err(e) = delete_record(country) {
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
            if let Err(e) = create_record(
                country,
                *beer_servings,
                *spirit_servings,
                *wine_servings,
                *total_alcohol,
            ) {
                eprintln!("Error in create_record: {}", e);
                process::exit(1);
            }
        }
        Actions::GeneralQuery { query } => {
            match general_query(query) {
                Ok(data) => {
                    println!("Query results:");
                    for row in data {
                        println!("{:?}", row); // Use {:?} to print the tuple as Debug
                    }
                }
                Err(e) => {
                    eprintln!("Error in general_query: {}", e);
                    process::exit(1);
                }
            }
        }
        Actions::ReadData => match read_data() {
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
        },
        Actions::ReadRecord { country } => match read_record(country) {
            Ok(Some(record)) => {
                println!("Record found: {:?}", record);
            }
            Ok(None) => {
                println!("No record found for country '{}'", country);
            }
            Err(e) => {
                eprintln!("Error in read_record: {}", e);
                process::exit(1);
            }
        },
    }
}
