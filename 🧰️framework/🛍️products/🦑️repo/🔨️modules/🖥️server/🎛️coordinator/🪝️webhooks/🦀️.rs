//! 🪝️ Webhooks of the coordinator: the GitHub and Discord webhook routes, the signature verification and the identity helpers. See [`super`].

use super::*;

//#region 🪝️Webhooks
impl Server {
    pub(super) fn github_webhook(&self, request: &Request) -> Response {
        if !self.config.github_secret.is_empty() {
            let signature = request.header("x-hub-signature-256");
            if !verify_github_signature(&request.body, &signature, &self.config.github_secret) {
                return Response::error(401, "invalid signature");
            }
        }
        let event_type = request.header("x-github-event");
        let mut published = Map::new();
        published.insert("type".to_string(), Value::String(event_type.clone()));
        if let Err(failure) = self.publish("GitHubIssueEventReceived", "github", &Value::Object(published)) {
            return Response::error(500, &failure.to_string());
        }
        let payload = request.json().unwrap_or(Value::Null);
        match event_type.as_str() {
            "issue_comment" => self.cache_github_comment(&payload),
            "issues" => {
                if let Err(failure) = self.github_issue_event(&payload) {
                    return Response::error(500, &failure.to_string());
                }
            }
            "push" => {
                if let Err(failure) = self.github_push_event(&payload) {
                    return Response::error(500, &failure.to_string());
                }
            }
            _ => {}
        }
        Response::empty(200)
    }

    fn cache_github_comment(&self, payload: &Value) {
        let (issue, repo, actor) = github_identity(payload);
        let body = payload.get("comment").and_then(|comment| comment.get("body")).and_then(Value::as_str).unwrap_or_default().to_string();
        if issue == 0 || repo.is_empty() || actor.is_empty() || body.is_empty() {
            return;
        }
        self.github_cache.lock().expect("github").insert(format!("{repo}#{issue}#{actor}"), (body, unix_seconds()));
    }

    fn cached_comment(&self, repo: &str, issue: i64, actor: &str) -> String {
        let key = format!("{repo}#{issue}#{actor}");
        let mut cache = self.github_cache.lock().expect("github");
        match cache.get(&key) {
            None => String::new(),
            Some((body, when)) => {
                if unix_seconds().saturating_sub(*when) > 90 {
                    cache.remove(&key);
                    String::new()
                } else {
                    body.clone()
                }
            }
        }
    }

    fn github_issue_event(&self, payload: &Value) -> Result<(), StoreError> {
        let action = payload.get("action").and_then(Value::as_str).unwrap_or_default();
        let (issue, repo, actor) = github_identity(payload);
        if issue == 0 || repo.is_empty() || actor.is_empty() {
            return Ok(());
        }
        let comment = self.cached_comment(&repo, issue, &actor);
        if comment.is_empty() {
            return Ok(());
        }
        let kind = match action {
            "closed" => "TicketClosed",
            "reopened" => "TicketReopened",
            _ => return Ok(()),
        };
        let mut object = Map::new();
        object.insert("issue".to_string(), Value::from(issue));
        object.insert("comment".to_string(), Value::String(comment));
        self.publish(kind, "github", &Value::Object(object))
    }

    fn github_push_event(&self, payload: &Value) -> Result<(), StoreError> {
        let (_, _, mut actor) = github_identity(payload);
        if actor.is_empty() {
            actor = payload.get("pusher").and_then(|pusher| pusher.get("name")).and_then(Value::as_str).unwrap_or_default().to_string();
        }
        let mut files = Vec::new();
        if let Some(commits) = payload.get("commits").and_then(Value::as_array) {
            for commit in commits {
                for key in ["added", "modified"] {
                    if let Some(items) = commit.get(key).and_then(Value::as_array) {
                        files.extend(items.iter().filter_map(Value::as_str).map(str::to_string));
                    }
                }
            }
        }
        if actor.is_empty() || files.is_empty() {
            return Ok(());
        }
        self.repository.record_checkpoint(&actor, &files)
    }

    /// 🔔️ Sends the Discord notification a ticket lifecycle event asks for, when one is configured.
    pub(super) fn notify(&self, kind: &str, payload: &Value) {
        let title = match kind {
            "TicketOpened" | "TicketReopened" => "# Prompt",
            "TicketClosed" => "# Summary",
            _ => return,
        };
        self.notify_discord(title, &to_go_json(payload));
    }

    fn notify_discord(&self, title: &str, body: &str) {
        if self.config.discord_webhook.is_empty() {
            return;
        }
        let mut object = Map::new();
        object.insert("content".to_string(), Value::String(format!("{title}\n{body}")));
        let encoded = to_go_json(&Value::Object(object));
        let Some(rest) = self.config.discord_webhook.strip_prefix("http://") else { return };
        let (authority, path) = rest.split_once('/').unwrap_or((rest, ""));
        let Ok(mut stream) = TcpStream::connect(authority) else { return };
        let request = format!(
            "POST /{path} HTTP/1.1\r\nHost: {authority}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{encoded}",
            encoded.len()
        );
        let _ = stream.write_all(request.as_bytes());
    }
}

fn github_identity(payload: &Value) -> (i64, String, String) {
    let issue = payload.get("issue").and_then(|issue| issue.get("number")).and_then(Value::as_i64).unwrap_or(0);
    let repo = payload.get("repository").and_then(|repo| repo.get("full_name")).and_then(Value::as_str).unwrap_or_default().to_string();
    let actor = payload.get("sender").and_then(|sender| sender.get("login")).and_then(Value::as_str).unwrap_or_default().to_string();
    (issue, repo, actor)
}

/// 🔏️ Validates the HMAC-SHA256 signature of a webhook payload.
pub fn verify_github_signature(body: &[u8], signature: &str, secret: &str) -> bool {
    let Some((_, provided)) = signature.split_once('=') else { return false };
    let computed = hmac_sha256(secret.as_bytes(), body);
    computed.len() == provided.len() && computed.bytes().zip(provided.bytes()).fold(0u8, |sum, (left, right)| sum | (left ^ right)) == 0
}

fn hmac_sha256(key: &[u8], message: &[u8]) -> String {
    let mut block = [0u8; 64];
    if key.len() > 64 {
        let folded = hex_to_bytes(&digest(key));
        block[..folded.len()].copy_from_slice(&folded);
    } else {
        block[..key.len()].copy_from_slice(key);
    }
    let mut inner = Sha256::new();
    inner.update(&block.map(|byte| byte ^ 0x36));
    inner.update(message);
    let mut outer = Sha256::new();
    outer.update(&block.map(|byte| byte ^ 0x5c));
    outer.update(&hex_to_bytes(&inner.finish()));
    outer.finish()
}

fn hex_to_bytes(text: &str) -> Vec<u8> {
    (0..text.len() / 2).filter_map(|index| u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).ok()).collect()
}
//#endregion 🪝️Webhooks
