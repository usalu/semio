//! 🗒️ `topicStats` — one named inference: BCF is an issue-tracking format, not geometry, so the
//! closest honest derived statistic is a count/fold over the topic tree rather than a bounding
//! box: `topicCount` is `topics.len()`; `commentCount`/`viewpointCount` are the sum of every
//! topic's own `comments.len()`/`viewpoints.len()`; `authorCount` is the size of the distinct-
//! author set built from every topic's `creation_author` PLUS every comment's own `author` —
//! both are genuine BCF-XML author fields (`markup.xsd`'s `<Topic>/<CreationAuthor>` and
//! `<Comment>/<Author>`), never a derived/synthetic identity.

use std::collections::BTreeSet;

use crate::artifacts::bcf::standards::v2_1::subsets::any::schema::snapshot::BcfSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️TopicStats
/// 🗒️ Bcf's topic/comment/viewpoint/author counts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BcfTopicStats {
    pub topic_count: u32,
    pub comment_count: u32,
    pub viewpoint_count: u32,
    pub author_count: u32,
}

/// 🗒️ Computes [`BcfTopicStats`] via one pass over `topics` — see module doc comment for the
/// exact per-field derivation.
pub fn compute_bcf_topic_stats(snapshot: &BcfSnapshot) -> BcfTopicStats {
    let mut comment_count = 0u32;
    let mut viewpoint_count = 0u32;
    let mut authors: BTreeSet<&str> = BTreeSet::new();

    for topic in &snapshot.topics {
        authors.insert(topic.creation_author.as_str());
        comment_count += topic.comments.len() as u32;
        viewpoint_count += topic.viewpoints.len() as u32;
        for comment in &topic.comments {
            authors.insert(comment.author.as_str());
        }
    }

    BcfTopicStats { topic_count: snapshot.topics.len() as u32, comment_count, viewpoint_count, author_count: authors.len() as u32 }
}
//#endregion 🔖️TopicStats

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::bcf::standards::v2_1::subsets::any::schema::snapshot::{BcfComment, BcfTopic, BcfViewpoint};
    use crate::artifacts::bcf::STDIO_BCF_DOCUMENT_SCHEMA;

    fn comment(guid: &str, author: &str) -> BcfComment {
        BcfComment { guid: guid.into(), date: "2026-01-01T00:00:00Z".into(), author: author.into(), text: "note".into(), viewpoint_ref: None }
    }

    fn viewpoint(guid: &str) -> BcfViewpoint {
        BcfViewpoint { guid: guid.into(), camera: None, components: None, snapshot: None }
    }

    #[test]
    fn counts_topics_comments_viewpoints_and_distinct_authors() {
        let snapshot = BcfSnapshot {
            schema: STDIO_BCF_DOCUMENT_SCHEMA.into(),
            version: "2.1".into(),
            topics: vec![
                BcfTopic {
                    guid: "t1".into(),
                    title: "Clash".into(),
                    description: String::new(),
                    status: "Open".into(),
                    priority: "High".into(),
                    labels: Vec::new(),
                    creation_date: "2026-01-01T00:00:00Z".into(),
                    creation_author: "alice@example.com".into(),
                    comments: vec![comment("c1", "alice@example.com"), comment("c2", "bob@example.com")],
                    viewpoints: vec![viewpoint("v1")],
                },
                BcfTopic {
                    guid: "t2".into(),
                    title: "Clearance".into(),
                    description: String::new(),
                    status: "Closed".into(),
                    priority: "Low".into(),
                    labels: Vec::new(),
                    creation_date: "2026-01-02T00:00:00Z".into(),
                    creation_author: "carol@example.com".into(),
                    comments: vec![comment("c3", "carol@example.com")],
                    viewpoints: vec![viewpoint("v2"), viewpoint("v3")],
                },
            ],
            parts: Vec::new(),
        };
        let stats = compute_bcf_topic_stats(&snapshot);
        assert_eq!(stats.topic_count, 2);
        assert_eq!(stats.comment_count, 3);
        assert_eq!(stats.viewpoint_count, 3);
        // alice, bob, carol — 3 distinct authors across creation_author + comment.author.
        assert_eq!(stats.author_count, 3);
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = BcfSnapshot::default();
        assert_eq!(compute_bcf_topic_stats(&snapshot), compute_bcf_topic_stats(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(compute_bcf_topic_stats(&BcfSnapshot::default()), BcfTopicStats::default());
    }
}
//#endregion 🧪️Tests
