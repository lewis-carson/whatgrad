use aura::{Scope, Value};

fn main() {
    let scope = Scope::new();
    let x = scope.value(1.0);
    let y = scope.value(1.0);

    let z = -2.0 * x + x * x * x * y + 2.0 * y;
    let grad = z.backprop();
    println!(
        "dz/dx of z = -2x + x^3 * y + 2y at x=1.0, y=1.0 is {}",
        grad.wrt(x)
    );
    println!(
        "dz/dy of z = -2x + x^3 * y + 2y at x=1.0, y=1.0 is {}",
        grad.wrt(y)
    );
}
