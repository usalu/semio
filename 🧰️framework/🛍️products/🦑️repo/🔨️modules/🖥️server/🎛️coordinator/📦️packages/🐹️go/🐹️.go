// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// The repo coordinator: an owned append-only event store, its crash-recovery protocol, the
// event-sourced repository and its replayed projections, and the HTTP surface the repo CLI speaks to.

// #endregion 🧲️Header

// Package coordinator owns the repo coordinator service: storage, projections and HTTP.
package coordinator

import (
	"bufio"
	"bytes"
	"context"
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log"
	"math/rand"
	"net"
	"net/http"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"sync"
	"time"

	repopkg "github.com/usalu/semio/repo/events"
	langpkg "github.com/usalu/semio/repo/languages"
)

// #region 🛡️Durability
var ErrDurabilityUnsupported = errors.New("store metadata durability unsupported")

type storeFile interface {
	io.Reader
	io.Writer
	Stat() (os.FileInfo, error)
	Sync() error
	Truncate(int64) error
	Close() error
}

type storeOperations interface {
	MkdirAll(string, os.FileMode) error
	Open(string) (storeFile, error)
	OpenFile(string, int, os.FileMode) (storeFile, error)
	ReadFile(string) ([]byte, error)
	Lstat(string) (os.FileInfo, error)
	Stat(string) (os.FileInfo, error)
	Rename(string, string) error
	Remove(string) error
	SyncParent(string) error
	Chtimes(string, time.Time, time.Time) error
}

type nativeStoreOperations struct{}

func (nativeStoreOperations) MkdirAll(path string, mode os.FileMode) error {
	return os.MkdirAll(path, mode)
}

func (nativeStoreOperations) Open(path string) (storeFile, error) {
	return os.Open(path)
}

func (nativeStoreOperations) OpenFile(path string, flag int, mode os.FileMode) (storeFile, error) {
	return os.OpenFile(path, flag, mode)
}

func (nativeStoreOperations) ReadFile(path string) ([]byte, error) {
	return os.ReadFile(path)
}

func (nativeStoreOperations) Lstat(path string) (os.FileInfo, error) {
	return os.Lstat(path)
}

func (nativeStoreOperations) Stat(path string) (os.FileInfo, error) {
	return os.Stat(path)
}

func (nativeStoreOperations) Rename(source string, destination string) error {
	return renameStorePath(source, destination)
}

func (nativeStoreOperations) Remove(path string) error {
	err := os.Remove(path)
	if os.IsNotExist(err) {
		return nil
	}
	return err
}

func (nativeStoreOperations) SyncParent(path string) error {
	parent := filepath.Dir(path)
	if err := syncStoreParent(parent); err != nil {
		return fmt.Errorf("%w: %s: %v", ErrDurabilityUnsupported, parent, err)
	}
	return nil
}

func (nativeStoreOperations) Chtimes(path string, access time.Time, modified time.Time) error {
	return os.Chtimes(path, access, modified)
}

// 💥️ErrInjectedFault is the failure a store armed for fault injection raises.
var ErrInjectedFault = errors.New("injected durability fault")

// 💥️faultStoreOperations fails the armed durable mutation and passes every other call through.
type faultStoreOperations struct {
	inner storeOperations
	mu    sync.Mutex
	armed int
	seen  int
}

func (operations *faultStoreOperations) arm(at int) {
	operations.mu.Lock()
	operations.armed = at
	operations.seen = 0
	operations.mu.Unlock()
}

func (operations *faultStoreOperations) mutate(path string) error {
	operations.mu.Lock()
	defer operations.mu.Unlock()
	if operations.armed <= 0 || strings.HasSuffix(path, ".lock") {
		return nil
	}
	operations.seen++
	if operations.seen == operations.armed {
		return fmt.Errorf("%w at mutation %d on %s", ErrInjectedFault, operations.seen, path)
	}
	return nil
}

func (operations *faultStoreOperations) MkdirAll(path string, mode os.FileMode) error {
	return operations.inner.MkdirAll(path, mode)
}

func (operations *faultStoreOperations) Open(path string) (storeFile, error) {
	return operations.inner.Open(path)
}

func (operations *faultStoreOperations) OpenFile(path string, flag int, mode os.FileMode) (storeFile, error) {
	if flag&os.O_CREATE != 0 {
		if err := operations.mutate(path); err != nil {
			return nil, err
		}
	}
	return operations.inner.OpenFile(path, flag, mode)
}

func (operations *faultStoreOperations) ReadFile(path string) ([]byte, error) {
	return operations.inner.ReadFile(path)
}

func (operations *faultStoreOperations) Lstat(path string) (os.FileInfo, error) {
	return operations.inner.Lstat(path)
}

func (operations *faultStoreOperations) Stat(path string) (os.FileInfo, error) {
	return operations.inner.Stat(path)
}

func (operations *faultStoreOperations) Rename(source string, destination string) error {
	if err := operations.mutate(destination); err != nil {
		return err
	}
	return operations.inner.Rename(source, destination)
}

func (operations *faultStoreOperations) Remove(path string) error {
	if err := operations.mutate(path); err != nil {
		return err
	}
	return operations.inner.Remove(path)
}

func (operations *faultStoreOperations) SyncParent(path string) error {
	return operations.inner.SyncParent(path)
}

func (operations *faultStoreOperations) Chtimes(path string, access time.Time, modified time.Time) error {
	return operations.inner.Chtimes(path, access, modified)
}

// #endregion 🛡️Durability

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

// 🔏️EventChecksum is the SHA-256 identity of a record over its NUL-separated header and canonical payload.
func EventChecksum(event EventEnvelope) string {
	return eventChecksum(event)
}

