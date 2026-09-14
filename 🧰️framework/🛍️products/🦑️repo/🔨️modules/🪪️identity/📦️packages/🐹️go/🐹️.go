// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Package identity owns platform-entropy identifiers, relative time rendering, and the
// entity-emoji codec that turns repository artifacts into semantic ids and back.

// #endregion 🧲️Header

package identity

import (
	"crypto/rand"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"sync"
	"time"
)

// #region 🆔️Identifier

// 🆔️ID is a 128-bit random identifier rendered in the canonical dashed hexadecimal form.
type ID [16]byte

// 🎲️Entropy supplies the raw bytes an identifier is built from so tests can inject a seed.
type Entropy interface {
	Fill(buffer []byte) error
}

// 🖥️platformEntropy draws identifier bytes from the operating system generator.
type platformEntropy struct{}

// 🎰️Fill fills the buffer from the operating system generator.
func (platformEntropy) Fill(buffer []byte) error {
	_, err := rand.Read(buffer)
	return err
}

// 🌱️SeededEntropy is a deterministic, reproducible entropy source for tests and fixtures.
type SeededEntropy struct {
	state uint64
}

// 🎯️NewSeededEntropy returns a deterministic entropy source for the given seed.
func NewSeededEntropy(seed uint64) *SeededEntropy {
	return &SeededEntropy{state: seed}
}

// 🎰️Fill fills the buffer from a splitmix64 stream so both implementations agree byte for byte.
func (entropy *SeededEntropy) Fill(buffer []byte) error {
	for index := range buffer {
		entropy.state += 0x9E3779B97F4A7C15
		value := entropy.state
		value = (value ^ (value >> 30)) * 0xBF58476D1CE4E5B9
		value = (value ^ (value >> 27)) * 0x94D049BB133111EB
		value ^= value >> 31
		buffer[index] = byte(value >> 56)
	}
	return nil
}

// 🖥️PlatformEntropy returns the operating system entropy source.
func PlatformEntropy() Entropy { return platformEntropy{} }

// 🆕️New returns a version 4 identifier drawn from the platform entropy source.
func New() ID {
	value, err := NewFrom(PlatformEntropy())
	if err != nil {
		panic(err)
	}
	return value
}

// 🎲️NewFrom returns a version 4 identifier drawn from an explicit entropy source.
func NewFrom(entropy Entropy) (ID, error) {
	var value ID
	if err := entropy.Fill(value[:]); err != nil {
		return value, err
	}
	value[6] = value[6]&0x0f | 0x40
	value[8] = value[8]&0x3f | 0x80
	return value, nil
}

// 🔤️String renders the identifier in the canonical 8-4-4-4-12 hexadecimal form.
func (value ID) String() string {
	encoded := make([]byte, 36)
	hex.Encode(encoded[0:8], value[0:4])
	encoded[8] = '-'
	hex.Encode(encoded[9:13], value[4:6])
	encoded[13] = '-'
	hex.Encode(encoded[14:18], value[6:8])
	encoded[18] = '-'
	hex.Encode(encoded[19:23], value[8:10])
	encoded[23] = '-'
	hex.Encode(encoded[24:36], value[10:16])
	return string(encoded)
}

// #endregion 🆔️Identifier

// #region ⏳️RelativeTime

// 🕰️Clock supplies the reference instant relative time is measured against.
type Clock interface {
	Now() time.Time
}

// ⏰️systemClock reads the host wall clock.
type systemClock struct{}

// 🕐️Now returns the host wall clock instant.
func (systemClock) Now() time.Time { return time.Now() }

// 🧊️FixedClock returns a constant instant so relative-time rendering is reproducible.
type FixedClock struct{ Instant time.Time }

// 🕐️Now returns the constant instant.
func (clock FixedClock) Now() time.Time { return clock.Instant }

// ⏰️SystemClock returns the host wall clock.
func SystemClock() Clock { return systemClock{} }

// ⏳️Time renders a stable, concise relative timestamp against the host wall clock.
func Time(value time.Time) string {
	return TimeAt(value, SystemClock())
}

