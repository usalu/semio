use super::*;

#[test]
fn length_units_convert_to_metres_exactly() {
    assert_eq!(LengthUnit::Millimetre.to_si(90.0), 0.09);
    assert_eq!(LengthUnit::Centimetre.to_si(250.0), 2.5);
    assert_eq!(LengthUnit::Metre.to_si(2.4), 2.4);
    assert_eq!(LengthUnit::Kilometre.to_si(1.5), 1500.0);
    assert!((LengthUnit::Inch.to_si(10.0) - 0.254).abs() < 1e-15);
    assert!((LengthUnit::Foot.to_si(1.0) - 0.3048).abs() < 1e-15);
}

#[test]
fn angle_area_and_volume_units_convert_to_si() {
    assert_eq!(AngleUnit::Degree.to_si(180.0), std::f64::consts::PI);
    assert_eq!(AngleUnit::Radian.to_si(1.25), 1.25);
    assert_eq!(AreaUnit::SquareCentimetre.to_si(10_000.0), 1.0);
    assert_eq!(AreaUnit::SquareMillimetre.to_si(1_000_000.0), 1.0);
    assert_eq!(VolumeUnit::Litre.to_si(1000.0), 1.0);
    assert_eq!(VolumeUnit::CubicCentimetre.to_si(1_000_000.0), 1.0);
}

#[test]
fn every_unit_symbol_round_trips_and_aliases_resolve() {
    for u in LengthUnit::ALL {
        assert_eq!(LengthUnit::from_symbol(u.symbol()), Some(*u));
    }
    for u in AngleUnit::ALL {
        assert_eq!(AngleUnit::from_symbol(u.symbol()), Some(*u));
    }
    for u in AreaUnit::ALL {
        assert_eq!(AreaUnit::from_symbol(u.symbol()), Some(*u));
    }
    for u in VolumeUnit::ALL {
        assert_eq!(VolumeUnit::from_symbol(u.symbol()), Some(*u));
    }
    assert_eq!(AngleUnit::from_symbol("°"), Some(AngleUnit::Degree));
    assert_eq!(AreaUnit::from_symbol("m²"), Some(AreaUnit::SquareMetre));
    assert_eq!(VolumeUnit::from_symbol("m³"), Some(VolumeUnit::CubicMetre));
    assert_eq!(LengthUnit::from_symbol("yd"), None);
}

#[test]
fn unit_symbols_are_unique_across_kinds() {
    let mut seen = std::collections::BTreeSet::new();
    let all = LengthUnit::ALL.iter().map(|u| u.symbol()).chain(AngleUnit::ALL.iter().map(|u| u.symbol())).chain(AreaUnit::ALL.iter().map(|u| u.symbol())).chain(VolumeUnit::ALL.iter().map(|u| u.symbol()));
    for symbol in all {
        assert!(seen.insert(symbol), "duplicate unit symbol {symbol}");
    }
}

#[test]
fn functions_resolve_by_name_with_their_arity() {
    for f in Function::ALL {
        assert_eq!(Function::from_name(f.name()), Some(*f));
    }
    assert_eq!(Function::Atan2.arity(), 2);
    assert_eq!(Function::Sqrt.arity(), 1);
    assert_eq!(Function::from_name("min"), None);
}

#[test]
fn children_follow_the_documented_path_order() {
    let e = Expr::conditional(Expr::Bool(true), Expr::Number(1.0), Expr::binary(BinaryOp::Add, Expr::Number(2.0), Expr::param("x")));
    assert_eq!(e.children().len(), 3);
    assert_eq!(e.at(&[2, 1]), Some(&Expr::param("x")));
    assert_eq!(e.at(&[3]), None);
    assert_eq!(e.at(&[]), Some(&e));
    assert_eq!(e.size(), 6);
    let call = Expr::Call(Function::Atan2, vec![Expr::Number(1.0), Expr::Number(2.0)]);
    assert_eq!(call.at(&[1]), Some(&Expr::Number(2.0)));
}

#[test]
fn canonical_trees_have_finite_non_negative_literals_and_exact_arity() {
    assert!(Expr::Number(0.0).is_canonical());
    assert!(!Expr::Number(-1.0).is_canonical());
    assert!(!Expr::Number(-0.0).is_canonical());
    assert!(!Expr::Length(f64::NAN, LengthUnit::Metre).is_canonical());
    assert!(!Expr::Call(Function::Sqrt, vec![]).is_canonical());
    assert!(!Expr::param("").is_canonical());
    assert!(!Expr::unary(UnaryOp::Negate, Expr::Number(f64::INFINITY)).is_canonical());
}

#[test]
fn values_report_kind_and_magnitude() {
    assert_eq!(Value::Length(2.0).kind(), Kind::Length);
    assert_eq!(Value::Bool(true).magnitude(), None);
    assert_eq!(Value::Text("a".into()).magnitude(), None);
    assert_eq!(Value::Area(3.0).magnitude(), Some(3.0));
    assert_eq!(Value::of_kind(Kind::Volume, 1.0), Some(Value::Volume(1.0)));
    assert_eq!(Value::of_kind(Kind::Bool, 1.0), None);
}
