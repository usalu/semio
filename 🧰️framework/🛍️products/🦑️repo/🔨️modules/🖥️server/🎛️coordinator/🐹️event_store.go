// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Owned, deterministic, append-only event storage for the coordinator.

// #endregion 🧲️Header

package main

import (
	"bytes"
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"sync"
	"time"
)

// #region 📜️Schema

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

// #endregion 📜️Schema

// #region 🗄️Store

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
	return openEventStoreWithOperations(ctx, path, limits, nativeStoreOperations{})
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

// #endregion 🗄️Store

// #region 💾️Commit

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

// #endregion 💾️Commit

// #region 🛟️Recovery

func (store *EventStore) recoverLocked(ctx context.Context) error {
	store.setRecoveryStatus(StoreRecoveryStatus{})
	if err := store.removeDurably(store.stagePath() + ".next"); err != nil {
		return fmt.Errorf("%w: remove incomplete stage: %w", ErrStoreUnavailable, err)
	}
	data, err := store.operations.ReadFile(store.stagePath())
	if os.IsNotExist(err) {
		if err := store.removeDurably(store.nextPath()); err != nil {
			return fmt.Errorf("%w: remove incomplete append: %w", ErrStoreUnavailable, err)
		}
		backup, inspectErr := store.inspectFile(ctx, store.backupPath())
		if inspectErr != nil {
			return inspectErr
		}
		if backup.exists {
			return fmt.Errorf("%w: orphan backup", ErrStoreCorrupt)
		}
		return nil
	}
	if err != nil {
		return fmt.Errorf("%w: read stage: %v", ErrStoreUnavailable, err)
	}
	var stage appendStage
	if json.Unmarshal(data, &stage) != nil || stage.Schema != eventStoreStageSchema || stage.PriorSize < 0 || stage.NextSize <= stage.PriorSize || stage.PriorChecksum == "" || stage.NextChecksum == "" {
		return fmt.Errorf("%w: invalid append stage", ErrStoreCorrupt)
	}
	current, err := store.inspectFile(ctx, store.path)
	if err != nil {
		return err
	}
	backup, err := store.inspectFile(ctx, store.backupPath())
	if err != nil {
		return err
	}
	if fileMatches(current, stage.NextSize, stage.NextChecksum) {
		if err := store.cleanupRecoveryArtifacts(); err != nil {
			return err
		}
		store.setRecoveryStatus(StoreRecoveryStatus{Recovered: true, Action: "committed-cleanup"})
		return nil
	}
	if fileMatches(backup, stage.PriorSize, stage.PriorChecksum) {
		if err := store.removeDurably(store.path); err != nil {
			return fmt.Errorf("%w: remove incomplete log: %w", ErrStoreUnavailable, err)
		}
		if err := store.renameDurably(store.backupPath(), store.path); err != nil {
			return fmt.Errorf("%w: restore prior: %w", ErrStoreUnavailable, err)
		}
		if err := store.cleanupRecoveryArtifacts(); err != nil {
			return err
		}
		store.setRecoveryStatus(StoreRecoveryStatus{Recovered: true, Action: "prior-restored"})
		return nil
	}
	if stage.PriorExists && fileMatches(current, stage.PriorSize, stage.PriorChecksum) {
		if err := store.cleanupRecoveryArtifacts(); err != nil {
			return err
		}
		store.setRecoveryStatus(StoreRecoveryStatus{Recovered: true, Action: "prior-cleanup"})
		return nil
	}
	if !stage.PriorExists && !current.exists {
		if err := store.cleanupRecoveryArtifacts(); err != nil {
			return err
		}
		store.setRecoveryStatus(StoreRecoveryStatus{Recovered: true, Action: "empty-cleanup"})
		return nil
	}
	return fmt.Errorf("%w: neither committed nor prior log is valid", ErrStoreCorrupt)
}

func (store *EventStore) setRecoveryStatus(status StoreRecoveryStatus) {
	store.recoveryMu.Lock()
	store.recovery = status
	store.recoveryMu.Unlock()
}

