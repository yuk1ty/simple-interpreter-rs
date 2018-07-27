use std::fmt;

#[derive(Clone)]
pub enum Token {
    Number(i32),
    Add(Box<Token>, Box<Token>),
    Multiply(Box<Token>, Box<Token>),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Token::*;
        // TODO clean up
        match self {
            &Number(v) => write!(f, "{}", v),
            &Add(ref blv, ref brv) => write!(f, "{} + {}", blv.to_string(), brv.to_string()),
            &Multiply(ref blv, ref brv) => write!(f, "{} * {}", blv.to_string(), brv.to_string()),
        }
    }
}

impl Token {
    pub fn is_reducible(&self) -> bool {
        use Token::*;
        match *self {
            Number(_) => false,
            Add(_, _) => true,
            Multiply(_, _) => true,
        }
    }

    pub fn reduce(&self) -> Token {
        use Token::*;
        match self.clone() {
            Number(_) => panic!("Number token couldn't reduce!"),
            Add(ref blv, ref brv) if blv.is_reducible() => Add(Box::new(blv.reduce()), brv.clone()),
            Add(ref blv, ref brv) if brv.is_reducible() => Add(blv.clone(), Box::new(brv.reduce())),
            Add(blv, brv) => match *blv {
                Number(left_value) => match *brv {
                    Number(right_value) => Number(left_value + right_value),
                    _ => panic!("Unexpected error in Add!"),
                },
                _ => panic!("Unexpected error in Add!"),
            },
            Multiply(ref blv, ref brv) if blv.is_reducible() => {
                Multiply(Box::new(blv.reduce()), brv.clone())
            }
            Multiply(ref blv, ref brv) if brv.is_reducible() => {
                Multiply(blv.clone(), Box::new(brv.reduce()))
            }
            Multiply(blv, brv) => match *blv {
                Number(left_value) => match *brv {
                    Number(right_value) => Number(left_value * right_value),
                    _ => panic!("Unexpected error in Multiply!"),
                },
                _ => panic!("Unexpected error in Multiply!"),
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
    println!("{}", actual);
    println!("Hello, world!");
}

#[test]
fn test_to_string() {
    use Token::*;
    let actual = Add(
        Box::new(Multiply(Box::new(Number(1)), Box::new(Number(2)))),
        Box::new(Multiply(Box::new(Number(3)), Box::new(Number(4)))),
    );

    let expected = "1 * 2 + 3 * 4";

    assert_eq!(actual.to_string(), expected);
}

#[test]
fn test_is_reducible() {
    use Token::*;
    let number_token = Number(1).is_reducible();
    let add_token = Add(Box::new(Number(1)), Box::new(Number(2))).is_reducible();
    let multiply_token = Multiply(Box::new(Number(1)), Box::new(Number(2))).is_reducible();

    assert_eq!(number_token, false);
    assert_eq!(add_token, true);
    assert_eq!(multiply_token, true);
}

#[test]
fn test_reduce() {
    use Token::*;
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
