use crate::currency::Currency;
use std::fmt;

// Macro that provides a constant to iterate over the Iso Enum.
macro_rules! define_enum {
    ($Name:ident { $($Variant:ident),* $(,)* }) =>
    {
        #[derive(Debug)]
        pub enum $Name {
            $($Variant),*,
        }
        pub const ISO_CURRENCIES: &'static [$Name] = &[$($Name::$Variant),*];
    }
}

define_enum!(Iso { USD });

impl fmt::Display for Iso {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Returns a Currency object given an ISO enum.
pub fn from_enum(code: &Iso) -> Currency {
    use Iso::*;

    match code {
        USD => Currency {
            code: "USD",
            name: "United States Dollar",
            symbol: "$",
        },
    }
}
