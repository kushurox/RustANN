mod expressions;
use expressions::Value;

fn main(){
    let m = Value::new(2.0, None);
    let x = Value::new(3.0, None);
    let b = Value::new(1.0, None);
    let y = &(&m * &x) + &b;
    println!("{} \n{} \n{}", y.diff(&x), y.diff(&m), y.diff(&b));
}