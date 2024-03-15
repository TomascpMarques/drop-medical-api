#[derive(serde::Deserialize, getset::Getters, getset::Setters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
pub struct Settings {
    env: Environment,
    application: ApplicationSettings,
    database: DatabaseSettings,
}

#[derive(serde::Deserialize, getset::Getters, getset::Setters)]
#[getset(get = "pub", set = "pub")]
pub struct DatabaseSettings {
    uri: String,
}

#[derive(serde::Deserialize, getset::Getters, getset::Setters)]
#[getset(get = "pub", set = "pub")]
pub struct ApplicationSettings {
    host: String,
    port: u16,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Cant retrieve current dir");
    let configuration_dir = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENV");
    let environment_filename = environment.file_name();

    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_dir.join("base.toml")))
        .add_source(config::File::from(
            configuration_dir.join(environment_filename),
        ))
        .set_override("env", environment.as_str())?
        .build()?;

    settings.try_deserialize::<Settings>()
}

#[derive(serde::Deserialize)]
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn file_name(&self) -> &'static str {
        match self {
            Environment::Local => "local.toml",
            Environment::Production => "production.toml",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Environment::Local),
            "production" => Ok(Environment::Production),
            _ => Err("Invalid environment specified".into()),
        }
    }
}
