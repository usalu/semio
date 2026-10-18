package events

// #region 🌨️Payloads

// 📦️TicketPayload holds common ticket identifiers.
type TicketPayload struct {
	ID    string `json:"id"`
	Year  int    `json:"year,omitempty"`
	Month int    `json:"month,omitempty"`
	Day   int    `json:"day,omitempty"`
	Slug  string `json:"slug,omitempty"`
}

// 🎫️TicketOpenPayload payload for ticket.open.
type TicketOpenPayload struct {
	TicketPayload
	Title  string `json:"title"`
	Prompt string `json:"prompt"`
	LLM    string `json:"llm,omitempty"`
	Effort string `json:"effort,omitempty"`
	Client string `json:"client"`
	Author string `json:"author,omitempty"`
	Goal   string `json:"goal"`
	Parent string `json:"parent,omitempty"`
}

// 📪️TicketClosePayload payload for ticket.close.
type TicketClosePayload struct {
	TicketPayload
	Summary string   `json:"summary"`
	Files   []string `json:"files"`
	Author  string   `json:"author,omitempty"`
}

// 🔓️TicketReopenPayload payload for ticket.reopen.
type TicketReopenPayload struct {
	TicketPayload
	Prompt string `json:"prompt"`
	LLM    string `json:"llm,omitempty"`
	Effort string `json:"effort,omitempty"`
	Client string `json:"client"`
	Author string `json:"author,omitempty"`
}

// ♻️TicketChangePayload payload for ticket.change.
type TicketChangePayload struct {
	TicketPayload
	Title  *string `json:"title,omitempty"`
	Prompt *string `json:"prompt,omitempty"`
	LLM    *string `json:"llm,omitempty"`
	Effort *string `json:"effort,omitempty"`
	Goal   *string `json:"goal,omitempty"`
	Parent *string `json:"parent,omitempty"`
	Author string  `json:"author,omitempty"`
}

// ⛳️GoalPayload holds common goal identifiers.
type GoalPayload struct {
	ID string `json:"id"`
}

// 🎯️GoalOpenPayload payload for goal.open.
type GoalOpenPayload struct {
	GoalPayload
	Title       string `json:"title"`
	Description string `json:"description,omitempty"`
	LLM         string `json:"llm,omitempty"`
	Effort      string `json:"effort,omitempty"`
	Parent      string `json:"parent,omitempty"`
	Author      string `json:"author,omitempty"`
}

// 🏁️GoalClosePayload payload for goal.close.
type GoalClosePayload struct {
	GoalPayload
	Summary string `json:"summary"`
	Author  string `json:"author,omitempty"`
}

// 🔄️GoalReopenPayload payload for goal.reopen.
type GoalReopenPayload struct {
	GoalPayload
	Prompt string `json:"prompt"`
	Client string `json:"client"`
	LLM    string `json:"llm,omitempty"`
	Effort string `json:"effort,omitempty"`
	Author string `json:"author,omitempty"`
}

// 📐️GoalChangePayload payload for goal.change.
type GoalChangePayload struct {
	GoalPayload
	Title       *string `json:"title,omitempty"`
	Description *string `json:"description,omitempty"`
	LLM         *string `json:"llm,omitempty"`
	Effort      *string `json:"effort,omitempty"`
	Parent      *string `json:"parent,omitempty"`
	Author      string  `json:"author,omitempty"`
}

// 👥️ContributorPayload holds contributor identifiers.
type ContributorPayload struct {
	Github string `json:"github"`
	Author string `json:"author,omitempty"`
}

// 💾️CheckpointPayload payload for checkpoint (GitHub push).
type CheckpointPayload struct {
	Author       string   `json:"author"`
	Github       string   `json:"github"`
	Sha          string   `json:"sha"`
	Message      string   `json:"message"`
	Files        []string `json:"files"`
	Technologies []string `json:"technologies,omitempty"`
	Bundles      []string `json:"bundles,omitempty"`
	Folders      []string `json:"folders,omitempty"`
	FilesChanged []string `json:"files_changed,omitempty"`
	Sections     []string `json:"sections,omitempty"`
	Definitions  []string `json:"definitions,omitempty"`
}

// ✅️TodoPayload holds todo identifiers.
type TodoPayload struct {
	ID       string `json:"id"`
	ParentID string `json:"parent_id,omitempty"`
	Name     string `json:"name,omitempty"`
	Author   string `json:"author,omitempty"`
}

// 🆕️TodoCreatePayload payload for todo.create.
type TodoCreatePayload struct {
	TodoPayload
}

// ✏️TodoChangePayload payload for todo.change.
type TodoChangePayload struct {
	TodoPayload
	Name        *string `json:"name,omitempty"`
	Description *string `json:"description,omitempty"`
}

// 🗑️TodoDeletePayload payload for todo.delete.
type TodoDeletePayload struct {
	TodoPayload
}

// 💼️WorkItem represents a single item a contributor is working on (technology, bundle, folder, file, section, definition, ticket, goal, todo).
type WorkItem struct {
	Kind string `json:"kind"`
	ID   string `json:"id"`
}

// 🤝️ContributorWork holds all work items for one contributor.
type ContributorWork struct {
	Github       string   `json:"github"`
	Tickets      []string `json:"tickets"`
	Goals        []string `json:"goals"`
	Todos        []string `json:"todos"`
	Technologies []string `json:"technologies"`
	Bundles      []string `json:"bundles"`
	Folders      []string `json:"folders"`
	Files        []string `json:"files"`
	Sections     []string `json:"sections"`
	Definitions  []string `json:"definitions"`
}

// 📝️DraftPayload holds draft identifiers.
type DraftPayload struct {
	Slug   string `json:"slug"`
	Title  string `json:"title,omitempty"`
	Author string `json:"author,omitempty"`
}

// 📄️FilePayload holds file operation identifiers.
type FilePayload struct {
	Path   string `json:"path"`
	From   string `json:"from,omitempty"`
	Author string `json:"author,omitempty"`
}

// 📁️FolderPayload holds folder operation identifiers.
type FolderPayload struct {
	Path   string `json:"path"`
	From   string `json:"from,omitempty"`
	Author string `json:"author,omitempty"`
}

// 📑️SectionPayload holds section operation identifiers.
type SectionPayload struct {
	File    string `json:"file"`
	Name    string `json:"name"`
	OldName string `json:"old_name,omitempty"`
	Parent  string `json:"parent,omitempty"`
	Author  string `json:"author,omitempty"`
}

// 🧬️IntegratePayload holds integrate operation identifiers.
type IntegratePayload struct {
	Source        string `json:"source"`
	TargetFile    string `json:"target_file"`
	TargetSection string `json:"target_section"`
	Author        string `json:"author,omitempty"`
}

// 🧲️ExtractPayload holds extract operation identifiers.
type ExtractPayload struct {
	SourceFile    string `json:"source_file"`
	SourceSection string `json:"source_section"`
	TargetFile    string `json:"target_file"`
	Author        string `json:"author,omitempty"`
}

// #endregion 🌨️Payloads