func (store *EventStore) rollbackLocked() error {
	data, err := store.operations.ReadFile(store.stagePath())
	if os.IsNotExist(err) {
		return store.removeDurably(store.nextPath())
	}
	if err != nil {
		return err
	}
	var stage appendStage
	if err := json.Unmarshal(data, &stage); err != nil {
		return err
	}
	backup, err := store.inspectFile(context.Background(), store.backupPath())
	if err != nil {
		return err
	}
	if fileMatches(backup, stage.PriorSize, stage.PriorChecksum) {
		if err := store.removeDurably(store.path); err != nil {
			return err
		}
		if err := store.renameDurably(store.backupPath(), store.path); err != nil {
			return err
		}
	} else if !stage.PriorExists {
		if err := store.removeDurably(store.path); err != nil {
			return err
		}
	}
	return store.cleanupRecoveryArtifacts()
}

func (store *EventStore) cleanupRecoveryArtifacts() error {
	for _, path := range []string{store.nextPath(), store.backupPath(), store.stagePath(), store.stagePath() + ".next"} {
		if err := store.removeDurably(path); err != nil {
			return err
		}
	}
	return nil
}

func (store *EventStore) inspectFile(ctx context.Context, path string) (persistedFile, error) {
	file, err := store.operations.Open(path)
	if os.IsNotExist(err) {
		return persistedFile{checksum: bytesChecksum(nil)}, nil
	}
	if err != nil {
		return persistedFile{}, fmt.Errorf("%w: %v", ErrStoreUnavailable, err)
	}
	defer file.Close()
	info, err := file.Stat()
	if err != nil {
		return persistedFile{}, err
	}
	hash := sha256.New()
	buffer := make([]byte, 64*1024)
	for {
		if err := ctx.Err(); err != nil {
			return persistedFile{}, err
		}
		read, readErr := file.Read(buffer)
		if read > 0 {
			_, _ = hash.Write(buffer[:read])
		}
		if errors.Is(readErr, io.EOF) {
			break
		}
		if readErr != nil {
			return persistedFile{}, readErr
		}
	}
	return persistedFile{exists: true, size: info.Size(), checksum: hex.EncodeToString(hash.Sum(nil))}, nil
}

func fileMatches(file persistedFile, size int64, checksum string) bool {
	return file.exists && file.size == size && file.checksum == checksum
}

func (store *EventStore) stagePath() string  { return store.path + ".stage" }
func (store *EventStore) nextPath() string   { return store.path + ".next" }
func (store *EventStore) backupPath() string { return store.path + ".backup" }

// #endregion 🛟️Recovery

// #region ⏪️Replay

func (store *EventStore) replayLocked(ctx context.Context, progress func(StoreProgress), recoverTail bool) ([]EventEnvelope, error) {
	data, err := store.readOptionalBounded(ctx, store.path, store.limits.MaxLogBytes, progress)
	if err != nil {
		return nil, err
	}
	if data == nil || len(data) == 0 {
		return []EventEnvelope{}, nil
	}
	lastNewline := bytes.LastIndexByte(data, '\n')
	if lastNewline != len(data)-1 {
		if !recoverTail {
			return nil, fmt.Errorf("%w: partial tail", ErrStoreCorrupt)
		}
		if err := store.truncateAndSync(store.path, int64(lastNewline+1)); err != nil {
			return nil, err
		}
		data = data[:lastNewline+1]
	}
	lines := bytes.Split(data, []byte{'\n'})
	events := make([]EventEnvelope, 0, len(lines)-1)
	seen := map[string]struct{}{}
	sequences := map[string]uint64{}
	for index, line := range lines[:len(lines)-1] {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		if len(events) >= store.limits.MaxReplayEvents {
			return nil, fmt.Errorf("%w: replay events exceed %d", ErrStoreLimit, store.limits.MaxReplayEvents)
		}
		if len(line) > store.limits.MaxAppendBytes {
			return nil, fmt.Errorf("%w: encoded event bytes %d > %d", ErrStoreLimit, len(line), store.limits.MaxAppendBytes)
		}
		var event EventEnvelope
		if len(line) == 0 || json.Unmarshal(line, &event) != nil {
			return nil, fmt.Errorf("%w: malformed event %d", ErrStoreCorrupt, index+1)
		}
		canonical, err := encodeEvent(event)
		if err != nil || !bytes.Equal(canonical, append(append([]byte(nil), line...), '\n')) {
			return nil, fmt.Errorf("%w: non-canonical event %d", ErrStoreCorrupt, index+1)
		}
		if !validEventScalar(event.Stream) || !validEventScalar(event.ID) || event.Generation == 0 || !validEventScalar(event.Type) || event.Sequence != sequences[event.Stream]+1 || event.Checksum != eventChecksum(event) {
			return nil, fmt.Errorf("%w: invalid event %d", ErrStoreCorrupt, index+1)
		}
		if err := validateJSONPayload(event.Payload, store.limits.MaxJSONDepth); err != nil {
			return nil, fmt.Errorf("%w: invalid payload %d", ErrStoreCorrupt, index+1)
		}
		if _, duplicate := seen[event.ID]; duplicate {
			return nil, fmt.Errorf("%w: %s", ErrDuplicateEvent, event.ID)
		}
		seen[event.ID] = struct{}{}
		sequences[event.Stream] = event.Sequence
		events = append(events, event)
		reportStoreProgress(progress, StoreProgress{Phase: "replayed", Current: len(events), Total: 0})
	}
	return events, nil
}

