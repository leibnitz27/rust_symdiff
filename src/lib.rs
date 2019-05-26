use std::fmt;
use std::rc::Rc;
/*
 * A trivial symbolic differentiation engine, as a Rust learning exercise.
 *
 */
pub enum Expression {
    Lit(i32),
    Var(char),
    Add(Rc<Expression>, Rc<Expression>),
    Sub(Rc<Expression>, Rc<Expression>),
    Mul(Rc<Expression>, Rc<Expression>),
    Div(Rc<Expression>, Rc<Expression>),
    SimpleExp(Rc<Expression>, i32)
}

impl Expression {
    pub fn lit(val : i32) -> Rc<Expression> {
        Rc::new(Expression::Lit(val))
    }

    pub fn var(name : char) -> Rc<Expression> {
        Rc::new(Expression::Var(name))
    }

    pub fn add(lhs : Rc<Expression>, rhs : Rc<Expression>) -> Rc<Expression> {
        match (&*lhs, &*rhs) {
            (Expression::Lit(0), _) => rhs,
            (_, Expression::Lit(0)) => lhs,
            _ => Rc::new(Expression::Add(lhs, rhs))
        }
    }

    pub fn sub(lhs : Rc<Expression>, rhs : Rc<Expression>) -> Rc<Expression> {
        Rc::new(Expression::Sub(lhs, rhs))
    }

    pub fn mul(lhs : Rc<Expression>, rhs : Rc<Expression>) -> Rc<Expression> {
        match (&*lhs, &*rhs) {
            (Expression::Lit(0), _) => lhs,
            (Expression::Lit(1), _) => rhs,
            (_, Expression::Lit(0)) => rhs,
            (_, Expression::Lit(1)) => lhs,
            _ => Rc::new(Expression::Mul(lhs, rhs))
        }
    }

    pub fn div(lhs : Rc<Expression>, rhs : Rc<Expression>) -> Rc<Expression> {
        Rc::new(Expression::Div(lhs, rhs))
    }

    pub fn exp(base : Rc<Expression>, power : Rc<Expression>) -> Rc<Expression> {
        match *power {
            Expression::Lit(p) => Rc::new(Expression::SimpleExp(base, p)),
            _ => panic!("Can't handle non literal exponents")
        }
    }

    pub fn differentiate(&self) -> Rc<Expression> {
        match self {
            Expression::Lit(_) => Expression::lit(0),
            Expression::Var('x') => Expression::lit(1),
            Expression::Var(_) => Expression::lit(0),
            Expression::Add(a,b) => {
                Expression::add(a.differentiate(),b.differentiate())
            },
            Expression::Sub(a,b) => {
                Expression::sub(a.differentiate(),b.differentiate())
            },
            Expression::Mul(a,b) => {
                let lhs = Expression::mul(a.clone(),b.differentiate());
                let rhs = Expression::mul(b.clone(),a.differentiate());
                Expression::add(lhs, rhs)
            },
            Expression::Div(a,b) => {
                Expression::div(
                    Expression::sub(
                        Expression::mul(b.clone(),a.differentiate()),
                        Expression::mul(a.clone(),b.differentiate())
                    ),
                    Expression::exp(b.clone(), Expression::lit(2))
                )
            },
            Expression::SimpleExp(b,p) => {
                Expression::mul(
                    Expression::mul(
                        Expression::lit(*p),
                        Expression::exp(b.clone(), Expression::lit(p-1))
                    ), b.differentiate()
                )
            }
        }
    }
}

// TODO : Bracketing doesn't work if you use write like this, because it doesn't obey precedence.
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Expression::Lit(v) => write!(f, "{}", v),
            Expression::Var(c) => write!(f, "{}", c),
            Expression::Add(a,b) => write!(f, "{}+{}", a,b),
            Expression::Sub(a,b) => write!(f, "{}-{}", a,b),
            Expression::Mul(a,b) => write!(f, "{}*{}", a,b),
            Expression::Div(a,b) => write!(f, "{}/{}", a,b),
            Expression::SimpleExp(b,p) => write!(f, "{}^{}", b, p),
        }
    }
}
