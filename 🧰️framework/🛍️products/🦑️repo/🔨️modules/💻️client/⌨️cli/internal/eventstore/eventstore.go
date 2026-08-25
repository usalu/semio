// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Package eventstore provides a recoverable append-only log with deterministic replay.

// #endregion 🧲️Header

package eventstore

import (
	"bufio"
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
	"runtime"
	"sync"
)

// #region 📜️Schema

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

type Event struct {
	Schema   string          `json:"schema"`
	Sequence uint64          `json:"sequence"`
	ID       string          `json:"id"`
	Kind     string          `json:"kind"`
	Data     json.RawMessage `json:"data"`
	Checksum string          `json:"checksum"`
}

type Input struct {
	ID   string
	Kind string
	Data interface{}
}

type Progress struct {
	Current int
	Total   int
	Step    string
}

type Store struct{ Path string }

type stage struct {
	Schema        string `json:"schema"`
	PriorExists   bool   `json:"priorExists"`
	PriorSize     int64  `json:"priorSize"`
	PriorChecksum string `json:"priorChecksum"`
	BatchSize     int    `json:"batchSize"`
	BatchChecksum string `json:"batchChecksum"`
}

// #endregion 📜️Schema

// #region ➕️Append

func (store Store) Append(ctx context.Context, inputs []Input, progress func(Progress)) ([]Event, error) {
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
	created := make([]Event, 0, len(inputs))
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
		event := Event{Schema: Schema, Sequence: uint64(len(existing) + index + 1), ID: input.ID, Kind: input.Kind, Data: data}
		event.Checksum = checksum(event)
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
		BatchChecksum: digest(batch.Bytes()),
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

// #endregion ➕️Append

// #region 🛟️Recovery

type priorState struct {
	exists   bool
	size     int64
	checksum string
}

func inspectPrior(path string) (priorState, error) {
	file, err := os.Open(path)
	if os.IsNotExist(err) {
		return priorState{checksum: digest(nil)}, nil
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
		committed = digest(segment) == staged.BatchChecksum
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

// #endregion 🛟️Recovery

// #region ⏪️Replay

func (store Store) Replay(ctx context.Context, progress func(Progress)) ([]Event, error) {
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

func (store Store) replay(ctx context.Context, progress func(Progress)) ([]Event, error) {
	file, err := os.Open(store.Path)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	reader := bufio.NewReaderSize(file, 64*1024)
	var events []Event
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
			var event Event
			if err := json.Unmarshal(line, &event); err != nil {
				return nil, fmt.Errorf("%w at sequence %d: %v", ErrCorrupt, len(events)+1, err)
			}
			expected := uint64(len(events) + 1)
			if event.Schema != Schema || event.Sequence != expected || event.ID == "" || event.Kind == "" || event.Checksum != checksum(event) {
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

func checksum(event Event) string {
	digestValue := sha256.New()
	fmt.Fprintf(digestValue, "%s\x00%d\x00%s\x00%s\x00", event.Schema, event.Sequence, event.ID, event.Kind)
	digestValue.Write(event.Data)
	return hex.EncodeToString(digestValue.Sum(nil))
}

func digest(data []byte) string {
	value := sha256.Sum256(data)
	return hex.EncodeToString(value[:])
}

func report(progress func(Progress), value Progress) {
	if progress != nil {
		progress(value)
	}
}

// #endregion ⏪️Replay
