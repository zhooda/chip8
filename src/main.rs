use std::fs::File;
use std::io::Read;


fn main() {
    let mut file = File::open("roms/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);


}
