
#[derive(MyDerive)]
struct Foo {
    #[asset(path = "asd", File)]
    pub field1: Vec<u8>,
    #[asset(path = "asd", Sound)]
    pub field2: f32,
    pub field3: f32,
}

fn main() {
    foo()
}
