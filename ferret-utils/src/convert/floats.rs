pub trait FloatType {
    fn is_ge_one(&self) -> bool;
    fn mul_set(&mut self, value: f32);
    fn div_set(&mut self, value: f32);
    fn minus_self_as_int(&mut self);
    fn is_negative(&self)  -> bool;
    fn invert_sign(&mut self);
    fn int_modulus(&self, value: u8) -> u8;
}

impl FloatType for f32 {
    fn is_ge_one(&self) -> bool {
        *self >= 1.0
    }

    fn mul_set(&mut self, value: f32) {
        *self *= value
    }

    fn div_set(&mut self, value: f32) {
        *self /= value
    }

    fn minus_self_as_int(&mut self) {
        *self -= (*self as u32) as Self;
    }

    fn is_negative(&self)  -> bool {
        *self < 0.0
    }

    fn invert_sign(&mut self) {
        *self = -*self
    }

    fn int_modulus(&self, value: u8) -> u8 {
        (*self % value as Self) as u8
    }
}

impl FloatType for f64 {
    fn is_ge_one(&self) -> bool {
        *self >= 1.0
    }

    fn mul_set(&mut self, value: f32) {
        *self *= value as f64
    }

    fn div_set(&mut self, value: f32) {
        *self /= value as f64
    }

    fn minus_self_as_int(&mut self) {
        *self -= (*self as u32) as Self;
    }

    fn is_negative(&self)  -> bool {
        *self < 0.0
    }

    fn invert_sign(&mut self) {
        *self = -*self
    }

    fn int_modulus(&self, value: u8) -> u8 {
        (*self % value as Self) as u8
    }
}

// https://gist.github.com/Linaiz/4e27ea8d9760050008e3638a6fcf8be8
pub fn float_to_string<T>(mut num: T, buffer: &mut [char], decimal_count: u8)
where T: FloatType + Copy + Clone
{
    let mut count = 0;

    if num.is_negative() {
        buffer[count] = '-';
        num.invert_sign();
        count += 1;
    }

    let start = count;
    let mut temp = num;

    while temp.is_ge_one() {
        let res = temp.int_modulus(10);
        buffer[count] = (res + '0' as u8) as char;
        count += 1;
        temp.div_set(10.0);
    }


    if count == 0 {
        buffer[0] = '0';
        count = 1;
    } else {
        buffer[start..count].reverse();
    }

    buffer[count] = '.';
    count += 1;

    temp = num;
    temp.minus_self_as_int();

    for index in 0..decimal_count {
        temp.mul_set(10.0);
        let res = temp.int_modulus(10);
        buffer[index as usize + count] = (res + '0' as u8) as char;
    }

    buffer[count + decimal_count as usize] = '\0';
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_float_zero() {
        let expected = ['0', '.', '0', '\0'];
        let mut result = ['\0'; 4];

        float_to_string(0.0, &mut result, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn convert_float_positive() {
        let expected = ['2', '4', '6', '8', '.', '0', '0', '0', '1', '\0'];
        let mut result = ['\0'; 10];

        float_to_string(2468.0001, &mut result, 4);

        assert_eq!(result, expected);
    }

    #[test]
    fn convert_float_negative() {
        //
        let expected = ['-', '2', '3', '4', '.', '0', '\0'];
        let mut result = ['\0'; 7];

        float_to_string(-234.0, &mut result, 1);

        assert_eq!(result, expected);
    }
}