#[macro_use]
extern crate log;

mod tasks;

use std::thread;
use std::time::Duration;
use std::vec::Vec;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc
};
use syslog::{Facility, Formatter3164, BasicLogger};
use log::LevelFilter;
use blinkt::Blinkt;
use simple_signal::{self, Signal};

use crate::tasks::Task;
use crate::tasks::OnlineTask;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "blinkt_monitor".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
            .map(|()| log::set_max_level(LevelFilter::Info))?;

    let mut tasks = Vec::<(Task, i32)>::new();
    tasks.push((Box::new(OnlineTask), 3000));

    let mut tasks_cooldown = Vec::<i32>::new();
    for (_, _) in &tasks {
        tasks_cooldown.push(0);
    }

    let brightness = 0.1;
    let mut blinkt = Blinkt::new()?;
    blinkt.set_clear_on_drop(true);
    blinkt.set_pixel_rgbb(7, 0, 10, 0, brightness);
    blinkt.show()?;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    simple_signal::set_handler(&[Signal::Int, Signal::Term], move |_signals| {
        r.store(false, Ordering::SeqCst);
    });

    let interval = 100;
    while running.load(Ordering::SeqCst) {
        for (task_index, (task, task_interval)) in (&tasks).into_iter().enumerate() {
            if tasks_cooldown[task_index] <= 0 {
                task(&mut blinkt, brightness);
                tasks_cooldown[task_index] = *task_interval;
            }
            tasks_cooldown[task_index] -= interval;
        }
        blinkt.show()?;
        thread::sleep(Duration::from_millis(interval as u64));
    }

    Ok(())
}
