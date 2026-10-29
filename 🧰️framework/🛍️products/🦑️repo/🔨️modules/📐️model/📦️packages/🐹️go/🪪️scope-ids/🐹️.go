// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🪪️scope_ids carries the scope filters and the codebase identifier builders.

// #endregion 🧲️Header

package model

import (
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"
	sync "sync"

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
func GetBundleByPath(path string) *Bundle {
	return NewArtifactIndex(workspace.GetRootDir(), LookupBundles()).GetBundleByPath(path)
}

// 🔷️findBundleInfo holds the data fields for a findBundleInfo record.
func findBundleInfo(path string) (name, root string, ok bool) {
	bundles := LookupBundles()
	normalizedPath := workspace.NormalizePath(path)
	var matchedBundle string
	var matchedRoot string
	var matchedLen int
	for _, bundle := range bundles {
		root := workspace.NormalizePath(bundle.Root)
		if ArtifactBoundaryOwnsPath(root, normalizedPath) {
			length := len(root)
			if root == "." {
				length = 0
			}
			if length > matchedLen || matchedBundle == "" {
				matchedBundle = bundle.Name
				matchedRoot = root
				matchedLen = length
			}
		}
	}
	if matchedLen > 0 || matchedBundle != "" {
		return matchedBundle, matchedRoot, true
	}
	return "", "", false
}

// 🛤️resolveParentIDFromPath holds the data fields for a resolveParentIDFromPath record.
func ResolveParentIDFromPath(dirPath string) string {
	return NewArtifactIndex(workspace.GetRootDir(), LookupBundles()).ResolveParentIDFromPath(dirPath)
}

// 📁️buildFolderID holds the data fields for a buildFolderID record.
func BuildFolderID(path string, bundleID *string) string {
	return NewArtifactIndex(workspace.GetRootDir(), LookupBundles()).BuildFolderID(path, bundleID)
}

// 📄️buildFileID holds the data fields for a buildFileID record.
func BuildFileID(path string, bundleID *string) string {
	return NewArtifactIndex(workspace.GetRootDir(), LookupBundles()).BuildFileID(path, bundleID)
}

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
// 🧭️ArtifactBoundary binds one repository directory to its domain identifier.
type ArtifactBoundary struct{ Root, ID string }

// 🗂️ArtifactLayout owns one invocation's directory observations and ancestor identities.
type ArtifactLayout struct {
	rootDir        string
	coordinateRoot string
	boundaries     map[string]ArtifactBoundary
	folderKinds    map[string]FolderKind
	parentIDs      map[string]string
	readDirectory  func(string) ([]os.DirEntry, error)
	mutex          sync.Mutex
}

// 🪪️ArtifactIndex binds a domain bundle catalog to an invocation-owned layout.
type ArtifactIndex struct {
	*ArtifactLayout
	bundles     []Bundle
	bundleRoots map[string]*Bundle
}

// 🌳️NewArtifactLayout snapshots boundary facts without retaining a global root cache.
func NewArtifactLayout(rootDir string, boundaries []ArtifactBoundary) *ArtifactLayout {
	anchor, _ := filepath.Abs(rootDir)
	layout := &ArtifactLayout{rootDir: rootDir, coordinateRoot: anchor, boundaries: make(map[string]ArtifactBoundary), folderKinds: make(map[string]FolderKind), parentIDs: make(map[string]string), readDirectory: readArtifactDirectory}
	for _, boundary := range boundaries {
		root, valid := layout.namespacePath(boundary.Root)
		if !valid {
			continue
		}
		boundary.Root = root
		if _, exists := layout.boundaries[root]; !exists {
			layout.boundaries[root] = boundary
		}
	}
	return layout
}

// 🏡️Root returns the repository anchor of this invocation's layout.
func (index *ArtifactLayout) Root() string { return index.rootDir }

// 🧷️BoundaryIDByPath selects the nearest owned domain boundary.
func (index *ArtifactLayout) BoundaryIDByPath(path string) (string, bool) {
	boundary, found := index.boundaryByPath(path)
	return boundary.ID, found
}
func (index *ArtifactLayout) boundaryByPath(path string) (ArtifactBoundary, bool) {
	normalized, valid := index.namespacePath(path)
	if !valid {
		return ArtifactBoundary{}, false
	}
	current := normalized
	for {
		if boundary, found := index.boundaries[current]; found && ArtifactBoundaryOwnsPath(current, normalized) {
			return boundary, true
		}
		parent := filepath.Dir(current)
		if parent == current || current == "." || current == "" {
			break
		}
		current = parent
	}
	return ArtifactBoundary{}, false
}

// 🪆️NewArtifactIndex snapshots a bundle catalog and its corresponding layout boundaries.
func NewArtifactIndex(rootDir string, bundles []Bundle) *ArtifactIndex {
	owned := append([]Bundle(nil), bundles...)
	boundaries := make([]ArtifactBoundary, len(owned))
	for index, bundle := range owned {
		boundaries[index] = ArtifactBoundary{Root: bundle.Root, ID: bundle.GetID()}
	}
	layout := NewArtifactLayout(rootDir, boundaries)
	roots := make(map[string]*Bundle)
	for index := range owned {
		root, valid := layout.namespacePath(owned[index].Root)
		if !valid {
			continue
		}
		if _, exists := roots[root]; !exists {
			roots[root] = &owned[index]
		}
	}
	return &ArtifactIndex{ArtifactLayout: layout, bundles: owned, bundleRoots: roots}
}

func (index *ArtifactIndex) GetBundleByPath(path string) *Bundle {
	boundary, found := index.boundaryByPath(path)
	if !found {
		return nil
	}
	return index.bundleRoots[workspace.NormalizePath(boundary.Root)]
}

// 🏠️ArtifactBoundaryOwnsPath treats the repository directory as the fallback boundary.
func ArtifactBoundaryOwnsPath(root, path string) bool {
	root = workspace.NormalizePath(root)
	path = workspace.NormalizePath(path)
	return path == root || strings.HasPrefix(path, root+"/") || root == "." && !filepath.IsAbs(path) && path != ".." && !strings.HasPrefix(path, "../")
}

func (index *ArtifactLayout) ResolveParentIDFromPath(dirPath string) string {
	index.mutex.Lock()
	defer index.mutex.Unlock()
	normalized, valid := index.namespacePath(dirPath)
	if !valid {
		return ""
	}
	if cached, ok := index.parentIDs[normalized]; ok {
		return cached
	}
	if normalized == "." || normalized == "" {
		id := ""
		if boundary, found := index.boundaryByPath("."); found {
			id = boundary.ID
		}
		index.parentIDs[normalized] = id
		return id
	}
	parentID, currentPath := "", ""
	parts := strings.Split(normalized, "/")
	if boundary, found := index.boundaryByPath(normalized); found {
		root := workspace.NormalizePath(boundary.Root)
		if normalized == root {
			index.parentIDs[normalized] = boundary.ID
			return boundary.ID
		}
		if root == "." {
			parentID = boundary.ID
		} else if strings.HasPrefix(normalized, root+"/") {
			parentID = boundary.ID
			currentPath = root
			parts = strings.Split(normalized[len(root)+1:], "/")
		}
	}
	for _, part := range parts {
		if currentPath == "" {
			currentPath = part
		} else {
			currentPath += "/" + part
		}
		if cached, ok := index.parentIDs[currentPath]; ok {
			parentID = cached
			continue
		}
		emoji := EmojiFolderRequired
		if index.folderKind(currentPath) == FolderKindOrganization {
			emoji = EmojiFolderOrg
		}
		parentID += EmojiText(emoji) + workspace.Flat(part)
		index.parentIDs[currentPath] = parentID
	}
	index.parentIDs[normalized] = parentID
	return parentID
}

func (index *ArtifactLayout) BuildFolderID(path string, _ *string) string {
	return index.ResolveParentIDFromPath(path)
}

func (index *ArtifactLayout) BuildFileID(path string, _ *string) string {
	normalized, valid := index.namespacePath(path)
	if !valid {
		return ""
	}
	name := filepath.Base(normalized)
	dir := filepath.Dir(normalized)
	parentID := ""
	if dir != "." && dir != "" {
		parentID = index.ResolveParentIDFromPath(dir)
	} else if boundary, found := index.boundaryByPath(normalized); found {
		parentID = boundary.ID
	}
	return GetArtifactID("file", map[string]interface{}{"path": normalized, "kind": DeriveFileKind(name), "parentId": parentID})
}

func (index *ArtifactLayout) namespacePath(path string) (string, bool) {
	normalized := workspace.NormalizePath(path)
	if filepath.IsAbs(filepath.FromSlash(normalized)) {
		relative, err := filepath.Rel(index.coordinateRoot, filepath.FromSlash(normalized))
		if err != nil {
			return "", false
		}
		normalized = filepath.ToSlash(relative)
	}
	normalized = filepath.ToSlash(filepath.Clean(filepath.FromSlash(normalized)))
	if normalized == ".." || strings.HasPrefix(normalized, "../") {
		return "", false
	}
	return normalized, true
}
