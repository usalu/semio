// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Package workspace owns repository-root discovery, the `.🧬semio` layout, `📋️config.toml` settings, and the owned glob and ignore primitives every other repo module matches paths with. This file carries the root directory state, the layout names and paths, root discovery and the root watchers.

// #endregion 🧲️Header

package workspace

import (
	"os"
 "fmt"
 "unicode/utf16"
	"path/filepath"
	"strings"
	"sync"

)

// #region 🏠️Workspace

// 🧬️SemioDirName is the workspace-local semio directory inside a monorepo root.
const SemioDirName = ".🧬semio"

// 🦑️RepoDirName is the repo product directory inside the semio directory.
const RepoDirName = "🦑️repo"

// 🎫️TicketsDirName is the ticket collection directory inside the repo meta directory.
const TicketsDirName = "🎫️tickets"

// 🎯️GoalsDirName is the goal collection directory inside the repo meta directory.
const GoalsDirName = "🎯️goals"

// 🧑️DevsDirName is the contributor collection directory inside the repo meta directory.
const DevsDirName = "🧑️‍💻️devs"

// 📁️FilesIndexName is the codebase file index inside the repo meta directory.
const FilesIndexName = "📁️files.json"

// ⚙️ConfigFileName is the repo settings file inside the repo meta directory.
const ConfigFileName = "📋️config.toml"

// 🧬️SemioDirForRoot returns the workspace-local semio root under an explicit monorepo root.
func SemioDirForRoot(repoRoot string) string {
	return filepath.Join(repoRoot, SemioDirName)
}

// 📦️RepoMetaDirForRoot returns the repo meta dir for an explicit monorepo root.
func RepoMetaDirForRoot(repoRoot string) string {
	return filepath.Join(repoRoot, SemioDirName, RepoDirName)
}

// 🛤️RepoMetaPathForRoot joins a relative path onto the repo meta dir of an explicit monorepo root.
func RepoMetaPathForRoot(repoRoot string, path string) string {
	return filepath.Join(RepoMetaDirForRoot(repoRoot), path)
}

// 🎫️TicketsDirForRoot returns the ticket collection directory of an explicit monorepo root.
func TicketsDirForRoot(repoRoot string) string {
	return RepoMetaPathForRoot(repoRoot, TicketsDirName)
}

// 🎯️GoalsDirForRoot returns the goal collection directory of an explicit monorepo root.
func GoalsDirForRoot(repoRoot string) string {
	return RepoMetaPathForRoot(repoRoot, GoalsDirName)
}

// 🧑️DevsDirForRoot returns the contributor collection directory of an explicit monorepo root.
func DevsDirForRoot(repoRoot string) string {
	return RepoMetaPathForRoot(repoRoot, DevsDirName)
}

// 📁️FilesIndexForRoot returns the codebase file index path of an explicit monorepo root.
func FilesIndexForRoot(repoRoot string) string {
	return RepoMetaPathForRoot(repoRoot, FilesIndexName)
}

// 🧭️FindRepoRoot walks upwards from startDir, returns the first monorepo root it recognises and reports the resolution to the observer.
func FindRepoRoot(startDir string) string {
	resolved := ResolveRepoRoot(startDir)
	rootStateMutex.RLock()
	observe := rootResolutionObserver
	rootStateMutex.RUnlock()
	if observe != nil {
		observe(resolved)
	}
	return resolved
}


