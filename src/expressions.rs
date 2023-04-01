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
    label: Option<&'static str>,
    from: Option<(Value, Value)>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value{
    ptr: Rc<RefCell<ValueData>>
}

impl Value{
    pub fn new(val: f64, label: Option<&'static str>) -> Self{
        Value{
            ptr:Rc::new(RefCell::new(ValueData { val, op: Op::None, label, from: None }))
        }
    }
    fn new_from(val: f64, op: Op, lhs: Value, rhs: Value) -> Value{
        Value { ptr: Rc::new(RefCell::new(ValueData { val: val, op: op, label: None, from: Some((lhs, rhs)) })) }
    }

    pub fn set_label(&self, lbl: &'static str){
        self.ptr.borrow_mut().label = Some(lbl);
    }

    pub fn get_label(&self) -> &'static str {
        return self.ptr.borrow().label.unwrap_or("No Name");
    }
}

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

impl Value{
    pub fn pow(&self, exponent: &Value) -> Self{
        Value::new_from(self.ptr.borrow().val.powf(exponent.ptr.borrow().val), Op::Pow, self.clone(), exponent.clone())
    }

    pub fn diff(&self, dep: &Value) -> Value{
        // differentiates current varaible with respect to any other variable by using chain rule.
        // finds total derivative
        let curr = self.ptr.borrow();
        if self == dep {
            return Value::new(1.0, None);
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
                    &(&v1.diff(dep) * &v2) + &(&v2.diff(dep) * &v1)
                },

                Op::Div => todo!(),
                Op::Pow => todo!(),
                Op::None => todo!(),
            }
        } else {
            return Value::new(0.0, None);
        }
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