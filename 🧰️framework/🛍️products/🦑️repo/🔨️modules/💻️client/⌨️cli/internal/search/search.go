// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Package search provides a deterministic, cancellable, append-replayed text index.

// #endregion 🧲️Header

package search

import (
	"bufio"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"sync"
)

// #region 📜️Schema

const (
	schema            = "semio.search.event/1"
	MaxDocumentBytes  = 1 << 20
	MaxDocuments      = 250_000
	MaxTraversalNodes = 500_000
	MaxPendingEvents  = 500_000
	MaxIndexBytes     = 256 << 20
	MaxQueryBytes     = 4 << 10
	MaxQueryTerms     = 32
)

var (
	ErrCorrupt  = errors.New("corrupt index")
	ErrTooLarge = errors.New("index limit exceeded")
)

type Query interface{ terms() []*MatchQuery }

type MatchQuery struct {
	Term      string `json:"term"`
	Fuzziness int    `json:"fuzziness"`
}

func (query *MatchQuery) SetFuzziness(value int) { query.Fuzziness = value }
func (query *MatchQuery) terms() []*MatchQuery   { return []*MatchQuery{query} }

type conjunctionQuery struct{ Queries []Query }

func (query conjunctionQuery) terms() []*MatchQuery {
	var terms []*MatchQuery
	for _, child := range query.Queries {
		terms = append(terms, child.terms()...)
	}
	return terms
}

type SearchRequest struct {
	Query Query
	Size  int
}

type SearchHit struct {
	ID    string  `json:"id"`
	Score float64 `json:"score"`
}

type SearchResult struct {
	Total uint64       `json:"total"`
	Hits  []*SearchHit `json:"hits"`
}

type IndexMapping struct{}

type Index interface {
	Index(string, interface{}) error
	IndexContext(context.Context, string, interface{}, func(Progress)) error
	Delete(string) error
	DeleteContext(context.Context, string, func(Progress)) error
	Search(*SearchRequest) (*SearchResult, error)
	SearchContext(context.Context, *SearchRequest, func(int, int)) (*SearchResult, error)
	Close() error
	CloseContext(context.Context, func(Progress)) error
}

type Progress struct {
	Current int
	Total   int
	Step    string
}

type event struct {
	Schema string `json:"schema"`
	Seq    uint64 `json:"seq"`
	Kind   string `json:"kind"`
	ID     string `json:"id"`
	Text   string `json:"text,omitempty"`
}

type ownedIndex struct {
	mu      sync.RWMutex
	path    string
	docs    map[string]string
	seen    map[uint64]struct{}
	seq     uint64
	pending []event
	closed  bool
}

// #endregion 📜️Schema

// #region 📝️AppendReplay

func NewIndexMapping() *IndexMapping { return &IndexMapping{} }

func New(path string, _ *IndexMapping) (Index, error) {
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return nil, err
	}
	index := &ownedIndex{path: filepath.Join(path, "events.jsonl"), docs: map[string]string{}, seen: map[uint64]struct{}{}}
	if _, err := os.Stat(index.path); err == nil {
		return nil, fmt.Errorf("index already exists: %s", path)
	}
	return index, nil
}

func NewMemOnly(_ *IndexMapping) (Index, error) {
	return &ownedIndex{docs: map[string]string{}, seen: map[uint64]struct{}{}}, nil
}

func Open(path string) (Index, error) {
	index := &ownedIndex{path: filepath.Join(path, "events.jsonl"), docs: map[string]string{}, seen: map[uint64]struct{}{}}
	if err := recoverIndexFile(index.path); err != nil {
		return nil, err
	}
	file, err := os.Open(index.path)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	buffer := make([]byte, 64*1024)
	scanner.Buffer(buffer, 8*1024*1024)
	expected := uint64(1)
	for scanner.Scan() {
		if len(scanner.Bytes()) > MaxDocumentBytes*2 {
			return nil, fmt.Errorf("%w: event %d too large", ErrCorrupt, expected)
		}
		var item event
		if err := json.Unmarshal(scanner.Bytes(), &item); err != nil {
			return nil, fmt.Errorf("%w: event %d: %v", ErrCorrupt, expected, err)
		}
		if item.Schema != schema || item.Seq != expected {
			return nil, fmt.Errorf("%w: sequence got %d, want %d", ErrCorrupt, item.Seq, expected)
		}
		if _, duplicate := index.seen[item.Seq]; duplicate {
			return nil, fmt.Errorf("duplicate index event %d", item.Seq)
		}
		if err := index.apply(item); err != nil {
			return nil, fmt.Errorf("%w: %v", ErrCorrupt, err)
		}
		index.seen[item.Seq] = struct{}{}
		index.seq = item.Seq
		expected++
	}
	if err := scanner.Err(); err != nil {
		return nil, fmt.Errorf("%w: %v", ErrCorrupt, err)
	}
	return index, nil
}

