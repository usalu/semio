// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🪪️scope_ids carries the scope filters and the codebase identifier builders.

// #endregion 🧲️Header

package model

import (
	fmt "fmt"
	filepath "path/filepath"
	strings "strings"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🪪️Scope Ids

// ❌️ScopeToFiles MUST return a non-nil error when the operation fails.
func ScopeToFiles(rootDir string, scope workspace.Scope, bundles []Bundle) ([]string, error) {
	ignorePatterns := []string{"**/node_modules/**", "**/.venv/**"}
	var files []string
	var err error
	switch scope.Kind {
	case workspace.ScopeRepo:
		files, err = workspace.GlobByExtension(rootDir, "**/*", []string{"ts", "tsx", "py", "cs", "go", "rs"}, ignorePatterns, true)
	case workspace.ScopeTechnology:
		for _, proj := range bundles {
			if proj.Name == scope.TechnologyName {
				files, err = workspace.GlobByExtension(rootDir, proj.Root+"/**/*", []string{"ts", "tsx", "py", "cs", "go", "rs"}, ignorePatterns, true)
				break
			}
		}
	case workspace.ScopeFolder:
		if scope.FilePath != "" {
			files, err = workspace.GlobByExtension(rootDir, scope.FilePath+"**/*", []string{"ts", "tsx", "py", "cs", "go", "rs"}, ignorePatterns, true)
		}
	case workspace.ScopeFile, workspace.ScopeSection, workspace.ScopeDefinition:
		if scope.FilePath != "" {
			files = []string{scope.FilePath}
		}
	}
	if err != nil {
		return nil, err
	}
	files = FilterConsideredFiles(rootDir, files)
	files = FilterGitIgnored(rootDir, files)
	return files, nil
}

// 🧹️filterConsideredFiles holds the data fields for a filterConsideredFiles record.
func FilterConsideredFiles(rootDir string, files []string) []string {
	if len(files) == 0 {
		return files
	}
	filtered := make([]string, 0, len(files))
	patterns, _ := workspace.LoadGitignore(rootDir)
	ignore := workspace.CompileIgnoreLines(patterns...)
	for _, filePath := range files {
		if workspace.IsRepoExcludedPath(filePath) {
			continue
		}
		if ignore.MatchesPath(workspace.NormalizePath(filePath)) {
			continue
		}
		filtered = append(filtered, filePath)
	}
	return filtered
}

// 🏗️buildBreachID holds the data fields for a buildBreachID record.
func BuildBreachID(scope string, line int, col int) string {
	if line > 0 && col > 0 {
		return fmt.Sprintf("repo/breach/%s#%d:%d", scope, line, col)
	}
	if line > 0 {
		return fmt.Sprintf("repo/breach/%s#%d", scope, line)
	}
	return fmt.Sprintf("repo/breach/%s", scope)
}

// 📦️GetBundleByPath MUST retrieve the requested value or return an error.
// 🔻️GetBundleByPath retrieves and returns the bundle by path.
func GetBundleByPath(path string) *Bundle { return (ArtifactIndex{RootDir:workspace.GetRootDir(), Bundles: LookupBundles()}).GetBundleByPath(path) }

// 🔷️findBundleInfo holds the data fields for a findBundleInfo record.
func findBundleInfo(path string) (name, root string, ok bool) {
	bundles := LookupBundles()
	normalizedPath := workspace.NormalizePath(path)
	var matchedBundle string
	var matchedRoot string
	var matchedLen int
	for _, bundle := range bundles {
		root := workspace.NormalizePath(bundle.Root)
		if strings.HasPrefix(normalizedPath, root+"/") || normalizedPath == root {
			if len(root) > matchedLen {
				matchedBundle = bundle.Name
				matchedRoot = root
				matchedLen = len(root)
			}
		}
	}
	if matchedLen > 0 || matchedBundle != "" {
		return matchedBundle, matchedRoot, true
	}
	return "", "", false
}

// 🛤️resolveParentIDFromPath holds the data fields for a resolveParentIDFromPath record.
func ResolveParentIDFromPath(dirPath string) string { return (ArtifactIndex{RootDir:workspace.GetRootDir(), Bundles: LookupBundles()}).ResolveParentIDFromPath(dirPath) }

// 📁️buildFolderID holds the data fields for a buildFolderID record.
func BuildFolderID(path string, bundleID *string) string { return (ArtifactIndex{RootDir:workspace.GetRootDir(), Bundles: LookupBundles()}).BuildFolderID(path, bundleID) }

// 📄️buildFileID holds the data fields for a buildFileID record.
func BuildFileID(path string, bundleID *string) string { return (ArtifactIndex{RootDir:workspace.GetRootDir(), Bundles: LookupBundles()}).BuildFileID(path, bundleID) }

// 📑️buildSectionID holds the data fields for a buildSectionID record.
func BuildSectionID(fileID string, sectionPath []string) string {
	if len(sectionPath) == 0 || (len(sectionPath) == 1 && sectionPath[0] == "") {
		return fileID
	}
	var segments []string
	for _, s := range sectionPath {
		if s != "" {
			segments = append(segments, s)
		}
	}
	if len(segments) == 0 {
		return fileID
	}
	result := fileID
	for _, segment := range segments {
		emoji, name := ExtractEntityEmoji(segment)
		if emoji == "" {
			emoji = EmojiSection
		}
		result += EmojiText(emoji) + workspace.Flat(name)
	}
	return result
}

// 🧪️isTestFunctionName holds the data fields for a isTestFunctionName record.
func isTestFunctionName(name string) bool {

	for _, prefix := range []string{"Test", "Benchmark", "Fuzz"} {
		if strings.HasPrefix(name, prefix) && len(name) > len(prefix) && name[len(prefix)] >= 'A' && name[len(prefix)] <= 'Z' {
			return true
		}
	}

	if strings.HasPrefix(name, "test_") {
		return true
	}
	return false
}

// 📖️buildDefinitionID holds the data fields for a buildDefinitionID record.
func BuildDefinitionID(fileID string, sectionPath []string, name string, kind DefinitionKind) string {
	effectiveKind := kind
	if kind == DefinitionKindImplementation && strings.Contains(fileID, EmojiText(EmojiFileLab)) && isTestFunctionName(name) {
		effectiveKind = DefinitionKindTest
	}
	data := map[string]interface{}{"kind": string(effectiveKind)}
	return BuildSectionID(fileID, sectionPath) + definitionKindEmoji(data) + workspace.Flat(name)
}

// 🧹️filterGitIgnored holds the data fields for a filterGitIgnored record.
func FilterGitIgnored(rootDir string, files []string) []string {
	if len(files) == 0 {
		return files
	}
	relPaths := make([]string, len(files))
	for i, filePath := range files {
		relPaths[i] = workspace.NormalizePath(filePath)
	}
	ignored := workspace.GetGitIgnoredSet(rootDir, relPaths)
	patterns, _ := workspace.LoadGitignore(rootDir)
	ignore := workspace.CompileIgnoreLines(patterns...)
	filtered := make([]string, 0, len(files))
	for i, filePath := range files {
		normalized := workspace.NormalizeRepoPath(filePath)
		if normalized != "" && ignored[relPaths[i]] {
			continue
		}
		if normalized != "" && ignore.MatchesPath(normalized) {
			continue
		}
		filtered = append(filtered, filePath)
	}
	return filtered
}

// #endregion 🪪️Scope Ids

// 🗺️ArtifactIndex binds identifier construction to one immutable bundle snapshot.
type ArtifactIndex struct { RootDir string; Bundles []Bundle }

func (index ArtifactIndex) GetBundleByPath(path string) *Bundle {
	bundles := index.Bundles
	normalizedPath := workspace.NormalizePath(path)
	var bestMatch *Bundle
	var matchedLen int
	for i := range bundles {
		bundle := &bundles[i]
		root := workspace.NormalizePath(bundle.Root)
		if strings.HasPrefix(normalizedPath, root+"/") || normalizedPath == root {
			if len(root) > matchedLen {
				bestMatch = bundle
				matchedLen = len(root)
			}
		}
	}
	return bestMatch
}

func (index ArtifactIndex) ResolveParentIDFromPath(dirPath string) string {
	normalized := workspace.NormalizePath(dirPath)
	if normalized == "." || normalized == "" {
		bundle := index.GetBundleByPath(".")
		if bundle != nil {
			return bundle.GetID()
		}
		return ""
	}
	bundle := index.GetBundleByPath(normalized)
	if bundle != nil {
		bundleRoot := workspace.NormalizePath(bundle.Root)
		if normalized == bundleRoot {
			return bundle.GetID()
		}
		if strings.HasPrefix(normalized, bundleRoot+"/") {
			relPath := normalized[len(bundleRoot)+1:]
			parts := strings.Split(relPath, "/")
			parentID := bundle.GetID()
			currentPath := bundleRoot
			for _, part := range parts {
				currentPath += "/" + part
				kind := index.FolderKind(currentPath)
				if kind == FolderKindOrganization {
					parentID += EmojiText(EmojiFolderOrg) + workspace.Flat(part)
				} else {
					parentID += EmojiText(EmojiFolderRequired) + workspace.Flat(part)
				}
			}
			return parentID
		}
	}
	parts := strings.Split(normalized, "/")
	parentID := ""
	currentPath := ""
	for _, part := range parts {
		if currentPath == "" {
			currentPath = part
		} else {
			currentPath += "/" + part
		}
		kind := index.FolderKind(currentPath)
		if kind == FolderKindOrganization {
			parentID += EmojiText(EmojiFolderOrg) + workspace.Flat(part)
		} else {
			parentID += EmojiText(EmojiFolderRequired) + workspace.Flat(part)
		}
	}
	return parentID
}

func (index ArtifactIndex) BuildFolderID(path string, bundleID *string) string {
	normalized := workspace.NormalizePath(path)
	if normalized == "." || normalized == "" {
		bundle := index.GetBundleByPath(".")
		if bundle != nil {
			return bundle.GetID()
		}
		return ""
	}
	dir := filepath.Dir(normalized)
	name := filepath.Base(normalized)
	parentID := ""
	bundle := index.GetBundleByPath(normalized)
	if bundle != nil {
		bundleRoot := workspace.NormalizePath(bundle.Root)
		if normalized == bundleRoot {
			return bundle.GetID()
		}
		if dir == bundleRoot {
			parentID = bundle.GetID()
		} else if strings.HasPrefix(dir, bundleRoot+"/") {
			parentID = index.ResolveParentIDFromPath(dir)
		} else {
			parentID = index.ResolveParentIDFromPath(dir)
		}
	} else if dir != "." && dir != "" {
		parentID = index.ResolveParentIDFromPath(dir)
	} else {
		repoBundle := index.GetBundleByPath(".")
		if repoBundle != nil {
			parentID = repoBundle.GetID()
		}
	}
	kind := index.FolderKind(normalized)
	if kind == FolderKindRoot {
		if bundle != nil {
			return bundle.GetID()
		}
	}
	if kind == FolderKindOrganization {
		return parentID + EmojiText(EmojiFolderOrg) + workspace.Flat(name)
	}
	return parentID + EmojiText(EmojiFolderRequired) + workspace.Flat(name)
}

func (index ArtifactIndex) BuildFileID(path string, bundleID *string) string {
	normalized := workspace.NormalizePath(path)
	name := filepath.Base(normalized)
	kind := DeriveFileKind(name)
	dir := filepath.Dir(normalized)
	parentID := ""
	if dir != "." && dir != "" {
		bundle := index.GetBundleByPath(normalized)
		if bundle != nil && workspace.NormalizePath(bundle.Root) == workspace.NormalizePath(dir) {
			parentID = bundle.GetID()
		} else {
			parentID = index.ResolveParentIDFromPath(dir)
		}
	} else {
		bundle := index.GetBundleByPath(normalized)
		if bundle != nil {
			parentID = bundle.GetID()
		}
	}
	return GetArtifactID("file", map[string]interface{}{"path": normalized, "kind": kind, "parentId": parentID})
}