// 🧾️EncodeEvent renders one envelope as its canonical line, terminated by the record separator.
func EncodeEvent(event EventEnvelope) ([]byte, error) {
	return encodeEvent(event)
}

// ♻️CanonicalPayload normalises a payload the way an append persists it.
func CanonicalPayload(payload []byte) (json.RawMessage, error) {
	return canonicalJSONPayload(payload, DefaultStoreLimits().MaxJSONDepth)
}

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

// #endregion 🗄️EventStore

// #region 🧠️Projection
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

// #endregion 🧠️Projection

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

// #region 🌐️Http
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
func LoadConfig() Config {
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
	Emoji     string     `json:"emoji"`
	Prompt    string     `json:"prompt"`
	Summary   string     `json:"summary"`
	LLM       string     `json:"llm"`
	Client    string     `json:"client"`
	Author    string     `json:"author"`
	GitHub    string     `json:"github_issue"`
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
	Column     *int       `json:"column"`
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
	Type      string    `json:"type"`
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
	TicketID    string `json:"ticket_id"`
	Title       string `json:"title"`
	Prompt      string `json:"prompt"`
	LLM         string `json:"llm"`
	Client      string `json:"client"`
	Author      string `json:"author"`
	GitHubIssue string `json:"github_issue"`
}

// 📨️TicketCloseRequest is the JSON payload for closing a ticket.
type TicketCloseRequest struct {
	TicketID string   `json:"ticket_id"`
	Summary  string   `json:"summary"`
	Files    []string `json:"files"`
}

// 🔓️TicketReopenRequest is the JSON payload for reopening a closed ticket.
type TicketReopenRequest struct {
	TicketID string `json:"ticket_id"`
	Prompt   string `json:"prompt"`
	LLM      string `json:"llm"`
	Title    string `json:"title"`
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
	FilePath string `json:"file_path"`
	Content  string `json:"content"`
}

// Asynchronous in-process event bus for decoupled event publishing and subscription. MUST persist events to the database before dispatching.

// 🎯️EventHandler is a callback invoked when an event of a subscribed type is published.
type EventHandler func(context.Context, Event) error

type eventDispatch struct {
	ctx    context.Context
	event  Event
	result chan error
}

// 📡️EventBus is a buffered channel-based event dispatcher with persistent storage.
type EventBus struct {
	ch       chan eventDispatch
	handlers map[string][]EventHandler
	db       CoordinatorRepository
	ctx      context.Context
	cancel   context.CancelFunc
	wg       sync.WaitGroup
}

// 🗄️NewEventBus creates a new event bus backed by the given database.
// 🆕️MUST initialize the channel buffer to 256 and create a cancellable context.
func NewEventBus(db CoordinatorRepository) *EventBus {
	ctx, cancel := context.WithCancel(context.Background())
	return &EventBus{
		ch:       make(chan eventDispatch, 256),
		handlers: map[string][]EventHandler{},
		db:       db,
		ctx:      ctx,
		cancel:   cancel,
	}
}

// 🏷️Subscribe registers a handler for the given event type.
// ➕️MUST append the handler to the handlers map.
func (b *EventBus) Subscribe(eventType string, handler EventHandler) {
	b.handlers[eventType] = append(b.handlers[eventType], handler)
}

// 📬️Publish persists an event and dispatches it to subscribers.
// 💾️MUST store the event in the database before sending to the channel.
func (b *EventBus) Publish(ctx context.Context, eventType string, source string, payload interface{}) error {
	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		return err
	}
	event := Event{
		ID:        newID(),
		Type:      eventType,
		Source:    source,
		Payload:   string(payloadBytes),
		CreatedAt: time.Now().UTC(),
	}
	if err := b.db.recordPublishedEvent(ctx, event); err != nil {
		return err
	}
	dispatch := eventDispatch{ctx: ctx, event: event, result: make(chan error, 1)}
	select {
	case b.ch <- dispatch:
	case <-b.ctx.Done():
		return errors.New("event bus closed")
	case <-ctx.Done():
		return ctx.Err()
	}
	select {
	case err := <-dispatch.result:
		return err
	case <-b.ctx.Done():
		return errors.New("event bus closed")
	case <-ctx.Done():
		return ctx.Err()
	}
}

// ▶️Start launches the event dispatch goroutine.
// ▶️MUST consume events from the channel and invoke registered handlers.
func (b *EventBus) Start() {
	b.wg.Add(1)
	go func() {
		defer b.wg.Done()
		for {
			select {
			case dispatch := <-b.ch:
				var handlerErr error
				if handlers := b.handlers[dispatch.event.Type]; len(handlers) > 0 {
					for _, handler := range handlers {
						handlerErr = errors.Join(handlerErr, handler(dispatch.ctx, dispatch.event))
					}
				}
				dispatch.result <- handlerErr
			case <-b.ctx.Done():
				return
			}
		}
	}()
}

// ⏹️Stop cancels the event bus context and waits for the dispatch goroutine to finish.
// ⏹️MUST block until the goroutine exits.
func (b *EventBus) Stop() {
	b.cancel()
	b.wg.Wait()
}

// Unified diff parser that extracts file paths and hunk line ranges from patch text. MUST handle standard git diff output format.

// 🧩️hunkHeader is a regex pattern matching unified diff hunk headers.
var hunkHeader = regexp.MustCompile(`@@ -([0-9]+)(?:,([0-9]+))? \+([0-9]+)(?:,([0-9]+))? @@`)

