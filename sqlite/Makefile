# Makefile for Rust Project

all: check build format lint test release

check:
	cargo check

build:
	cargo build

format:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test

release:
	cargo build --release

# Custom tasks

# Example: Extract data
extract: 
	cargo run -- extract

# Example: Transform and Load data
load:
	cargo run -- load

# Example: Read from the database
read-data:
	cargo run -- read-data

# Example: Create a database entry
create:
	cargo run -- create-record Testland 300 200 150 12.5

# Example: Read a specific database entry
read:
	cargo run -- read-record Testland

# Example: Update a database entry
update:
	cargo run -- update-record Testland 320 210 160 13.0

# Example: Delete a database entry
delete:
	cargo run -- delete-record Testland
	
# Example: Custom query to check for Germany in the DrinksDB table
query:
	cargo run -- general-query "SELECT * FROM DrinksDB WHERE country = 'Germany';"
