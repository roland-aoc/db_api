mod util;
mod model;
mod controller;
mod service;
mod repository;
mod infrastructure;

pub fn receive(data: String) -> Result<(), Box<dyn std::error::Error>> {
    controller::post(data)
}
