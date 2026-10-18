// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// The source index cache, scope claim mapping and the scope listing handler.

// #endregion 🧲️Header

package main

import (
	"net/http"
	"sort"
	"time"

	repopkg "github.com/usalu/semio/repo/go"
)

// #region 🎯️Scope

// Source code indexer that delegates to the shared repo/go parsing package. MUST support region-marker-based sections and language-specific definition patterns.

// 🔭️IndexCache holds in-memory caches of indexed scopes partitioned by file path.
type IndexCache struct {
	Sections    map[string][]Scope
	Definitions map[string][]Scope
	Files       map[string]Scope
}

// 🆕️newIndexCache creates an empty IndexCache with initialized maps.
func newIndexCache() IndexCache {
	return IndexCache{
		Sections:    map[string][]Scope{},
		Definitions: map[string][]Scope{},
		Files:       map[string]Scope{},
	}
}

// 🏗️buildScopesForFile delegates to the shared repopkg.BuildScopesForFile and converts ScopeEntry to Scope.
func buildScopesForFile(path string, content string) []Scope {
	now := time.Now().UTC()
	entries := repopkg.BuildScopesForFile(path, content)
	scopes := make([]Scope, len(entries))
	for i, e := range entries {
		scopes[i] = Scope{
			ID:          e.ID,
			Kind:        e.Kind,
			FilePath:    e.FilePath,
			SectionPath: e.SectionPath,
			Definition:  e.Definition,
			StartLine:   e.StartLine,
			EndLine:     e.EndLine,
			UpdatedAt:   now,
		}
	}
	return scopes
}

// Scope claim mapping logic that associates diff hunks with overlapping scopes. MUST detect multi-ticket conflicts.

// 🔭️mapClaims maps diff hunks to overlapping scopes and returns claimed IDs.
func mapClaims(scopes []Scope, diff DiffResult) ([]string, map[string][]Scope) {
	claimed := map[string][]Scope{}
	var claimedIDs []string
	for _, file := range diff.Files {
		if file.Path == "" {
			continue
		}
		fileScopes := filterScopesByFile(scopes, file.Path)
		for _, hunk := range file.Hunks {
			if hunk.NewRange.End == 0 {
				continue
			}
			for _, scope := range fileScopes {
				if scope.StartLine == 0 && scope.EndLine == 0 {
					continue
				}
				if rangesOverlap(hunk.NewRange, LineRange{Start: scope.StartLine, End: scope.EndLine}) {
					if scope.Kind == "definition" || scope.Kind == "section" {
						claimed[scope.ID] = append(claimed[scope.ID], scope)
					}
				}
			}
		}
	}
	sort.Strings(claimedIDs)
	return claimedIDs, claimed
}

// 🧹️filterScopesByFile returns scopes matching the given file path.
func filterScopesByFile(scopes []Scope, filePath string) []Scope {
	var filtered []Scope
	for _, scope := range scopes {
		if scope.FilePath == filePath {
			filtered = append(filtered, scope)
		}
	}
	return filtered
}

// 🧪️rangesOverlap tests whether two line ranges overlap.
func rangesOverlap(a LineRange, b LineRange) bool {
	if a.Start == 0 || b.Start == 0 {
		return false
	}
	return a.Start <= b.End && b.Start <= a.End
}

// 🔤️appendIfMissing appends a string to a slice only if it is not already present.
func appendIfMissing(list []string, value string) []string {
	for _, item := range list {
		if item == value {
			return list
		}
	}
	return append(list, value)
}

// 🔍️handleScopes returns scopes for a given file query parameter.
func (s *Server) handleScopes(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	filePath := r.URL.Query().Get("file")
	if filePath == "" {
		s.respondError(w, http.StatusBadRequest, "file query required")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	scopes, err := s.db.projectScopesByFile(ctx, filePath)
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, scopes)
}

// #endregion 🎯️Scope
