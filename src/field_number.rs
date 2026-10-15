/// A protobuf field number that is guaranteed to be valid, i.e. in the range 1 to 2^29-1.
///
/// The only ways to create an instance are [`FieldNumber::new`] and [`assert_field_number_valid`],
/// which both check the range. So code receiving a `FieldNumber` does not need to check again.
///
/// The range 19000 to 19999 is reserved for the protobuf implementation. Those
/// field numbers can be encoded just fine and are considered valid here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldNumber(u32);

impl FieldNumber {
    /// The smallest valid field number.
    pub const MIN: u32 = 1;

    /// The largest valid field number (2^29-1).
    ///
    /// "The smallest field number you can specify is 1, and the largest is 2^29-1, or 536,870,911"
    /// <https://protobuf.dev/programming-guides/proto3/#assigning-field-numbers>
    pub const MAX: u32 = (1 << 29) - 1;

    /// Creates a field number. Returns `None` if the value is outside of the valid range 1 to 2^29-1.
    pub fn new(value: u32) -> Option<Self> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Returns the field number as an integer in the range 1 to 2^29-1.
    pub fn get(self) -> u32 {
        self.0
    }
}

/// Creates a [`FieldNumber`] and panics if the field number is outside of the valid range 1 to 2^29-1.
///
/// Field numbers outside of this range cannot be encoded correctly in a tag. E.g. 0 is
/// not a valid field number and 2^29 would silently become field number 0 in the tag.
pub fn assert_field_number_valid(field_number: u32) -> FieldNumber {
    match FieldNumber::new(field_number) {
        Some(field_number) => field_number,
        None => panic!(
            "Invalid field number {field_number}. Must be in range {} to {}.",
            FieldNumber::MIN,
            FieldNumber::MAX
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    #[test]
    fn field_number_new_works() {
        // valid
        for value in [
            1, 2, 15, 16, 18999,
            19000, // reserved for the protobuf implementation but valid in the encoding
            19999, 20000, 536870910, 536870911,
        ] {
            assert_eq!(FieldNumber::new(value).unwrap().get(), value);
        }

        // invalid
        for value in [0, 536870912, 536870913, 1 << 30, 1 << 31, u32::MAX] {
            assert_eq!(FieldNumber::new(value), None, "Accepted {value}");
        }
    }

    #[test]
    fn field_number_min_max_works() {
        assert_eq!(FieldNumber::MIN, 1);
        assert_eq!(FieldNumber::MAX, 536870911);
        assert!(FieldNumber::new(FieldNumber::MIN).is_some());
        assert!(FieldNumber::new(FieldNumber::MAX).is_some());
        assert!(FieldNumber::new(FieldNumber::MIN - 1).is_none());
        assert!(FieldNumber::new(FieldNumber::MAX + 1).is_none());
    }

    #[test]
    fn assert_field_number_valid_works() {
        // valid
        for value in [1, 2, 15, 16, 19000, 536870911] {
            assert_eq!(assert_field_number_valid(value).get(), value);
        }

        // invalid
        for field_number in [0, 536870912, 536870913, 1 << 30, 1 << 31, u32::MAX] {
            let result = std::panic::catch_unwind(|| assert_field_number_valid(field_number));
            assert!(result.is_err(), "Did not panic for {field_number}");
        }
    }

    #[test]
    #[should_panic(expected = "Invalid field number 0. Must be in range 1 to 536870911.")]
    fn assert_field_number_valid_panics_for_zero() {
        assert_field_number_valid(0);
    }

    #[test]
    #[should_panic(expected = "Invalid field number 536870912. Must be in range 1 to 536870911.")]
    fn assert_field_number_valid_panics_for_too_large() {
        assert_field_number_valid(536870912);
    }
}
