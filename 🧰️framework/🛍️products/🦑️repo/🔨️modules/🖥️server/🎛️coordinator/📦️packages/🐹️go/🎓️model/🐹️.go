// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📋️model carries the coordinator data model: tickets, scopes, warnings, breaches, events and the API request and response payloads.

// #endregion 🧲️Header

package coordinator

import (
	"time"
)

// #region 📋️Model

// Data model types for tickets, scopes, warnings, breachs, events, and API request/response payloads. MUST mirror the owned event schema.

// 🎫️Ticket represents a tracked work item with lifecycle status.
type Ticket struct {
	ID        string     `json:"id"`
	Status    string     `json:"status"`
	Title     string     `json:"title"`
	Emoji     string     `json:"emoji"`
	Prompt    string     `json:"prompt"`
	Summary   string     `json:"summary"`
	LLM       string     `json:"llm"`
	Client    string     `json:"client"`
	Author    string     `json:"author"`
	GitHub    string     `json:"github_issue"`
	CreatedAt time.Time  `json:"created_at"`
	ClosedAt  *time.Time `json:"closed_at"`
}

// 📖️Scope represents a code region (file, section, or definition) with line range.
type Scope struct {
	ID          string    `json:"id"`
	Kind        string    `json:"kind"`
	FilePath    string    `json:"file_path"`
	SectionPath string    `json:"section_path"`
	Definition  string    `json:"definition_name"`
	StartLine   int       `json:"start_line"`
	EndLine     int       `json:"end_line"`
	UpdatedAt   time.Time `json:"updated_at"`
}

// 🔭️Warning represents a detected issue such as a scope conflict between tickets.
type Warning struct {
	ID             string     `json:"id"`
	Kind           string     `json:"kind"`
	Severity       string     `json:"severity"`
	Message        string     `json:"message"`
	TicketID       string     `json:"ticket_id"`
	ScopeID        string     `json:"scope_id"`
	CreatedAt      time.Time  `json:"created_at"`
	Acknowledged   *time.Time `json:"acknowledged_at"`
	AcknowledgedBy string     `json:"ack_by"`
}

// 📜️Breach represents a policy breach detected in source code.
type Breach struct {
	ID         string     `json:"id"`
	Kind       string     `json:"kind"`
	Priority   string     `json:"priority"`
	ScopeID    string     `json:"scope_id"`
	FilePath   string     `json:"file_path"`
	Line       *int       `json:"line"`
	Column     *int       `json:"column"`
	Summary    string     `json:"summary"`
	Excerpt    string     `json:"excerpt"`
	Autofix    bool       `json:"autofixable"`
	DetectedAt time.Time  `json:"detected_at"`
	TicketID   string     `json:"ticket_id"`
	ResolvedAt *time.Time `json:"resolved_at"`
}

// 📡️Event represents a system event persisted to the event log.
type Event struct {
	ID        string    `json:"id"`
	Type      string    `json:"type"`
	Source    string    `json:"source"`
	Payload   string    `json:"payload_json"`
	CreatedAt time.Time `json:"created_at"`
}

// 🔢️LineRange represents a contiguous range of line numbers.
type LineRange struct {
	Start int
	End   int
}

// 🔷️DiffHunk represents a single hunk with old and new line ranges from a unified diff.
type DiffHunk struct {
	OldRange LineRange
	NewRange LineRange
}

// 📍️DiffFile represents a single file entry in a unified diff with its hunks.
type DiffFile struct {
	Path    string
	Hunks   []DiffHunk
	Deleted bool
	Created bool
}

// 🔬️DiffResult aggregates all parsed diff files from a patch.
type DiffResult struct {
	Files []DiffFile
}

// 📸️FileSnapshot holds the full content of a file for snapshot-based indexing.
type FileSnapshot struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

// 📦️TicketOpenRequest is the JSON payload for opening a new ticket.
type TicketOpenRequest struct {
	TicketID    string `json:"ticket_id"`
	Title       string `json:"title"`
	Prompt      string `json:"prompt"`
	LLM         string `json:"llm"`
	Client      string `json:"client"`
	Author      string `json:"author"`
	GitHubIssue string `json:"github_issue"`
}

// 📨️TicketCloseRequest is the JSON payload for closing a ticket.
type TicketCloseRequest struct {
	TicketID string   `json:"ticket_id"`
	Summary  string   `json:"summary"`
	Files    []string `json:"files"`
}

// 🔓️TicketReopenRequest is the JSON payload for reopening a closed ticket.
type TicketReopenRequest struct {
	TicketID string `json:"ticket_id"`
	Prompt   string `json:"prompt"`
	LLM      string `json:"llm"`
	Title    string `json:"title"`
}

// 📋️DiffIngestRequest is the JSON payload for ingesting a diff patch.
type DiffIngestRequest struct {
	TicketID  string         `json:"ticket_id"`
	RepoID    string         `json:"repo_id"`
	Patch     string         `json:"patch"`
	Snapshots []FileSnapshot `json:"snapshots"`
}

// 📩️DiffIngestResponse holds the results of a diff ingestion operation.
type DiffIngestResponse struct {
	ChangedFiles  []string  `json:"changed_files"`
	ClaimedScopes []string  `json:"claimed_scopes"`
	Warnings      []Warning `json:"warnings"`
	Breachs       []Breach  `json:"breachs"`
	Blockers      []string  `json:"blockers"`
}

// 📄️IndexFileRequest is the JSON payload for indexing a single file.
type IndexFileRequest struct {
	FilePath string `json:"file_path"`
	Content  string `json:"content"`
}

// #endregion 📋️Model
