// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Event-sourced coordinator repository and deterministic projections.

// #endregion 🧲️Header

package main

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"sync"
	"time"
)

// #region 📜️Contract

var (
	ErrProjectionNotFound = errors.New("projection not found")
	ErrProjectionLimit    = errors.New("projection query limit exceeded")
)

const coordinatorStream = "coordinator"

const (
	defaultProjectionMaxItems   = 1_000_000
	defaultProjectionMaxResults = 10_000
)

const (
	eventPublished           = "event.published"
	eventTicketRecorded      = "ticket.recorded"
	eventScopesRecorded      = "scopes.recorded"
	eventClaimRecorded       = "claim.recorded"
	eventWarningsRecorded    = "warnings.recorded"
	eventContributorRecorded = "contributor.recorded"
	eventContributorReleased = "contributor.released"
	eventCheckpointRecorded  = "checkpoint.recorded"
)

// 🗄️CoordinatorRepository separates commands from replayed read projections.
type CoordinatorRepository interface {
	Close() error
	Reopen(context.Context) error
	recordPublishedEvent(context.Context, Event) error
	recordTicket(context.Context, Ticket) error
	projectTickets(context.Context, string, ...ProjectionQuery) ([]Ticket, error)
	projectTicket(context.Context, string, ...ProjectionQuery) (*Ticket, error)
	recordScopes(context.Context, string, []Scope) error
	projectScopesByFile(context.Context, string, ...ProjectionQuery) ([]Scope, error)
	recordClaim(context.Context, string, string, string, time.Time) error
	projectClaimsByTicket(context.Context, string, ...ProjectionQuery) ([]Scope, error)
	recordWarnings(context.Context, []Warning) error
	projectWarnings(context.Context, string, ...ProjectionQuery) ([]Warning, error)
	projectBreachs(context.Context, string, ...ProjectionQuery) ([]Breach, error)
	projectConflicts(context.Context, ...ProjectionQuery) ([]struct {
		ScopeID string
		Tickets []string
	}, error)
	recordContributorWork(context.Context, string, string, string) error
	recordContributorRelease(context.Context, string, []struct{ Kind, ID string }) error
	projectContributorsOnItem(context.Context, string, string, ...ProjectionQuery) ([]string, error)
	recordCheckpoint(context.Context, string, []string) error
}

// 🔎️ProjectionQuery bounds projection work and observes deterministic traversal progress.
type ProjectionQuery struct {
	MaxItems   int
	MaxResults int
	Progress   func(ProjectionProgress)
}

// 📊️ProjectionProgress reports one completed projection traversal step.
type ProjectionProgress struct {
	Phase   string
	Current int
	Total   int
}

type projectionTraversal struct {
	ctx     context.Context
	query   ProjectionQuery
	current int
}

type repositoryCommand struct {
	Type       string
	Payload    any
	Generation uint64
}

type scopesRecordedPayload struct {
	FilePath string  `json:"file_path"`
	Scopes   []Scope `json:"scopes"`
}

type claimRecordedPayload struct {
	TicketID string    `json:"ticket_id"`
	ScopeID  string    `json:"scope_id"`
	Type     string    `json:"claim_type"`
	At       time.Time `json:"at"`
}

type warningsRecordedPayload struct {
	Warnings []Warning `json:"warnings"`
}

type contributorRecordedPayload struct {
	GitHub string `json:"github"`
	Kind   string `json:"kind"`
	ItemID string `json:"item_id"`
}

type contributorReleasedPayload struct {
	GitHub string `json:"github"`
	Items  []struct {
		Kind string `json:"kind"`
		ID   string `json:"id"`
	} `json:"items"`
}

type checkpointRecordedPayload struct {
	GitHub string   `json:"github"`
	Files  []string `json:"files"`
}

// #endregion 📜️Contract

// #region 📊️Projection

type claimProjection struct {
	Type string
	At   time.Time
}

type coordinatorProjection struct {
	sequence     uint64
	tickets      map[string]Ticket
	scopes       map[string]map[string]Scope
	claims       map[string]map[string]claimProjection
	warnings     map[string]Warning
	breachs      map[string]Breach
	contributors map[string]map[string]struct{}
}

func newCoordinatorProjection() coordinatorProjection {
	return coordinatorProjection{
		tickets:      map[string]Ticket{},
		scopes:       map[string]map[string]Scope{},
		claims:       map[string]map[string]claimProjection{},
		warnings:     map[string]Warning{},
		breachs:      map[string]Breach{},
		contributors: map[string]map[string]struct{}{},
	}
}

