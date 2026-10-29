package search

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
	"math"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"sync"
)

const (
	ShardCount         = 64
	MaxManifestBytes   = 64 << 10
	DefaultTreeHits    = 100
	MaxResultTreeNodes = 10000
	MaxResultTreeBytes = 16 << 20
	MaxTraversalDepth  = 256
	shardSchema        = "semio.search.shards/1"
)

type ShardInfo struct {
	Name      string `json:"name"`
	Bytes     int64  `json:"bytes"`
	SHA256    string `json:"sha256"`
	Documents int    `json:"documents"`
	Events    uint64 `json:"events"`
}

type ShardManifest struct {
	Schema    string      `json:"schema"`
	Snapshot  string      `json:"snapshot"`
	Documents int         `json:"documents"`
	Shards    []ShardInfo `json:"shards"`
}

type shardWriter struct {
	file   *os.File
	buffer *bufio.Writer
	events uint64
	bytes  int64
}

type ShardedIndex struct {
	mu          sync.Mutex
	root        string
	writers     [ShardCount]*shardWriter
	files       []*os.File
	manifest    ShardManifest
	fingerprint string
	building    bool
	closed      bool
}

// 🧵️ Spools bounded hash partitions without retaining the repository's documents or pending events.
func NewSharded(path string, snapshot string) (*ShardedIndex, error) {
	if snapshot == "" || len(snapshot) > 256 {
		return nil, errors.New("bounded nonempty search snapshot required")
	}
	if err := ordinarySearchDirectory(filepath.Dir(path)); err != nil {
		return nil, err
	}
	if err := os.Mkdir(path, 0o755); err != nil && !os.IsExist(err) {
		return nil, err
	}
	if err := ordinarySearchDirectory(path); err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(path)
	if err != nil {
		return nil, err
	}
	if len(entries) > 0 {
		return nil, errors.New("search index already exists")
	}
	return &ShardedIndex{root: path, building: true, manifest: ShardManifest{Schema: shardSchema, Snapshot: snapshot, Shards: []ShardInfo{}}}, nil
}

func shardName(id string) string {
	digest := sha256.Sum256([]byte(id))
	return fmt.Sprintf("s%02d", int(digest[0])%ShardCount)
}

func searchDocumentText(value interface{}) (string, error) {
	switch document := value.(type) {
	case map[string]interface{}:
		parts := make([]string, 0, 2)
		for _, field := range []string{"text", "path"} {
			if part, ok := document[field].(string); ok {
				parts = append(parts, part)
			}
		}
		return strings.TrimSpace(strings.Join(parts, " ")), nil
	default:
		data, err := json.Marshal(value)
		return string(data), err
	}
}

func documentAttributes(value interface{}) map[string]string {
	if document, ok := value.(map[string]interface{}); ok {
		if attributes, ok := document["attributes"].(map[string]string); ok {
			return attributes
		}
	}
	return nil
}

