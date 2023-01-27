use whatgrad::Scope;

#[test]
fn addition() {
    let scope = Scope::new();

    let x = scope.value(7.0);
    let y = scope.value(11.0);

    let z = x + y;

    let grad = z.backwards();

    assert_eq!(grad.wrt(x), 1.0);
}

#[test]
fn multiplication() {
    let scope = Scope::new();

    let x = scope.value(7.0);
    let y = scope.value(11.0);

    let z = x * y;

    let grad = z.backwards();

    assert_eq!(grad.wrt(x), 11.0);
}

#[test]
fn subtraction() {
    let scope = Scope::new();

    let x = scope.value(7.0);
    let y = scope.value(11.0);

    let z = x - y;

    let grad = z.backwards();

    assert_eq!(grad.wrt(x), 1.0);
    assert_eq!(grad.wrt(y), -1.0);
}

#[test]
fn division() {
    let scope = Scope::new();

    let x = scope.value(7.0);
    let y = scope.value(11.0);

    let z = x / y;

    let grad = z.backwards();

    assert_eq!(grad.wrt(x), 1.0 / 11.0);
    assert_eq!(grad.wrt(y), -7.0 / 11.0 / 11.0);
}

#[test]
fn negation() {
    let scope = Scope::new();

    let x = scope.value(7.0);

    let z = -x;

    let grad = z.backwards();

    assert_eq!(grad.wrt(x), -1.0);
}

#[test]
fn composition() {
    let scope = Scope::new();

    let x = scope.value(7.0);
    let y = scope.value(11.0);

    let z = (x + y) * (x - y);

    let grad = z.backwards();

    assert_eq!(grad.wrt(x), 2.0 * 7.0);
    assert_eq!(grad.wrt(y), -(2.0 * 11.0));
}
