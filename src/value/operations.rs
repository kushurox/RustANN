use super::{Value, Op};
use std::{ops::{Add, Mul, Sub, Div}};

impl Add for &Value{
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        Value::new_from(self.ptr.borrow().val + rhs.ptr.borrow().val, Op::Add, self.clone(), rhs.clone())
    }
}

impl Mul for &Value{
    type Output = Value;
    fn mul(self, rhs: Self) -> Self::Output {
        Value::new_from(self.ptr.borrow().val * rhs.ptr.borrow().val, Op::Mul, self.clone(), rhs.clone())
    }
}

impl Sub for &Value{
    type Output = Value;
    fn sub(self, rhs: Self) -> Self::Output {
        Value::new_from(self.ptr.borrow().val - rhs.ptr.borrow().val, Op::Sub, self.clone(), rhs.clone())
    }
}

impl Div for &Value{
    type Output = Value;
    fn div(self, rhs: Self) -> Self::Output {
        Value::new_from(self.ptr.borrow().val / rhs.ptr.borrow().val, Op::Div, self.clone(), rhs.clone())
    }
}


impl Add<Value> for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let temp = self.ptr.borrow().val + rhs.ptr.borrow().val;
        Value::new_from(temp, Op::Add, self, rhs)
    }
}

impl Sub<Value> for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let temp = self.ptr.borrow().val - rhs.ptr.borrow().val;
        Value::new_from(temp, Op::Sub, self, rhs)
    }
}

impl Mul<Value> for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let temp = self.ptr.borrow().val * rhs.ptr.borrow().val;
        Value::new_from(temp, Op::Mul, self, rhs)
    }
}

impl Div<Value> for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let temp = self.ptr.borrow().val / rhs.ptr.borrow().val;
        Value::new_from(temp, Op::Div, self, rhs)
    }
}

impl Add<Value> for &Value{
    type Output = Value;
    fn add(self, rhs: Value) -> Self::Output {
        let temp = self.ptr.borrow().val + rhs.ptr.borrow().val;
        Value::new_from(temp, Op::Add, self.clone(), rhs)
    }
}

impl Mul<Value> for &Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Value {
        let temp = self.ptr.borrow().val * rhs.ptr.borrow().val;
        Value::new_from(temp, Op::Mul, self.clone(), rhs)
    }
}