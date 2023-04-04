/*
    ops:
    0 - add
    1 - subtract
    2 - mul
    3 - divide
    4 - power

 */

use std::{rc::Rc, ops::{Add, Mul, Sub, Div}, cell::RefCell, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    None
}

#[derive(Debug, Clone, PartialEq)]
struct ValueData{
    val: f64,
    op: Op,
    from: Option<(Value, Value)>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value{
    ptr: Rc<RefCell<ValueData>>
}

impl Value{
    pub fn new(val: f64) -> Self{
        Value{
            ptr:Rc::new(RefCell::new(ValueData { val, op: Op::None, from: None }))
        }
    }
    fn new_from(val: f64, op: Op, lhs: Value, rhs: Value) -> Value{
        Value { ptr: Rc::new(RefCell::new(ValueData { val: val, op: op, from: Some((lhs, rhs)) })) }
    }
}



impl Value{
    pub fn pow(&self, exponent: &Value) -> Self{
        Value::new_from(self.ptr.borrow().val.powf(exponent.ptr.borrow().val), Op::Pow, self.clone(), exponent.clone())
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

                Op::Div => todo!(),
                Op::Pow => todo!(),
                Op::None => todo!(),
            }
        } else {
            return Value::new(0.0);
        }
    }

}



impl Display for Value{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vdata = self.ptr.borrow();
        if let Some((v1, v2)) = &vdata.from{
            write!(f, "Value({}, children: [{}, {}])", vdata.val, v1, v2)
            // write!(f, "Value({}, children: [])", vdata.val)

        } else {
            write!(f, "Value({}, children: [])", vdata.val)
        }
    }
}


mod operations;