func (index *ShardedIndex) Index(id string, value interface{}) error {
	return index.IndexContext(context.Background(), id, value, nil)
}
func (index *ShardedIndex) IndexContext(ctx context.Context, id string, value interface{}, progress func(Progress)) error {
	text, err := searchDocumentText(value)
	if err != nil {
		return err
	}
	if len(text) > MaxDocumentBytes {
		return fmt.Errorf("%w: document bytes", ErrTooLarge)
	}
	if err = index.append(ctx, "indexed", id, text, documentAttributes(value)); err != nil {
		return err
	}
	report(progress, Progress{Current: 1, Total: 1, Step: "indexed"})
	return ctx.Err()
}
func (index *ShardedIndex) Delete(id string) error {
	return index.DeleteContext(context.Background(), id, nil)
}
func (index *ShardedIndex) DeleteContext(ctx context.Context, id string, progress func(Progress)) error {
	if err := index.append(ctx, "deleted", id, "", nil); err != nil {
		return err
	}
	report(progress, Progress{Current: 1, Total: 1, Step: "deleted"})
	return ctx.Err()
}
func (index *ShardedIndex) append(ctx context.Context, kind, id, text string, attributes map[string]string) error {
	if err := ctx.Err(); err != nil {
		return err
	}
	if id == "" || len(id) > MaxIdentifierBytes {
		return fmt.Errorf("%w: bounded nonempty document identifier required", ErrTooLarge)
	}
	index.mu.Lock()
	defer index.mu.Unlock()
	if index.closed || !index.building {
		return errors.New("search index is not an open writer")
	}
	name := shardName(id)
	number, _ := strconv.Atoi(name[1:])
	writer := index.writers[number]
	if writer == nil {
		directory := filepath.Join(index.root, name)
		if err := os.Mkdir(directory, 0o755); err != nil {
			return err
		}
		file, err := os.OpenFile(filepath.Join(directory, "events.jsonl"), os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0o600)
		if err != nil {
			return err
		}
		writer = &shardWriter{file: file, buffer: bufio.NewWriterSize(file, 64<<10)}
		index.writers[number] = writer
	}
	if writer.events >= MaxPendingEvents {
		return fmt.Errorf("%w: shard event capacity", ErrTooLarge)
	}
	item := event{Schema: schema, Seq: writer.events + 1, Kind: kind, ID: id, Text: text, Attributes: attributes}
	encoded, err := json.Marshal(item)
	if err != nil {
		return err
	}
	encoded = append(encoded, '\n')
	if len(encoded) > MaxDocumentBytes*2 || writer.bytes+int64(len(encoded)) > MaxIndexBytes {
		return fmt.Errorf("%w: shard byte capacity", ErrTooLarge)
	}
	count, err := writer.buffer.Write(encoded)
	if err != nil {
		return err
	}
	if count != len(encoded) {
		return io.ErrShortWrite
	}
	writer.events++
	writer.bytes += int64(count)
	return ctx.Err()
}

// 🧯️ Closes only this owner's bounded descriptors so failed staging can be retired on every host.
func (index *ShardedIndex) Abort() error {
	index.mu.Lock()
	defer index.mu.Unlock()
	return index.abort()
}
func (index *ShardedIndex) abort() error {
	var failures []error
	for _, writer := range index.writers {
		if writer != nil && writer.file != nil {
			if err := writer.file.Close(); err != nil {
				failures = append(failures, err)
			}
			writer.file = nil
		}
	}
	for _, file := range index.files {
		if err := file.Close(); err != nil {
			failures = append(failures, err)
		}
	}
	index.files = nil
	index.closed = true
	return errors.Join(failures...)
}
func (index *ShardedIndex) Close() error { return index.CloseContext(context.Background(), nil) }
func (index *ShardedIndex) CloseContext(ctx context.Context, progress func(Progress)) error {
	index.mu.Lock()
	defer index.mu.Unlock()
	if index.closed {
		return nil
	}
	if !index.building {
		err := index.abort()
		return errors.Join(ctx.Err(), err)
	}
	fail := func(err error) error { return errors.Join(err, index.abort()) }
	for i, writer := range index.writers {
		if writer == nil {
			continue
		}
		if err := ctx.Err(); err != nil {
			return fail(err)
		}
		if err := writer.buffer.Flush(); err != nil {
			return fail(err)
		}
		if err := writer.file.Sync(); err != nil {
			return fail(err)
		}
		if err := writer.file.Close(); err != nil {
			return fail(err)
		}
		writer.file = nil
		report(progress, Progress{Current: i + 1, Total: ShardCount, Step: "persisted"})
		if err := ctx.Err(); err != nil {
			return fail(err)
		}
	}
	for i, writer := range index.writers {
		if writer == nil {
			continue
		}
		name := fmt.Sprintf("s%02d", i)
		file, err := openRegularSearchFile(filepath.Join(index.root, name, "events.jsonl"))
		if err != nil {
			return fail(err)
		}
		replay, info, err := replayShard(ctx, file, name, nil)
		closing := file.Close()
		if err != nil || closing != nil {
			return fail(errors.Join(err, closing))
		}
		info.Events = writer.events
		if replay.seq != writer.events || info.Bytes != writer.bytes {
			return fail(fmt.Errorf("%w: staged shard extent", ErrCorrupt))
		}
		index.manifest.Documents += info.Documents
		index.manifest.Shards = append(index.manifest.Shards, info)
		report(progress, Progress{Current: i + 1, Total: ShardCount, Step: "verified"})
		if err = ctx.Err(); err != nil {
			return fail(err)
		}
	}
	data, err := json.Marshal(index.manifest)
	if err != nil {
		return fail(err)
	}
	if len(data) > MaxManifestBytes {
		return fail(fmt.Errorf("%w: manifest bytes", ErrTooLarge))
	}
	temporary := filepath.Join(index.root, "manifest.next")
	file, err := os.OpenFile(temporary, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0o600)
	if err != nil {
		return fail(err)
	}
	if count, writeErr := file.Write(data); writeErr != nil || count != len(data) {
		file.Close()
		os.Remove(temporary)
		if writeErr == nil {
			writeErr = io.ErrShortWrite
		}
		return fail(writeErr)
	}
	if err = file.Sync(); err != nil {
		file.Close()
		os.Remove(temporary)
		return fail(err)
	}
	if err = file.Close(); err != nil {
		os.Remove(temporary)
		return fail(err)
	}
	if err = ctx.Err(); err != nil {
		os.Remove(temporary)
		return fail(err)
	}
	if err = os.Rename(temporary, filepath.Join(index.root, "manifest.json")); err != nil {
		os.Remove(temporary)
		return fail(err)
	}
	index.closed = true
	report(progress, Progress{Current: index.manifest.Documents, Total: index.manifest.Documents, Step: "committed"})
	return nil
}

