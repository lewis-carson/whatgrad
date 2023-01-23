use whatgrad::Scope;

fn main() {
    let scope = Scope::new();
    let x = scope.value(7.0);
    let y = scope.value(11.0);

    let z = x * y;
    let grad = z.backwards();

    println!("dz/dx = {}", grad.wrt(x));
    println!("dz/dy = {}", grad.wrt(y));
}
