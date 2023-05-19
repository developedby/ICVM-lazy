use crate::main;
use crate::term;

pub fn test() {

  let code = "
// Church multiplication
def mul = λn λm λs (n (m s))

// Church nats
def c1 = λf λx (f x)
def c2 = λf λx (dup #b f0 f1 = f; (f0 (f1 x)))
def c3 = λf λx (dup #c f0 f1 = f; dup #c f2 f3 = f0; (f1 (f2 (f3 x))))

// Church powers of two
def p1 = c2          // 2
def p2 = (mul c2 p1) // 4
def p3 = (mul c2 p2) // 8
def p4 = (mul c2 p3) // 16
def p5 = (mul c2 p4) // 32
def p6 = (mul c2 p5) // 64
def p7 = (mul c2 p6) // 128
def p8 = (mul c2 p7) // 256

// Booleans
def true = λt λf t
def false = λt λf f
def not = λb ((b false) true)
def neg = λb λt λf (b f t)

// Negates 'true' 256 times: 'neg' is faster than 'not' because it fuses
//λa λb λc
//dup a0 a1 = a

λx λy (x y (c3 not true))
";

  let term = term::from_string(code.as_bytes());
  let (norm, rules) = term::normal_with_stats(&term);

  println!("{}\n", norm);
  println!("{:?} rewrites", rules);

}