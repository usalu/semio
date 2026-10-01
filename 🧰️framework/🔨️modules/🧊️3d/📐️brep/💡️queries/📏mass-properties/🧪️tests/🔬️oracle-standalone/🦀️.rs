#[cfg(test)]
    #[test]
    fn watertightness_of_box_is_watertight() {
        use crate::brep::operations::primitives::make_box;
        use crate::brep::representation::topology::history::OpRecorder;
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let _ = solid;
        let report = watertightness_of_body(&body);
        assert_eq!(report.verdict, WatertightnessVerdict::Watertight);
    }
