//! 🔓️ Authored release schedules; every ancestor must be available at the supplied instant.
use crate::schema::{CourseView, Timestamp};

/// 📆️ The latest applicable release; Some(None) awaits scheduling and None is unrestricted.
pub fn course_release(course: &CourseView, page: &str, task: Option<&str>) -> Option<Option<Timestamp>> {
    let unlocks = course.unlocks.as_ref()?;
    let topic = course.topics.as_ref().and_then(|topics| topics.iter().find(|topic| topic.quizzes.iter().any(|quiz| quiz == page))).map(|topic| topic.id.as_str());
    let question = task.map(|task| format!("{page}/{task}"));
    let mut latest: Option<Timestamp> = None;
    for key in [topic, Some(page), question.as_deref()].into_iter().flatten() {
        if let Some(at) = unlocks.get(key) {
            let Some(at) = at else { return Some(None); };
            latest = Some(latest.unwrap_or(0).max(*at));
        }
    }
    latest.map(Some)
}

/// 🚪️ Whether a topic, quiz or question is available at the supplied Unix millisecond instant.
pub fn course_access(course: &CourseView, page: &str, now: Timestamp, task: Option<&str>) -> bool {
    match course_release(course, page, task) {
        None => true,
        Some(Some(at)) => now >= at,
        Some(None) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn shared_release_vectors() {
        let vectors: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔓️access/🔣️.json")).unwrap();
        for row in vectors["cases"].as_array().unwrap() {
            let course: CourseView = serde_json::from_value(serde_json::json!({
                "id": "access", "title": {"en":"Access", "de":"Zugang"},
                "introduction": {"title":{"en":"Access", "de":"Zugang"}, "paragraphs":[]},
                "quizzes":[], "badges":[], "topics":[{"id":"energy", "emoji":"⚡", "title":{"en":"Energy", "de":"Energie"}, "quizzes":["physics"]}],
                "unlocks":row["unlocks"]
            })).unwrap();
            assert_eq!(course_access(&course, row["page"].as_str().unwrap(), row["at"].as_u64().unwrap(), row["task"].as_str()), row["allowed"].as_bool().unwrap(), "{}", row["name"]);
        }
    }
}
