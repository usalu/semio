//! 🏠️ Context core of the production context: the filesystem repo context, its construction, accessors, wiring of the ticket, goal and todo aggregates and its readers. See [`super`].

use super::*;

//#region 🗄️FsRepoContext
/// 🗄️ The production context: every aggregate answered from the repository on disk through the
/// domain crate that owns it. Twin of `NewRepoContext` in `🔗️graphql/📦️packages/🐹️go/🩻️context/🐹️.go`.
pub struct FsRepoContext {
    pub(super) root: std::path::PathBuf,
    pub(super) codebase: Codebase,
}

impl FsRepoContext {
    /// 🆕️ Opens the context of one repository root.
    pub fn open(root: &std::path::Path) -> FsRepoContext {
        FsRepoContext { root: root.to_path_buf(), codebase: Codebase::new(root) }
    }

    /// 🏠️ The repository root the context reads.
    pub fn root(&self) -> &std::path::Path {
        &self.root
    }

    /// 🗂️ The codebase the context walks.
    pub fn codebase(&self) -> &Codebase {
        &self.codebase
    }

    /// ✍️ The alias the invoking git identity resolves to.
    pub fn author(&self) -> String {
        semio_framework_repo_contributors::git_author_alias(&self.root)
    }

    pub(super) fn slash_root(&self) -> String {
        self.root.display().to_string().replace('\\', "/")
    }

    fn layout(&self) -> TicketLayout {
        TicketLayout::new(format!("{}/{}/{}", self.slash_root(), semio_framework_repo_workspace::SEMIO_DIR_NAME, semio_framework_repo_workspace::REPO_DIR_NAME))
    }

    /// 🎫️ Runs one operation against the ticket domain bound to this repository.
    pub fn with_tickets<R>(&self, kind: McpClientKind, body: impl FnOnce(&TicketService<'_, FileTicketStore, ManagementProviders, TicketSystemClock, CoordinatorEventSink>) -> R) -> R {
        let store = FileTicketStore::new();
        let tracker = semio_framework_repo_providers::system_management_provider();
        let clock = TicketSystemClock;
        let events = CoordinatorEventSink;
        let roots = PlanRoots { repo_root: self.slash_root(), home_dir: std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).unwrap_or_default().replace('\\', "/") };
        let metadata = InteractionMetadata { author: self.author(), system: (if cfg!(target_os = "macos") { "mac" } else if cfg!(windows) { "windows" } else { "linux" }).into(), checkpoint: VersionControlProvider::current_checkpoint(&semio_framework_repo_providers::system_version_control_provider(), &self.slash_root()).unwrap_or_default() };
        let service = TicketService::new(self.layout(), &store, &tracker, &clock, &events).with_roots(roots).with_mcp_client(kind).with_interaction_metadata(metadata);
        body(&service)
    }

    /// 🎯️ Runs one operation against the goal aggregate bound to this repository.
    pub fn with_goals<R>(&self, no_management: bool, body: impl FnOnce(&Goals<'_>) -> R) -> R {
        let store = FsGoalStore::new(&self.root);
        let emitter = CoordinatorEmitter;
        let author = self.author();
        if no_management {
            body(&Goals::new(&store, &NullManagement, &emitter, &author))
        } else {
            body(&Goals::new(&store, &ProviderManagement::system(), &emitter, &author))
        }
    }

    pub(super) fn with_todos<R>(&self, body: impl FnOnce(&Todos<'_>) -> R) -> R {
        let tree = FsTodoTree::new(&self.root);
        let emitter = CoordinatorEmitter;
        body(&Todos::new(&tree, &emitter, &self.author()))
    }

    pub(super) fn considered(&self) -> CodebaseContext<'_> {
        let mut context = CodebaseContext::new(&self.codebase);
        context.load_bundles();
        context.load_files();
        context
    }

    pub(super) fn read(&self, path: &str) -> Option<String> {
        std::fs::read_to_string(self.root.join(path)).ok()
    }

    pub(super) fn ticket(&self, year: i64, month: i64, day: i64, slug: &str) -> Result<Ticket, ContextError> {
        self.with_tickets(McpClientKind::Generic, |service| service.read(&TicketId::new(year, month, day, slug))).map_err(|error| ContextError::new(error.message))
    }

    pub(super) fn ticket_by_id(&self, id: &str) -> Result<Ticket, ContextError> {
        let id = TicketId::parse(id).map_err(|error| ContextError::new(error.message))?;
        self.ticket(id.year, id.month, id.day, &id.slug)
    }
}
//#endregion 🗄️FsRepoContext
