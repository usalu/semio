// 🐹️ Go side of the important-document transaction case. Every wrong bundle and the injected write
// failure are produced through the in-memory store, so nothing touches a real filesystem.
package adapter

import (
	"encoding/json"
	"fmt"

	tickets "github.com/usalu/semio/repo/tickets"
	host "semio.tech/repo/test"
)

// region 🔖️World

type transactionVectors struct {
	RepoMetaDir    string   `json:"repoMetaDir"`
	ID             string   `json:"id"`
	FailurePath    string   `json:"failurePath"`
	FailureMessage string   `json:"failureMessage"`
	Modes          []string `json:"modes"`
}

type world struct {
	store   *tickets.MemoryTicketStore
	layout  tickets.TicketLayout
	service *tickets.TicketService
}

func loadTransactionVectors(ctx *host.Context) (transactionVectors, error) {
	data, err := ctx.FixtureBytes("local://💾️cases.json")
	if err != nil {
		return transactionVectors{}, err
	}
	var vectors transactionVectors
	err = json.Unmarshal(data, &vectors)
	return vectors, err
}

func newWorld(vectors transactionVectors) *world {
	store := tickets.NewMemoryTicketStore()
	layout := tickets.NewTicketLayout(vectors.RepoMetaDir)
	clock := tickets.NewFixedTicketClock(26, 9, 6, "2026-09-06 12:00:00")
	service := tickets.NewTicketService(layout, store, tickets.NullIssueTracker{}, clock, tickets.NewRecordingEventSink())
	return &world{store: store, layout: layout, service: service}
}

func openRequest() tickets.TicketOpenRequest {
	return tickets.TicketOpenRequest{
		Emoji:        "💾️",
		Title:        "Important Document Transaction",
		Prompt:       "Prove the transaction",
		Client:       "claude-code",
		Goal:         "🎯️aioptimizedrepo",
		NoIssue:      true,
		NoManagement: true,
	}
}

func closeRequest(id string) tickets.TicketCloseRequest {
	return tickets.TicketCloseRequest{ID: id, Summary: "done", Files: []string{"a/b.rs"}, NoManagement: true}
}

func reopenRequest(id string) tickets.TicketReopenRequest {
	return tickets.TicketReopenRequest{ID: id, Prompt: "again", Client: "claude-code", NoManagement: true}
}

func presence(store *tickets.MemoryTicketStore, path string) string {
	if tickets.StoreExists(store, path) {
		return "present"
	}
	return "absent"
}

// endregion 🔖️World

// region 🔖️Scenarios

func aWrongBundleIsRefusedBeforeAnythingIsRemoved(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadTransactionVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, mode := range vectors.Modes {
		current := newWorld(vectors)
		opened, err := current.service.Open(openRequest())
		if err != nil {
			return host.Outcome{}, err
		}
		id, err := tickets.ParseTicketID(opened.ID)
		if err != nil {
			return host.Outcome{}, err
		}
		important := current.layout.ImportantPath(id)
		switch mode {
		case "missing":
			if err := current.store.RemoveFile(important); err != nil {
				return host.Outcome{}, err
			}
		case "not-empty":
			if err := current.store.Write(important, "not empty at all"); err != nil {
				return host.Outcome{}, err
			}
		case "not-a-file":
			if err := current.store.RemoveFile(important); err != nil {
				return host.Outcome{}, err
			}
			current.store.SeedSymlink(important)
		default:
			current.store.SeedFile(tickets.JoinPath(current.layout.ImportantDir(id), "🪤️stray.md"), "stray")
		}
		refusal := "accepted"
		if _, err := current.service.Close(closeRequest(opened.ID)); err != nil {
			refusal = fmt.Sprintf("%s:%s", tickets.TicketErrorClass(err), err.Error())
		}
		stillThere := "no"
		if tickets.StoreExists(current.store, current.layout.DocumentPath(id)) {
			stillThere = "yes"
		}
		projection[mode] = map[string]any{"refusal": refusal, "documentStillThere": stillThere}
	}
	return host.Outcome{Projection: projection}, nil
}

