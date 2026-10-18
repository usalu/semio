// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🌲️snapshot carries the filesystem snapshot and the test scope resolution over it.

// #endregion 🧲️Header

package testrunner

import (
	gopath "path"
	sort "sort"
	strings "strings"
)

// #region 🌲️Snapshot

// 🌲️EntryKind says whether a snapshot entry is a file or a directory.
type EntryKind string

// 📄️EntryKindFile is a regular file.
const EntryKindFile EntryKind = "file"

// 📁️EntryKindDirectory is a directory.
const EntryKindDirectory EntryKind = "directory"

// 📄️SnapshotEntry is one path in a planning snapshot, with contents only where a rule reads them.
type SnapshotEntry struct {
	// 🛤️Path is repository-relative or absolute, always forward-slashed.
	Path string `json:"path"`
	// 🌲️Kind is file or directory.
	Kind EntryKind `json:"kind"`
	// 📝️Contents is the text, present only for files a planning rule inspects.
	Contents *string `json:"contents,omitempty"`
}

// 📦️SnapshotBundle is the two bundle fields planning needs, projected from 📐️model's richer Bundle.
type SnapshotBundle struct {
	// 🏷️Name is the technology/bundle name.
	Name string `json:"name"`
	// 🛤️Root is the repository-relative bundle root.
	Root string `json:"root"`
}

// 🌍️FilesystemSnapshot is everything planning is allowed to know about the world.
type FilesystemSnapshot struct {
	// 🏠️Root is the repository root every relative path is joined onto.
	Root string `json:"root"`
	// 🌲️Entries are every path the plan may probe.
	Entries []SnapshotEntry `json:"entries"`
	// 📦️Bundles is the bundle table.
	Bundles []SnapshotBundle `json:"bundles"`
	// 🐍️UvAvailable says whether uv is on PATH.
	UvAvailable bool `json:"uvAvailable"`
}

// 🧭️Absolute is the absolute form of a repository-relative path.
func (snapshot *FilesystemSnapshot) Absolute(path string) string {
	if IsAbsolutePath(path) {
		return CleanPath(path)
	}
	return JoinPath(snapshot.Root, path)
}

// ✅️FileExists is true only for a present regular file.
func (snapshot *FilesystemSnapshot) FileExists(path string) bool {
	wanted := CleanPath(path)
	for _, entry := range snapshot.Entries {
		if entry.Kind == EntryKindFile && CleanPath(entry.Path) == wanted {
			return true
		}
	}
	return false
}

// 📁️DirectoryExists says whether a directory is present.
func (snapshot *FilesystemSnapshot) DirectoryExists(path string) bool {
	wanted := CleanPath(path)
	for _, entry := range snapshot.Entries {
		if entry.Kind == EntryKindDirectory && CleanPath(entry.Path) == wanted {
			return true
		}
	}
	return false
}

// 📝️ReadText answers the contents of a recorded file.
func (snapshot *FilesystemSnapshot) ReadText(path string) (string, bool) {
	wanted := CleanPath(path)
	for _, entry := range snapshot.Entries {
		if entry.Kind != EntryKindFile || CleanPath(entry.Path) != wanted {
			continue
		}
		if entry.Contents == nil {
			return "", false
		}
		return *entry.Contents, true
	}
	return "", false
}

// 🔎️GlobChildren is a non-recursive match against the direct children of a directory.
func (snapshot *FilesystemSnapshot) GlobChildren(directory string, pattern string) []string {
	directory = CleanPath(directory)
	matched := []string{}
	for _, entry := range snapshot.Entries {
		if entry.Kind != EntryKindFile {
			continue
		}
		candidate := CleanPath(entry.Path)
		if DirName(candidate) != directory {
			continue
		}
		if ok, err := gopath.Match(pattern, BaseName(candidate)); err == nil && ok {
			matched = append(matched, candidate)
		}
	}
	sort.Strings(matched)
	return matched
}

// 🚶️WalkFiles is every recorded file at or below root, sorted.
func (snapshot *FilesystemSnapshot) WalkFiles(root string, recursive bool) []string {
	root = CleanPath(root)
	prefix := strings.TrimRight(root, "/") + "/"
	found := []string{}
	for _, entry := range snapshot.Entries {
		if entry.Kind != EntryKindFile {
			continue
		}
		candidate := CleanPath(entry.Path)
		if candidate != root && !strings.HasPrefix(candidate, prefix) {
			continue
		}
		if !recursive && DirName(candidate) != root {
			continue
		}
		found = append(found, candidate)
	}
	sort.Strings(found)
	return found
}

