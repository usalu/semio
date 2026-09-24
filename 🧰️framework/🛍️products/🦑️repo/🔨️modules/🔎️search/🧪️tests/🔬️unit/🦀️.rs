use super::*;

fn conjunction(query: &str) -> Vec<MatchQuery> {
    query.split_whitespace().map(MatchQuery::new).collect()
}

#[test]
fn ranking_requires_every_term() {
    let mut index = Index::new();
    index.index("a", "deterministic event replay").unwrap();
    index.index("b", "recursive glob matcher").unwrap();
    index.index("c", "event storage").unwrap();
    let result = index.search(&conjunction("event replay"), 10).unwrap();
    assert_eq!(result.total, 1);
    assert_eq!(result.hits[0].id, "a");
}

#[test]
fn ties_break_on_identifier() {
    let mut index = Index::new();
    index.index("b", "alpha").unwrap();
    index.index("a", "alpha").unwrap();
    let result = index.search(&conjunction("alpha"), 10).unwrap();
    assert_eq!(result.hits.iter().map(|hit| hit.id.as_str()).collect::<Vec<_>>(), vec!["a", "b"]);
}

#[test]
fn size_truncates_but_total_does_not() {
    let mut index = Index::new();
    for id in ["a", "b", "c"] {
        index.index(id, "alpha").unwrap();
    }
    let result = index.search(&conjunction("alpha"), 2).unwrap();
    assert_eq!(result.total, 3);
    assert_eq!(result.hits.len(), 2);
}

#[test]
fn replay_rejects_a_sequence_gap() {
    let log = "{\"schema\":\"semio.search.event/1\",\"seq\":2,\"kind\":\"indexed\",\"id\":\"a\",\"text\":\"x\"}";
    assert!(matches!(Index::replay(log), Err(SearchError::Corrupt(_))));
}

#[test]
fn replay_restores_documents() {
    let mut index = Index::new();
    index.index("a", "alpha").unwrap();
    index.delete("a").unwrap();
    index.index("b", "beta").unwrap();
    let replayed = Index::replay(&index.pending_log()).unwrap();
    assert_eq!(replayed.search(&conjunction("beta"), 10).unwrap().total, 1);
    assert_eq!(replayed.search(&conjunction("alpha"), 10).unwrap().total, 0);
}

#[test]
fn fuzziness_widens_the_match() {
    let mut index = Index::new();
    index.index("a", "replay").unwrap();
    assert_eq!(index.search(&[MatchQuery::new("replayy")], 10).unwrap().total, 0);
    assert_eq!(index.search(&[MatchQuery::new("replayy").with_fuzziness(1)], 10).unwrap().total, 1);
}