func replayShard(ctx context.Context, file *os.File, name string, expected *ShardInfo) (*ownedIndex, ShardInfo, error) {
	info := ShardInfo{Name: name}
	before, err := file.Stat()
	if err != nil {
		return nil, info, err
	}
	if !before.Mode().IsRegular() || before.Size() > MaxIndexBytes {
		return nil, info, fmt.Errorf("%w: bounded regular shard required", ErrCorrupt)
	}
	if _, err = file.Seek(0, io.SeekStart); err != nil {
		return nil, info, err
	}
	hash := sha256.New()
	scanner := bufio.NewScanner(io.TeeReader(io.LimitReader(file, MaxIndexBytes+1), hash))
	scanner.Buffer(make([]byte, 64<<10), MaxDocumentBytes*2+1)
	index := &ownedIndex{docs: map[string]string{}, seen: map[uint64]struct{}{}}
	for scanner.Scan() {
		if err = ctx.Err(); err != nil {
			return nil, info, err
		}
		line := scanner.Bytes()
		if len(line) > MaxDocumentBytes*2 {
			return nil, info, fmt.Errorf("%w: shard event bytes", ErrCorrupt)
		}
		decoder := json.NewDecoder(bytes.NewReader(line))
		decoder.DisallowUnknownFields()
		var item event
		if err = decoder.Decode(&item); err != nil {
			return nil, info, fmt.Errorf("%w: %v", ErrCorrupt, err)
		}
		if err = decoder.Decode(new(interface{})); err != io.EOF {
			return nil, info, fmt.Errorf("%w: trailing shard event", ErrCorrupt)
		}
		if item.Schema != schema || item.Seq != index.seq+1 || item.Seq > MaxPendingEvents || shardName(item.ID) != name {
			return nil, info, fmt.Errorf("%w: shard event sequence or partition", ErrCorrupt)
		}
		if err = index.apply(item); err != nil {
			return nil, info, fmt.Errorf("%w: %v", ErrCorrupt, err)
		}
		index.seq = item.Seq
	}
	if err = scanner.Err(); err != nil {
		return nil, info, fmt.Errorf("%w: %v", ErrCorrupt, err)
	}
	after, err := file.Stat()
	if err != nil {
		return nil, info, err
	}
	if !os.SameFile(before, after) || before.Size() != after.Size() || !before.ModTime().Equal(after.ModTime()) {
		return nil, info, fmt.Errorf("%w: shard changed during replay", ErrCorrupt)
	}
	info.Bytes = after.Size()
	info.SHA256 = hex.EncodeToString(hash.Sum(nil))
	info.Documents = len(index.docs)
	info.Events = index.seq
	if expected != nil && info != *expected {
		return nil, info, fmt.Errorf("%w: shard manifest mismatch", ErrCorrupt)
	}
	if err = ctx.Err(); err != nil {
		return nil, info, err
	}
	return index, info, nil
}

