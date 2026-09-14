// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// 📡️ Repo events: kinds, envelope, payloads, coordinator emit and the append-only event store.
// #endregion 🧲️Header

// 📡️ Package events owns every changing interaction of the repo domain.
//
// Kind strings are declared once in ../../🧬️schema/🔣️event-kinds.json and mirrored by the
// constants below; [LoadKindCatalog] reads that file and 🧪️.go asserts the two agree.
package events

import (
	bufio "bufio"
	bytes "bytes"
	context "context"
	sha256 "crypto/sha256"
	hex "encoding/hex"
	json "encoding/json"
	errors "errors"
	fmt "fmt"
	io "io"
	http "net/http"
	os "os"
	filepath "path/filepath"
	runtime "runtime"
	strings "strings"
	sync "sync"
	time "time"
)

// #region 📋️EventKind

// 📡️EventKind identifies a changing interaction. CLI emits; server subscribes and notifies.
type EventKind string

const (
	EventTicketOpenStarting        EventKind = "ticket.open.starting"
	EventTicketOpenEnded           EventKind = "ticket.open.ended"
	EventTicketCloseStarting       EventKind = "ticket.close.starting"
	EventTicketCloseEnded          EventKind = "ticket.close.ended"
	EventTicketReopenStarting      EventKind = "ticket.reopen.starting"
	EventTicketReopenEnded         EventKind = "ticket.reopen.ended"
	EventTicketChangeStarting      EventKind = "ticket.change.starting"
	EventTicketChangeEnded         EventKind = "ticket.change.ended"
	EventTicketReadStarting        EventKind = "ticket.read.starting"
	EventTicketReadEnded           EventKind = "ticket.read.ended"
	EventGoalOpenStarting          EventKind = "goal.open.starting"
	EventGoalOpenEnded             EventKind = "goal.open.ended"
	EventGoalCloseStarting         EventKind = "goal.close.starting"
	EventGoalCloseEnded            EventKind = "goal.close.ended"
	EventGoalReopenStarting        EventKind = "goal.reopen.starting"
	EventGoalReopenEnded           EventKind = "goal.reopen.ended"
	EventGoalChangeStarting        EventKind = "goal.change.starting"
	EventGoalChangeEnded           EventKind = "goal.change.ended"
	EventContributorAddStarting    EventKind = "contributor.add.starting"
	EventContributorAddEnded       EventKind = "contributor.add.ended"
	EventContributorRemoveStarting EventKind = "contributor.remove.starting"
	EventContributorRemoveEnded    EventKind = "contributor.remove.ended"
	EventCheckpointStarting        EventKind = "checkpoint.starting"
	EventCheckpointEnded           EventKind = "checkpoint.ended"
	EventTodoCreateStarting        EventKind = "todo.create.starting"
	EventTodoCreateEnded           EventKind = "todo.create.ended"
	EventTodoChangeStarting        EventKind = "todo.change.starting"
	EventTodoChangeEnded           EventKind = "todo.change.ended"
	EventTodoDeleteStarting        EventKind = "todo.delete.starting"
	EventTodoDeleteEnded           EventKind = "todo.delete.ended"
	EventDraftCreateStarting       EventKind = "draft.create.starting"
	EventDraftCreateEnded          EventKind = "draft.create.ended"
	EventDraftDeleteStarting       EventKind = "draft.delete.starting"
	EventDraftDeleteEnded          EventKind = "draft.delete.ended"
	EventFileCreateStarting        EventKind = "file.create.starting"
	EventFileCreateEnded           EventKind = "file.create.ended"
	EventFileMoveStarting          EventKind = "file.move.starting"
	EventFileMoveEnded             EventKind = "file.move.ended"
	EventFileDeleteStarting        EventKind = "file.delete.starting"
	EventFileDeleteEnded           EventKind = "file.delete.ended"
	EventFolderCreateStarting      EventKind = "folder.create.starting"
	EventFolderCreateEnded         EventKind = "folder.create.ended"
	EventFolderMoveStarting        EventKind = "folder.move.starting"
	EventFolderMoveEnded           EventKind = "folder.move.ended"
	EventFolderDeleteStarting      EventKind = "folder.delete.starting"
	EventFolderDeleteEnded         EventKind = "folder.delete.ended"
	EventSectionCreateStarting     EventKind = "section.create.starting"
	EventSectionCreateEnded        EventKind = "section.create.ended"
	EventSectionMoveStarting       EventKind = "section.move.starting"
	EventSectionMoveEnded          EventKind = "section.move.ended"
	EventSectionDeleteStarting     EventKind = "section.delete.starting"
	EventSectionDeleteEnded        EventKind = "section.delete.ended"
	EventIntegrateStarting         EventKind = "integrate.starting"
	EventIntegrateEnded            EventKind = "integrate.ended"
	EventExtractStarting           EventKind = "extract.starting"
	EventExtractEnded              EventKind = "extract.ended"
	EventExportStarting            EventKind = "export.starting"
	EventExportEnded               EventKind = "export.ended"
	EventAnalyzeStarting           EventKind = "analyze.starting"
	EventAnalyzeEnded              EventKind = "analyze.ended"
	EventFixStarting               EventKind = "fix.starting"
	EventFixEnded                  EventKind = "fix.ended"
	EventTreeStarting              EventKind = "tree.starting"
	EventTreeEnded                 EventKind = "tree.ended"
	EventGraphqlStarting           EventKind = "graphql.starting"
	EventGraphqlEnded              EventKind = "graphql.ended"
	EventMoveStarting              EventKind = "move.starting"
	EventMoveEnded                 EventKind = "move.ended"
	EventPolicyCheckStarting       EventKind = "policy.check.starting"
	EventPolicyCheckEnded          EventKind = "policy.check.ended"
)

