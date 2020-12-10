mod config;

use log::error;
use simple_logger::SimpleLogger;
use std::{
    os::unix::process::CommandExt,
    process::{Child, Command},
    thread, time,
};

fn exited(child: &mut Child) -> bool {
    match child.try_wait() {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(e) => {
            error!("failed checking return code for process: {}", e);
            true
        }
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();

    let config = config::CONFIG;

    let mut childs = Vec::new();
    for process in config.processes.iter() {
        let path = process.path.to_string();
        let args: Vec<String> = process.params.iter().map(|p| p.to_string()).collect();
        let gid = process.gid as u32;
        let uid = process.uid as u32;
        childs.push(
            Command::new(path)
                .args(args)
                .gid(gid)
                .uid(uid)
                .spawn()
                .expect("failed spawning"),
        );
    }

    // wait untill one child finishes execution then kill all of the rest
    loop {
        let finished = childs.iter_mut().any(|child| exited(child));
        if finished {
            // one process exited close all processes and exit
            for child in childs.iter_mut() {
                if !exited(child) {
                    child
                        .kill()
                        .expect(format!("failed killing {}", child.id()).as_str());
                }
            }
            break;
        } else {
            // all processes didn't exit so sleep a little
            thread::sleep(time::Duration::from_millis(100));
        }
    }
}
