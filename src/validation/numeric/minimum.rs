use super::impl_composited_validation1;
use super::impl_numeric_composited_validation;
use crate::MinimumErrorParams;

/// Minimum validation of the number.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
pub trait ValidateMinimum<T>
where
    T: PartialOrd + PartialEq,
{
    fn validate_minimum(&self, minimum: T) -> Result<(), MinimumErrorParams>;
}

impl_composited_validation1!(
    ValidateCompositedMinimum,
    ValidateMinimum,
    MinimumErrorParams,
    validate_composited_minimum,
    validate_minimum
);

macro_rules! impl_validate_numeric_minimum {
    ($type:ty) => {
        impl ValidateMinimum<$type> for $type {
            fn validate_minimum(&self, minimum: $type) -> Result<(), MinimumErrorParams> {
                if *self >= minimum {
                    Ok(())
                } else {
                    Err(MinimumErrorParams::new(minimum))
                }
            }
        }

        impl_numeric_composited_validation!(
            ValidateCompositedMinimum,
            ValidateMinimum,
            MinimumErrorParams,
            validate_composited_minimum,
            validate_minimum,
            $type
        );
    };
}

impl_validate_numeric_minimum!(i8);
impl_validate_numeric_minimum!(i16);
impl_validate_numeric_minimum!(i32);
impl_validate_numeric_minimum!(i64);
impl_validate_numeric_minimum!(i128);
impl_validate_numeric_minimum!(isize);
impl_validate_numeric_minimum!(u8);
impl_validate_numeric_minimum!(u16);
impl_validate_numeric_minimum!(u32);
impl_validate_numeric_minimum!(u64);
impl_validate_numeric_minimum!(u128);
impl_validate_numeric_minimum!(usize);
impl_validate_numeric_minimum!(std::num::NonZeroI8);
impl_validate_numeric_minimum!(std::num::NonZeroI16);
impl_validate_numeric_minimum!(std::num::NonZeroI32);
impl_validate_numeric_minimum!(std::num::NonZeroI64);
impl_validate_numeric_minimum!(std::num::NonZeroI128);
impl_validate_numeric_minimum!(std::num::NonZeroIsize);
impl_validate_numeric_minimum!(std::num::NonZeroU8);
impl_validate_numeric_minimum!(std::num::NonZeroU16);
impl_validate_numeric_minimum!(std::num::NonZeroU32);
impl_validate_numeric_minimum!(std::num::NonZeroU64);
impl_validate_numeric_minimum!(std::num::NonZeroU128);
impl_validate_numeric_minimum!(std::num::NonZeroUsize);
impl_validate_numeric_minimum!(f32);
impl_validate_numeric_minimum!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_minimum_is_true() {
        assert!(ValidateMinimum::validate_minimum(&10, 9).is_ok());
        assert!(ValidateMinimum::validate_minimum(&10, 10).is_ok());
    }

    #[test]
    fn test_validate_numeric_minimum_is_false() {
        assert!(ValidateMinimum::validate_minimum(&5, 6).is_err());
    }

    #[test]
    fn test_validate_numeric_minimum_specified_type() {
        assert!(ValidateMinimum::validate_minimum(&0.5, 0.2).is_ok());
        assert!(ValidateMinimum::validate_minimum(&5u8, 0).is_ok());
        assert!(ValidateMinimum::validate_minimum(&4u16, 0).is_ok());
        assert!(ValidateMinimum::validate_minimum(&6u32, 0).is_ok());
    }
}