// 🗂️AllEventKinds is the ordered catalog of every declared kind.
func AllEventKinds() []EventKind {
	return []EventKind{
		EventTicketOpenStarting, EventTicketOpenEnded,
		EventTicketCloseStarting, EventTicketCloseEnded,
		EventTicketReopenStarting, EventTicketReopenEnded,
		EventTicketChangeStarting, EventTicketChangeEnded,
		EventTicketReadStarting, EventTicketReadEnded,
		EventGoalOpenStarting, EventGoalOpenEnded,
		EventGoalCloseStarting, EventGoalCloseEnded,
		EventGoalReopenStarting, EventGoalReopenEnded,
		EventGoalChangeStarting, EventGoalChangeEnded,
		EventContributorAddStarting, EventContributorAddEnded,
		EventContributorRemoveStarting, EventContributorRemoveEnded,
		EventCheckpointStarting, EventCheckpointEnded,
		EventTodoCreateStarting, EventTodoCreateEnded,
		EventTodoChangeStarting, EventTodoChangeEnded,
		EventTodoDeleteStarting, EventTodoDeleteEnded,
		EventDraftCreateStarting, EventDraftCreateEnded,
		EventDraftDeleteStarting, EventDraftDeleteEnded,
		EventFileCreateStarting, EventFileCreateEnded,
		EventFileMoveStarting, EventFileMoveEnded,
		EventFileDeleteStarting, EventFileDeleteEnded,
		EventFolderCreateStarting, EventFolderCreateEnded,
		EventFolderMoveStarting, EventFolderMoveEnded,
		EventFolderDeleteStarting, EventFolderDeleteEnded,
		EventSectionCreateStarting, EventSectionCreateEnded,
		EventSectionMoveStarting, EventSectionMoveEnded,
		EventSectionDeleteStarting, EventSectionDeleteEnded,
		EventIntegrateStarting, EventIntegrateEnded,
		EventExtractStarting, EventExtractEnded,
		EventExportStarting, EventExportEnded,
		EventAnalyzeStarting, EventAnalyzeEnded,
		EventFixStarting, EventFixEnded,
		EventTreeStarting, EventTreeEnded,
		EventGraphqlStarting, EventGraphqlEnded,
		EventMoveStarting, EventMoveEnded,
		EventPolicyCheckStarting, EventPolicyCheckEnded,
	}
}

// 🔣️KindCatalogEntry is one row of ../../🧬️schema/🔣️event-kinds.json.
type KindCatalogEntry struct {
	Constant string    `json:"constant"`
	Kind     EventKind `json:"kind"`
}

