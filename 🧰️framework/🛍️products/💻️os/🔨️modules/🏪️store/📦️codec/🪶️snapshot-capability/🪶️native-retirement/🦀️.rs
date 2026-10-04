use super::ArtifactSqliteSnapshot;

pub(super) struct OwnedSqliteSnapshot<P: ArtifactSqliteSnapshot>(Option<P>);

impl<P: ArtifactSqliteSnapshot> OwnedSqliteSnapshot<P> {
    pub(super) fn new(snapshot: P) -> Self {
        Self(Some(snapshot))
    }
}

impl<P: ArtifactSqliteSnapshot> std::ops::Deref for OwnedSqliteSnapshot<P> {
    type Target = P;
    fn deref(&self) -> &P {
        self.0.as_ref().expect("owned snapshot is live until retirement")
    }
}

impl<P: ArtifactSqliteSnapshot> Drop for OwnedSqliteSnapshot<P> {
    fn drop(&mut self) {
        if let Some(snapshot) = self.0.take() {
            snapshot.retire_sqlite_snapshot();
        }
    }
}
