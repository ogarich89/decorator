pub struct User;

pub trait Info {
    fn info(&self, data: &'static str) -> String;
}

impl Info for User {
    fn info(&self, data: &'static str) -> String {
        data.to_string()
    }
}

pub struct SurnameDecorator {
    decorated: Box<dyn Info>,
    data: &'static str,
}

impl SurnameDecorator {
    pub fn new(decorated: Box<dyn Info>, data: &'static str) -> Self {
        Self { decorated, data }
    }
}

impl Info for SurnameDecorator {
    fn info(&self, data: &'static str) -> String {
        let str = format!("{} | {}", self.decorated.info(data), self.data);
        str
    }
}

pub struct NicknameDecorator {
    decorated: Box<dyn Info>,
    data: &'static str,
}

impl NicknameDecorator {
    pub fn new(decorated: Box<dyn Info>, data: &'static str) -> Self {
        Self { decorated, data }
    }
}

impl Info for NicknameDecorator {
    fn info(&self, data: &'static str) -> String {
        let str = format!("{} | {}", self.decorated.info(data), self.data);
        str
    }
}
