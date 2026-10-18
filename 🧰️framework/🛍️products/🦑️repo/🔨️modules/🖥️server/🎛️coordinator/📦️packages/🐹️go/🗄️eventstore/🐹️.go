// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🗄️eventstore carries the owned append-only event store: limits, envelopes, open, append, replay, commit and durable writes.

// #endregion 🧲️Header

package coordinator

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"sync"
	"time"
)

// #region 🗄️EventStore

// #region 🗄️EventStore
const eventStoreStageSchema = "semio.coordinator.event-stage/1"

var (
	ErrStoreUnavailable = errors.New("event store unavailable")
	ErrStoreCorrupt     = errors.New("event store corrupt")
	ErrDuplicateEvent   = errors.New("duplicate event")
	ErrSequenceConflict = errors.New("expected sequence conflict")
	ErrStoreLimit       = errors.New("event store limit exceeded")
	eventStoreLocks     sync.Map
)

// 📜️EventEnvelope is the language-neutral persisted event schema.
type EventEnvelope struct {
	Stream     string          `json:"stream"`
	Sequence   uint64          `json:"sequence"`
	ID         string          `json:"id"`
	Generation uint64          `json:"generation"`
	Type       string          `json:"type"`
	Payload    json.RawMessage `json:"payload"`
	Checksum   string          `json:"checksum"`
}

// 📨️EventInput is a sequence-free event proposed for an append command.
type EventInput struct {
	Stream     string
	ID         string
	Generation uint64
	Type       string
	Payload    json.RawMessage
}

// 📏️StoreLimits bounds persisted and in-flight work.
type StoreLimits struct {
	MaxPayloadBytes int
	MaxAppendBytes  int
	MaxAppendEvents int
	MaxReplayEvents int
	MaxJSONDepth    int
	MaxLogBytes     int64
}

// 📊️StoreProgress describes a bounded append or replay step.
type StoreProgress struct {
	Phase   string
	Current int
	Total   int
}

// 📦️AppendResult reports committed or idempotently observed events.
type AppendResult struct {
	Events         []EventEnvelope
	Duplicate      bool
	Committed      bool
	PendingCleanup bool
}

// 🛟️StoreRecoveryStatus exposes the last completed artifact-recovery action.
type StoreRecoveryStatus struct {
	Recovered bool
	Action    string
}

// 🧰️DefaultStoreLimits returns the production event-store bounds.
func DefaultStoreLimits() StoreLimits {
	return StoreLimits{
		MaxPayloadBytes: 1 << 20,
		MaxAppendBytes:  8 << 20,
		MaxAppendEvents: 4096,
		MaxReplayEvents: 1_000_000,
		MaxJSONDepth:    64,
		MaxLogBytes:     1 << 30,
	}
}

type appendStage struct {
	Schema        string `json:"schema"`
	PriorExists   bool   `json:"prior_exists"`
	PriorSize     int64  `json:"prior_size"`
	PriorChecksum string `json:"prior_checksum"`
	NextSize      int64  `json:"next_size"`
	NextChecksum  string `json:"next_checksum"`
}

type persistedFile struct {
	exists   bool
	size     int64
	checksum string
}

// 🧹️PendingCleanupError reports a committed append whose recovery artifacts still require cleanup.
type PendingCleanupError struct {
	Cause error
}

func (failure *PendingCleanupError) Error() string {
	return fmt.Sprintf("event append committed with pending cleanup: %v", failure.Cause)
}

func (failure *PendingCleanupError) Unwrap() error {
	return failure.Cause
}

// ✅️IsCommitted distinguishes maintenance failure from an uncommitted append.
func (failure *PendingCleanupError) IsCommitted() bool {
	return true
}

// 🗄️EventStore owns one append-only log and its crash-recovery artifacts.
type EventStore struct {
	path              string
	limits            StoreLimits
	operations        storeOperations
	interrupt         func(string) error
	heartbeatInterval time.Duration
	recoveryMu        sync.RWMutex
	recovery          StoreRecoveryStatus
}

// 🆕️OpenEventStore opens, recovers, and validates an event log.
func OpenEventStore(ctx context.Context, path string, limits StoreLimits) (*EventStore, error) {
	return openEventStoreWithOperations(ctx, path, limits, &faultStoreOperations{inner: nativeStoreOperations{}})
}