// 📦️BundleByName answers on exact name first, then on flattened equality.
func (snapshot *FilesystemSnapshot) BundleByName(name string) *SnapshotBundle {
	for index := range snapshot.Bundles {
		bundle := &snapshot.Bundles[index]
		if bundle.Name == name || Flat(bundle.Name) == Flat(name) {
			return bundle
		}
	}
	return nil
}

// 📦️BundleByPath answers the longest bundle root that prefixes the path.
func (snapshot *FilesystemSnapshot) BundleByPath(path string) *SnapshotBundle {
	path = CleanPath(NormalizeSeparators(path))
	var best *SnapshotBundle
	bestLength := -1
	for index := range snapshot.Bundles {
		bundle := &snapshot.Bundles[index]
		root := CleanPath(bundle.Root)
		if path != root && !strings.HasPrefix(path, strings.TrimRight(root, "/")+"/") {
			continue
		}
		if len(root) >= bestLength {
			best, bestLength = bundle, len(root)
		}
	}
	return best
}

// 🗣️DetectBundleLanguage probes manifests in a fixed, order-sensitive sequence.
func (snapshot *FilesystemSnapshot) DetectBundleLanguage(bundleRoot string) string {
	absolute := snapshot.Absolute(bundleRoot)
	if snapshot.FileExists(JoinPath(absolute, "go.mod")) {
		return "go"
	}
	if snapshot.FileExists(JoinPath(absolute, "Cargo.toml")) {
		return "rust"
	}
	if len(snapshot.GlobChildren(absolute, "*.csproj")) > 0 {
		return "csharp"
	}
	if len(snapshot.GlobChildren(absolute, "*.sln")) > 0 {
		return "csharp"
	}
	if snapshot.FileExists(JoinPath(absolute, "package.json")) {
		return "typescript"
	}
	if snapshot.FileExists(JoinPath(absolute, "pyproject.toml")) || snapshot.FileExists(JoinPath(absolute, "requirements.txt")) {
		return "python"
	}
	return ""
}

// #endregion 🌲️Snapshot

// #region 🔭️Scope

// 🔭️ScopeKind says how wide a test run reaches.
type ScopeKind string

// 🌍️ScopeKindAll is every bundle with a detectable language.
const ScopeKindAll ScopeKind = "all"

// 🧰️ScopeKindTechnology is every bundle of one technology.
const ScopeKindTechnology ScopeKind = "technology"

// ⌨️ScopeKindBundle is one bundle.
const ScopeKindBundle ScopeKind = "bundle"

// 🥼️ScopeKindFile is one file.
const ScopeKindFile ScopeKind = "file"

// 🔖️ScopeKindSection is one section of one file.
const ScopeKindSection ScopeKind = "section"

// 🧪️ScopeKindDefinition is one test function.
const ScopeKindDefinition ScopeKind = "definition"

// 🧪️TestScope is a resolved scope.
type TestScope struct {
	// 🔭️Kind is the granularity.
	Kind ScopeKind `json:"kind"`
	// 📦️BundleRoot is the bundle root, or the technology name for a technology scope.
	BundleRoot string `json:"bundle_root"`
	// 🥼️FilePath is the file path.
	FilePath string `json:"file_path"`
	// 🔖️Section is the section name.
	Section string `json:"section"`
	// 🧪️TestName is the flat test name.
	TestName string `json:"test_name"`
	// 🗣️Language is the language.
	Language string `json:"language"`
}

// 🌍️AllScope is the scope an empty or unrecognised selector resolves to.
func AllScope() TestScope { return TestScope{Kind: ScopeKindAll} }

// 🔗️ResolveTestScopes answers one `all` scope when no selector is given.
func (snapshot *FilesystemSnapshot) ResolveTestScopes(ids []string, identity EntityIdentity, languages LanguageTable) []TestScope {
	if len(ids) == 0 {
		return []TestScope{AllScope()}
	}
	scopes := make([]TestScope, 0, len(ids))
	for _, raw := range ids {
		scopes = append(scopes, snapshot.ResolveTestScope(raw, identity, languages))
	}
	return scopes
}