// 🪟️normalizeRootPath preserves rooted UNC paths while removing the Windows verbatim namespace.
func normalizeRootPath(value string) string { if strings.HasPrefix(value, `\\?\UNC\`) { return `\\` + strings.TrimPrefix(value, `\\?\UNC\`) }; return strings.TrimPrefix(value, `\\?\`) }

// 🎯️SelectRepoRoot validates an explicit directory itself, discovering only when it is absent.
func SelectRepoRoot(start string, explicit *string) (string, error) {
 selected := ""
 if explicit == nil { selected = FindRepoRoot(start) } else { if *explicit == "" { return "", fmt.Errorf("repository directory required / Repository-Verzeichnis erforderlich") }; selected = *explicit; if !filepath.IsAbs(selected) { selected = strings.TrimRight(start, string(os.PathSeparator)) + string(os.PathSeparator) + selected } }
 if len(utf16.Encode([]rune(selected))) > 256 { return "", fmt.Errorf("repository path exceeds 256 units / Repository-Pfad ueberschreitet 256 Einheiten") }
 selected, err := filepath.EvalSymlinks(selected); if err != nil { return "", err }; selected, err = filepath.Abs(selected); if err != nil { return "", err }; selected = normalizeRootPath(selected); info, err := os.Stat(selected); if err != nil { return "", err }
 if !info.IsDir() || len(utf16.Encode([]rune(selected))) > 256 { return "", fmt.Errorf("bounded repository directory required / begrenztes Repository-Verzeichnis erforderlich") }; return selected, nil
}

// 🔍️ResolveRepoRoot walks upwards from startDir and returns the first monorepo root it recognises without reporting it.
func ResolveRepoRoot(startDir string) string {
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
	if physical, canonicalErr := filepath.EvalSymlinks(dir); canonicalErr == nil { dir = physical }
	if found, ok := ascend(dir, func(candidate string) bool {
		info, statErr := os.Stat(filepath.Join(candidate, ".git"))
		return statErr == nil && (info.IsDir() || info.Mode().IsRegular())
	}); ok {
		return found
	}
	return dir
}

// ⬆️ascend walks from dir to the filesystem root and returns the first directory accepted by match.
func ascend(dir string, match func(string) bool) (string, bool) {
	searchDir := dir
	for {
		if match(searchDir) {
			return searchDir, true
		}
		parent := filepath.Dir(searchDir)
		if parent == searchDir {
			return "", false
		}
		searchDir = parent
	}
}

var (
	rootStateMutex         sync.RWMutex
	rootDir                string
	rootResolutionObserver func(resolved string)
)

// 🧷️BindWorkingDirectory binds the process to the repository containing the working directory; every entry point calls it once per invocation because no root is ever bound implicitly.
func BindWorkingDirectory() error {
	cwd, err := os.Getwd()
	if err != nil {
		return err
	}
	SetRootDir(cwd)
	return nil
}

// 🔓️LookupRootDir returns the bound root and whether a root is bound, without refusing.
func LookupRootDir() (string, bool) {
	rootStateMutex.RLock()
	defer rootStateMutex.RUnlock()
	return rootDir, rootDir != ""
}

// 🏪️GetRootDir returns the explicitly bound root and refuses while none is bound, so no lookup can fall back to the repository the process happens to run in.
func GetRootDir() string {
	root, bound := LookupRootDir()
	if !bound {
		panic("workspace: the repository root is not bound; call BindWorkingDirectory or SetRootDir first")
	}
	return root
}

// 📥️SetRootDir binds the monorepo root containing dir, drops the root-derived caches and notifies the watchers.
func SetRootDir(dir string) {
	resolved := FindRepoRoot(dir)
	rootStateMutex.Lock()
	rootDir = resolved
	rootStateMutex.Unlock()
	GitignoreMutex.Lock()
	CachedGitignore = nil
	GitignoreLoaded = false
	GitignoreMutex.Unlock()
	notifyRootDirWatchers()
}

// 🔭️ObserveRootResolution installs the observer that sees every root resolution and returns the function that restores the previous observer; it exists for the hermetic test guard of the nested sandbox module.
func ObserveRootResolution(observer func(resolved string)) func() {
	rootStateMutex.Lock()
	previous := rootResolutionObserver
	rootResolutionObserver = observer
	rootStateMutex.Unlock()
	return func() {
		rootStateMutex.Lock()
		rootResolutionObserver = previous
		rootStateMutex.Unlock()
	}
}

// 🔷️GetSemioRootDir MUST return the stored value without modification.
// 🧬️GetSemioRootDir returns the workspace-local semio root under the monorepo.
func GetSemioRootDir() string {
	return filepath.Join(GetRootDir(), ".🧬semio")
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

// 📝️GetDraftsPath MUST return the stored value without modification.
// 📦️GetDraftsPath returns the drafts path of the value.
func GetDraftsPath() string {
	return GetRepoMetaPath("✍️notes")
}

// 📨️GetContributorAvatarPath MUST retrieve the requested value or return an error.
// 🔖️GetContributorAvatarPath retrieves and returns the contributor avatar path.
func GetContributorAvatarPath(alias string) string {
	return filepath.Join(GetRepoMetaDir(), "🧑️‍💻️devs", alias, "avatar.png")
}

// ❌️GetContributorAvatarRoundPath MUST retrieve the requested value or return an error.
// 🔖️GetContributorAvatarRoundPath retrieves and returns the contributor avatar round path.
func GetContributorAvatarRoundPath(alias string) string {
	return filepath.Join(GetRepoMetaDir(), "🧑️‍💻️devs", alias, "avatar-round.png")
}

// 🛤️GetContributorPath MUST retrieve the requested value or return an error.
// 🔖️GetContributorPath retrieves and returns the contributor path.
func GetContributorPath(alias string) string {
	return filepath.Join(GetRepoMetaDir(), "🧑️‍💻️devs", alias)
}

// 📨️GetRepoGoalsDir MUST retrieve the requested value or return an error.
// 📖️GetRepoGoalsDir retrieves and returns the repo goals dir.
func GetRepoGoalsDir() string {
	return filepath.Join(GetRepoMetaDir(), "🎯️goals")
}

// 👀️rootDirWatchers are the caches that must be dropped when the repository root moves.
var rootDirWatchers []func()

// 🔔️WatchRootDir registers a cache invalidation to run whenever the repository root changes.
func WatchRootDir(reset func()) { rootDirWatchers = append(rootDirWatchers, reset) }

// 📣️notifyRootDirWatchers runs every registered cache invalidation.
func notifyRootDirWatchers() {
	for _, reset := range rootDirWatchers {
		reset()
	}
}

// 🧱️CanonicalPath resolves symlinks of the deepest existing ancestor so that two spellings of one directory compare equal.
func CanonicalPath(path string) string {
	absolute, err := filepath.Abs(path)
	if err != nil {
		return filepath.Clean(path)
	}
	remainder := ""
	for existing := absolute; ; existing = filepath.Dir(existing) {
		if resolved, err := filepath.EvalSymlinks(existing); err == nil {
			return filepath.Join(resolved, remainder)
		}
		if filepath.Dir(existing) == existing {
			return absolute
		}
		remainder = filepath.Join(filepath.Base(existing), remainder)
	}
}

// #endregion 🏠️Workspace