// ⌛️TimeAt renders a stable, concise relative timestamp against an explicit clock.
func TimeAt(value time.Time, clock Clock) string {
	return Duration(clock.Now().Sub(value))
}

// 🧮️Duration renders an elapsed duration as a concise, pluralised relative phrase.
func Duration(delta time.Duration) string {
	future := delta < 0
	if future {
		delta = -delta
	}
	amount, unit := 0, ""
	switch {
	case delta < time.Minute:
		amount, unit = max(1, int(delta/time.Second)), "second"
	case delta < time.Hour:
		amount, unit = int(delta/time.Minute), "minute"
	case delta < 24*time.Hour:
		amount, unit = int(delta/time.Hour), "hour"
	case delta < 30*24*time.Hour:
		amount, unit = int(delta/(24*time.Hour)), "day"
	case delta < 365*24*time.Hour:
		amount, unit = int(delta/(30*24*time.Hour)), "month"
	default:
		amount, unit = int(delta/(365*24*time.Hour)), "year"
	}
	if amount != 1 {
		unit += "s"
	}
	if future {
		return fmt.Sprintf("%d %s from now", amount, unit)
	}
	return fmt.Sprintf("%d %s ago", amount, unit)
}

// #endregion ⏳️RelativeTime

// #region 🔣️EmojiTable

// 🔣️EntityEmojiTable is the shared entity-emoji vocabulary both implementations load.
type EntityEmojiTable struct {
	Schema             string            `json:"schema"`
	TextDefaultEmojis  []string          `json:"textDefaultEmojis"`
	Entities           map[string]string `json:"entities"`
	Collections        map[string]string `json:"collections"`
	AllEntityOrder     []string          `json:"allEntityOrder"`
	SectionRefEmojis   []string          `json:"sectionRefEmojis"`
	DefinitionRefEmoji []string          `json:"definitionRefEmojis"`
	FolderRefEmojis    []string          `json:"folderRefEmojis"`
	FileRefEmojis      []string          `json:"fileRefEmojis"`
}

var (
	tableOnce  sync.Once
	tableValue EntityEmojiTable
	tableError error
)

// 🌍️EntityEmojiTableEnv overrides the emoji table location for a build whose source paths were trimmed.
const EntityEmojiTableEnv = "SEMIO_REPO_IDENTITY_EMOJI_TABLE"

// 📍️EntityEmojiTablePath returns the module-relative location of the shared emoji table.
func EntityEmojiTablePath() string {
	if override := strings.TrimSpace(os.Getenv(EntityEmojiTableEnv)); override != "" {
		return override
	}
	_, file, _, ok := runtime.Caller(0)
	if !ok {
		return ""
	}
	return filepath.Join(filepath.Dir(file), "..", "..", "🧬️schema", "🔣️entity-emojis.json")
}

// 📥️LoadEntityEmojiTable reads the shared emoji table once and caches it for the process.
func LoadEntityEmojiTable() (EntityEmojiTable, error) {
	tableOnce.Do(func() {
		path := EntityEmojiTablePath()
		if path == "" {
			tableError = fmt.Errorf("identity: cannot locate the entity emoji table")
			return
		}
		data, err := os.ReadFile(path)
		if err != nil {
			tableError = err
			return
		}
		tableError = json.Unmarshal(data, &tableValue)
	})
	return tableValue, tableError
}

// 🏷️Entity returns the emoji registered for an entity kind, or the empty string.
func Entity(name string) string {
	table, err := LoadEntityEmojiTable()
	if err != nil {
		return ""
	}
	return table.Entities[name]
}

// 🗃️Collection returns the emoji registered for a collection kind, or the empty string.
func Collection(name string) string {
	table, err := LoadEntityEmojiTable()
	if err != nil {
		return ""
	}
	return table.Collections[name]
}

