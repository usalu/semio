//! 🧪️ Shared winding corpus reaches owned snapshots, scene projection and SVG.
use super::*;
use dsl::{FromValue,ToValue};
use store::ArtifactPack;
#[test]
fn authored_fill_rules_survive_pack_and_reach_paint_picking_and_export() {
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in rows.as_array().unwrap() {
        let rule=FillRule::parse(row["rule"].as_str().unwrap()).unwrap();
        assert_eq!(FillRule::from_value(rule.to_value()).unwrap(),rule);
        let segments=crate::standards::v1::subsets::any::io::import::deserializers::artifacts::svg::v1_1::any::path::parse_editable_svg_path(row["path"].as_str().unwrap()).unwrap();
        let mut layer=crate::schema::create_drawing_path_layer("Compound",segments.clone());
        crate::schema::layer_base_mut(&mut layer).attributes.fill_rule=rule;
        let doc=crate::DrawingSnapshot {id:"winding".into(),layers:vec![layer],..Default::default()};
        let restored=crate::DrawingSnapshot::decode_pack(&doc.encode_pack()).unwrap();
        assert_eq!(restored,doc);
        let nodes=crate::schema::flatten_drawing_document_to_scene_nodes(&doc);
        assert_eq!(nodes[0].fill_rule.as_deref(),Some(rule.as_str()));
        let mut cursor=crate::schema::geometry::picking::PathHitCursor::new([50.0,50.0],[1.0,0.0,0.0,1.0,0.0,0.0],0.0,0.001);
        while !cursor.step(&segments) {}
        assert_eq!(cursor.contains(true,false,rule==FillRule::Evenodd),row["hit"].as_bool().unwrap());
        let (svg,_,_)=crate::standards::v1::subsets::any::io::drawing_document_to_svg(&doc).unwrap();
        assert!(svg.contains(&format!("fill-rule=\"{}\"",rule.as_str())));
    }
}
