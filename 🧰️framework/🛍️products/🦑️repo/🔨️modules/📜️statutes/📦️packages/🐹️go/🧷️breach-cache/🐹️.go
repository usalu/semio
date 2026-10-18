package statutes

import (
	bytes "bytes"
	gzip "compress/gzip"
	sha256 "crypto/sha256"
	hex "encoding/hex"
	json "encoding/json"
	io "io"
	os "os"
	filepath "path/filepath"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🧷️Breach Cache

// 🧷️BreachCacheEnvelope is the on-disk JSON shape of one breach-cache document: the entity the
// analysis ran over, the script that produced it and the breaches it found.
type BreachCacheEnvelope struct {
	EntityID string         `json:"entityId"`
	Script   string         `json:"script"`
	Breachs  []model.Breach `json:"breachs"`
}

// 🔐️SHA256Hex returns the lowercase hexadecimal SHA-256 digest of one byte string.
func SHA256Hex(data []byte) string {
	digest := sha256.Sum256(data)
	return hex.EncodeToString(digest[:])
}

// 🗜️GzipEncode compresses one payload into a single gzip member.
func GzipEncode(data []byte) []byte {
	var out bytes.Buffer
	writer := gzip.NewWriter(&out)
	if _, err := writer.Write(data); err != nil {
		return nil
	}
	if err := writer.Close(); err != nil {
		return nil
	}
	return out.Bytes()
}

// 📂️GzipDecode MUST return a non-nil error when the member is not a readable gzip stream.
// 📂️GzipDecode inflates one gzip member back into the payload it carries.
func GzipDecode(data []byte) ([]byte, error) {
	reader, err := gzip.NewReader(bytes.NewReader(data))
	if err != nil {
		return nil, err
	}
	defer func() { _ = reader.Close() }()
	payload, err := io.ReadAll(reader)
	if err != nil {
		return nil, err
	}
	return payload, nil
}

// 📄️EncodeBreachCacheJSON MUST return a non-nil error when the document cannot be encoded.
// 📄️EncodeBreachCacheJSON returns the canonical JSON encoding of one breach-cache document:
// declaration order, no HTML escaping and no trailing newline, so the digest is the same everywhere.
func EncodeBreachCacheJSON(envelope BreachCacheEnvelope) (string, error) {
	var out bytes.Buffer
	encoder := json.NewEncoder(&out)
	encoder.SetEscapeHTML(false)
	if err := encoder.Encode(envelope); err != nil {
		return "", err
	}
	return strings.TrimSuffix(out.String(), "\n"), nil
}

// 📄️ParseBreachCache MUST return a non-nil error when the document is not one envelope.
// 📄️ParseBreachCache returns the breach-cache document one canonical JSON encoding carries.
func ParseBreachCache(document string) (BreachCacheEnvelope, error) {
	var envelope BreachCacheEnvelope
	if err := json.Unmarshal([]byte(document), &envelope); err != nil {
		return BreachCacheEnvelope{}, err
	}
	if envelope.Breachs == nil {
		envelope.Breachs = []model.Breach{}
	}
	return envelope, nil
}

// 🔐️BreachCacheDigest MUST return a non-nil error when the document cannot be encoded.
// 🔐️BreachCacheDigest returns the digest that keys one breach-cache document, over its
// canonical JSON encoding.
func BreachCacheDigest(envelope BreachCacheEnvelope) (string, error) {
	encoded, err := EncodeBreachCacheJSON(envelope)
	if err != nil {
		return "", err
	}
	return SHA256Hex([]byte(encoded)), nil
}

// 🗜️EncodeBreachCache MUST return a non-nil error when the document cannot be encoded.
// 🗜️EncodeBreachCache returns the compressed breach-cache member: gzip over the canonical
// JSON encoding.
func EncodeBreachCache(envelope BreachCacheEnvelope) ([]byte, error) {
	encoded, err := EncodeBreachCacheJSON(envelope)
	if err != nil {
		return nil, err
	}
	return GzipEncode([]byte(encoded)), nil
}

// 📂️DecodeBreachCache MUST return a non-nil error when the member is not one envelope.
// 📂️DecodeBreachCache returns the breach-cache document one compressed member carries.
func DecodeBreachCache(data []byte) (BreachCacheEnvelope, error) {
	payload, err := GzipDecode(data)
	if err != nil {
		return BreachCacheEnvelope{}, err
	}
	return ParseBreachCache(string(payload))
}

func LoadBreachsFromCache(repoRoot string) ([]model.Breach, error) {
	dir := filepath.Join(workspace.RepoMetaDirForRoot(repoRoot), "⚡️cache", "breaches")
	entries, err := os.ReadDir(dir)
	if err != nil {
		if os.IsNotExist(err) {
			return nil, nil
		}
		return nil, err
	}
	var all []model.Breach
	for _, e := range entries {
		if e.IsDir() || !strings.HasSuffix(e.Name(), ".json") {
			continue
		}
		data, err := os.ReadFile(filepath.Join(dir, e.Name()))
		if err != nil {
			return nil, err
		}
		env, err := ParseBreachCache(string(data))
		if err != nil {
			return nil, err
		}
		all = append(all, env.Breachs...)
	}
	return all, nil
}

func BreachMatchesScope(b *model.Breach, s workspace.Scope) bool {
	bp := workspace.NormalizePath(b.Scope)
	switch s.Kind {
	case workspace.ScopeRepo:
		return true
	case workspace.ScopeFile:
		fp := workspace.NormalizePath(s.FilePath)
		return bp == fp || strings.HasPrefix(bp, fp+"#") || strings.HasPrefix(bp, fp+"::")
	case workspace.ScopeFolder:
		fp := strings.TrimSuffix(workspace.NormalizePath(s.FilePath), "/")
		if fp == "" {
			return true
		}
		return bp == fp || strings.HasPrefix(bp, fp+"/")
	case workspace.ScopeTechnology:
		tn := strings.TrimSuffix(workspace.NormalizePath(s.TechnologyName), "/")
		if tn == "" {
			return true
		}
		return bp == tn || strings.HasPrefix(bp, tn+"/")
	case workspace.ScopeSection, workspace.ScopeDefinition:
		fp := workspace.NormalizePath(s.FilePath)
		return strings.HasPrefix(bp, fp)
	default:
		return true
	}
}

// #endregion 🧷️Breach Cache
