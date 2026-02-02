use std::time::Duration;

pub struct Backoff {
    current: Duration,
    max: Duration,
    factor: f64,
}

impl Backoff {
    pub fn new(initial: Duration, max: Duration, factor: f64) -> Self {
        Self {
            current: initial,
            max,
            factor,
        }
    }

    pub fn next_delay(&mut self) -> Duration {
        let delay = self.current;
        let next_millis = (self.current.as_millis() as f64 * self.factor).round() as u64;
        self.current = Duration::from_millis(next_millis.min(self.max.as_millis() as u64));
        delay
    }
}
