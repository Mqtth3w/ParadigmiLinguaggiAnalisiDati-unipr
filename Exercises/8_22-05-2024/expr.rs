
//not completed yet
pub struct Num {
    val: i32,
}
impl Num {
    pub fn new(val: &i32) -> Num {
        Num{ val: val }
    }
    pub fn eval(&self) -> Num {
        println!("{}", self.val)
    }
}
pub struct Sum {
    val1: Num,
    val2: Num,
}
impl Sum {
    pub fn new(val1: Num, val2: Num) -> Num {
        Sum{ val1: val1, val2: val2 }
    }
    pub fn eval(&self) {
        println!("{} + {} = {} ", self.val1.eval(), self.val2.eval(), self.val1.eval() + self.val2.eval())
    }
}
pub struct Mul {
    val1: Num,
    val2: Num,
}
impl Mul {
    pub fn new(val1: Num, val2: Num) -> Num {
        Mul{ val1: val1, val2: val2 }
    }
    pub fn eval(&self) {
        println!("{} * {} = {} ", self.val1.eval(), self.val2.eval(), self.val1.eval() * self.val2.eval())
    }
}
pub trait Expr {
    pub exprs: Vec<T>
}
impl<T: Expr> {
    pub fn eval(&self) {
        for e in self.exprs.iter() {
            e.eval();
        }
    }
}

fn main() {
    let v: Vec<Box<dyn Expr>> = vec![
        Box::new(Mul::new(5, 9)),
        Box::new(Sum::new(4, 5))];
    for a in v { a.eval(); }
}