// 📂️ Retains the exact shard descriptors so concurrent atomic cache publication cannot mix generations.
func OpenSharded(path string) (*ShardedIndex, error) {
	return OpenShardedContext(context.Background(), path, nil)
}
func OpenShardedContext(ctx context.Context, path string, progress func(Progress)) (*ShardedIndex, error) {
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	if err := ordinarySearchDirectory(path); err != nil {
		return nil, err
	}
	file, err := openRegularSearchFile(filepath.Join(path, "manifest.json"))
	if err != nil {
		return nil, err
	}
	data, err := io.ReadAll(io.LimitReader(file, MaxManifestBytes+1))
	closing := file.Close()
	if err != nil || closing != nil {
		return nil, errors.Join(err, closing)
	}
	if len(data) > MaxManifestBytes {
		return nil, fmt.Errorf("%w: manifest byte capacity", ErrCorrupt)
	}
	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	var manifest ShardManifest
	if err = decoder.Decode(&manifest); err != nil {
		return nil, fmt.Errorf("%w: %v", ErrCorrupt, err)
	}
	if err = decoder.Decode(new(interface{})); err != io.EOF {
		return nil, fmt.Errorf("%w: trailing manifest", ErrCorrupt)
	}
	if manifest.Schema != shardSchema || manifest.Snapshot == "" || len(manifest.Snapshot) > 256 || len(manifest.Shards) > ShardCount || manifest.Documents < 0 {
		return nil, fmt.Errorf("%w: bounded shard manifest required", ErrCorrupt)
	}
	digest := sha256.Sum256(data)
	index := &ShardedIndex{root: path, manifest: manifest, fingerprint: hex.EncodeToString(digest[:])}
	last := ""
	documents := 0
	fail := func(err error) (*ShardedIndex, error) { return nil, errors.Join(err, index.abort()) }
	for i, info := range manifest.Shards {
		number, numberErr := strconv.Atoi(strings.TrimPrefix(info.Name, "s"))
		hash, hashErr := hex.DecodeString(info.SHA256)
		if numberErr != nil || number < 0 || number >= ShardCount || info.Name != fmt.Sprintf("s%02d", number) || info.Name <= last || info.Bytes < 1 || info.Bytes > MaxIndexBytes || info.Documents < 0 || info.Documents > MaxDocuments || info.Events < 1 || info.Events > MaxPendingEvents || hashErr != nil || len(hash) != sha256.Size || strings.ToLower(info.SHA256) != info.SHA256 {
			return fail(fmt.Errorf("%w: closed shard identity required", ErrCorrupt))
		}
		last = info.Name
		directory := filepath.Join(path, info.Name)
		row, rowErr := os.Lstat(directory)
		if rowErr != nil {
			return fail(rowErr)
		}
		if !row.IsDir() || row.Mode()&os.ModeSymlink != 0 {
			return fail(fmt.Errorf("%w: ordinary shard directory required", ErrCorrupt))
		}
		name := filepath.Join(directory, "events.jsonl")
		row, rowErr = os.Lstat(name)
		if rowErr != nil {
			return fail(rowErr)
		}
		if !row.Mode().IsRegular() || row.Mode()&os.ModeSymlink != 0 {
			return fail(fmt.Errorf("%w: ordinary shard file required", ErrCorrupt))
		}
		shard, openErr := openRegularSearchFile(name)
		if openErr != nil {
			return fail(openErr)
		}
		index.files = append(index.files, shard)
		if _, _, err = replayShard(ctx, shard, info.Name, &info); err != nil {
			return fail(err)
		}
		documents += info.Documents
		report(progress, Progress{Current: i + 1, Total: len(manifest.Shards), Step: "opened"})
		if err = ctx.Err(); err != nil {
			return fail(err)
		}
	}
	if documents != manifest.Documents {
		return fail(fmt.Errorf("%w: complete shard document count required", ErrCorrupt))
	}
	return index, nil
}
func (index *ShardedIndex) Search(request *SearchRequest) (*SearchResult, error) {
	return index.SearchContext(context.Background(), request, nil)
}
func (index *ShardedIndex) SearchContext(ctx context.Context, request *SearchRequest, progress func(int, int)) (*SearchResult, error) {
	limit, err := searchLimit(request, false)
	if err != nil {
		return nil, err
	}
	if err = ctx.Err(); err != nil {
		return nil, err
	}
	index.mu.Lock()
	defer index.mu.Unlock()
	if index.closed || index.building {
		return nil, errors.New("readable committed search generation required")
	}
	queryFingerprint, err := index.queryFingerprint(request)
	if err != nil {
		return nil, err
	}
	if request.After != nil && (request.After.Fingerprint != queryFingerprint || request.After.ID == "" || len(request.After.ID) > MaxIdentifierBytes || math.IsNaN(request.After.Score) || math.IsInf(request.After.Score, 0)) {
		return nil, ErrStaleCursor
	}
	selected := &hitHeap{}
	result := &SearchResult{}
	eligible := uint64(0)
	offset := 0
	for i, file := range index.files {
		shard, _, err := replayShard(ctx, file, index.manifest.Shards[i].Name, &index.manifest.Shards[i])
		if err != nil {
			return nil, err
		}
		current, err := shard.visitMatches(ctx, request, func(done, total int) {
			if progress != nil {
				progress(offset+done, index.manifest.Documents)
			}
		}, func(hit SearchHit) error {
			if request.After != nil && (hit.Score > request.After.Score || hit.Score == request.After.Score && hit.ID <= request.After.ID) {
				return nil
			}
			eligible++
			retainHit(selected, hit, limit)
			return ctx.Err()
		})
		if err != nil {
			return nil, err
		}
		result.Total += current.Total
		offset += len(shard.docs)
		result.Statistics.PeakResidentDocuments = max(result.Statistics.PeakResidentDocuments, shard.peakDocuments)
		result.Statistics.PeakResidentDocumentBytes = max(result.Statistics.PeakResidentDocumentBytes, shard.peakDocumentBytes)
	}
	result.Hits = rankedHits(selected)
	result.Statistics.PeakRetainedHits = len(result.Hits)
	if limit > 0 && eligible > uint64(len(result.Hits)) {
		last := result.Hits[len(result.Hits)-1]
		result.Next = &SearchCursor{Fingerprint: queryFingerprint, ID: last.ID, Score: last.Score}
	}
	if err = ctx.Err(); err != nil {
		return nil, err
	}
	return result, nil
}

