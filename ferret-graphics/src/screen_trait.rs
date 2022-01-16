pub trait ScreenTrait {
    fn get_width(&self) -> u16;
    fn get_height(&self) -> u16;

    fn set_pixel(&mut self, x: u16, y: u16, r: u8, g: u8, b: u8);
    fn clear(&mut self, r: u8, g: u8, b: u8);

    fn update_screen(&mut self) -> Result<(), ()>;
}