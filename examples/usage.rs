use whatgrad::Scope;

fn main() {
    let scope = Scope::new();
    let x = scope.value(7.0);
    let y = scope.value(11.0);

    let z = x / 5.0;
    let grad = z.backwards();

    println!("dz/dx = {}", grad.wrt(x));
}
