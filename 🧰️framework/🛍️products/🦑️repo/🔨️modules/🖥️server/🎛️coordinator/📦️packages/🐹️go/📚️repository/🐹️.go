// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📚️repository carries the event-sourced coordinator repository: command payloads, open, reload, execute and the record operations.

// #endregion 🧲️Header

package coordinator

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

// #region 📚️Repository

// #region 📚️Repository
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

// 🗄️EventRepository executes commands and derives every query projection by replay.
type EventRepository struct {
	store      *EventStore
	commands   sync.Mutex
	mu         sync.RWMutex
	closed     bool
	projection coordinatorProjection
}

// 🗄️openDatabase opens the owned coordinator event repository.
func OpenDatabase(path string) (*EventRepository, error) {
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

// #endregion 📚️Repository
