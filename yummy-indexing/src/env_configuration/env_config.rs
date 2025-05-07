use crate::common::*;

#[doc = "Function to globally initialize the 'INDEX_LIST_PATH' variable"]
pub static INDEX_LIST_PATH: once_lazy<String> = once_lazy::new(|| {
    env::var("INDEX_LIST_PATH").expect("[ENV file read Error] 'INDEX_LIST_PATH' must be set")
});

#[doc = "Function to globally initialize the 'SYSTEM_CONFIG_PATH' variable"]
pub static SYSTEM_CONFIG_PATH: once_lazy<String> = once_lazy::new(|| {
    env::var("SYSTEM_CONFIG_PATH").expect("[ENV file read Error] 'SYSTEM_CONFIG_PATH' must be set")
});

#[doc = "환경마다 env 를 변경해주는 코드"]
pub fn load_env() {
    let env_type: String = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());

    let env_file = match env_type.as_str() {
        "prod" => ".env.prod",
        "dev" => ".env.dev",
        _ => ".env.dev",
    };

    from_filename(env_file).ok();
    info!("Loaded environment file: {}", env_file);
}
