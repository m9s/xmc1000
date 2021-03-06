#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NVIC_ICPR {
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
#[doc = "Possible values of the field `CLRPEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRPENDR {
    #[doc = "Read: Interrupt node is not pending. Write: No effect."]
    VALUE1,
    #[doc = "Read: Interrupt node is pending. Write: Remove interrupt state from pending."]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl CLRPENDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            CLRPENDR::VALUE1 => 0,
            CLRPENDR::VALUE2 => 1,
            CLRPENDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> CLRPENDR {
        match value {
            0 => CLRPENDR::VALUE1,
            1 => CLRPENDR::VALUE2,
            i => CLRPENDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CLRPENDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CLRPENDR::VALUE2
    }
}
#[doc = "Values that can be written to the field `CLRPEND`"]
pub enum CLRPENDW {
    #[doc = "Read: Interrupt node is not pending. Write: No effect."]
    VALUE1,
    #[doc = "Read: Interrupt node is pending. Write: Remove interrupt state from pending."]
    VALUE2,
}
impl CLRPENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            CLRPENDW::VALUE1 => 0,
            CLRPENDW::VALUE2 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLRPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLRPENDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Read: Interrupt node is not pending. Write: No effect."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLRPENDW::VALUE1)
    }
    #[doc = "Read: Interrupt node is pending. Write: Remove interrupt state from pending."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLRPENDW::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:31 - Interrupt Node Clear-pending"]
    #[inline]
    pub fn clrpend(&self) -> CLRPENDR {
        CLRPENDR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
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
    #[doc = "Bits 0:31 - Interrupt Node Clear-pending"]
    #[inline]
    pub fn clrpend(&mut self) -> _CLRPENDW {
        _CLRPENDW { w: self }
    }
}
