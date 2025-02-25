mod argsets;
mod command;
mod constants;
mod helpers;
mod interfaces;
mod node_mgmt;

use anyhow::{anyhow, Result};
use env_logger::Env;

use constants::{defaults, envvars};
use helpers::load_dotenv;

const CMD_INIT: &str = "init";
const CMD_KVS_GET: &str = "kvs-get";
const CMD_KVS_SET: &str = "kvs-set";
const CMD_MQTT_SUB: &str = "mqtt-sub";
const CMD_WEB_UI: &str = "web-ui";

fn main() -> Result<()> {
    load_dotenv();
    env_logger::Builder::from_env(
        Env::default().filter_or(envvars::LOG_LEVEL, defaults::LOG_LEVEL),
    )
    .init();

    let mut args = pico_args::Arguments::from_env();
    match args.subcommand()?.as_deref() {
        Some(CMD_INIT) => command::init(),
        Some(CMD_KVS_GET) => command::kvs_get(argsets::KvsGetArgs {
            key: args.free_from_str()?,
        }),
        Some(CMD_KVS_SET) => command::kvs_set(argsets::KvsSetArgs {
            key: args.free_from_str()?,
            value: args.free_from_str()?,
        }),
        Some(CMD_MQTT_SUB) => { command::mqtt_sub(); Ok(()) },
        Some(CMD_WEB_UI) => { command::web_ui(); Ok(()) },
        _ => Err(anyhow!(
            "Subcommand must be one of '{CMD_INIT}', '{CMD_KVS_GET}', '{CMD_KVS_SET}'"
        )),
    }
}
