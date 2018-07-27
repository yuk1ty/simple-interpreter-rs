use std::fmt;
use std::cell::RefCell;

pub struct Machine {
    pub expression: RefCell<Token>,
}

impl Machine {
    pub fn new(expression: Token) -> Self {
        Machine {
            expression: RefCell::new(expression),
        }
    }

    pub fn run(&self) {
        while self.expression.borrow().is_reducible() {
            println!("{}", self.expression.borrow());
            self.step();
        }

        println!("{}", self.expression.borrow());
    }

    fn step(&self) {
        self.expression
            .replace(self.expression.clone().into_inner().reduce());
    }
}

#[derive(Clone)]
pub enum Token {
    Number(i32),
    BoolValue(bool),
    Add(Box<Token>, Box<Token>),
    Multiply(Box<Token>, Box<Token>),
    LessThan(Box<Token>, Box<Token>),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Token::*;
        // TODO clean up
        match self {
            &Number(v) => write!(f, "{}", v),
            &BoolValue(ref b) => write!(f, "{}", b),
            &Add(ref blv, ref brv) => write!(f, "{} + {}", blv.to_string(), brv.to_string()),
            &Multiply(ref blv, ref brv) => write!(f, "{} * {}", blv.to_string(), brv.to_string()),
            &LessThan(ref blv, ref brv) => write!(f, "{} < {}", blv.to_string(), brv.to_string()),
        }
    }
}

impl Token {
    pub fn is_reducible(&self) -> bool {
        use Token::*;
        match *self {
            Number(_) => false,
            BoolValue(_) => false,
            Add(_, _) => true,
            Multiply(_, _) => true,
            LessThan(_, _) => true,
        }
    }

    pub fn reduce(&self) -> Token {
        use Token::*;
        match self {
            &Number(_) => panic!("Number token couldn't reduce!"),
            &BoolValue(_) => panic!("BoolValue token couldn't reduce!"),
            &Add(ref blv, ref brv) if blv.is_reducible() => {
                Add(Box::new(blv.reduce()), brv.clone())
            }
            &Add(ref blv, ref brv) if brv.is_reducible() => {
                Add(blv.clone(), Box::new(brv.reduce()))
            }
            &Add(ref blv, ref brv) => match **blv {
                Number(left_value) => match **brv {
                    Number(right_value) => Number(left_value + right_value),
                    _ => panic!("Unexpected error in Add!"),
                },
                _ => panic!("Unexpected error in Add!"),
            },
            &Multiply(ref blv, ref brv) if blv.is_reducible() => {
                Multiply(Box::new(blv.reduce()), brv.clone())
            }
            &Multiply(ref blv, ref brv) if brv.is_reducible() => {
                Multiply(blv.clone(), Box::new(brv.reduce()))
            }
            &Multiply(ref blv, ref brv) => match **blv {
                Number(left_value) => match **brv {
                    Number(right_value) => Number(left_value * right_value),
                    _ => panic!("Unexpected error in Multiply!"),
                },
                _ => panic!("Unexpected error in Multiply!"),
            },
            &LessThan(ref blv, ref brv) if blv.is_reducible() => {
                LessThan(Box::new(blv.reduce()), brv.clone())
            }
            &LessThan(ref blv, ref brv) if brv.is_reducible() => {
                LessThan(blv.clone(), Box::new(brv.reduce()))
            }
            &LessThan(ref blv, ref brv) => match **blv {
                Number(left_value) => match **brv {
                    Number(right_value) => BoolValue(left_value < right_value),
                    _ => panic!("Unexpected error in LessThan!"),
                },
                _ => panic!("Unexpected error in LessThan!"),
            },
        }
    }
}

fn main() {
    use Token::*;
    let actual = Add(
        Box::new(Multiply(Box::new(Number(1)), Box::new(Number(2)))),
        Box::new(Multiply(Box::new(Number(3)), Box::new(Number(4)))),
    );

    Machine::new(actual).run();
}

#[test]
fn test_to_string() {
    use Token::*;
    {
        let actual = Add(
            Box::new(Multiply(Box::new(Number(1)), Box::new(Number(2)))),
            Box::new(Multiply(Box::new(Number(3)), Box::new(Number(4)))),
        );

        let expected = "1 * 2 + 3 * 4";

        assert_eq!(actual.to_string(), expected);
    }

    {
        let bool_actual = BoolValue(true);
        let bool_expected = "true";

        assert_eq!(bool_actual.to_string(), bool_expected);
    }
}

#[test]
fn test_is_reducible() {
    use Token::*;
    let number_token = Number(1).is_reducible();
    let add_token = Add(Box::new(Number(1)), Box::new(Number(2))).is_reducible();
    let multiply_token = Multiply(Box::new(Number(1)), Box::new(Number(2))).is_reducible();
    let bool_token = BoolValue(true).is_reducible();

    assert_eq!(number_token, false);
    assert_eq!(add_token, true);
    assert_eq!(multiply_token, true);
    assert_eq!(bool_token, false);
}

#[test]
fn test_reduce() {
    use Token::*;
    {
        let expression = Add(
            Box::new(Multiply(Box::new(Number(1)), Box::new(Number(2)))),
            Box::new(Multiply(Box::new(Number(3)), Box::new(Number(4)))),
        );

        let reduced = expression.reduce();
        let reduced2 = reduced.reduce();
        let reduced3 = reduced2.reduce();

        assert_eq!(reduced.to_string(), "2 + 3 * 4");
        assert_eq!(reduced2.to_string(), "2 + 12");
        assert_eq!(reduced3.to_string(), "14");
    }

    {
        let expression = LessThan(
            Box::new(Number(5)),
            Box::new(Add(Box::new(Number(2)), Box::new(Number(2)))),
        );

        let reduced = expression.reduce();
        let reduced2 = reduced.reduce();

        assert_eq!(reduced.to_string(), "5 < 4");
        assert_eq!(reduced2.to_string(), "false");
    }
}

#[test]
fn test_vm() {
    use Token::*;
    let expression = Add(
        Box::new(Multiply(Box::new(Number(1)), Box::new(Number(2)))),
        Box::new(Multiply(Box::new(Number(3)), Box::new(Number(4)))),
    );

    let vm = Machine::new(expression).run();

    assert_eq!(vm, ());
}