// 📇️KindCatalog is the parsed schema-side kind list.
type KindCatalog struct {
	SchemaVersion int                `json:"schemaVersion"`
	Kinds         []KindCatalogEntry `json:"kinds"`
}

var (
	kindCatalogOnce  sync.Once
	kindCatalogValue KindCatalog
	kindCatalogError error
)

// 📍️KindCatalogPath resolves ../../🧬️schema/🔣️event-kinds.json relative to this source file.
func KindCatalogPath() string {
	_, self, _, ok := runtime.Caller(0)
	if !ok {
		return ""
	}
	return filepath.Join(filepath.Dir(self), "..", "..", "🧬️schema", "🔣️event-kinds.json")
}

// 📥️LoadKindCatalog reads the schema-side kind list once per process.
func LoadKindCatalog() (KindCatalog, error) {
	kindCatalogOnce.Do(func() {
		path := KindCatalogPath()
		if path == "" {
			kindCatalogError = errors.New("kind catalog path is unresolvable")
			return
		}
		data, err := os.ReadFile(path)
		if err != nil {
			kindCatalogError = err
			return
		}
		kindCatalogError = json.Unmarshal(data, &kindCatalogValue)
	})
	return kindCatalogValue, kindCatalogError
}

// #endregion 📋️EventKind

// #region ✉️Envelope

// ✉️Event is the canonical envelope for a changing interaction sent from CLI to server.
type Event struct {
	Kind    EventKind       `json:"kind"`
	Source  string          `json:"source"`
	Payload json.RawMessage `json:"payload"`
}

// #endregion ✉️Envelope

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

// #region 📤️Emit

// 🛰️Emitter delivers an envelope to the coordinator.
type Emitter interface {
	Emit(kind EventKind, source string, payload interface{})
}

// 🌐️HTTPEmitter posts envelopes to COMPOSE_SERVER_ADDR.
type HTTPEmitter struct{}

// 📤️Emit posts an event to the repo server. No-operation when COMPOSE_SERVER_ADDR is unset.
func (HTTPEmitter) Emit(kind EventKind, source string, payload interface{}) {
	Emit(kind, source, payload)
}

// 🔗️EmitURL renders the coordinator endpoint for an address, empty when the address is empty.
func EmitURL(addr string) string {
	addr = strings.TrimSpace(addr)
	if addr == "" {
		return ""
	}
	url := addr
	if !strings.HasPrefix(addr, "http://") && !strings.HasPrefix(addr, "https://") {
		url = "http://" + addr
	}
	return strings.TrimSuffix(url, "/") + "/api/v1/events"
}

// 📤️Emit posts an event to the repo server. No-operation when COMPOSE_SERVER_ADDR is unset.
func Emit(kind EventKind, source string, payload interface{}) {
	url := EmitURL(os.Getenv("COMPOSE_SERVER_ADDR"))
	if url == "" {
		return
	}
	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		return
	}
	body, err := json.Marshal(Event{Kind: kind, Source: source, Payload: payloadBytes})
	if err != nil {
		return
	}
	req, err := http.NewRequest(http.MethodPost, url, bytes.NewReader(body))
	if err != nil {
		return
	}
	req.Header.Set("Content-Type", "application/json")
	if token := strings.TrimSpace(os.Getenv("COMPOSE_SERVER_TOKEN")); token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}
	client := &http.Client{Timeout: 5 * time.Second}
	_, _ = client.Do(req)
}

// #endregion 📤️Emit

// #region 🗄️Store

const (
	Schema         = "semio.event/1"
	stageSchema    = "semio.event.stage/1"
	MaxEventSize   = 1 << 20
	MaxBatchSize   = 64 << 20
	MaxBatchEvents = 100_000
)

var (
	ErrDuplicate = errors.New("duplicate event")
	ErrCorrupt   = errors.New("corrupt event log")
	ErrTooLarge  = errors.New("event exceeds maximum size")
	storeLocks   sync.Map
)

// 🧾️StoreEvent is one committed record of the append-only log.
type StoreEvent struct {
	Schema   string          `json:"schema"`
	Sequence uint64          `json:"sequence"`
	ID       string          `json:"id"`
	Kind     string          `json:"kind"`
	Data     json.RawMessage `json:"data"`
	Checksum string          `json:"checksum"`
}

