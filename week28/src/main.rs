use chrono::{Local, Utc};

fn main() {
    let utc = Utc::now();
    let local = Local::now();
    print!("{}", utc); // e.g. `2014-11-28T12:45:59.324310806Z`
    print!("{}", local) // e.g. `2014-11-28T12:45:59.324310806+00:00`
}
