//! 🗒️ `topicStats` — one named inference: BCF is an issue-tracking format, not geometry, so the
//! closest honest derived statistic is a count/fold over the topic tree rather than a bounding
//! box: `topicCount` is `topics.len()`; `commentCount`/`viewpointCount` are the sum of every
//! topic's own `comments.len()`/`viewpoints.len()`; `authorCount` is the size of the distinct-
//! author set built from every topic's `creation_author` PLUS every comment's own `author` —
//! both are genuine BCF-XML author fields (`markup.xsd`'s `<Topic>/<CreationAuthor>` and
//! `<Comment>/<Author>`), never a derived/synthetic identity.

use std::collections::BTreeSet;

use crate::standards::v2_1::subsets::any::schema::snapshot::BcfSnapshot;

//#region 🔖️TopicStats
/// 🗒️ Bcf's topic/comment/viewpoint/author counts.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BcfTopicStats {
    pub topic_count: u32,
    pub comment_count: u32,
    pub viewpoint_count: u32,
    pub author_count: u32,
}

/// 🗒️ Computes [`BcfTopicStats`] via one pass over `topics` — see module doc comment for the
/// exact per-field derivation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
