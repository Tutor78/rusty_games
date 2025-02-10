use std::fs::create_dir_all;
use std::path::{ PathBuf };
use inquire::{Text, Confirm, Select};
use directories::BaseDirs;
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct User {
    username: String,
}

fn create_profile() -> User {
    let player_name = loop {
        let name_input = Text::new("Before we begin, what is your name?")
            .prompt();

        let validate_name = name_input.unwrap();

        if validate_name.chars().all(char::is_whitespace) {
            println!("Please enter a valid name!");
        } else {
            break validate_name;
        }
    };

    let user = User { username: player_name };

    user
}

pub fn play_again() -> bool {
    let play_again = Confirm::new("Play again?").prompt().unwrap();

    play_again
}

pub fn difficulty() -> String {
    let difficulty_options = vec!["Easy", "Normal", "Hard"];

    let difficulty = Select::new("Choose your difficulty:", difficulty_options)
        .prompt()
        .unwrap()
        .to_string();

    difficulty
}

// The following functions all deal with the db file
pub fn create_db() -> Result<()> {
    let path = path_to_db();
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (username TEXT NOT NULL PRIMARY KEY)",
        (),
    )?;

    Ok(())
}

fn path_to_db() -> String {
    let dir_path = path_to_data_dir();

    let db_path = PathBuf::from(dir_path.join("rusty.db3"));

    db_path.to_str().unwrap().to_string()
}

pub fn create_data_dir() -> Result<()> {
    let dir_path = path_to_data_dir();

    create_dir_all(dir_path)
        .expect("Could not create Rusty Games Directory!");

    Ok(())
}

fn path_to_data_dir() -> PathBuf {
    let binding = BaseDirs::new().unwrap();
    let path = binding.data_local_dir();

    let dir_path = path.join("Rusty Games").join("data");

    dir_path
}

pub fn add_user_to_db() -> Result<()> {
    let path = path_to_db();
    let conn = Connection::open(path)?;

    let user = create_profile();

    conn.execute(
        "INSERT INTO user (username) VALUES (?1)",
        (&user.username,),
    )?;

    Ok(())
}

pub fn recall_db_data() -> Result<()> {
    let path = path_to_db();
    let conn = Connection::open(path)?;

    let mut stmt = conn.prepare("SELECT username FROM user")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            username: row.get(0)?,
        })
    })?;

    for user in user_iter {
        println!("Found user: {:?}", user?);
    }

    Ok(())
}