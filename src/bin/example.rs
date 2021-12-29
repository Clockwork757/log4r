use std::env::args;

use log::info;
use log4r::{init, SECURE_EXTENSION_PREFIX};

pub fn main() {
    init();
    let args: String = args().skip(2).collect::<Vec<_>>().join(" ");
    info!("cool log example :) {} {}", SECURE_EXTENSION_PREFIX, args);
}
