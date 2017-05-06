pub fn sometimes_crash(magic: &str) {
    match magic {
        "crash" | "halt" | "fire" => panic!("at the disco"),
        _ => {},
    }
}
