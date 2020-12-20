use std::ops::{Add, Div, Mul, Neg, Sub};

use num::integer::{gcd, lcm};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Value {
    Integer(i64),
    Rational {
        quotient: i64,
        remainder: i64,
        divisor: i64,
    },
}
impl Value {
    pub fn simplify(self) -> Self {
        #[cfg(test)]
        println!("Simplifying: {:#?}", self);
        match self {
            Self::Integer(_) => self,
            Self::Rational {
                mut quotient,
                mut remainder,
                mut divisor,
            } => {
                let common = gcd(remainder, divisor);
                if common != divisor {
                    remainder /= common;
                    divisor /= common;
                }

                if remainder / divisor != 0 {
                    quotient += remainder / divisor;
                    remainder -= divisor * (remainder / divisor);
                }

                if remainder != 0 {
                    #[cfg(test)]
                    println!(
                        "Result: {:#?}",
                        Self::Rational {
                            quotient,
                            remainder,
                            divisor,
                        }
                    );
                    Self::Rational {
                        quotient,
                        remainder,
                        divisor,
                    }
                } else {
                    #[cfg(test)]
                    println!("Result: {:#?}", Self::Integer(quotient));
                    Self::Integer(quotient)
                }
            }
        }
    }
}
impl PartialEq<i64> for Value {
    fn eq(&self, other: &i64) -> bool {
        match self {
            Self::Integer(i) => i == other,
            Self::Rational { .. } => false,
        }
    }
}
impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Self::Integer(i)
    }
}
impl From<f64> for Value {
    fn from(f: f64) -> Self {
        match fraction::GenericFraction::<i64>::from(f) {
            fraction::GenericFraction::Rational(_, ratio) => Self::Rational {
                quotient: 0,
                remainder: *ratio.numer(),
                divisor: *ratio.denom(),
            },
            _ => panic!(),
        }
    }
}
impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs + rhs),
            (
                Self::Rational {
                    quotient,
                    remainder,
                    divisor,
                },
                Self::Integer(rhs),
            )
            | (
                Self::Integer(rhs),
                Self::Rational {
                    quotient,
                    remainder,
                    divisor,
                },
            ) => Self::Rational {
                quotient: quotient + rhs,
                remainder,
                divisor,
            },
            (
                Self::Rational {
                    quotient: lhs_quotient,
                    remainder: lhs_remainder,
                    divisor: lhs_divisor,
                },
                Self::Rational {
                    quotient: rhs_quotient,
                    remainder: rhs_remainder,
                    divisor: rhs_divisor,
                },
            ) => {
                let divisor = lcm(lhs_divisor, rhs_divisor);
                let quotient = lhs_quotient + rhs_quotient;
                let remainder = (lhs_remainder * divisor.checked_div(lhs_divisor).unwrap_or(1))
                    + (rhs_remainder * divisor.checked_div(rhs_divisor).unwrap_or(1));

                Self::Rational {
                    quotient,
                    remainder,
                    divisor,
                }
                .simplify()
            }
        }
    }
}
impl Sub for Value {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs - rhs),
            (
                Self::Rational {
                    quotient,
                    remainder,
                    divisor,
                },
                Self::Integer(rhs),
            )
            | (
                Self::Integer(rhs),
                Self::Rational {
                    quotient,
                    remainder,
                    divisor,
                },
            ) => Self::Rational {
                quotient: quotient - rhs,
                remainder,
                divisor,
            },
            (
                Self::Rational {
                    quotient: lhs_quotient,
                    remainder: lhs_remainder,
                    divisor: lhs_divisor,
                },
                Self::Rational {
                    quotient: rhs_quotient,
                    remainder: rhs_remainder,
                    divisor: rhs_divisor,
                },
            ) => {
                let divisor = lcm(lhs_divisor, rhs_divisor);
                let quotient = lhs_quotient - rhs_quotient;
                let remainder = (lhs_remainder * divisor.checked_div(lhs_divisor).unwrap_or(1))
                    - (rhs_remainder * divisor.checked_div(rhs_divisor).unwrap_or(1));

                Self::Rational {
                    quotient,
                    remainder,
                    divisor,
                }
                .simplify()
            }
        }
    }
}
impl Mul for Value {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs * rhs),
            (
                Self::Rational {
                    quotient,
                    remainder,
                    divisor,
                },
                Self::Integer(rhs),
            )
            | (
                Self::Integer(rhs),
                Self::Rational {
                    quotient,
                    remainder,
                    divisor,
                },
            ) => {
                Self::Rational {
                    quotient,
                    remainder,
                    divisor,
                } * Self::Rational {
                    quotient: rhs,
                    remainder: 0,
                    divisor: 1,
                }
            }
            (
                Self::Rational {
                    quotient: lhs_quotient,
                    remainder: lhs_remainder,
                    divisor: lhs_divisor,
                },
                Self::Rational {
                    quotient: rhs_quotient,
                    remainder: rhs_remainder,
                    divisor: rhs_divisor,
                },
            ) => {
                let remainder = ((lhs_quotient * lhs_divisor) + lhs_remainder)
                    * ((rhs_quotient * rhs_divisor) + rhs_remainder);
                let divisor = lhs_divisor * rhs_divisor;

                Self::Rational {
                    quotient: 0,
                    remainder,
                    divisor,
                }
                .simplify()
            }
        }
    }
}
impl Div for Value {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Rational {
                quotient: 0,
                remainder: lhs,
                divisor: rhs,
            }
            .simplify(),
            (
                Self::Rational {
                    quotient,
                    remainder,
                    divisor,
                },
                Self::Integer(rhs),
            )
            | (
                Self::Integer(rhs),
                Self::Rational {
                    quotient,
                    remainder,
                    divisor,
                },
            ) => {
                Self::Rational {
                    quotient,
                    remainder,
                    divisor,
                } / Self::Rational {
                    quotient: rhs,
                    remainder: 0,
                    divisor: 1,
                }
            }
            (
                Self::Rational {
                    quotient: lhs_quotient,
                    remainder: lhs_remainder,
                    divisor: lhs_divisor,
                },
                Self::Rational {
                    quotient: rhs_quotient,
                    remainder: rhs_remainder,
                    divisor: rhs_divisor,
                },
            ) => {
                let remainder = ((lhs_quotient * lhs_divisor) + lhs_remainder) * rhs_divisor;
                let divisor = lhs_divisor * ((rhs_quotient * rhs_divisor) + rhs_remainder);

                Self::Rational {
                    quotient: 0,
                    remainder,
                    divisor,
                }
                .simplify()
            }
        }
    }
}
impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(i) => Self::Integer(-i),
            Self::Rational {
                mut quotient,
                remainder,
                divisor,
            } => {
                if quotient == 0 {
                    quotient = -1;
                } else {
                    quotient = -quotient;
                }
                Self::Rational {
                    quotient,
                    remainder,
                    divisor,
                }
            }
        }
    }
}
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::Rational {
                quotient,
                remainder,
                divisor,
            } => {
                write!(f, "{} ({} / {})", quotient, remainder, divisor)
            }
        }
    }
}
