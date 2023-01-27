use whatgrad::Scope;

fn main() {
    let scope = Scope::new();
    let x = scope.value(1);
    let y = scope.value(2);

    let z = x * y;

    let grad = z.backwards();

    println!("dz/dx = {}", grad.wrt(x));
}
