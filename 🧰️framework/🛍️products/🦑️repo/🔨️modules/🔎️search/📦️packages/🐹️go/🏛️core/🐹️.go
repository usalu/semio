package search

import (
	"bufio"
	"container/heap"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"reflect"
	"sort"
	"strings"
	"sync"
)

// #region 📜️Schema

const (
	schema             = "semio.search.event/1"
	MaxDocumentBytes   = 1 << 20
	MaxIdentifierBytes = 4096
	MaxDocuments       = 250_000
	MaxTraversalNodes  = 500_000
	MaxPendingEvents   = 500_000
	MaxIndexBytes      = 256 << 20
	MaxQueryBytes      = 4 << 10
	MaxQueryTerms      = 32
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

const MaxQueryResults = 10000
const MaxQueryFuzziness = 32

var ErrStaleCursor = errors.New("search cursor belongs to another index generation")

type SearchCursor struct {
	Fingerprint string  `json:"fingerprint"`
	ID          string  `json:"id"`
	Score       float64 `json:"score"`
}

type SearchStatistics struct {
	PeakResidentDocuments     int `json:"peakResidentDocuments"`
	PeakPendingEvents         int `json:"peakPendingEvents"`
	PeakRetainedHits          int `json:"peakRetainedHits"`
	PeakResidentDocumentBytes int `json:"peakResidentDocumentBytes"`
}

type SearchRequest struct {
	Query     Query
	Size      int
	After     *SearchCursor
	Accept    func(string, map[string]string) bool
	FilterKey string
}

type SearchHit struct {
	ID    string  `json:"id"`
	Score float64 `json:"score"`
}

type SearchResult struct {
	Total      uint64           `json:"total"`
	Next       *SearchCursor    `json:"next,omitempty"`
	Statistics SearchStatistics `json:"statistics"`
	Hits       []*SearchHit     `json:"hits"`
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
	Schema     string            `json:"schema"`
	Seq        uint64            `json:"seq"`
	Kind       string            `json:"kind"`
	ID         string            `json:"id"`
	Text       string            `json:"text,omitempty"`
	Attributes map[string]string `json:"attributes,omitempty"`
}

type ownedIndex struct {
	mu                sync.RWMutex
	path              string
	docs              map[string]string
	attributes        map[string]map[string]string
	seen              map[uint64]struct{}
	seq               uint64
	pending           []event
	pendingBytes      int
	documentBytes     int
	peakDocuments     int
	peakDocumentBytes int
	closed            bool
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
	text, err := searchDocumentText(value)
	if err != nil {
		return err
	}
	if len(text) > MaxDocumentBytes {
		return fmt.Errorf("%w: document %q has %d bytes, maximum %d", ErrTooLarge, id, len(text), MaxDocumentBytes)
	}
	if err := ctx.Err(); err != nil {
		return err
	}
	if err := index.append(ctx, "indexed", id, text, documentAttributes(value)); err != nil {
		return err
	}
	report(progress, Progress{Current: 1, Total: 1, Step: "indexed"})
	return nil
}

func (index *ownedIndex) Delete(id string) error {
	return index.DeleteContext(context.Background(), id, nil)
}

func (index *ownedIndex) DeleteContext(ctx context.Context, id string, progress func(Progress)) error {
	if err := index.append(ctx, "deleted", id, "", nil); err != nil {
		return err
	}
	report(progress, Progress{Current: 1, Total: 1, Step: "deleted"})
	return nil
}

func (index *ownedIndex) append(ctx context.Context, kind, id, text string, attributes map[string]string) error {
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
	if id == "" || len(id) > MaxIdentifierBytes {
		return fmt.Errorf("%w: bounded nonempty document identifier required", ErrTooLarge)
	}
	if err := validateAttributes(attributes); err != nil {
		return err
	}
	if len(index.pending) >= MaxPendingEvents {
		return fmt.Errorf("%w: pending event capacity", ErrTooLarge)
	}
	if attributes != nil {
		owned := make(map[string]string, len(attributes))
		for key, value := range attributes {
			owned[key] = value
		}
		attributes = owned
	}
	item := event{Schema: schema, Seq: index.seq + 1, Kind: kind, ID: id, Text: text, Attributes: attributes}
	encoded, err := json.Marshal(item)
	if err != nil {
		return err
	}
	if len(encoded) > MaxDocumentBytes*2 || index.pendingBytes+len(encoded)+1 > MaxIndexBytes {
		return fmt.Errorf("%w: pending event byte capacity", ErrTooLarge)
	}
	if err = index.apply(item); err != nil {
		return err
	}
	index.seq++
	index.pending = append(index.pending, item)
	index.pendingBytes += len(encoded) + 1
	return nil
}

func (index *ownedIndex) apply(item event) error {
	if item.ID == "" || len(item.ID) > MaxIdentifierBytes {
		return fmt.Errorf("%w: bounded nonempty document identifier required", ErrTooLarge)
	}
	previous, exists := index.docs[item.ID]
	oldBytes := 0
	if exists {
		oldBytes = len(item.ID) + len(previous) + attributeBytes(index.attributes[item.ID])
	}
	switch item.Kind {
	case "indexed":
		if len(item.Text) > MaxDocumentBytes {
			return fmt.Errorf("%w: document byte capacity", ErrTooLarge)
		}
		if !exists && len(index.docs) >= MaxDocuments {
			return fmt.Errorf("%w: document capacity", ErrTooLarge)
		}
		if err := validateAttributes(item.Attributes); err != nil {
			return err
		}
		nextBytes := index.documentBytes - oldBytes + len(item.ID) + len(item.Text) + attributeBytes(item.Attributes)
		if nextBytes > MaxIndexBytes {
			return fmt.Errorf("%w: resident document byte capacity", ErrTooLarge)
		}
		if len(item.Attributes) > 0 {
			if index.attributes == nil {
				index.attributes = map[string]map[string]string{}
			}
			index.attributes[item.ID] = item.Attributes
		} else {
			delete(index.attributes, item.ID)
		}
		index.docs[item.ID] = item.Text
		index.documentBytes = nextBytes
		index.peakDocuments = max(index.peakDocuments, len(index.docs))
		index.peakDocumentBytes = max(index.peakDocumentBytes, index.documentBytes)
	case "deleted":
		delete(index.docs, item.ID)
		delete(index.attributes, item.ID)
		index.documentBytes -= oldBytes
	default:
		return fmt.Errorf("unknown index event kind %q", item.Kind)
	}
	return nil
}
func attributeBytes(attributes map[string]string) int {
	size := 0
	for key, value := range attributes {
		size += len(key) + len(value)
	}
	return size
}
func validateAttributes(attributes map[string]string) error {
	if len(attributes) > 16 {
		return fmt.Errorf("%w: document attribute fields", ErrTooLarge)
	}
	if attributeBytes(attributes) > 4096 {
		return fmt.Errorf("%w: document attribute bytes", ErrTooLarge)
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
		index.pending = nil
		index.pendingBytes = 0
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
	index.pendingBytes = 0
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
	if request != nil && request.After != nil {
		return nil, ErrStaleCursor
	}
	limit, err := searchLimit(request, true)
	if err != nil {
		return nil, err
	}
	selected := &hitHeap{}
	heap.Init(selected)
	result, err := index.visitMatches(ctx, request, progress, func(hit SearchHit) error { retainHit(selected, hit, limit); return nil })
	if err != nil {
		return nil, err
	}
	result.Hits = rankedHits(selected)
	result.Statistics.PeakRetainedHits = len(result.Hits)
	return result, nil
}

func (index *ownedIndex) visitMatches(ctx context.Context, request *SearchRequest, progress func(int, int), visit func(SearchHit) error) (*SearchResult, error) {
	terms, err := searchTerms(request)
	if err != nil {
		return nil, err
	}
	if err = ctx.Err(); err != nil {
		return nil, err
	}
	index.mu.RLock()
	defer index.mu.RUnlock()
	total, offset := len(index.docs), 0
	result := &SearchResult{Statistics: SearchStatistics{PeakResidentDocuments: index.peakDocuments, PeakResidentDocumentBytes: index.peakDocumentBytes, PeakPendingEvents: len(index.pending)}}
	for id, text := range index.docs {
		if err = ctx.Err(); err != nil {
			return nil, err
		}
		if progress != nil {
			progress(offset, total)
		}
		if err = ctx.Err(); err != nil {
			return nil, err
		}
		offset++
		if request.Accept != nil && !request.Accept(id, index.attributes[id]) {
			continue
		}
		value, matched := scoreContext(ctx, text, terms)
		if err = ctx.Err(); err != nil {
			return nil, err
		}
		if matched {
			result.Total++
			if visit != nil {
				if err = visit(SearchHit{ID: id, Score: value}); err != nil {
					return nil, err
				}
			}
		}
	}
	if err = ctx.Err(); err != nil {
		return nil, err
	}
	return result, nil
}

func searchTerms(request *SearchRequest) ([]*MatchQuery, error) {
	if request == nil || request.Query == nil {
		return nil, errors.New("search query is required")
	}
	if len(request.FilterKey) > MaxQueryBytes {
		return nil, fmt.Errorf("%w: query eligibility bytes", ErrTooLarge)
	}
	terms := request.Query.terms()
	if len(terms) > MaxQueryTerms {
		return nil, fmt.Errorf("%w: query terms > %d", ErrTooLarge, MaxQueryTerms)
	}
	bytes := 0
	for _, term := range terms {
		if term == nil {
			return nil, errors.New("search term is required")
		}
		bytes += len(term.Term)
	}
	if bytes > MaxQueryBytes {
		return nil, fmt.Errorf("%w: query bytes > %d", ErrTooLarge, MaxQueryBytes)
	}
	return terms, nil
}

func searchLimit(request *SearchRequest, unlimited bool) (int, error) {
	if _, err := searchTerms(request); err != nil {
		return 0, err
	}
	if unlimited && request.Size == -1 {
		return MaxDocuments, nil
	}
	if request.Size < 0 || request.Size > MaxQueryResults {
		return 0, fmt.Errorf("%w: query result size must be between 0 and %d", ErrTooLarge, MaxQueryResults)
	}
	return request.Size, nil
}

type hitHeap []*SearchHit

func (h hitHeap) Len() int                { return len(h) }
func (h hitHeap) Less(i, j int) bool      { return betterHit(h[j], h[i]) }
func (h hitHeap) Swap(i, j int)           { h[i], h[j] = h[j], h[i] }
func (h *hitHeap) Push(value interface{}) { *h = append(*h, value.(*SearchHit)) }
func (h *hitHeap) Pop() interface{} {
	old := *h
	last := len(old) - 1
	value := old[last]
	*h = old[:last]
	return value
}
func betterHit(a, b *SearchHit) bool { return a.Score > b.Score || a.Score == b.Score && a.ID < b.ID }
func retainHit(h *hitHeap, hit SearchHit, limit int) {
	if limit == 0 {
		return
	}
	if h.Len() < limit {
		candidate := hit
		heap.Push(h, &candidate)
	} else if betterHit(&hit, (*h)[0]) {
		candidate := hit
		(*h)[0] = &candidate
		heap.Fix(h, 0)
	}
}
func rankedHits(h *hitHeap) []*SearchHit {
	result := []*SearchHit(*h)
	sort.Slice(result, func(i, j int) bool { return betterHit(result[i], result[j]) })
	return result
}

func score(text string, queries []*MatchQuery) (float64, bool) {
	return scoreContext(context.Background(), text, queries)
}

// 🎯️ Matches one bounded document without constructing an index or retaining other stream items.
func MatchContext(ctx context.Context, text string, query Query) (bool, error) {
	if len(text) > MaxDocumentBytes {
		return false, fmt.Errorf("%w: document bytes", ErrTooLarge)
	}
	terms, err := searchTerms(&SearchRequest{Query: query})
	if err != nil {
		return false, err
	}
	if err = ctx.Err(); err != nil {
		return false, err
	}
	_, matched := scoreContext(ctx, text, terms)
	return matched, ctx.Err()
}

func scoreContext(ctx context.Context, text string, queries []*MatchQuery) (float64, bool) {
	lower := strings.ToLower(text)
	words := strings.FieldsFuncSeq(lower, func(r rune) bool { return !((r >= 'a' && r <= 'z') || (r >= '0' && r <= '9') || r >= 0x80) })
	var scratch distanceScratch
	total := 0.0
	for _, query := range queries {
		if query.Fuzziness < 0 {
			return 0, false
		}
		term := strings.ToLower(query.Term)
		best := query.Fuzziness + 1
		if strings.Contains(lower, term) {
			total += float64(query.Fuzziness + 1)
			continue
		}
		for word := range words {
			if ctx.Err() != nil {
				return 0, false
			}
			distance := boundedDistanceWithScratch(ctx, term, word, query.Fuzziness, &scratch)
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
	return boundedDistanceContext(context.Background(), left, right, maximum)
}
type distanceScratch [2][2*MaxQueryFuzziness + 1]int

func boundedDistanceContext(ctx context.Context, left, right string, maximum int) int {
	var scratch distanceScratch
	return boundedDistanceWithScratch(ctx, left, right, maximum, &scratch)
}
func boundedDistanceWithScratch(ctx context.Context, left, right string, maximum int, scratch *distanceScratch) int {
	if left == right {
		return 0
	}
	if len(left) == 0 {
		return len(right)
	}
	if len(right) == 0 {
		return len(left)
	}
	if maximum < 0 || abs(len(left)-len(right)) > maximum {
		return maximum + 1
	}
	var previous, current []int
	if len(right) < len(scratch[0]) {
		previous, current = scratch[0][:len(right)+1], scratch[1][:len(right)+1]
	} else {
		previous, current = make([]int, len(right)+1), make([]int, len(right)+1)
	}
	for j := range previous {
		previous[j] = maximum + 1
		if j <= maximum {
			previous[j] = j
		}
	}
	for i := 1; i <= len(left); i++ {
		if ctx.Err() != nil {
			return maximum + 1
		}
		current[0] = i
		start, end := max(1, i-maximum), min(len(right), i+maximum)
		if start > 1 {
			current[start-1] = maximum + 1
		}
		minimum := maximum + 1
		for j := start; j <= end; j++ {
			cost := 0
			if left[i-1] != right[j-1] {
				cost = 1
			}
			current[j] = min(previous[j]+1, current[j-1]+1, previous[j-1]+cost)
			minimum = min(minimum, current[j])
		}
		if end < len(right) {
			current[end+1] = maximum + 1
		}
		if minimum > maximum {
			return maximum + 1
		}
		previous, current = current, previous
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

// 🧾️ FieldText renders indexed fields in lexical key order and sequences in their declared order.
func FieldText(data map[string]interface{}) string {
	keys := make([]string, 0, len(data))
	for key := range data {
		keys = append(keys, key)
	}
	sort.Strings(keys)
	parts := make([]string, 0, len(keys))
	for _, key := range keys {
		value := data[key]
		if physical := reflect.ValueOf(value); physical.IsValid() {
			switch physical.Kind() {
			case reflect.Slice, reflect.Map, reflect.Ptr:
				if physical.IsNil() {
					value = nil
				}
			}
		}
		switch typed := value.(type) {
		case nil:
			parts = append(parts, "")
		case string:
			parts = append(parts, typed)
		case []string:
			parts = append(parts, strings.Join(typed, " "))
		default:
			sequence := reflect.ValueOf(value)
			if sequence.IsValid() && (sequence.Kind() == reflect.Slice || sequence.Kind() == reflect.Array) {
				items := make([]string, sequence.Len())
				for i := range items {
					items[i] = fmt.Sprint(sequence.Index(i).Interface())
				}
				parts = append(parts, strings.Join(items, " "))
			} else {
				parts = append(parts, fmt.Sprint(value))
			}
		}
	}
	return strings.Join(parts, " ")
}
