pub mod documentation;
pub mod calculator;
#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::sync::Mutex;
use libloading::Library;

use crate::calculator::{calc, library::MetaLib};


#[get("/")]
fn index() -> Template {
    Template::render("index", HashMap::<String, f64>::new())
}

#[launch]
fn rocket() -> _ {
    let calclib = unsafe {
        Library::new("./math/libmath.so").expect("DLL load failed")
    };

    rocket::build()
        .manage(MetaLib { calclib: Mutex::new(calclib) })
        .mount("/public", FileServer::from("static"))
        .mount("/", routes![index])
        .mount("/calc", routes![calc::index, calc::calculate])
        .mount("/docs", routes![documentation::index, documentation::posts])
        .attach(Template::fairing())
}
