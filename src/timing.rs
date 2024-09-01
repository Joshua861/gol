use crate::prelude::*;
use lazy_static::lazy_static;
use std::{fmt::Display, sync::RwLock};

lazy_static! {
    pub static ref TIMERS: RwLock<Vec<Timer>> = RwLock::new(Vec::new());
}

pub fn add_timer(name: String, time: usize) {
    TIMERS.write().unwrap().push(Timer { name, time })
}

pub fn get_timers() -> Vec<Timer> {
    TIMERS.read().unwrap().clone()
}

pub fn clear_timers() {
    TIMERS.write().unwrap().clear()
}

#[macro_export]
macro_rules! time {
    ($name:expr, $block:block) => {
        {
            #[cfg(debug_assertions)]
            let timer = std::time::Instant::now();
            $block
            #[cfg(debug_assertions)]
            $crate::timing::add_timer($name.to_string(), timer.elapsed().as_micros() as usize);
        }
    };
}

#[derive(Clone)]
pub struct Timer {
    name: String,
    time: usize,
}

impl Display for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}μs", self.name, fmt_num(self.time))
    }
}
