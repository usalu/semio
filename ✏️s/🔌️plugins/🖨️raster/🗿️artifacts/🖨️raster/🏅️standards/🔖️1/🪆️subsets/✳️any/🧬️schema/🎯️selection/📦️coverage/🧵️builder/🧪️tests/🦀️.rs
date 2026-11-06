use super::*;
use semio_plugin::artifact_app_laws::OriginalWorkFixture;
use semio_framework_job::{admit_original_construction,OriginalConstructionDemand,receive_original_retained_work};

fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:128}}
fn advance(caller:&mut OriginalWorkFixture,builder:&mut RasterSelectionSpansBuilder){caller.turn(grant(),|cx|{let turn=builder.advance(cx.retained_grant());receive_original_retained_work(cx,turn)?;Ok(())}).unwrap();}
fn close(caller:&mut OriginalWorkFixture,builder:&mut RasterSelectionSpansBuilder){builder.begin_close();for _ in 0..1_048_576{if builder.terminal_is_empty(){return;}caller.turn(grant(),|cx|{let d=builder.close_demands(cx.retained_grant().maximum_copy_bytes)?;assert!(d.release_bytes<=4096);semio_framework_job::close_original_retained_step(cx,d,|g|builder.close_step(g))?;Ok(())}).unwrap();}panic!("packed selection builder failed its original close");}

#[test]
fn packed_coverage_pages_preserve_flat_rows_and_close_with_four_kib_releases(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../fixture.json")).unwrap();assert_eq!(RASTER_SELECTION_PAGE_ROWS,fixture["pageRows"].as_u64().unwrap()as usize);assert_eq!(size_of::<RasterSelectionSpan>(),fixture["currentSpanBytes"].as_u64().unwrap()as usize);assert!(SealedShared::<RasterSpanPage>::birth_bytes()<=4096);
    for count in fixture["counts"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){
        let mut caller=OriginalWorkFixture::new();let mut builder=None;caller.turn(grant(),|cx|{admit_original_construction(cx,&mut builder,OriginalConstructionDemand{items:1,copy_bytes:size_of::<RasterSelectionSpansBuilder>(),capacity_bytes:0,release_bytes:0,depth:1},|_|Ok((RasterSelectionSpansBuilder::new(count)?,posted(size_of::<RasterSelectionSpansBuilder>()))))?;Ok(())}).unwrap();let mut builder=builder.unwrap();let mut accepted=0;let mut row=None;let mut finishing=false;
        for _ in 0..count*8+1024{if builder.output_ready(){break;}if builder.accepts_row(){if row.is_none(){row=Some(RasterSelectionSpan{start:accepted as u32,length:1,coverage:if accepted%2==0{128}else{255}});}caller.turn(grant(),|cx|{let turn=builder.push(&mut row,cx.retained_grant());receive_original_retained_work(cx,RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work})?;Ok(())}).unwrap();if row.is_none(){accepted+=1;}}
            else if accepted==count&&!finishing{caller.turn(grant(),|cx|{let turn=builder.finish(cx.retained_grant());receive_original_retained_work(cx,RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work})?;Ok(())}).unwrap();finishing=true;}else{advance(&mut caller,&mut builder);}}
        assert!(builder.output_ready());let mut output=None;caller.turn(grant(),|cx|{let turn=builder.take(cx.retained_grant());let result=turn.result.map(|taken|{let(value,p)=taken.unwrap();output=Some(value);RetainedCloneStep::Progress(p)});receive_original_retained_work(cx,RetainedWorkOutcome{result,work:turn.work})?;Ok(())}).unwrap();let output=output.unwrap();assert_eq!(output.len(),count);assert_eq!(output.pages().len(),RasterSelectionSpans::page_count(count));for(index,row)in output.iter().enumerate(){assert_eq!((row.start,row.length,row.coverage),(index as u32,1,if index%2==0{128}else{255}));}close(&mut caller,&mut builder);caller.retire_owned(output,grant());caller.finish();
    }
}

#[test]
fn denied_and_cancelled_packed_frontiers_preserve_the_input_row(){
    for cut in [0,1,255,256,257]{let mut caller=OriginalWorkFixture::new();let mut builder=RasterSelectionSpansBuilder::new(512).unwrap();let mut accepted=0;let mut row=None;for _ in 0..10000{if accepted==cut{break;}if builder.accepts_row(){if row.is_none(){row=Some(RasterSelectionSpan{start:accepted,length:1,coverage:255});}let before=row;let denied=builder.push(&mut row,RetainedCloneGrant::default());assert_eq!(denied.work,0);assert_eq!(denied.result.unwrap(),Default::default());assert_eq!(row,before);caller.turn(grant(),|cx|{let turn=builder.push(&mut row,cx.retained_grant());receive_original_retained_work(cx,RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work})?;Ok(())}).unwrap();if row.is_none(){accepted+=1;}}else{advance(&mut caller,&mut builder);}}assert_eq!(accepted,cut);assert!(!builder.output_ready());close(&mut caller,&mut builder);caller.finish();}
}
