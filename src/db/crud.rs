use rusqlite::{params, Connection, Result};
use crate::db::database_struct::User;
use crate::db::database_struct::ListeningSession;
use crate::utils::pop_up_message::*;
use std::io::stdout;
use log::{info, error};
use std::path::PathBuf;

pub fn get_listening_session() -> Result<Option<ListeningSession>> {

    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    let err_message = "Error connecting to the database.";

    if let Ok(conn) = Connection::open(db_path) {
        let mut stmt = conn.prepare(
            "SELECT id_session, id_item, current_time_playback, duration, is_finished, id_pod
             FROM listening_session
             LIMIT 1",
        )?;

        let mut rows = stmt.query(params![])?;

        if let Some(row) = rows.next()? {
            let session = ListeningSession {
                id_session: row.get(0)?,
                id_item: row.get(1)?,
                current_time: row.get(2)?,
                duration: row.get(3)?,
                is_finished: row.get(4)?,
                id_pod: row.get(5)?,
            };
            return Ok(Some(session));
        }
    } else {
        let mut stdout = stdout();
        let _ = pop_message(&mut stdout, 3, err_message);
        error!("[get_listening_session] {}", err_message);
    }

    Ok(None)
}

// insert data into `listening_session` table
pub fn insert_listening_session(
    id_session: String,
    id_item: String,
    current_time: u32,
    duration: String,
    id_pod: String,
) -> Result<()> {

    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    let err_message = "Error connecting to the database.";

    if let Ok(conn) = Connection::open(db_path) {
        conn.execute("DELETE FROM listening_session", params![])?;
        conn.execute(
            "INSERT INTO listening_session (id_session, id_item, current_time_playback, duration, is_finished, id_pod) 
             VALUES (?1, ?2, ?3, ?4, 0, ?5)",
            params![id_session, id_item, current_time, duration, id_pod],
        )?;
    } else {
        let mut stdout = stdout();
        let _ = pop_message(&mut stdout, 3, err_message);
        error!("[insert_listening_session] {}", err_message);
    }

    Ok(())
}

// Update current_time (for `listening_session` table)
pub fn update_current_time(value: u32, id_session: &str) -> Result<()> {

    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    let err_message = "Error connecting to the database.";

    if let Ok(conn) = Connection::open(db_path) {

        conn.execute(
            "UPDATE listening_session SET current_time_playback = ?1 WHERE id_session = ?2",
            params![value, id_session],
        )?;
    } else {
        let mut stdout = stdout();
        let _ = pop_message(&mut stdout, 3, err_message);
        error!("[update_current_time] {}", err_message);
    }

    Ok(())
}

// Update is_finished (for `listening_session` table)
pub fn update_is_finished(value: &str, id_session: &str) -> Result<()> {

    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    let err_message = "Error connecting to the database.";

    if let Ok(conn) = Connection::open(db_path) {

        conn.execute(
            "UPDATE listening_session SET is_finished = ?1 WHERE id_session = ?2",
            params![value, id_session],
        )?;
    } else {
        let mut stdout = stdout();
        let _ = pop_message(&mut stdout, 3, err_message);
        error!("[update_is_finished] {}", err_message);
    }

    Ok(())
}

// Delete an user
pub fn delete_user(username: &str) -> Result<()> {
    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    let message = format!("User '{}' deleted. Please restart the app to apply the changes.", &username);
    let err_message = "Error connecting to the database.";
    if let Ok(conn) = Connection::open(db_path) {

        let rows_deleted = conn.execute(
            "DELETE FROM users WHERE username = ?1",
            params![username],
        )?;

        if rows_deleted > 0 {
            let mut stdout = stdout();
            let _ = pop_message(&mut stdout, 3, message.as_str());
            info!("[delete_user] User deleted.");
        } else {
            //println!("No user found with this username '{}'.", username);
        }
    } else {
        let mut stdout = stdout();
        let _ = pop_message(&mut stdout, 3, err_message);
        error!("[delete user] {}", err_message);
    }

    Ok(())
}

// Update is_loop_break
pub fn update_is_loop_break(value: &str, username: &str) -> Result<()> {

    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    let err_message = "Error connecting to the database.";

    if let Ok(conn) = Connection::open(db_path) {

        conn.execute(
            "UPDATE users SET is_loop_break = ?1 WHERE username = ?2",
            params![value, username],
        )?;
    } else {
        let mut stdout = stdout();
        let _ = pop_message(&mut stdout, 3, err_message);
        error!("[update_is_loop_break] {}", err_message);
    }

    Ok(())
}


// get is_loop_break
pub fn get_is_loop_break(username: &str) -> String {
    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    let conn = match Connection::open(db_path) {
        Ok(c) => c,
        Err(_) => return String::from("Error: unable open database"),
    };

    let mut stmt = match conn.prepare("SELECT is_loop_break FROM users WHERE username = ?1") {
        Ok(s) => s,
        Err(_) => return String::from("Error to prepare reqwest"),
    };

    match stmt.query_row(params![username], |row| row.get::<_, String>(0)) {
        Ok(id) => id,
        Err(_) => String::from("No db found"),
    }
}

