use futures::executor::block_on;
use std::future::Future;

#[link(wasm_import_module = "module")]
extern "C" {
    fn jsAbs(input: i32) -> u32;
}
#[no_mangle]
pub fn print_hello() {
    println!("Hello, world!");
}
#[no_mangle]
pub fn add_sum(a: u32, b: u32) -> u32 {
    a + b
}
async fn async_foo() -> u8 {
    5
}
fn async_main() -> impl Future<Output = u8> {
    println!("run_async_main");
    async {
        let x: u8 = async_foo().await;
        x + 5
    }
}
#[no_mangle]
pub fn run_async_main(a: u8) -> u8 {
    let x = block_on(async_main());
    x + a
}
#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}
#[export_name = "exported_symbol_name"]
pub fn callable_from_c(x: i32) -> bool {
    let a = "Hello";
    let b = "Tyr";
    let callback = |msg: &str| {
        println!("{} {}: {}", a, b, msg);
    };
    callback("How are you?");
    x > 5
}
fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}
fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}
#[export_name = "exported_function_test"]
pub fn exported_function_test() {
    println!("apply square: {}", apply(2, square));
    println!("apply cube: {}", apply(2, cube));
    unsafe {
        println!("Absolute value of -20 according to C: {}", jsAbs(-20));
    }
}
