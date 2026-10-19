use super::*;

#[test]
fn mounted_geometric_cells_match_independent_quadrature_and_tensor_contraction() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🌀️geometric-cells/🔣️.json")).expect("geometric corpus");
    let tolerance = fixture["tolerance"].as_f64().unwrap();
    let mut cells = 0;
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let properties = &case["properties"];
        let element: Elements = match case["kind"].as_str().unwrap() {
            "bar3" => Bar3 { id: id.into(), node_a: "a".into(), node_b: "b".into(), e: fixture_number(properties, "e"), a: fixture_number(properties, "a"), density: 0.0 }.into(),
            "frame3" => Frame3 { id: id.into(), node_a: "a".into(), node_b: "b".into(), e: fixture_number(properties, "e"), g: 1.0, a: fixture_number(properties, "a"), iy: 1.0, iz: 1.0, j: 1.0, roll: fixture_number(properties, "roll"), density: 0.0 }.into(),
            "tet4" => Tet4 { id: id.into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into()], e: fixture_number(properties, "e"), nu: fixture_number(properties, "nu"), density: 0.0 }.into(),
            _ => panic!("unsupported geometric corpus"),
        };
        let context = ElementContext { positions: case["positions"].as_array().unwrap().iter().map(|row| std::array::from_fn(|axis| row[axis].as_f64().unwrap())).collect() };
        let displacement: Vec<f64> = case["displacement"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect();
        let side = displacement.len();
        let cold = element.geometric_stiffness(&context, &VecD::from_vec(displacement.clone())).expect("batch geometric matrix");
        for row in 0..side {
            for column in 0..side {
                let actual = element.mounted_geometric_stiffness_cell(&context, &displacement, row, column).expect("mounted geometric cell");
                let expected = case["matrix"][row * side + column].as_f64().unwrap();
                assert!((actual - expected).abs() <= tolerance * expected.abs().max(1.0), "{id} ({row},{column}) {actual} != {expected}");
                assert!((actual - cold.get(row, column)).abs() <= tolerance * expected.abs().max(1.0), "{id} common batch formula");
                let transposed = element.mounted_geometric_stiffness_cell(&context, &displacement, column, row).unwrap();
                assert!((actual - transposed).abs() <= tolerance * expected.abs().max(1.0), "{id} symmetry");
                cells += 1;
            }
        }
        assert_eq!(element.mounted_geometric_stiffness_cell(&context, &displacement, side, 0), None);
        assert_eq!(element.mounted_geometric_stiffness_cell(&context, &displacement, 0, side), None);
        assert_eq!(element.mounted_geometric_stiffness_cell(&context, &displacement[..side - 1], 0, 0), None);
        assert_eq!(element.mounted_geometric_stiffness_cell(&ElementContext { positions: Vec::new() }, &displacement, 0, 0), None);
        let mut invalid = displacement;
        invalid[0] = f64::NAN;
        assert_eq!(element.mounted_geometric_stiffness_cell(&context, &invalid, 0, 0), None);
    }
    println!("[DEBUG] mounted geometric stiffness {cells} cells match independent quadrature and tensor contraction without scratch owners");
}
