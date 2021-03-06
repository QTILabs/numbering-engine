use dotenv::vars as get_all_vars;
use std::collections::HashMap;
use std::net::Ipv4Addr;

pub(crate) struct AppConfig {
    pub(crate) bind_ip: Ipv4Addr,
    pub(crate) bind_port: u16,
}

impl AppConfig {
    pub(crate) fn load_from_env() -> Self {
        let all_vars: HashMap<String, String> = get_all_vars().collect();
        let bind_ip;
        let bind_port;

        if !all_vars.contains_key("BIND_IP") {
            bind_ip = "127.0.0.1".parse().unwrap();
        } else {
            bind_ip = all_vars.get("BIND_IP").unwrap().parse().unwrap();
        }

        if !all_vars.contains_key("BIND_PORT") {
            bind_port = 8080;
        } else {
            bind_port = all_vars.get("BIND_PORT").unwrap().parse().unwrap();
        }

        Self { bind_ip, bind_port }
    }
}
