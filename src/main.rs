mod test_image;
mod test_chna;

use rand::prelude::*;
use rabbit_rust_iknow::read_write_test;

#[derive(Debug)]
struct Stp {
    age: u32
}

struct Parent {
    stp: Stp,
    stp2: Stp
}

fn main() {
    read_write_test();
    test2();
}


fn rand(start: usize, end: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(start..end).try_into().unwrap()
}


// 使用下标元素不会出现所有权变化
fn test1() {
    println!("使用下标元素不会出现所有权变化");
    let mines = vec![1, 2, 3, 4, 5];
    let mine = mines[3];
    println!("mines: {:?}", mines);
    println!("mine: {:?}", mine);
}
fn test2() -> Option<u32> {
    let found: Vec<u32> = Vec::new();

    if let Some(item) = found.get(0) {
        let item = *item;
    }
    let item = match found.get(0) {
        Some(item) => {
            *item
        },
        None => {
            panic!();
        }
    };
    let item = *found.get(0)?;

    return Some(0);
}
