//! 🧪 Independent foreign readers remain discoverable and resolve through exact route keys.
use super::*;

const TXT: Dialect = Dialect { artifact_kind: "s.test.text", standard: StandardId("1"), subset: SubsetId("*") };
const ALPHA: Dialect = Dialect { artifact_kind: "s.test.alpha", standard: StandardId("1"), subset: SubsetId("*") };
const BETA: Dialect = Dialect { artifact_kind: "s.test.beta", standard: StandardId("1"), subset: SubsetId("*") };
fn never(_sources: &[ErasedComposeSource]) -> ComposeFuture<'_> {
    Box::pin(async { Err(ComposeError { message: "route fixture".into(), diagnostics: vec![] }) })
}

#[test]
fn same_target_discovery_unions_sources_and_keys_remain_distinct() {
    let native = ComposerEntry { writes: TXT, reads: &[TXT], compose: never };
    let alpha = ComposerEntry { writes: TXT, reads: &[ALPHA], compose: never };
    let beta = ComposerEntry { writes: TXT, reads: &[BETA], compose: never };
    let rows = composer_dialect_rows([&native, &alpha, &beta, &alpha]);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].0, ArtifactDialect::from(TXT));
    assert_eq!(rows[0].1.len(), 3);
    for dialect in [TXT, ALPHA, BETA] {
        assert!(rows[0].1.contains(&dialect.into()));
    }
    let keys = composer_entries_by_key([&native, &alpha, &beta]).unwrap();
    let alpha_key = IoKey::from_owner_counterpart(ALPHA, TXT, IoDirection::Export);
    let beta_key = IoKey::from_owner_counterpart(BETA, TXT, IoDirection::Export);
    assert_eq!(keys[&alpha_key].reads, &[ALPHA]);
    assert_eq!(keys[&beta_key].reads, &[BETA]);
}
