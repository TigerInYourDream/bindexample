/* automatically generated by rust-bindgen 0.69.4 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MyClassOpaque {
    _unused: [u8; 0],
}
pub type MyClassHandle = *mut MyClassOpaque;
extern "C" {
    pub fn MyClass_create() -> MyClassHandle;
}
extern "C" {
    pub fn MyClass_destroy(handle: MyClassHandle);
}
extern "C" {
    pub fn MyClass_myMethod(handle: MyClassHandle);
}
