#![allow(non_snake_case)]

use log::{
    // info,
    // trace,
    debug,
    // warn,
};
use std::{
    time::Duration,
};

///
/// Interval provides precise cycle period. 
/// It can be configured by calling `Interval::new(period)` with constatnt `period` in seconds
/// method `wait()` should be called cyclically
/// the timestamp at the end of wait will be stored into `previous`
/// on the next call `wait()` current wait delay will be calculated as: `now` - `previous`
pub struct Interval {
    pub period: f64,
    sleepDelta: Duration,
    waitInterval: u128,
    previous: u128,
    // sleeped: u128,
    limit: u128,
    start: std::time::Instant,
}
impl Interval {
    /// 
    /// `period`, seconds - looping interval
    pub fn new(period: f64) -> Self {
        let interval = Duration::from_secs_f64(period);
        let sleepDelta = Duration::from_secs_f64(period * 0.001);
        let waitInterval = interval.as_nanos();
        debug!("interval : {:?}", interval);
        debug!("interval ms : {:?}", interval.as_millis());
        debug!("interval mcs: {:?}", interval.as_micros());
        debug!("interval ns : {:?}", interval.as_nanos());
        debug!("sleepDelta : {:?}", sleepDelta);
        debug!("waitInterval : {:?}ns\n", waitInterval);
        Self {
            period,
            sleepDelta,
            waitInterval,
            previous: 0,
            // sleeped: 0,
            limit: 0,
            start: std::time::Instant::now(),
        }
    }
    ///
    /// Waits until `period` exceeded
    pub fn wait(&mut self) {
        // let mut times = vec![];
        self.previous = self.start.elapsed().as_nanos();
        self.limit = self.waitInterval + self.previous;
        while self.start.elapsed().as_nanos() < self.limit {
            std::thread::sleep(self.sleepDelta);
            // self.sleeped += 1;
        }
        // times.push([start.elapsed().as_nanos() - previous, sleeped]);
        // debug!("elapsed : {:?}ns", [self.start.elapsed().as_nanos() - self.previous, self.sleeped]);
        self.previous = self.start.elapsed().as_nanos();
        // for t in times {
        //     trace!("at: {:?}", t);
        // }
    }
}
