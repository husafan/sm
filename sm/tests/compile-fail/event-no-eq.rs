use ::sm::sm;

#[derive(Clone, Copy, Debug)]
struct HelloWorld;
impl sm::Event for HelloWorld {}
//~^ ERROR the trait bound `HelloWorld: std::cmp::Eq` is not satisfied

fn main() {}
