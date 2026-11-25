//! 🧪️ Flat original span arrays retain their wire order across packed page boundaries.
use super::*;
use semio_plugin::artifact_app_laws::OriginalWorkFixture;
use semio_framework_value::{ToValue,retained_clone::OriginalEffectReply};
use semio_framework_value::persistent::PersistentList;

fn loan()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:1<<17,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}}
fn caller()->OriginalWorkFixture{OriginalWorkFixture::with_source(semio_framework_job::OperationId(991),semio_framework_job::Generation(17),OriginalWorkFixture::stock(),||Some(1),u64::MAX,1<<24)}

#[test]
fn original_flat_coverage_receives_packed_pages(){
    for count in [0,1,255,256,257,4097]{
        let rows=(0..count).map(|index|RasterSelectionSpan{start:index as u32,length:1,coverage:128}).collect::<Vec<_>>();
        let mut caller=caller();
        let grant=loan();
        let packed=caller.receive_native::<_,RasterSelectionSpans>(rows.to_value(),grant);
        assert_eq!(packed.len(),count);
        assert_eq!(packed.maximum_end(),count as u64);
        assert_eq!(packed.pages().len(),RasterSelectionSpans::page_count(count));
        assert!(packed.iter().enumerate().all(|(index,row)|row.start==index as u32&&row.length==1&&row.coverage==128));
        caller.retire_owned(packed,grant);
        caller.finish();
    }
}

#[test]
fn original_coverage_construction_proves_span_order_length_and_coverage(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../fixture.json")).unwrap();
    let grant=loan();
    for rows in fixture["invalidRows"].as_array().unwrap(){let rows=rows.as_array().unwrap().iter().map(|row|RasterSelectionSpan{start:row["start"].as_u64().unwrap()as u32,length:row["length"].as_u64().unwrap()as u32,coverage:row["coverage"].as_u64().unwrap()as u32}).collect::<Vec<_>>();let mut caller=caller();let error=caller.try_receive_native::<_,RasterSelectionSpans>(rows.to_value(),grant).err().expect("invalid coverage must refuse before publication");caller.retire_owned(error,grant);caller.finish();}
    let mut caller=caller();let root=caller.receive_native::<_,RasterSelectionSpans>(vec![RasterSelectionSpan{start:0,length:1,coverage:0}].to_value(),grant);assert_eq!(root.maximum_end(),1);assert_eq!(root[0].coverage,0);caller.retire_owned(root,grant);caller.finish();
}

fn project_pages<'a>(pages: RetainedCloneRef<'a, PersistentList<RasterSpanPage>>, effect: &mut RetainedAllocationEffect<'_>) -> (Option<PersistentList<RasterSpanPage>>, RetainedCloneStep) {
    let mut output = None;
    let mut slot = None;
    PersistentList::project_into(pages, &mut output, effect, OriginalEffectReply::borrow(&mut slot).unwrap()).unwrap();
    let step = slot.take().expect("persistent projection reply").ok().expect("persistent projection is funded or denied");
    (output, step)
}

#[test]
fn packed_coverage_projection_shares_pages_for_exact_work() {
    use semio_framework_pixels::{BlockingTurns, EffectTurns};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../fixture.json")).unwrap();
    let mut caller = caller();
    let stock = OriginalWorkFixture::stock();
    let demand = PersistentList::<super::super::RasterSpanPage>::projection_demand();
    let grant = RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: demand.copy_bytes,
        maximum_depth: demand.depth,
        ..Default::default()
    };
    for scenario in fixture["alias"].as_array().unwrap() {
        let count = scenario["count"].as_u64().unwrap() as u32;
        let work = scenario["work"].as_u64().unwrap();
        let rows = (0..count).map(|start| RasterSelectionSpan { start, length: 1, coverage: 128 }).collect::<Vec<_>>();
        let packed = caller.receive_native::<_, RasterSelectionSpans>(rows.to_value(), loan());
        let source = caller.admit_source(packed, stock);
        let original = source.borrow();
        let pages = original.project(0, |spans| spans.pages());
        assert_eq!(PersistentList::projection_work_demand(pages).unwrap(), work);
        for (offered, work) in [
            (RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - 1, ..grant }, work),
            (grant, work - 1),
        ] {
            BlockingTurns::new(work).turn(offered, |effect| {
                let before = effect.performed_boundary();
                let (alias, step) = project_pages(pages, effect);
                assert!(alias.is_none());
                assert_eq!(step.progress(), Default::default());
                assert_eq!(effect.performed_since(before).unwrap(), 0);
                assert!(effect.actual().is_none());
            }).unwrap();
        }
        let alias = BlockingTurns::new(work).turn(grant, |effect| {
            let before = effect.performed_boundary();
            let (alias, step) = project_pages(pages, effect);
            assert!(matches!(step, RetainedCloneStep::Complete(_)));
            let alias = alias.unwrap();
            assert_eq!(effect.performed_since(before).unwrap(), u128::from(work));
            assert!(effect.actual().is_none());
            alias
        }).unwrap();
        assert!(pages.get().same_root(&alias));
        assert_eq!(alias.len(), scenario["pages"].as_u64().unwrap() as usize);
        assert!(original.get().iter().map(|row| row.start).eq(0..count));
        caller.retire_owned(alias, stock);
        caller.retire_owned(source, stock);
    }
    caller.finish();
}