// 📨️Input is an uncommitted record offered to the log.
type Input struct {
	ID   string
	Kind string
	Data interface{}
}

// ⏳️Progress reports one step of an append or replay.
type Progress struct {
	Current int
	Total   int
	Step    string
}

// 🗄️Store is a recoverable append-only JSONL log with deterministic replay.
type Store struct{ Path string }

type stage struct {
	Schema        string `json:"schema"`
	PriorExists   bool   `json:"priorExists"`
	PriorSize     int64  `json:"priorSize"`
	PriorChecksum string `json:"priorChecksum"`
	BatchSize     int    `json:"batchSize"`
	BatchChecksum string `json:"batchChecksum"`
}

// ➕️Append stages, writes and syncs a batch, leaving the prior log untouched on any failure.
func (store Store) Append(ctx context.Context, inputs []Input, progress func(Progress)) ([]StoreEvent, error) {
	if store.Path == "" {
		return nil, errors.New("event log path is required")
	}
	lock := storeLock(store.Path)
	lock.Lock()
	defer lock.Unlock()
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	if err := store.recover(); err != nil {
		return nil, err
	}
	existing, err := store.replay(ctx, nil)
	if err != nil && !os.IsNotExist(err) {
		return nil, err
	}
	if len(inputs) > MaxBatchEvents {
		return nil, fmt.Errorf("%w: %d events > %d", ErrTooLarge, len(inputs), MaxBatchEvents)
	}
	seen := make(map[string]struct{}, len(existing)+len(inputs))
	for _, item := range existing {
		seen[item.ID] = struct{}{}
	}
	created := make([]StoreEvent, 0, len(inputs))
	var batch bytes.Buffer
	encoder := json.NewEncoder(&batch)
	for index, input := range inputs {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		if input.ID == "" || input.Kind == "" {
			return nil, errors.New("event id and kind are required")
		}
		if _, duplicate := seen[input.ID]; duplicate {
			return nil, fmt.Errorf("%w: %s", ErrDuplicate, input.ID)
		}
		data, err := json.Marshal(input.Data)
		if err != nil {
			return nil, err
		}
		if len(data) > MaxEventSize {
			return nil, fmt.Errorf("%w: %d > %d", ErrTooLarge, len(data), MaxEventSize)
		}
		event := StoreEvent{Schema: Schema, Sequence: uint64(len(existing) + index + 1), ID: input.ID, Kind: input.Kind, Data: data}
		event.Checksum = Checksum(event)
		if err := encoder.Encode(event); err != nil {
			return nil, err
		}
		if batch.Len() > MaxBatchSize {
			return nil, fmt.Errorf("%w: batch > %d", ErrTooLarge, MaxBatchSize)
		}
		created = append(created, event)
		seen[input.ID] = struct{}{}
		report(progress, Progress{Current: index + 1, Total: len(inputs), Step: "encoded"})
	}
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	if len(created) == 0 {
		return created, nil
	}
	if err := os.MkdirAll(filepath.Dir(store.Path), 0o755); err != nil {
		return nil, err
	}
	prior, err := inspectPrior(store.Path)
	if err != nil {
		return nil, err
	}
	staged := stage{
		Schema:        stageSchema,
		PriorExists:   prior.exists,
		PriorSize:     prior.size,
		PriorChecksum: prior.checksum,
		BatchSize:     batch.Len(),
		BatchChecksum: Digest(batch.Bytes()),
	}
	if err := store.writeStage(staged); err != nil {
		return nil, err
	}
	report(progress, Progress{Current: 0, Total: len(created), Step: "staged"})
	if err := ctx.Err(); err != nil {
		return nil, errors.Join(err, store.rollback(staged))
	}
	output, err := os.OpenFile(store.Path, os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0o644)
	if err != nil {
		return nil, errors.Join(err, store.rollback(staged))
	}
	written, writeErr := output.Write(batch.Bytes())
	if writeErr == nil && written != batch.Len() {
		writeErr = io.ErrShortWrite
	}
	report(progress, Progress{Current: written, Total: batch.Len(), Step: "appended"})
	if writeErr == nil {
		writeErr = ctx.Err()
	}
	if writeErr == nil {
		writeErr = output.Sync()
	}
	if writeErr == nil {
		report(progress, Progress{Current: len(created), Total: len(created), Step: "synced"})
		writeErr = ctx.Err()
	}
	closeErr := output.Close()
	if writeErr == nil {
		writeErr = closeErr
	}
	if writeErr != nil {
		return nil, errors.Join(writeErr, store.rollback(staged))
	}
	_ = os.Remove(store.stagePath())
	_ = syncDirectory(filepath.Dir(store.Path))
	report(progress, Progress{Current: len(created), Total: len(created), Step: "committed"})
	return created, nil
}

