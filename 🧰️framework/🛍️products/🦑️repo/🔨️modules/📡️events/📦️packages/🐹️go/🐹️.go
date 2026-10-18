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
	json "encoding/json"
	errors "errors"
	os "os"
	filepath "path/filepath"
	runtime "runtime"
	sync "sync"
)

// #region 📋️Event Kinds

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

// ✉️Event is the canonical envelope for a changing interaction sent from CLI to server.
type Event struct {
	Kind    EventKind       `json:"kind"`
	Source  string          `json:"source"`
	Payload json.RawMessage `json:"payload"`
}

// #endregion 📋️Event Kinds
