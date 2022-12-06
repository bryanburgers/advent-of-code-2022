struct Visualization {
    bytes: Vec<u8>,
    start: usize,
    good_start: usize,
    end: usize,
    valid: bool,
}

impl Visualization {
    pub fn new(size: usize) -> Self {
        Visualization {
            bytes: Vec::with_capacity(size),
            start: 0,
            good_start: 0,
            end: 0,
            valid: true,
        }
    }

    pub fn find_good_start(&self) -> usize {
        let mut good_start = self.start;
        let last_byte = self.bytes[self.end - 1];
        for idx in self.start..(self.end - 1) {
            if self.bytes[idx] == last_byte {
                good_start = idx + 1;
            }
        }
        good_start
    }

    pub fn tick(&mut self, include_invalid: bool) -> bool {
        let mut end_of_buffer = self.tick_one();
        while !include_invalid && !end_of_buffer && !self.valid {
            end_of_buffer = self.tick_one();
        }
        end_of_buffer
    }

    pub fn tick_one(&mut self) -> bool {
        if self.valid {
            self.end += 1;
        } else {
            self.start += 1;
        }
        self.good_start = self.find_good_start();
        self.valid = self.start == self.good_start;

        self.end >= self.bytes.len()
    }

    pub fn draw(&self, canvas: &Canvas) {
        canvas.clear();
        canvas.gray();

        for (idx, byte) in self.bytes.iter().enumerate() {
            let x = idx % 64;
            let y = idx / 64;

            if self.start <= idx && idx < self.end {
                if idx < self.good_start {
                    canvas.dark_red()
                } else {
                    canvas.dark_green()
                }
                canvas.fill_box(x, y);

                if idx < self.good_start {
                    canvas.bright_red()
                } else {
                    canvas.bright_green()
                }
            } else {
                canvas.gray();
            }

            canvas.draw_byte(x, y, *byte)
        }
    }
}

struct Canvas;

impl Canvas {
    fn set_fill_color_rgb(&self, r: u8, g: u8, b: u8) {
        unsafe { sys::fill_color_rgb(r, g, b) }
    }

    fn gray(&self) {
        self.set_fill_color_rgb(100, 100, 100);
    }

    fn dark_red(&self) {
        self.set_fill_color_rgb(80, 0, 0)
    }
    fn bright_red(&self) {
        self.set_fill_color_rgb(255, 0, 0)
    }
    fn dark_green(&self) {
        self.set_fill_color_rgb(0, 80, 0)
    }
    fn bright_green(&self) {
        self.set_fill_color_rgb(0, 255, 0)
    }

    fn draw_byte(&self, x: usize, y: usize, byte: u8) {
        unsafe { sys::draw_byte(x, y, byte) }
    }

    fn fill_box(&self, x: usize, y: usize) {
        unsafe { sys::fill_box(x, y) }
    }

    fn clear(&self) {
        unsafe { sys::clear() }
    }
}

mod sys {
    use super::*;

    #[no_mangle]
    extern "C" fn visualization_create(string_size: u32) -> *mut Visualization {
        let visualization = Box::new(Visualization::new(string_size as usize));
        Box::into_raw(visualization)
    }

    #[no_mangle]
    extern "C" fn visualization_bytes_address(visualization: *mut Visualization) -> *mut u8 {
        let mut visualization = unsafe { Box::from_raw(visualization) };
        let ptr = visualization.bytes.as_mut_ptr();
        std::mem::forget(visualization);
        ptr
    }

    #[no_mangle]
    extern "C" fn visualization_bytes_set(visualization: &mut Visualization, size: u32) {
        unsafe { visualization.bytes.set_len(size as usize) }
    }

    #[no_mangle]
    extern "C" fn visualization_tick(
        visualization: &mut Visualization,
        include_invalid: bool,
    ) -> bool {
        let r = visualization.tick(include_invalid);
        visualization.draw(&Canvas);
        r
    }

    #[no_mangle]
    extern "C" fn visualization_free(visualization: Box<Visualization>) {
        std::mem::drop(visualization)
    }

    #[allow(dead_code)]
    pub fn log(text: impl AsRef<str>) {
        let str = text.as_ref();
        let ptr = str.as_ptr();
        let len = str.len() as u32;
        unsafe { console_log(ptr, len) };
    }

    extern "C" {
        /// Draw a single byte to the screen, in a box
        pub fn draw_byte(x: usize, y: usize, byte: u8);
        pub fn fill_box(x: usize, y: usize);
        pub fn clear();
        pub fn fill_color_rgb(r: u8, g: u8, b: u8);
        pub fn console_log(ptr: *const u8, len: u32);
    }
}
