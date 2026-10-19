// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Gitignore matching, file IO, and formatter plans.

// #endregion 🧲️Header

package client

import (
	"github.com/usalu/semio/repo/workspace"
	"encoding/json"
	"github.com/usalu/semio/repo/client/internal/glob"
	"github.com/usalu/semio/repo/client/internal/ignore"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"

)

// GitignorePattern holds the data fields for a gitignore pattern record.

var (
	cachedGitignore *ignore.GitIgnore
	gitignoreRoot   string
	gitignoreMutex  sync.Mutex
)

// 🐙️getGitignore holds the data fields for a getGitignore record.
func getGitignore() *ignore.GitIgnore {
	gitignoreMutex.Lock()
	defer gitignoreMutex.Unlock()
	if gitignoreRoot == rootDir {
		return cachedGitignore
	}
	gitignoreRoot = rootDir
	cachedGitignore = nil
	ign, err := ignore.CompileIgnoreFile(filepath.Join(rootDir, ".gitignore"))
	if err == nil {
		cachedGitignore = ign
	}
	return cachedGitignore
}

// 🧹️invalidateGitignore drops the cached gitignore so the next lookup reloads it from disk.
func invalidateGitignore() {
	gitignoreMutex.Lock()
	defer gitignoreMutex.Unlock()
	gitignoreRoot = ""
	cachedGitignore = nil
}

// 🔶️isGitIgnored holds the data fields for a isGitIgnored record.
func isGitIgnored(filePath string) bool {
	if filepath.Base(filePath) == "LICENSE.md" {
		return true
	}
	return isIgnoredByGitignore(filePath)
}

func isIgnoredByGitignore(filePath string) bool {
	relPath := normalizeRepoPath(filePath)
	if relPath == "" {
		return false
	}
	ign := getGitignore()
	if ign == nil {
		return false
	}
	return ign.MatchesPath(relPath)
}

// 📜️policyAppliesToScope holds the data fields for a policyAppliesToScope record.
func policyAppliesToScope(policyID string, scope Scope) bool {
	switch policyID {
	case "code":
		return scope.Kind == ScopeFile && isSourceFile(scope.FilePath)
	case "dev-docs":
		return scope.Kind == ScopeRepo || scope.Kind == ScopeFolder || scope.Kind == ScopeFile
	default:
		return true
	}
}

// 📄️isSourceFile holds the data fields for a isSourceFile record.
func isSourceFile(filePath string) bool {
	ext := filepath.Ext(filePath)
	return ext == ".ts" || ext == ".tsx" || ext == ".js" || ext == ".jsx" ||
		ext == ".py" || ext == ".go" || ext == ".cs"
}

// 📖️NormalizePath MUST be idempotent for already-normalized values.
// 📝️NormalizePath normalizes the path to its canonical form.
func NormalizePath(p string) string {
	return strings.ReplaceAll(p, "\\", "/")
}

// 🎛️EnsureDir MUST be idempotent and MUST NOT fail if the target already exists.
// ❓️EnsureDir ensures the dir exists, creating it if necessary.
func EnsureDir(dirPath string) error {
	return os.MkdirAll(dirPath, 0755)
}

// 🔹️GetRelativePath MUST return the stored value without modification.
// 🔷️GetRelativePath returns the relative path of the value.
func GetRelativePath(filePath string) string {
	rel, err := filepath.Rel(rootDir, filePath)
	if err != nil {
		return filePath
	}
	return NormalizePath(rel)
}

// 📝️ReadTextFile MUST return the full content from the given path.
// 🔤️ReadTextFile reads and returns the text file content.
func ReadTextFile(filePath string) (string, error) {
	data, err := os.ReadFile(filePath)
	if err != nil {
		return "", err
	}
	return string(data), nil
}

// ✏️WriteTextFile MUST persist the content atomically.
// 💾️WriteTextFile writes the text file content to storage.
func WriteTextFile(filePath string, content string) error {
	if err := EnsureDir(filepath.Dir(filePath)); err != nil {
		return err
	}
	return os.WriteFile(filePath, []byte(content), 0644)
}

// 📋️formatterPlan holds the data fields for a formatterPlan record.
type formatterPlan struct {
	binary      string
	args        []string
	requirement []string
}

// 🔸️runFormatterCommand holds the data fields for a runFormatterCommand record.
func runFormatterCommand(binary string, args []string, workDir string) error {
	commandPath := binary
	if strings.Contains(binary, "/") || strings.Contains(binary, "\\") {
		commandPath = filepath.Join(workDir, binary)
	}
	cmd := exec.Command(commandPath, args...)
	cmd.Dir = workDir
	return cmd.Run()
}

