// #region 🧲️Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// GraphQL server for the monorepo management API.

// #endregion 🧲️Header

// #region 🔑️Package
// 🗜️Package declaration for the repo server binary. MUST be package main.
package main

// #endregion 🔑️Package

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"math/rand"
	"net/http"
	"os"
	"strconv"
	"strings"
	"sync"
	"time"

	repopkg "github.com/usalu/semio/repo/go"
)

// #region 🧩️Component

// Server configuration loading from environment variables. MUST provide sensible defaults.

// ⚙️Config holds all server configuration values.
type Config struct {
	Address          string
	DatabasePath     string
	RepoRoot         string
	Token            string
	GitHubSecret     string
	DiscordWebhook   string
	RequestBodyLimit int64
}

// 🖥️loadConfig reads server configuration from environment variables with fallback defaults.
func loadConfig() Config {
	cwd, _ := os.Getwd()
	return Config{
		Address:          envOrDefault("COMPOSE_SERVER_ADDR", "127.0.0.1:8787"),
		DatabasePath:     envOrDefault("COMPOSE_SERVER_DB", "compose-server.db"),
		RepoRoot:         envOrDefault("COMPOSE_SERVER_REPO_ROOT", cwd),
		Token:            envOrDefault("COMPOSE_SERVER_TOKEN", ""),
		GitHubSecret:     envOrDefault("COMPOSE_SERVER_GITHUB_SECRET", ""),
		DiscordWebhook:   envOrDefault("COMPOSE_SERVER_DISCORD_WEBHOOK", ""),
		RequestBodyLimit: envOrDefaultInt64("COMPOSE_SERVER_BODY_LIMIT", 10*1024*1024),
	}
}

// 📦️envOrDefault returns the environment variable value or the fallback if empty.
func envOrDefault(key, fallback string) string {
	if value := strings.TrimSpace(os.Getenv(key)); value != "" {
		return value
	}
	return fallback
}

// 🔬️envOrDefaultInt64 returns the parsed int64 environment variable or the fallback.
func envOrDefaultInt64(key string, fallback int64) int64 {
	if value := strings.TrimSpace(os.Getenv(key)); value != "" {
		if parsed, err := strconv.ParseInt(value, 10, 64); err == nil {
			return parsed
		}
	}
	return fallback
}

// Data model types for tickets, scopes, warnings, breachs, events, and API request/response payloads. MUST mirror the owned event schema.

// 🎫️Ticket represents a tracked work item with lifecycle status.
type Ticket struct {
	ID        string     `json:"id"`
	Status    string     `json:"status"`
	Title     string     `json:"title"`
	Prompt    string     `json:"prompt"`
	Summary   string     `json:"summary"`
	LLM       string     `json:"llm"`
	Client    string     `json:"client"`
	Author    string     `json:"author"`
	GitHub    string     `json:"github_issue"`
	Goal      string     `json:"goal"`
	Parent    *string    `json:"parent"`
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
	Col        *int       `json:"col"`
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
	Kind      string    `json:"kind"`
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
	Action      string  `json:"action"`
	TicketID    string  `json:"ticket_id"`
	Title       string  `json:"title"`
	Prompt      string  `json:"prompt"`
	LLM         string  `json:"llm"`
	Client      string  `json:"client"`
	Author      string  `json:"author"`
	GitHubIssue string  `json:"github_issue"`
	Goal        string  `json:"goal"`
	Parent      *string `json:"parent"`
}

// 📨️TicketCloseRequest is the JSON payload for closing a ticket.
type TicketCloseRequest struct {
	Action   string   `json:"action"`
	TicketID string   `json:"ticket_id"`
	Summary  string   `json:"summary"`
	Files    []string `json:"files"`
}

// 🔓️TicketReopenRequest is the JSON payload for reopening a closed ticket.
type TicketReopenRequest struct {
	Action   string `json:"action"`
	TicketID string `json:"ticket_id"`
	Prompt   string `json:"prompt"`
	LLM      string `json:"llm"`
	Title    string `json:"title"`
	Client   string `json:"client"`
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
	Action   string `json:"action"`
	FilePath string `json:"file_path"`
	Content  string `json:"content"`
}

// HTTP server with ticket lifecycle, diff ingestion, indexing, and webhook endpoints. MUST enforce authentication on mutating routes.

// 🗄️Server is the main HTTP server holding configuration, database, event bus, and caches.
type Server struct {
	config      Config
	db          CoordinatorRepository
	bus         *EventBus
	logger      *log.Logger
	cache       IndexCache
	cacheLock   sync.RWMutex
	githubCache map[string]GitHubComment
	ghLock      sync.Mutex
}

