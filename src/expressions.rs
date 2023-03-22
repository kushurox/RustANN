/*
    ops:
    0 - add
    1 - subtract
    2 - mul
    3 - divide
    4 - power

 */

use std::{rc::Rc, ops::{Add, Mul, Sub, Div}, cell::RefCell};

#[derive(Debug, Clone)]
struct ValueData{
    val: f64,
    op: Option<i8>,
    label: Option<String>,
    from: Option<(Value, Value)>
}

#[derive(Debug, Clone)]
pub struct Value{
    ptr: Rc<RefCell<ValueData>>
}

impl Value{
    pub fn new(val: f64, label: Option<String>) -> Self{
        Value{
            ptr:Rc::new(RefCell::new(ValueData { val, op: None, label, from: None }))
        }
    }
    fn new_from(val: f64, op: i8, lhs: Value, rhs: Value) -> Value{
        Value { ptr: Rc::new(RefCell::new(ValueData { val: val, op: Some(op), label: None, from: Some((lhs, rhs)) })) }
    }
}

impl Add for &Value{
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        Value::new_from(self.ptr.borrow().val + rhs.ptr.borrow().val, 0, self.clone(), rhs.clone())
    }
}

impl Mul for &Value{
    type Output = Value;
    fn mul(self, rhs: Self) -> Self::Output {
        Value::new_from(self.ptr.borrow().val * rhs.ptr.borrow().val, 2, self.clone(), rhs.clone())
    }
}

impl Sub for &Value{
    type Output = Value;
    fn sub(self, rhs: Self) -> Self::Output {
        Value::new_from(self.ptr.borrow().val - rhs.ptr.borrow().val, 1, self.clone(), rhs.clone())
    }
}

impl Div for &Value{
    type Output = Value;
    fn div(self, rhs: Self) -> Self::Output {
        Value::new_from(self.ptr.borrow().val / rhs.ptr.borrow().val, 3, self.clone(), rhs.clone())
    }
}

impl Value{
    pub fn pow(&self, exponent: &Value) -> Self{
        Value::new_from(self.ptr.borrow().val.powf(exponent.ptr.borrow().val), 4, self.clone(), exponent.clone())
    }
}


impl std::fmt::Display for Value{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!(); // rewrite Option<String> to use Option<&str>
        
        if let Some(a) = &self.ptr.borrow().from {
            write!(f, "Value({}, children:({}, {}), label:)", self.ptr.borrow().val, a.0, a.1)

        }
        else {
            write!(f, "Value({}, label:)", self.ptr.borrow().val)
        }
    }
    
}