func (store Store) writeStage(value stage) error {
	next := store.stagePath() + ".next"
	_ = os.Remove(next)
	output, err := os.OpenFile(next, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0o600)
	if err != nil {
		return err
	}
	encoded, err := json.Marshal(value)
	if err == nil {
		_, err = output.Write(append(encoded, '\n'))
	}
	if err == nil {
		err = output.Sync()
	}
	closeErr := output.Close()
	if err == nil {
		err = closeErr
	}
	if err != nil {
		_ = os.Remove(next)
		return err
	}
	if err := os.Rename(next, store.stagePath()); err != nil {
		_ = os.Remove(next)
		return err
	}
	return syncDirectory(filepath.Dir(store.Path))
}

type priorState struct {
	exists   bool
	size     int64
	checksum string
}

func inspectPrior(path string) (priorState, error) {
	file, err := os.Open(path)
	if os.IsNotExist(err) {
		return priorState{checksum: Digest(nil)}, nil
	}
	if err != nil {
		return priorState{}, err
	}
	defer file.Close()
	info, err := file.Stat()
	if err != nil {
		return priorState{}, err
	}
	hash := sha256.New()
	if _, err := io.Copy(hash, file); err != nil {
		return priorState{}, err
	}
	return priorState{exists: true, size: info.Size(), checksum: hex.EncodeToString(hash.Sum(nil))}, nil
}

func (store Store) recover() error {
	_ = os.Remove(store.stagePath() + ".next")
	data, err := os.ReadFile(store.stagePath())
	if os.IsNotExist(err) {
		return nil
	}
	if err != nil {
		return err
	}
	var staged stage
	if err := json.Unmarshal(data, &staged); err != nil || staged.Schema != stageSchema || staged.PriorSize < 0 || staged.BatchSize <= 0 {
		return fmt.Errorf("%w: invalid append stage", ErrCorrupt)
	}
	file, err := os.OpenFile(store.Path, os.O_RDWR, 0o644)
	if os.IsNotExist(err) && !staged.PriorExists {
		return os.Remove(store.stagePath())
	}
	if err != nil {
		return fmt.Errorf("%w: staged log missing: %v", ErrCorrupt, err)
	}
	defer file.Close()
	info, err := file.Stat()
	if err != nil {
		return err
	}
	if info.Size() < staged.PriorSize || info.Size() > staged.PriorSize+int64(staged.BatchSize) {
		return fmt.Errorf("%w: staged append size", ErrCorrupt)
	}
	prefix := sha256.New()
	if _, err := io.CopyN(prefix, file, staged.PriorSize); err != nil && !errors.Is(err, io.EOF) {
		return err
	}
	if hex.EncodeToString(prefix.Sum(nil)) != staged.PriorChecksum {
		return fmt.Errorf("%w: staged append prefix", ErrCorrupt)
	}
	committed := false
	if info.Size() == staged.PriorSize+int64(staged.BatchSize) {
		segment := make([]byte, staged.BatchSize)
		if _, err := file.ReadAt(segment, staged.PriorSize); err != nil {
			return err
		}
		committed = Digest(segment) == staged.BatchChecksum
	}
	if !committed && info.Size() != staged.PriorSize {
		if err := file.Truncate(staged.PriorSize); err != nil {
			return err
		}
		if err := file.Sync(); err != nil {
			return err
		}
	}
	if !staged.PriorExists && staged.PriorSize == 0 && !committed {
		if err := file.Close(); err != nil {
			return err
		}
		if err := os.Remove(store.Path); err != nil && !os.IsNotExist(err) {
			return err
		}
	}
	if err := os.Remove(store.stagePath()); err != nil {
		return err
	}
	return syncDirectory(filepath.Dir(store.Path))
}

