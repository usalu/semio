use super::*;

#[test]
fn lerp_endpoints() {
    assert!((lerp(0.5, 0.0, 1.0, 0.0, 10.0) - 5.0).abs() < 1e-9);
}

#[test]
fn newton_finds_sqrt() {
    let r = newton_raphson(2.0, |x| x * x - 2.0, |x| 2.0 * x, 20, 1e-10).unwrap();
    assert!((r - std::f64::consts::SQRT_2).abs() < 1e-8);
}

#[test]
fn simpson_integrates_x_squared() {
    let integral = simpson_integrate(|x| x * x, 0.0, 1.0, 100);
    assert!((integral - 1.0 / 3.0).abs() < 1e-6);
}

#[test]
fn shared_dense_solution_and_inverse_agree() {
    let matrix = [4.0, 1.0, 0.5, 1.0, 3.0, -1.0, 0.5, -1.0, 5.0];
    let mut system = matrix;
    let mut rhs = [1.0, 2.0, 3.0];
    let mut cursor=semio_framework_math_dense::DenseSolveCursor::new(3,9,3);
    loop {let turn=cursor.step(&mut system,&mut rhs,semio_framework_math_dense::DenseSolveBudget {items:1,copy_bytes:64,depth:1,capacity_bytes:0});match turn.status {semio_framework_math_dense::DenseSolveStatus::Complete=>break,semio_framework_math_dense::DenseSolveStatus::Pending=>{},status=>panic!("{status:?}")}}
    let (mut inverse, mut work) = ([0.0; 9], [0.0; 9]);
    assert!(invert_dense(&matrix, &mut inverse, &mut work, 3));
    for row in 0..3 {
        let via_inverse: f64 = (0..3).map(|column| inverse[row * 3 + column] * [1.0, 2.0, 3.0][column]).sum();
        assert!((via_inverse - rhs[row]).abs() < 1e-12);
        let residual: f64 = (0..3).map(|column| matrix[row * 3 + column] * rhs[column]).sum::<f64>() - [1.0, 2.0, 3.0][row];
        assert!(residual.abs() < 1e-12);
    }
    assert!(!invert_dense(&[1.0, 2.0, 2.0, 4.0], &mut [0.0; 4], &mut [0.0; 4], 2));
}
