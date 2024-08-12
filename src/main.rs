use bindings::{MyClass_create, MyClass_destroy, MyClass_myMethod};

mod bindings;

fn main() {

    unsafe {
        let obj = MyClass_create();
        MyClass_myMethod(obj);
        MyClass_destroy(obj);
    }
    println!("Hello, world!");
}
