use num_traits::{Float, ToPrimitive};

fn solve<T: Float, U: Float>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn solve2<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}


fn main() {
    let a: f64 = 3.0;
    let b: f32 = 4.0;

    let c = 1;
    let d: i64 = 5;
    // or a as f64
    println!("{}", solve(a, b));

    println!("{}", solve2(a, d));
}