func (projection *coordinatorProjection) apply(event EventEnvelope) error {
	if event.Stream != coordinatorStream {
		return nil
	}
	switch event.Type {
	case eventPublished:
		var value Event
		if err := json.Unmarshal(event.Payload, &value); err != nil {
			return err
		}
	case eventTicketRecorded:
		var value Ticket
		if err := json.Unmarshal(event.Payload, &value); err != nil {
			return err
		}
		projection.tickets[value.ID] = value
	case eventScopesRecorded:
		var value scopesRecordedPayload
		if err := json.Unmarshal(event.Payload, &value); err != nil {
			return err
		}
		byID := make(map[string]Scope, len(value.Scopes))
		for _, scope := range value.Scopes {
			byID[scope.ID] = scope
		}
		projection.scopes[value.FilePath] = byID
	case eventClaimRecorded:
		var value claimRecordedPayload
		if err := json.Unmarshal(event.Payload, &value); err != nil {
			return err
		}
		if projection.claims[value.TicketID] == nil {
			projection.claims[value.TicketID] = map[string]claimProjection{}
		}
		projection.claims[value.TicketID][value.ScopeID] = claimProjection{Type: value.Type, At: value.At}
	case eventWarningsRecorded:
		var value warningsRecordedPayload
		if err := json.Unmarshal(event.Payload, &value); err != nil {
			return err
		}
		for id, warning := range projection.warnings {
			if warning.Kind == "conflict" {
				delete(projection.warnings, id)
			}
		}
		for _, warning := range value.Warnings {
			projection.warnings[warning.ID] = warning
		}
	case eventContributorRecorded:
		var value contributorRecordedPayload
		if err := json.Unmarshal(event.Payload, &value); err != nil {
			return err
		}
		if projection.contributors[value.GitHub] == nil {
			projection.contributors[value.GitHub] = map[string]struct{}{}
		}
		projection.contributors[value.GitHub][contributorKey(value.Kind, value.ItemID)] = struct{}{}
	case eventContributorReleased:
		var value contributorReleasedPayload
		if err := json.Unmarshal(event.Payload, &value); err != nil {
			return err
		}
		for _, item := range value.Items {
			delete(projection.contributors[value.GitHub], contributorKey(item.Kind, item.ID))
		}
	case eventCheckpointRecorded:
		var value checkpointRecordedPayload
		if err := json.Unmarshal(event.Payload, &value); err != nil {
			return err
		}
		for _, file := range value.Files {
			delete(projection.contributors[value.GitHub], contributorKey("file", file))
		}
	default:
		return fmt.Errorf("unknown coordinator event type %q", event.Type)
	}
	projection.sequence = event.Sequence
	return nil
}

func contributorKey(kind string, id string) string { return kind + "\x00" + id }

// #endregion 📊️Projection

// #region 🗄️Repository

// 🗄️EventRepository executes commands and derives every query projection by replay.
type EventRepository struct {
	store      *EventStore
	commands   sync.Mutex
	mu         sync.RWMutex
	closed     bool
	projection coordinatorProjection
}

// 🗄️openDatabase opens the owned coordinator event repository.
func openDatabase(path string) (*EventRepository, error) {
	store, err := OpenEventStore(context.Background(), path, DefaultStoreLimits())
	if err != nil {
		return nil, err
	}
	repository := &EventRepository{store: store, projection: newCoordinatorProjection()}
	if err := repository.reload(context.Background()); err != nil {
		return nil, err
	}
	return repository, nil
}

// 📪️Close makes repository operations explicitly unavailable without discarding durable state.
func (repository *EventRepository) Close() error {
	repository.commands.Lock()
	defer repository.commands.Unlock()
	repository.mu.Lock()
	repository.closed = true
	repository.mu.Unlock()
	return nil
}

// 🔌️Reopen recovers short storage shortages and rebuilds projections without blocking other processes indefinitely.
func (repository *EventRepository) Reopen(ctx context.Context) error {
	repository.commands.Lock()
	defer repository.commands.Unlock()
	if err := repository.reload(ctx); err != nil {
		return err
	}
	repository.mu.Lock()
	repository.closed = false
	repository.mu.Unlock()
	return nil
}

