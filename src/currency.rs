mod iso;

pub use crate::Error;
pub use iso::Iso;
use std::collections::HashMap;
use std::fmt;

lazy_static! {
    static ref CURRENCIES_BY_ALPHA_CODE: HashMap<String, Currency> =
        Currency::generate_currencies_by_alpha_code();
}

#[derive(Debug, PartialEq, Eq)]
pub struct Currency {
    pub code: &'static str,
    pub name: &'static str,
    pub symbol: &'static str,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl Currency {
    pub fn find(code: &str) -> Result<&'static Currency, Error> {
        Currency::from_string(code.to_string())
    }

    pub fn from_string(code: String) -> Result<&'static Currency, Error> {
        Currency::find_by_alpha_iso(code).ok_or(Error::InvalidCurrency)
    }

    /// Returns a Currency given an alphabetic ISO-4217 currency code.
    pub fn find_by_alpha_iso(code: String) -> Option<&'static Currency> {
        match CURRENCIES_BY_ALPHA_CODE.get(&code.to_uppercase()) {
            Some(c) => Some(c),
            None => None,
        }
    }

    /// Returns a Currency Hashmap, keyed by ISO alphabetic code.
    fn generate_currencies_by_alpha_code() -> HashMap<String, Currency> {
        let mut num_map: HashMap<String, Currency> = HashMap::new();

        for item in iso::ISO_CURRENCIES {
            let currency = iso::from_enum(item);
            num_map.insert(currency.code.to_string(), currency);
        }
        num_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn currency_find_known_can_be_found() {
        let currency_by_alpha = Currency::find("USD").unwrap();
        assert_eq!(currency_by_alpha.code, "USD");
        assert_eq!(currency_by_alpha.symbol, "$");
    }

    #[test]
    fn currency_find_unknown_code_raise_invalid_currency_error() {
        assert_eq!(Currency::find("AAA").unwrap_err(), Error::InvalidCurrency,);
    }
}
