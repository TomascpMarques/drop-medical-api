use std::collections::HashMap;

use drop_medical_api::{
    configuration::{self, Settings},
    database, setup_app_router,
};

use once_cell::sync::Lazy;
use reqwest::StatusCode;
use serde_json::json;
use sqlx::PgPool;
use tracing::info;

#[derive(getset::Getters)]
#[getset(get = "pub")]
pub struct TestApp {
    address: String,
}

static TRACING: Lazy<()> = Lazy::new(|| {
    // Server tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
});

#[tokio::test]
async fn register_user() {
    let test_app = spawn_app().await;

    let address = format!("http://{}/api/users/register", test_app.address());
    let client = reqwest::Client::new();

    let reg_json = json!({
        "name": "John Doe",
        "email": "john.doe@e.mail.com",
        "password": "super_secret"
    });

    let req = client
        .post(address.as_str())
        .json(&reg_json)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(req.status(), StatusCode::OK);
    assert_eq!(req.content_length(), Some(0));
}

#[tokio::test]
async fn login_user() {
    let test_app = spawn_app().await;

    let address = format!("http://{}/api/users/register", test_app.address());
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to setup cookie store for reqwest in login_user.");

    let reg_json = json!({
        "name": "John Doe",
        "email": "john.doe@e.mail.com",
        "password": "super_secret"
    });

    let req = client
        .post(address.as_str())
        .json(&reg_json)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(req.status(), StatusCode::OK);
    assert_eq!(req.content_length(), Some(0));
    assert!(req.headers().get("set-cookie").is_some());

    let address = format!("http://{}/api/users/login", test_app.address());

    let mut params = HashMap::new();
    params.insert("email", "john.doe@e.mail.com");
    params.insert("password", "super_secret");

    let req = client
        .post(address.as_str())
        .form(&params)
        .send()
        .await
        .expect("Failed to send login request");

    assert_eq!(req.status(), StatusCode::OK);
    assert_eq!(req.content_length(), Some(2));
}

#[tokio::test]
async fn register_dropper() {
    let test_app = spawn_app().await;

    let address = format!("http://{}/api/users/register", test_app.address());
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to setup cookie store for reqwest in login_user.");

    let reg_json = json!({
        "name": "John Doe",
        "email": "john.doe@e.mail.com",
        "password": "super_secret"
    });

    let req = client
        .post(address.as_str())
        .json(&reg_json)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(req.status(), StatusCode::OK);
    assert_eq!(req.content_length(), Some(0));
    assert!(req.headers().get("set-cookie").is_some());

    let address = format!("http://{}/api/users/login", test_app.address());

    let mut params = HashMap::new();
    params.insert("email", "john.doe@e.mail.com");
    params.insert("password", "super_secret");

    let req = client
        .post(address.as_str())
        .form(&params)
        .send()
        .await
        .expect("Failed to send login request");

    assert_eq!(req.status(), StatusCode::OK);
    assert_eq!(req.content_length(), Some(2));

    let address = format!("http://{}/api/droppers/register", test_app.address());

    let mut params = HashMap::new();
    params.insert("n", "SupaDroper");

    let req = client
        .post(address.as_str())
        .form(&params)
        .send()
        .await
        .expect("Failed to send dropper register request");

    dbg!(&req.headers());
    dbg!(req.text().await.unwrap());
    // assert_eq!(req.status(), StatusCode::OK);
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let config = configuration::get_config().unwrap();
    let db_pool = configure_database(&config).await;

    // DB setup
    let app = setup_app_router(&db_pool).expect("Failed to setup app");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind address");

    let address = format!("127.0.0.1:{}", listener.local_addr().unwrap().port());

    let _ = tokio::task::spawn(async { axum::serve(listener, app).await });

    info!("Server listening on: http://{address}/");

    TestApp { address }
}

/// # About
/// Creates a mock database with no data, used for testing.
/// In order for each test to have new clean database.
pub async fn configure_database(config: &Settings) -> PgPool {
    let db_name = format!("test_{}", uuid::Uuid::now_v7());
    let db_name = db_name.replace("-", "");
    let db_name = db_name.as_str();

    let db_url = database::connection_str_without_db(config);
    let connection_pool = PgPool::connect(db_url.as_str())
        .await
        .expect("Failed to connect to Postgres (created for tests)");

    let query = format!("create database {}", db_name);
    let query = query.trim();

    sqlx::query(query)
        .execute(&connection_pool)
        .await
        .expect("Failed to create TEST database.");

    // Database migration
    let mut db_url = database::connection_str_without_db(config);
    db_url.push_str(db_name);
    let db_url = db_url.as_str();

    let connection_pool = PgPool::connect(db_url)
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("database/migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to run migrations on database.");

    connection_pool
}

/// # Category: **TESTING**
/// # About:
/// The test suit creaates one database for each test, due to isolation concerns.
/// Therefore, in order not to cripple the performance of the configured data base,
/// in the conf file _(configuration.yaml)_ we drop all the databases used in tests.
#[tokio::test]
#[ignore = "Clean up should only RUN AFTER ALL LOCAL TESTS !!!"]
async fn clean_up_local_dbs() {
    // Database dropping used in tests
    let config = configuration::get_config().expect("Failed to read configs");

    let pg_pool = sqlx::PgPool::connect(database::connection_str_without_db(&config).as_str())
        .await
        .expect("Failed to establish db connection");

    let test_dbs = sqlx::query!(
        r#"
        SELECT datname FROM pg_database
        WHERE datistemplate = false AND datname LIKE 'test_%';
        "#
    )
    .fetch_all(&pg_pool)
    .await
    .expect("Failed to get test databases name");

    for db_name in test_dbs {
        let db_name = db_name.datname;
        let sql = format!(r#"DROP DATABASE IF EXISTS "{}""#, db_name);

        sqlx::query(sql.as_str())
            .execute(&pg_pool)
            .await
            .expect("FAILED TO DROP TEST DATABASE!!!");
    }
}
