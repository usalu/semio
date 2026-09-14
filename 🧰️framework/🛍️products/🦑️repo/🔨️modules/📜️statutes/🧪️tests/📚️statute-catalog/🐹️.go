// 🐹️ Go side of the statute catalog case.
package adapter

import (
	"fmt"
	"sort"
	"strconv"
	"strings"

	model "github.com/usalu/semio/repo/model"
	statutes "github.com/usalu/semio/repo/statutes"
	host "semio.tech/repo/test"
)

// region 🔖️Rendering

// policyOwners is the policy that first claims each statute through its territory tree.
func policyOwners() map[string]string {
	owner := map[string]string{}
	for _, policy := range statutes.GetPolicies() {
		for _, kind := range policy.AllKinds() {
			if _, seen := owner[string(kind)]; !seen {
				owner[string(kind)] = policy.ID
			}
		}
	}
	return owner
}

// renderTerritory renders one territory and its nested territories as a flat, depth-first path list.
func renderTerritory(prefix string, territory model.Territory, out *[]string) {
	path := territory.Name
	if prefix != "" {
		path = prefix + "/" + territory.Name
	}
	kinds := make([]string, 0, len(territory.Kinds))
	for _, kind := range territory.Kinds {
		kinds = append(kinds, string(kind))
	}
	*out = append(*out, fmt.Sprintf("%s|%s|%s", path, strings.Join(territory.Scopes, ";"), strings.Join(kinds, ",")))
	for _, child := range territory.Groups {
		renderTerritory(path, child, out)
	}
}

// endregion 🔖️Rendering

// region 🔖️Scenarios

func theCatalogIsOneTable(_ *host.Context) (host.Outcome, error) {
	owner := policyOwners()
	kinds := make([]string, 0, len(model.StatuteInfoTable))
	for kind := range model.StatuteInfoTable {
		kinds = append(kinds, string(kind))
	}
	sort.Strings(kinds)
	statuteRows := make([]string, 0, len(kinds))
	for _, kind := range kinds {
		meta := model.Statute(kind).Info()
		statuteRows = append(statuteRows, fmt.Sprintf("%s|%s|%s|%t|%s|%s", kind, owner[kind], meta.Priority, meta.Autofixable, meta.Reason, meta.Solution))
	}
	policyRows := []string{}
	for _, policy := range statutes.GetPolicies() {
		policyRows = append(policyRows, fmt.Sprintf("%s|%s|%s|%s", policy.ID, policy.Name, policy.Description, strings.Join(policy.Scopes, ";")))
		for _, group := range policy.Groups {
			renderTerritory(policy.ID, group, &policyRows)
		}
	}
	return host.Outcome{Projection: map[string]any{
		"schemaVersion": "1",
		"statutes":      statuteRows,
		"policies":      policyRows,
	}}, nil
}

func theCatalogIsConsistent(_ *host.Context) (host.Outcome, error) {
	declared := map[string]bool{}
	for kind := range model.StatuteInfoTable {
		declared[string(kind)] = true
	}
	unknown := []string{}
	claimedTwice := []string{}
	owner := map[string]string{}
	for _, policy := range statutes.GetPolicies() {
		for _, statute := range policy.AllKinds() {
			kind := string(statute)
			if !declared[kind] {
				unknown = append(unknown, policy.ID+":"+kind)
			}
			if previous, seen := owner[kind]; seen && previous != policy.ID {
				claimedTwice = append(claimedTwice, fmt.Sprintf("%s:%s+%s", kind, previous, policy.ID))
			} else if !seen {
				owner[kind] = policy.ID
			}
		}
	}
	missingMetadata := []string{}
	unclaimed := []string{}
	for kind := range declared {
		if model.Statute(kind).Info().Reason == "Unknown breach" {
			missingMetadata = append(missingMetadata, kind)
		}
		if _, claimed := owner[kind]; !claimed {
			unclaimed = append(unclaimed, kind)
		}
	}
	sort.Strings(unknown)
	sort.Strings(claimedTwice)
	sort.Strings(missingMetadata)
	sort.Strings(unclaimed)
	return host.Outcome{Projection: map[string]any{
		"declared":        strconv.Itoa(len(declared)),
		"unknown":         unknown,
		"claimedTwice":    claimedTwice,
		"missingMetadata": missingMetadata,
		"unclaimed":       unclaimed,
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("the-catalog-is-one-table", theCatalogIsOneTable).
		Subject("the-catalog-is-consistent", theCatalogIsConsistent)
}

// endregion 🔖️Registration