// 🧲️parseUnifiedDiff extracts file paths and hunk ranges from a unified diff patch.
func parseUnifiedDiff(patch string) DiffResult {
	scanner := bufio.NewScanner(strings.NewReader(patch))
	var files []DiffFile
	var current *DiffFile
	for scanner.Scan() {
		line := scanner.Text()
		if strings.HasPrefix(line, "diff --git ") {
			parts := strings.Split(line, " ")
			if len(parts) >= 4 {
				path := strings.TrimPrefix(parts[3], "b/")
				files = append(files, DiffFile{Path: path})
				current = &files[len(files)-1]
			}
			continue
		}
		if strings.HasPrefix(line, "--- ") && current != nil {
			if strings.Contains(line, "/dev/null") {
			}
			continue
		}
		if strings.HasPrefix(line, "+++ ") && current != nil {
			if strings.Contains(line, "/dev/null") {
				current.Deleted = true
			}
			continue
		}
		if strings.HasPrefix(line, "@@ ") && current != nil {
			match := hunkHeader.FindStringSubmatch(line)
			if len(match) >= 5 {
				oldStart := parseHunkInt(match[1])
				oldCount := parseHunkIntWithDefault(match[2], 1)
				newStart := parseHunkInt(match[3])
				newCount := parseHunkIntWithDefault(match[4], 1)
				current.Hunks = append(current.Hunks, DiffHunk{
					OldRange: LineRange{Start: oldStart, End: oldStart + oldCount - 1},
					NewRange: LineRange{Start: newStart, End: newStart + newCount - 1},
				})
			}
		}
	}
	return DiffResult{Files: files}
}

// 🔬️parseHunkInt parses a hunk header integer value.
func parseHunkInt(value string) int {
	parsed, _ := strconv.Atoi(value)
	return parsed
}

// 🔷️parseHunkIntWithDefault parses a hunk header integer or returns the fallback.
func parseHunkIntWithDefault(value string, fallback int) int {
	if value == "" {
		return fallback
	}
	parsed, err := strconv.Atoi(value)
	if err != nil {
		return fallback
	}
	return parsed
}

// Source code indexer that delegates to the shared repo/go parsing package. MUST support region-marker-based sections and language-specific definition patterns.

// 🔭️IndexCache holds in-memory caches of indexed scopes partitioned by file path.
type IndexCache struct {
	Sections    map[string][]Scope
	Definitions map[string][]Scope
	Files       map[string]Scope
}

// 🆕️newIndexCache creates an empty IndexCache with initialized maps.
func newIndexCache() IndexCache {
	return IndexCache{
		Sections:    map[string][]Scope{},
		Definitions: map[string][]Scope{},
		Files:       map[string]Scope{},
	}
}

// 🏗️buildScopesForFile delegates to the shared langpkg.BuildScopesForFile and converts ScopeEntry to Scope.
func buildScopesForFile(path string, content string) []Scope {
	now := time.Now().UTC()
	entries := langpkg.BuildScopesForFile(path, content)
	scopes := make([]Scope, len(entries))
	for i, e := range entries {
		scopes[i] = Scope{
			ID:          e.ID,
			Kind:        e.Kind,
			FilePath:    e.FilePath,
			SectionPath: e.SectionPath,
			Definition:  e.Definition,
			StartLine:   e.StartLine,
			EndLine:     e.EndLine,
			UpdatedAt:   now,
		}
	}
	return scopes
}

// Scope claim mapping logic that associates diff hunks with overlapping scopes. MUST detect multi-ticket conflicts.

// 🔭️mapClaims maps diff hunks to overlapping scopes and returns claimed IDs.
func mapClaims(scopes []Scope, diff DiffResult) ([]string, map[string][]Scope) {
	claimed := map[string][]Scope{}
	var claimedIDs []string
	for _, file := range diff.Files {
		if file.Path == "" {
			continue
		}
		fileScopes := filterScopesByFile(scopes, file.Path)
		for _, hunk := range file.Hunks {
			if hunk.NewRange.End == 0 {
				continue
			}
			for _, scope := range fileScopes {
				if scope.StartLine == 0 && scope.EndLine == 0 {
					continue
				}
				if rangesOverlap(hunk.NewRange, LineRange{Start: scope.StartLine, End: scope.EndLine}) {
					if scope.Kind == "definition" || scope.Kind == "section" {
						claimed[scope.ID] = append(claimed[scope.ID], scope)
						claimedIDs = appendIfMissing(claimedIDs, scope.ID)
					}
				}
			}
		}
	}
	sort.Strings(claimedIDs)
	return claimedIDs, claimed
}

// 🧹️filterScopesByFile returns scopes matching the given file path.
func filterScopesByFile(scopes []Scope, filePath string) []Scope {
	var filtered []Scope
	for _, scope := range scopes {
		if scope.FilePath == filePath {
			filtered = append(filtered, scope)
		}
	}
	return filtered
}

// 🧪️rangesOverlap tests whether two line ranges overlap.
func rangesOverlap(a LineRange, b LineRange) bool {
	if a.Start == 0 || b.Start == 0 {
		return false
	}
	return a.Start <= b.End && b.Start <= a.End
}

// 🔤️appendIfMissing appends a string to a slice only if it is not already present.
func appendIfMissing(list []string, value string) []string {
	for _, item := range list {
		if item == value {
			return list
		}
	}
	return append(list, value)
}

// Conflict warning generation from multi-ticket scope overlaps. MUST produce error-severity warnings for blocking conflicts.