func (index *ShardedIndex) queryFingerprint(request *SearchRequest) (string, error) {
	terms, err := searchTerms(request)
	if err != nil {
		return "", err
	}
	if request.Accept != nil && request.FilterKey == "" {
		return "", errors.New("search eligibility key is required")
	}
	normalized := make([]MatchQuery, 0, len(terms))
	for _, term := range terms {
		normalized = append(normalized, MatchQuery{Term: strings.ToLower(term.Term), Fuzziness: term.Fuzziness})
	}
	data, err := json.Marshal(struct {
		Generation string
		Terms      []MatchQuery
		Filter     string
	}{index.fingerprint, normalized, request.FilterKey})
	if err != nil {
		return "", err
	}
	digest := sha256.Sum256(data)
	return hex.EncodeToString(digest[:]), nil
}

func ordinarySearchDirectory(path string) error {
	current, err := filepath.Abs(path)
	if err != nil {
		return err
	}
	for {
		info, err := os.Lstat(current)
		if err != nil {
			return err
		}
		if !info.IsDir() || info.Mode()&os.ModeSymlink != 0 {
			return errors.New("ordinary search directory required")
		}
		parent := filepath.Dir(current)
		if parent == current {
			return nil
		}
		current = parent
	}
}
func openRegularSearchFile(path string) (*os.File, error) {
	before, err := os.Lstat(path)
	if err != nil {
		return nil, err
	}
	if !before.Mode().IsRegular() || before.Mode()&os.ModeSymlink != 0 {
		return nil, fmt.Errorf("%w: ordinary search file required", ErrCorrupt)
	}
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	opened, err := file.Stat()
	if err != nil || !os.SameFile(before, opened) {
		if err == nil {
			err = fmt.Errorf("%w: search file identity changed", ErrCorrupt)
		}
		return nil, errors.Join(err, file.Close())
	}
	return file, nil
}
