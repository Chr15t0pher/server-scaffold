use once_cell::sync::Lazy;
use reqwest::{Client, Proxy};
use secrecy::ExposeSecret;
use server_scaffold::{
    configuration::{get_configuration, DatabaseSettings},
    startup::{get_database_pool, Application},
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::{Connection, Executor, PgConnection, PgPool};

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

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = uuid::Uuid::new_v4().to_string();

    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.database.database_name = uuid::Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };

    configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let address = format!("http://127.0.0.1:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address: address,
        db_pool: get_database_pool(&configuration),
    }
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

pub fn build_client() -> Client {
    Client::builder()
        .proxy(Proxy::custom(|_| Some("")))
        .build()
        .expect("Failed to build a reqwest client.")
}