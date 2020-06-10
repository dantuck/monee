use std::fmt;
use std::str::FromStr;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Money {
    amount: i64
}

impl FromStr for Money {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let mut amount: i64 = 0;

        if s != "" {
            let mut decimal_found: bool = false;
            let mut decimal_fill: u8 = 0;
            let mut decimal_places: u8 = 0;
            let mut last_num: i64 = 0;
            
            for char in s.chars() {
                if char == '.' {
                    decimal_found = true;
                    continue;
                }

                let num = char.to_digit(10)
                    .ok_or("Failing parsing string".to_string())? as i64;

                if decimal_found {
                    decimal_fill += 1;
                    decimal_places += 1;

                    if decimal_places == 3 {
                        if num > 4 {
                            // Add num
                            amount = amount.checked_add(1).ok_or("Failing parsing string".to_string())?;
                        }
                        decimal_places -= 1;
                        break;
                    }
                }

                last_num = num;

                // Multiply by 10 to pad right with one place
                amount = amount.checked_mul(10).ok_or("Failing parsing string".to_string())?;

                // Add num
                amount = amount.checked_add(num).ok_or("Failing parsing string".to_string())?;
            }

            if decimal_places == 2 && last_num != 0 {
                decimal_fill = 0;
            }

            if decimal_found && decimal_fill > 0 {
                loop {
                    amount *= 10;
                    decimal_fill -= 1;

                    if decimal_fill == 0 {
                        break;
                    }
                }
            }
        }

        Ok(Money { amount })
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("{:03}", self.amount);

        if self.amount == 0 {
            s = format!("{}", self.amount)
        } else {
            s.insert(s.len() - 2, '.');
        }
        write!(f, "{}", s.to_string())
    }
}

impl fmt::Debug for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

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

    let money: Money = "111.0".parse().unwrap();
    assert_eq!("111.00", money.to_string());

    let money: Money = "111.1".parse().unwrap();
    assert_eq!("111.10", money.to_string());

    let money: Money = "111.01".parse().unwrap();
    assert_eq!("111.01", money.to_string());

    let money: Money = "111.012".parse().unwrap();
    assert_eq!("111.01", money.to_string());

    let money: Money = "111.015".parse().unwrap();
    assert_eq!("111.02", money.to_string());

    let money: Money = "11123.0154".parse().unwrap();
    assert_eq!("11123.02", money.to_string());
}


#[test]
fn test_failing_creating_from_string() {
    let money = "111f.05f".parse::<Money>();
    assert_eq!(Err("Failing parsing string".to_string()), money);
}