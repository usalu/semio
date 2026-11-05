//! 🎟️ Bake fixtures retain one genuine source across private work, handoff and retirement.
use semio_framework_plugin::{Emit,artifact_app_laws::OriginalWorkFixture};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneStep,RetainedWorkOutcome};
use semio_s_artifact_raster_raster_editor::{RasterCommand,RasterConfigMutation,commands::flatten_layers::LayerBakeOwner};
use semio_s_artifact_raster_raster_model::{RasterLayerNode,SemioImageSnapshot,snapshot::RasterSnapshot};
use semio_s_artifact_raster_raster_mutations::RasterMutation;
pub fn loan()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}}
pub fn bake<const MERGE:bool>(command:&RasterCommand,document:&RasterSnapshot,cut:Option<usize>)->Result<Option<Emit<RasterMutation,RasterConfigMutation>>,String>{
    let mut caller=OriginalWorkFixture::new();let mut owner=match LayerBakeOwner::<MERGE>::new(command,document){Ok(owner)=>owner,Err(error)=>{let message=error.message.to_string();caller.retire_owned(error,loan());caller.finish();return Err(message)}};let mut refused=None;
    for turn in 0..100_000{if owner.output_ready()||cut.is_some_and(|cut|turn>=cut){break}let result=caller.turn(loan(),|cx|{let turn=owner.advance(command,document,32768,cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,turn)?;Ok(())});if let Err(error)=result{refused=Some(error);break}assert!(turn<99_999);}
    let mut output=None;if cut.is_none()&&refused.is_none(){assert!(owner.output_ready());let result=caller.turn(loan(),|cx|{let turn=owner.take_emit(cx.retained_grant());let work=turn.work;let result=turn.result.map(|value|if let Some((emit,progress))=value{output=Some(emit);RetainedCloneStep::Progress(progress)}else{RetainedCloneStep::Progress(Default::default())});semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result,work})?;Ok(())});if let Err(error)=result{refused=Some(error);}}
    owner.cancel();for turn in 0..100_000{if owner.terminal_is_empty(){break}caller.turn(loan(),|cx|{let demand=owner.close_demands(32768)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.close_step(grant))?;Ok(())}).unwrap();assert!(turn<99_999);}assert!(owner.terminal_is_empty());
    if let Some(error)=refused{if let Some(emit)=output{caller.retire_owned(emit,loan());}let message=error.primary.message().to_owned();caller.retire_owned(error,loan());caller.finish();Err(message)}else{caller.finish();Ok(output)}
}
pub fn composite(document:&RasterSnapshot)->semio_framework_pixels::compositing::layers::RasterStackResult{
    let command=RasterCommand::FlattenLayers(semio_s_artifact_raster_raster_editor::commands::flatten_layers::FlattenLayers{name:"Fixture".into()});let emit=bake::<false>(&command,document,None).unwrap().unwrap();
    let layer=emit.artifact_mutations.iter().find_map(|mutation|if let RasterMutation::CreateLayer(create)=mutation{Some(create.layer.as_ref())}else{None}).unwrap();let RasterLayerNode::Pixel{image_key:Some(key),transform,..}=layer else{panic!("bake fixture output layer")};
    let image=emit.artifact_mutations.iter().find_map(|mutation|if let RasterMutation::AddLayerAsset(add)=mutation{Some(add.asset.image())}else{None});let original=if image.is_none(){Some(document.assets.get(key).unwrap().local_owner::<SemioImageSnapshot>().unwrap())}else{None};let image=image.or_else(||original.as_deref()).unwrap();
    let result=semio_framework_pixels::compositing::layers::RasterStackResult{image:semio_framework_pixels::RasterImage{width:image.width,height:image.height,pixels:image.frames[0].rgba8.clone()},origin:[transform.x-f64::from(image.width)/2.0,transform.y-f64::from(image.height)/2.0],empty:false};
    let mut caller=OriginalWorkFixture::new();caller.retire_owned(emit,loan());caller.finish();result
}
