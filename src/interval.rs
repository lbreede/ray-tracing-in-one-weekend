pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    // fn default() -> Self {
    //     Self {
    //         min: f64::INFINITY,
    //         max: f64::NEG_INFINITY,
    //     }
    // }

    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    // fn size(&self) -> f64 {
    //     self.max - self.min
    // }

    // fn contains(&self, x: f64) -> bool {
    //     self.min <= x && x <= self.max
    // }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }

    // fn empty() -> Self {
    //     Self {
    //         min: f64::INFINITY,
    //         max: f64::NEG_INFINITY,
    //     }
    // }
    // fn universe() -> Self {
    //     Self {
    //         min: f64::NEG_INFINITY,
    //         max: f64::INFINITY,
    //     }
    // }
}
