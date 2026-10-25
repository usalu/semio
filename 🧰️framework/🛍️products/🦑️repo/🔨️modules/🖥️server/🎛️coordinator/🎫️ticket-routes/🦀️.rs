//! 🎫️ Ticket routes of the coordinator: the ticket open, close, reopen, query and detail routes. See [`super`].

use super::*;

//#region 🌐️Http
impl Server {
    pub(super) fn ticket_open(&self, request: &Request) -> Response {
        let Some(payload) = request.json() else {
            return Response::error(400, "invalid JSON body");
        };
        let text = |key: &str| payload.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
        if text("ticket_id").is_empty() || text("title").is_empty() {
            return Response::error(400, "ticket_id and title required");
        }
        let ticket = Ticket {
            id: text("ticket_id"),
            status: "open".to_string(),
            title: text("title"),
            prompt: text("prompt"),
            llm: text("llm"),
            client: text("client"),
            author: text("author"),
            github_issue: text("github_issue"),
            created_at: now_rfc3339(),
            ..Ticket::default()
        };
        match self.record_and_publish(&ticket, "TicketOpened") {
            Err(failure) => Response::error(500, &failure.to_string()),
            Ok(()) => Response::json(200, &ticket.to_value()),
        }
    }

    pub(super) fn ticket_close(&self, request: &Request) -> Response {
        let Some(payload) = request.json() else {
            return Response::error(400, "invalid JSON body");
        };
        let text = |key: &str| payload.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
        if text("ticket_id").is_empty() || text("summary").is_empty() {
            return Response::error(400, "ticket_id and summary required");
        }
        let mut ticket = match self.repository.project_ticket(&text("ticket_id")) {
            Err(failure) => return Response::error(404, &failure.to_string()),
            Ok(ticket) => ticket,
        };
        ticket.status = "closed".to_string();
        ticket.summary = text("summary");
        ticket.closed_at = Some(now_rfc3339());
        match self.record_and_publish(&ticket, "TicketClosed") {
            Err(failure) => Response::error(500, &failure.to_string()),
            Ok(()) => Response::json(200, &ticket.to_value()),
        }
    }

    pub(super) fn ticket_reopen(&self, request: &Request) -> Response {
        let Some(payload) = request.json() else {
            return Response::error(400, "invalid JSON body");
        };
        let text = |key: &str| payload.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
        if text("ticket_id").is_empty() || text("prompt").is_empty() {
            return Response::error(400, "ticket_id and prompt required");
        }
        let mut ticket = match self.repository.project_ticket(&text("ticket_id")) {
            Err(failure) => return Response::error(404, &failure.to_string()),
            Ok(ticket) => ticket,
        };
        ticket.status = "open".to_string();
        ticket.prompt = text("prompt");
        ticket.llm = text("llm");
        if !text("title").is_empty() {
            ticket.title = text("title");
        }
        ticket.closed_at = None;
        match self.record_and_publish(&ticket, "TicketReopened") {
            Err(failure) => Response::error(500, &failure.to_string()),
            Ok(()) => Response::json(200, &ticket.to_value()),
        }
    }

    fn record_and_publish(&self, ticket: &Ticket, kind: &str) -> Result<(), StoreError> {
        self.repository.record_ticket(ticket)?;
        self.publish(kind, "repo-cli", &ticket.to_value())
    }

    pub(super) fn tickets_query(&self, request: &Request) -> Response {
        let status = request.query.get("status").cloned().unwrap_or_default();
        let tickets: Vec<Value> = self.repository.project_tickets(&status).iter().map(Ticket::to_value).collect();
        Response::json(200, &Value::Array(tickets))
    }

    pub(super) fn ticket_detail(&self, request: &Request) -> Response {
        let rest = request.path.trim_start_matches("/ticket/");
        if rest.is_empty() {
            return Response::error(404, "ticket not found");
        }
        if let Some(ticket_id) = rest.strip_suffix("/claims") {
            let claims: Vec<Value> = self.repository.project_claims_by_ticket(ticket_id).iter().map(Scope::to_value).collect();
            return Response::json(200, &Value::Array(claims));
        }
        match self.repository.project_ticket(rest) {
            Err(failure) => Response::error(404, &failure.to_string()),
            Ok(ticket) => Response::json(200, &ticket.to_value()),
        }
    }
}
//#endregion 🌐️Http
