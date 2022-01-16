pub trait IntegerType {
    fn is_zero(&self) -> bool;
    fn is_lt_zero(&self) -> bool;
    fn invert_sign(&mut self);
    fn div_set(&mut self, value: u8);
    fn modulus(&mut self, value: u8) -> u8;
}

impl IntegerType for u8 {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_lt_zero(&self) -> bool {
        false
    }

    fn invert_sign(&mut self) {}

    fn div_set(&mut self, value: u8) {
        *self /= value
    }

    fn modulus(&mut self, value: u8) -> u8 {
        *self % value
    }
}

impl IntegerType for u16 {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_lt_zero(&self) -> bool {
        false
    }

    fn invert_sign(&mut self) {}

    fn div_set(&mut self, value: u8) {
        *self /= value as Self
    }

    fn modulus(&mut self, value: u8) -> u8 {
        (*self % value as Self) as u8
    }
}

impl IntegerType for u32 {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_lt_zero(&self) -> bool {
        false
    }

    fn invert_sign(&mut self) {}

    fn div_set(&mut self, value: u8) {
        *self /= value as Self
    }

    fn modulus(&mut self, value: u8) -> u8 {
        (*self % value as Self) as u8
    }
}

impl IntegerType for u64 {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_lt_zero(&self) -> bool {
        false
    }

    fn invert_sign(&mut self) {}

    fn div_set(&mut self, value: u8) {
        *self /= value as Self
    }

    fn modulus(&mut self, value: u8) -> u8 {
        (*self % value as Self) as u8
    }
}

impl IntegerType for u128 {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_lt_zero(&self) -> bool {
        false
    }

    fn invert_sign(&mut self) {}

    fn div_set(&mut self, value: u8) {
        *self /= value as Self
    }

    fn modulus(&mut self, value: u8) -> u8 {
        (*self % value as Self) as u8
    }
}

impl IntegerType for i8 {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_lt_zero(&self) -> bool {
        false
    }

    fn invert_sign(&mut self) {}

    fn div_set(&mut self, value: u8) {
        *self = (*self as i16 / value as i16) as i8;
    }

    fn modulus(&mut self, value: u8) -> u8 {
        (*self as i16 % value as i16) as u8
    }
}

impl IntegerType for i16 {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_lt_zero(&self) -> bool {
        false
    }

    fn invert_sign(&mut self) {}

    fn div_set(&mut self, value: u8) {
        *self /= value as Self
    }

    fn modulus(&mut self, value: u8) -> u8 {
        (*self % value as Self) as u8
    }
}

impl IntegerType for i32 {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_lt_zero(&self) -> bool {
        false
    }

    fn invert_sign(&mut self) {}

    fn div_set(&mut self, value: u8) {
        *self /= value as Self
    }

    fn modulus(&mut self, value: u8) -> u8 {
        (*self % value as Self) as u8
    }
}

impl IntegerType for i64 {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_lt_zero(&self) -> bool {
        false
    }

    fn invert_sign(&mut self) {}

    fn div_set(&mut self, value: u8) {
        *self /= value as Self
    }

    fn modulus(&mut self, value: u8) -> u8 {
        (*self % value as Self) as u8
    }
}

impl IntegerType for i128 {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_lt_zero(&self) -> bool {
        false
    }

    fn invert_sign(&mut self) {}

    fn div_set(&mut self, value: u8) {
        *self /= value as Self
    }

    fn modulus(&mut self, value: u8) -> u8 {
        (*self % value as Self) as u8
    }
}


// ported from: https://www.geeksforgeeks.org/implement-itoa/
pub fn integer_to_string<T>(mut num: T, buffer: &mut [char], base: u8)
where T: IntegerType {
    let mut index = 0;
    let mut is_negative = false;

    if num.is_zero() {
        buffer[index] = '0';
        buffer[index + 1] = '\0';
        return;
    }

    if num.is_lt_zero() && base == 10 {
        is_negative = true;
        num.invert_sign();
    }

    while !num.is_zero() {
        let unit = num.modulus(base);
        buffer[index] =
            if unit > 9 {
                (unit - 10 + 'a' as u8) as char
            } else {
                (unit + '0' as u8) as char
            };

        num.div_set(base);

        index += 1;
    }

    if is_negative {
        buffer[index] = '-';
        index += 1;
    }

    buffer[index] = '\0';
    buffer[0..index-1].reverse()
}
