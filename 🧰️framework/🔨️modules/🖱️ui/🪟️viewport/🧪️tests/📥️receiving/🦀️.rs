//! 📥️ Viewport poses round-trip through their DslField encoding via original receiving and refuse foreign records.
use semio_framework_dsl_record::receiving::owned::OwnedScalar;
use semio_framework_dsl_record::{DslField,FieldValue,RecordValue};
use semio_ui_viewport::{Viewport2d,Viewport3dOrbit};

#[test]
fn planar_pose_round_trips_through_receiving(){
    for pose in [Viewport2d::default(),Viewport2d{x:-3.5,y:12.0,zoom:0.25}]{
        let mut source=Some(pose.to_value());
        assert_eq!(<Viewport2d as OwnedScalar>::take_source(&mut source).unwrap(),pose);
        assert_eq!(<Viewport2d as OwnedScalar>::take_source(&mut source).unwrap(),Viewport2d::from_value(&pose.to_value()).unwrap());
    }
}
#[test]
fn orbit_pose_round_trips_with_and_without_up(){
    for up in [None,Some([0.0,1.0,0.0])]{
        let pose=Viewport3dOrbit{position:[1.0,2.0,3.0],target:[0.0,0.0,0.0],zoom:2.0,up};
        let mut source=Some(pose.to_value());
        assert_eq!(<Viewport3dOrbit as OwnedScalar>::take_source(&mut source).unwrap(),pose);
    }
}
#[test]
fn invalid_poses_are_refused_and_keep_their_source(){
    let zoom_zero=FieldValue::Record(RecordValue{fields:[(1,FieldValue::Float(0.0)),(2,FieldValue::Float(0.0)),(3,FieldValue::Float(0.0))].into_iter().collect()});
    let oversized=FieldValue::Record(RecordValue{fields:(0..64u16).map(|id|(id,FieldValue::Float(1.0))).collect()});
    for foreign in [FieldValue::Bool(true),zoom_zero,oversized]{
        let mut source=Some(foreign.clone());
        assert!(<Viewport2d as OwnedScalar>::take_source(&mut source).is_err(),"refused: {foreign:?}");
        assert!(source.is_some());
    }
}
