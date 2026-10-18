// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// ANSI colors, human renderer, and result formatting.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"strings"
	"time"
)

const (
	ColorReset  = "\033[0m"
	ColorRed    = "\033[31m"
	ColorGreen  = "\033[32m"
	ColorYellow = "\033[33m"
	ColorBlue   = "\033[34m"
	ColorDim    = "\033[2m"
	ColorBold   = "\033[1m"
)

// 💿️colorize holds the data fields for a colorize record.
func colorize(s string, color string, enabled bool) string {
	if !enabled {
		return s
	}
	return color + s + ColorReset
}

// 💿️HumanRenderer holds the data fields for a human renderer record.
type HumanRenderer struct {
	Verbose bool
}

// 🔶️Render MUST produce a complete  output.
// ⏹️Render renders the  into its output representation.
func (r HumanRenderer) Render(ctx context.Context, out, errOut io.Writer, stream <-chan Event) (int, error) {
	exitCode := 0
	isTTY := false
	if f, ok := out.(*os.File); ok {
		stat, _ := f.Stat()
		if (stat.Mode() & os.ModeCharDevice) != 0 {
			isTTY = true
		}
	}

	if os.Getenv("NO_COLOR") != "" {
		isTTY = false
	}

	startTime := time.Now()

	for event := range stream {
		if event.Kind == KindDone && event.Done != nil {
			exitCode = event.Done.ExitCode
			duration := time.Since(startTime).Round(time.Millisecond)

			var summary string
			if exitCode != 0 {
				summary = renderTemplate(textTpl, "text/done_failure", map[string]interface{}{
					"IsTTY":    isTTY,
					"Command":  event.Command,
					"Duration": duration,
					"ExitCode": exitCode,
				})
			} else {
				summary = renderTemplate(textTpl, "text/done_success", map[string]interface{}{
					"IsTTY":    isTTY,
					"Command":  event.Command,
					"Duration": duration,
				})
			}

			if isTTY {
				fmt.Fprint(out, "\r\033[K")
			}
			fmt.Fprintln(out, summary)
			continue
		}
		if event.Kind == KindError && event.Error != nil {
			if isTTY {
				fmt.Fprint(out, "\r\033[K")
			}
			fmt.Fprintln(errOut, renderTemplate(textTpl, "text/error", map[string]interface{}{
				"IsTTY":   isTTY,
				"Message": event.Error.Message,
			}))
			if r.Verbose && event.Error.Detail != "" {
				fmt.Fprintln(errOut, renderTemplate(textTpl, "text/error_detail", map[string]interface{}{
					"Detail": event.Error.Detail,
				}))
			}
			continue
		}
		if event.Kind == KindLog && event.Message != "" {
			if isTTY {
				fmt.Fprint(out, "\r\033[K")
			}
			fmt.Fprintln(errOut, renderTemplate(textTpl, "text/log", map[string]interface{}{
				"IsTTY":   isTTY,
				"Message": event.Message,
			}))
			continue
		}
		if event.Kind == KindResult && len(event.Data) > 0 {
			if isTTY {
				fmt.Fprint(out, "\r\033[K")
			}
			fmt.Fprint(out, formatResult(event.Command, event.Data, isTTY))
		}
		if event.Kind == KindProgress && event.Progress != nil {
			if isTTY {
				fmt.Fprint(out, renderTemplate(textTpl, "text/progress_tty", map[string]interface{}{
					"Percent": event.Progress.Percent,
					"Current": event.Progress.Current,
					"Total":   event.Progress.Total,
					"Step":    event.Progress.Step,
				}))
			} else {
				if event.Progress.Percent%10 == 0 && event.Progress.Percent > 0 {
					fmt.Fprintln(out, renderTemplate(textTpl, "text/progress", map[string]interface{}{
						"Percent": event.Progress.Percent,
						"Step":    event.Progress.Step,
					}))
				}
			}
		}
	}
	return exitCode, nil
}

