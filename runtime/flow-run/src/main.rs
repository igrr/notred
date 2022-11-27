use clap::arg;
use env_logger;
use notred::*;
use std::env;
use std::fs;
use std::time::Duration;

fn main() {
    env_logger::init();

    let app = clap::app_from_crate!().arg(arg!(-f --flow <NAME>));
    let matches = app.get_matches();
    let flow_name = matches.value_of("flow").expect("Missing --flow argument");

    let flow_json = fs::read_to_string(flow_name).expect("Failed to read input flow file");

    let mut flow = notred::FlowState::new(flow_json.as_str()).expect("Failed to build the flow");

    loop {
        let res = flow.run_once(Duration::from_millis(100));
        if let Ok(()) = res {
            continue;
        }
        if let Err(Error::Terminate(_)) = res {
            break;
        }
        if let Err(Error::Timeout(_)) = res {
            continue;
        }
        res.expect("Failure while running flow");
    }
}
