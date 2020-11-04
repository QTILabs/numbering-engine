#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod db;

fn main() {
    println!("Hello, world!");
    let _ = db::establish_connection();
}
