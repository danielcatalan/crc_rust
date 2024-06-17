pub trait Reflectable {
    fn reflect(&self) -> Self;
}

pub trait BitReflecttion {
    fn process<T: Reflectable>(val: T) -> T;
}

pub struct NoReflect;
pub struct Reflect;

impl BitReflecttion for NoReflect {
    #[inline(always)]
    fn process<T: Reflectable>(val: T) -> T {
        val
    }
}

impl BitReflecttion for Reflect {
    #[inline(always)]
    fn process<T: Reflectable>(val: T) -> T {
        val.reflect()
    }
}

impl Reflectable for u8 {
    #[inline(always)]
    fn reflect(&self) -> Self {
        self.reverse_bits()
    }
}

impl Reflectable for u16 {
    #[inline(always)]
    fn reflect(&self) -> Self {
        self.reverse_bits()
    }
}

impl Reflectable for u32 {
    #[inline(always)]
    fn reflect(&self) -> Self {
        self.reverse_bits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noreflect_u8() {
        let x: u8 = 0x80;

        let y = NoReflect::process(x);
        assert_eq!(0x80, y);
    }

    #[test]
    fn test_reflect_u8() {
        let x: u8 = 0x80;

        let y = Reflect::process(x);
        assert_eq!(0x01, y);
    }
}
