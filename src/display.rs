const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    screen: [u8; WIDTH*HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [0; WIDTH*HEIGHT],
        }
    }

    pub fn index_from_xy(x: usize, y: usize) -> usize {
        y * WIDTH + x
    }

    pub fn debug_draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        let mut erased = false;
        let mut x_pos = x as usize;
        let mut y_pos = y as ysize;
        let mut b = byte;

        for _ in 0..8 {
            x_pos %= WIDTH;
            y_pos %= HEIGHT;
            let idx = Display::index_from_xy(x_pos, y_pos);
            let bit = (b & 0b1000_0000) >> 7;
            let previous = self.screen[idx]
            self.screen[idx] ^= bit;

            if previous == 1 && self.screen[idx] == 0 {
                erased = true;
            }

            x_pos += 1;
            b <<= 1;
        }

        erased
    }

    pub fn clear(&mut self) {
        for pixel in self.screen.iter_mut() {
            *pixel = 0;
        }
    }

    pub fn get_display_buffer(&self) -> &[u8] {
        &self.screen
    }
}
