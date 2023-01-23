# whatgrad

`whatgrad` is a crate for computing the gradient of a function with respect to its arguments. Full writeup [here](https://lewiscarson.com/).

## Usage

```rust

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

```