func formatResult(command string, data json.RawMessage, isTTY bool) string {
	var raw map[string]interface{}
	if err := json.Unmarshal(data, &raw); err != nil {
		return renderTemplate(textTpl, "text/result_fallback", map[string]interface{}{
			"IsTTY": isTTY,
			"Data":  string(data),
		}) + "\n"
	}
	payload := raw
	if inner, ok := raw["data"].(map[string]interface{}); ok {
		payload = inner
	}

	var sb strings.Builder

	if output, ok := payload["markdown"].(string); ok {
		return output
	}

	if analyze, ok := payload["analyze"].(map[string]interface{}); ok {
		if metrics, ok := analyze["metrics"].(map[string]interface{}); ok {
			total := metrics["total"]
			autofixable := metrics["autofixable"]
			sb.WriteString(renderTemplate(textTpl, "text/analyze_summary", map[string]interface{}{
				"IsTTY":       isTTY,
				"Total":       total,
				"Autofixable": autofixable,
			}) + "\n")
		}
		if breachs, ok := analyze["breachs"].([]interface{}); ok {
			for _, v := range breachs {
				if vio, ok := v.(map[string]interface{}); ok {
					kind := ""
					if k, ok := vio["kind"].(map[string]interface{}); ok {
						kind = fmt.Sprintf("%v", k["id"])
					} else if kStr, ok := vio["kind"].(string); ok {
						kind = kStr
					}
					scope := fmt.Sprintf("%v", vio["scope"])
					line := fmt.Sprintf("%v", vio["line"])
					summary := fmt.Sprintf("%v", vio["summary"])
					sb.WriteString(renderTemplate(textTpl, "text/breach", map[string]interface{}{
						"IsTTY":   isTTY,
						"Kind":    kind,
						"Scope":   scope,
						"Line":    line,
						"Summary": summary,
					}) + "\n")
				}
			}
		}
		if sb.Len() > 0 {
			return sb.String()
		}
	}

	if fix, ok := payload["fix"].(map[string]interface{}); ok {
		fixed := fix["fixed"]
		remaining := fix["remaining"]
		return renderTemplate(textTpl, "text/fix", map[string]interface{}{
			"IsTTY":     isTTY,
			"Fixed":     fixed,
			"Remaining": remaining,
		}) + "\n"
	}

	if repo, ok := payload["repo"].(map[string]interface{}); ok {
		goals, goalsOk := repo["goals"].([]interface{})
		tickets, _ := repo["tickets"].([]interface{})
		if goalsOk {
			return renderGoalTree(goals, tickets, isTTY, false)
		}
	}

	termWidth := 0
	if isTTY {
		termWidth = getTerminalWidth()
	}

	renderList := func(kind string, items []interface{}) (string, bool) {
		if len(items) == 0 {
			return "", true
		}
		var b strings.Builder
		for _, it := range items {
			if m, ok := it.(map[string]interface{}); ok {
				line := renderEntityHuman(kind, m, isTTY)
				if termWidth > 0 {
					line = truncateANSI(line, termWidth)
				}
				b.WriteString(line)
				b.WriteString("\n")
			}
		}
		return b.String(), true
	}

	if repo, ok := payload["repo"].(map[string]interface{}); ok {
		listKindPairs := []struct {
			key  string
			kind string
		}{
			{"tickets", "ticket"},
			{"goals", "goal"},
			{"bundles", "bundle"},
			{"technologies", "technology"},
			{"folders", "folder"},
			{"files", "file"},
			{"contributors", "contributor"},
			{"policies", "policy"},
			{"statutes", "statute"},
		}
		for _, lk := range listKindPairs {
			if items, ok := repo[lk.key].([]interface{}); ok {
				out, _ := renderList(lk.kind, items)
				return out
			}
		}
	}

	topListPairs := []struct {
		key  string
		kind string
	}{
		{"todos", "todo"},
		{"sections", "section"},
		{"definitions", "definition"},
		{"drafts", "draft"},
	}
	for _, lk := range topListPairs {
		if items, ok := payload[lk.key].([]interface{}); ok {
			out, _ := renderList(lk.kind, items)
			return out
		}
	}

	singleKeys := []string{
		"ticket", "goal", "bundle", "folder", "policy",
		"contributor", "draft", "todo", "technology", "definition",
	}
	for _, key := range singleKeys {
		if entity, ok := payload[key].(map[string]interface{}); ok {
			if topID, ok := payload["id"].(string); ok {
				if _, has := entity["id"]; !has {
					entity["id"] = topID
				}
			}
			return renderEntityHuman(key, entity, isTTY) + "\n"
		}
	}

	if file, ok := payload["file"].(map[string]interface{}); ok {
		line := renderEntityHuman("file", file, isTTY)
		if termWidth > 0 {
			line = truncateANSI(line, termWidth)
		}
		sb.WriteString(line + "\n")
		if sections, ok := file["sections"].([]interface{}); ok {
			var printSections func(items []interface{}, indent string)
			printSections = func(items []interface{}, indent string) {
				for _, item := range items {
					if sec, ok := item.(map[string]interface{}); ok {
						secStr := renderEntityHuman("section", sec, isTTY)
						if termWidth > 0 {
							secStr = truncateANSI(secStr, termWidth)
						}
						sb.WriteString(indent + secStr + "\n")
						if children, ok := sec["children"].([]interface{}); ok && len(children) > 0 {
							printSections(children, indent+"  ")
						}
					}
				}
			}
			printSections(sections, "  ")
		}
		if definitions, ok := file["definitions"].([]interface{}); ok {
			for _, d := range definitions {
				if def, ok := d.(map[string]interface{}); ok {
					defStr := renderEntityHuman("definition", def, isTTY)
					if termWidth > 0 {
						defStr = truncateANSI(defStr, termWidth)
					}
					sb.WriteString("  " + defStr + "\n")
				}
			}
		}
		if sb.Len() > 0 {
			return sb.String()
		}
	}

	if sec, ok := payload["section"].(map[string]interface{}); ok {
		b, _ := json.Marshal(sec)
		var s Section
		json.Unmarshal(b, &s)
		return renderSectionTree(&s, isTTY, false)
	}

	for key, val := range payload {
		if entityMap, ok := val.(map[string]interface{}); ok {
			kind := inferEntityKind(key)
			if kind != "" {
				return renderEntityHuman(kind, entityMap, isTTY) + "\n"
			}
		}
	}

	if id, ok := payload["id"].(string); ok {
		if path, ok := payload["path"].(string); ok {
			return renderTemplate(textTpl, "text/id_path", map[string]interface{}{
				"IsTTY": isTTY,
				"ID":    id,
				"Path":  path,
			}) + "\n"
		}
		return renderTemplate(textTpl, "text/id_only", map[string]interface{}{
			"IsTTY": isTTY,
			"ID":    id,
		}) + "\n"
	}

	return renderEntityHuman("root", payload, isTTY) + "\n"
}
