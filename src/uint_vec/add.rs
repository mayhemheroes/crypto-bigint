//! [`UintVec`] addition operations.

use crate::{CheckedAdd, Limb, UintVec, Zero};
use subtle::CtOption;

impl UintVec {
    /// Computes `a + b + carry`, returning the result along with the new carry.
    #[inline(always)]
    pub fn adc(&self, rhs: &Self, carry: Limb) -> (Self, Limb) {
        Self::chain(self, rhs, carry, |a, b, c| a.adc(b, c))
    }

    /// Perform wrapping addition, discarding overflow.
    pub fn wrapping_add(&self, rhs: &Self) -> Self {
        self.adc(rhs, Limb::ZERO).0
    }
}

impl CheckedAdd<&UintVec> for UintVec {
    type Output = Self;

    fn checked_add(&self, rhs: &Self) -> CtOption<Self> {
        let (result, carry) = self.adc(rhs, Limb::ZERO);
        CtOption::new(result, carry.is_zero())
    }
}

#[cfg(test)]
mod tests {
    use super::{CheckedAdd, Limb, UintVec};

    #[test]
    fn adc_no_carry() {
        let (res, carry) = UintVec::zero().adc(&UintVec::one(), Limb::ZERO);
        assert_eq!(res, UintVec::one());
        assert_eq!(carry, Limb::ZERO);
    }

    #[test]
    fn adc_with_carry() {
        let (res, carry) = UintVec::max(Limb::BITS)
            .unwrap()
            .adc(&UintVec::one(), Limb::ZERO);
        assert_eq!(res, UintVec::zero());
        assert_eq!(carry, Limb::ONE);
    }

    #[test]
    fn checked_add_ok() {
        let result = UintVec::zero().checked_add(&UintVec::one());
        assert_eq!(result.unwrap(), UintVec::one());
    }

    #[test]
    fn checked_add_overflow() {
        let result = UintVec::max(Limb::BITS)
            .unwrap()
            .checked_add(&UintVec::one());
        assert!(!bool::from(result.is_some()));
    }
}