// Update is_vlv_launched_first_time
pub fn update_is_vlc_launched_first_time(value: &str, username: &str) -> Result<()> {

    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    let err_message = "Error connecting to the database.";

    if let Ok(conn) = Connection::open(db_path) {

        conn.execute(
            "UPDATE users SET is_vlc_launched_first_time = ?1 WHERE username = ?2",
            params![value, username],
        )?;
    } else {
        let mut stdout = stdout();
        let _ = pop_message(&mut stdout, 3, err_message);
        error!("[is_vlc_launched_first_time] {}", err_message);
    }

    Ok(())
}
// get is_vlc_launched_first_time
pub fn get_is_vlc_launched_first_time(username: &str) -> String {
    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    let conn = match Connection::open(db_path) {
        Ok(c) => c,
        Err(_) => return String::from("Error: unable open database"),
    };

    let mut stmt = match conn.prepare("SELECT is_vlc_launched_first_time FROM users WHERE username = ?1") {
        Ok(s) => s,
        Err(_) => return String::from("Error to prepare reqwest"),
    };

    match stmt.query_row(params![username], |row| row.get::<_, String>(0)) {
        Ok(id) => id,
        Err(_) => String::from("No db found"),
    }
}
// Update id_selected_lib
pub fn update_id_selected_lib(id_selected_lib: &str, username: &str) -> Result<()> {

    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    let message = "The library has been updated. Please refresh the app to apply the changes.";
    let err_message = "Error connecting to the database.";
    if let Ok(conn) = Connection::open(db_path) {

        conn.execute(
            "UPDATE users SET id_selected_lib = ?1 WHERE username = ?2",
            params![id_selected_lib, username],
        )?;
        let mut stdout = stdout();
        let _ = pop_message(&mut stdout, 3, message);
        info!("[update_id_selected_lib] The library has been updated");

    } else {
        let mut stdout = stdout();
        let _ = pop_message(&mut stdout, 3, err_message);
        error!("[update_id_selected_lib] {}", err_message);
    }

    Ok(())
}

// update default user 
pub fn update_default_user(conn: &Connection, username: &str) -> Result<()> {
    // Mark all user as 0 by default
    conn.execute(
        "UPDATE users SET is_default_usr = 0",
        [],
    )?;

    // Put the desired user as default
    conn.execute(
        "UPDATE users SET is_default_usr = 1 WHERE username = ?1",
        params![username],
    )?;

    Ok(())
}

// Insert user in database
pub fn db_insert_usr(users : &Vec<User>)  -> Result<()> {   
    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    let conn = Connection::open(db_path)?;
    for user in users {
        conn.execute(
            "INSERT OR REPLACE INTO users (username, server_address, token, is_default_usr, name_selected_lib, id_selected_lib, is_loop_break, is_vlc_launched_first_time) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
            user.username,
            user.server_address,
            user.token,
            if user.is_default_usr { 1 } else { 0 },
            user.name_selected_lib,
            user.id_selected_lib,
            user.is_loop_break,
            user.is_vlc_launched_first_time
            ],
        )?;
    }
    Ok(())
}


// Select default user
pub fn select_default_usr() -> Result<Vec<String>> {
    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    let conn = Connection::open(db_path)?;

    let mut stmt = conn.prepare(
        "SELECT username, server_address, token, is_default_usr, name_selected_lib, id_selected_lib, is_loop_break, is_vlc_launched_first_time
         FROM users WHERE is_default_usr = 1 LIMIT 1"
    )?;


    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            username: row.get(0)?,
            server_address: row.get(1)?,
            token: row.get(2)?,
            is_default_usr: row.get::<_, i32>(3)? != 0,  // convert 0/1 in bool
            name_selected_lib: row.get(4)?,
            id_selected_lib: row.get(5)?,
            is_loop_break: row.get(6)?,
            is_vlc_launched_first_time: row.get(7)?,
        })
    })?;

    let mut result = Vec::new();

    for user in user_iter {
        match user {
            Ok(user) => {
                result.push(user.username);
                result.push(user.server_address);
                result.push(user.token);
                result.push(user.is_default_usr.to_string());
                result.push(user.name_selected_lib);
                result.push(user.id_selected_lib);
                result.push(user.is_loop_break);
                result.push(user.is_vlc_launched_first_time);
            }
            Err(e) => {
                println!("Error occurred: {}", e);
                //return Err(rusqlite::Error::FromSqlConversionFailure(0, "Failed to map user".to_string()));
            }
        }
    }

    if result.is_empty() {
        //println!("No default user found.");
    }

    Ok(result)  
}

// Init db and table if not exist
pub fn init_db() -> Result<()> {
    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    // Open or create db
    let conn = Connection::open(db_path)?;

    //Create table `users` if there is none 
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
                username TEXT PRIMARY KEY,
                server_address TEXT NOT NULL,
                token TEXT NOT NULL,
                is_default_usr INTEGER NOT NULL DEFAULT 0,
                name_selected_lib TEXT NOT NULL,
                id_selected_lib TEXT NOT NULL,
                is_loop_break TEXT NOT NULL,
                is_vlc_launched_first_time TEXT NOT NULL
            )",
        [],
    )?;

    //Create table `listening_session` if there is none 
    conn.execute(
        "CREATE TABLE IF NOT EXISTS listening_session (
            id_session TEXT PRIMARY KEY,
            id_item TEXT NOT NULL,
            current_time_playback INTEGER NOT NULL,
            duration TEXT NOT NULL,
            is_finished INTEGER NOT NULL DEFAULT 0,
            id_pod TEXT NOT NULL
            )",
        [],
    )?;


    Ok(())
}


