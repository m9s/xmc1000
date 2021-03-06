#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DTT {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct DIMDIVR {
    bits: u16,
}
impl DIMDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `DTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTENR {
    #[doc = "No dithering"]
    VALUE1,
    #[doc = "Dithering is added to every dimming step if the dimming level is below 128; the coarse curve is used for the entire dimming range"]
    VALUE2,
}
impl DTENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DTENR::VALUE1 => false,
            DTENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTENR {
        match value {
            false => DTENR::VALUE1,
            true => DTENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DTENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DTENR::VALUE2
    }
}
#[doc = "Possible values of the field `CSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSELR {
    #[doc = "Coarse curve"]
    VALUE1,
    #[doc = "Fine curve"]
    VALUE2,
}
impl CSELR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CSELR::VALUE1 => false,
            CSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSELR {
        match value {
            false => CSELR::VALUE1,
            true => CSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CSELR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _DIMDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _DIMDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DTEN`"]
pub enum DTENW {
    #[doc = "No dithering"]
    VALUE1,
    #[doc = "Dithering is added to every dimming step if the dimming level is below 128; the coarse curve is used for the entire dimming range"]
    VALUE2,
}
impl DTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTENW::VALUE1 => false,
            DTENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No dithering"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DTENW::VALUE1)
    }
    #[doc = "Dithering is added to every dimming step if the dimming level is below 128; the coarse curve is used for the entire dimming range"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DTENW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSEL`"]
pub enum CSELW {
    #[doc = "Coarse curve"]
    VALUE1,
    #[doc = "Fine curve"]
    VALUE2,
}
impl CSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSELW::VALUE1 => false,
            CSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Coarse curve"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CSELW::VALUE1)
    }
    #[doc = "Fine curve"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSELW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - Dimming Clock Divider"]
    #[inline]
    pub fn dimdiv(&self) -> DIMDIVR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DIMDIVR { bits }
    }
    #[doc = "Bit 16 - Dither Enable"]
    #[inline]
    pub fn dten(&self) -> DTENR {
        DTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Curve Select"]
    #[inline]
    pub fn csel(&self) -> CSELR {
        CSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - Dimming Clock Divider"]
    #[inline]
    pub fn dimdiv(&mut self) -> _DIMDIVW {
        _DIMDIVW { w: self }
    }
    #[doc = "Bit 16 - Dither Enable"]
    #[inline]
    pub fn dten(&mut self) -> _DTENW {
        _DTENW { w: self }
    }
    #[doc = "Bit 17 - Curve Select"]
    #[inline]
    pub fn csel(&mut self) -> _CSELW {
        _CSELW { w: self }
    }
}
