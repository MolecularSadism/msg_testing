use msg_testing::assert_approx_eq;

#[test]
fn passes_inside_default_tolerance() {
    assert_approx_eq!(1.0_f32, 1.000_05);
    assert_approx_eq!(-3.5_f32, -3.500_04);
    assert_approx_eq!(0.0_f32, 0.0);
}

#[test]
#[should_panic(expected = "1.0 !~= 1.5")]
fn panics_outside_default_tolerance() {
    assert_approx_eq!(1.0_f32, 1.5);
}

#[test]
fn passes_with_custom_tolerance() {
    assert_approx_eq!(1.0_f32, 1.05, 0.1);
    assert_approx_eq!(100.0_f32, 99.0, 2.0);
}

#[test]
#[should_panic(expected = "(tolerance: 0.01)")]
fn panics_outside_custom_tolerance() {
    assert_approx_eq!(1.0_f32, 1.05, 0.01);
}

#[test]
fn f64_operands() {
    assert_approx_eq!(2.0_f64, 2.0 + 5e-5);
    assert_approx_eq!(2.0_f64, 2.5, 0.6);
}

#[test]
#[should_panic(expected = "2.0 !~= 2.5")]
fn f64_operands_panic() {
    assert_approx_eq!(2.0_f64, 2.5);
}

#[test]
fn equal_infinities_pass() {
    assert_approx_eq!(f32::INFINITY, f32::INFINITY);
    assert_approx_eq!(f32::NEG_INFINITY, f32::NEG_INFINITY);
    assert_approx_eq!(f64::INFINITY, f64::INFINITY, 0.5);
}

#[test]
#[should_panic(expected = "NaN !~= NaN")]
fn nan_operands_panic() {
    assert_approx_eq!(f32::NAN, f32::NAN);
}

#[test]
fn difference_equal_to_tolerance_passes() {
    assert_approx_eq!(1.0_f32, 1.5, 0.5);
    assert_approx_eq!(1.5_f32, 1.0, 0.5);
}

#[test]
fn both_forms_are_expressions() {
    let () = assert_approx_eq!(1.0_f32, 1.0);
    let () = assert_approx_eq!(1.0_f32, 1.0, 0.5);
}

#[test]
fn accepts_trailing_comma() {
    assert_approx_eq!(1.0_f32, 1.0,);
    assert_approx_eq!(1.0_f32, 1.0, 0.5,);
}
