use csv::ReaderBuilder;
use reqwest::blocking::get;
use rusqlite::{params, Connection, Result};
use std::fs::File;
use std::io::{self, BufReader, Write};

type DrinkData = Vec<(String, i32, i32, i32, f64)>;


pub fn extract(url: &str, file_path: &str) -> io::Result<()> {
    // Fetch the data from the provided URL
    let response = get(url).expect("Failed to fetch data from URL");
    let mut file = File::create(file_path)?;
    file.write_all(&response.bytes().expect("Failed to read bytes"))?;
    Ok(())
}

pub fn load(file_path: &str) -> Result<()> {
    let conn = Connection::open("DrinksDB.db")?;
    conn.execute("DROP TABLE IF EXISTS DrinksDB", [])?;
    conn.execute(
        "CREATE TABLE DrinksDB (
            country TEXT,
            beer_servings INTEGER,
            spirit_servings INTEGER,
            wine_servings INTEGER,
            total_litres_of_pure_alcohol REAL
        )",
        [],
    )?;

    let file = File::open(file_path).expect("Failed to open CSV file");
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(file));

    let mut stmt = conn.prepare(
        "INSERT INTO DrinksDB (country, beer_servings, spirit_servings, wine_servings, total_litres_of_pure_alcohol) VALUES (?, ?, ?, ?, ?)"
    )?;

    for result in rdr.records() {
        let record = result.expect("Failed to read record");
        stmt.execute(params![
            &record[0], // Add & to pass a double reference
            record[1].parse::<i32>().unwrap_or(0),
            record[2].parse::<i32>().unwrap_or(0),
            record[3].parse::<i32>().unwrap_or(0),
            record[4].parse::<f64>().unwrap_or(0.0),
        ])?;
    }
    Ok(())
}

pub fn create_record(
    country: &str,
    beer_servings: i32,
    spirit_servings: i32,
    wine_servings: i32,
    total_alcohol: f64,
) -> Result<()> {
    let conn = Connection::open("DrinksDB.db")?;
    conn.execute(
        "INSERT INTO DrinksDB (country, beer_servings, spirit_servings, wine_servings, total_litres_of_pure_alcohol) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![country, beer_servings, spirit_servings, wine_servings, total_alcohol],
    )?;
    Ok(())
}

pub fn update_record(
    country: &str,
    beer_servings: i32,
    spirit_servings: i32,
    wine_servings: i32,
    total_alcohol: f64,
) -> Result<()> {
    let conn = Connection::open("DrinksDB.db")?;
    conn.execute(
        "UPDATE DrinksDB SET beer_servings=?1, spirit_servings=?2, wine_servings=?3, total_litres_of_pure_alcohol=?4 WHERE country=?5",
        params![beer_servings, spirit_servings, wine_servings, total_alcohol, country],
    )?;
    Ok(())
}

pub fn delete_record(country: &str) -> Result<()> {
    let conn = Connection::open("DrinksDB.db")?;
    conn.execute("DELETE FROM DrinksDB WHERE country=?", params![country])?;
    Ok(())
}

pub fn read_data() -> Result<DrinkData> {
    let conn = Connection::open("DrinksDB.db")?;
    let mut stmt = conn.prepare("SELECT * FROM DrinksDB")?;
    let records = stmt
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(records)
}

pub fn general_query(query: &str) -> Result<DrinkData> {
    let conn = Connection::open("DrinksDB.db")?;
    let mut results = Vec::new();

    if query.trim().to_lowercase().starts_with("select") {
        let mut stmt = conn.prepare(query)?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get(0)?, // country as String
                row.get(1)?, // beer_servings as i32
                row.get(2)?, // spirit_servings as i32
                row.get(3)?, // wine_servings as i32
                row.get(4)?, // total_litres_of_pure_alcohol as f64
            ))
        })?;

        for row in rows {
            results.push(row?);
        }
    } else {
        conn.execute(query, [])?;
    }
    Ok(results)
}

// Logging functionality (writes the query to a markdown log file)
pub fn log_query(query: &str) -> io::Result<()> {
    let mut file = File::options()
        .append(true)
        .create(true)
        .open("drinks_query_log.md")?;
    writeln!(file, "```sql\n{}\n```\n", query)?;
    Ok(())
}
