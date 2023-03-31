/*
    ops:
    0 - add
    1 - subtract
    2 - mul
    3 - divide
    4 - power

 */

use std::{rc::Rc, ops::{Add, Mul, Sub, Div}, cell::RefCell, fmt::Display};

#[derive(Debug, Clone)]
struct ValueData{
    val: f64,
    op: Option<i8>,
    label: Option<&'static str>,
    from: Option<(Value, Value)>
}

#[derive(Debug, Clone)]
pub struct Value{
    ptr: Rc<RefCell<ValueData>>
}

impl Value{
    pub fn new(val: f64, label: Option<&'static str>) -> Self{
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



impl Display for Value{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vdata = self.ptr.borrow();
        if let Some((v1, v2)) = &vdata.from{
            if let Some(lbl) = vdata.label {
                write!(f, "Value({}, label='{}', children: [{}, {}])", vdata.val, lbl, v1, v2)
            } else {
                write!(f, "Value({}, children: [{}, {}])", vdata.val, v1, v2)
            }
        } else {
            if let Some(lbl) = vdata.label {
                write!(f, "Value({}, label='{}', children: [])", vdata.val, lbl)
            } else {
                write!(f, "Value({}, children: [])", vdata.val)
            }
        }
    }
}