func (store *EventStore) readOptionalBounded(ctx context.Context, path string, maximum int64, progress func(StoreProgress)) ([]byte, error) {
	file, err := store.operations.Open(path)
	if os.IsNotExist(err) {
		return nil, nil
	}
	if err != nil {
		return nil, fmt.Errorf("%w: %v", ErrStoreUnavailable, err)
	}
	defer file.Close()
	info, err := file.Stat()
	if err != nil {
		return nil, err
	}
	if info.Size() > maximum {
		return nil, fmt.Errorf("%w: log bytes %d > %d", ErrStoreLimit, info.Size(), maximum)
	}
	data := make([]byte, 0, info.Size())
	buffer := make([]byte, 64*1024)
	for {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		read, readErr := file.Read(buffer)
		if read > 0 {
			data = append(data, buffer[:read]...)
			reportStoreProgress(progress, StoreProgress{Phase: "read", Current: len(data), Total: int(info.Size())})
		}
		if errors.Is(readErr, io.EOF) {
			break
		}
		if readErr != nil {
			return nil, fmt.Errorf("%w: %v", ErrStoreUnavailable, readErr)
		}
	}
	return data, nil
}

func (store *EventStore) truncateAndSync(path string, size int64) error {
	file, err := store.operations.OpenFile(path, os.O_RDWR, 0o600)
	if err != nil {
		return fmt.Errorf("%w: %v", ErrStoreUnavailable, err)
	}
	if err := file.Truncate(size); err != nil {
		return errors.Join(err, file.Close())
	}
	if err := file.Sync(); err != nil {
		return errors.Join(err, file.Close())
	}
	return file.Close()
}

// #endregion ⏪️Replay

// #region 🔐️Concurrency

func (store *EventStore) lock(ctx context.Context) (func() error, error) {
	absolute, err := filepath.Abs(store.path)
	if err != nil {
		return nil, err
	}
	candidate := make(chan struct{}, 1)
	candidate <- struct{}{}
	value, _ := eventStoreLocks.LoadOrStore(absolute, candidate)
	semaphore := value.(chan struct{})
	select {
	case <-ctx.Done():
		return nil, ctx.Err()
	case <-semaphore:
	}
	lockPath := store.path + ".lock"
	for {
		file, openErr := store.operations.OpenFile(lockPath, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0o600)
		if openErr == nil {
			if _, err := fmt.Fprintf(file, "%d\n", os.Getpid()); err != nil {
				semaphore <- struct{}{}
				return nil, store.abandonLock(file, lockPath, fmt.Errorf("%w: write lock: %w", ErrStoreUnavailable, err))
			}
			if err := file.Sync(); err != nil {
				semaphore <- struct{}{}
				return nil, store.abandonLock(file, lockPath, fmt.Errorf("%w: sync lock: %w", ErrStoreUnavailable, err))
			}
			if err := file.Close(); err != nil {
				semaphore <- struct{}{}
				return nil, errors.Join(fmt.Errorf("%w: close lock: %w", ErrStoreUnavailable, err), store.operations.Remove(lockPath))
			}
			stopHeartbeat := make(chan struct{})
			heartbeatDone := make(chan error, 1)
			go store.heartbeatStoreLock(lockPath, stopHeartbeat, heartbeatDone)
			return func() error {
				close(stopHeartbeat)
				heartbeatErr := <-heartbeatDone
				removeErr := store.operations.Remove(lockPath)
				semaphore <- struct{}{}
				return errors.Join(heartbeatErr, removeErr)
			}, nil
		}
		if !os.IsExist(openErr) {
			semaphore <- struct{}{}
			return nil, fmt.Errorf("%w: acquire lock: %v", ErrStoreUnavailable, openErr)
		}
		if info, statErr := store.operations.Stat(lockPath); statErr == nil && time.Since(info.ModTime()) > 30*time.Second {
			if err := store.operations.Remove(lockPath); err != nil {
				semaphore <- struct{}{}
				return nil, fmt.Errorf("%w: remove stale lock: %v", ErrStoreUnavailable, err)
			}
			continue
		}
		select {
		case <-ctx.Done():
			semaphore <- struct{}{}
			return nil, ctx.Err()
		case <-time.After(2 * time.Millisecond):
		}
	}
}

