package search

import (
	"bytes"
	"compress/gzip"
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

const MaxSnapshotBytes = 256 << 20
const MaxSnapshotCompressedBytes = 64 << 20
const snapshotSchema = "semio.search.snapshot/1"

// 🗃️ SnapshotCacheRequest binds one derived projection to its input identity and cancellation.
type SnapshotCacheRequest[T any] struct {
	Directory   string
	Fingerprint string
	Shape       string
	Build       func(context.Context) (T, error)
	Progress    func(Progress)
}
type snapshotMeta struct {
	Schema      string
	Fingerprint string
	Shape       string
	SHA256      string
	Bytes       int64
}
type contextReader struct {
	context context.Context
	reader  io.Reader
}

func (reader contextReader) Read(value []byte) (int, error) {
	if err := reader.context.Err(); err != nil {
		return 0, err
	}
	return reader.reader.Read(value)
}

type snapshotWriter struct {
	context   context.Context
	writer    io.Writer
	remaining int64
}

func (writer *snapshotWriter) Write(value []byte) (int, error) {
	if err := writer.context.Err(); err != nil {
		return 0, err
	}
	if int64(len(value)) > writer.remaining {
		return 0, fmt.Errorf("%w: snapshot byte budget", ErrTooLarge)
	}
	count, err := writer.writer.Write(value)
	writer.remaining -= int64(count)
	return count, err
}

// 🧊️ LoadOrBuildSnapshot retains a kernel lease while reading or publishing a bounded generation.
func LoadOrBuildSnapshot[T any](ctx context.Context, request SnapshotCacheRequest[T]) (value T, failure error) {
	if err := ctx.Err(); err != nil {
		return value, err
	}
	if request.Fingerprint == "" || len(request.Fingerprint) > 256 || len(request.Shape) > 4096 || request.Build == nil {
		return value, errors.New("bounded snapshot request required")
	}
	if err := createSnapshotDirectory(request.Directory); err != nil {
		return value, err
	}
	lease, err := AcquireGenerationLease(ctx, filepath.Join(request.Directory, ".snapshot.lock"), false, request.Progress)
	if err != nil {
		return value, err
	}
	defer func() {
		failure = errors.Join(failure, lease.Close())
		if failure != nil {
			var empty T
			value = empty
		}
	}()
	current := filepath.Join(request.Directory, "snapshot")
	if err := RecoverGenerationSwap(current); err != nil {
		return value, err
	}
	report(request.Progress, Progress{Step: "snapshot-reading"})
	cached, meta, err := readSnapshot[T](ctx, current)
	if err == nil && meta.Fingerprint == request.Fingerprint && meta.Shape == request.Shape {
		return cached, ctx.Err()
	}
	if err != nil && !os.IsNotExist(err) {
		return value, err
	}
	report(request.Progress, Progress{Step: "snapshot-building"})
	if err := ctx.Err(); err != nil {
		return value, err
	}
	value, err = request.Build(ctx)
	if err != nil {
		return value, err
	}
	if err := ctx.Err(); err != nil {
		return value, err
	}
	report(request.Progress, Progress{Step: "snapshot-writing"})
	staged := current + ".next"
	if err := os.Mkdir(staged, 0755); err != nil {
		return value, err
	}
	defer os.RemoveAll(staged)
	if err := writeSnapshot(ctx, staged, value, snapshotMeta{Schema: snapshotSchema, Fingerprint: request.Fingerprint, Shape: request.Shape}); err != nil {
		return value, err
	}
	if err := ctx.Err(); err != nil {
		return value, err
	}
	if err := CommitGenerationSwap(current, staged); err != nil {
		return value, err
	}
	report(request.Progress, Progress{Current: 1, Total: 1, Step: "snapshot-ready"})
	return value, ctx.Err()
}

func openSnapshotFile(path string, maximum int64) (*os.File, error) {
	before, err := os.Lstat(path)
	if err != nil {
		return nil, err
	}
	if !before.Mode().IsRegular() || before.Mode()&os.ModeSymlink != 0 || before.Size() > maximum {
		return nil, fmt.Errorf("%w: bounded regular snapshot file required", ErrCorrupt)
	}
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	after, err := file.Stat()
	if err != nil || !os.SameFile(before, after) {
		file.Close()
		return nil, fmt.Errorf("%w: snapshot identity changed", ErrCorrupt)
	}
	return file, nil
}
func readSnapshot[T any](ctx context.Context, directory string) (value T, meta snapshotMeta, failure error) {
	file, err := openSnapshotFile(filepath.Join(directory, "meta.json"), MaxManifestBytes)
	if err != nil {
		if os.IsNotExist(err) {
			if _, directoryError := os.Lstat(directory); directoryError == nil {
				return value, meta, fmt.Errorf("%w: published snapshot metadata missing", ErrCorrupt)
			}
		}
		return value, meta, err
	}
	metadata, err := io.ReadAll(io.LimitReader(contextReader{ctx, file}, MaxManifestBytes+1))
	closing := file.Close()
	if err != nil || closing != nil {
		return value, meta, errors.Join(err, closing)
	}
	if len(metadata) > MaxManifestBytes {
		return value, meta, fmt.Errorf("%w: snapshot metadata", ErrCorrupt)
	}
	metadataDecoder := json.NewDecoder(bytes.NewReader(metadata))
	metadataDecoder.DisallowUnknownFields()
	if err := metadataDecoder.Decode(&meta); err != nil {
		return value, meta, err
	}
	if err := metadataDecoder.Decode(new(any)); err != io.EOF {
		return value, meta, errors.Join(fmt.Errorf("%w: trailing snapshot metadata", ErrCorrupt), err, ctx.Err())
	}
	if meta.Schema != snapshotSchema || meta.Bytes < 0 || meta.Bytes > MaxSnapshotCompressedBytes || len(meta.SHA256) != 64 {
		return value, meta, fmt.Errorf("%w: snapshot metadata", ErrCorrupt)
	}
	payload, err := openSnapshotFile(filepath.Join(directory, "payload.json.gz"), MaxSnapshotCompressedBytes)
	if err != nil {
		if os.IsNotExist(err) {
			return value, meta, fmt.Errorf("%w: published snapshot payload missing", ErrCorrupt)
		}
		return value, meta, err
	}
	defer func() { failure = errors.Join(failure, payload.Close()) }()
	digest := sha256.New()
	compressed := io.TeeReader(contextReader{ctx, payload}, digest)
	reader, err := gzip.NewReader(compressed)
	if err != nil {
		return value, meta, err
	}
	limited := &io.LimitedReader{R: contextReader{ctx, reader}, N: MaxSnapshotBytes + 1}
	decoder := json.NewDecoder(limited)
	decodeErr := decoder.Decode(&value)
	if decodeErr == nil {
		if err := decoder.Decode(new(any)); err != io.EOF {
			decodeErr = errors.Join(fmt.Errorf("%w: trailing snapshot data", ErrCorrupt), err, ctx.Err())
		}
	}
	closeErr := reader.Close()
	if decodeErr != nil || closeErr != nil {
		return value, meta, errors.Join(decodeErr, closeErr)
	}
	status, err := payload.Stat()
	if err != nil {
		return value, meta, err
	}
	if limited.N <= 0 || status.Size() != meta.Bytes || hex.EncodeToString(digest.Sum(nil)) != meta.SHA256 {
		return value, meta, fmt.Errorf("%w: snapshot extent or checksum", ErrCorrupt)
	}
	return value, meta, ctx.Err()
}
func writeSnapshot[T any](ctx context.Context, directory string, value T, meta snapshotMeta) error {
	file, err := os.OpenFile(filepath.Join(directory, "payload.json.gz"), os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0600)
	if err != nil {
		return err
	}
	digest := sha256.New()
	compressed := &snapshotWriter{context: ctx, writer: io.MultiWriter(file, digest), remaining: MaxSnapshotCompressedBytes}
	writer, err := gzip.NewWriterLevel(compressed, gzip.BestSpeed)
	if err != nil {
		file.Close()
		return err
	}
	plain := &snapshotWriter{context: ctx, writer: writer, remaining: MaxSnapshotBytes}
	encoding := json.NewEncoder(plain).Encode(value)
	closing := writer.Close()
	syncing := file.Sync()
	stat, statErr := file.Stat()
	fileClosing := file.Close()
	if failure := errors.Join(encoding, closing, syncing, statErr, fileClosing); failure != nil {
		return failure
	}
	meta.Bytes = stat.Size()
	meta.SHA256 = hex.EncodeToString(digest.Sum(nil))
	data, err := json.Marshal(meta)
	if err != nil {
		return err
	}
	if len(data) > MaxManifestBytes {
		return fmt.Errorf("%w: snapshot metadata", ErrTooLarge)
	}
	metadata, err := os.OpenFile(filepath.Join(directory, "meta.json"), os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0600)
	if err != nil {
		return err
	}
	_, writing := metadata.Write(data)
	syncing = metadata.Sync()
	return errors.Join(writing, syncing, metadata.Close())
}

// 🛟️ RecoverGenerationSwap restores the prior complete generation after interrupted publication.
func RecoverGenerationSwap(current string) error {
	for _, path := range []string{current, current + ".next", current + ".previous"} {
		if _, err := generationDirectoryState(path); err != nil {
			return err
		}
	}
	if _, err := os.Stat(current + ".previous"); err == nil {
		if _, err := os.Stat(current); os.IsNotExist(err) {
			if err := os.Rename(current+".previous", current); err != nil {
				return err
			}
		} else if err != nil {
			return err
		} else if err := os.RemoveAll(current + ".previous"); err != nil {
			return err
		}
	} else if !os.IsNotExist(err) {
		return err
	}
	return os.RemoveAll(current + ".next")
}

// 📦️ CommitGenerationSwap preserves the previous generation until the staged directory is published.
func CommitGenerationSwap(current, staged string) error {
	if staged != current+".next" {
		return errors.New("owned staged generation required")
	}
	if exists, err := generationDirectoryState(staged); err != nil {
		return err
	} else if !exists {
		return os.ErrNotExist
	}
	previous := current + ".previous"
	if exists, err := generationDirectoryState(previous); err != nil {
		return err
	} else if exists {
		return errors.New("generation backup already exists")
	}
	exists, err := generationDirectoryState(current)
	if err != nil {
		return err
	}
	if exists {
		if err := os.Rename(current, previous); err != nil {
			return err
		}
	}
	if err := os.Rename(staged, current); err != nil {
		if exists {
			return errors.Join(err, os.Rename(previous, current))
		}
		return err
	}
	if exists {
		return os.RemoveAll(previous)
	}
	return nil
}
func generationDirectoryState(path string) (bool, error) {
	info, err := os.Lstat(path)
	if os.IsNotExist(err) {
		return false, nil
	}
	if err != nil {
		return false, err
	}
	if !info.IsDir() || info.Mode()&os.ModeSymlink != 0 {
		return false, fmt.Errorf("%w: ordinary generation directory required", ErrCorrupt)
	}
	return true, nil
}

// 📡️ NewProgressReporter keeps operation progress on a caller-owned diagnostic stream.
func NewProgressReporter(writer io.Writer) func(Progress) {
	var mutex sync.Mutex
	var last time.Time
	return func(value Progress) {
		mutex.Lock()
		defer mutex.Unlock()
		now := time.Now()
		if !last.IsZero() && now.Sub(last) < time.Second && value.Step != "ready" && value.Step != "snapshot-ready" {
			return
		}
		last = now
		data, err := json.Marshal(map[string]any{"progress": map[string]any{"phase": value.Step, "current": value.Current, "total": value.Total}})
		if err == nil {
			fmt.Fprintln(writer, string(data))
		}
	}
}

// 🌳️FingerprintSource streams authored source identities and bytes in lexical path order.
func FingerprintSource(ctx context.Context, root string, excluded func(string) bool, progress func(Progress)) (string, error) {
	digest := sha256.New()
	buffer := make([]byte, 32<<10)
	visited := 0
	err := filepath.WalkDir(root, func(path string, entry os.DirEntry, failure error) error {
		if err := ctx.Err(); err != nil {
			return err
		}
		if failure != nil {
			return failure
		}
		relative, err := filepath.Rel(root, path)
		if err != nil {
			return err
		}
		relative = filepath.ToSlash(relative)
		visited++
		if visited%256 == 0 {
			report(progress, Progress{Current: visited, Step: "fingerprinting-source"})
		}
		if relative == "." {
			return nil
		}
		if excluded(relative) {
			if entry.IsDir() {
				return filepath.SkipDir
			}
			return nil
		}
		if entry.Type()&os.ModeSymlink != 0 {
			return nil
		}
		kind := "file"
		if entry.IsDir() {
			kind = "directory"
		}
		fmt.Fprintf(digest, "%d:%s:%s\n", len(relative), relative, kind)
		if entry.IsDir() {
			return nil
		}
		info, err := entry.Info()
		if err != nil {
			return err
		}
		if !info.Mode().IsRegular() {
			return nil
		}
		fmt.Fprintf(digest, "%d:", info.Size())
		file, err := os.Open(path)
		if err != nil {
			return err
		}
		_, reading := io.CopyBuffer(digest, contextReader{ctx, file}, buffer)
		return errors.Join(reading, file.Close())
	})
	if err != nil {
		return "", err
	}
	return hex.EncodeToString(digest.Sum(nil)), ctx.Err()
}

func createSnapshotDirectory(path string) error {
	absolute, err := filepath.Abs(path)
	if err != nil {
		return err
	}
	if _, err := os.Lstat(absolute); err == nil {
		return ordinarySearchDirectory(absolute)
	} else if !os.IsNotExist(err) {
		return err
	}
	parent := filepath.Dir(absolute)
	if parent == absolute {
		return errors.New("snapshot directory parent required")
	}
	if err := createSnapshotDirectory(parent); err != nil {
		return err
	}
	if err := os.Mkdir(absolute, 0755); err != nil && !os.IsExist(err) {
		return err
	}
	return ordinarySearchDirectory(absolute)
}