// 💥️ArmFault makes the store's next armed durable mutation fail with ErrInjectedFault; 0 disarms.
//
// The language-agnostic `💥️filesystem-fault-recovery` case drives this from a fixture so that both
// implementations answer the same interruption question without a platform-specific fault harness.
func (store *EventStore) ArmFault(at int) {
	if operations, ok := store.operations.(*faultStoreOperations); ok {
		operations.arm(at)
	}
}

func openEventStoreWithOperations(ctx context.Context, path string, limits StoreLimits, operations storeOperations) (opened *EventStore, returnErr error) {
	if path == "" {
		return nil, fmt.Errorf("%w: path is required", ErrStoreUnavailable)
	}
	if operations == nil {
		return nil, fmt.Errorf("%w: operations are required", ErrStoreUnavailable)
	}
	if err := validateStoreLimits(limits); err != nil {
		return nil, err
	}
	store := &EventStore{path: filepath.Clean(path), limits: limits, operations: operations, heartbeatInterval: 5 * time.Second}
	if err := operations.MkdirAll(filepath.Dir(store.path), 0o755); err != nil {
		return nil, fmt.Errorf("%w: %v", ErrStoreUnavailable, err)
	}
	unlock, err := store.lock(ctx)
	if err != nil {
		return nil, err
	}
	defer func() {
		returnErr = errors.Join(returnErr, unlock())
	}()
	if err := store.recoverLocked(ctx); err != nil {
		return nil, err
	}
	if _, err := store.replayLocked(ctx, nil, true); err != nil {
		return nil, err
	}
	return store, nil
}

func validateStoreLimits(limits StoreLimits) error {
	if limits.MaxPayloadBytes <= 0 || limits.MaxAppendBytes <= 0 || limits.MaxAppendEvents <= 0 || limits.MaxReplayEvents <= 0 || limits.MaxJSONDepth <= 0 || limits.MaxLogBytes <= 0 {
		return fmt.Errorf("%w: every bound must be positive", ErrStoreLimit)
	}
	return nil
}

// ➕️Append stages and atomically commits events at the expected stream sequence.
func (store *EventStore) Append(ctx context.Context, expectedSequence uint64, inputs []EventInput, progress func(StoreProgress)) (result AppendResult, returnErr error) {
	if err := ctx.Err(); err != nil {
		return AppendResult{}, err
	}
	if len(inputs) == 0 {
		return AppendResult{}, nil
	}
	if len(inputs) > store.limits.MaxAppendEvents {
		return AppendResult{}, fmt.Errorf("%w: events %d > %d", ErrStoreLimit, len(inputs), store.limits.MaxAppendEvents)
	}
	unlock, err := store.lock(ctx)
	if err != nil {
		return AppendResult{}, err
	}
	defer func() {
		returnErr = errors.Join(returnErr, unlock())
	}()
	if err := store.recoverLocked(ctx); err != nil {
		return AppendResult{}, err
	}
	existing, err := store.replayLocked(ctx, progress, true)
	if err != nil {
		return AppendResult{}, err
	}
	created, duplicate, err := store.prepareAppend(expectedSequence, existing, inputs, progress)
	if err != nil || duplicate {
		return AppendResult{Events: created, Duplicate: duplicate, Committed: duplicate}, err
	}
	encoded, err := encodeEventBatch(created)
	if err != nil {
		return AppendResult{}, err
	}
	priorBytes, err := store.readOptionalBounded(ctx, store.path, store.limits.MaxLogBytes, progress)
	if err != nil {
		return AppendResult{}, err
	}
	if int64(len(priorBytes)+len(encoded)) > store.limits.MaxLogBytes {
		return AppendResult{}, fmt.Errorf("%w: log bytes exceed %d", ErrStoreLimit, store.limits.MaxLogBytes)
	}
	stage := appendStage{
		Schema:        eventStoreStageSchema,
		PriorExists:   priorBytes != nil,
		PriorSize:     int64(len(priorBytes)),
		PriorChecksum: bytesChecksum(priorBytes),
		NextSize:      int64(len(priorBytes) + len(encoded)),
		NextChecksum:  bytesChecksum(append(append([]byte(nil), priorBytes...), encoded...)),
	}
	if err := store.commit(ctx, stage, priorBytes, encoded, progress); err != nil {
		var cleanup *PendingCleanupError
		if errors.As(err, &cleanup) {
			return AppendResult{Events: created, Committed: true, PendingCleanup: true}, err
		}
		return AppendResult{}, err
	}
	return AppendResult{Events: created, Committed: true}, nil
}

