mod util;
mod parser;
mod model;
mod controller;
mod service;

pub fn receive(data: Option<&str>) {
    controller::post(data);
}