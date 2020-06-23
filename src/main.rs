mod log;

use log::{trace, warning, info, error};

fn main() {
    trace("Log for TRACE test");

    warning("Log for WARNING test");

    info("Log for INFO test");

    error("Log for ERROR test");
}