func aFailedSaveRollsTheCreationBack(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadTransactionVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	current := newWorld(vectors)
	opened, err := current.service.Open(openRequest())
	if err != nil {
		return host.Outcome{}, err
	}
	id, err := tickets.ParseTicketID(opened.ID)
	if err != nil {
		return host.Outcome{}, err
	}
	if _, err := current.service.Close(closeRequest(opened.ID)); err != nil {
		return host.Outcome{}, err
	}
	current.store.FailWrite(vectors.FailurePath, vectors.FailureMessage)
	failure := "accepted"
	if _, err := current.service.Reopen(reopenRequest(opened.ID)); err != nil {
		failure = fmt.Sprintf("%s:%s", tickets.TicketErrorClass(err), err.Error())
	}
	journal := tickets.TransactionJournal{}
	creation, err := tickets.EnsureImportantDocument(current.store, current.layout.ImportantPath(id), &journal)
	if err != nil {
		return host.Outcome{}, err
	}
	if err := tickets.RollbackImportantCreation(current.store, creation, &journal); err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"failure":            failure,
		"replayJournal":      journal.Trace(),
		"importantDocument":  presence(current.store, current.layout.ImportantPath(id)),
		"importantDirectory": presence(current.store, current.layout.ImportantDir(id)),
	}}, nil
}

func aPreservedDocumentIsNotRolledBack(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadTransactionVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	current := newWorld(vectors)
	opened, err := current.service.Open(openRequest())
	if err != nil {
		return host.Outcome{}, err
	}
	id, err := tickets.ParseTicketID(opened.ID)
	if err != nil {
		return host.Outcome{}, err
	}
	if _, err := current.service.Close(closeRequest(opened.ID)); err != nil {
		return host.Outcome{}, err
	}
	if err := current.store.Write(current.layout.ImportantPath(id), ""); err != nil {
		return host.Outcome{}, err
	}
	current.store.FailWrite(vectors.FailurePath, vectors.FailureMessage)
	failure := "accepted"
	if _, err := current.service.Reopen(reopenRequest(opened.ID)); err != nil {
		failure = fmt.Sprintf("%s:%s", tickets.TicketErrorClass(err), err.Error())
	}
	journal := tickets.TransactionJournal{}
	creation, err := tickets.EnsureImportantDocument(current.store, current.layout.ImportantPath(id), &journal)
	if err != nil {
		return host.Outcome{}, err
	}
	if err := tickets.RollbackImportantCreation(current.store, creation, &journal); err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"failure":           failure,
		"replayJournal":     journal.Trace(),
		"importantDocument": presence(current.store, current.layout.ImportantPath(id)),
	}}, nil
}

func theJournalRecordsEveryStepInOrder(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadTransactionVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	current := newWorld(vectors)
	opened, err := current.service.Open(openRequest())
	if err != nil {
		return host.Outcome{}, err
	}
	id, err := tickets.ParseTicketID(opened.ID)
	if err != nil {
		return host.Outcome{}, err
	}
	closed, err := current.service.Close(closeRequest(opened.ID))
	if err != nil {
		return host.Outcome{}, err
	}
	reopened, err := current.service.Reopen(reopenRequest(opened.ID))
	if err != nil {
		return host.Outcome{}, err
	}
	inspection := tickets.TransactionJournal{}
	preimage, inspectErr := tickets.InspectImportantDocument(current.store, current.layout.ImportantPath(id), &inspection)
	inspected := preimage.Path
	if inspectErr != nil {
		inspected = inspectErr.Error()
	}
	return host.Outcome{Projection: map[string]any{
		"open":       opened.Journal.Trace(),
		"close":      closed.Journal.Trace(),
		"reopen":     reopened.Journal.Trace(),
		"inspection": inspection.Trace(),
		"inspected":  inspected,
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("a-wrong-bundle-is-refused-before-anything-is-removed", aWrongBundleIsRefusedBeforeAnythingIsRemoved).
		Subject("a-failed-save-rolls-the-creation-back", aFailedSaveRollsTheCreationBack).
		Subject("a-preserved-document-is-not-rolled-back", aPreservedDocumentIsNotRolledBack).
		Subject("the-journal-records-every-step-in-order", theJournalRecordsEveryStepInOrder)
}

// endregion 🔖️Registration
