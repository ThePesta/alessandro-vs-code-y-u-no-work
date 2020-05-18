#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use std::{env, fs};
use postgres::{Client, NoTls};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = env::args().collect::<Vec<String>>();
    let file_name = file_name
        .get(1)
        .map(|file_name| file_name.trim())
        .expect("Filename argument is missing");
    let result = the_program(file_name);
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["asdad ", "\n"],
            &match (&file_name,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(
                    arg0,
                    ::core::fmt::Display::fmt,
                )],
            },
        ));
    };
    result
}
fn the_program(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::connect("host=localhost user=postgres password=test", NoTls)?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS results (
            id          SERIAL PRIMARY KEY,
            filename    TEXT NOT NULL,
            word_count  INTEGER NOT NULL
        )
    ",
    )?;
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let word_count = contents.split_whitespace().fold(0, |total, _| total + 1);
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["Number of words: ", "\n"],
            &match (&word_count,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(
                    arg0,
                    ::core::fmt::Display::fmt,
                )],
            },
        ));
    };
    client.execute(
        "INSERT INTO results (filename, word_count) VALUES ($1, $2)",
        &[&file_name, &word_count],
    )?;
    let my_slice = &[];
    for row in client.query("SELECT * FROM results", my_slice)? {
        let id: i32 = row.get("id");
        let file_name: &str = row.get("filename");
        let word_count: i32 = row.get("word_count");
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["found row: ", " ", " ", "\n"],
                &match (&id, &file_name, &word_count) {
                    (arg0, arg1, arg2) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Display::fmt),
                    ],
                },
            ));
        };
    }
    Ok(())
}
