//! 🦠️ Forward and inverse of the BCF oracle: the mutation kinds applied to the model and their computed inverse specs. See [`super`].

use super::*;

//#region 🔖️Forward
fn find_viewpoint_mut<'a>(doc: &'a mut ODoc, topic_guid: &str, guid: &str) -> Result<&'a mut OViewpoint, String> {
    let topic = doc.topics.iter_mut().find(|topic| topic.guid == topic_guid).ok_or_else(|| format!("no topic named {topic_guid:?}"))?;
    topic.viewpoints.iter_mut().find(|viewpoint| viewpoint.guid == guid).ok_or_else(|| format!("no viewpoint named {guid:?}"))
}

/// 🦠️ Applies one declared mutation kind, described by `spec` (`{"kind": ..., "params": {...}}`),
/// to an already-decoded document. An unrecognised kind, or a named topic/comment/viewpoint that
/// does not exist, is an error — never a silent no-op.
pub(super) fn apply_kind(doc: &mut ODoc, kind: &str, params: &Json) -> Result<(), String> {
    let index_param = || match params.get("index") {
        Some(Json::Number(index)) => Some(*index as usize),
        _ => None,
    };
    match kind {
        "set-version" => doc.version = params.str("version"),
        "insert-topic" => {
            let topic = topic_from_json(&params.get("topic").cloned().unwrap_or(Json::Null))?;
            if doc.topics.iter().any(|existing| existing.guid == topic.guid) {
                return Err(format!("insert-topic: a topic named {:?} already exists", topic.guid));
            }
            let at = index_param().unwrap_or(doc.topics.len()).min(doc.topics.len());
            doc.topics.insert(at, topic);
        }
        "remove-topic" => {
            let guid = params.str("guid");
            let before = doc.topics.len();
            doc.topics.retain(|topic| topic.guid != guid);
            if doc.topics.len() == before {
                return Err(format!("remove-topic: no topic named {guid:?}"));
            }
        }
        "set-topic-markup" => {
            let guid = params.str("guid");
            let topic = doc.topics.iter_mut().find(|topic| topic.guid == guid).ok_or_else(|| format!("set-topic-markup: no topic named {guid:?}"))?;
            if let Some(Json::String(value)) = params.get("title") {
                topic.title = value.clone();
            }
            if let Some(Json::String(value)) = params.get("description") {
                topic.description = value.clone();
            }
            if let Some(Json::String(value)) = params.get("status") {
                topic.status = value.clone();
            }
            if let Some(Json::String(value)) = params.get("priority") {
                topic.priority = value.clone();
            }
            if let Some(Json::Array(_)) = params.get("labels") {
                topic.labels = strings(params.array("labels"));
            }
            if let Some(Json::String(value)) = params.get("creationDate") {
                topic.creation_date = value.clone();
            }
            if let Some(Json::String(value)) = params.get("creationAuthor") {
                topic.creation_author = value.clone();
            }
        }
        "insert-comment" => {
            let topic_guid = params.str("topicGuid");
            let comment = comment_from_json(&params.get("comment").cloned().unwrap_or(Json::Null));
            let topic = doc.topics.iter_mut().find(|topic| topic.guid == topic_guid).ok_or_else(|| format!("insert-comment: no topic named {topic_guid:?}"))?;
            if topic.comments.iter().any(|existing| existing.guid == comment.guid) {
                return Err(format!("insert-comment: a comment named {:?} already exists", comment.guid));
            }
            let at = index_param().unwrap_or(topic.comments.len()).min(topic.comments.len());
            topic.comments.insert(at, comment);
        }
        "remove-comment" => {
            let topic_guid = params.str("topicGuid");
            let guid = params.str("guid");
            let topic = doc.topics.iter_mut().find(|topic| topic.guid == topic_guid).ok_or_else(|| format!("remove-comment: no topic named {topic_guid:?}"))?;
            let before = topic.comments.len();
            topic.comments.retain(|comment| comment.guid != guid);
            if topic.comments.len() == before {
                return Err(format!("remove-comment: no comment named {guid:?}"));
            }
        }
        "set-comment" => {
            let topic_guid = params.str("topicGuid");
            let guid = params.str("guid");
            let topic = doc.topics.iter_mut().find(|topic| topic.guid == topic_guid).ok_or_else(|| format!("set-comment: no topic named {topic_guid:?}"))?;
            let comment = topic.comments.iter_mut().find(|comment| comment.guid == guid).ok_or_else(|| format!("set-comment: no comment named {guid:?}"))?;
            if let Some(Json::String(value)) = params.get("date") {
                comment.date = value.clone();
            }
            if let Some(Json::String(value)) = params.get("author") {
                comment.author = value.clone();
            }
            if let Some(Json::String(value)) = params.get("text") {
                comment.text = value.clone();
            }
            if let Some(value) = params.get("viewpointRef") {
                comment.viewpoint_ref = match value {
                    Json::String(reference) if !reference.is_empty() => Some(reference.clone()),
                    _ => None,
                };
            }
        }
        "insert-viewpoint" => {
            let topic_guid = params.str("topicGuid");
            let viewpoint = viewpoint_from_json(&params.get("viewpoint").cloned().unwrap_or(Json::Null))?;
            let topic = doc.topics.iter_mut().find(|topic| topic.guid == topic_guid).ok_or_else(|| format!("insert-viewpoint: no topic named {topic_guid:?}"))?;
            if topic.viewpoints.iter().any(|existing| existing.guid == viewpoint.guid) {
                return Err(format!("insert-viewpoint: a viewpoint named {:?} already exists", viewpoint.guid));
            }
            let at = index_param().unwrap_or(topic.viewpoints.len()).min(topic.viewpoints.len());
            topic.viewpoints.insert(at, viewpoint);
        }
        "remove-viewpoint" => {
            let topic_guid = params.str("topicGuid");
            let guid = params.str("guid");
            let topic = doc.topics.iter_mut().find(|topic| topic.guid == topic_guid).ok_or_else(|| format!("remove-viewpoint: no topic named {topic_guid:?}"))?;
            let before = topic.viewpoints.len();
            topic.viewpoints.retain(|viewpoint| viewpoint.guid != guid);
            if topic.viewpoints.len() == before {
                return Err(format!("remove-viewpoint: no viewpoint named {guid:?}"));
            }
        }
        "set-viewpoint-camera" => {
            let viewpoint = find_viewpoint_mut(doc, &params.str("topicGuid"), &params.str("guid"))?;
            viewpoint.camera = match params.get("camera") {
                Some(Json::Null) | None => None,
                Some(node) => Some(camera_from_json(node)?),
            };
        }
        "set-viewpoint-components" => {
            let viewpoint = find_viewpoint_mut(doc, &params.str("topicGuid"), &params.str("guid"))?;
            viewpoint.components = match params.get("components") {
                Some(Json::Null) | None => None,
                Some(node) => Some(components_from_json(node)),
            };
        }
        "set-viewpoint-snapshot" => {
            let viewpoint = find_viewpoint_mut(doc, &params.str("topicGuid"), &params.str("guid"))?;
            viewpoint.snapshot = optional_bytes(params.get("snapshot"))?;
        }
        other => return Err(format!("mutation kind {other:?} has no oracle implementation")),
    }
    Ok(())
}
//#endregion 🔖️Forward

