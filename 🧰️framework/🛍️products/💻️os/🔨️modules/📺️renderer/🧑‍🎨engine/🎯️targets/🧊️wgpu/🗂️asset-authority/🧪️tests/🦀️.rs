//! 🗂️ Unit tests of asset-authority; the work package moves the tests of its sources here.

use super::*;

#[test]
fn map_tile_retry_retains_exact_shared_request() {
    let mut authority = WorldAssetIoAuthority::default();
    let surface = WorldAssetMetadataId::try_from_str("map").unwrap();
    let key = WorldAssetMetadataId::try_from_str("0/0/0").unwrap();
    for vector in [false, true] {
        let kind = WorldAssetRequestKind::MapTile { surface, key, vector, z: 0, x: 0, y: 0 };
        let token = authority.reserve_request(1, 0, kind, "/0/0/0").unwrap();
        let mut fetch = RendererAssetFetchOwner::Shared(authority.take_next().unwrap());
        assert!(fetch.allows_transport_retry());

        let owner = fetch.owner_mut();
        authority.reserve_response(owner, 3).unwrap();
        assert!(owner.push_page(WorldAssetResponsePage::try_from_owned(vec![1, 2]).ok().unwrap()).is_ok());
        assert_eq!(owner.retire_partial_response_step(), Ok(true));
        authority.release_partial_response(owner).unwrap();

        assert_eq!(authority.reserve_request(2, 0, kind, "/0/0/0"), Ok(token));
        assert!(authority.take_next().is_none());
        authority.reserve_response(owner, 1).unwrap();
        assert!(owner.push_page(WorldAssetResponsePage::try_from_owned(vec![3]).ok().unwrap()).is_ok());
        authority.seal_response(owner).unwrap();
        assert_eq!(owner.received_bytes(), 1);
        assert_eq!(owner.token(), token);
        let RendererAssetFetchOwner::Shared(owner) = fetch else { unreachable!() };
        assert!(authority.return_owner(owner).is_ok());
        let mut owner = authority.take_completed(token, 1, 0).unwrap();
        owner.begin_close();
        while !owner.close_step() {}
        assert!(authority.finish(owner).is_ok());
    }
}
