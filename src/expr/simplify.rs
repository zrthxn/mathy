// src/calc/simplify.rs

use crate::expr::expr::*;
use crate::expr::expr::Expr::*;
use crate::number::number::*;


pub fn simplify(e: Expr) -> Expr {
    match e {
        Neg(ref i) => {
            neg(simplify(unpack(i)))
        },
        Add(ref l, ref r) => {
            let left = simplify(unpack(l));
            let right = simplify(unpack(r));

            match (left, right) {
                (Const(l), Const(r)) => Const(l + r),
                (Const(x), b) => {
                    if x.is_zero() {
                        simplify(b)
                    } else {
                        add(Const(x), simplify(b))
                    }
                },
                (a, Const(x)) => {
                    if x.is_zero() {
                        simplify(a)
                    } else {
                        add(Const(x), simplify(a))
                    }
                },

		// now we look for Euler's trig identity
		(Pow(ref l, ref p1), Pow(ref r, ref p2)) => {
		    let left = unpack(l);
		    let right = unpack(r);
		    let power1 = unpack(p1);
		    let power2 = unpack(p2);

		    if power1 == con(2.0) && power2 == con(2.0) {
			match (left, right) {
			    (Cos(ref a), Sin(ref b)) => {
				let a1 = unpack(a);
				let b1 = unpack(b);
				if a1 == b1 {
				    con(1.0)
				} else {
				   add(pow(a1, power1), pow(b1, power2)) 
				}
			    },
			    (Sin(ref a), Cos(ref b)) => {
				let a1 = unpack(a);
				let b1 = unpack(b);
				if a1 == b1 {
				    con(1.0)
				} else {
				   add(pow(a1, power1), pow(b1, power2)) 
				}
			    }
			    (a, b) => add(pow(a, power1), pow(b, power2))
			}
		    } else {
			add(pow(left, power1), pow(right, power2))
		    }
		},
                (a, b)  => add(a, b),
            }
        }, // end addition addition logic 

        Sub(ref l, ref r) => {
            let left = simplify(unpack(l));
            let right = simplify(unpack(r));
            match (left, right) {
                (Const(a), Const(b)) => Const(a - b),
                (Const(x), b) => {
                    if x.is_zero() {
                        neg(simplify(b))
                    } else {
                        sub(Const(x), simplify(b))
                    }
                },
                (a, Const(x)) => {
                    if x.is_zero() {
                        simplify(a)
                    } else {
                        sub(simplify(a), Const(x))
                    }
                },
                (a, Neg(i)) => {
                    let inner = unpack(&i);
                    simplify(add(simplify(a), simplify(inner))) 
                },
                (a, b) => sub(simplify(a), simplify(b)),
            }
        }, // end subtraction logic 

        Mul(ref l, ref r) => {
            let left = unpack(l);
            let right = unpack(r);

            match (left, right) {
                (Const(x), Const(y)) => Const(x * y),
                (Const(x), b) => {
                    if x.is_zero() {
                        zero()
                    } else if x.real_eq(1.0) {
                        simplify(b)
                    } else if x.real_eq(-1.0) {
                        neg(simplify(b))
                    } else {
                        mul(Const(x), simplify(b))
                    }
                },
                (a, Const(y)) => {
                    if y.is_zero() {
                        zero()
                    } else if y.real_eq(1.0) {
                        simplify(a)
                    } else if y.real_eq(-1.0) {
                        neg(simplify(a))
                    } else {
                       mul(Const(y), simplify(a)) 
                    }
                },
		(left, Neg(r)) => {
		    let right = unpack(&r);
		    if left == right {
			neg(powf(left, 2.0))
		    } else {
			mul(left, Neg(r))
		    }
		},
		(Neg(l), right) => {
		    let left = unpack(&l);
		    if left == right {
			neg(powf(left, 2.0))
		    } else {
			mul(Neg(l), right)
		    }
		},
		(Var(a), Var(b)) => {
		    if a == b {
			powf(var(a), 2.0)
		    } else {
			mul(var(a), var(b))
		    }
		},
		(left, Pow(b, p)) => {
		    let right = unpack(&b);
		    let power = unpack(&p);
		    if left == right {
			pow(left, power + 1.0)
		    } else {
			mul(left, pow(right, power))
		    }
		},
		(Pow(a, p), right) => {
		    let left = unpack(&a);
		    let power = unpack(&p);
		    if left == right {
			pow(right, power + 1.0)
		    } else {
			mul(right, pow(left, power))
		    }
		},
                (a, b) => {
		    if a == b {
			powf(a, 2.0)
		    } else {
			mul(simplify(a), simplify(b))
		    }
		},
            }
        }, // end multiplication logic

        Div(ref l, ref r) => {
            let left = unpack(l);
            let right = unpack(r);

            match (left, right) {
                (numerator, Const(x)) => {
                    if x.real_eq(1.0) {
                        simplify(numerator)   
                    } else if x.real_eq(0.0) {
                        Const(nan())
                    } else {
                        div(simplify(numerator), Const(x))
                    }
                },

                (a, b) => div(simplify(a), simplify(b)),
            }
        }, // end division logic

	Pow(ref l, ref r) => {
	    let left = unpack(l);
	    let right = unpack(r);

	    match (left, right) {
		(Const(b), Const(p)) => Const(b.pow(p)),
		(a, Const(b)) => {
		    if b.real_eq(1.0) {
			simplify(a)
		    } else {
			pow(simplify(a), simplify(Const(b)))
		    }
		}
		(a, b) => pow(simplify(a), simplify(b)),
	    }
	},

	Exp(ref i) => {
	    let inner = unpack(i);

	    match inner {
		// e^x and ln(x) are inverse functions
		Ln(ref a) => {
		    unpack(a)
		} 
		a => exp(a),
	    }
	},

        _ => e,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::number::number::Number::*;

    const ZERO : Expr = Const(Real(0.0));
    const ONE : Expr = Const(Real(1.0));

    #[test]
    fn test_add_simplify() {
        let tests = vec![
            zero() + con(1.0),
            con(1.0) + zero(),
            zero() + zero(),
            con(1.0) + con(1.0),
        ];

        let answers = vec![
            ONE, ONE, ZERO, con(2.0), 
        ];

        for (l, r) in tests.iter().zip(answers.iter()) {
            println!("Left: {:?}", l);
            println!("Right: {:?}", r);
            println!("Simplified left: {:?}", simplify(l.clone()));
            assert_eq!(simplify(l.clone()), r.clone());
        }
    }

    #[test]
    fn test_e_ln_x() {
	let input = exp(ln(var('x')));
	let output = simplify(input);
	let expected = var('x');

	assert_eq!(output, expected);
    }

}


// end src/calc/simplify.rs
