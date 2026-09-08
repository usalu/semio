mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn box_sdf_is_negative_inside_and_positive_outside() {
        let b = Sdf::Box { half_extents: Pnt3::new(1.0, 1.0, 1.0), placement: Trsf::IDENTITY };
        assert!(b.eval(Pnt3::new(0.0, 0.0, 0.0)) < 0.0);
        assert!(b.eval(Pnt3::new(5.0, 0.0, 0.0)) > 0.0);
        assert!((b.eval(Pnt3::new(1.0, 0.0, 0.0))).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn sphere_sdf_matches_analytic_distance() {
        let s = Sdf::Sphere { radius: 2.0, placement: Trsf::IDENTITY };
        assert!((s.eval(Pnt3::new(5.0, 0.0, 0.0)) - 3.0).abs() < 1e-9);
        assert!((s.eval(Pnt3::new(0.0, 0.0, 0.0)) - (-2.0)).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn cylinder_sdf_is_correct_on_axis_and_cap() {
        let c = Sdf::Cylinder { radius: 1.0, half_height: 2.0, placement: Trsf::IDENTITY };
        assert!((c.eval(Pnt3::new(0.0, 0.0, 0.0)) - (-1.0)).abs() < 1e-9);
        assert!((c.eval(Pnt3::new(0.0, 0.0, 5.0)) - 3.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn torus_sdf_is_negative_on_major_circle_and_positive_outside_tube() {
        let t = Sdf::Torus { major_radius: 2.0, minor_radius: 0.5, placement: Trsf::IDENTITY };
        assert!(t.eval(Pnt3::new(2.0, 0.0, 0.0)) < 0.0);
        assert!((t.eval(Pnt3::new(2.5, 0.0, 0.0))).abs() < 1e-8);
        assert!(t.eval(Pnt3::new(0.0, 0.0, 0.0)) > 0.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn cone_sdf_is_negative_inside_taper_and_positive_outside() {
        let c = Sdf::Cone { radius: 1.0, half_height: 1.0, placement: Trsf::IDENTITY };
        assert!(c.eval(Pnt3::new(0.0, 0.0, -0.5)) < 0.0);
        assert!((c.eval(Pnt3::new(1.0, 0.0, -1.0))).abs() < 1e-8);
        assert!(c.eval(Pnt3::new(2.0, 0.0, 0.0)) > 0.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn union_is_the_min_and_matches_containment_of_either_operand() {
        let a = Sdf::Sphere { radius: 1.0, placement: Trsf::translation(crate::standards::v1::subsets::brep::schema::snapshot::vector::Vec3::new(-1.0, 0.0, 0.0)) };
        let b = Sdf::Sphere { radius: 1.0, placement: Trsf::translation(crate::standards::v1::subsets::brep::schema::snapshot::vector::Vec3::new(1.0, 0.0, 0.0)) };
        let u = a.union(b);
        assert!(u.contains(Pnt3::new(-1.0, 0.0, 0.0), 1e-9));
        assert!(u.contains(Pnt3::new(1.0, 0.0, 0.0), 1e-9));
        assert!(!u.contains(Pnt3::new(5.0, 0.0, 0.0), 1e-9));
    }

    #[semio_framework_async_macros::async_test]
    async fn difference_removes_the_second_operand() {
        let big = Sdf::Sphere { radius: 2.0, placement: Trsf::IDENTITY };
        let small = Sdf::Sphere { radius: 1.0, placement: Trsf::IDENTITY };
        let d = big.difference(small);
        assert!(!d.contains(Pnt3::new(0.0, 0.0, 0.0), 1e-9));
        assert!(d.contains(Pnt3::new(1.5, 0.0, 0.0), 1e-9));
    }

    #[semio_framework_async_macros::async_test]
    async fn placed_box_sdf_respects_transform() {
        let placement = Trsf::translation(crate::standards::v1::subsets::brep::schema::snapshot::vector::Vec3::new(10.0, 0.0, 0.0));
        let b = Sdf::Box { half_extents: Pnt3::new(1.0, 1.0, 1.0), placement };
        assert!(b.eval(Pnt3::new(10.0, 0.0, 0.0)) < 0.0);
        assert!(b.eval(Pnt3::new(0.0, 0.0, 0.0)) > 0.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn closed_form_mass_matches_textbook_box_sphere_cylinder() {
        let half = Pnt3::new(1.0, 2.0, 3.0);
        assert!((ClosedFormMass::box_volume(half) - 48.0).abs() < 1e-12);
        assert!((ClosedFormMass::box_surface_area(half) - 88.0).abs() < 1e-12);
        assert!((ClosedFormMass::sphere_volume(3.0) - 36.0 * std::f64::consts::PI).abs() < 1e-9);
        assert!((ClosedFormMass::sphere_surface_area(3.0) - 36.0 * std::f64::consts::PI).abs() < 1e-9);
        assert!((ClosedFormMass::cylinder_volume(2.0, 3.0) - 24.0 * std::f64::consts::PI).abs() < 1e-9);
        assert!((ClosedFormMass::cylinder_surface_area(2.0, 3.0) - 32.0 * std::f64::consts::PI).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn watertightness_stub_classifies_boundary_edge_count() {
        let tight = watertightness_from_boundary_edge_count(0);
        assert_eq!(tight.verdict, WatertightnessVerdict::Watertight);
        let open = watertightness_from_boundary_edge_count(3);
        assert_eq!(open.verdict, WatertightnessVerdict::HasBoundaryEdges { count: 3 });
        assert_eq!(watertightness_stub_unchecked().verdict, WatertightnessVerdict::NotChecked);
    }
}
