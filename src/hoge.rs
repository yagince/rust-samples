pub struct Hoge {
    number: i32,
}

impl Hoge {
    pub fn new(number: i32) -> Hoge { Hoge{number: number} }
    pub fn num(&self) -> i32 { self.number }
}
