//! 👥️ The presence round trip law: the app-typed presence lane of one instance reaches a peer instance through the real peer-roster ingress, changes no document and leaves with the departing peer. See [`super`].

use super::*;

/// 🫂️ Publishes one roster through the real peer-roster ingress of `app` (reserve, admit, push the remaining pages) and funds one maintenance step per pending turn under `close_loan` until the roster reaches its ordered outcome, whose fault (if any) is returned.
pub async fn publish_fixture_presence_roster<P:PluginApp>(app:&mut P,seq:u64,own_color:Option<u8>,peers:&[protocol::PresencePeer],now_ms:i64,cx:&mut semio_framework_job::StepContext<'_>,close_loan:RetainedCloneGrant)->Option<semio_framework::Fault>{
    let mut pages=Vec::with_capacity(peers.len());
    for peer in peers{pages.push(protocol::encode_presence_peer(peer).await);}
    let admission=app.reserve_presence_ingress(seq).expect("fixture roster admission before decode");
    let generation=admission.generation();
    let first=pages.first().cloned().unwrap_or_default();
    let cursor=protocol::PresenceCommandCursor::admit_page(seq,own_color,pages.len() as u32,semio_framework::kernel::FixedCommandPage::try_copy_from(&first).expect("fixture roster page is fixed authority")).map_err(|(fault,_)|fault).expect("fixture roster cursor");
    app.admit_presence_ingress(admission,cursor,now_ms);
    let mut next=1usize;
    for _ in 0..OriginalFixtureCaller::maximum_turns(){
        if let Some(bytes)=pages.get(next){
            let page=semio_framework::kernel::FixedCommandPage::try_copy_from(bytes).expect("fixture roster page is fixed authority");
            if app.push_presence_ingress(generation,next as u32,page).is_ok(){next+=1}
        }
        if app.maintenance_step(cx,close_loan).is_err(){panic!("fixture roster maintenance step refused")}
        if let Some(outcome)=app.take_presence_outcome(){return outcome.fault}
    }
    panic!("fixture roster did not reach an observable outcome within its authored turn bound")
}

/// ⚖️ Law: `source`'s local app-typed presence (read through `probe`, equal to `expected`) is carried by its ephemeral lane, a roster naming `actor` delivers it to `peer`'s typed peer root and live peer metadata without changing `peer`'s document, and an empty roster retires it again.
pub async fn assert_presence_round_trip<A,M,Q>(source:&VcsArtifactApp<A,M>,peer:&mut VcsArtifactApp<A,M>,actor:&str,probe:impl Fn(&A::Presence)->Q,expected:Q,cx:&mut semio_framework_job::StepContext<'_>,close_loan:RetainedCloneGrant)
where A:ArtifactApp,M:store::ReferenceSpaceMember+store::MemberFactory+Send+'static,Q:PartialEq+std::fmt::Debug{
    assert_eq!(probe(source.stores.presence_store.local()),expected,"the source's local presence holds the published pose");
    let lane=source.ephemeral_snapshot().await;
    assert!(lane.presence_generation>0&&!lane.presence.is_empty(),"a published local presence rides the ephemeral lane");
    let document=peer.document_pack().await.expect("peer document before the roster");
    assert!(peer.stores.presence_store.peers_root().is_empty(),"the peer starts without remote presence");
    let remote=protocol::PresencePeer{actor:actor.to_string(),connected_at_ms:1,label:None,presence_pack:Some(lane.presence),user_id:None,role:None,drag_ghost_json:None,interaction:None,color:Some(1),surface:None,views:Vec::new(),ui:None,tool_run:None,principal_kind:None,active_tool:None,history_edit:None,typing:Vec::new()};
    assert!(publish_fixture_presence_roster(peer,1,Some(9),std::slice::from_ref(&remote),1_000,cx,close_loan).await.is_none(),"the roster is admitted");
    let root=peer.stores.presence_store.peers_root();
    let delivered:Vec<Q>=root.peers().filter(|(name,_)|*name==actor).map(|(_,presence)|probe(presence)).collect();
    assert_eq!(delivered,vec![expected],"the peer observes exactly the source's pose under the source's actor");
    assert_eq!(peer.roster.peer_presence_root().len,1,"the live peer roster names the source once");
    assert_eq!(peer.document_pack().await.expect("peer document after the roster"),document,"presence delivery changes no document");
    drop(root);
    assert!(publish_fixture_presence_roster(peer,2,Some(9),&[],2_000,cx,close_loan).await.is_none(),"the departure roster is admitted");
    assert!(peer.stores.presence_store.peers_root().is_empty(),"the departed peer's typed presence is retired");
    assert_eq!(peer.roster.peer_presence_root().len,0,"the departed peer's live metadata is retired");
    assert_eq!(peer.document_pack().await.expect("peer document after the departure"),document,"presence departure changes no document");
}