func (repository *EventRepository) reload(ctx context.Context) error {
	events, err := repository.store.Replay(ctx, nil)
	if err != nil {
		return err
	}
	projection := newCoordinatorProjection()
	for _, event := range events {
		if err := projection.apply(event); err != nil {
			return fmt.Errorf("%w: projection at sequence %d: %v", ErrStoreCorrupt, event.Sequence, err)
		}
	}
	repository.mu.Lock()
	repository.projection = projection
	repository.mu.Unlock()
	return nil
}

func (repository *EventRepository) execute(ctx context.Context, command repositoryCommand) error {
	if err := ctx.Err(); err != nil {
		return err
	}
	payload, err := json.Marshal(command.Payload)
	if err != nil {
		return err
	}
	input := EventInput{Stream: coordinatorStream, ID: commandID(command.Type, command.Generation, payload), Generation: command.Generation, Type: command.Type, Payload: payload}
	repository.commands.Lock()
	defer repository.commands.Unlock()
	for attempt := 0; attempt < 3; attempt++ {
		repository.mu.RLock()
		closed := repository.closed
		expected := repository.projection.sequence
		repository.mu.RUnlock()
		if closed {
			return ErrStoreUnavailable
		}
		result, appendErr := repository.store.Append(ctx, expected, []EventInput{input}, nil)
		if appendErr == nil || result.Committed {
			repository.mu.Lock()
			for _, event := range result.Events {
				if event.Sequence > repository.projection.sequence {
					if err := repository.projection.apply(event); err != nil {
						repository.mu.Unlock()
						return err
					}
				}
			}
			repository.mu.Unlock()
			return appendErr
		}
		if !errors.Is(appendErr, ErrSequenceConflict) {
			return appendErr
		}
		if err := repository.reload(ctx); err != nil {
			return err
		}
	}
	return ErrSequenceConflict
}

func commandID(eventType string, generation uint64, payload []byte) string {
	hash := sha256.New()
	fmt.Fprintf(hash, "%s\x00%d\x00", eventType, generation)
	hash.Write(payload)
	return "command-" + hex.EncodeToString(hash.Sum(nil))
}

func (repository *EventRepository) queryReady(ctx context.Context) error {
	if err := ctx.Err(); err != nil {
		return err
	}
	repository.mu.RLock()
	closed := repository.closed
	repository.mu.RUnlock()
	if closed {
		return ErrStoreUnavailable
	}
	return nil
}

// #endregion 🗄️Repository

// #region 📨️Commands

func (repository *EventRepository) recordPublishedEvent(ctx context.Context, event Event) error {
	return repository.execute(ctx, repositoryCommand{Type: eventPublished, Payload: event, Generation: 1})
}

func (repository *EventRepository) recordTicket(ctx context.Context, ticket Ticket) error {
	return repository.execute(ctx, repositoryCommand{Type: eventTicketRecorded, Payload: ticket, Generation: 1})
}

func (repository *EventRepository) recordScopes(ctx context.Context, filePath string, scopes []Scope) error {
	return repository.execute(ctx, repositoryCommand{Type: eventScopesRecorded, Payload: scopesRecordedPayload{FilePath: filePath, Scopes: scopes}, Generation: 1})
}

func (repository *EventRepository) recordClaim(ctx context.Context, ticketID string, scopeID string, claimType string, now time.Time) error {
	return repository.execute(ctx, repositoryCommand{Type: eventClaimRecorded, Payload: claimRecordedPayload{TicketID: ticketID, ScopeID: scopeID, Type: claimType, At: now.UTC()}, Generation: 1})
}

func (repository *EventRepository) recordWarnings(ctx context.Context, warnings []Warning) error {
	return repository.execute(ctx, repositoryCommand{Type: eventWarningsRecorded, Payload: warningsRecordedPayload{Warnings: warnings}, Generation: 1})
}

func (repository *EventRepository) recordContributorWork(ctx context.Context, github string, kind string, itemID string) error {
	return repository.execute(ctx, repositoryCommand{Type: eventContributorRecorded, Payload: contributorRecordedPayload{GitHub: github, Kind: kind, ItemID: itemID}, Generation: 1})
}

func (repository *EventRepository) recordContributorRelease(ctx context.Context, github string, kindsAndIDs []struct{ Kind, ID string }) error {
	value := contributorReleasedPayload{GitHub: github}
	for _, item := range kindsAndIDs {
		value.Items = append(value.Items, struct {
			Kind string `json:"kind"`
			ID   string `json:"id"`
		}{Kind: item.Kind, ID: item.ID})
	}
	return repository.execute(ctx, repositoryCommand{Type: eventContributorReleased, Payload: value, Generation: 1})
}

