// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Entity rendering helpers.

// #endregion 🧲️Header

package client

import (
	"fmt"
	humanize "github.com/usalu/semio/repo/client/internal/humanize"
	"os"
	"strconv"
	"strings"
	"unicode/utf8"
)

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
func collectEntityProps(kind string, data map[string]interface{}, truncateDesc bool) []string {
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
			if tParsed, err := parseFlexibleTime(createdStr); err == nil {
				createdDisplay = "created " + humanize.Time(tParsed)
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
			if tParsed, err := parseFlexibleTime(dueDate); err == nil {
				dueDisplay = humanize.Time(tParsed)
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
				if tParsed, err := parseFlexibleTime(createdStr); err == nil {
					dateDisplay = "opened " + humanize.Time(tParsed)
				} else {
					dateDisplay = "opened " + createdStr
				}
			}
		} else {
			if finishedStr != "" {
				if tParsed, err := parseFlexibleTime(finishedStr); err == nil {
					dateDisplay = "closed " + humanize.Time(tParsed)
				} else {
					dateDisplay = "closed " + finishedStr
				}
			} else if createdStr != "" {
				if tParsed, err := parseFlexibleTime(createdStr); err == nil {
					dateDisplay = humanize.Time(tParsed)
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

func renderEntityHuman(kind string, data map[string]interface{}, isTTY bool) string {
	id := GetArtifactID(kind, data)
	props := collectEntityProps(kind, data, false)
	return sanitizeSingleLine(renderTemplate(textTpl, "text/entity", map[string]interface{}{
		"ID":    id,
		"Props": props,
		"IsTTY": isTTY,
	}))
}

func inferEntityKind(key string) string {
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
func renderEntityMarkdownLink(kind string, data map[string]interface{}) string {
	id := GetArtifactID(kind, data)
	uri := GetArtifactURI(kind, data)
	props := collectEntityProps(kind, data, false)
	return sanitizeSingleLine(renderTemplate(mdTpl, "md/entity_link", map[string]interface{}{
		"ID":    id,
		"URI":   uri,
		"Props": props,
	}))
}

// 🔹️renderEntityMarkdown holds the data fields for a renderEntityMarkdown record.
func renderEntityMarkdown(kind string, data map[string]interface{}) string {
	id := GetArtifactID(kind, data)
	uri := GetArtifactURI(kind, data)
	props := collectEntityProps(kind, data, false)
	return sanitizeSingleLine(renderTemplate(mdTpl, "md/entity_item", map[string]interface{}{
		"ID":    id,
		"URI":   uri,
		"Props": props,
	}))
}

// 🔸️getTerminalWidth holds the data fields for a getTerminalWidth record.
func getTerminalWidth() int {
	w := os.Getenv("COLUMNS")
	if w != "" {
		if n, err := strconv.Atoi(w); err == nil && n > 0 {
			return n
		}
	}
	return 120
}

func truncateANSI(s string, maxVisible int) string {
	if maxVisible <= 0 {
		return s
	}
	visible := 0
	for i := 0; i < len(s); {
		if s[i] == 0x1b {
			j := i + 1
			for j < len(s) && s[j] != 'm' {
				j++
			}
			if j < len(s) {
				j++
			}
			i = j
			continue
		}
		_, size := utf8.DecodeRuneInString(s[i:])
		if size <= 0 {
			break
		}
		i += size
		visible++
		if visible > maxVisible {
			break
		}
	}
	if visible <= maxVisible {
		return s
	}
	if maxVisible <= 3 {
		maxVisible = 3
	}
	limit := maxVisible - 3
	var b strings.Builder
	visible = 0
	for i := 0; i < len(s); {
		if s[i] == 0x1b {
			j := i + 1
			for j < len(s) && s[j] != 'm' {
				j++
			}
			if j < len(s) {
				j++
			}
			b.WriteString(s[i:j])
			i = j
			continue
		}
		r, size := utf8.DecodeRuneInString(s[i:])
		if size <= 0 {
			break
		}
		if visible >= limit {
			break
		}
		b.WriteRune(r)
		visible++
		i += size
	}
	b.WriteString("...")
	b.WriteString("\x1b[0m")
	return b.String()
}
