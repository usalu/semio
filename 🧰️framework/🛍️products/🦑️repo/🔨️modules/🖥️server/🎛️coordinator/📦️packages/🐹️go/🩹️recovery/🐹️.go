// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🩹️recovery carries the event store crash-recovery protocol: recovery, rollback, file inspection, bounded replay, truncation and the store lock.

// #endregion 🧲️Header

package coordinator

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
	"time"
)

// #region 🩹️Recovery

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

// #endregion 🩹️Recovery