func (store *EventStore) abandonLock(file storeFile, path string, cause error) error {
	return errors.Join(cause, file.Close(), store.operations.Remove(path))
}

func (store *EventStore) heartbeatStoreLock(path string, stop <-chan struct{}, done chan<- error) {
	ticker := time.NewTicker(store.heartbeatInterval)
	defer ticker.Stop()
	var heartbeatErr error
	for {
		select {
		case <-stop:
			done <- heartbeatErr
			return
		case now := <-ticker.C:
			if err := store.operations.Chtimes(path, now, now); err != nil && heartbeatErr == nil {
				heartbeatErr = err
			}
		}
	}
}

// #endregion 🔐️Concurrency

// #region 🔢️Encoding

func encodeEventBatch(events []EventEnvelope) ([]byte, error) {
	var output bytes.Buffer
	for _, event := range events {
		line, err := encodeEvent(event)
		if err != nil {
			return nil, err
		}
		output.Write(line)
	}
	return output.Bytes(), nil
}

func encodeEvent(event EventEnvelope) ([]byte, error) {
	encoded, err := json.Marshal(event)
	if err != nil {
		return nil, err
	}
	return append(encoded, '\n'), nil
}

func eventChecksum(event EventEnvelope) string {
	hash := sha256.New()
	fmt.Fprintf(hash, "%s\x00%d\x00%s\x00%d\x00%s\x00", event.Stream, event.Sequence, event.ID, event.Generation, event.Type)
	hash.Write(event.Payload)
	return hex.EncodeToString(hash.Sum(nil))
}

func bytesChecksum(data []byte) string {
	value := sha256.Sum256(data)
	return hex.EncodeToString(value[:])
}

func validEventScalar(value string) bool {
	return value != "" && !bytes.ContainsRune([]byte(value), '\x00')
}

func validateJSONPayload(payload []byte, maximumDepth int) error {
	if len(payload) == 0 || !json.Valid(payload) {
		return errors.New("event payload must be valid JSON")
	}
	depth := 0
	inString := false
	escaped := false
	for _, value := range payload {
		if inString {
			if escaped {
				escaped = false
			} else if value == '\\' {
				escaped = true
			} else if value == '"' {
				inString = false
			}
			continue
		}
		if value == '"' {
			inString = true
			continue
		}
		if value == '{' || value == '[' {
			depth++
			if depth > maximumDepth {
				return fmt.Errorf("%w: JSON depth %d > %d", ErrStoreLimit, depth, maximumDepth)
			}
		} else if value == '}' || value == ']' {
			depth--
		}
	}
	return nil
}

func canonicalJSONPayload(payload []byte, maximumDepth int) (json.RawMessage, error) {
	if err := validateJSONPayload(payload, maximumDepth); err != nil {
		return nil, err
	}
	decoder := json.NewDecoder(bytes.NewReader(payload))
	decoder.UseNumber()
	var value any
	if err := decoder.Decode(&value); err != nil {
		return nil, err
	}
	canonical, err := json.Marshal(value)
	if err != nil {
		return nil, err
	}
	if err := validateJSONPayload(canonical, maximumDepth); err != nil {
		return nil, err
	}
	return canonical, nil
}

func reportStoreProgress(progress func(StoreProgress), value StoreProgress) {
	if progress != nil {
		progress(value)
	}
}

// #endregion 🔢️Encoding
