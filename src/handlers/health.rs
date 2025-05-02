use reqwest::StatusCode;

pub async fn health_check() -> Result<&'static str, StatusCode> {
    Ok("Online")
}