func (index *ownedIndex) Index(id string, value interface{}) error {
	return index.IndexContext(context.Background(), id, value, nil)
}

func (index *ownedIndex) IndexContext(ctx context.Context, id string, value interface{}, progress func(Progress)) error {
	if err := ctx.Err(); err != nil {
		return err
	}
	text := ""
	switch document := value.(type) {
	case map[string]interface{}:
		for _, field := range []string{"text", "path"} {
			if part, ok := document[field].(string); ok {
				text += " " + part
			}
		}
	default:
		encoded, err := json.Marshal(document)
		if err != nil {
			return err
		}
		text = string(encoded)
	}
	text = strings.TrimSpace(text)
	if len(text) > MaxDocumentBytes {
		return fmt.Errorf("%w: document %q has %d bytes, maximum %d", ErrTooLarge, id, len(text), MaxDocumentBytes)
	}
	if err := ctx.Err(); err != nil {
		return err
	}
	if err := index.append(ctx, "indexed", id, text); err != nil {
		return err
	}
	report(progress, Progress{Current: 1, Total: 1, Step: "indexed"})
	return nil
}

func (index *ownedIndex) Delete(id string) error {
	return index.DeleteContext(context.Background(), id, nil)
}

func (index *ownedIndex) DeleteContext(ctx context.Context, id string, progress func(Progress)) error {
	if err := index.append(ctx, "deleted", id, ""); err != nil {
		return err
	}
	report(progress, Progress{Current: 1, Total: 1, Step: "deleted"})
	return nil
}

func (index *ownedIndex) append(ctx context.Context, kind, id, text string) error {
	if err := ctx.Err(); err != nil {
		return err
	}
	index.mu.Lock()
	defer index.mu.Unlock()
	if err := ctx.Err(); err != nil {
		return err
	}
	if index.closed {
		return errors.New("index closed")
	}
	if id == "" {
		return errors.New("index event has empty id")
	}
	if kind == "indexed" {
		if _, exists := index.docs[id]; !exists && len(index.docs) >= MaxDocuments {
			return fmt.Errorf("%w: documents > %d", ErrTooLarge, MaxDocuments)
		}
	}
	if len(index.pending) >= MaxPendingEvents {
		return fmt.Errorf("%w: pending events > %d", ErrTooLarge, MaxPendingEvents)
	}
	index.seq++
	item := event{Schema: schema, Seq: index.seq, Kind: kind, ID: id, Text: text}
	if err := index.apply(item); err != nil {
		index.seq--
		return err
	}
	index.pending = append(index.pending, item)
	return nil
}

func (index *ownedIndex) apply(item event) error {
	switch item.Kind {
	case "indexed":
		if item.ID == "" {
			return errors.New("index event has empty id")
		}
		if len(item.Text) > MaxDocumentBytes {
			return fmt.Errorf("document %q exceeds %d bytes", item.ID, MaxDocumentBytes)
		}
		if _, exists := index.docs[item.ID]; !exists && len(index.docs) >= MaxDocuments {
			return fmt.Errorf("documents exceed %d", MaxDocuments)
		}
		index.docs[item.ID] = item.Text
	case "deleted":
		delete(index.docs, item.ID)
	default:
		return fmt.Errorf("unknown index event kind %q", item.Kind)
	}
	return nil
}

func (index *ownedIndex) Close() error {
	return index.CloseContext(context.Background(), nil)
}