// 😀️AllEntityEmojis returns the deduplicated, presentation-normalised entity emoji vocabulary.
func AllEntityEmojis() []string {
	table, err := LoadEntityEmojiTable()
	if err != nil {
		return nil
	}
	seen := map[string]bool{}
	var result []string
	for _, name := range table.AllEntityOrder {
		raw, found := table.Entities[name]
		if !found {
			raw = table.Collections[name]
		}
		normalized := EmojiText(raw)
		if normalized == "" || seen[normalized] {
			continue
		}
		seen[normalized] = true
		result = append(result, normalized)
	}
	return result
}

// #endregion 🔣️EmojiTable

// #region 😀️EmojiCodec

// 😀️EmojiText normalises an emoji to its presentation form for the text-default code points.
func EmojiText(emoji string) string {
	stripped := strings.ReplaceAll(emoji, "\uFE0E", "")
	base := strings.ReplaceAll(stripped, "\uFE0F", "")
	table, err := LoadEntityEmojiTable()
	if err != nil {
		return base
	}
	for _, textDefault := range table.TextDefaultEmojis {
		if strings.Contains(base, textDefault) {
			return strings.ReplaceAll(base, textDefault, textDefault+"\uFE0F")
		}
	}
	return base
}

// 🧲️ExtractEntityEmoji splits the leading emoji grapheme from the remaining text.
func ExtractEntityEmoji(value string) (string, string) {
	if value == "" {
		return "", ""
	}
	runes := []rune(value)
	if len(runes) == 0 {
		return "", ""
	}
	index := 0
	first := runes[index]
	if strings.ContainsRune("0123456789#*", first) {
		end := 1
		if end < len(runes) && runes[end] == 0xFE0F {
			end++
		}
		if end < len(runes) && runes[end] == 0x20E3 {
			return string(runes[:end+1]), string(runes[end+1:])
		}
		return "", value
	}
	if !IsEmojiRune(first) {
		return "", value
	}
	index++
	if first >= 0x1F1E6 && first <= 0x1F1FF && index < len(runes) && runes[index] >= 0x1F1E6 && runes[index] <= 0x1F1FF {
		index++
	}
	for index < len(runes) {
		current := runes[index]
		switch {
		case current == 0xFE0F || current == 0xFE0E || current == 0x20E3:
			index++
		case current == 0x200D:
			index++
			if index < len(runes) {
				index++
				for index < len(runes) && (runes[index] == 0xFE0F || runes[index] == 0xFE0E) {
					index++
				}
			}
		case current >= 0x1F3FB && current <= 0x1F3FF:
			index++
		default:
			return string(runes[:index]), string(runes[index:])
		}
	}
	return string(runes[:index]), string(runes[index:])
}

// 🔷️IsEmojiRune reports whether a rune can start or continue an emoji grapheme.
func IsEmojiRune(value rune) bool {
	for _, span := range emojiRanges {
		if value >= span[0] && value <= span[1] {
			return true
		}
	}
	return false
}

// 🗺️emojiRanges is the closed code-point span table both implementations share.
var emojiRanges = [][2]rune{
	{0x00A9, 0x00A9}, {0x00AE, 0x00AE},
	{0x200D, 0x200D},
	{0x2139, 0x2139}, {0x2194, 0x2195}, {0x2196, 0x2199}, {0x21A9, 0x21AA},
	{0x231A, 0x231B}, {0x2300, 0x23FF},
	{0x25AA, 0x25AB}, {0x25B6, 0x25B6}, {0x25C0, 0x25C0}, {0x25FB, 0x25FE},
	{0x2600, 0x26FF}, {0x2700, 0x27BF},
	{0x2934, 0x2935}, {0x2B05, 0x2B07}, {0x2B50, 0x2B55},
	{0x3030, 0x3030}, {0x303D, 0x303D}, {0x3297, 0x3297}, {0x3299, 0x3299},
	{0xFE00, 0xFE0F},
	{0x1F1E6, 0x1F1FF},
	{0x1F300, 0x1F5FF}, {0x1F600, 0x1F64F}, {0x1F680, 0x1F6FF},
	{0x1F700, 0x1F77F}, {0x1F780, 0x1F7FF}, {0x1F800, 0x1F8FF},
	{0x1F900, 0x1F9FF}, {0x1FA00, 0x1FA6F}, {0x1FA70, 0x1FAFF},
}

