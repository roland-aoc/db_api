mod controller;
mod infrastructure;
mod model;
mod repository;
mod service;
mod util;

pub fn receive(data: String) -> Result<(), Box<dyn std::error::Error>> {
    controller::post(data)
}