func (store *EventStore) prepareAppend(expectedSequence uint64, existing []EventEnvelope, inputs []EventInput, progress func(StoreProgress)) ([]EventEnvelope, bool, error) {
	normalized := make([]EventInput, len(inputs))
	for index, input := range inputs {
		if !validEventScalar(input.Stream) || !validEventScalar(input.ID) || !validEventScalar(input.Type) || input.Generation == 0 {
			return nil, false, errors.New("event stream, id, generation, and type are required")
		}
		if len(input.Payload) > store.limits.MaxPayloadBytes {
			return nil, false, fmt.Errorf("%w: payload bytes %d > %d", ErrStoreLimit, len(input.Payload), store.limits.MaxPayloadBytes)
		}
		payload, err := canonicalJSONPayload(input.Payload, store.limits.MaxJSONDepth)
		if err != nil {
			return nil, false, err
		}
		input.Payload = payload
		normalized[index] = input
	}
	inputs = normalized
	byID := make(map[string]EventEnvelope, len(existing))
	sequences := map[string]uint64{}
	for _, event := range existing {
		byID[event.ID] = event
		sequences[event.Stream] = event.Sequence
	}
	duplicateEvents := make([]EventEnvelope, 0, len(inputs))
	allDuplicate := true
	for _, input := range inputs {
		event, exists := byID[input.ID]
		if !exists {
			allDuplicate = false
			continue
		}
		if !sameEventInput(event, input) {
			return nil, false, fmt.Errorf("%w: id %q has different content", ErrDuplicateEvent, input.ID)
		}
		duplicateEvents = append(duplicateEvents, event)
	}
	if allDuplicate {
		return duplicateEvents, true, nil
	}
	if len(duplicateEvents) != 0 {
		return nil, false, fmt.Errorf("%w: mixed duplicate and new batch", ErrDuplicateEvent)
	}
	stream := inputs[0].Stream
	if stream == "" {
		return nil, false, errors.New("event stream is required")
	}
	if sequences[stream] != expectedSequence {
		return nil, false, fmt.Errorf("%w: stream %q expected %d actual %d", ErrSequenceConflict, stream, expectedSequence, sequences[stream])
	}
	created := make([]EventEnvelope, 0, len(inputs))
	seen := map[string]struct{}{}
	var encodedBytes int
	for index, input := range inputs {
		if input.Stream != stream {
			return nil, false, errors.New("one append batch must target one stream")
		}
		if _, exists := seen[input.ID]; exists {
			return nil, false, fmt.Errorf("%w: %s", ErrDuplicateEvent, input.ID)
		}
		event := EventEnvelope{Stream: stream, Sequence: expectedSequence + uint64(index) + 1, ID: input.ID, Generation: input.Generation, Type: input.Type, Payload: append(json.RawMessage(nil), input.Payload...)}
		event.Checksum = eventChecksum(event)
		line, err := encodeEvent(event)
		if err != nil {
			return nil, false, err
		}
		encodedBytes += len(line)
		if encodedBytes > store.limits.MaxAppendBytes {
			return nil, false, fmt.Errorf("%w: append bytes %d > %d", ErrStoreLimit, encodedBytes, store.limits.MaxAppendBytes)
		}
		created = append(created, event)
		seen[input.ID] = struct{}{}
		reportStoreProgress(progress, StoreProgress{Phase: "encoded", Current: index + 1, Total: len(inputs)})
	}
	return created, false, nil
}

func sameEventInput(event EventEnvelope, input EventInput) bool {
	return event.Stream == input.Stream && event.ID == input.ID && event.Generation == input.Generation && event.Type == input.Type && bytes.Equal(event.Payload, input.Payload)
}

// ⏪️Replay recovers and returns a deterministic copy of every valid event.
func (store *EventStore) Replay(ctx context.Context, progress func(StoreProgress)) (events []EventEnvelope, returnErr error) {
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	unlock, err := store.lock(ctx)
	if err != nil {
		return nil, err
	}
	defer func() {
		returnErr = errors.Join(returnErr, unlock())
	}()
	if err := store.recoverLocked(ctx); err != nil {
		return nil, err
	}
	return store.replayLocked(ctx, progress, true)
}

// 🛟️RecoveryStatus returns the last completed recovery action.
func (store *EventStore) RecoveryStatus() StoreRecoveryStatus {
	store.recoveryMu.RLock()
	defer store.recoveryMu.RUnlock()
	return store.recovery
}

