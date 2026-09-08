mod runtime_cleanup_fault_vector_tests {
    use super::*;

    const FIXTURE: &str = include_str!("../../🩺️runtime-fault-vectors.json");

    #[test]
    fn every_declared_variant_decodes_to_its_language_neutral_wire_vector() {
        let fixture: Value = serde_json::from_str(FIXTURE).expect("language-neutral cleanup fault vectors");
        assert_eq!(fixture["ceilingUs"].as_u64(), Some(semio_framework_trace::INTERACTIVE_STEP_CEILING_US));
        let vectors = fixture["vectors"].as_array().expect("fixture vectors");
        assert_eq!(vectors.len(), RUNTIME_CLEANUP_FAULTS.len());
        for (declared, expected) in RUNTIME_CLEANUP_FAULTS.iter().zip(vectors) {
            let vector = declared.vector();
            assert_eq!(vector.slug, expected["variant"].as_str().expect("variant"));
            assert_eq!(vector.code, expected["code"].as_str().expect("code"), "{}", vector.slug);
            assert_eq!(vector.detail, expected["detail"].as_str().expect("detail"), "{}", vector.slug);
            assert_eq!(RuntimeMaintenanceStatus::from_repr(RuntimeMaintenanceStatus::Fault(*declared).repr()).cause(), Some(*declared), "{}", vector.slug);
            assert_eq!(RuntimeCloseStatus::from_repr(RuntimeCloseStatus::Fault(*declared).repr()).cause(), Some(*declared), "{}", vector.slug);
            for scope in ["live", "close"] {
                let fault = runtime_cleanup_fault(scope, *declared, 7, 12_345);
                assert_eq!(fault.origin, FaultOrigin::Plugin);
                assert_eq!(format!("{:?}", fault.severity), "Error");
                assert_eq!(fault.code.0, vector.code);
                assert_eq!(fault.message, format!("runtime {scope} cleanup faulted for instance 7: {} [{}] (elapsed 12345us, ceiling {}us)", vector.detail, vector.slug, semio_framework_trace::INTERACTIVE_STEP_CEILING_US));
                assert!(runtime_cleanup_fault(scope, *declared, 7, RUNTIME_CLEANUP_UNMEASURED_US).message.ends_with(&format!("(elapsed unmeasured, ceiling {}us)", semio_framework_trace::INTERACTIVE_STEP_CEILING_US)));
            }
        }
    }

    #[test]
    fn non_fault_status_reprs_never_collide_with_a_fault_variant() {
        for status in [RuntimeMaintenanceStatus::Ready, RuntimeMaintenanceStatus::Queued, RuntimeMaintenanceStatus::Running] {
            assert_eq!(RuntimeMaintenanceStatus::from_repr(status.repr()), status);
            assert_eq!(status.cause(), None);
        }
        for status in [RuntimeCloseStatus::Ready, RuntimeCloseStatus::Queued, RuntimeCloseStatus::Running, RuntimeCloseStatus::Complete, RuntimeCloseStatus::ExternalWait, RuntimeCloseStatus::DeadlineYield] {
            assert_eq!(RuntimeCloseStatus::from_repr(status.repr()), status);
            assert_eq!(status.cause(), None);
        }
        let codes: std::collections::BTreeSet<&str> = RUNTIME_CLEANUP_FAULTS.iter().map(|fault| fault.vector().code).collect();
        assert_eq!(codes.len(), RUNTIME_CLEANUP_FAULTS.len());
    }
}
