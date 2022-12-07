use futures::{executor::block_on};
use std::future::Future;
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
    x % 3 == 0
}
