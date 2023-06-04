use anyhow::Result;

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub title: String,
    pub max_mem_log_size: usize,
}

impl Config {
    pub fn new() -> Result<Config> {
        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()?;
        let title = std::env::var("TITLE").unwrap_or_else(|_| "axum-log-stream".to_string());
        let max_mem_log_size = std::env::var("MAX_MEM_LOG_SIZE")
            .unwrap_or_else(|_| "1000000".to_string())
            .parse()?;

        Ok(Config {
            host,
            port,
            title,
            max_mem_log_size,
        })
    }
}
