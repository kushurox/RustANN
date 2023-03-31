mod expressions;
use expressions::Value;

fn main(){
    let a = Value::new(3.0, Some("hello"));
    let b = Value::new(2.0, Some("Bye"));
    let c  = &a + &b;
    println!("{c}");
}