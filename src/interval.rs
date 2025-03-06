#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        // default interval is empty
        Self { min: f64::INFINITY, max: f64::NEG_INFINITY }
    }
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    // create a new interval tightly enclosing both
    // input intervals
    pub fn from_intervals(a: &Interval, b: &Interval) -> Self {
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max >= b.max { a.max } else { b.max };

        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    // inclusive
    pub fn contains(&self, x: f64) -> bool {
        (x >= self.min) && (x <= self.max)
    }
    // exclusive
    pub fn surrounds(&self, x: f64) -> bool {
        (x > self.min) && (x < self.max)
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min { return self.min; }
        if x > self.max { return self.max; }
        return x;
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta/2.0;
        Self { min: self.min-padding, max: self.max+padding }
    }
}

const EMPTY: Interval = Interval { min: f64::INFINITY, max: f64::NEG_INFINITY };
const UNIVERSE: Interval = Interval { min: f64::NEG_INFINITY, max: f64::INFINITY };
