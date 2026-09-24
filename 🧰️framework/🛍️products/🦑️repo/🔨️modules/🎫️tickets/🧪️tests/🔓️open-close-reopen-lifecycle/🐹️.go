// 🐹️ Go side of the ticket lifecycle case. Everything runs against an in-memory store, a frozen
// clock, a null issue tracker and a recording event sink — no filesystem, no management binary.
package adapter

import (
	"encoding/json"
	"fmt"

	tickets "github.com/usalu/semio/repo/tickets"
	host "semio.tech/repo/test"
)

// region 🔖️World

type clockVector struct {
	Year  int    `json:"year"`
	Month int    `json:"month"`
	Day   int    `json:"day"`
	Stamp string `json:"stamp"`
}

type purgeVector struct {
	OversizedFile string `json:"oversizedFile"`
	SmallFile     string `json:"smallFile"`
}

type lifecycleVectors struct {
	RepoMetaDir string          `json:"repoMetaDir"`
	Clock       clockVector     `json:"clock"`
	Open        json.RawMessage `json:"open"`
	Close       json.RawMessage `json:"close"`
	Reopen      json.RawMessage `json:"reopen"`
	Change      json.RawMessage `json:"change"`
	Purge       purgeVector     `json:"purge"`
}

type world struct {
	store   *tickets.MemoryTicketStore
	events  *tickets.RecordingEventSink
	layout  tickets.TicketLayout
	service *tickets.TicketService
}

func loadLifecycleVectors(ctx *host.Context) (lifecycleVectors, error) {
	data, err := ctx.FixtureBytes("shared://🔓️open-close-reopen-lifecycle/🔓️lifecycle.json")
	if err != nil {
		return lifecycleVectors{}, err
	}
	var vectors lifecycleVectors
	err = json.Unmarshal(data, &vectors)
	return vectors, err
}

func newWorld(vectors lifecycleVectors) *world {
	store := tickets.NewMemoryTicketStore()
	sink := tickets.NewRecordingEventSink()
	layout := tickets.NewTicketLayout(vectors.RepoMetaDir)
	clock := tickets.NewFixedTicketClock(vectors.Clock.Year, vectors.Clock.Month, vectors.Clock.Day, vectors.Clock.Stamp)
	service := tickets.NewTicketService(layout, store, tickets.NullIssueTracker{}, clock, sink)
	return &world{store: store, events: sink, layout: layout, service: service}
}

func rawText(raw json.RawMessage) string {
	if len(raw) == 0 {
		return "{}"
	}
	return string(raw)
}

func outcomeProjection(outcome tickets.TicketOutcome) map[string]any {
	warnings := outcome.Warnings
	if warnings == nil {
		warnings = []string{}
	}
	return map[string]any{
		"id":       outcome.ID,
		"relPath":  outcome.RelPath,
		"status":   outcome.Status,
		"document": outcome.Document,
		"journal":  outcome.Journal.Trace(),
		"warnings": warnings,
	}
}

func eventProjection(current *world) []map[string]any {
	recorded := current.events.Recorded()
	projected := make([]map[string]any, 0, len(recorded))
	for _, event := range recorded {
		projected = append(projected, map[string]any{"kind": event.Kind, "source": event.Source, "payload": event.Payload})
	}
	return projected
}

func openTicket(current *world, vectors lifecycleVectors) (tickets.TicketOutcome, error) {
	request, err := tickets.ParseOpenRequest(rawText(vectors.Open))
	if err != nil {
		return tickets.TicketOutcome{}, err
	}
	return current.service.Open(request)
}

// endregion 🔖️World

// region 🔖️Scenarios

func anOpenMaterialisesTheFolderAndEmits(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadLifecycleVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	current := newWorld(vectors)
	outcome, err := openTicket(current, vectors)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"outcome": outcomeProjection(outcome),
		"paths":   current.store.Paths(),
		"events":  eventProjection(current),
	}}, nil
}

func aCloseConsumesTheImportantDocument(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadLifecycleVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	current := newWorld(vectors)
	if _, err := openTicket(current, vectors); err != nil {
		return host.Outcome{}, err
	}
	request, err := tickets.ParseCloseRequest(rawText(vectors.Close))
	if err != nil {
		return host.Outcome{}, err
	}
	outcome, err := current.service.Close(request)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"outcome": outcomeProjection(outcome),
		"paths":   current.store.Paths(),
		"events":  eventProjection(current),
	}}, nil
}

