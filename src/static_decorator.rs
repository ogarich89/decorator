pub trait Time {
    fn convert(&self, time: i32) -> i32;
}

pub struct Days;
impl Time for Days {
    fn convert(&self, time: i32) -> i32 {
        time
    }
}

pub struct HoursDecorator {
    decorated: Days,
}
impl HoursDecorator {
    pub fn new(decorated: Days) -> Self {
        Self { decorated }
    }
}
impl Time for HoursDecorator {
    fn convert(&self, time: i32) -> i32 {
        self.decorated.convert(time) * 24
    }
}

pub struct MinutesDecorator {
    decorated: HoursDecorator,
}
impl MinutesDecorator {
    pub fn new(decorated: HoursDecorator) -> Self {
        Self { decorated }
    }
}
impl Time for MinutesDecorator {
    fn convert(&self, time: i32) -> i32 {
        self.decorated.convert(time) * 60
    }
}

pub struct SecondsDecorator {
    decorated: MinutesDecorator,
}

impl SecondsDecorator {
    pub fn new(decorated: MinutesDecorator) -> Self {
        Self { decorated }
    }
}

impl Time for SecondsDecorator {
    fn convert(&self, time: i32) -> i32 {
        self.decorated.convert(time) * 60
    }
}
