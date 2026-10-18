// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🗄️store carries the contributor store seam: memory and filesystem stores, listing, search and resolution.

// #endregion 🧲️Header

package contributors

import (
	json "encoding/json"
	os "os"
	filepath "path/filepath"
	sort "sort"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🗄️ContributorStore

// 📄️StoredContributor is one directory name and document pair of a contributor store.
type StoredContributor struct {
	Directory string `json:"directory"`
	Document  string `json:"document"`
}

// 🗄️ContributorStore is where contributor documents live, one per directory.
type ContributorStore interface {
	// 📋️Documents returns every directory name and document pair, in directory name order.
	Documents() []StoredContributor
}

// 🧠️MemoryContributorStore is the in-memory store the language-agnostic tests run against.
type MemoryContributorStore struct {
	documents []StoredContributor
}

// 🌱️NewMemoryContributorStore seeds a store with directory name and document pairs.
func NewMemoryContributorStore(entries []StoredContributor) *MemoryContributorStore {
	documents := append([]StoredContributor{}, entries...)
	sort.SliceStable(documents, func(left, right int) bool { return documents[left].Directory < documents[right].Directory })
	return &MemoryContributorStore{documents: documents}
}

// 📋️Documents returns the seeded pairs in directory name order.
func (store *MemoryContributorStore) Documents() []StoredContributor {
	return append([]StoredContributor{}, store.documents...)
}

// 💽️FsContributorStore is the store over a real devs tree of a repository root.
type FsContributorStore struct {
	root string
}

// 🆕️NewFsContributorStore builds the store of the devs directory of a repository root.
func NewFsContributorStore(repoRoot string) *FsContributorStore {
	return &FsContributorStore{root: workspace.DevsDirForRoot(repoRoot)}
}

// 📋️Documents reads every contributor document of the devs directory, in directory name order.
func (store *FsContributorStore) Documents() []StoredContributor {
	entries, err := os.ReadDir(store.root)
	if err != nil {
		return nil
	}
	var found []StoredContributor
	for _, entry := range entries {
		if !entry.IsDir() {
			continue
		}
		data, err := os.ReadFile(filepath.Join(store.root, entry.Name(), "🧑️‍💻️contributor.json"))
		if err != nil {
			continue
		}
		found = append(found, StoredContributor{Directory: entry.Name(), Document: string(data)})
	}
	sort.SliceStable(found, func(left, right int) bool { return found[left].Directory < found[right].Directory })
	return found
}

// 🏠️hostContributors returns every contributor of the repository the process runs in.
func hostContributors() []model.Contributor {
	return ListContributors(NewFsContributorStore(workspace.GetRootDir()))
}

// 📋️ListContributors returns every contributor of a store, with the directory name standing in
// for a blank alias or handle. An empty store yields the single unknown contributor.
func ListContributors(store ContributorStore) []model.Contributor {
	documents := store.Documents()
	if len(documents) == 0 {
		return []model.Contributor{{Alias: "unknown", Github: "unknown", Name: "Unknown"}}
	}
	result := make([]model.Contributor, 0, len(documents))
	for _, entry := range documents {
		var contributor model.Contributor
		if err := json.Unmarshal([]byte(entry.Document), &contributor); err != nil {
			continue
		}
		if contributor.Alias == "" {
			contributor.Alias = entry.Directory
		}
		if contributor.Github == "" {
			contributor.Github = entry.Directory
		}
		result = append(result, contributor)
	}
	return result
}

// 🔎️SearchContributorsFrom returns every contributor whose alias, name or handle contains the term.
func SearchContributorsFrom(store ContributorStore, term string) []model.Contributor {
	needle := strings.ToLower(term)
	found := make([]model.Contributor, 0)
	for _, contributor := range ListContributors(store) {
		haystack := strings.ToLower(contributor.Alias + " " + contributor.Name + " " + contributor.Github)
		if needle == "" || strings.Contains(haystack, needle) {
			found = append(found, contributor)
		}
	}
	return found
}

// 🤝️ResolveAuthorToAlias returns the alias of the contributor an author line belongs to: an email
// match wins over a name match, both case-insensitive over the primary value and every recorded
// alternative, and an author nobody claims falls back to the author line itself.
func ResolveAuthorToAlias(store ContributorStore, name, email string) string {
	for _, contributor := range ListContributors(store) {
		if email != "" && containsFold(append([]string{contributor.Email}, contributor.Emails...), email) {
			return contributor.Alias
		}
		if name != "" && containsFold(append([]string{contributor.Name}, contributor.Names...), name) {
			return contributor.Alias
		}
	}
	switch {
	case name != "" && email != "":
		return name + " <" + email + ">"
	case name != "":
		return name
	case email != "":
		return email
	}
	return ""
}

// 🔎️FindContributor returns the alias an author line resolves to, unknown when it names nobody.
func FindContributor(store ContributorStore, authorLine string) string {
	parsed := ParseGitAuthor(authorLine)
	if parsed.Name == "" && parsed.Email == "" {
		return "unknown"
	}
	return ResolveAuthorToAlias(store, parsed.Name, parsed.Email)
}

// 🔠️containsFold reports whether a candidate equals a value, case-insensitively.
func containsFold(candidates []string, value string) bool {
	for _, candidate := range candidates {
		if strings.EqualFold(candidate, value) {
			return true
		}
	}
	return false
}

// #endregion 🗄️ContributorStore
