//! 🌐️ HTTP server core of the coordinator: the server, its construction, authorization, request handling and guard. See [`super`].

use super::*;

//#region 🌐️Http
/// 🗄️ The coordinator's request-scoped state.
#[derive(Debug)]
pub struct Server {
    pub config: Config,
    pub repository: Repository,
    pub(super) github_cache: Mutex<HashMap<String, (String, u64)>>,
}

impl Server {
    /// ⚙️ Wires a server over an already opened repository.
    pub fn new(config: Config, repository: Repository) -> Self {
        Self { config, repository, github_cache: Mutex::new(HashMap::new()) }
    }

    fn authorized(&self, request: &Request) -> bool {
        if self.config.token.is_empty() {
            return true;
        }
        let authorization = request.header("authorization");
        match authorization.split_once(' ') {
            Some(("Bearer", token)) => token == self.config.token,
            _ => false,
        }
    }

    /// 🧭️ Answers one request through the full coordinator route table.
    pub fn handle(&self, request: &Request) -> Response {
        match request.path.as_str() {
            "/healthz" => self.guard(request, "GET", false, |_| Response::text(200, "ok")),
            "/ticket/open" => self.guard(request, "POST", true, |server| server.ticket_open(request)),
            "/ticket/close" => self.guard(request, "POST", true, |server| server.ticket_close(request)),
            "/ticket/reopen" => self.guard(request, "POST", true, |server| server.ticket_reopen(request)),
            "/tickets" => self.guard(request, "GET", true, |server| server.tickets_query(request)),
            "/diff/ingest" => self.guard(request, "POST", true, |server| server.diff_ingest(request)),
            "/repo/reindex" => self.guard(request, "POST", true, |server| server.reindex(request)),
            "/repo/index-file" => self.guard(request, "POST", true, |server| server.index_file(request)),
            "/warnings" => self.guard(request, "GET", true, |server| server.warnings(request)),
            "/breachs" => self.guard(request, "GET", true, |server| server.breachs(request)),
            "/scopes" => self.guard(request, "GET", true, |server| server.scopes(request)),
            "/events" | "/api/v1/events" => self.guard(request, "POST", true, |server| server.events(request)),
            "/webhooks/github" => self.guard(request, "POST", true, |server| server.github_webhook(request)),
            path if path.starts_with("/ticket/") => self.guard(request, "GET", true, |server| server.ticket_detail(request)),
            _ => Response::error(404, "not found"),
        }
    }

    pub(super) fn guard(&self, request: &Request, method: &str, authenticated: bool, handler: impl FnOnce(&Self) -> Response) -> Response {
        if request.method != method {
            return Response::empty(405);
        }
        if authenticated && !self.authorized(request) {
            return Response::error(401, "unauthorized");
        }
        handler(self)
    }
}
//#endregion 🌐️Http
