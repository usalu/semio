// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Repo context drafts, policies, statutes, breach cache, and analyze.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// ⬛️GetDrafts MUST retrieve the requested value or return an error.
// 🟠️GetDrafts retrieves and returns the drafts.
func (c *repoContext) GetDrafts() ([]*Draft, error) {
	return ListDrafts()
}

// ⬜️DraftCreate MUST return a non-nil error when the operation fails.
// 🟡️DraftCreate performs the draft create operation on the repo context.
func (c *repoContext) DraftCreate(input DraftCreateInput) (*Draft, error) {
	return CreateDraft(input.Title, input.Files)
}

// 🟥️DraftDelete MUST return a non-nil error when the operation fails.
// 🟢️DraftDelete performs the draft delete operation on the repo context.
func (c *repoContext) DraftDelete(id string) (bool, error) {
	return true, DeleteDraft(id)
}

// 🟧️GetPolicies MUST retrieve the requested value or return an error.
// 🟣️GetPolicies retrieves and returns the policies.
func (c *repoContext) GetPolicies() []*Policy {
	policies := GetRegisteredPolicies()
	result := make([]*Policy, len(policies))
	for i := range policies {
		var descPtr *string
		if policies[i].Description != "" {
			d := policies[i].Description
			descPtr = &d
		}
		var statutes []*StatuteMeta
		for _, kind := range policies[i].AllKinds() {
			meta := kind.Info()
			meta.PolicyID = policies[i].ID
			normalizeStatuteMeta(&meta)
			statutes = append(statutes, &meta)
		}
		result[i] = &Policy{
			ID:          policies[i].ID,
			Name:        policies[i].Name,
			Description: descPtr,
			Scopes:      policies[i].Scopes,
			Groups:      policies[i].Groups,
			Statutes:    statutes,
		}
	}
	return result
}

// 🟨️GetStatutes MUST retrieve the requested value or return an error.
// 🟤️GetStatutes retrieves and returns the statutes.
func (c *repoContext) GetStatutes() []*StatuteMeta {
	var result []*StatuteMeta
	for _, meta := range statuteInfoTable {
		m := meta
		normalizeStatuteMeta(&m)
		result = append(result, &m)
	}
	return result
}

type breachCacheEnvelope struct {
	EntityID string   `json:"entityId"`
	Script   string   `json:"script"`
	Breachs  []Breach `json:"breachs"`
}

func loadBreachsFromCache(repoRoot string) ([]Breach, error) {
	dir := filepath.Join(RepoMetaDirForRoot(repoRoot), "⚡️cache", "breaches")
	entries, err := os.ReadDir(dir)
	if err != nil {
		if os.IsNotExist(err) {
			return nil, nil
		}
		return nil, err
	}
	var all []Breach
	for _, e := range entries {
		if e.IsDir() || !strings.HasSuffix(e.Name(), ".json") {
			continue
		}
		data, err := os.ReadFile(filepath.Join(dir, e.Name()))
		if err != nil {
			return nil, err
		}
		var env breachCacheEnvelope
		if err := json.Unmarshal(data, &env); err != nil {
			return nil, err
		}
		all = append(all, env.Breachs...)
	}
	return all, nil
}

func breachMatchesScope(b *Breach, s Scope) bool {
	bp := NormalizePath(b.Scope)
	switch s.Kind {
	case ScopeRepo:
		return true
	case ScopeFile:
		fp := NormalizePath(s.FilePath)
		return bp == fp || strings.HasPrefix(bp, fp+"#") || strings.HasPrefix(bp, fp+"::")
	case ScopeFolder:
		fp := strings.TrimSuffix(NormalizePath(s.FilePath), "/")
		if fp == "" {
			return true
		}
		return bp == fp || strings.HasPrefix(bp, fp+"/")
	case ScopeTechnology:
		tn := strings.TrimSuffix(NormalizePath(s.TechnologyName), "/")
		if tn == "" {
			return true
		}
		return bp == tn || strings.HasPrefix(bp, tn+"/")
	case ScopeSection, ScopeDefinition:
		fp := NormalizePath(s.FilePath)
		return strings.HasPrefix(bp, fp)
	default:
		return true
	}
}

// 🔬️Analyze MUST return a non-nil error when the operation fails.
// ⚪️Analyze performs the analyze operation on the repo context.
func (c *repoContext) Analyze(scope *string) (*AnalyzeResult, error) {
	scopeStr := "compose"
	if scope != nil {
		scopeStr = *scope
	}
	s := ParseScope(scopeStr)
	all, err := loadBreachsFromCache(rootDir)
	if err != nil {
		return nil, err
	}
	var filtered []Breach
	for i := range all {
		if breachMatchesScope(&all[i], s) {
			filtered = append(filtered, all[i])
		}
	}
	pc := &PriorityCount{}
	autofixN := 0
	result := make([]*Breach, len(filtered))
	for i := range filtered {
		if filtered[i].ID == "" {
			fmt.Printf("[TRACE] Analyze found breach with empty id: %+v\n", filtered[i])
		}
		result[i] = &filtered[i]
		switch filtered[i].Priority() {
		case BreachPriorityHigh:
			pc.High++
		case BreachPriorityMedium:
			pc.Medium++
		default:
			pc.Low++
		}
		if filtered[i].Autofixable() {
			autofixN++
		}
	}
	return &AnalyzeResult{
		Breachs: result,
		Metrics: &AnalyzeMetrics{
			Total:       len(filtered),
			ByPriority:  pc,
			Autofixable: autofixN,
		},
	}, nil
}

// 🏷️inferDefinitionKindFromLine holds the data fields for a inferDefinitionKindFromLine record.
func inferDefinitionKindFromLine(line string) DefinitionKind {
	lower := strings.ToLower(strings.TrimSpace(line))
	switch {
	case strings.HasPrefix(lower, "type "),
		strings.HasPrefix(lower, "interface "),
		strings.HasPrefix(lower, "trait "),
		strings.HasPrefix(lower, "protocol "),
		strings.HasPrefix(lower, "input "),
		strings.HasPrefix(lower, "union "),
		strings.HasPrefix(lower, "scalar "),
		strings.HasPrefix(lower, "delegate "),
		strings.HasPrefix(lower, "record "):
		return DefinitionKindInterface
	case strings.HasPrefix(lower, "const "),
		strings.HasPrefix(lower, "export const "),
		strings.HasPrefix(lower, "enum "),
		strings.HasPrefix(lower, "var "),
		strings.HasPrefix(lower, "let "),
		strings.HasPrefix(lower, "static "):
		return DefinitionKindConstant
	default:
		return DefinitionKindImplementation
	}
}
