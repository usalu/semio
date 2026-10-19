// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🛡️policy carries the policy, statute and analysis queries of the repository context.

// #endregion 🧲️Header

package graphql

import (
	fmt "fmt"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	statutespkg "github.com/usalu/semio/repo/statutes"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🛡️Policy

// 🟧️GetPolicies MUST retrieve the requested value or return an error.
// 🟣️GetPolicies retrieves and returns the policies.
func (c *RepoContext) GetPolicies() []*model.Policy {
	policies := statutespkg.GetRegisteredPolicies()
	result := make([]*model.Policy, len(policies))
	for i := range policies {
		var descPtr *string
		if policies[i].Description != "" {
			d := policies[i].Description
			descPtr = &d
		}
		var statutes []*model.StatuteMeta
		for _, kind := range policies[i].AllKinds() {
			meta := kind.Info()
			meta.PolicyID = policies[i].ID
			model.NormalizeStatuteMeta(&meta)
			statutes = append(statutes, &meta)
		}
		result[i] = &model.Policy{
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
// ⛳️GetInteractions aggregates every interaction recorded on a ticket or a goal.
func (c *RepoContext) GetInteractions() ([]model.InteractionResource, error) {
	return ticketspkg.ListInteractions()
}

func (c *RepoContext) GetStatutes() []*model.StatuteMeta {
	var result []*model.StatuteMeta
	for _, meta := range model.StatuteInfoTable {
		m := meta
		model.NormalizeStatuteMeta(&m)
		result = append(result, &m)
	}
	return result
}

// 🔬️Analyze MUST return a non-nil error when the operation fails.
// ⚪️Analyze performs the analyze operation on the repo context.
func (c *RepoContext) Analyze(scope *string) (*model.AnalyzeResult, error) {
	scopeStr := "compose"
	if scope != nil {
		scopeStr = *scope
	}
	s := workspace.ParseScope(scopeStr)
	all, err := statutespkg.LoadBreachsFromCache(workspace.GetRootDir())
	if err != nil {
		return nil, err
	}
	var filtered []model.Breach
	for i := range all {
		if statutespkg.BreachMatchesScope(&all[i], s) {
			filtered = append(filtered, all[i])
		}
	}
	pc := &model.PriorityCount{}
	autofixN := 0
	result := make([]*model.Breach, len(filtered))
	for i := range filtered {
		if filtered[i].ID == "" {
			fmt.Printf("[TRACE] Analyze found breach with empty id: %+v\n", filtered[i])
		}
		result[i] = &filtered[i]
		switch filtered[i].Priority() {
		case model.BreachPriorityHigh:
			pc.High++
		case model.BreachPriorityMedium:
			pc.Medium++
		default:
			pc.Low++
		}
		if filtered[i].Autofixable() {
			autofixN++
		}
	}
	return &model.AnalyzeResult{
		Breachs: result,
		Metrics: &model.AnalyzeMetrics{
			Total:       len(filtered),
			ByPriority:  pc,
			Autofixable: autofixN,
		},
	}, nil
}

// 🏷️inferDefinitionKindFromLine holds the data fields for a inferDefinitionKindFromLine record.
func inferDefinitionKindFromLine(line string) model.DefinitionKind {
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
		return model.DefinitionKindInterface
	case strings.HasPrefix(lower, "const "),
		strings.HasPrefix(lower, "export const "),
		strings.HasPrefix(lower, "enum "),
		strings.HasPrefix(lower, "var "),
		strings.HasPrefix(lower, "let "),
		strings.HasPrefix(lower, "static "):
		return model.DefinitionKindConstant
	default:
		return model.DefinitionKindImplementation
	}
}

// #endregion 🛡️Policy
