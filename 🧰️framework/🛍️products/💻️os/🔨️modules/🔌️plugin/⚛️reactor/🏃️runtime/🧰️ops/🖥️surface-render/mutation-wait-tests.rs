//! 🧪️ A surface mutation waits for its own instance's open capture and is never mistaken for another event.
/// 🪟️ A mount or hide addressed at an instance whose registry is pinned by an open original view capture waits in its slot instead of aborting the guest; every other event is served.
#[test]
fn surface_mutation_events_wait_only_while_their_own_instance_holds_a_capture(){
 use semio_framework::kernel::Event;
 use super::surface_mutation_waits;
 let visible=|surface:&str|Event::SurfaceVisible{surface:surface.into(),body_key:"graph".into(),view_state:Vec::new()};
 let hidden=|surface:&str|Event::SurfaceHidden{surface:surface.into()};
 for event in [visible("1:left"),hidden("1:left")]{
  assert!(surface_mutation_waits(&event,|instance|instance==1),"held capture defers the mutation");
  assert!(!surface_mutation_waits(&event,|_|false),"a released capture replays it");
 }
 for event in [visible("2:left"),hidden("2:left")]{assert!(!surface_mutation_waits(&event,|instance|instance==1),"another instance is not pinned");}
 assert!(!surface_mutation_waits(&visible("left"),|_|true),"a surface without an instance prefix names no registry");
 for event in [Event::Wake,Event::SurfaceResized{surface:"1:left".into(),width:1,height:1}]{assert!(!surface_mutation_waits(&event,|_|true),"only registry mutations wait");}
}
