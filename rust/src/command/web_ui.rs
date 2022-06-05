use anyhow::Result;

use crate::web_ui::router;

pub fn web_ui() -> Result<()> {
    println!("Now listening on localhost:8000");

    // The `start_server` starts listening forever on the given address.
    rouille::start_server("localhost:8000", move |request| {
        router(request)
    });
}