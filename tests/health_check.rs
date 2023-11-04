#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use reqwest::{Client, Proxy};
    use secrecy::ExposeSecret;
    use server_scaffold::{
        configuration::{get_configuration, DatabaseSettings},
        telemetry::{get_subscriber, init_subscriber},
    };
    use sqlx::{query, Connection, Executor, PgConnection, PgPool};
    use std::net::TcpListener;

    static TRACING: Lazy<()> = Lazy::new(|| {
        let default_filter_level = "info".into();
        let subscriber_name = "test".into();

        if std::env::var("TEST_LOG").is_ok() {
            let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
            init_subscriber(subscriber);
        } else {
            //  All of the output is directed into std::io::sink.
            let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
            init_subscriber(subscriber);
        }
    });

    struct TestApp {
        address: String,
        db_pool: PgPool,
    }

    async fn spawn_app() -> TestApp {
        Lazy::force(&TRACING);

        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
        let port = listener.local_addr().unwrap().port();
        let address = format!("http://localhost:{}", port);
        let mut configuration = get_configuration().expect("Failed to read configuration.");
        configuration.database.database_name = uuid::Uuid::new_v4().to_string();
        let db_pool = configure_database(&configuration.database).await;
        let server = server_scaffold::startup::run(listener, db_pool.clone())
            .expect("Failed to bind address.");
        let _ = tokio::spawn(server);

        TestApp { address, db_pool }
    }

    async fn configure_database(config: &DatabaseSettings) -> PgPool {
        let mut connection =
            PgConnection::connect(&config.connection_string_without_dbname().expose_secret())
                .await
                .expect("Failed to connect to database.");

        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
            .await
            .expect("Failed to create database");

        let db_pool = PgPool::connect(&config.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres");

        sqlx::migrate!("./migrations")
            .run(&db_pool)
            .await
            .expect("Failed to migrate the database");

        db_pool
    }

    fn build_client() -> Client {
        Client::builder()
            .proxy(Proxy::custom(|_| Some("")))
            .build()
            .expect("Failed to build a reqwest client.")
    }

    #[tokio::test]
    async fn health_check_works() {
        let app = spawn_app().await;
        let client = build_client();

        let response = client
            .get(&format!("{}/health_check", &app.address))
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
        assert_eq!(response.content_length(), Some(0));
    }

    #[tokio::test]
    async fn subscribe_returns_a_200_for_valid_form_data() {
        let app = spawn_app().await;
        let client = build_client();
        let connection = get_configuration().expect("Failed to read configuration.");
        let connection_string = connection.database.connection_string();

        let body = "name=Chris%20topher&email=any_thing%40gmail.com";
        let response = client
            .post(format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());

        let pool = PgPool::connect(&connection_string.expose_secret())
            .await
            .expect("Failed to connect to pool.");

        query!("SELECT email, name FROM subscriptions")
            .fetch_one(&pool)
            .await
            .expect("");
    }

    #[tokio::test]
    async fn subscribe_returns_a_400_when_data_is_missing() {
        // Arrange
        let app = spawn_app().await;
        let client = build_client();
        let test_cases = vec![
            ("name=le%20guin", "missing the email"),
            ("email=ursula_le_guin%40gmail.com", "missing the name"),
            ("", "missing both name and email"),
        ];

        for (invalid_body, error_message) in test_cases {
            // Act
            let response = client
                .post(format!("{}/subscriptions", &app.address))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(invalid_body)
                .send()
                .await
                .expect("Failed to execute request.");

            // Assert
            assert_eq!(
                400,
                response.status().as_u16(),
                // Additional customised error message on test failure
                "The API did not fail with 400 Bad Request when the payload was {}.",
                error_message
            );
        }
    }
}
