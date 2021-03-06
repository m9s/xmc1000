#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDISC {
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
#[doc = "Possible values of the field `PDIS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS0R {
    #[doc = "Digital Pad input is enabled. Analog and digital input/output path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS0R {
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
            PDIS0R::VALUE1 => false,
            PDIS0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS0R {
        match value {
            false => PDIS0R::VALUE1,
            true => PDIS0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS0R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS1R {
    #[doc = "Digital Pad input is enabled. Analog and digital input/output path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS1R {
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
            PDIS1R::VALUE1 => false,
            PDIS1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS1R {
        match value {
            false => PDIS1R::VALUE1,
            true => PDIS1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS1R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS2R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS2R {
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
            PDIS2R::VALUE1 => false,
            PDIS2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS2R {
        match value {
            false => PDIS2R::VALUE1,
            true => PDIS2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS2R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS3R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS3R {
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
            PDIS3R::VALUE1 => false,
            PDIS3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS3R {
        match value {
            false => PDIS3R::VALUE1,
            true => PDIS3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS3R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS4R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS4R {
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
            PDIS4R::VALUE1 => false,
            PDIS4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS4R {
        match value {
            false => PDIS4R::VALUE1,
            true => PDIS4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS4R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS5R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS5R {
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
            PDIS5R::VALUE1 => false,
            PDIS5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS5R {
        match value {
            false => PDIS5R::VALUE1,
            true => PDIS5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS5R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS6R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS6R {
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
            PDIS6R::VALUE1 => false,
            PDIS6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS6R {
        match value {
            false => PDIS6R::VALUE1,
            true => PDIS6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS6R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS7R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS7R {
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
            PDIS7R::VALUE1 => false,
            PDIS7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS7R {
        match value {
            false => PDIS7R::VALUE1,
            true => PDIS7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS7R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS8R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS8R {
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
            PDIS8R::VALUE1 => false,
            PDIS8R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS8R {
        match value {
            false => PDIS8R::VALUE1,
            true => PDIS8R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS8R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS8R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS9R {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS9R {
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
            PDIS9R::VALUE1 => false,
            PDIS9R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS9R {
        match value {
            false => PDIS9R::VALUE1,
            true => PDIS9R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS9R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS9R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS10R {
    #[doc = "Digital Pad input is enabled. Analog and digital input/output path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS10R {
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
            PDIS10R::VALUE1 => false,
            PDIS10R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS10R {
        match value {
            false => PDIS10R::VALUE1,
            true => PDIS10R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS10R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS10R::VALUE2
    }
}
#[doc = "Possible values of the field `PDIS11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS11R {
    #[doc = "Digital Pad input is enabled. Analog and digital input/output path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS11R {
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
            PDIS11R::VALUE1 => false,
            PDIS11R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDIS11R {
        match value {
            false => PDIS11R::VALUE1,
            true => PDIS11R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDIS11R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDIS11R::VALUE2
    }
}
#[doc = "Values that can be written to the field `PDIS0`"]
pub enum PDIS0W {
    #[doc = "Digital Pad input is enabled. Analog and digital input/output path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS0W::VALUE1 => false,
            PDIS0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS0W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input/output path active."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS0W::VALUE1)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS0W::VALUE2)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDIS1`"]
pub enum PDIS1W {
    #[doc = "Digital Pad input is enabled. Analog and digital input/output path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS1W::VALUE1 => false,
            PDIS1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input/output path active."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS1W::VALUE1)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS1W::VALUE2)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDIS2`"]
pub enum PDIS2W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS2W::VALUE1 => false,
            PDIS2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS2W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS2W::VALUE1)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS2W::VALUE2)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDIS3`"]
pub enum PDIS3W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS3W::VALUE1 => false,
            PDIS3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS3W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS3W::VALUE1)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS3W::VALUE2)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDIS4`"]
pub enum PDIS4W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS4W::VALUE1 => false,
            PDIS4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS4W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS4W::VALUE1)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS4W::VALUE2)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDIS5`"]
pub enum PDIS5W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS5W::VALUE1 => false,
            PDIS5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS5W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS5W::VALUE1)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS5W::VALUE2)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDIS6`"]
pub enum PDIS6W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS6W::VALUE1 => false,
            PDIS6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS6W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS6W::VALUE1)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS6W::VALUE2)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDIS7`"]
pub enum PDIS7W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS7W::VALUE1 => false,
            PDIS7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS7W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS7W::VALUE1)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS7W::VALUE2)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDIS8`"]
pub enum PDIS8W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS8W::VALUE1 => false,
            PDIS8W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS8W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS8W::VALUE1)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS8W::VALUE2)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDIS9`"]
pub enum PDIS9W {
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS9W::VALUE1 => false,
            PDIS9W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS9W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input path active."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS9W::VALUE1)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS9W::VALUE2)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDIS10`"]
pub enum PDIS10W {
    #[doc = "Digital Pad input is enabled. Analog and digital input/output path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS10W::VALUE1 => false,
            PDIS10W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS10W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input/output path active."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS10W::VALUE1)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS10W::VALUE2)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDIS11`"]
pub enum PDIS11W {
    #[doc = "Digital Pad input is enabled. Analog and digital input/output path active."]
    VALUE1,
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    VALUE2,
}
impl PDIS11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDIS11W::VALUE1 => false,
            PDIS11W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDIS11W<'a> {
    w: &'a mut W,
}
impl<'a> _PDIS11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDIS11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital Pad input is enabled. Analog and digital input/output path active."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS11W::VALUE1)
    }
    #[doc = "Digital Pad input is disabled. Analog input path active. (default)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS11W::VALUE2)
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
        const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - Pad Disable for Port 2 Pin 0"]
    #[inline]
    pub fn pdis0(&self) -> PDIS0R {
        PDIS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pad Disable for Port 2 Pin 1"]
    #[inline]
    pub fn pdis1(&self) -> PDIS1R {
        PDIS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Pad Disable for Port 2 Pin 2"]
    #[inline]
    pub fn pdis2(&self) -> PDIS2R {
        PDIS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Pad Disable for Port 2 Pin 3"]
    #[inline]
    pub fn pdis3(&self) -> PDIS3R {
        PDIS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Pad Disable for Port 2 Pin 4"]
    #[inline]
    pub fn pdis4(&self) -> PDIS4R {
        PDIS4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Pad Disable for Port 2 Pin 5"]
    #[inline]
    pub fn pdis5(&self) -> PDIS5R {
        PDIS5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Pad Disable for Port 2 Pin 6"]
    #[inline]
    pub fn pdis6(&self) -> PDIS6R {
        PDIS6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Pad Disable for Port 2 Pin 7"]
    #[inline]
    pub fn pdis7(&self) -> PDIS7R {
        PDIS7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad Disable for Port 2 Pin 8"]
    #[inline]
    pub fn pdis8(&self) -> PDIS8R {
        PDIS8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Pad Disable for Port 2 Pin 9"]
    #[inline]
    pub fn pdis9(&self) -> PDIS9R {
        PDIS9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Pad Disable for Port 2 Pin 10"]
    #[inline]
    pub fn pdis10(&self) -> PDIS10R {
        PDIS10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Pad Disable for Port 2 Pin 11"]
    #[inline]
    pub fn pdis11(&self) -> PDIS11R {
        PDIS11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - Pad Disable for Port 2 Pin 0"]
    #[inline]
    pub fn pdis0(&mut self) -> _PDIS0W {
        _PDIS0W { w: self }
    }
    #[doc = "Bit 1 - Pad Disable for Port 2 Pin 1"]
    #[inline]
    pub fn pdis1(&mut self) -> _PDIS1W {
        _PDIS1W { w: self }
    }
    #[doc = "Bit 2 - Pad Disable for Port 2 Pin 2"]
    #[inline]
    pub fn pdis2(&mut self) -> _PDIS2W {
        _PDIS2W { w: self }
    }
    #[doc = "Bit 3 - Pad Disable for Port 2 Pin 3"]
    #[inline]
    pub fn pdis3(&mut self) -> _PDIS3W {
        _PDIS3W { w: self }
    }
    #[doc = "Bit 4 - Pad Disable for Port 2 Pin 4"]
    #[inline]
    pub fn pdis4(&mut self) -> _PDIS4W {
        _PDIS4W { w: self }
    }
    #[doc = "Bit 5 - Pad Disable for Port 2 Pin 5"]
    #[inline]
    pub fn pdis5(&mut self) -> _PDIS5W {
        _PDIS5W { w: self }
    }
    #[doc = "Bit 6 - Pad Disable for Port 2 Pin 6"]
    #[inline]
    pub fn pdis6(&mut self) -> _PDIS6W {
        _PDIS6W { w: self }
    }
    #[doc = "Bit 7 - Pad Disable for Port 2 Pin 7"]
    #[inline]
    pub fn pdis7(&mut self) -> _PDIS7W {
        _PDIS7W { w: self }
    }
    #[doc = "Bit 8 - Pad Disable for Port 2 Pin 8"]
    #[inline]
    pub fn pdis8(&mut self) -> _PDIS8W {
        _PDIS8W { w: self }
    }
    #[doc = "Bit 9 - Pad Disable for Port 2 Pin 9"]
    #[inline]
    pub fn pdis9(&mut self) -> _PDIS9W {
        _PDIS9W { w: self }
    }
    #[doc = "Bit 10 - Pad Disable for Port 2 Pin 10"]
    #[inline]
    pub fn pdis10(&mut self) -> _PDIS10W {
        _PDIS10W { w: self }
    }
    #[doc = "Bit 11 - Pad Disable for Port 2 Pin 11"]
    #[inline]
    pub fn pdis11(&mut self) -> _PDIS11W {
        _PDIS11W { w: self }
    }
}
