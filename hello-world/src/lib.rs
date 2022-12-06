#[no_mangle]
pub fn print_hello() {
    println!("Hello, world!");
}
#[no_mangle]
pub fn add_sum(a: u32, b: u32) -> u32 {
    a + b
}
#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}
#[export_name = "exported_symbol_name"]
pub fn callable_from_c(x: i32) -> bool {
    x % 3 == 0
}