// ⚙️NewServer creates a new Server with the given config, database, and event bus.
// 💾️MUST initialize the index cache and GitHub comment cache.
func NewServer(config Config, db CoordinatorRepository, bus *EventBus) *Server {
	return &Server{
		config:      config,
		db:          db,
		bus:         bus,
		logger:      log.New(os.Stdout, "", log.LstdFlags),
		cache:       newIndexCache(),
		githubCache: map[string]GitHubComment{},
	}
}

// 📨️newRequestContext creates a request-scoped context with a 15-second timeout.
func (s *Server) newRequestContext(r *http.Request) (context.Context, context.CancelFunc) {
	return context.WithTimeout(r.Context(), 15*time.Second)
}

// 🖥️requireAuth checks the bearer token against the configured server token.
func (s *Server) requireAuth(r *http.Request) bool {
	if s.config.Token == "" {
		return true
	}
	auth := r.Header.Get("Authorization")
	if auth == "" {
		return false
	}
	parts := strings.SplitN(auth, " ", 2)
	if len(parts) != 2 || parts[0] != "Bearer" {
		return false
	}
	return parts[1] == s.config.Token
}

// 📋️decodeJSON reads and decodes a JSON request body with size limits.
func (s *Server) decodeJSON(r *http.Request, payload interface{}) error {
	decoder := json.NewDecoder(io.LimitReader(r.Body, s.config.RequestBodyLimit))
	decoder.DisallowUnknownFields()
	return decoder.Decode(payload)
}

// 📩️writeJSON writes a JSON response with the given status code.
func (s *Server) writeJSON(w http.ResponseWriter, status int, payload interface{}) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	_ = json.NewEncoder(w).Encode(payload)
}

// ❌️respondError writes a JSON error response.
func (s *Server) respondError(w http.ResponseWriter, status int, message string) {
	s.writeJSON(w, status, map[string]string{"error": message})
}

// 📦️handleEvents accepts CLI event payloads and persists/publishes them.
func (s *Server) handleEvents(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	var ev repopkg.Event
	if err := s.decodeJSON(r, &ev); err != nil {
		s.respondError(w, http.StatusBadRequest, err.Error())
		return
	}
	if ev.Kind == "" {
		s.respondError(w, http.StatusBadRequest, "kind required")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	var payload interface{}
	if err := json.Unmarshal(ev.Payload, &payload); err != nil {
		payload = map[string]string{"raw": string(ev.Payload)}
	}
	if err := s.bus.Publish(ctx, string(ev.Kind), ev.Source, payload); err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

// ✔️handleHealth responds with 200 OK for liveness checks.
func (s *Server) handleHealth(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte("ok"))
}

// 💿️newID holds the data fields for a newID record.
func newID() string {
	return fmt.Sprintf("%d-%d", time.Now().UTC().UnixNano(), rand.Int63())
}

// 🔷️main holds the data fields for a main record.
func main() {
	config := loadConfig()
	db, err := openDatabase(config.DatabasePath)
	if err != nil {
		log.Fatal(err)
	}
	bus := NewEventBus(db)
	server := NewServer(config, db, bus)
	server.registerNotifications()
	bus.Start()
	defer bus.Stop()
	mux := http.NewServeMux()
	mux.HandleFunc("/healthz", server.handleHealth)
	mux.HandleFunc("/ticket/open", server.handleTicketOpen)
	mux.HandleFunc("/ticket/close", server.handleTicketClose)
	mux.HandleFunc("/ticket/reopen", server.handleTicketReopen)
	mux.HandleFunc("/tickets", server.handleTicketsQuery)
	mux.HandleFunc("/ticket/", server.handleTicketDetail)
	mux.HandleFunc("/diff/ingest", server.handleDiffIngest)
	mux.HandleFunc("/repo/reindex", server.handleReindex)
	mux.HandleFunc("/repo/index-file", server.handleIndexFile)
	mux.HandleFunc("/warnings", server.handleWarnings)
	mux.HandleFunc("/breachs", server.handleBreachs)
	mux.HandleFunc("/scopes", server.handleScopes)
	mux.HandleFunc("/events", server.handleEvents)
	mux.HandleFunc("/webhooks/github", server.handleGitHubWebhook)
	log.Printf("repo server listening on %s", config.Address)
	if err := http.ListenAndServe(config.Address, mux); err != nil {
		log.Fatal(err)
	}
}

// #endregion 🧩️Component
