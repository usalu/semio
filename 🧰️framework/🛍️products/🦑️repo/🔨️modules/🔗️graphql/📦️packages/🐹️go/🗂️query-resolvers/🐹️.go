// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🗂️query_resolvers carries the query resolvers.

// #endregion 🧲️Header

package graphql

import (
	json "encoding/json"
	sort "sort"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🗂️QueryResolvers

/** 📥️ Decodes one coerced input object into a domain input type. */
func decodeInput(value interface{}, target interface{}) *ExecutionError {
	if value == nil {
		value = map[string]interface{}{}
	}
	encoded, err := json.Marshal(value)
	if err != nil {
		return NewExecutionError(err.Error())
	}
	if err := json.Unmarshal(encoded, target); err != nil {
		return NewExecutionError(err.Error())
	}
	return nil
}

/** ❌️ Lifts one context failure into the execution error chain. */
func lift(err error) *ExecutionError {
	if err == nil {
		return nil
	}
	return NewExecutionError(err.Error())
}

/** 🔤️ The string an argument carries, or the empty string. */
func argText(args map[string]interface{}, name string) string {
	if value, ok := args[name].(string); ok {
		return value
	}
	return ""
}

/** 🔤️ The string an argument carries, or nil when it is absent. */
func argOptionalText(args map[string]interface{}, name string) *string {
	if value, ok := args[name].(string); ok {
		return &value
	}
	return nil
}

/** 🔢️ The integer an argument carries, or nil when it is absent. */
func argInt(args map[string]interface{}, name string) *int {
	switch value := args[name].(type) {
	case int:
		return &value
	case int64:
		converted := int(value)
		return &converted
	case float64:
		converted := int(value)
		return &converted
	default:
		return nil
	}
}

/** 🧹️ The filter argument of one field, decoded. */
func (run *execution) filterOf(args map[string]interface{}) *model.FilterInput {
	raw, found := args["filter"]
	if !found || raw == nil {
		return nil
	}
	filter := &model.FilterInput{}
	if decodeInput(raw, filter) != nil {
		return nil
	}
	return filter
}

/** 📜️ Every technology, filtered and sorted by name. */
func (run *execution) technologyRows(args map[string]interface{}) interface{} {
	filter := run.filterOf(args)
	technologies := append([]*model.Technology{}, run.technologies...)
	sort.SliceStable(technologies, func(left, right int) bool { return technologies[left].Name < technologies[right].Name })
	rows := make([]interface{}, 0, len(technologies))
	for _, technology := range technologies {
		if !matchesFilterInput(technology.Name, filter) && !matchesFilterInput(technologyID(technology), filter) {
			continue
		}
		rows = append(rows, technologySource(technology, run.technologies, run.rootDir))
	}
	return rows
}

/** 🔵️ Every bundle, filtered. */
func (run *execution) bundles(args map[string]interface{}) interface{} {
	filter := run.filterOf(args)
	rows := make([]interface{}, 0)
	for _, bundle := range run.repo.GetBundles() {
		if !matchesFilterInput(bundle.Name, filter) {
			continue
		}
		rows = append(rows, bundleSource(bundle, run.technologies, run.rootDir))
	}
	return rows
}

/** 🎫️ Every ticket matching the date, status and text filters. */
func (run *execution) tickets(args map[string]interface{}) (interface{}, *ExecutionError) {
	var status *model.TicketStatus
	if wire, ok := args["status"].(string); ok {
		switch wire {
		case "open":
			value := model.TicketStatusOpen
			status = &value
		case "closed":
			value := model.TicketStatusClosed
			status = &value
		}
	}
	filter := run.filterOf(args)
	tickets, err := run.repo.GetTickets(argInt(args, "year"), argInt(args, "month"), argInt(args, "day"), status)
	if err != nil {
		return nil, lift(err)
	}
	rows := make([]interface{}, 0, len(tickets))
	for _, ticket := range tickets {
		if !matchesFilterInput(ticket.Slug, filter) {
			continue
		}
		rows = append(rows, ticketSource(ticket, run.rootDir))
	}
	return rows, nil
}

/** 🌿️ Strips one entity emoji prefix, ignoring presentation selectors. */
func stripEntityPrefix(value string, emoji string) (string, bool) {
	prefix := strings.NewReplacer("︎", "", "️", "").Replace(emoji)
	if prefix == "" {
		return "", false
	}
	rest, found := strings.CutPrefix(value, prefix)
	return rest, found
}

/** 🌿️ Routes one artifact identifier to the aggregate it names. */
func (run *execution) node(args map[string]interface{}) (interface{}, *ExecutionError) {
	raw := argText(args, "id")
	clean := strings.NewReplacer("︎", "", "️", "").Replace(raw)
	if clean == "" {
		return run.repoSource(), nil
	}
	selectorFree := strings.NewReplacer("︎", "", "️", "")
	for _, bundle := range run.repo.GetBundles() {
		if selectorFree.Replace(bundleID(bundle, run.technologies)) == clean {
			return bundleSource(bundle, run.technologies, run.rootDir), nil
		}
	}
	for _, kind := range []string{"technology-user", "technology-infrastructure", "technology-research", "technology-mono"} {
		rest, found := stripEntityPrefix(clean, entityEmoji(kind))
		if !found {
			continue
		}
		for _, technology := range run.technologies {
			if workspace.Flat(technology.Name) == rest || technology.Name == rest {
				return technologySource(technology, run.technologies, run.rootDir), nil
			}
		}
		return nil, nil
	}
	for _, kind := range []string{"folder-organization", "folder-required", "folder-root"} {
		if rest, found := stripEntityPrefix(clean, entityEmoji(kind)); found {
			return run.folderByPath(rest), nil
		}
	}
	for _, kind := range []string{"file-code", "file-lab", "file-script", "file-docs", "file-config", "file-resource", "file-template", "file-license"} {
		if rest, found := stripEntityPrefix(clean, entityEmoji(kind)); found {
			return run.fileByPath(rest), nil
		}
	}
	for _, kind := range []string{"definition-impl", "definition-interface", "definition-constant", "definition-test"} {
		rest, found := stripEntityPrefix(clean, entityEmoji(kind))
		if !found {
			continue
		}
		for _, definition := range run.repo.GetDefinitions() {
			if workspace.Flat(definition.Name) == rest || definition.Name == rest {
				return definitionSource(definition), nil
			}
		}
		return nil, nil
	}
	if rest, found := stripEntityPrefix(clean, entityEmoji("ticket")); found {
		slug := strings.SplitN(rest, "?", 2)[0]
		tickets, err := run.repo.GetTickets(nil, nil, nil, nil)
		if err != nil {
			return nil, lift(err)
		}
		for _, ticket := range tickets {
			if workspace.Flat(ticket.Slug) == slug || ticket.Slug == slug {
				return ticketSource(ticket, run.rootDir), nil
			}
		}
		return nil, nil
	}
	if rest, found := stripEntityPrefix(clean, entityEmoji("contributor")); found {
		contributors, err := run.repo.GetContributors()
		if err != nil {
			return nil, lift(err)
		}
		for _, contributor := range contributors {
			if workspace.Flat(contributor.Alias) == rest || contributor.Github == rest {
				return contributorSource(contributor), nil
			}
		}
		return nil, nil
	}
	if rest, found := stripEntityPrefix(clean, entityEmoji("policy")); found {
		return run.policyByID(rest), nil
	}
	for _, prefix := range []string{"repo:", "bundle:", "folder:", "file:", "contributor:", "policy:"} {
		rest, found := strings.CutPrefix(raw, prefix)
		if !found {
			continue
		}
		switch prefix {
		case "repo:":
			return run.repoSource(), nil
		case "bundle:":
			return run.bundleByName(rest), nil
		case "folder:":
			return run.folderByPath(rest), nil
		case "file:":
			return run.fileByPath(rest), nil
		case "contributor:":
			return run.contributorByID(rest)
		default:
			return run.policyByID(rest), nil
		}
	}
	return nil, NewExecutionError("invalid node id format: " + raw)
}

/** 💠️ The repository root object. */
func (run *execution) repoSource() interface{} {
	return tagged(map[string]interface{}{"id": "repo:compose", "name": "compose", "path": run.rootDir}, "Repo")
}

/** 🔵️ One bundle by name or identifier. */
func (run *execution) bundleByName(name string) interface{} {
	bundles := run.repo.GetBundles()
	for _, bundle := range bundles {
		if bundle.Name == name || bundleID(bundle, run.technologies) == name {
			return bundleSource(bundle, run.technologies, run.rootDir)
		}
	}
	return bundleSource(&model.Bundle{Name: name, Kind: model.BundleKindLibrary}, run.technologies, run.rootDir)
}

/** 🩷️ One folder by path. */
func (run *execution) folderByPath(path string) interface{} {
	normalized := strings.ReplaceAll(path, "\\", "/")
	for _, folder := range run.repo.GetFolders() {
		if folder.Path == normalized {
			return folderSource(folder)
		}
	}
	return nil
}

/** 📄️ One file by path. */
func (run *execution) fileByPath(path string) interface{} {
	normalized := strings.ReplaceAll(path, "\\", "/")
	for _, file := range run.repo.GetFiles() {
		if file.Path == normalized {
			return fileSource(file)
		}
	}
	return nil
}

/** 🧑️‍💻️ One contributor by github handle, falling back to a bare record. */
func (run *execution) contributorByID(id string) (interface{}, *ExecutionError) {
	contributors, err := run.repo.GetContributors()
	if err != nil {
		return nil, lift(err)
	}
	for _, contributor := range contributors {
		if contributor.Github == id {
			return contributorSource(contributor), nil
		}
	}
	return contributorSource(&model.Contributor{Alias: id, Github: id}), nil
}

/** 👮️ One policy by name or identifier, falling back to a bare record. */
func (run *execution) policyByID(id string) interface{} {
	for _, policy := range run.repo.GetPolicies() {
		if policy.Name == id || policy.ID == id || strings.EqualFold(policy.Name, id) || strings.EqualFold(policy.ID, id) {
			return policySource(policy, run.statutes)
		}
	}
	return policySource(&model.Policy{ID: "repo/policy/" + id, Name: id, Scopes: []string{}, Groups: []model.Territory{}, Statutes: []*model.StatuteMeta{}}, run.statutes)
}

// #endregion 🗂️QueryResolvers
