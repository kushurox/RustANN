use super::{Value, Op};

impl Value{
    pub fn pow(&self, exponent: &Value) -> Self{
        Value::new_from(self.ptr.borrow().val.powf(exponent.ptr.borrow().val), Op::Pow, self.clone(), exponent.clone())
    }

    pub fn powi(&self, exponent: i32) -> Value{
        Value::new_from(self.ptr.borrow().val.powi(exponent), Op::Pow, self.clone(), Value::new(exponent.into()))
    }

    pub fn diff(&self, dep: &Value) -> Value{
        // differentiates current varaible with respect to any other variable by using chain rule.
        // finds total derivative
        let curr = self.ptr.borrow();
        if self == dep {
            return Value::new(1.0);
        }
        if let Some((v1, v2)) = &curr.from {
            match curr.op {
                Op::Add => {
                    &v1.diff(dep) + &v2.diff(dep)
                },
                Op::Sub => {
                    &v1.diff(dep) - &v2.diff(dep)
                },
                Op::Mul => {
                    &(&v1.diff(dep) * v2) + &(&v2.diff(dep) * v1)
                },

                Op::Div => {
                    &((&v1.diff(&dep) * v2) - (&v2.diff(&dep)*v1))/&v2.powi(2)
                },
                Op::Pow => {
                    todo!()
                },
                Op::None => todo!(),
            }
        } else {
            return Value::new(0.0);
        }
    }


    pub fn grad(&self) -> () {
        let parent = self.ptr.borrow();
        if let Some((v1, v2)) = &parent.from {
            match parent.op {
                Op::Add => {
                    v1.ptr.borrow_mut().grad = parent.grad;   // actually its curr.grad * 1.0, but we know what happens if we * 1
                    v2.ptr.borrow_mut().grad = parent.grad;
                },
                Op::Sub => {
                    v1.ptr.borrow_mut().grad = parent.grad; // v1 - v2 so curr.grad and -curr.grad
                    v2.ptr.borrow_mut().grad = -parent.grad;
                },
                Op::Mul => {
                    v1.ptr.borrow_mut().grad = parent.grad * v2.ptr.borrow().val; // df/dx = df/df * df/d(mx) * d(mx)/dx
                    v2.ptr.borrow_mut().grad = parent.grad * v1.ptr.borrow().val;
                },
                Op::Div => {
                    todo!()
                },
                Op::Pow => todo!(),
                Op::None => todo!(),
            }
            v1.grad();
            v2.grad();
        }
    }

}

//  y = mx + b
//      dy/dx  =  m
//      dy/dm  =  x
//      dy/db  =  1
//      dy/dy  =  1

/*
f(x) = 3x + 2
f(5) = (3)(5) + 2 = 17


*/