func (store Store) rollback(staged stage) error {
	file, err := os.OpenFile(store.Path, os.O_RDWR, 0o644)
	if os.IsNotExist(err) && !staged.PriorExists {
		_ = os.Remove(store.stagePath())
		return nil
	}
	if err != nil {
		return err
	}
	if err := file.Truncate(staged.PriorSize); err != nil {
		file.Close()
		return err
	}
	if err := file.Sync(); err != nil {
		file.Close()
		return err
	}
	if err := file.Close(); err != nil {
		return err
	}
	if !staged.PriorExists {
		if err := os.Remove(store.Path); err != nil && !os.IsNotExist(err) {
			return err
		}
	}
	if err := os.Remove(store.stagePath()); err != nil && !os.IsNotExist(err) {
		return err
	}
	return syncDirectory(filepath.Dir(store.Path))
}

func (store Store) stagePath() string { return store.Path + ".stage" }

func storeLock(path string) *sync.Mutex {
	value, _ := storeLocks.LoadOrStore(filepath.Clean(path), &sync.Mutex{})
	return value.(*sync.Mutex)
}

func syncDirectory(path string) error {
	if runtime.GOOS == "windows" {
		return nil
	}
	directory, err := os.Open(path)
	if err != nil {
		return err
	}
	defer directory.Close()
	return directory.Sync()
}

// ⏪️Replay recovers any interrupted append, then reads and validates the whole log.
func (store Store) Replay(ctx context.Context, progress func(Progress)) ([]StoreEvent, error) {
	if store.Path == "" {
		return nil, errors.New("event log path is required")
	}
	lock := storeLock(store.Path)
	lock.Lock()
	defer lock.Unlock()
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	if err := store.recover(); err != nil {
		return nil, err
	}
	return store.replay(ctx, progress)
}

func (store Store) replay(ctx context.Context, progress func(Progress)) ([]StoreEvent, error) {
	file, err := os.Open(store.Path)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	reader := bufio.NewReaderSize(file, 64*1024)
	var events []StoreEvent
	seen := map[string]struct{}{}
	for {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		line, readErr := reader.ReadBytes('\n')
		if len(line) > MaxEventSize*2 {
			return nil, fmt.Errorf("%w: encoded event too large", ErrCorrupt)
		}
		if len(line) > 0 {
			if errors.Is(readErr, io.EOF) && line[len(line)-1] != '\n' {
				return nil, fmt.Errorf("%w: incomplete event at sequence %d", ErrCorrupt, len(events)+1)
			}
			var event StoreEvent
			if err := json.Unmarshal(line, &event); err != nil {
				return nil, fmt.Errorf("%w at sequence %d: %v", ErrCorrupt, len(events)+1, err)
			}
			expected := uint64(len(events) + 1)
			if event.Schema != Schema || event.Sequence != expected || event.ID == "" || event.Kind == "" || event.Checksum != Checksum(event) {
				return nil, fmt.Errorf("%w at sequence %d", ErrCorrupt, expected)
			}
			if _, duplicate := seen[event.ID]; duplicate {
				return nil, fmt.Errorf("%w: %s", ErrDuplicate, event.ID)
			}
			seen[event.ID] = struct{}{}
			events = append(events, event)
			report(progress, Progress{Current: len(events), Step: "replayed"})
		}
		if errors.Is(readErr, io.EOF) {
			break
		}
		if readErr != nil {
			return nil, readErr
		}
	}
	return events, nil
}

// 🔏️Checksum is the sha256 identity of a record over schema, sequence, id, kind and data.
func Checksum(event StoreEvent) string {
	value := sha256.New()
	fmt.Fprintf(value, "%s\x00%d\x00%s\x00%s\x00", event.Schema, event.Sequence, event.ID, event.Kind)
	value.Write(event.Data)
	return hex.EncodeToString(value.Sum(nil))
}

// 🧮️Digest is the hex sha256 of raw bytes.
func Digest(data []byte) string {
	value := sha256.Sum256(data)
	return hex.EncodeToString(value[:])
}

func report(progress func(Progress), value Progress) {
	if progress != nil {
		progress(value)
	}
}

// #endregion 🗄️Store

// #region 📦️Export

// 🧭️ExportSource is the port an export reads its entities through.
type ExportSource interface {
	ExportEntities() []ExportEntity
}

