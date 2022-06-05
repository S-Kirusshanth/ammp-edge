use handlebars::Handlebars;
use kvstore::KVDb;
use rouille::Response;
use serde_json::value::{Map, Value as Json};

use crate::constants::keys;

fn prepare_data(kvs: &KVDb) -> Map<String, Json> {
    let mut data = Map::new();
    
    data.insert("node_id".to_string(), kvs.get(keys::NODE_ID).unwrap().unwrap());

    data
}

pub fn index(kvs: &KVDb) -> Response {
    let mut handlebars = Handlebars::new();

    // handlebars.register_helper("format", Box::new(format_helper));
    // handlebars.register_helper("ranking_label", Box::new(rank_helper));
    // // handlebars.register_helper("format", Box::new(FORMAT_HELPER));

    let data = prepare_data(kvs);

    handlebars
        .register_template_file("template", "./src/web_ui/render_files/index.hbs")
        .unwrap();

    // Builds a `Response` object that contains the "hello world" text.
    rouille::Response::html(handlebars.render("template", &data).unwrap())
}
