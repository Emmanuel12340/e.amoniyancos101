use std::f64;

fn main() {

    let a:f64 = 1.0;
    let b:f64 = -3.0;
    let c:f64 = 2.0;

    let discriminant = b * b -4.0 * a * c;

    if discriminant> 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b + discriminant.sqrt()) / (2.0 * a);
        println!("The equation has two real roots: {} and {}", root1, root2);
    } else if discriminant ==0.0 {
        let root = -b / (2.0 * a);
        println!("The equatio has one real root: {}", root)
    } else {
        let real_part = -b / (2.0 * a);
        let imaginary_part = (-discriminant).sqrt() / (2.0 * a);
        println!("The eguation has two complex roots: {} + {}i and {} - {}i",
                  real_part, imaginary_part, real_part, imaginary_part,);
    }
    
}
