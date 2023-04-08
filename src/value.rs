/*
    ops:
    0 - add
    1 - subtract
    2 - mul
    3 - divide
    4 - power

 */

use std::{rc::Rc, cell::RefCell, fmt::Display};

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
    from: Option<(Value, Value)>,
    grad: f64
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value{
    ptr: Rc<RefCell<ValueData>>
}

impl Value{
    pub fn new(val: f64) -> Self{
        Value{
            ptr:Rc::new(RefCell::new(ValueData { val, op: Op::None, from: None, grad: 1.0}))
        }
    }
    fn new_from(val: f64, op: Op, lhs: Value, rhs: Value) -> Value{
        Value { ptr: Rc::new(RefCell::new(ValueData { val: val, op: op, from: Some((lhs, rhs)), grad: 1.0})) }
    }
}


impl Display for Value{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.ptr.borrow();
        write!(f, "Value({}, grad={})", v.val, v.grad)
    }
}

impl Value{

}


mod operations;
mod grad;