func (index *ownedIndex) CloseContext(ctx context.Context, progress func(Progress)) error {
	if err := ctx.Err(); err != nil {
		return err
	}
	index.mu.Lock()
	defer index.mu.Unlock()
	if err := ctx.Err(); err != nil {
		return err
	}
	if index.closed {
		return nil
	}
	if index.path == "" || len(index.pending) == 0 {
		index.closed = true
		return nil
	}
	if err := os.MkdirAll(filepath.Dir(index.path), 0o755); err != nil {
		return err
	}
	if err := recoverIndexFile(index.path); err != nil {
		return err
	}
	temporary := index.path + ".next"
	_ = os.Remove(temporary)
	output, err := os.OpenFile(temporary, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0o644)
	if err != nil {
		return err
	}
	cleanup := func(failure error) error {
		_ = output.Close()
		_ = os.Remove(temporary)
		return failure
	}
	written := int64(0)
	if current, openErr := os.Open(index.path); openErr == nil {
		copied, copyErr := copyContext(ctx, output, current, MaxIndexBytes)
		written += copied
		if copyErr != nil {
			current.Close()
			return cleanup(copyErr)
		}
		if err := current.Close(); err != nil {
			return cleanup(err)
		}
		report(progress, Progress{Current: int(written), Step: "copied"})
	} else if !os.IsNotExist(openErr) {
		return cleanup(openErr)
	}
	for offset, item := range index.pending {
		if err := ctx.Err(); err != nil {
			return cleanup(err)
		}
		encoded, err := json.Marshal(item)
		if err != nil {
			return cleanup(err)
		}
		encoded = append(encoded, '\n')
		written += int64(len(encoded))
		if written > MaxIndexBytes {
			return cleanup(fmt.Errorf("%w: persisted bytes > %d", ErrTooLarge, MaxIndexBytes))
		}
		count, err := output.Write(encoded)
		if err != nil {
			return cleanup(err)
		}
		if count != len(encoded) {
			return cleanup(io.ErrShortWrite)
		}
		report(progress, Progress{Current: offset + 1, Total: len(index.pending), Step: "persisted"})
	}
	if err := ctx.Err(); err != nil {
		return cleanup(err)
	}
	if err := output.Sync(); err != nil {
		return cleanup(err)
	}
	report(progress, Progress{Current: len(index.pending), Total: len(index.pending), Step: "synced"})
	if err := ctx.Err(); err != nil {
		return cleanup(err)
	}
	if err := output.Close(); err != nil {
		_ = os.Remove(temporary)
		return err
	}
	if err := replaceIndexFile(index.path, temporary); err != nil {
		return err
	}
	index.pending = nil
	index.closed = true
	report(progress, Progress{Current: 1, Total: 1, Step: "committed"})
	return nil
}

func copyContext(ctx context.Context, destination io.Writer, source io.Reader, maximum int64) (int64, error) {
	buffer := make([]byte, 64*1024)
	var written int64
	for {
		if err := ctx.Err(); err != nil {
			return written, err
		}
		read, readErr := source.Read(buffer)
		if read > 0 {
			written += int64(read)
			if written > maximum {
				return written, fmt.Errorf("%w: persisted bytes > %d", ErrTooLarge, maximum)
			}
			count, err := destination.Write(buffer[:read])
			if err != nil {
				return written, err
			}
			if count != read {
				return written, io.ErrShortWrite
			}
		}
		if errors.Is(readErr, io.EOF) {
			return written, nil
		}
		if readErr != nil {
			return written, readErr
		}
	}
}

func recoverIndexFile(path string) error {
	backup := path + ".previous"
	_, backupErr := os.Stat(backup)
	if os.IsNotExist(backupErr) {
		_ = os.Remove(path + ".next")
		return nil
	}
	if backupErr != nil {
		return backupErr
	}
	if _, err := os.Stat(path); os.IsNotExist(err) {
		if err := os.Rename(backup, path); err != nil {
			return err
		}
	} else if err != nil {
		return err
	} else if err := os.Remove(backup); err != nil {
		return err
	}
	_ = os.Remove(path + ".next")
	return nil
}

func replaceIndexFile(path, replacement string) error {
	backup := path + ".previous"
	hadCurrent := false
	if _, err := os.Stat(path); err == nil {
		hadCurrent = true
		if err := os.Rename(path, backup); err != nil {
			_ = os.Remove(replacement)
			return err
		}
	} else if !os.IsNotExist(err) {
		_ = os.Remove(replacement)
		return err
	}
	if err := os.Rename(replacement, path); err != nil {
		if hadCurrent {
			_ = os.Rename(backup, path)
		}
		_ = os.Remove(replacement)
		return err
	}
	if hadCurrent {
		_ = os.Remove(backup)
	}
	return nil
}