// 🔷️ResolveTestScope strips variation selectors, parses `repo://` and follows the `p/` and `f/` routes.
func (snapshot *FilesystemSnapshot) ResolveTestScope(raw string, identity EntityIdentity, languages LanguageTable) TestScope {
	normalized := strings.TrimSpace(strings.Map(stripVariationSelector, raw))
	uri := normalized
	if !strings.HasPrefix(normalized, "repo://") {
		uri = identity.IdToUri(normalized)
	}
	if uri == "" {
		return AllScope()
	}
	path := strings.TrimPrefix(uri, "repo://")
	if rest, ok := strings.CutPrefix(path, "p/"); ok {
		parts := strings.SplitN(rest, "/", 3)
		if len(parts) == 2 {
			return TestScope{Kind: ScopeKindTechnology, BundleRoot: identity.PathFromUriPath(parts[1])}
		}
		if len(parts) >= 3 {
			if bundleRest, ok := strings.CutPrefix(parts[2], "b/"); ok {
				bundleParts := strings.SplitN(bundleRest, "/", 3)
				if len(bundleParts) >= 2 {
					technology := identity.PathFromUriPath(parts[1])
					bundleName := strings.TrimLeft(technology, "@") + "/" + identity.PathFromUriPath(bundleParts[1])
					bundleRoot := bundleName
					if found := snapshot.BundleByName(bundleName); found != nil {
						bundleRoot = found.Root
					}
					if len(bundleParts) == 2 {
						return TestScope{Kind: ScopeKindBundle, BundleRoot: bundleRoot, Language: snapshot.DetectBundleLanguage(bundleRoot)}
					}
					return snapshot.ResolveTestScopeFromBundleSubPath(bundleRoot, bundleParts[2], identity, languages)
				}
			}
		}
	}
	if rest, ok := strings.CutPrefix(path, "f/"); ok {
		parts := strings.SplitN(rest, "/", 2)
		filePath := identity.PathFromUriPath(parts[0])
		language, _ := languages.LanguageOf(filePath)
		if len(parts) == 1 {
			return TestScope{Kind: ScopeKindFile, FilePath: filePath, Language: language}
		}
		return ResolveTestScopeFromFileSubPath(filePath, language, parts[1], identity)
	}
	return AllScope()
}

func stripVariationSelector(value rune) rune {
	if value == '\uFE0E' || value == '\uFE0F' {
		return -1
	}
	return value
}

// 📦️ResolveTestScopeFromBundleSubPath narrows on the `f`, `s`, `d` and `fd` markers in order.
func (snapshot *FilesystemSnapshot) ResolveTestScopeFromBundleSubPath(bundleRoot string, subPath string, identity EntityIdentity, languages LanguageTable) TestScope {
	language := snapshot.DetectBundleLanguage(bundleRoot)
	parts := strings.Split(subPath, "/")
	scope := TestScope{Kind: ScopeKindBundle, BundleRoot: bundleRoot}
	for index := 0; index < len(parts); index++ {
		switch {
		case parts[index] == "f" && index+1 < len(parts):
			scope.FilePath = JoinPath(bundleRoot, identity.PathFromUriPath(parts[index+1]))
			scope.Kind = ScopeKindFile
			index++
		case parts[index] == "s" && index+1 < len(parts):
			scope.Section = identity.PathFromUriPath(parts[index+1])
			scope.Kind = ScopeKindSection
			index++
		case parts[index] == "d" && index+2 < len(parts):
			scope.TestName = identity.PathFromUriPath(parts[index+2])
			scope.Kind = ScopeKindDefinition
			index += 2
		case parts[index] == "fd" && index+2 < len(parts):
			index += 2
		}
	}
	if found, ok := languages.LanguageOf(scope.FilePath); ok {
		language = found
	}
	scope.Language = language
	return scope
}

// 📄️ResolveTestScopeFromFileSubPath narrows a file scope on the `s` and `d` markers.
func ResolveTestScopeFromFileSubPath(filePath string, language string, subPath string, identity EntityIdentity) TestScope {
	parts := strings.Split(subPath, "/")
	scope := TestScope{Kind: ScopeKindFile, FilePath: filePath, Language: language}
	for index := 0; index < len(parts); index++ {
		switch {
		case parts[index] == "s" && index+1 < len(parts):
			scope.Section = identity.PathFromUriPath(parts[index+1])
			scope.Kind = ScopeKindSection
			index++
		case parts[index] == "d" && index+2 < len(parts):
			scope.TestName = identity.PathFromUriPath(parts[index+2])
			scope.Kind = ScopeKindDefinition
			index += 2
		}
	}
	return scope
}

// #endregion 🔭️Scope