func (repository *EventRepository) recordCheckpoint(ctx context.Context, github string, files []string) error {
	return repository.execute(ctx, repositoryCommand{Type: eventCheckpointRecorded, Payload: checkpointRecordedPayload{GitHub: github, Files: files}, Generation: 1})
}

// #endregion 📨️Commands

// #region 🔎️Queries

func (repository *EventRepository) projectTickets(ctx context.Context, status string, queries ...ProjectionQuery) ([]Ticket, error) {
	traversal, err := repository.beginProjection(ctx, queries)
	if err != nil {
		return nil, err
	}
	repository.mu.RLock()
	defer repository.mu.RUnlock()
	result := make([]Ticket, 0, projectionResultCapacity(traversal, len(repository.projection.tickets)))
	keys, err := projectionKeys(traversal, "tickets.keys", repository.projection.tickets)
	if err != nil {
		return nil, err
	}
	for _, id := range keys {
		if err := traversal.step("tickets.fold", len(keys)); err != nil {
			return nil, err
		}
		ticket := repository.projection.tickets[id]
		if status == "" || ticket.Status == status {
			if err := traversal.result(len(result) + 1); err != nil {
				return nil, err
			}
			result = append(result, ticket)
		}
	}
	return result, nil
}

func (repository *EventRepository) projectTicket(ctx context.Context, ticketID string, queries ...ProjectionQuery) (*Ticket, error) {
	traversal, err := repository.beginProjection(ctx, queries)
	if err != nil {
		return nil, err
	}
	repository.mu.RLock()
	defer repository.mu.RUnlock()
	if err := traversal.step("ticket.lookup", 1); err != nil {
		return nil, err
	}
	ticket, exists := repository.projection.tickets[ticketID]
	if !exists {
		return nil, fmt.Errorf("%w: ticket %q", ErrProjectionNotFound, ticketID)
	}
	if err := traversal.result(1); err != nil {
		return nil, err
	}
	return &ticket, nil
}

func (repository *EventRepository) projectScopesByFile(ctx context.Context, filePath string, queries ...ProjectionQuery) ([]Scope, error) {
	traversal, err := repository.beginProjection(ctx, queries)
	if err != nil {
		return nil, err
	}
	repository.mu.RLock()
	defer repository.mu.RUnlock()
	byID := repository.projection.scopes[filePath]
	result := make([]Scope, 0, projectionResultCapacity(traversal, len(byID)))
	keys, err := projectionKeys(traversal, "scopes.keys", byID)
	if err != nil {
		return nil, err
	}
	for _, id := range keys {
		if err := traversal.step("scopes.fold", len(keys)); err != nil {
			return nil, err
		}
		if err := traversal.result(len(result) + 1); err != nil {
			return nil, err
		}
		result = append(result, byID[id])
	}
	return result, nil
}

func (repository *EventRepository) projectClaimsByTicket(ctx context.Context, ticketID string, queries ...ProjectionQuery) ([]Scope, error) {
	traversal, err := repository.beginProjection(ctx, queries)
	if err != nil {
		return nil, err
	}
	repository.mu.RLock()
	defer repository.mu.RUnlock()
	byID := map[string]Scope{}
	for _, scopes := range repository.projection.scopes {
		if err := traversal.step("claims.file-fold", len(repository.projection.scopes)); err != nil {
			return nil, err
		}
		for id, scope := range scopes {
			if err := traversal.step("claims.scope-fold", len(scopes)); err != nil {
				return nil, err
			}
			byID[id] = scope
		}
	}
	claims := repository.projection.claims[ticketID]
	result := make([]Scope, 0, projectionResultCapacity(traversal, len(claims)))
	keys, err := projectionKeys(traversal, "claims.keys", claims)
	if err != nil {
		return nil, err
	}
	for _, id := range keys {
		if err := traversal.step("claims.fold", len(keys)); err != nil {
			return nil, err
		}
		if scope, exists := byID[id]; exists {
			if err := traversal.result(len(result) + 1); err != nil {
				return nil, err
			}
			result = append(result, scope)
		}
	}
	return result, nil
}

