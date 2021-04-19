use chrono;

// INFO, DEBUG, ERROR


const log_level: &str = std::env!("LOG_LEVEL");


fn log_level_i(level: &str) -> i32 {
    match level {
        "DEBUG" => 0,
        "INFO" => 1,
        "ERROR" => 2,
        _ => -1
    }
}

macro_rules! create_function {
    ($func_name:ident, $current_level:expr) => {
        pub fn $func_name(msg: &str) {
            if log_level_i($current_level) >= log_level_i(log_level) {
                let out = format!("{} ({}) -- {}",
                    chrono::offset::Utc::now(), $current_level, msg);

                if $current_level == "ERROR" {
                    eprintln!("{}", out);
                }
                else {
                    println!("{}", out);
                }
                
            }
        }
    };
}



create_function!(info, "INFO");
create_function!(debug, "DEBUG");
create_function!(error, "ERROR");