//#region 🔖️Inverse
/// ↩️ Reads `base` (the CURRENT, pre-mutation document) to build the wire spec that undoes `{kind,
/// params}` — same law `BcfMutation::inverse` proves at the Rust-model level, computed here
/// against the reference libraries instead. A target the forward step could not find leaves
/// nothing to undo (`None`), as that method's empty inverse does.
pub(super) fn inverse_spec(base: &ODoc, kind: &str, params: &Json) -> Result<Option<Json>, String> {
    let spec = |inverse_kind: &str, inverse_params: Json| obj(vec![("kind", Json::String(inverse_kind.to_string())), ("params", inverse_params)]);
    let find_topic = |guid: &str| base.topics.iter().find(|topic| topic.guid == guid);
    let find_comment = |topic_guid: &str, guid: &str| find_topic(topic_guid).and_then(|topic| topic.comments.iter().find(|comment| comment.guid == guid));
    let find_viewpoint = |topic_guid: &str, guid: &str| find_topic(topic_guid).and_then(|topic| topic.viewpoints.iter().find(|viewpoint| viewpoint.guid == guid));

    Ok(Some(match kind {
        "set-version" => spec("set-version", obj(vec![("version", Json::String(base.version.clone()))])),
        "insert-topic" => spec("remove-topic", obj(vec![("guid", Json::String(params.get("topic").map(|topic| topic.str("guid")).unwrap_or_default()))])),
        "remove-topic" => match find_topic(&params.str("guid")) {
            Some(topic) => spec("insert-topic", obj(vec![("topic", topic_to_json(topic)), ("index", Json::Number(base.topics.iter().position(|candidate| candidate.guid == topic.guid).unwrap_or_default() as f64))])),
            None => return Ok(None),
        },
        "set-topic-markup" => {
            let guid = params.str("guid");
            let mut inverse_params = vec![("guid", Json::String(guid.clone()))];
            if let Some(topic) = find_topic(&guid) {
                if params.get("title").is_some() {
                    inverse_params.push(("title", Json::String(topic.title.clone())));
                }
                if params.get("description").is_some() {
                    inverse_params.push(("description", Json::String(topic.description.clone())));
                }
                if params.get("status").is_some() {
                    inverse_params.push(("status", Json::String(topic.status.clone())));
                }
                if params.get("priority").is_some() {
                    inverse_params.push(("priority", Json::String(topic.priority.clone())));
                }
                if params.get("labels").is_some() {
                    inverse_params.push(("labels", strings_json(&topic.labels)));
                }
                if params.get("creationDate").is_some() {
                    inverse_params.push(("creationDate", Json::String(topic.creation_date.clone())));
                }
                if params.get("creationAuthor").is_some() {
                    inverse_params.push(("creationAuthor", Json::String(topic.creation_author.clone())));
                }
            }
            spec("set-topic-markup", obj(inverse_params))
        }
        "insert-comment" => spec("remove-comment", obj(vec![("topicGuid", Json::String(params.str("topicGuid"))), ("guid", Json::String(params.get("comment").map(|comment| comment.str("guid")).unwrap_or_default()))])),
        "remove-comment" => match find_comment(&params.str("topicGuid"), &params.str("guid")) {
            Some(comment) => spec("insert-comment", obj(vec![("topicGuid", Json::String(params.str("topicGuid"))), ("comment", comment_to_json(comment)), ("index", Json::Number(find_topic(&params.str("topicGuid")).and_then(|topic| topic.comments.iter().position(|candidate| candidate.guid == comment.guid)).unwrap_or_default() as f64))])),
            None => return Ok(None),
        },
        "set-comment" => {
            let topic_guid = params.str("topicGuid");
            let guid = params.str("guid");
            let mut inverse_params = vec![("topicGuid", Json::String(topic_guid.clone())), ("guid", Json::String(guid.clone()))];
            if let Some(comment) = find_comment(&topic_guid, &guid) {
                if params.get("date").is_some() {
                    inverse_params.push(("date", Json::String(comment.date.clone())));
                }
                if params.get("author").is_some() {
                    inverse_params.push(("author", Json::String(comment.author.clone())));
                }
                if params.get("text").is_some() {
                    inverse_params.push(("text", Json::String(comment.text.clone())));
                }
                if params.get("viewpointRef").is_some() {
                    inverse_params.push((
                        "viewpointRef",
                        match &comment.viewpoint_ref {
                            Some(reference) => Json::String(reference.clone()),
                            None => Json::Null,
                        },
                    ));
                }
            }
            spec("set-comment", obj(inverse_params))
        }
        "insert-viewpoint" => spec("remove-viewpoint", obj(vec![("topicGuid", Json::String(params.str("topicGuid"))), ("guid", Json::String(params.get("viewpoint").map(|viewpoint| viewpoint.str("guid")).unwrap_or_default()))])),
        "remove-viewpoint" => match find_viewpoint(&params.str("topicGuid"), &params.str("guid")) {
            Some(viewpoint) => spec("insert-viewpoint", obj(vec![("topicGuid", Json::String(params.str("topicGuid"))), ("viewpoint", viewpoint_to_json(viewpoint)), ("index", Json::Number(find_topic(&params.str("topicGuid")).and_then(|topic| topic.viewpoints.iter().position(|candidate| candidate.guid == viewpoint.guid)).unwrap_or_default() as f64))])),
            None => return Ok(None),
        },
        "set-viewpoint-camera" => {
            let camera = find_viewpoint(&params.str("topicGuid"), &params.str("guid")).and_then(|viewpoint| viewpoint.camera.as_ref());
            spec(
                "set-viewpoint-camera",
                obj(vec![
                    ("topicGuid", Json::String(params.str("topicGuid"))),
                    ("guid", Json::String(params.str("guid"))),
                    (
                        "camera",
                        match camera {
                            Some(camera) => camera_to_json(camera),
                            None => Json::Null,
                        },
                    ),
                ]),
            )
        }
        "set-viewpoint-components" => {
            let components = find_viewpoint(&params.str("topicGuid"), &params.str("guid")).and_then(|viewpoint| viewpoint.components.as_ref());
            spec(
                "set-viewpoint-components",
                obj(vec![
                    ("topicGuid", Json::String(params.str("topicGuid"))),
                    ("guid", Json::String(params.str("guid"))),
                    (
                        "components",
                        match components {
                            Some(components) => components_to_json(components),
                            None => Json::Null,
                        },
                    ),
                ]),
            )
        }
        "set-viewpoint-snapshot" => {
            let snapshot = find_viewpoint(&params.str("topicGuid"), &params.str("guid")).and_then(|viewpoint| viewpoint.snapshot.as_ref());
            spec(
                "set-viewpoint-snapshot",
                obj(vec![
                    ("topicGuid", Json::String(params.str("topicGuid"))),
                    ("guid", Json::String(params.str("guid"))),
                    (
                        "snapshot",
                        match snapshot {
                            Some(bytes) => bytes_json(bytes),
                            None => Json::Null,
                        },
                    ),
                ]),
            )
        }
        other => return Err(format!("mutation kind {other:?} has no oracle inverse")),
    }))
}
//#endregion 🔖️Inverse
