mod foo {
    pub struct Foo;
}

use foo::Foo;

fn main() {
    let _foo = Foo;
}