func aReopenRestoresTheImportantDocument(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadLifecycleVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	current := newWorld(vectors)
	if _, err := openTicket(current, vectors); err != nil {
		return host.Outcome{}, err
	}
	closeRequest, err := tickets.ParseCloseRequest(rawText(vectors.Close))
	if err != nil {
		return host.Outcome{}, err
	}
	if _, err := current.service.Close(closeRequest); err != nil {
		return host.Outcome{}, err
	}
	reopenRequest, err := tickets.ParseReopenRequest(rawText(vectors.Reopen))
	if err != nil {
		return host.Outcome{}, err
	}
	outcome, err := current.service.Reopen(reopenRequest)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"outcome": outcomeProjection(outcome),
		"paths":   current.store.Paths(),
		"events":  eventProjection(current),
	}}, nil
}

func aChangeRenamesTheFolder(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadLifecycleVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	current := newWorld(vectors)
	if _, err := openTicket(current, vectors); err != nil {
		return host.Outcome{}, err
	}
	request, err := tickets.ParseChangeRequest(rawText(vectors.Change))
	if err != nil {
		return host.Outcome{}, err
	}
	outcome, err := current.service.Change(request)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"outcome": outcomeProjection(outcome),
		"paths":   current.store.Paths(),
		"events":  eventProjection(current),
	}}, nil
}

func aCloseRefusesATicketThatIsNotOpen(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadLifecycleVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	current := newWorld(vectors)
	if _, err := openTicket(current, vectors); err != nil {
		return host.Outcome{}, err
	}
	valid, err := tickets.ParseCloseRequest(rawText(vectors.Close))
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	refuse := func(name string, request tickets.TicketCloseRequest) {
		if _, err := current.service.Close(request); err != nil {
			projection[name] = fmt.Sprintf("%s:%s", tickets.TicketErrorClass(err), err.Error())
			return
		}
		projection[name] = "accepted"
	}
	noSummary := valid
	noSummary.Summary = ""
	refuse("no-summary", noSummary)
	noFiles := valid
	noFiles.Files = nil
	refuse("no-files", noFiles)
	unknownID := valid
	unknownID.ID = "26/09/06/NO-SUCH-TICKET"
	refuse("unknown-id", unknownID)
	malformedID := valid
	malformedID.ID = "nonsense"
	refuse("malformed-id", malformedID)
	refuse("first-close", valid)
	refuse("second-close", valid)
	return host.Outcome{Projection: projection}, nil
}

func fileInputsAreNormalisedBeforeTheyAreRecorded(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadLifecycleVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	request, err := tickets.ParseCloseRequest(rawText(vectors.Close))
	if err != nil {
		return host.Outcome{}, err
	}
	input := request.Files
	if input == nil {
		input = []string{}
	}
	return host.Outcome{Projection: map[string]any{
		"input":      input,
		"normalised": tickets.NormalizeTicketFileInputs(request.Files),
	}}, nil
}

func anOversizedArtifactIsPurged(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadLifecycleVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	current := newWorld(vectors)
	outcome, err := openTicket(current, vectors)
	if err != nil {
		return host.Outcome{}, err
	}
	id, err := tickets.ParseTicketID(outcome.ID)
	if err != nil {
		return host.Outcome{}, err
	}
	folder := current.layout.TicketDir(id)
	current.store.SeedSizedFile(tickets.JoinPath(folder, vectors.Purge.OversizedFile), tickets.OversizedFileBytes+1)
	current.store.SeedSizedFile(tickets.JoinPath(folder, vectors.Purge.SmallFile), 128)
	report, err := current.service.PurgeArtifacts(id)
	if err != nil {
		return host.Outcome{}, err
	}
	surviving := make([]string, 0)
	for _, path := range current.store.Paths() {
		if tickets.StoreIsFile(current.store, path) {
			surviving = append(surviving, path)
		}
	}
	return host.Outcome{Projection: map[string]any{
		"removedFiles":       report.RemovedFiles,
		"removedDirectories": report.RemovedDirectories,
		"surviving":          surviving,
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("an-open-materialises-the-folder-and-emits", anOpenMaterialisesTheFolderAndEmits).
		Subject("a-close-consumes-the-important-document", aCloseConsumesTheImportantDocument).
		Subject("a-reopen-restores-the-important-document", aReopenRestoresTheImportantDocument).
		Subject("a-change-renames-the-folder", aChangeRenamesTheFolder).
		Subject("a-close-refuses-a-ticket-that-is-not-open", aCloseRefusesATicketThatIsNotOpen).
		Subject("file-inputs-are-normalised-before-they-are-recorded", fileInputsAreNormalisedBeforeTheyAreRecorded).
		Subject("an-oversized-artifact-is-purged", anOversizedArtifactIsPurged)
}

// endregion 🔖️Registration