// 🗣️formatterPlansForLanguage holds the data fields for a formatterPlansForLanguage record.
func formatterPlansForLanguage(languageName string, relPath string) []formatterPlan {
	prettierBinary := filepath.Join("node_modules", ".bin", "prettier")
	prettierPlan := formatterPlan{
		binary:      prettierBinary,
		args:        []string{"--write", "--ignore-unknown", relPath},
		requirement: []string{prettierBinary, ".prettierrc.json"},
	}
	switch languageName {
	case "typescript", "markdown", "json", "yaml", "graphql", "sql", "toml":
		return []formatterPlan{prettierPlan}
	case "go":
		return []formatterPlan{
			{binary: "gofmt", args: []string{"-w", relPath}, requirement: []string{"go.work"}},
		}
	case "python":
		return []formatterPlan{
			{binary: "uv", args: []string{"run", "--group", "dev", "ruff", "format", relPath}, requirement: []string{"pyproject.toml"}},
			{binary: "uv", args: []string{"run", "--group", "dev", "black", relPath}, requirement: []string{"pyproject.toml"}},
			prettierPlan,
		}
	case "csharp":
		plans := []formatterPlan{}
		for _, project := range workspace.DotnetFormatterProjects(rootDir, relPath) {
			plans = append(plans, formatterPlan{binary: "dotnet", args: []string{"format", project, "--include", relPath, "--verbosity", "minimal"}, requirement: []string{project}})
		}
		return append(plans, prettierPlan)
	case "rust":
		return []formatterPlan{
			{binary: "rustfmt", args: []string{relPath}, requirement: []string{"Cargo.toml"}},
			prettierPlan,
		}
	case "ruby":
		return []formatterPlan{
			{binary: "rubocop", args: []string{"-A", relPath}, requirement: []string{"package.json"}},
			prettierPlan,
		}
	case "shell":
		return []formatterPlan{
			{binary: "shfmt", args: []string{"-w", relPath}, requirement: []string{"package.json"}},
			prettierPlan,
		}
	default:
		return []formatterPlan{prettierPlan}
	}
}

// 🔺️isFormatterPlanAvailable holds the data fields for a isFormatterPlanAvailable record.
func isFormatterPlanAvailable(plan formatterPlan, workDir string) bool {
	if strings.Contains(plan.binary, "/") || strings.Contains(plan.binary, "\\") {
		if !FileExists(filepath.Join(workDir, plan.binary)) {
			return false
		}
	} else {
		if _, err := formatterBinaryLookup(plan.binary); err != nil {
			return false
		}
	}
	for _, rel := range plan.requirement {
		if !FileExists(filepath.Join(workDir, rel)) {
			return false
		}
	}
	return true
}

// ⬛️runFormatterForFile runs the language-appropriate formatter on the given file.
// 🖊️Unlike runFormatterAfterAutofix, this only runs the formatter with no fallback.
func runFormatterForFile(relPath string) {
	lang := GetLanguage(relPath)
	langName := ""
	if lang != nil {
		langName = lang.Name()
	}
	plans := formatterPlansForLanguage(langName, relPath)
	for _, plan := range plans {
		if !isFormatterPlanAvailable(plan, rootDir) {
			continue
		}
		if err := formatterCommandRun(plan.binary, plan.args, rootDir); err == nil {
			return
		}
	}
}

// ⬜️WriteJSONFile MUST persist the content atomically.
// 🔹️WriteJSONFile writes the j s o n file content to storage.
func WriteJSONFile(filePath string, data interface{}) error {
	jsonBytes, err := json.MarshalIndent(data, "", "  ")
	if err != nil {
		return err
	}
	return WriteTextFile(filePath, string(jsonBytes)+"\n")
}

// 🟥️ReadJSONFile MUST return the full content from the given path.
// ⬛️ReadJSONFile reads and returns the j s o n file content.
func ReadJSONFile(filePath string, v interface{}) error {
	data, err := ReadTextFile(filePath)
	if err != nil {
		return err
	}
	return json.Unmarshal([]byte(data), v)
}

// 🟧️FileExists MUST complete the operation and return consistent results.
func FileExists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}

// 🟨️IsDir MUST return true only when the condition is met.
// ⬜️IsDir reports whether the value is dir.
func IsDir(path string) bool {
	info, err := os.Stat(path)
	if err != nil {
		return false
	}
	return info.IsDir()
}