// 🔤️Flat lowercases an identifier segment and drops every character outside `A-Za-z0-9` and the
// non-ASCII range, so a name that already carries its taxonomy emoji keeps it.
func Flat(value string) string {
	var builder strings.Builder
	for _, current := range value {
		if (current >= 'a' && current <= 'z') || (current >= 'A' && current <= 'Z') || (current >= '0' && current <= '9') || current > 0x7F {
			builder.WriteRune(current)
		}
	}
	return strings.ToLower(builder.String())
}

// 💠️Slugify splits camel case into `-` boundaries, upper cases, collapses every run outside
// `[A-Z0-9]` into a single `-` and trims the result. This is the repo's identifier rule: a goal id,
// a todo id, a ticket slug and a draft id are all this function applied to a human title.
func Slugify(text string) string {
	runes := []rune(text)
	var spaced strings.Builder
	for index, current := range runes {
		if index > 0 && current >= 'A' && current <= 'Z' {
			previous := runes[index-1]
			leavesAWord := previous >= 'a' && previous <= 'z'
			startsAWord := previous >= 'A' && previous <= 'Z' && index+1 < len(runes) && runes[index+1] >= 'a' && runes[index+1] <= 'z'
			if leavesAWord || startsAWord {
				spaced.WriteRune('-')
			}
		}
		spaced.WriteRune(current)
	}
	upper := strings.ToUpper(spaced.String())
	var slug strings.Builder
	inSeparator := false
	for _, character := range upper {
		if (character >= 'A' && character <= 'Z') || (character >= '0' && character <= '9') {
			slug.WriteRune(character)
			inSeparator = false
		} else if !inSeparator {
			slug.WriteRune('-')
			inSeparator = true
		}
	}
	return strings.Trim(slug.String(), "-")
}

// #endregion 😀️EmojiCodec

// #region 🧱️SemanticId

// 💿️SemanticId is one emoji-tagged identifier segment.
type SemanticId struct {
	Emoji string
	Value string
}

// 🔤️String renders the segment with its emoji in presentation form.
func (id SemanticId) String() string {
	if id.Value == "" {
		return EmojiText(id.Emoji)
	}
	return EmojiText(id.Emoji) + id.Value
}

// 🧬️ParseSemanticIds splits a compose id into its emoji-tagged segments.
func ParseSemanticIds(composeID string) []SemanticId {
	var result []SemanticId
	remaining := composeID
	for remaining != "" {
		emoji, rest := ExtractEntityEmoji(remaining)
		if emoji == "" {
			if len(result) == 0 {
				return nil
			}
			result[len(result)-1].Value += rest
			return result
		}
		value := ""
		cursor := rest
		for cursor != "" {
			nextEmoji, _ := ExtractEntityEmoji(cursor)
			if nextEmoji != "" {
				break
			}
			runes := []rune(cursor)
			value += string(runes[0])
			cursor = string(runes[1:])
		}
		result = append(result, SemanticId{Emoji: EmojiText(emoji), Value: value})
		remaining = cursor
	}
	return result
}

// 🎯️GoalPathToComposeID converts a filesystem goal path into the emoji compose id.
func GoalPathToComposeID(goalPath string) string {
	if goalPath == "" {
		return ""
	}
	prefix := EmojiText(Entity("goal"))
	result := ""
	for _, part := range strings.Split(goalPath, "/") {
		result += prefix + Flat(part)
	}
	return result
}

// 🛤️ComposeIDToGoalSegments splits a goal compose id back into its flattened segments.
func ComposeIDToGoalSegments(composeID string) []string {
	if composeID == "" {
		return nil
	}
	prefix := EmojiText(Entity("goal"))
	trimmed := strings.TrimPrefix(composeID, prefix)
	var result []string
	for _, segment := range strings.Split(trimmed, prefix) {
		if segment != "" {
			result = append(result, segment)
		}
	}
	return result
}

