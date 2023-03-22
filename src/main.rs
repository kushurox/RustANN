mod expressions;
use expressions::Value;

fn main(){
    let a = Value::new(3.0, None);
    let b = Value::new(2.0, None);
    let c  = &a + &b;
    // println!("{c}");
}