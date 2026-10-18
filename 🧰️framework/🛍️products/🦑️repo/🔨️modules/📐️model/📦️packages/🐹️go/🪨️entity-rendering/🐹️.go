// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🪨️entity_rendering carries the entity property collection and the human and markdown renderings.

// #endregion 🧲️Header

package model

import (
	fmt "fmt"
	strings "strings"

	identity "github.com/usalu/semio/repo/identity"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🪨️Entity Rendering

// #region 🪨️Entity Rendering
// 🖊️Entity rendering functions for formatted output generation.
func extractCreatedStr(data map[string]interface{}) string {
	createdStr := ""
	if dates, ok := data["dates"].(map[string]interface{}); ok {
		createdStr, _ = dates["created"].(string)
		if createdStr == "" {
			createdStr, _ = dates["started"].(string)
		}
	}
	if createdStr == "" {
		if dates, ok := data["date"].(map[string]interface{}); ok {
			createdStr, _ = dates["created"].(string)
			if createdStr == "" {
				createdStr, _ = dates["started"].(string)
			}
		}
	}
	if createdStr == "" {
		createdStr, _ = data["createdAt"].(string)
	}
	if createdStr == "" {
		if started, ok := data["started"].(string); ok {
			createdStr = started
		}
	}
	return createdStr
}

// 💿️extractFinishedStr holds the data fields for a extractFinishedStr record.
func extractFinishedStr(data map[string]interface{}) string {
	finishedStr := ""
	if dates, ok := data["dates"].(map[string]interface{}); ok {
		finishedStr, _ = dates["finished"].(string)
	}
	if finishedStr == "" {
		if dates, ok := data["date"].(map[string]interface{}); ok {
			finishedStr, _ = dates["finished"].(string)
		}
	}
	if finishedStr == "" {
		finishedStr, _ = data["finishedAt"].(string)
	}
	if finishedStr == "" {
		if finished, ok := data["finished"].(string); ok {
			finishedStr = finished
		}
	}
	return finishedStr
}

func sanitizeProp(v string) string {
	v = strings.ReplaceAll(v, "\r\n", " ")
	v = strings.ReplaceAll(v, "\n", " ")
	v = strings.ReplaceAll(v, "\r", " ")
	v = strings.ReplaceAll(v, "`", "'")
	for strings.Contains(v, "  ") {
		v = strings.ReplaceAll(v, "  ", " ")
	}
	return strings.TrimSpace(v)
}

// 🔷️sanitizeSingleLine holds the data fields for a sanitizeSingleLine record.
func sanitizeSingleLine(v string) string {
	v = strings.ReplaceAll(v, "\r\n", " ")
	v = strings.ReplaceAll(v, "\n", " ")
	v = strings.ReplaceAll(v, "\r", " ")
	return v
}

// 🔶️collectEntityProps holds the data fields for a collectEntityProps record.
func CollectEntityProps(kind string, data map[string]interface{}, truncateDesc bool) []string {
	var props []string
	appendNonEmpty := func(vals ...string) {
		for _, v := range vals {
			if v != "" {
				v = sanitizeProp(v)
				if v != "" {
					props = append(props, v)
				}
			}
		}
	}
	switch kind {
	case "goal":
		title, _ := data["title"].(string)
		status, _ := data["status"].(string)
		createdStr := extractCreatedStr(data)
		createdDisplay := ""
		if createdStr != "" {
			if tParsed, err := workspace.ParseFlexibleTime(createdStr); err == nil {
				createdDisplay = "created " + identity.Time(tParsed)
			} else {
				createdDisplay = "created " + createdStr
			}
		}
		dueDate, _ := data["dueDate"].(string)
		if dueDate == "" {
			if dates, ok := data["dates"].(map[string]interface{}); ok {
				dueDate, _ = dates["due"].(string)
			}
		}
		dueDisplay := ""
		if dueDate != "" {
			if tParsed, err := workspace.ParseFlexibleTime(dueDate); err == nil {
				dueDisplay = identity.Time(tParsed)
			} else {
				dueDisplay = dueDate
			}
		}
		description, _ := data["description"].(string)
		if truncateDesc && len(description) > 80 {
			description = description[:80] + "..."
		}
		appendNonEmpty(title, status, createdDisplay, dueDisplay, description)
	case "ticket":
		title, _ := data["title"].(string)
		status, _ := data["status"].(string)
		createdStr := extractCreatedStr(data)
		finishedStr := extractFinishedStr(data)
		dateDisplay := ""
		if status == "open" || status == "OPEN" {
			if createdStr != "" {
				if tParsed, err := workspace.ParseFlexibleTime(createdStr); err == nil {
					dateDisplay = "opened " + identity.Time(tParsed)
				} else {
					dateDisplay = "opened " + createdStr
				}
			}
		} else {
			if finishedStr != "" {
				if tParsed, err := workspace.ParseFlexibleTime(finishedStr); err == nil {
					dateDisplay = "closed " + identity.Time(tParsed)
				} else {
					dateDisplay = "closed " + finishedStr
				}
			} else if createdStr != "" {
				if tParsed, err := workspace.ParseFlexibleTime(createdStr); err == nil {
					dateDisplay = identity.Time(tParsed)
				}
			}
		}
		prompt, _ := data["prompt"].(string)
		summary, _ := data["summary"].(string)
		content := ""
		if status == "open" || status == "OPEN" {
			content = prompt
		} else {
			content = summary
		}
		if truncateDesc && len(content) > 80 {
			content = content[:80] + "..."
		}
		appendNonEmpty(title, status, dateDisplay, content)
	case "bundle":
		root, _ := data["root"].(string)
		ptype, _ := data["projectType"].(string)
		if ptype == "" {
			ptype, _ = data["type"].(string)
		}
		appendNonEmpty(root, ptype)
	case "folder":
	case "file":
	case "section":
		start, _ := data["startLine"].(float64)
		end, _ := data["endLine"].(float64)
		if start > 0 || end > 0 {
			appendNonEmpty(fmt.Sprintf(":%d-%d", int(start), int(end)))
		}
	case "definition":
		name, _ := data["name"].(string)
		start, _ := data["startLine"].(float64)
		end, _ := data["endLine"].(float64)
		appendNonEmpty(name)
		if start > 0 || end > 0 {
			appendNonEmpty(fmt.Sprintf(":%d-%d", int(start), int(end)))
		}
	case "contributor":
		name, _ := data["name"].(string)
		appendNonEmpty(name)
	case "todo":
		name, _ := data["name"].(string)
		appendNonEmpty(name)
	case "draft":
	case "policy":
		desc, _ := data["description"].(string)
		appendNonEmpty(desc)
	case "statute":
		desc, _ := data["description"].(string)
		appendNonEmpty(desc)
	case "technology":
		desc, _ := data["description"].(string)
		appendNonEmpty(desc)
	case "checkpoint":
		msg, _ := data["message"].(string)
		appendNonEmpty(msg)
	case "root":
		name, _ := data["name"].(string)
		appendNonEmpty(name)
	}
	return props
}

// #endregion 🪨️Entity Rendering
func RenderEntityHuman(kind string, data map[string]interface{}, isTTY bool) string {
	id := GetArtifactID(kind, data)
	props := CollectEntityProps(kind, data, false)
	return sanitizeSingleLine(RenderTemplate(TextTpl, "text/entity", map[string]interface{}{
		"ID":    id,
		"Props": props,
		"IsTTY": isTTY,
	}))
}

func InferEntityKind(key string) string {
	key = strings.ToLower(key)
	prefixes := []struct {
		prefix string
		kind   string
	}{
		{"ticket", "ticket"},
		{"goal", "goal"},
		{"file", "file"},
		{"folder", "folder"},
		{"section", "section"},
		{"definition", "definition"},
		{"contributor", "contributor"},
		{"todo", "todo"},
		{"draft", "draft"},
		{"policy", "policy"},
		{"breachkind", "statute"},
		{"bundle", "bundle"},
		{"technology", "technology"},
		{"checkpoint", "checkpoint"},
		{"repo", "root"},
		{"syncgithub", "root"},
		{"syncmanagement", "root"},
		{"integrate", "file"},
		{"extract", "file"},
		{"fix", "root"},
	}
	for _, p := range prefixes {
		if strings.HasPrefix(key, p.prefix) {
			return p.kind
		}
	}
	return ""
}

// 📰️renderEntityMarkdownLink holds the data fields for a renderEntityMarkdownLink record.
func RenderEntityMarkdownLink(kind string, data map[string]interface{}) string {
	id := GetArtifactID(kind, data)
	uri := GetArtifactURI(kind, data)
	props := CollectEntityProps(kind, data, false)
	return sanitizeSingleLine(RenderTemplate(MdTpl, "md/entity_link", map[string]interface{}{
		"ID":    id,
		"URI":   uri,
		"Props": props,
	}))
}

// 🔹️renderEntityMarkdown holds the data fields for a renderEntityMarkdown record.
func RenderEntityMarkdown(kind string, data map[string]interface{}) string {
	id := GetArtifactID(kind, data)
	uri := GetArtifactURI(kind, data)
	props := CollectEntityProps(kind, data, false)
	return sanitizeSingleLine(RenderTemplate(MdTpl, "md/entity_item", map[string]interface{}{
		"ID":    id,
		"URI":   uri,
		"Props": props,
	}))
}

// #endregion 🪨️Entity Rendering
