use super::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::cad::schema::snapshot::CadEntityRecord;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_cad() -> SemioCadSnapshot {
    SemioCadSnapshot {
        entities: vec![
            CadEntityRecord { handle: "h1".into(), layer: "0".into(), entity: CadEntity::Line { a: SemioPoint2 { x: 0.0, y: 0.0 }, b: SemioPoint2 { x: 5.0, y: 0.0 } } },
            CadEntityRecord { handle: "h2".into(), layer: "0".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 2.0, y: 2.0 }, radius: 1.5 } },
            CadEntityRecord { handle: "h3".into(), layer: "0".into(), entity: CadEntity::Text { position: SemioPoint2::default(), height: 1.0, rotation: 0.0, content: "dropped".into() } },
        ],
        ..SemioCadSnapshot::default()
    }
}

/// 🧪️ Real round trip through step's own real Part-21 text codec.
#[semio_framework_async_macros::async_test]
async fn real_text_round_trip_through_step_codec() {
    let cad = sample_cad();
    let step = semio_framework_plugin::resolve_ready(SemioCadToStep::serialize(&cad)).expect("serialize");
    assert_eq!(step.header.file_schema.schemas, vec!["AUTOMOTIVE_DESIGN".to_string()]);
    assert_eq!(step.entities.len(), 7, "4 for LINE + 3 for CIRCLE; Text is dropped");

    let text = store::ArtifactDsl::print_dsl(&step);
    let reparsed = <StepSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("reparse real step text");
    assert_eq!(reparsed, step, "step's own codec_retention_law must hold on our emitted graph");

    let line_count = reparsed.entities.iter().filter(|e| e.name == "LINE").count();
    let circle_count = reparsed.entities.iter().filter(|e| e.name == "CIRCLE").count();
    assert_eq!(line_count, 1);
    assert_eq!(circle_count, 1);
}
