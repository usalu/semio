use crate::artifacts::obj::schema::mutations::ObjMutation;
use crate::artifacts::obj::ObjSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &ObjSnapshot, mutation: &ObjMutation) -> Vec<ObjMutation> {
    <ObjMutation as Mutation<ObjSnapshot>>::inverse(mutation, base).await
}
