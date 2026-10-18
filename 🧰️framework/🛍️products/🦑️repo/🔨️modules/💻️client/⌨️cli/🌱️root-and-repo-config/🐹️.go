// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Repository root discovery and repo config.

// #endregion 🧲️Header

package client

import (
	"fmt"
	graphql "github.com/usalu/semio/repo/graphql"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"
)

var (
	rootDir               string
	executor              *graphql.Executor
	executorOnce          sync.Once
	formatterBinaryLookup = exec.LookPath
	formatterCommandRun   = runFormatterCommand
)

// 🕸️ensureExecutor lazily initializes the GraphQL executor on first use.
// This avoids the expensive initialization (LoadBundles, schema build) for
// 🕸️commands that don't need GraphQL (e.g. hook calls).
func ensureExecutor() {
	executorOnce.Do(func() {
		var e error
		executor, e = graphql.NewExecutor(rootDir)
		if e != nil {
			fmt.Fprintf(os.Stderr, "Warning: Failed to initialize GraphQL executor: %v\n", e)
			executor = nil
		}
	})
}

// ⚠️writeWarningf writes operational diagnostics without contaminating structured stdout.
func writeWarningf(format string, args ...interface{}) {
	fmt.Fprintf(os.Stderr, "Warning: "+format+"\n", args...)
}

// 💿️init holds the data fields for a init record.
func init() {
	wd, err := os.Getwd()
	if err != nil {
		rootDir = "."
	} else {
		rootDir = findRepoRoot(wd)
	}
	SetRootDir(rootDir)
}

// 🏪️GetRootDir MUST return the stored value without modification.
// 📖️GetRootDir returns the root dir of the value.
func GetRootDir() string {
	return rootDir
}

// 📥️SetRootDir MUST update the value on the receiver.
// 🗺️SetRootDir sets the root dir on the value.
func SetRootDir(dir string) {
	rootDir = findRepoRoot(dir)
	invalidateGitignore()
	InvalidateTechnologyCache()
}

// 🔷️GetSemioRootDir MUST return the stored value without modification.
// 🧬️GetSemioRootDir returns the workspace-local semio root under the monorepo.
func GetSemioRootDir() string {
	return filepath.Join(GetRootDir(), ".🧬semio")
}

// 🔷️RepoMetaDirForRoot MUST return the repo meta dir for an explicit monorepo root.
func RepoMetaDirForRoot(repoRoot string) string {
	return filepath.Join(repoRoot, ".🧬semio", "🦑️repo")
}

// 🔷️GetRepoMetaDir MUST return the stored value without modification.
// 📦️GetRepoMetaDir returns the repo meta dir of the value.
func GetRepoMetaDir() string {
	return RepoMetaDirForRoot(GetRootDir())
}

// 🛤️GetRepoMetaPath MUST return the stored value without modification.
// 📰️GetRepoMetaPath returns the repo meta path of the value.
func GetRepoMetaPath(path string) string {
	return filepath.Join(GetRepoMetaDir(), path)
}

// 📋️LoggingConfig holds hook logging switches and detail for `.🧬semio/🦑️repo/📋️config.toml` `[logging]`.
type LoggingConfig struct {
	Session    bool
	Operations bool
	Plan       bool
	Detail     string
}

// ⚙️RepoConfig holds repo-wide settings loaded from `.🧬semio/🦑️repo/📋️config.toml`.
type RepoConfig struct {
	Logging LoggingConfig
}

// 🔷️DefaultRepoConfig returns defaults when config is missing (`session` off).
func DefaultRepoConfig() RepoConfig {
	return RepoConfig{
		Logging: LoggingConfig{
			Session:    false,
			Operations: true,
			Plan:       true,
			Detail:     "standard",
		},
	}
}

func parseRepoConfigBool(value string) bool {
	switch strings.ToLower(strings.TrimSpace(value)) {
	case "true", "yes", "1", "on":
		return true
	default:
		return false
	}
}

func unquoteRepoConfigValue(value string) string {
	value = strings.TrimSpace(value)
	if len(value) >= 2 {
		if (value[0] == '"' && value[len(value)-1] == '"') || (value[0] == '\'' && value[len(value)-1] == '\'') {
			return value[1 : len(value)-1]
		}
	}
	return value
}

// 📥️LoadRepoConfig reads `.🧬semio/🦑️repo/📋️config.toml` under repoRoot; missing file yields defaults.
func LoadRepoConfig(repoRoot string) RepoConfig {
	cfg := DefaultRepoConfig()
	if strings.TrimSpace(repoRoot) == "" {
		return cfg
	}
	path := filepath.Join(RepoMetaDirForRoot(repoRoot), "📋️config.toml")
	data, err := ReadTextFile(path)
	if err != nil {
		return cfg
	}
	section := ""
	for _, line := range strings.Split(data, "\n") {
		trim := strings.TrimSpace(line)
		if trim == "" || strings.HasPrefix(trim, "#") {
			continue
		}
		if strings.HasPrefix(trim, "[") && strings.HasSuffix(trim, "]") {
			section = strings.ToLower(strings.Trim(trim, "[]"))
			continue
		}
		if !strings.Contains(trim, "=") {
			continue
		}
		parts := strings.SplitN(trim, "=", 2)
		key := strings.ToLower(strings.TrimSpace(parts[0]))
		val := unquoteRepoConfigValue(parts[1])
		switch section {
		case "logging":
			switch key {
			case "session":
				cfg.Logging.Session = parseRepoConfigBool(val)
			case "operations":
				cfg.Logging.Operations = parseRepoConfigBool(val)
			case "plan":
				cfg.Logging.Plan = parseRepoConfigBool(val)
			case "detail":
				if val != "" {
					cfg.Logging.Detail = val
				}
			}
		}
	}
	return cfg
}

func (lg LoggingConfig) includeResponse() bool {
	switch strings.ToLower(strings.TrimSpace(lg.Detail)) {
	case "minimal":
		return false
	default:
		return true
	}
}

func (lg LoggingConfig) includeNative() bool {
	return strings.ToLower(strings.TrimSpace(lg.Detail)) == "full"
}

func findRepoRoot(startDir string) string {
	if strings.TrimSpace(startDir) == "" {
		cwd, err := os.Getwd()
		if err != nil {
			return "."
		}
		startDir = cwd
	}
	dir, err := filepath.Abs(startDir)
	if err != nil {
		return startDir
	}

	searchDir := dir
	for {
		if FileExists(filepath.Join(searchDir, "repo", "cli", "main.go")) {
			return searchDir
		}
		parent := filepath.Dir(searchDir)
		if parent == searchDir {
			break
		}
		searchDir = parent
	}

	searchDir = dir
	for {
		if _, err := os.Stat(filepath.Join(searchDir, ".git")); err == nil {
			return searchDir
		}
		parent := filepath.Dir(searchDir)
		if parent == searchDir {
			break
		}
		searchDir = parent
	}

	searchDir = dir
	for {
		if _, err := os.Stat(filepath.Join(searchDir, "go.mod")); err == nil {
			return searchDir
		}
		parent := filepath.Dir(searchDir)
		if parent == searchDir {
			return dir
		}
		searchDir = parent
	}
}
