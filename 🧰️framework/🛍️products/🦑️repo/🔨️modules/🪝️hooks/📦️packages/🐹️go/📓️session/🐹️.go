// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📓️session carries session event logging and the memory and directory session stores.

// #endregion 🧲️Header

package hooks

import (
	json "encoding/json"
	fmt "fmt"
	os "os"
	filepath "path/filepath"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 📓️SessionLogging

// 📍️SessionEventEntry is one recorded hook invocation in a session log.
type SessionEventEntry struct {
	Event    json.RawMessage        `json:"event"`
	Native   *SessionNativeEnvelope `json:"native,omitempty"`
	Response *SessionResponse       `json:"response,omitempty"`
}

// 🧾️SessionNativeEnvelope is the native payload, kept only at `full` detail.
type SessionNativeEnvelope struct {
	Event json.RawMessage `json:"event"`
}

// 📤️SessionResponse is what the hook answered, kept at `standard` and `full` detail.
type SessionResponse struct {
	Blocked *bool  `json:"blocked,omitempty"`
	Message string `json:"message,omitempty"`
	Reason  string `json:"reason,omitempty"`
}

// 🔸️SessionMeta is the `session.json` record of one agent session.
type SessionMeta struct {
	ID          string                 `json:"id"`
	URI         string                 `json:"uri,omitempty"`
	Client      string                 `json:"client,omitempty"`
	Second      string                 `json:"second,omitempty"`
	Checkpoint  string                 `json:"checkpoint,omitempty"`
	Contributor string                 `json:"contributor,omitempty"`
	Transcript  string                 `json:"transcript,omitempty"`
	Events      []SessionEventEntry    `json:"events,omitempty"`
	Plan        *model.TicketAgentPlan `json:"plan,omitempty"`
}

// 💾️SessionStore is where a session log lives. The filesystem implementation is one of several: a
// caller under test hands in a memory store and never touches a disk.
type SessionStore interface {
	Read(sessionID string) (SessionMeta, bool)
	Write(sessionID string, meta SessionMeta) error
}

// 🧠️MemorySessionStore is an in-memory session store — the deterministic default for tests and dry runs.
type MemorySessionStore struct {
	sessions map[string]SessionMeta
}

// 🆕️NewMemorySessionStore builds an empty store.
func NewMemorySessionStore() *MemorySessionStore {
	return &MemorySessionStore{sessions: map[string]SessionMeta{}}
}

// 📥️Read answers the recorded session, when one exists.
func (s *MemorySessionStore) Read(sessionID string) (SessionMeta, bool) {
	meta, present := s.sessions[sessionID]
	return meta, present
}

// 📤️Write replaces the recorded session.
func (s *MemorySessionStore) Write(sessionID string, meta SessionMeta) error {
	if s.sessions == nil {
		s.sessions = map[string]SessionMeta{}
	}
	s.sessions[sessionID] = meta
	return nil
}

// 📋️Sessions lists every recorded session, by id.
func (s *MemorySessionStore) Sessions() map[string]SessionMeta { return s.sessions }

// 💽️DirectorySessionStore is a session store rooted at one directory, writing `<root>/<session id>/session.json`.
type DirectorySessionStore struct {
	Root string
}

// 🆕️NewDirectorySessionStore builds a store under root.
func NewDirectorySessionStore(root string) *DirectorySessionStore {
	return &DirectorySessionStore{Root: root}
}

// 📄️Path answers the file one session is recorded in.
func (s *DirectorySessionStore) Path(sessionID string) string {
	return filepath.Join(s.Root, sessionID, SessionFileName)
}

// 📥️Read answers the recorded session, when one exists.
func (s *DirectorySessionStore) Read(sessionID string) (SessionMeta, bool) {
	data, err := os.ReadFile(s.Path(sessionID))
	if err != nil {
		return SessionMeta{}, false
	}
	var meta SessionMeta
	if err := json.Unmarshal(data, &meta); err != nil {
		return SessionMeta{}, false
	}
	return meta, true
}

// 📤️Write replaces the recorded session.
func (s *DirectorySessionStore) Write(sessionID string, meta SessionMeta) error {
	path := s.Path(sessionID)
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return NewHookError(fmt.Sprintf("cannot create %s: %v", filepath.Dir(path), err))
	}
	return workspace.WriteJSONFile(path, meta)
}