// 💿️buildConflictWarnings creates warning records from detected scope conflicts.
func buildConflictWarnings(conflicts []struct {
	ScopeID string
	Tickets []string
}) []Warning {
	now := time.Now().UTC()
	var warnings []Warning
	for _, conflict := range conflicts {
		message := fmt.Sprintf("conflict on %s across tickets %s", conflict.ScopeID, strings.Join(conflict.Tickets, ", "))
		warnings = append(warnings, Warning{
			ID:        newID(),
			Kind:      "conflict",
			Severity:  "error",
			Message:   message,
			ScopeID:   conflict.ScopeID,
			CreatedAt: now,
		})
	}
	return warnings
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

// 🎫️handleTicketOpen creates a new ticket from the request payload.
func (s *Server) handleTicketOpen(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	var payload TicketOpenRequest
	if err := s.decodeJSON(r, &payload); err != nil {
		s.respondError(w, http.StatusBadRequest, err.Error())
		return
	}
	if payload.TicketID == "" || payload.Title == "" {
		s.respondError(w, http.StatusBadRequest, "ticket_id and title required")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	now := time.Now().UTC()
	ticket := Ticket{
		ID:        payload.TicketID,
		Status:    "open",
		Title:     payload.Title,
		Prompt:    payload.Prompt,
		LLM:       payload.LLM,
		Client:    payload.Client,
		Author:    payload.Author,
		GitHub:    payload.GitHubIssue,
		CreatedAt: now,
	}
	if err := s.db.recordTicket(ctx, ticket); err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	if err := s.bus.Publish(ctx, "TicketOpened", "repo-cli", ticket); err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, ticket)
}

// 📪️handleTicketClose closes an existing ticket with a summary.
func (s *Server) handleTicketClose(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	var payload TicketCloseRequest
	if err := s.decodeJSON(r, &payload); err != nil {
		s.respondError(w, http.StatusBadRequest, err.Error())
		return
	}
	if payload.TicketID == "" || payload.Summary == "" {
		s.respondError(w, http.StatusBadRequest, "ticket_id and summary required")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	ticket, err := s.db.projectTicket(ctx, payload.TicketID)
	if err != nil {
		s.respondError(w, http.StatusNotFound, err.Error())
		return
	}
	now := time.Now().UTC()
	ticket.Status = "closed"
	ticket.Summary = payload.Summary
	ticket.ClosedAt = &now
	if err := s.db.recordTicket(ctx, *ticket); err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	if err := s.bus.Publish(ctx, "TicketClosed", "repo-cli", ticket); err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, ticket)
}

// 🔓️handleTicketReopen reopens a closed ticket with a new prompt.
func (s *Server) handleTicketReopen(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	var payload TicketReopenRequest
	if err := s.decodeJSON(r, &payload); err != nil {
		s.respondError(w, http.StatusBadRequest, err.Error())
		return
	}
	if payload.TicketID == "" || payload.Prompt == "" {
		s.respondError(w, http.StatusBadRequest, "ticket_id and prompt required")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	ticket, err := s.db.projectTicket(ctx, payload.TicketID)
	if err != nil {
		s.respondError(w, http.StatusNotFound, err.Error())
		return
	}
	ticket.Status = "open"
	ticket.Prompt = payload.Prompt
	ticket.LLM = payload.LLM
	if payload.Title != "" {
		ticket.Title = payload.Title
	}
	ticket.ClosedAt = nil
	if err := s.db.recordTicket(ctx, *ticket); err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	if err := s.bus.Publish(ctx, "TicketReopened", "repo-cli", ticket); err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, ticket)
}

// 🧹️handleTicketsQuery lists tickets optionally filtered by status.
func (s *Server) handleTicketsQuery(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	status := r.URL.Query().Get("status")
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	tickets, err := s.db.projectTickets(ctx, status)
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, tickets)
}

