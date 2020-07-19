
use std::env;

mod config;

use config::player::game;

fn emit_banner() {
    for _i in 1..20 {
        println!("");
    }
    println!("----------------------------------");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_file = &args[1];
    let (mut table, config) = config::build_from_json(config_file);

    emit_banner();

    println!("TRACER config: {:?}", config);
    println!("TRACER table: {}", table);
    game::play_tourney(&config, &mut table);
    println!("Ready.");
}