// 📄️SessionFileName is the file name a session log is recorded in.
const SessionFileName = "session.json"

// 📁️SessionLogDir answers the directory a session log lives in.
func SessionLogDir(repoRoot string, year int, month int, day int, sessionID string) string {
	return filepath.Join(workspace.RepoMetaDirForRoot(repoRoot), "⚡️cache", "🤖️generated", model.FormatYearDir(year%100), model.FormatMonthDir(month), model.FormatDayDir(day), sessionID)
}

// 🪪️ResolveLogSessionID answers the session id a log is filed under: the payload's, `kiro-<pid>` for
// the Kiro CLI, otherwise the literal `unknown`.
func ResolveLogSessionID(ctx HookContext, kiroParentPID int) string {
	if session := ExtractSessionID(ctx.Input); session != "" {
		return session
	}
	if ctx.Client == "kiro-cli" && kiroParentPID > 0 {
		return fmt.Sprintf("kiro-%d", kiroParentPID)
	}
	return "unknown"
}

// 📓️RecordSessionHook appends one hook invocation to a session log, creating the record on first
// sight. Nothing is written for a `version.*` event or when `[logging] session` is off.
func RecordSessionHook(store SessionStore, ctx HookContext, result HookResult, sessionID string, logging workspace.LoggingConfig, environment HookEnvironment) (*SessionMeta, error) {
	event, known := ctx.ResolvedEvent()
	if !known {
		return nil, nil
	}
	if HookEventKind(event) == model.HookKindVersion || !logging.Session {
		return nil, nil
	}
	input := ctx.Input
	meta, _ := store.Read(sessionID)
	second := FirstNonEmpty(ctx.Second, FindNestedString(input, "second"))
	checkpoint := ExtractCheckpointID(input, environment)
	transcript := ExtractTranscript(input)
	if meta.ID == "" {
		meta.ID = sessionID
		meta.URI = "repo://session/" + sessionID
		meta.Client = ctx.Client
		meta.Second = second
		meta.Checkpoint = checkpoint
		meta.Contributor = "unknown"
		meta.Transcript = transcript
	} else {
		if meta.Client == "" {
			meta.Client = ctx.Client
		}
		if meta.Second == "" {
			meta.Second = second
		}
		if meta.Checkpoint == "" {
			meta.Checkpoint = checkpoint
		}
		if meta.Transcript == "" {
			meta.Transcript = transcript
		}
	}
	neutral := map[string]interface{}{
		"kind":        string(event),
		"client":      ctx.Client,
		"session":     sessionID,
		"second":      second,
		"checkpoint":  checkpoint,
		"contributor": "unknown",
	}
	if transcript != "" {
		neutral["transcript"] = transcript
	}
	neutralJSON, err := json.Marshal(neutral)
	if err != nil {
		return nil, NewHookError(err.Error())
	}
	entry := SessionEventEntry{Event: neutralJSON}
	if logging.IncludeNative() && len(input) > 0 {
		entry.Native = &SessionNativeEnvelope{Event: append(json.RawMessage(nil), input...)}
	}
	if logging.IncludeResponse() {
		entry.Response = &SessionResponse{}
		if !result.Allowed {
			blocked := true
			entry.Response.Blocked = &blocked
			entry.Response.Message = result.Message
			entry.Response.Reason = result.Message
		}
	}
	meta.Events = append(meta.Events, entry)
	if logging.Plan && event == model.HookAgentToolPlanUpdatingEnded {
		if steps := ExtractPlanSteps(input, ctx.ToolArgs); len(steps) > 0 {
			var existing []model.TicketAgentPlanStep
			if meta.Plan != nil {
				existing = meta.Plan.Steps
			}
			meta.Plan = &model.TicketAgentPlan{Steps: MergePlanSteps(existing, steps, second)}
		}
	}
	if err := store.Write(sessionID, meta); err != nil {
		return nil, err
	}
	return &meta, nil
}

// #endregion 📓️SessionLogging