func (store *EventStore) commit(ctx context.Context, stage appendStage, prior []byte, appended []byte, progress func(StoreProgress)) error {
	next := store.nextPath()
	if err := store.writeSyncedExclusive(next, prior, appended); err != nil {
		return fmt.Errorf("%w: %w", ErrStoreUnavailable, err)
	}
	if err := store.afterPhase(ctx, "next-synced", progress); err != nil {
		return err
	}
	if err := store.writeStage(stage); err != nil {
		return errors.Join(err, store.rollbackLocked())
	}
	if err := store.afterPhase(ctx, "stage-synced", progress); err != nil {
		return err
	}
	if stage.PriorExists {
		if err := store.removeDurably(store.backupPath()); err != nil {
			return errors.Join(fmt.Errorf("%w: remove stale backup: %w", ErrStoreUnavailable, err), store.rollbackLocked())
		}
		if err := store.renameDurably(store.path, store.backupPath()); err != nil {
			return errors.Join(fmt.Errorf("%w: backup prior: %w", ErrStoreUnavailable, err), store.rollbackLocked())
		}
	}
	if err := store.afterPhase(ctx, "prior-backed-up", progress); err != nil {
		return err
	}
	if err := store.renameDurably(next, store.path); err != nil {
		return errors.Join(fmt.Errorf("%w: replace log: %w", ErrStoreUnavailable, err), store.rollbackLocked())
	}
	if err := store.afterPhase(ctx, "log-replaced", progress); err != nil {
		return err
	}
	reportStoreProgress(progress, StoreProgress{Phase: "committed", Current: 1, Total: 1})
	if store.interrupt != nil {
		store.interrupt("committed")
	}
	if err := store.cleanupRecoveryArtifacts(); err != nil {
		return &PendingCleanupError{Cause: err}
	}
	return nil
}

func (store *EventStore) afterPhase(ctx context.Context, phase string, progress func(StoreProgress)) error {
	reportStoreProgress(progress, StoreProgress{Phase: phase, Current: 1, Total: 1})
	if store.interrupt != nil {
		if err := store.interrupt(phase); err != nil {
			return errors.Join(err, store.rollbackLocked())
		}
	}
	if err := ctx.Err(); err != nil {
		return errors.Join(err, store.rollbackLocked())
	}
	return nil
}

func (store *EventStore) writeStage(stage appendStage) error {
	encoded, err := json.Marshal(stage)
	if err != nil {
		return err
	}
	next := store.stagePath() + ".next"
	if err := store.writeSyncedExclusive(next, append(encoded, '\n')); err != nil {
		return fmt.Errorf("%w: write stage: %w", ErrStoreUnavailable, err)
	}
	if err := store.renameDurably(next, store.stagePath()); err != nil {
		return fmt.Errorf("%w: activate stage: %w", ErrStoreUnavailable, err)
	}
	return nil
}

func (store *EventStore) writeSyncedExclusive(path string, chunks ...[]byte) (returnErr error) {
	if err := store.removeDurably(path); err != nil {
		return err
	}
	file, err := store.operations.OpenFile(path, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0o600)
	if err != nil {
		return err
	}
	succeeded := false
	closed := false
	defer func() {
		if !succeeded {
			var closeErr error
			if !closed {
				closeErr = file.Close()
			}
			returnErr = errors.Join(returnErr, closeErr, store.operations.Remove(path))
		}
	}()
	for _, chunk := range chunks {
		written, writeErr := file.Write(chunk)
		if writeErr != nil || written != len(chunk) {
			if writeErr == nil {
				writeErr = io.ErrShortWrite
			}
			return writeErr
		}
	}
	if err := file.Sync(); err != nil {
		return err
	}
	closeErr := file.Close()
	closed = true
	if closeErr != nil {
		return closeErr
	}
	if err := store.operations.SyncParent(path); err != nil {
		return err
	}
	succeeded = true
	return nil
}

func (store *EventStore) renameDurably(source string, destination string) error {
	if err := store.operations.Rename(source, destination); err != nil {
		return err
	}
	return store.operations.SyncParent(destination)
}

func (store *EventStore) removeDurably(path string) error {
	if _, err := store.operations.Lstat(path); os.IsNotExist(err) {
		return nil
	} else if err != nil {
		return err
	}
	if err := store.operations.Remove(path); err != nil {
		return err
	}
	return store.operations.SyncParent(path)
}

// #endregion 🗄️EventStore
