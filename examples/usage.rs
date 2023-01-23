use aura::Scope;

fn main() {
    let scope = Scope::new();
    let x = scope.value(1.0);
    let y = scope.value(1.0);

    let z = -2.0 * x + x * x * x * y;
    let grad = z.backprop();

    println!("dz/dx grad {}", grad.wrt(x));
}
