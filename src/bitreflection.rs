use crate::Reflectable;

pub trait BitReflecttion {
    fn process_in<T: Reflectable>(val: T) -> T;
    fn process_out<T: Reflectable>(val: T) -> T;
}

pub struct NoReflect;
pub struct Reflect;

impl BitReflecttion for NoReflect {
    #[inline(always)]
    fn process_in<T: Reflectable>(val: T) -> T {
        val
    }

    #[inline(always)]
    fn process_out<T: Reflectable>(val: T) -> T {
        val
    }
}

impl BitReflecttion for Reflect {
    #[inline(always)]
    fn process_in<T: Reflectable>(val: T) -> T {
        val.reflect()
    }
    fn process_out<T: Reflectable>(val: T) -> T {
        val.reflect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noreflect_u8() {
        let x: u8 = 0x80;

        let y = NoReflect::process_in(x);
        assert_eq!(0x80, y);
    }

    #[test]
    fn test_reflect_u8() {
        let x: u8 = 0x80;

        let y = Reflect::process_in(x);
        assert_eq!(0x01, y);
    }
}