// 🏷️ExportEntity is one entity of a repository snapshot.
type ExportEntity struct {
	Kind  string
	ID    string
	Value interface{}
}

// 🧾️ExportSnapshot is the deterministic identity and per-kind counts of one export batch.
type ExportSnapshot struct {
	Snapshot string         `json:"snapshot"`
	Counts   map[string]int `json:"counts"`
	Inputs   []Input        `json:"-"`
}

// 🧮️BuildExportSnapshot sorts entities by id, hashes them and namespaces every input id.
func BuildExportSnapshot(ctx context.Context, entities []ExportEntity) (ExportSnapshot, error) {
	snapshot := ExportSnapshot{Counts: map[string]int{}}
	inputs := make([]Input, 0, len(entities))
	for _, entity := range entities {
		if err := ctx.Err(); err != nil {
			return ExportSnapshot{}, err
		}
		snapshot.Counts[entity.Kind]++
		inputs = append(inputs, Input{ID: entity.Kind + ":" + entity.ID, Kind: entity.Kind + ".recorded", Data: entity.Value})
	}
	sortInputs(inputs)
	hash := sha256.New()
	for _, input := range inputs {
		if err := ctx.Err(); err != nil {
			return ExportSnapshot{}, err
		}
		data, err := json.Marshal(input.Data)
		if err != nil {
			return ExportSnapshot{}, err
		}
		fmt.Fprintf(hash, "%s\x00%s\x00", input.ID, input.Kind)
		hash.Write(data)
	}
	snapshot.Snapshot = hex.EncodeToString(hash.Sum(nil))
	for index := range inputs {
		inputs[index].ID = "snapshot:" + snapshot.Snapshot + ":" + inputs[index].ID
	}
	snapshot.Inputs = inputs
	return snapshot, nil
}

func sortInputs(inputs []Input) {
	for outer := 1; outer < len(inputs); outer++ {
		current := inputs[outer]
		inner := outer - 1
		for inner >= 0 && inputs[inner].ID > current.ID {
			inputs[inner+1] = inputs[inner]
			inner--
		}
		inputs[inner+1] = current
	}
}

// #endregion 📦️Export

// #region 🚚️Split

// #region ⚡️Server Client

// 🖥️getServerAddr returns the server address from COMPOSE_SERVER_ADDR env var.
func GetServerAddr() string {
	addr := strings.TrimSpace(os.Getenv("COMPOSE_SERVER_ADDR"))
	if addr == "" {
		return ""
	}
	if !strings.HasPrefix(addr, "http://") && !strings.HasPrefix(addr, "https://") {
		addr = "http://" + addr
	}
	return strings.TrimSuffix(addr, "/")
}

// 🎟️getServerToken returns the API key from COMPOSE_SERVER_TOKEN env var.
func GetServerToken() string {
	return strings.TrimSpace(os.Getenv("COMPOSE_SERVER_TOKEN"))
}

// 📨️serverRequest sends an authenticated HTTP request to the server.
func ServerRequest(method, path string, body interface{}) (*http.Response, error) {
	addr := GetServerAddr()
	if addr == "" {
		return nil, fmt.Errorf("COMPOSE_SERVER_ADDR not set")
	}
	url := addr + path
	var reqBody io.Reader
	if body != nil {
		jsonBytes, err := json.Marshal(body)
		if err != nil {
			return nil, err
		}
		reqBody = bytes.NewReader(jsonBytes)
	}
	req, err := http.NewRequest(method, url, reqBody)
	if err != nil {
		return nil, err
	}
	req.Header.Set("Content-Type", "application/json")
	if token := GetServerToken(); token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}
	client := &http.Client{Timeout: 10 * time.Second}
	return client.Do(req)
}

// 🔐️serverWhoami calls the auth whoami endpoint and returns developer info.
func ServerWhoami() (map[string]interface{}, error) {
	resp, err := ServerRequest("GET", "/api/v1/auth", nil)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()
	if resp.StatusCode != 200 {
		return nil, fmt.Errorf("auth failed: %s", resp.Status)
	}
	var result map[string]interface{}
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, err
	}
	return result, nil
}

// #endregion ⚡️Server Client

// #endregion 🚚️Split
