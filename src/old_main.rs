// Suggestion: Can you live-code a program counts words in a text file
// and writes the result in a SQL database?

use postgres::{Client, NoTls};
use std::{env, fs};

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let file_name = env::args().collect::<Vec<String>>();
    let file_name = file_name
        .get(1)
        .map(|file_name| file_name.trim())
        .expect("Filename argument is missing");

    let mut client = Client::connect("host=localhost user=postgres password=test", NoTls)?;
    the_program(file_name, &mut client)?;

    println!("da file name: {}", file_name);
    Ok(())
}

fn setup_db_table(table_name: &str, client: &mut Client) -> Result<(), Error> {
    let query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
        id          SERIAL PRIMARY KEY,
        filename    TEXT NOT NULL,
        word_count  INTEGER NOT NULL
        )",
        table_name
    );
    client.batch_execute(&query)?;
    Ok(())
}

fn the_program(file_name: &str, client: &mut Client) -> Result<(), Error> {
    setup_db_table("results", client)?;

    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let word_count = contents.split_whitespace().fold(0, |total, _| total + 1);
    println!("Number of words: {}", word_count);

    client.execute(
        "INSERT INTO results (filename, word_count) VALUES ($1, $2)",
        &[&file_name, &word_count],
    )?;

    let my_slice = &[];

    for row in client.query("SELECT * FROM results", my_slice)? {
        let id: i32 = row.get("id");
        let file_name: &str = row.get("filename");
        let word_count: i32 = row.get("word_count");

        println!("found row: {} {} {}", id, file_name, word_count);
    }

    Ok(())
}
