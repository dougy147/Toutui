use reqwest::Client;
use serde::{Deserialize, Serialize};
use color_eyre::eyre::{Result, Report};
use crate::db::crud::*;
use crate::db::database_struct::User;
use crate::api::libraries::get_all_libraries::*;
use crate::api::utils::collect_get_all_libraries::*;
use crate::app::App;
use crate::app::AppView;

#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct LoginResponse {
    user: UserInfo,
}

#[derive(Deserialize, Debug)]
struct UserInfo {
    token: String,
}

/// Login
/// https://api.audiobookshelf.org/#server

/// The login function takes a username and password, makes a POST request and returns a token.
pub async fn login(username: &str, password: &str, url: &str) -> Result<()> {
    let login_url = format!("{}/login", url);
    let client = Client::new();
    // Struct for data request
    let login_data = LoginRequest {
        username: username.to_string(),
        password: password.to_string(),
    };

    // Send POST request
    let response = client
        .post(login_url)
        .header("Content-Type", "application/json")
        .json(&login_data)
        .send()
        .await?;

    // Checking the status of the response
    if response.status().is_success() {
        let login_response: LoginResponse = response.json().await?;

        let all_libraries = get_all_libraries(login_response.user.token.as_str()).await?;
        let library_names = collect_library_names(&all_libraries).await;
        let media_types = collect_media_types(&all_libraries).await;
        let library_ids = collect_library_ids(&all_libraries).await;

                // init a new user
                let users = vec![
            User {
                server_adress: url.to_string(),
                username: username.to_string(),
                password: password.to_string(),
                token: login_response.user.token.clone(),
                is_default_usr: false,
                name_selected_lib: library_names[0].clone(), // by default we take the first library
                id_selected_lib: library_ids[0].clone(),
            }
        ];

        // insert the new user in database
        db_insert_usr(&users);

        Ok(()) 
    } else {
        Err(Report::new(std::io::Error::new(std::io::ErrorKind::Other, "Login failed"))) 
    }
}
