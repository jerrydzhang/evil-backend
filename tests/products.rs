#[cfg(test)]
mod user_tests {
    use std::collections::HashMap;

    fn get_test_user_token() -> String {
        dotenv::dotenv().ok();

        if std::env::var("DATABASE_URL").unwrap() != "postgres://evil_admin:evil@localhost/test_db" {
            panic!("DATABASE_URL must be set to postgres://evil_admin:evil@localhost/test_db");
        }

        let username = std::env::var("TEST_USER_USERNAME").unwrap();
        let password = std::env::var("TEST_USER_PASSWORD").unwrap();
        let audience = std::env::var("AUTH0_AUDIENCE").unwrap();
        let client_id = std::env::var("AUTH0_CLIENT_ID").unwrap();
        let client_secret = std::env::var("AUTH0_CLIENT_SECRET").unwrap();
        let url = std::env::var("AUTH0_DOMAIN").unwrap();

        let mut params = HashMap::new();
        params.insert("grant_type", "password");
        params.insert("username", &username);
        params.insert("password", &password);
        params.insert("audience", &audience);
        params.insert("scope", "read:sample");
        params.insert("client_id", &client_id);
        params.insert("client_secret", &client_secret);
        
        let claim = reqwest::blocking::Client::new()
            .post(&format!("{}/oauth/token", url))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .unwrap()
            .json::<serde_json::Value>()
            .unwrap();

        claim["access_token"].as_str().unwrap().to_string()
    }
}