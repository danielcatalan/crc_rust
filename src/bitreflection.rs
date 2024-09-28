use crate::Reflectable;

pub trait BitReflecttion {
    fn process_in<T: Reflectable>(val: T) -> T;
    fn process_out<T: Reflectable>(val: T) -> T;
}

pub struct NoReflect;

pub struct ReflectIn;

pub struct ReflectOut;

pub struct ReflectInOut;

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

impl BitReflecttion for ReflectIn {
    #[inline(always)]
    fn process_in<T: Reflectable>(val: T) -> T {
        val.reflect()
    }

    #[inline(always)]
    fn process_out<T: Reflectable>(val: T) -> T {
        val
    }
}

impl BitReflecttion for ReflectOut {
    #[inline(always)]
    fn process_in<T: Reflectable>(val: T) -> T {
        val
    }
    fn process_out<T: Reflectable>(val: T) -> T {
        val.reflect()
    }
}

impl BitReflecttion for ReflectInOut {
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

        let y = ReflectInOut::process_in(x);
        assert_eq!(0x01, y);
    }
}