func (repository *EventRepository) projectWarnings(ctx context.Context, ticketID string, queries ...ProjectionQuery) ([]Warning, error) {
	traversal, err := repository.beginProjection(ctx, queries)
	if err != nil {
		return nil, err
	}
	repository.mu.RLock()
	defer repository.mu.RUnlock()
	result := make([]Warning, 0, projectionResultCapacity(traversal, len(repository.projection.warnings)))
	keys, err := projectionKeys(traversal, "warnings.keys", repository.projection.warnings)
	if err != nil {
		return nil, err
	}
	for _, id := range keys {
		if err := traversal.step("warnings.fold", len(keys)); err != nil {
			return nil, err
		}
		warning := repository.projection.warnings[id]
		if ticketID == "" || warning.TicketID == ticketID {
			if err := traversal.result(len(result) + 1); err != nil {
				return nil, err
			}
			result = append(result, warning)
		}
	}
	return result, nil
}

func (repository *EventRepository) projectBreachs(ctx context.Context, ticketID string, queries ...ProjectionQuery) ([]Breach, error) {
	traversal, err := repository.beginProjection(ctx, queries)
	if err != nil {
		return nil, err
	}
	repository.mu.RLock()
	defer repository.mu.RUnlock()
	result := make([]Breach, 0, projectionResultCapacity(traversal, len(repository.projection.breachs)))
	keys, err := projectionKeys(traversal, "breachs.keys", repository.projection.breachs)
	if err != nil {
		return nil, err
	}
	for _, id := range keys {
		if err := traversal.step("breachs.fold", len(keys)); err != nil {
			return nil, err
		}
		breach := repository.projection.breachs[id]
		if ticketID == "" || breach.TicketID == ticketID {
			if err := traversal.result(len(result) + 1); err != nil {
				return nil, err
			}
			result = append(result, breach)
		}
	}
	return result, nil
}

func (repository *EventRepository) projectConflicts(ctx context.Context, queries ...ProjectionQuery) ([]struct {
	ScopeID string
	Tickets []string
}, error) {
	traversal, err := repository.beginProjection(ctx, queries)
	if err != nil {
		return nil, err
	}
	repository.mu.RLock()
	defer repository.mu.RUnlock()
	byScope := map[string][]string{}
	for ticketID, claims := range repository.projection.claims {
		if err := traversal.step("conflicts.ticket-fold", len(repository.projection.claims)); err != nil {
			return nil, err
		}
		if repository.projection.tickets[ticketID].Status != "open" {
			continue
		}
		for scopeID := range claims {
			if err := traversal.step("conflicts.claim-fold", len(claims)); err != nil {
				return nil, err
			}
			byScope[scopeID] = append(byScope[scopeID], ticketID)
		}
	}
	result := make([]struct {
		ScopeID string
		Tickets []string
	}, 0, projectionResultCapacity(traversal, len(byScope)))
	keys, err := projectionKeys(traversal, "conflicts.keys", byScope)
	if err != nil {
		return nil, err
	}
	for _, scopeID := range keys {
		if err := traversal.step("conflicts.fold", len(keys)); err != nil {
			return nil, err
		}
		tickets := byScope[scopeID]
		if len(tickets) > 1 {
			if err := sortProjectionStrings(traversal, "conflicts.tickets-sort", tickets); err != nil {
				return nil, err
			}
			if err := traversal.result(len(result) + 1); err != nil {
				return nil, err
			}
			result = append(result, struct {
				ScopeID string
				Tickets []string
			}{ScopeID: scopeID, Tickets: tickets})
		}
	}
	return result, nil
}

func (repository *EventRepository) projectContributorsOnItem(ctx context.Context, kind string, itemID string, queries ...ProjectionQuery) ([]string, error) {
	traversal, err := repository.beginProjection(ctx, queries)
	if err != nil {
		return nil, err
	}
	repository.mu.RLock()
	defer repository.mu.RUnlock()
	key := contributorKey(kind, itemID)
	result := make([]string, 0, projectionResultCapacity(traversal, len(repository.projection.contributors)))
	for github, items := range repository.projection.contributors {
		if err := traversal.step("contributors.fold", len(repository.projection.contributors)); err != nil {
			return nil, err
		}
		if _, exists := items[key]; exists {
			if err := traversal.result(len(result) + 1); err != nil {
				return nil, err
			}
			result = append(result, github)
		}
	}
	if err := sortProjectionStrings(traversal, "contributors.sort", result); err != nil {
		return nil, err
	}
	return result, nil
}

