mod controller;
mod infrastructure;
mod model;
mod repository;
mod service;
mod util;

// pub async fn receive(data: String) -> Result<(), Box<dyn std::error::Error>> {
pub async fn receive() {
    controller::post().await
}