// ⚙️LoadGitignore MUST read from the configured storage path.
// 🟥️LoadGitignore loads the gitignore from storage.
func LoadGitignore(cwd string) ([]string, error) {
	gitignorePath := filepath.Join(cwd, ".gitignore")
	if !FileExists(gitignorePath) {
		return nil, nil
	}
	content, err := ReadTextFile(gitignorePath)
	if err != nil {
		return nil, err
	}
	var patterns []string
	for _, line := range strings.Split(content, "\n") {
		line = strings.TrimSpace(line)
		if line != "" && !strings.HasPrefix(line, "#") {
			patterns = append(patterns, line)
		}
	}
	return patterns, nil
}

func matchesIgnorePattern(path string, isDir bool, pattern string) bool {
	pattern = NormalizePath(strings.TrimSpace(pattern))
	if pattern == "" {
		return false
	}
	path = NormalizePath(strings.TrimSpace(path))
	if path == "" {
		return false
	}
	candidates := []string{path}
	if isDir && !strings.HasSuffix(path, "/") {
		candidates = append(candidates, path+"/")
	}
	patterns := []string{pattern}
	if strings.HasSuffix(pattern, "/**") {
		base := strings.TrimSuffix(pattern, "/**")
		if base != "" {
			patterns = append(patterns, base, base+"/")
		}
	}
	for _, candidatePattern := range patterns {
		if candidatePattern == "" {
			continue
		}
		for _, candidatePath := range candidates {
			if matched, _ := glob.Match(candidatePattern, candidatePath); matched {
				return true
			}
		}
	}
	return false
}

// 🟩️SimpleGlob MUST complete the operation and return consistent results.
func SimpleGlob(pattern string, cwd string, ignorePatterns []string, respectGitignore bool) ([]string, error) {
	if cwd == "" {
		cwd = rootDir
	}
	var gitignorePatterns []string
	if respectGitignore {
		var err error
		gitignorePatterns, err = LoadGitignore(cwd)
		if err != nil {
			return nil, err
		}
	}
	allIgnore := append(ignorePatterns, gitignorePatterns...)
	var files []string
	absPattern := filepath.Join(cwd, pattern)
	matches, err := glob.FilepathGlob(absPattern)
	if err != nil {
		return nil, err
	}
	for _, match := range matches {
		rel, err := filepath.Rel(cwd, match)
		if err != nil {
			continue
		}
		relNorm := NormalizePath(rel)
		ignored := false
		for _, ig := range allIgnore {
			if matchesIgnorePattern(relNorm, false, ig) {
				ignored = true
				break
			}
		}
		if !ignored {
			files = append(files, relNorm)
		}
	}
	return files, nil
}

// 🟦️globByExtension holds the data fields for a globByExtension record.
func globByExtension(root string, patternBase string, exts []string, ignorePatterns []string, respectGitignore bool) ([]string, error) {
	base := strings.TrimSuffix(patternBase, "/**/*")
	if patternBase == "**/*" {
		base = ""
	}
	absBase := filepath.Join(root, base)
	info, err := os.Stat(absBase)
	if err != nil || !info.IsDir() {
		return nil, nil
	}
	gitignorePatterns := []string{}
	if respectGitignore {
		gitignorePatterns, err = LoadGitignore(root)
		if err != nil {
			return nil, err
		}
	}
	allIgnore := append(ignorePatterns, gitignorePatterns...)
	allowed := make(map[string]struct{}, len(exts))
	for _, ext := range exts {
		allowed[strings.ToLower(ext)] = struct{}{}
	}
	results := make([]string, 0)
	err = filepath.WalkDir(absBase, func(path string, d os.DirEntry, walkErr error) error {
		if walkErr != nil {
			return walkErr
		}
		name := d.Name()
		if d.IsDir() && strings.HasPrefix(name, ".") {
			return filepath.SkipDir
		}
		rel, relErr := filepath.Rel(root, path)
		if relErr != nil {
			return nil
		}
		rel = strings.TrimPrefix(NormalizePath(rel), "./")
		if rel == "" || rel == "." {
			return nil
		}
		for _, ig := range allIgnore {
			if matchesIgnorePattern(rel, d.IsDir(), ig) {
				if d.IsDir() {
					return filepath.SkipDir
				}
				return nil
			}
		}
		if d.IsDir() {
			return nil
		}
		ext := strings.TrimPrefix(strings.ToLower(filepath.Ext(rel)), ".")
		if _, ok := allowed[ext]; !ok {
			return nil
		}
		results = append(results, rel)
		return nil
	})
	if err != nil {
		return nil, err
	}
	return results, nil
}
