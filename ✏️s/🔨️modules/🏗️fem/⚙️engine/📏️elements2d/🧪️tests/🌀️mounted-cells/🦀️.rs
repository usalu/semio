//! 🌀️ Verify mounted planar cells against independent element matrices.
use semio_s_fem_algebra::VecD;
use semio_s_fem_elements2d::{Bar2, BeamEb2, PlaneKind, Tri3Cst};
use semio_s_fem_model::{Element, ElementContext};
fn numbers(field: &str) -> Vec<f64> {
    field.split(',').map(|entry| entry.parse().unwrap()).collect()
}

#[test]
fn mounted_mass_and_geometric_cells_match_reference_matrices() {
    let fixture = include_str!("../🧫️fixtures/🌀️mounted-cells/🔣️.tsv");
    let mut lines = fixture.lines();
    assert_eq!(lines.next().unwrap(), "id\tkind\te\tdensity\tarea\tnu\tthickness\tpositions\tdisplacement\tmass\tgeometric");
    for line in lines {
        let [id, kind, elastic, density, area, nu, thickness, positions, displacement, mass, geometric]: [&str; 11] =
            line.split('\t').collect::<Vec<_>>().try_into().expect("mounted cell fixture columns");
        let e = elastic.parse().unwrap();
        let density = density.parse().unwrap();
        let element: Box<dyn Element> = match kind {
            "bar" => Box::new(Bar2 { id: id.into(), start: "a".into(), end: "b".into(), e, area: area.parse().unwrap(), density }),
            "beam" => Box::new(BeamEb2 { id: id.into(), start: "a".into(), end: "b".into(), e, area: area.parse().unwrap(), iy: 1.0, density }),
            "stress" | "strain" => Box::new(Tri3Cst {
                id: id.into(),
                nodes: ["a".into(), "b".into(), "c".into()],
                e,
                nu: nu.parse().unwrap(),
                thickness: thickness.parse().unwrap(),
                kind: if kind == "stress" { PlaneKind::Stress } else { PlaneKind::Strain },
                density,
            }),
            _ => panic!("unsupported corpus element {kind}"),
        };
        let context = ElementContext { positions: numbers(positions).chunks(3).map(|point| point.try_into().unwrap()).collect() };
        let displacement = VecD::from_vec(numbers(displacement));
        let cold_mass = element.mass(&context).unwrap();
        let cold_geometric = element.geometric_stiffness(&context, &displacement).unwrap();
        let side = displacement.len();
        let expected_mass = numbers(mass);
        let expected_geometric = numbers(geometric);
        assert_eq!(expected_mass.len(), side * side);
        assert_eq!(expected_geometric.len(), side * side);
        for row in 0..side {
            for column in 0..side {
                let mass = element.mounted_mass_cell(&context.positions, row, column).expect("mounted mass cell");
                let geometric = element.mounted_geometric_stiffness_cell(&context, &displacement.0, row, column).expect("mounted geometric cell");
                assert_eq!(mass, element.mounted_mass_cell(&context.positions, column, row).unwrap());
                assert_eq!(geometric, element.mounted_geometric_stiffness_cell(&context, &displacement.0, column, row).unwrap());
                let index = row * side + column;
                for (key, expected, mounted, cold) in [
                    ("mass", expected_mass[index], mass, cold_mass.get(row, column)),
                    ("geometric", expected_geometric[index], geometric, cold_geometric.get(row, column)),
                ] {
                    let limit = 1e-12 * expected.abs().max(1.0);
                    assert!((mounted - expected).abs() <= limit, "{id} {key} ({row}, {column}) mounted {mounted} != {expected}");
                    assert!((cold - expected).abs() <= limit, "{id} {key} ({row}, {column}) batch {cold} != {expected}");
                }
            }
        }
        let degenerate = ElementContext { positions: vec![[0.0; 3]; context.positions.len()] };
        assert_eq!(element.mounted_mass_cell(&context.positions, side, 0), None);
        assert_eq!(element.mounted_mass_cell(&context.positions, 0, side), None);
        assert_eq!(element.mounted_mass_cell(&context.positions[1..], 0, 0), None);
        assert_eq!(element.mounted_mass_cell(&degenerate.positions, 0, 0), None);
        assert_eq!(element.mounted_geometric_stiffness_cell(&context, &displacement.0, side, 0), None);
        assert_eq!(element.mounted_geometric_stiffness_cell(&context, &displacement.0, 0, side), None);
        assert_eq!(element.mounted_geometric_stiffness_cell(&context, &displacement.0[1..], 0, 0), None);
        assert_eq!(element.mounted_geometric_stiffness_cell(&degenerate, &displacement.0, 0, 0), None);
        let mut nonfinite = displacement.0;
        nonfinite[0] = f64::NAN;
        assert_eq!(element.mounted_geometric_stiffness_cell(&context, &nonfinite, 0, 0), None);
    }
}
