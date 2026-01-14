
extern crate rocket;
use std::sync::Mutex;
use libloading::Library;
pub struct MetaLib {
    pub calclib: Mutex<Library>
}