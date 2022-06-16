use crate::web_ui::router;

pub fn web_ui() {
    println!("Now listening on 0.0.0.0:8000");

    // The `start_server` starts listening forever on the given address.
    rouille::start_server("0.0.0.0:8000", move |request| {
        router(request)
    });
}