#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod routes;
pub mod services;
pub mod models;
pub mod server;