// 🧲️handleTicketDetail returns a single ticket by its path-extracted ID.
func (s *Server) handleTicketDetail(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	path := strings.TrimPrefix(r.URL.Path, "/ticket/")
	if path == "" {
		s.respondError(w, http.StatusNotFound, "ticket not found")
		return
	}
	if strings.HasSuffix(path, "/claims") {
		s.handleTicketClaims(w, r)
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	ticket, err := s.db.projectTicket(ctx, path)
	if err != nil {
		s.respondError(w, http.StatusNotFound, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, ticket)
}

// 🔭️handleTicketClaims returns scope claims for a ticket.
func (s *Server) handleTicketClaims(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	path := strings.TrimSuffix(strings.TrimPrefix(r.URL.Path, "/ticket/"), "/claims")
	if path == "" {
		s.respondError(w, http.StatusNotFound, "ticket not found")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	claims, err := s.db.projectClaimsByTicket(ctx, path)
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, claims)
}

// ♻️handleDiffIngest ingests a diff patch, indexes changed files, maps claims, and returns results.
func (s *Server) handleDiffIngest(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	var payload DiffIngestRequest
	if err := s.decodeJSON(r, &payload); err != nil {
		s.respondError(w, http.StatusBadRequest, err.Error())
		return
	}
	if payload.TicketID == "" || payload.Patch == "" {
		s.respondError(w, http.StatusBadRequest, "ticket_id and patch required")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	result, warnings, breachs, err := s.processDiff(ctx, payload.TicketID, payload.Patch, payload.Snapshots)
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	response := DiffIngestResponse{
		ChangedFiles:  result.ChangedFiles,
		ClaimedScopes: result.ClaimedScopes,
		Warnings:      warnings,
		Breachs:       breachs,
		Blockers:      result.Blockers,
	}
	s.writeJSON(w, http.StatusOK, response)
}

// 🎯️handleReindex walks the repo and re-indexes all files.
func (s *Server) handleReindex(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	files, err := s.walkRepoFiles()
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	for _, file := range files {
		content, err := os.ReadFile(filepath.Join(s.config.RepoRoot, file))
		if err != nil {
			continue
		}
		if err := s.updateIndexForFile(ctx, file, string(content)); err != nil {
			s.respondError(w, http.StatusInternalServerError, err.Error())
			return
		}
	}
	s.writeJSON(w, http.StatusOK, map[string]int{"files": len(files)})
}

// 📄️handleIndexFile indexes a single file from the request payload.
func (s *Server) handleIndexFile(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	var payload IndexFileRequest
	if err := s.decodeJSON(r, &payload); err != nil {
		s.respondError(w, http.StatusBadRequest, err.Error())
		return
	}
	if payload.FilePath == "" {
		s.respondError(w, http.StatusBadRequest, "file_path required")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	if err := s.updateIndexForFile(ctx, payload.FilePath, payload.Content); err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

// ⚠️handleWarnings returns warnings optionally filtered by ticket ID.
func (s *Server) handleWarnings(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	warnings, err := s.db.projectWarnings(ctx, r.URL.Query().Get("ticket_id"))
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, warnings)
}

// 🔷️handleBreachs returns breachs optionally filtered by ticket ID.
func (s *Server) handleBreachs(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	breachs, err := s.db.projectBreachs(ctx, r.URL.Query().Get("ticket_id"))
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, breachs)
}

// 🔍️handleScopes returns scopes for a given file query parameter.
func (s *Server) handleScopes(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	filePath := r.URL.Query().Get("file")
	if filePath == "" {
		s.respondError(w, http.StatusBadRequest, "file query required")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	scopes, err := s.db.projectScopesByFile(ctx, filePath)
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, scopes)
}

// Diff processing pipeline that indexes changed files, maps claims, detects conflicts, and produces warnings. MUST be transactional per request.

// 🔷️ProcessResult holds the outcome of a diff processing operation.
type ProcessResult struct {
	ChangedFiles  []string
	ClaimedScopes []string
	Blockers      []string
}

// ♻️processDiff parses the patch, indexes changed files, maps claims, and detects conflicts.
// ⚠️MUST return warnings and breachs alongside the processing result.
func (s *Server) processDiff(ctx context.Context, ticketID string, patch string, snapshots []FileSnapshot) (ProcessResult, []Warning, []Breach, error) {
	diff := parseUnifiedDiff(patch)
	changedFiles := uniqueFiles(diff.Files)
	if err := s.bus.Publish(ctx, "DiffIngested", "repo-cli", map[string]interface{}{"ticket_id": ticketID, "files": changedFiles}); err != nil {
		return ProcessResult{}, nil, nil, err
	}
	contentByFile := snapshotMap(snapshots)
	for _, file := range changedFiles {
		content, ok := contentByFile[file]
		if !ok {
			data, err := os.ReadFile(filepath.Join(s.config.RepoRoot, file))
			if err != nil {
				continue
			}
			content = string(data)
		}
		if err := s.updateIndexForFile(ctx, file, content); err != nil {
			return ProcessResult{}, nil, nil, err
		}
	}
	s.cacheLock.RLock()
	var scopes []Scope
	for _, file := range changedFiles {
		scopes = append(scopes, s.cache.Sections[file]...)
		scopes = append(scopes, s.cache.Definitions[file]...)
	}
	s.cacheLock.RUnlock()
	now := time.Now().UTC()
	claimedIDs, _ := mapClaims(scopes, diff)
	for _, scopeID := range claimedIDs {
		if err := s.db.recordClaim(ctx, ticketID, scopeID, "touched", now); err != nil {
			return ProcessResult{}, nil, nil, err
		}
	}
	conflicts, err := s.db.projectConflicts(ctx)
	if err != nil {
		return ProcessResult{}, nil, nil, err
	}
	warnings := buildConflictWarnings(conflicts)
	if err := s.db.recordWarnings(ctx, warnings); err != nil {
		return ProcessResult{}, nil, nil, err
	}
	blockers := []string{}
	for _, warning := range warnings {
		if warning.Severity == "error" {
			blockers = append(blockers, warning.Message)
		}
	}
	result := ProcessResult{
		ChangedFiles:  changedFiles,
		ClaimedScopes: claimedIDs,
		Blockers:      blockers,
	}
	return result, warnings, []Breach{}, nil
}

// 🧲️uniqueFiles extracts deduplicated file paths from a diff result.
func uniqueFiles(files []DiffFile) []string {
	var list []string
	for _, file := range files {
		if file.Path != "" {
			list = appendIfMissing(list, file.Path)
		}
	}
	return list
}

// 📸️snapshotMap converts a slice of file snapshots into a path-to-content map.
func snapshotMap(snapshots []FileSnapshot) map[string]string {
	mapping := map[string]string{}
	for _, snapshot := range snapshots {
		mapping[snapshot.Path] = snapshot.Content
	}
	return mapping
}

// 🗄️updateIndexForFile builds scopes from file content and updates both the database and cache.
func (s *Server) updateIndexForFile(ctx context.Context, filePath string, content string) error {
	scopes := buildScopesForFile(filePath, content)
	var fileScope Scope
	var sections []Scope
	var definitions []Scope
	for _, scope := range scopes {
		if scope.Kind == "file" {
			fileScope = scope
		}
		if scope.Kind == "section" {
			sections = append(sections, scope)
		}
		if scope.Kind == "definition" {
			definitions = append(definitions, scope)
		}
	}
	if err := s.db.recordScopes(ctx, filePath, scopes); err != nil {
		return err
	}
	if err := s.bus.Publish(ctx, "IndexUpdated", "server", map[string]interface{}{"file": filePath}); err != nil {
		return err
	}
	s.cacheLock.Lock()
	s.cache.Files[filePath] = fileScope
	s.cache.Sections[filePath] = sections
	s.cache.Definitions[filePath] = definitions
	s.cacheLock.Unlock()
	return nil
}

// 📄️walkRepoFiles walks the repo root and returns all non-hidden file paths.
func (s *Server) walkRepoFiles() ([]string, error) {
	var files []string
	err := filepath.Walk(s.config.RepoRoot, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() {
			if strings.HasPrefix(info.Name(), ".") && info.Name() != "." {
				return filepath.SkipDir
			}
			return nil
		}
		rel, err := filepath.Rel(s.config.RepoRoot, path)
		if err != nil {
			return err
		}
		files = append(files, filepath.ToSlash(rel))
		return nil
	})
	if err != nil {
		return nil, err
	}
	return files, nil
}

// #endregion 🌐️Http

// #region 🪝️Webhooks
// GitHub webhook handlers for issue comment caching and issue event processing. MUST verify HMAC signatures when a secret is configured.

// 💬️GitHubComment stores a cached GitHub issue comment for correlating close/reopen events.
type GitHubComment struct {
	Body   string
	Actor  string
	Repo   string
	Issue  int
	Second time.Time
}

// 🐙️handleGitHubWebhook processes incoming GitHub webhook events.
func (s *Server) handleGitHubWebhook(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	body, err := io.ReadAll(r.Body)
	if err != nil {
		s.respondError(w, http.StatusBadRequest, err.Error())
		return
	}
	if s.config.GitHubSecret != "" {
		signature := r.Header.Get("X-Hub-Signature-256")
		if !verifyGitHubSignature(body, signature, s.config.GitHubSecret) {
			s.respondError(w, http.StatusUnauthorized, "invalid signature")
			return
		}
	}
	eventType := r.Header.Get("X-GitHub-Event")
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	if err := s.bus.Publish(ctx, "GitHubIssueEventReceived", "github", map[string]interface{}{"type": eventType}); err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	if eventType == "issue_comment" {
		var payload map[string]interface{}
		_ = json.Unmarshal(body, &payload)
		s.cacheGitHubComment(payload)
	}
	if eventType == "issues" {
		var payload map[string]interface{}
		_ = json.Unmarshal(body, &payload)
		if err := s.handleGitHubIssueEvent(ctx, payload); err != nil {
			s.respondError(w, http.StatusInternalServerError, err.Error())
			return
		}
	}
	if eventType == "push" {
		var payload map[string]interface{}
		_ = json.Unmarshal(body, &payload)
		if err := s.handleGitHubPushEvent(ctx, payload); err != nil {
			s.respondError(w, http.StatusInternalServerError, err.Error())
			return
		}
	}
	w.WriteHeader(http.StatusOK)
}

// 📦️verifyGitHubSignature validates the HMAC-SHA256 signature of a webhook payload.
func verifyGitHubSignature(body []byte, signature string, secret string) bool {
	parts := strings.SplitN(signature, "=", 2)
	if len(parts) != 2 {
		return false
	}
	mac := hmac.New(sha256.New, []byte(secret))
	mac.Write(body)
	computed := hex.EncodeToString(mac.Sum(nil))
	return hmac.Equal([]byte(computed), []byte(parts[1]))
}

// 📡️cacheGitHubComment stores a GitHub comment for correlating subsequent events.
func (s *Server) cacheGitHubComment(payload map[string]interface{}) {
	issue, repo, actor, body := extractIssueComment(payload)
	if issue == 0 || repo == "" || actor == "" || body == "" {
		return
	}
	key := fmt.Sprintf("%s#%d#%s", repo, issue, actor)
	s.ghLock.Lock()
	s.githubCache[key] = GitHubComment{
		Body:   body,
		Actor:  actor,
		Repo:   repo,
		Issue:  issue,
		Second: time.Now().UTC(),
	}
	s.ghLock.Unlock()
}

// 🔓️handleGitHubIssueEvent processes GitHub issue close/reopen events.
func (s *Server) handleGitHubIssueEvent(ctx context.Context, payload map[string]interface{}) error {
	action, _ := payload["action"].(string)
	issueNumber := extractIssueNumber(payload)
	repo := extractRepoFullName(payload)
	actor := extractActorLogin(payload)
	if issueNumber == 0 || repo == "" || actor == "" {
		return nil
	}
	comment := s.findCachedComment(repo, issueNumber, actor)
	if action == "closed" && comment.Body != "" {
		if err := s.bus.Publish(ctx, "TicketClosed", "github", map[string]interface{}{"issue": issueNumber, "comment": comment.Body}); err != nil {
			return err
		}
	}
	if action == "reopened" && comment.Body != "" {
		if err := s.bus.Publish(ctx, "TicketReopened", "github", map[string]interface{}{"issue": issueNumber, "comment": comment.Body}); err != nil {
			return err
		}
	}
	return nil
}

// 💾️findCachedComment retrieves a recently cached GitHub comment for the given issue.
func (s *Server) findCachedComment(repo string, issue int, actor string) GitHubComment {
	key := fmt.Sprintf("%s#%d#%s", repo, issue, actor)
	s.ghLock.Lock()
	defer s.ghLock.Unlock()
	comment, ok := s.githubCache[key]
	if !ok {
		return GitHubComment{}
	}
	if time.Since(comment.Second) > 90*time.Second {
		delete(s.githubCache, key)
		return GitHubComment{}
	}
	return comment
}

// 🧲️extractIssueComment extracts issue number, repo, actor, and body from a webhook payload.
func extractIssueComment(payload map[string]interface{}) (int, string, string, string) {
	issueNumber := extractIssueNumber(payload)
	repo := extractRepoFullName(payload)
	actor := extractActorLogin(payload)
	body := ""
	if comment, ok := payload["comment"].(map[string]interface{}); ok {
		body, _ = comment["body"].(string)
	}
	return issueNumber, repo, actor, body
}

// 🔢️extractIssueNumber extracts the issue number from a GitHub webhook payload.
func extractIssueNumber(payload map[string]interface{}) int {
	if issue, ok := payload["issue"].(map[string]interface{}); ok {
		if number, ok := issue["number"].(float64); ok {
			return int(number)
		}
	}
	return 0
}

// 🔷️extractRepoFullName extracts the repository full name from a GitHub webhook payload.
func extractRepoFullName(payload map[string]interface{}) string {
	if repo, ok := payload["repository"].(map[string]interface{}); ok {
		if name, ok := repo["full_name"].(string); ok {
			return name
		}
	}
	return ""
}

// 💿️handleGitHubPushEvent holds the data fields for a handleGitHubPushEvent record.
func (s *Server) handleGitHubPushEvent(ctx context.Context, payload map[string]interface{}) error {
	actor := extractActorLogin(payload)
	if actor == "" {
		if pusher, ok := payload["pusher"].(map[string]interface{}); ok {
			if name, ok := pusher["name"].(string); ok {
				actor = name
			}
		}
	}
	var files []string
	if checkpoints, ok := payload["commits"].([]interface{}); ok {
		for _, c := range checkpoints {
			if cm, ok := c.(map[string]interface{}); ok {
				if added, ok := cm["added"].([]interface{}); ok {
					for _, a := range added {
						if p, ok := a.(string); ok {
							files = append(files, p)
						}
					}
				}
				if modified, ok := cm["modified"].([]interface{}); ok {
					for _, m := range modified {
						if p, ok := m.(string); ok {
							files = append(files, p)
						}
					}
				}
			}
		}
	}
	if actor != "" && len(files) > 0 {
		if err := s.db.recordCheckpoint(ctx, actor, files); err != nil {
			return err
		}
	}
	return nil
}

// 📤️extractActorLogin extracts the sender login from a GitHub webhook payload.
func extractActorLogin(payload map[string]interface{}) string {
	if sender, ok := payload["sender"].(map[string]interface{}); ok {
		if login, ok := sender["login"].(string); ok {
			return login
		}
	}
	return ""
}

// ⚙️Discord notification integration for ticket lifecycle events. MUST silently skip when no webhook URL is configured.
func (s *Server) notifyDiscord(title string, body string) {
	if s.config.DiscordWebhook == "" {
		return
	}
	payload := map[string]string{"content": fmt.Sprintf("%s\n%s", title, body)}
	data, _ := json.Marshal(payload)
	request, err := http.NewRequest(http.MethodPost, s.config.DiscordWebhook, strings.NewReader(string(data)))
	if err != nil {
		return
	}
	request.Header.Set("Content-Type", "application/json")
	client := &http.Client{Timeout: 5 * time.Second}
	_, _ = client.Do(request)
}

// 🔔️registerNotifications subscribes to ticket lifecycle events and sends Discord notifications.
func (s *Server) registerNotifications() {
	s.bus.Subscribe("TicketOpened", func(ctx context.Context, event Event) error {
		s.notifyDiscord("# Prompt", event.Payload)
		return nil
	})
	s.bus.Subscribe("TicketClosed", func(ctx context.Context, event Event) error {
		s.notifyDiscord("# Summary", event.Payload)
		return nil
	})
	s.bus.Subscribe("TicketReopened", func(ctx context.Context, event Event) error {
		s.notifyDiscord("# Prompt", event.Payload)
		return nil
	})
	for _, kind := range []repopkg.EventKind{
		repopkg.EventTicketOpenEnded, repopkg.EventTicketCloseEnded, repopkg.EventTicketReopenEnded, repopkg.EventTicketChangeEnded,
		repopkg.EventGoalOpenEnded, repopkg.EventGoalCloseEnded, repopkg.EventGoalReopenEnded, repopkg.EventGoalChangeEnded,
		repopkg.EventContributorAddEnded, repopkg.EventContributorRemoveEnded,
		repopkg.EventTodoCreateEnded, repopkg.EventTodoChangeEnded, repopkg.EventTodoDeleteEnded,
	} {
		k := kind
		s.bus.Subscribe(string(k), func(ctx context.Context, event Event) error {
			return s.onCLIEvent(ctx, k, event)
		})
	}
	s.bus.Subscribe(string(repopkg.EventCheckpointEnded), func(ctx context.Context, event Event) error {
		return s.onCheckpointEvent(ctx, event)
	})
}

// 💿️onCLIEvent holds the data fields for a onCLIEvent record.
func (s *Server) onCLIEvent(ctx context.Context, kind repopkg.EventKind, event Event) error {
	author, items := s.extractAuthorAndItems(kind, event.Payload)
	if author == "" {
		return nil
	}
	for _, item := range items {
		if item.Kind == "" || item.ID == "" {
			continue
		}
		others, err := s.db.projectContributorsOnItem(ctx, item.Kind, item.ID)
		if err != nil {
			return err
		}
		others = filterOut(others, author)
		if err := s.db.recordContributorWork(ctx, author, item.Kind, item.ID); err != nil {
			return err
		}
		s.notifyDiscord(string(kind), event.Payload)
		if len(others) > 0 {
			s.notifyDiscord("⚠️ Conflict", fmt.Sprintf("%s working on %s:%s (others: %v)", author, item.Kind, item.ID, others))
		}
	}
	return nil
}

// 💾️onCheckpointEvent holds the data fields for a onCheckpointEvent record.
func (s *Server) onCheckpointEvent(ctx context.Context, event Event) error {
	var p repopkg.CheckpointPayload
	if json.Unmarshal([]byte(event.Payload), &p) != nil {
		return errors.New("invalid checkpoint event payload")
	}
	files := p.FilesChanged
	if len(files) == 0 {
		files = p.Files
	}
	return s.db.recordCheckpoint(ctx, p.Author, files)
}

// 🧲️extractAuthorAndItems holds the data fields for a extractAuthorAndItems record.
func (s *Server) extractAuthorAndItems(kind repopkg.EventKind, payloadJSON string) (author string, items []repopkg.WorkItem) {
	switch kind {
	case repopkg.EventTicketOpenEnded, repopkg.EventTicketCloseEnded, repopkg.EventTicketReopenEnded, repopkg.EventTicketChangeEnded:
		var p repopkg.TicketPayload
		if json.Unmarshal([]byte(payloadJSON), &p) != nil {
			return "", nil
		}
		author = getAuthorFromPayload(payloadJSON)
		if author == "" {
			return "", nil
		}
		id := p.ID
		if id == "" && (p.Year|p.Month|p.Day) != 0 {
			id = fmt.Sprintf("%d/%02d/%02d/%s", p.Year, p.Month, p.Day, p.Slug)
		}
		return author, []repopkg.WorkItem{{Kind: "ticket", ID: id}}
	case repopkg.EventGoalOpenEnded, repopkg.EventGoalCloseEnded, repopkg.EventGoalReopenEnded, repopkg.EventGoalChangeEnded:
		var p repopkg.GoalPayload
		if json.Unmarshal([]byte(payloadJSON), &p) != nil {
			return "", nil
		}
		author = getAuthorFromPayload(payloadJSON)
		return author, []repopkg.WorkItem{{Kind: "goal", ID: p.ID}}
	case repopkg.EventContributorAddEnded, repopkg.EventContributorRemoveEnded:
		var p repopkg.ContributorPayload
		if json.Unmarshal([]byte(payloadJSON), &p) != nil {
			return "", nil
		}
		return p.Author, []repopkg.WorkItem{{Kind: "contributor", ID: p.Github}}
	case repopkg.EventTodoCreateEnded, repopkg.EventTodoChangeEnded, repopkg.EventTodoDeleteEnded:
		var p repopkg.TodoPayload
		if json.Unmarshal([]byte(payloadJSON), &p) != nil {
			return "", nil
		}
		return p.Author, []repopkg.WorkItem{{Kind: "todo", ID: p.ID}}
	default:
		return "", nil
	}
}

// 📦️getAuthorFromPayload holds the data fields for a getAuthorFromPayload record.
func getAuthorFromPayload(payloadJSON string) string {
	var m map[string]interface{}
	if json.Unmarshal([]byte(payloadJSON), &m) != nil {
		return ""
	}
	if a, ok := m["author"].(string); ok {
		return a
	}
	return ""
}

// 🧹️filterOut holds the data fields for a filterOut record.
func filterOut(list []string, exclude string) []string {
	var out []string
	for _, x := range list {
		if x != exclude {
			out = append(out, x)
		}
	}
	return out
}

// #endregion 🪝️Webhooks

// #region 🚀️Main

// 🆔️newID mints a monotonic-prefixed unique identifier for an in-process event.
func newID() string {
	return fmt.Sprintf("%d-%d", time.Now().UTC().UnixNano(), rand.Int63())
}

// 🧭️NewMux wires every coordinator route onto a fresh multiplexer.
func NewMux(server *Server) *http.ServeMux {
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
	mux.HandleFunc("/api/v1/events", server.handleEvents)
	mux.HandleFunc("/webhooks/github", server.handleGitHubWebhook)
	return mux
}

// 🛎️Service is one running coordinator, owning its store, bus and listener.
type Service struct {
	Address    string
	repository *EventRepository
	bus        *EventBus
	listener   net.Listener
}

// 🚀️Start opens the store, wires the routes and serves them on the configured address.
func Start(config Config) (*Service, error) {
	repository, err := OpenDatabase(config.DatabasePath)
	if err != nil {
		return nil, err
	}
	bus := NewEventBus(repository)
	server := NewServer(config, repository, bus)
	server.registerNotifications()
	bus.Start()
	listener, err := net.Listen("tcp", config.Address)
	if err != nil {
		bus.Stop()
		return nil, errors.Join(err, repository.Close())
	}
	service := &Service{Address: listener.Addr().String(), repository: repository, bus: bus, listener: listener}
	go func() {
		_ = http.Serve(listener, NewMux(server))
	}()
	return service, nil
}

// 🛑️Stop closes the listener, drains the bus and releases the store.
func (service *Service) Stop() error {
	closeErr := service.listener.Close()
	service.bus.Stop()
	return errors.Join(closeErr, service.repository.Close())
}

// ▶️Main is the coordinator entry point the binary delegates to.
func Main() {
	config := LoadConfig()
	service, err := Start(config)
	if err != nil {
		log.Fatal(err)
	}
	log.Printf("repo coordinator listening on %s", service.Address)
	select {}
}

// #endregion 🚀️Main
