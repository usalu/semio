//! 🧪️ Retained context discovery keeps every prefix source-bound and allocation-free.
use semio_framework::ViewModel;
use semio_framework_plugin_plugin_program::surface_context::{SurfaceContexts,SurfaceWindowLeaseCursor};
#[test]
fn auxiliary_context_capture_is_incremental_and_refuses_substituted_sources_at_each_prefix(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
    let policy:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/context-cursor.json")).unwrap();
    let view:ViewModel=serde_json::from_value(fixture["view"].clone()).unwrap();let window="left";let locale=view.locale;
    for row in policy["invalidations"].as_array().unwrap(){for prefix in policy["mutationPrefixes"].as_array().unwrap(){
        let mut contexts=SurfaceContexts::default();contexts.insert("7:left".into(),"graph".into(),view.for_window_instance(window).unwrap()).unwrap();
        let mut substitute=SurfaceContexts::default();substitute.insert("7:left".into(),"graph".into(),view.for_window_instance(window).unwrap()).unwrap();
        let mut cursor=SurfaceWindowLeaseCursor::default();
        for _ in 0..prefix.as_u64().unwrap(){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(&contexts,window,locale));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if result.unwrap().is_some(){break}}
        let result=match row.as_str().unwrap(){
            "source-update"=>{contexts.update_view(&view);cursor.step(&contexts,window,locale)},
            "target-substitution"=>cursor.step(&contexts,"right",locale),
            "context-substitution"=>cursor.step(&substitute,window,locale),
            _=>unreachable!(),
        };
        assert!(result.is_err());assert!(cursor.step(&contexts,window,locale).is_err());
    }}
    let mut contexts=SurfaceContexts::default();contexts.insert("7:left".into(),"graph".into(),view.for_window_instance(window).unwrap()).unwrap();let mut cursor=SurfaceWindowLeaseCursor::default();let mut lease=None;
    for turn in 0..policy["maximumTurns"].as_u64().unwrap(){
        let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(&contexts,window,locale));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));lease=result.unwrap();if lease.is_some(){assert!(turn>=semio_framework_ui_contract::UI_RESIDENT_SLOTS as u64);break;}
    }
    let lease=lease.unwrap();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||contexts.borrow_window_lease(lease).is_some());assert!(result);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}

