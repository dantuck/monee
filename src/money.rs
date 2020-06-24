use std::fmt;
use std::str::FromStr;

use crate::currency::*;
use crate::Error;

#[derive(PartialEq, Eq, Clone)]
pub struct Money {
    amount: i64,
    currency: &'static Currency,
    is_negative: bool,
}

#[macro_export]
macro_rules! money {
    ($x:expr, $y:expr) => {
        Money::from_string($x.to_string(), $y.to_string()).unwrap();
    };
}

impl Money {
    pub fn from_string(s: String, currency: String) -> Result<Money, Error> {
        let currency = Currency::from_string(currency).unwrap_or(Currency::find("USD")?);
        let mut amount: i64 = 0;
        let mut is_negative: bool = false;

        if s != "" {
            let mut decimal_found: bool = false;
            let mut decimal_fill: u8 = 0;
            let mut decimal_places: u8 = 0;
            
            for char in s.chars() {
                if char == '-' {
                    is_negative = true;
                    continue;
                }
                if char == '.' {
                    decimal_found = true;
                    continue;
                }

                let num = char.to_digit(10)
                    .ok_or(Error::InvalidAmount)? as i64;

                if decimal_found {
                    decimal_fill += 1;
                    decimal_places += 1;

                    if decimal_places == 3 {
                        if num > 4 {
                            // Add num
                            amount = amount.checked_add(1).ok_or(Error::InvalidAmount)?;
                        }
                        decimal_places -= 1;
                        break;
                    }
                }

                // Multiply by 10 to pad right with one place
                amount = amount.checked_mul(10).ok_or(Error::InvalidAmount)?;

                // Add num
                amount = amount.checked_add(num).ok_or(Error::InvalidAmount)?;
            }

            if !decimal_found {
                decimal_fill = 2;
            }

            if decimal_places == 2 {
                decimal_fill = 0;
            }

            if decimal_fill > 0 {
                loop {
                    amount *= 10;
                    decimal_fill -= 1;

                    if decimal_fill == 0 {
                        break;
                    }
                }
            }
        }

        Ok(Money { amount, currency, is_negative })
    }
}

impl FromStr for Money {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Money::from_string(s.to_string(), "".to_string())
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("{:03}", self.amount);

        if self.amount == 0 {
            s = format!("{}", self.amount)
        } else {
            s.insert(s.len() - 2, '.');

            if self.is_negative {
                s.insert(0, '-');
            }

            if self.currency.symbol != "" {
                let fill = f.fill();
                if let Some(width) = f.width() {
                    for _ in 0..width {
                        s.insert(0, fill)
                    }
                }
                s = format!("{}{}", self.currency.symbol, s)
            }
        }
        write!(f, "{}", s.to_string())
    }
}

impl fmt::Debug for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_from_string() {
        let money: Money = "".parse().unwrap();
        assert_eq!("0", money.to_string());

        let money: Money = "0".parse().unwrap();
        assert_eq!("0", money.to_string());

        let money: Money = "0.0".parse().unwrap();
        assert_eq!("0", money.to_string());

        let money: Money = "0.00".parse().unwrap();
        assert_eq!("0", money.to_string());

        let money: Money = "111".parse().unwrap();
        assert_eq!("$111.00", money.to_string());

        let money: Money = "111.0".parse().unwrap();
        assert_eq!("$111.00", money.to_string());

        let money: Money = "111.1".parse().unwrap();
        assert_eq!("$111.10", money.to_string());

        let money: Money = "111.01".parse().unwrap();
        assert_eq!("$111.01", money.to_string());

        let money: Money = "112.00".parse().unwrap();
        assert_eq!("$112.00", money.to_string());

        let money = money!("19.9999", "USD");
        assert_eq!("$20.00", format!("{}", money));

        let money: Money = money!("20.00", "USD");
        assert_eq!("$20.00", money.to_string())
    }

    #[test]
    fn test_failing_creating_from_string() {
        let money = "111f.05f".parse::<Money>();
        assert_eq!(Error::InvalidAmount, money.unwrap_err());
    }

    #[test]
    fn money_fmt_separates_digits() {
        let usd = money!(0, "USD"); // Zero Dollars
        let expected_usd_fmt = "0";
        assert_eq!(format!("{}", usd), expected_usd_fmt);
    }

    #[test]
    fn money_format_rounds_exponent() {
        // // 19.999 rounds to 20 for USD
        let money = money!("19.9999", "USD");
        assert_eq!("$20.00", format!("{}", money));

        // // 29.111 rounds to 29.11 for USD
        let money = money!("29.111", "USD");
        assert_eq!("$29.11", format!("{}", money));

        let money: Money = "11123.0154".parse().unwrap();
        assert_eq!("$11123.02", money.to_string());
    }

    #[test]
    fn money_format_padding() {
        let money = money!("20.00", "USD");
        assert_eq!("$ 20.00", format!("{: >1}", money));
    }

    #[test]
    fn money_from_float() {
        let money = money!(20, "USD");
        assert_eq!("$ 20.00", format!("{: >1}", money));

        let money = money!(20.10, "USD");
        assert_eq!("$ 20.10", format!("{: >1}", money));

        let money = money!(20.105, "USD");
        assert_eq!("$ 20.11", format!("{: >1}", money));
    }

    #[test]
    fn money_allow_negative() {
        let money = money!(-20, "USD");
        assert_eq!("$ -20.00", format!("{: >1}", money));

        let money: Money = "-20".parse().unwrap();
        assert_eq!("$-20.00", money.to_string());
    }
}