pub async fn project_version() -> String {
    format!("Project Version: {}", env!("CARGO_PKG_VERSION"))
}