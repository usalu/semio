// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📌️cache carries the tree cache and the canonical tree serialization used for digests and payloads.

// #endregion 🧲️Header

package tree

import (
	bytes "bytes"
	gzip "compress/gzip"
	context "context"
	sha256 "crypto/sha256"
	hex "encoding/hex"
	json "encoding/json"
	fmt "fmt"
	io "io"
	os "os"
	filepath "path/filepath"
	sort "sort"
	strconv "strconv"
	strings "strings"
	time "time"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 📌️Tree Cache

// 🌳️getTreeCachePath holds the data fields for a getTreeCachePath record.
func getTreeCachePath() string {
	return filepath.Join(getCacheDir(), "tree.json.gz")
}

func getTreeCacheMetaPath() string {
	return filepath.Join(getCacheDir(), "tree-meta.json")
}

// 💿️saveTreeCache holds the data fields for a saveTreeCache record.
func saveTreeCache(tree *TreeNode, meta *cacheMeta) error {
	dir := getCacheDir()
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	data, err := json.Marshal(tree)
	if err != nil {
		return err
	}
	var buf bytes.Buffer
	gz, err := gzip.NewWriterLevel(&buf, gzip.BestSpeed)
	if err != nil {
		return err
	}
	if _, err := gz.Write(data); err != nil {
		gz.Close()
		return err
	}
	if err := gz.Close(); err != nil {
		return err
	}
	if err := os.WriteFile(getTreeCachePath(), buf.Bytes(), 0644); err != nil {
		return err
	}
	metaData, err := json.MarshalIndent(meta, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(getTreeCacheMetaPath(), metaData, 0644)
}

func loadTreeCache() (*TreeNode, *cacheMeta, error) {
	metaData, err := os.ReadFile(getTreeCacheMetaPath())
	if err != nil {
		return nil, nil, err
	}
	var meta cacheMeta
	if err := json.Unmarshal(metaData, &meta); err != nil {
		return nil, nil, err
	}
	if meta.SchemaVersion != cacheSchemaVersion {
		return nil, nil, fmt.Errorf("schema version mismatch")
	}
	compressed, err := os.ReadFile(getTreeCachePath())
	if err != nil {
		return nil, nil, err
	}
	gz, err := gzip.NewReader(bytes.NewReader(compressed))
	if err != nil {
		return nil, nil, err
	}
	defer gz.Close()
	data, err := io.ReadAll(gz)
	if err != nil {
		return nil, nil, err
	}
	var tree TreeNode
	if err := json.Unmarshal(data, &tree); err != nil {
		return nil, nil, err
	}
	return &tree, &meta, nil
}

// 🏢️BuildMonorepoTreeCached holds the data fields for a BuildMonorepoTreeCached record.
func BuildMonorepoTreeCached(ctx context.Context, opts ...TreeBuildOptions) *TreeNode {
	var buildOpts TreeBuildOptions
	if len(opts) > 0 {
		buildOpts = opts[0]
	}
	repoRoot := workspace.GetRootDir()

	// Acquire a directory-based lock so only one CLI process builds the tree at a time.
	// Other concurrent processes will block here and then use the cache.
	lockPath := filepath.Join(getCacheDir(), "tree.lock")
	os.MkdirAll(filepath.Dir(lockPath), 0755)
	for {
		err := os.Mkdir(lockPath, 0755)
		if err == nil {
			break
		}
		// If the lock directory exists, another process holds the lock.
		// Check if it's stale (older than 60 seconds) and remove it.
		if info, statErr := os.Stat(lockPath); statErr == nil {
			if time.Since(info.ModTime()) > 60*time.Second {
				os.RemoveAll(lockPath)
				continue
			}
		}
		time.Sleep(100 * time.Millisecond)
	}
	defer os.RemoveAll(lockPath)

	fp, newMeta := computeCompositeFingerprint(repoRoot)
	newMeta.IncludeSections = buildOpts.IncludeSections
	cached, cachedMeta, err := loadTreeCache()
	if err == nil && cachedMeta.Fingerprint == fp && cachedMeta.IncludeSections == buildOpts.IncludeSections {
		return cached
	}
	tree := BuildMonorepoTreeFromRepo(ctx, buildOpts)
	saveTreeCache(tree, newMeta)
	return tree
}

// #endregion 📌️Tree Cache

// #region 📌️PortedCache

// 💿️TreeCacheMeta is the metadata a cached tree carries.
type TreeCacheMeta struct {
	SchemaVersion   int    `json:"SchemaVersion"`
	Fingerprint     string `json:"Fingerprint"`
	IncludeSections bool   `json:"IncludeSections"`
	ContentDigest   string `json:"ContentDigest,omitempty"`
}

// 🆕️TreeCacheMetaOf MUST build the metadata of a freshly computed tree.
func TreeCacheMetaOf(tree *TreeNode, fingerprint string, includeSections bool) TreeCacheMeta {
	return TreeCacheMeta{SchemaVersion: TreeCacheSchemaVersion, Fingerprint: fingerprint, IncludeSections: includeSections, ContentDigest: TreeContentDigest(tree)}
}

// ♻️TreeCacheIsValid MUST return true only when schema, fingerprint and section choice all match.
func TreeCacheIsValid(meta TreeCacheMeta, fingerprint string, includeSections bool) bool {
	return meta.SchemaVersion == TreeCacheSchemaVersion && meta.Fingerprint == fingerprint && meta.IncludeSections == includeSections
}

// 🔏️TreeContentDigest MUST return the SHA-256 of the canonical JSON encoding of a tree.
func TreeContentDigest(tree *TreeNode) string {
	sum := sha256.Sum256([]byte(CanonicalTreeJSON(tree)))
	return hex.EncodeToString(sum[:])
}

// 🔣️CanonicalTreeJSON MUST encode a tree with every map key in sorted order.
func CanonicalTreeJSON(tree *TreeNode) string {
	var out strings.Builder
	writeCanonicalTreeNode(&out, tree)
	return out.String()
}

// 🔣️writeCanonicalTreeNode writes one node in the wire shape both implementations share.
func writeCanonicalTreeNode(out *strings.Builder, node *TreeNode) {
	if node == nil {
		out.WriteString("null")
		return
	}
	out.WriteString("{\"Kind\":")
	writeCanonicalString(out, string(node.Kind))
	out.WriteString(",\"ID\":")
	writeCanonicalString(out, node.ID)
	out.WriteString(",\"Label\":")
	writeCanonicalString(out, node.Label)
	out.WriteString(",\"URI\":")
	writeCanonicalString(out, node.URI)
	out.WriteString(",\"SubKind\":")
	writeCanonicalString(out, node.SubKind)
	out.WriteString(",\"Description\":")
	writeCanonicalString(out, node.Description)
	out.WriteString(",\"Summary\":")
	writeCanonicalString(out, node.Summary)
	out.WriteString(",\"Year\":" + strconv.Itoa(node.Year))
	out.WriteString(",\"Month\":" + strconv.Itoa(node.Month))
	out.WriteString(",\"Day\":" + strconv.Itoa(node.Day))
	out.WriteString(",\"Status\":")
	writeCanonicalString(out, node.Status)
	out.WriteString(",\"Contributor\":")
	writeCanonicalString(out, node.Contributor)
	out.WriteString(",\"Data\":")
	writeCanonicalData(out, node.Data)
	out.WriteString(",\"Children\":")
	if node.Children == nil {
		out.WriteString("null")
	} else {
		out.WriteString("[")
		for index, child := range node.Children {
			if index > 0 {
				out.WriteString(",")
			}
			writeCanonicalTreeNode(out, child)
		}
		out.WriteString("]")
	}
	out.WriteString("}")
}

// 🔣️writeCanonicalData writes a data map with sorted keys.
func writeCanonicalData(out *strings.Builder, data map[string]interface{}) {
	if data == nil {
		out.WriteString("null")
		return
	}
	keys := make([]string, 0, len(data))
	for key := range data {
		keys = append(keys, key)
	}
	sort.Strings(keys)
	out.WriteString("{")
	for index, key := range keys {
		if index > 0 {
			out.WriteString(",")
		}
		writeCanonicalString(out, key)
		out.WriteString(":")
		writeCanonicalValue(out, data[key])
	}
	out.WriteString("}")
}

// 🔣️writeCanonicalValue writes one JSON value the way the Rust encoder writes it.
func writeCanonicalValue(out *strings.Builder, value interface{}) {
	switch typed := value.(type) {
	case nil:
		out.WriteString("null")
	case string:
		writeCanonicalString(out, typed)
	case bool:
		out.WriteString(strconv.FormatBool(typed))
	case float64:
		out.WriteString(canonicalFloat(typed))
	case int:
		out.WriteString(canonicalFloat(float64(typed)))
	case int64:
		out.WriteString(canonicalFloat(float64(typed)))
	case []interface{}:
		out.WriteString("[")
		for index, item := range typed {
			if index > 0 {
				out.WriteString(",")
			}
			writeCanonicalValue(out, item)
		}
		out.WriteString("]")
	case []string:
		out.WriteString("[")
		for index, item := range typed {
			if index > 0 {
				out.WriteString(",")
			}
			writeCanonicalString(out, item)
		}
		out.WriteString("]")
	case map[string]interface{}:
		writeCanonicalData(out, typed)
	default:
		encoded, err := json.Marshal(typed)
		if err != nil {
			out.WriteString("null")
			return
		}
		out.Write(encoded)
	}
}

// 🔢️canonicalFloat formats a float the way the Rust encoder formats an f64.
func canonicalFloat(value float64) string {
	if value == float64(int64(value)) {
		return strconv.FormatInt(int64(value), 10) + ".0"
	}
	return strconv.FormatFloat(value, 'g', -1, 64)
}

// 🔣️writeCanonicalString writes a JSON string escaped the way the Rust encoder escapes it.
func writeCanonicalString(out *strings.Builder, value string) {
	out.WriteByte('"')
	for _, current := range value {
		switch current {
		case '"':
			out.WriteString("\\\"")
		case '\\':
			out.WriteString("\\\\")
		case '\n':
			out.WriteString("\\n")
		case '\r':
			out.WriteString("\\r")
		case '\t':
			out.WriteString("\\t")
		case '\b':
			out.WriteString("\\b")
		case '\f':
			out.WriteString("\\f")
		default:
			if current < 0x20 {
				out.WriteString(fmt.Sprintf("\\u%04x", current))
				continue
			}
			out.WriteRune(current)
		}
	}
	out.WriteByte('"')
}

// 🗜️EncodeTreeCachePayload MUST return the gzip-wrapped canonical JSON of a tree.
func EncodeTreeCachePayload(tree *TreeNode) ([]byte, error) {
	var buffer bytes.Buffer
	writer := gzip.NewWriter(&buffer)
	if _, err := writer.Write([]byte(CanonicalTreeJSON(tree))); err != nil {
		return nil, err
	}
	if err := writer.Close(); err != nil {
		return nil, err
	}
	return buffer.Bytes(), nil
}

// 🗜️DecodeTreeCachePayload MUST return an error when the payload is not a gzip-wrapped tree.
func DecodeTreeCachePayload(payload []byte) (*TreeNode, error) {
	reader, err := gzip.NewReader(bytes.NewReader(payload))
	if err != nil {
		return nil, err
	}
	defer func() { _ = reader.Close() }()
	plain, err := io.ReadAll(reader)
	if err != nil {
		return nil, err
	}
	node := &TreeNode{}
	if err := json.Unmarshal(plain, node); err != nil {
		return nil, err
	}
	return node, nil
}

// #endregion 📌️PortedCache

// 📅️checkpointDatePart reads one `YYYY-MM-DD` component out of the ISO timestamp a checkpoint
// carries as text, and reports zero when the timestamp is shorter than the component.
func checkpointDatePart(date string, from int, to int) int {
	if len(date) < to {
		return 0
	}
	value, err := strconv.Atoi(date[from:to])
	if err != nil {
		return 0
	}
	return value
}
