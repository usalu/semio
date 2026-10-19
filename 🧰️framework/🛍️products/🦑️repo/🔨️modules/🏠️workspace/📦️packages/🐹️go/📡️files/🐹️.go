// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📄️files carries the file, directory and repository path helpers.

// #endregion 🧲️Header

package workspace

import (
	bufio "bufio"
	json "encoding/json"
	os "os"
	filepath "path/filepath"
	strings "strings"
)

// #region 📄️Files

// 📄️isSourceFile holds the data fields for a isSourceFile record.
func IsSourceFile(filePath string) bool {
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
	rel, err := filepath.Rel(GetRootDir(), filePath)
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

// 🔗️PathToUriPath MUST complete the operation and return consistent results.
// 🌐️PathToUriPath performs the path to uri path operation (no whitespace, reversible).
func PathToUriPath(path string) string {
	segments := strings.Split(path, "/")
	for i, s := range segments {
		segments[i] = strings.ReplaceAll(s, " ", "%20")
	}
	return strings.Join(segments, "/")
}

// 🟫️PathFromUriPath MUST complete the operation and return consistent results.
// 🟨️PathFromUriPath performs the uri path to path operation (reverse of PathToUriPath).
func PathFromUriPath(uriPath string) string {
	segments := strings.Split(uriPath, "/")
	for i, s := range segments {
		segments[i] = strings.ReplaceAll(s, "%20", " ")
	}
	return strings.Join(segments, "/")
}

// 📸️ListDirEntries MUST return a consistent snapshot of available entries.
// 🔵️ListDirEntries returns all available dir entries entries.
func ListDirEntries(dir string, dirsOnly bool) ([]string, error) {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, e := range entries {
		if strings.HasPrefix(e.Name(), ".") {
			continue
		}
		if dirsOnly && !e.IsDir() {
			continue
		}
		if !dirsOnly && e.IsDir() {
			continue
		}
		names = append(names, e.Name())
	}
	return names, nil
}

// ⏹️WalkDir MUST visit every entry and MUST stop when the callback returns an error.
// 🔬️WalkDir recursively walks the dir and invokes the callback.
func WalkDir(dir string, fn func(path string, isDir bool) error) error {
	return filepath.Walk(dir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if strings.HasPrefix(info.Name(), ".") {
			if info.IsDir() {
				return filepath.SkipDir
			}
			return nil
		}
		return fn(path, info.IsDir())
	})
}

// 🔵️ReadLines MUST return the full content from the given path.
// 🟠️ReadLines reads and returns the lines content.
func ReadLines(filePath string) ([]string, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, scanner.Err()
}

// 🔻️CountLines MUST complete the operation and return consistent results.
func CountLines(content string) int {
	if content == "" {
		return 0
	}
	return strings.Count(content, "\n") + 1
}

// 📄️CountLinesInFile MUST complete the operation and return consistent results.
func CountLinesInFile(path string) int {
	content, err := ReadTextFile(path)
	if err != nil {
		return 0
	}
	return CountLines(content)
}

func NormalizeRepoPath(path string) string {
	normalized := NormalizePath(path)
	if filepath.IsAbs(path) {
		normalized = GetRelativePath(path)
	}
	normalized = strings.TrimPrefix(normalized, "./")
	return normalized
}

// 💿️isRepoExcludedPath holds the data fields for a isRepoExcludedPath record.
func IsRepoExcludedPath(path string) bool {
	normalized := NormalizeRepoPath(path)
	if normalized == "" {
		return false
	}
	if normalized == ".🧬semio" || strings.HasPrefix(normalized, ".🧬semio/") {
		return true
	}
	if normalized == "assets/repo" || strings.HasPrefix(normalized, "assets/repo/") || strings.Contains(normalized, "/asset/repo/") {
		return true
	}
	if normalized == "node_modules" || strings.HasPrefix(normalized, "node_modules/") ||
		strings.Contains(normalized, "/node_modules/") {
		return true
	}
	if normalized == ".git" || strings.HasPrefix(normalized, ".git/") || strings.Contains(normalized, "/.git/") {
		return true
	}
	for _, segment := range []string{"/dist/", "/build/", "/target/", "/__pycache__/", "/.next/", "/coverage/"} {
		if strings.Contains(normalized, segment) {
			return true
		}
	}
	base := filepath.Base(normalized)
	if strings.HasSuffix(base, ".Designer.cs") {
		return true
	}
	if strings.Contains(normalized, "/codegen/") {
		return true
	}
	return false
}

// 🛤️normalizeHookPath holds the data fields for a normalizeHookPath record.
func NormalizeHookPath(path string) string {
	path = strings.TrimSpace(path)
	if path == "" {
		return ""
	}
	if filepath.IsAbs(path) {
		path = GetRelativePath(path)
	}
	path = NormalizePath(path)
	return strings.TrimPrefix(path, "./")
}

// #endregion 📄️Files
