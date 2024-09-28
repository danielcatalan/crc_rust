pub trait Reflectable {
    fn reflect(&self) -> Self;
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