// 🤝️ContributorToComposeID converts a contributor alias into the emoji compose id.
func ContributorToComposeID(identifier string) string {
	if identifier == "" || identifier == "unknown" {
		return identifier
	}
	prefix := EmojiText(Entity("contributor"))
	if strings.HasPrefix(identifier, prefix) {
		return identifier
	}
	return prefix + Flat(identifier)
}

// 🐙️ComposeIDToContributorFlat converts a contributor compose id back into its flattened alias.
func ComposeIDToContributorFlat(composeID string) string {
	if composeID == "" || composeID == "unknown" {
		return composeID
	}
	prefix := EmojiText(Entity("contributor"))
	if !strings.HasPrefix(composeID, prefix) {
		return composeID
	}
	return composeID[len(prefix):]
}

// #endregion 🧱️SemanticId

// #region 🟦️ArtifactRef

// 🟦️ArtifactRef is a parsed reference to a folder, file, section or definition.
type ArtifactRef struct {
	Kind         string   `json:"kind"`
	Path         string   `json:"path"`
	SectionParts []string `json:"sectionParts,omitempty"`
}

// 🧹️NormalizePath rewrites a reference path to forward slashes without a leading `./`.
func NormalizePath(value string) string {
	normalized := strings.ReplaceAll(value, "\\", "/")
	for strings.HasPrefix(normalized, "./") {
		normalized = normalized[2:]
	}
	return normalized
}

// 💾️ParseArtifactRef classifies an emoji-prefixed or plain artifact reference.
func ParseArtifactRef(ref string) ArtifactRef {
	table, err := LoadEntityEmojiTable()
	clean := strings.ReplaceAll(ref, "\uFE0E", "")
	clean = strings.ReplaceAll(clean, "\uFE0F", "")
	if err == nil {
		if hit, ok := matchRefEmoji(clean, table.SectionRefEmojis); ok {
			parts := strings.SplitN(hit, "#", 2)
			var sections []string
			if len(parts) > 1 {
				sections = strings.Split(parts[1], "#")
			}
			return ArtifactRef{Kind: "section", Path: NormalizePath(parts[0]), SectionParts: sections}
		}
		if hit, ok := matchRefEmoji(clean, table.DefinitionRefEmoji); ok {
			return ArtifactRef{Kind: "definition", Path: NormalizePath(hit)}
		}
		if hit, ok := matchRefEmoji(clean, table.FolderRefEmojis); ok {
			return ArtifactRef{Kind: "folder", Path: NormalizePath(strings.TrimSuffix(hit, "/"))}
		}
		if hit, ok := matchRefEmoji(clean, table.FileRefEmojis); ok {
			return ArtifactRef{Kind: "file", Path: NormalizePath(hit)}
		}
	}
	if strings.Contains(ref, "#") {
		parts := strings.SplitN(ref, "#", 2)
		var sections []string
		if len(parts) > 1 {
			sections = strings.Split(parts[1], "#")
		}
		return ArtifactRef{Kind: "section", Path: NormalizePath(parts[0]), SectionParts: sections}
	}
	if strings.HasSuffix(ref, "/") {
		return ArtifactRef{Kind: "folder", Path: NormalizePath(strings.TrimSuffix(ref, "/"))}
	}
	return ArtifactRef{Kind: "file", Path: NormalizePath(ref)}
}

// 🔎️matchRefEmoji returns the remainder after the first matching prefix emoji.
func matchRefEmoji(clean string, emojis []string) (string, bool) {
	for _, emoji := range emojis {
		bare := strings.ReplaceAll(strings.ReplaceAll(emoji, "\uFE0E", ""), "\uFE0F", "")
		if bare != "" && strings.HasPrefix(clean, bare) {
			return clean[len(bare):], true
		}
	}
	return "", false
}

// #endregion 🟦️ArtifactRef