func (repository *EventRepository) beginProjection(ctx context.Context, queries []ProjectionQuery) (*projectionTraversal, error) {
	if err := repository.queryReady(ctx); err != nil {
		return nil, err
	}
	if len(queries) > 1 {
		return nil, errors.New("only one projection query contract is allowed")
	}
	query := ProjectionQuery{MaxItems: defaultProjectionMaxItems, MaxResults: defaultProjectionMaxResults}
	if len(queries) == 1 {
		query = queries[0]
	}
	if query.MaxItems <= 0 || query.MaxResults <= 0 {
		return nil, fmt.Errorf("%w: bounds must be positive", ErrProjectionLimit)
	}
	return &projectionTraversal{ctx: ctx, query: query}, nil
}

func (traversal *projectionTraversal) step(phase string, total int) error {
	if err := traversal.ctx.Err(); err != nil {
		return err
	}
	traversal.current++
	if traversal.current > traversal.query.MaxItems {
		return fmt.Errorf("%w: items %d > %d", ErrProjectionLimit, traversal.current, traversal.query.MaxItems)
	}
	if traversal.query.Progress != nil {
		traversal.query.Progress(ProjectionProgress{Phase: phase, Current: traversal.current, Total: total})
	}
	return traversal.ctx.Err()
}

func (traversal *projectionTraversal) result(count int) error {
	if count > traversal.query.MaxResults {
		return fmt.Errorf("%w: results %d > %d", ErrProjectionLimit, count, traversal.query.MaxResults)
	}
	return traversal.ctx.Err()
}

func (traversal *projectionTraversal) reserve(phase string, count int) error {
	if err := traversal.ctx.Err(); err != nil {
		return err
	}
	if count < 0 || count > traversal.query.MaxItems-traversal.current {
		return fmt.Errorf("%w: items %d > %d", ErrProjectionLimit, traversal.current+count, traversal.query.MaxItems)
	}
	traversal.current += count
	if traversal.query.Progress != nil {
		traversal.query.Progress(ProjectionProgress{Phase: phase, Current: traversal.current, Total: count})
	}
	return traversal.ctx.Err()
}

func projectionKeys[V any](traversal *projectionTraversal, phase string, values map[string]V) ([]string, error) {
	if err := traversal.ctx.Err(); err != nil {
		return nil, err
	}
	capacity := len(values)
	remaining := traversal.query.MaxItems - traversal.current
	if capacity > remaining {
		capacity = remaining
	}
	if capacity < 0 {
		capacity = 0
	}
	keys := make([]string, 0, capacity)
	for key := range values {
		if err := traversal.step(phase, len(values)); err != nil {
			return nil, err
		}
		keys = append(keys, key)
	}
	if err := traversal.ctx.Err(); err != nil {
		return nil, err
	}
	if err := sortProjectionStrings(traversal, phase+".sort", keys); err != nil {
		return nil, err
	}
	return keys, nil
}

func projectionResultCapacity(traversal *projectionTraversal, size int) int {
	capacity := size
	if capacity > traversal.query.MaxResults {
		capacity = traversal.query.MaxResults
	}
	remaining := traversal.query.MaxItems - traversal.current
	if capacity > remaining {
		capacity = remaining
	}
	if capacity < 0 {
		return 0
	}
	return capacity
}

func sortProjectionStrings(traversal *projectionTraversal, phase string, values []string) error {
	if len(values) < 2 {
		return traversal.ctx.Err()
	}
	if err := traversal.reserve(phase+".buffer", len(values)); err != nil {
		return err
	}
	buffer := make([]string, len(values))
	source := values
	destination := buffer
	sourceIsValues := true
	for width := 1; width < len(values); width *= 2 {
		for left := 0; left < len(values); left += width * 2 {
			middle := left + width
			if middle > len(values) {
				middle = len(values)
			}
			right := left + width*2
			if right > len(values) {
				right = len(values)
			}
			first := left
			second := middle
			for index := left; index < right; index++ {
				if err := traversal.step(phase+".merge", len(values)); err != nil {
					return err
				}
				if first < middle && (second >= right || source[first] <= source[second]) {
					destination[index] = source[first]
					first++
				} else {
					destination[index] = source[second]
					second++
				}
			}
		}
		source, destination = destination, source
		sourceIsValues = !sourceIsValues
		if width > len(values)/2 {
			break
		}
	}
	if !sourceIsValues {
		for index := range source {
			if err := traversal.step(phase+".copy", len(values)); err != nil {
				return err
			}
			values[index] = source[index]
		}
	}
	return traversal.ctx.Err()
}

// #endregion 🔎️Queries
