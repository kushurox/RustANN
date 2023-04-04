mod value;
// mod test;

use value::Value;

fn main(){
    let m = Value::new(3.0);
    let x = Value::new(5.0);
    let b = Value::new(3.2);

    let y = (&m * &x) + b; // <---- y = mx + b

    println!("{}", y.diff(&x));

}