func report(progress func(Progress), value Progress) {
	if progress != nil {
		progress(value)
	}
}

// #endregion 📝️AppendReplay

// #region 🔎️Query

func NewMatchQuery(term string) *MatchQuery       { return &MatchQuery{Term: term} }
func NewConjunctionQuery(queries ...Query) Query  { return conjunctionQuery{Queries: queries} }
func NewSearchRequest(query Query) *SearchRequest { return &SearchRequest{Query: query, Size: 10} }

func (index *ownedIndex) Search(request *SearchRequest) (*SearchResult, error) {
	return index.SearchContext(context.Background(), request, nil)
}

func (index *ownedIndex) SearchContext(ctx context.Context, request *SearchRequest, progress func(int, int)) (*SearchResult, error) {
	if request == nil || request.Query == nil {
		return nil, errors.New("search query is required")
	}
	terms := request.Query.terms()
	if len(terms) > MaxQueryTerms {
		return nil, fmt.Errorf("%w: query terms > %d", ErrTooLarge, MaxQueryTerms)
	}
	queryBytes := 0
	for _, term := range terms {
		queryBytes += len(term.Term)
	}
	if queryBytes > MaxQueryBytes {
		return nil, fmt.Errorf("%w: query bytes > %d", ErrTooLarge, MaxQueryBytes)
	}
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	index.mu.RLock()
	defer index.mu.RUnlock()
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	ids := make([]string, 0, len(index.docs))
	for id := range index.docs {
		ids = append(ids, id)
	}
	sort.Strings(ids)
	result := &SearchResult{}
	for offset, id := range ids {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		if progress != nil {
			progress(offset, len(ids))
		}
		score, match := score(index.docs[id], terms)
		if match {
			result.Hits = append(result.Hits, &SearchHit{ID: id, Score: score})
		}
	}
	sort.SliceStable(result.Hits, func(left, right int) bool {
		if result.Hits[left].Score == result.Hits[right].Score {
			return result.Hits[left].ID < result.Hits[right].ID
		}
		return result.Hits[left].Score > result.Hits[right].Score
	})
	result.Total = uint64(len(result.Hits))
	if request.Size >= 0 && len(result.Hits) > request.Size {
		result.Hits = result.Hits[:request.Size]
	}
	return result, nil
}

func score(text string, queries []*MatchQuery) (float64, bool) {
	words := strings.FieldsFunc(strings.ToLower(text), func(r rune) bool { return !((r >= 'a' && r <= 'z') || (r >= '0' && r <= '9') || r >= 0x80) })
	total := 0.0
	for _, query := range queries {
		term := strings.ToLower(query.Term)
		best := query.Fuzziness + 1
		if strings.Contains(strings.ToLower(text), term) {
			best = 0
		}
		for _, word := range words {
			distance := boundedDistance(term, word, query.Fuzziness)
			if distance < best {
				best = distance
			}
		}
		if best > query.Fuzziness {
			return 0, false
		}
		total += float64(query.Fuzziness - best + 1)
	}
	return total, true
}

func boundedDistance(left, right string, maximum int) int {
	if left == right {
		return 0
	}
	if maximum < 0 || abs(len(left)-len(right)) > maximum {
		return maximum + 1
	}
	previous := make([]int, len(right)+1)
	for index := range previous {
		previous[index] = index
	}
	for i := 1; i <= len(left); i++ {
		current := make([]int, len(right)+1)
		current[0] = i
		minimum := current[0]
		for j := 1; j <= len(right); j++ {
			cost := 0
			if left[i-1] != right[j-1] {
				cost = 1
			}
			current[j] = min(previous[j]+1, current[j-1]+1, previous[j-1]+cost)
			minimum = min(minimum, current[j])
		}
		if minimum > maximum {
			return maximum + 1
		}
		previous = current
	}
	return previous[len(right)]
}

func abs(value int) int {
	if value < 0 {
		return -value
	}
	return value
}

// #endregion 🔎️Query
