use std::fs::File;
use std::io::Read;

mod chip8;
mod memory;

fn pretty_hex(value: &u8) -> String {
    let mut x = format!("{:x}", value);
    if x.len() < 4 {
        for _ in 0..4 - x.len() {
            x = format!("0{}", x);
        }
    }
    x
}

fn dump(data: &Vec<u8>) {
    for i in 0..data.len() {
        println!("0x{} : 0x{}", pretty_hex(&(i as u8)), pretty_hex(&data[i]));
    }
}

fn main() {
    let mut file = File::open("roms/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    dump(&data);
}
