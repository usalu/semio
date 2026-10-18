// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// NDJSON and markdown stream renderers with tool result helpers.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"fmt"
	command "github.com/usalu/semio/repo/client/internal/command"
	"io"
	"strings"
)

// 🔌️StreamRenderer defines the interface contract for stream renderer operations.
type StreamRenderer interface {
	Render(ctx context.Context, out, errOut io.Writer, stream <-chan Event) (int, error)
}

// 🎨️NDJSONRenderer holds the data fields for a n d j s o n renderer record.
type NDJSONRenderer struct{}

// 🔷️Render MUST produce a complete  output.
// 💾️Render renders the  into its output representation.
func (r NDJSONRenderer) Render(ctx context.Context, out, errOut io.Writer, stream <-chan Event) (int, error) {
	encoder := json.NewEncoder(out)
	encoder.SetEscapeHTML(false)

	errEncoder := json.NewEncoder(errOut)
	errEncoder.SetEscapeHTML(false)

	exitCode := 0
	for event := range stream {
		if event.Kind == KindDone && event.Done != nil {
			exitCode = event.Done.ExitCode
		}
		if event.Kind == KindResult && event.Data != nil {
			out.Write(event.Data)
			out.Write([]byte("\n"))
		}
		if event.Kind == KindError && event.Error != nil {
			errEncoder.Encode(event.Error)
		}

		if f, ok := out.(interface{ Flush() error }); ok {
			f.Flush()
		}
	}
	return exitCode, nil
}

// 📰️MarkdownRenderer holds the data fields for a markdown renderer record.
type MarkdownRenderer struct{}

// 🔹️Render MUST produce a complete  output.
// 🔢️Render renders the  into its output representation.
func (r MarkdownRenderer) Render(ctx context.Context, out, errOut io.Writer, stream <-chan Event) (int, error) {
	exitCode := 0
	for event := range stream {
		if event.Kind == KindDone && event.Done != nil {
			exitCode = event.Done.ExitCode
			continue
		}
		if event.Kind == KindResult && len(event.Data) > 0 {
			fmt.Fprint(out, formatMarkdownResult(event.Command, event.Data))
		}
		if event.Kind == KindError && event.Error != nil {
			fmt.Fprintln(errOut, renderTemplate(mdTpl, "md/error", map[string]interface{}{
				"Message": event.Error.Message,
			}))
			if event.Error.Detail != "" {
				fmt.Fprintln(errOut, renderTemplate(mdTpl, "md/error_detail", map[string]interface{}{
					"Detail": event.Error.Detail,
				}))
			}
		}
	}
	return exitCode, nil
}

// 📋️formatMarkdownResult holds the data fields for a formatMarkdownResult record.
func formatMarkdownResult(command string, data json.RawMessage) string {
	var raw map[string]interface{}
	if err := json.Unmarshal(data, &raw); err != nil {
		return renderEntityMarkdownLink("root", map[string]interface{}{"name": string(data)}) + "\n"
	}
	payload := raw
	if inner, ok := raw["data"].(map[string]interface{}); ok {
		payload = inner
	}

	var sb strings.Builder

	if markdown, ok := payload["markdown"].(string); ok {
		return markdown
	}

	if analyze, ok := payload["analyze"].(map[string]interface{}); ok {
		if metrics, ok := analyze["metrics"].(map[string]interface{}); ok {
			total := metrics["total"]
			autofixable := metrics["autofixable"]
			sb.WriteString(renderTemplate(mdTpl, "md/analyze_total", map[string]interface{}{
				"Total": total,
			}) + "\n")
			if autofixable != nil {
				sb.WriteString(renderTemplate(mdTpl, "md/analyze_autofixable", map[string]interface{}{
					"Autofixable": autofixable,
				}) + "\n")
			}
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
					sb.WriteString(renderTemplate(mdTpl, "md/breach", map[string]interface{}{
						"Kind":    kind,
						"Scope":   scope,
						"Line":    line,
						"Summary": summary,
					}) + "\n")
				}
			}
		}
		return sb.String()
	}

	if repo, ok := payload["repo"].(map[string]interface{}); ok {
		goals, goalsOk := repo["goals"].([]interface{})
		tickets, _ := repo["tickets"].([]interface{})
		if goalsOk {
			return renderGoalTree(goals, tickets, false, true)
		}
	}

	listKeys := []struct {
		wrap string
		key  string
		kind string
	}{
		{"repo", "tickets", "ticket"},
		{"repo", "goals", "goal"},
		{"repo", "bundles", "bundle"},
		{"repo", "technologies", "technology"},
		{"repo", "folders", "folder"},
		{"repo", "files", "file"},
		{"repo", "contributors", "contributor"},
		{"repo", "policies", "policy"},
		{"repo", "statutes", "statute"},
		{"", "todos", "todo"},
		{"", "sections", "section"},
		{"", "definitions", "definition"},
		{"", "drafts", "draft"},
	}
	for _, lk := range listKeys {
		var container map[string]interface{}
		if lk.wrap != "" {
			if w, ok := payload[lk.wrap].(map[string]interface{}); ok {
				container = w
			}
		} else {
			container = payload
		}
		if container == nil {
			continue
		}
		if items, ok := container[lk.key].([]interface{}); ok {
			for _, item := range items {
				if m, ok := item.(map[string]interface{}); ok {
					sb.WriteString(renderEntityMarkdown(lk.kind, m) + "\n")
				}
			}
			return sb.String()
		}
	}

	isList := strings.HasSuffix(command, " list") || strings.HasSuffix(command, " tree")

	singleKeys := []string{
		"ticket", "goal", "bundle", "folder", "file", "definition",
		"contributor", "policy", "technology", "draft", "todo", "checkpoint",
	}
	for _, key := range singleKeys {
		if entity, ok := payload[key].(map[string]interface{}); ok {
			if topID, ok := payload["id"].(string); ok {
				if _, has := entity["id"]; !has {
					entity["id"] = topID
				}
			}
			if key == "file" && !isList {
				return formatMarkdownFile(entity)
			}
			if key == "section" {
				b, _ := json.Marshal(entity)
				var s Section
				json.Unmarshal(b, &s)
				return renderSectionTree(&s, false, true)
			}
			if isList {
				return renderEntityMarkdown(key, entity) + "\n"
			}
			return renderEntityMarkdownLink(key, entity) + "\n"
		}
	}

	if sec, ok := payload["section"].(map[string]interface{}); ok {
		b, _ := json.Marshal(sec)
		var s Section
		json.Unmarshal(b, &s)
		return renderSectionTree(&s, false, true)
	}

	for key, val := range payload {
		if entityMap, ok := val.(map[string]interface{}); ok {
			kind := inferEntityKind(key)
			if kind != "" {
				return renderEntityMarkdownLink(kind, entityMap) + "\n"
			}
		}
	}

	return renderEntityMarkdownLink("root", payload) + "\n"
}

// 📄️formatMarkdownFile holds the data fields for a formatMarkdownFile record.
func formatMarkdownFile(file map[string]interface{}) string {
	var sb strings.Builder
	sb.WriteString(renderEntityMarkdownLink("file", file) + "\n")

	filePath, _ := file["path"].(string)
	if id, ok := file["id"].(string); ok && id != "" {
		filePath = id
	}

	if sections, ok := file["sections"].([]interface{}); ok {
		var printSections func(items []interface{}, indent string)
		printSections = func(items []interface{}, indent string) {
			for _, item := range items {
				if sec, ok := item.(map[string]interface{}); ok {
					if _, hasPath := sec["filePath"]; !hasPath {
						sec["filePath"] = filePath
					}
					line := renderEntityMarkdown("section", sec)
					sb.WriteString(fmt.Sprintf("%s%s\n", indent, line))
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
				if _, hasPath := def["filePath"]; !hasPath {
					def["filePath"] = filePath
				}
				sb.WriteString("  " + renderEntityMarkdown("definition", def) + "\n")
			}
		}
	}
	return sb.String()
}

// 🔸️renderStream holds the data fields for a renderStream record.
func renderStream(cmd *command.Command, config *Config, stream <-chan Event) error {
	var renderer StreamRenderer
	if config.IsJSON() {
		renderer = NDJSONRenderer{}
	} else if config.IsMarkdown() {
		renderer = MarkdownRenderer{}
	} else {
		renderer = HumanRenderer{Verbose: config.Verbose}
	}

	exitCode, err := renderer.Render(cmd.Context(), cmd.OutOrStdout(), cmd.ErrOrStderr(), stream)
	if err != nil {
		return err
	}
	if exitCode != 0 {
		return ExitError{Code: exitCode}
	}
	return nil
}

// 📡️renderEventsToMarkdown holds the data fields for a renderEventsToMarkdown record.
func renderEventsToMarkdown(events []Event) string {
	var sb strings.Builder
	for _, event := range events {
		if event.Kind == KindResult && len(event.Data) > 0 {
			sb.WriteString(formatMarkdownResult(event.Command, event.Data))
		}
		if event.Kind == KindError && event.Error != nil {
			sb.WriteString(renderTemplate(mdTpl, "md/error", map[string]interface{}{
				"Message": event.Error.Message,
			}) + "\n")
		}
	}
	return sb.String()
}

// ❌️toolErrorResult holds the data fields for a toolErrorResult record.
func toolErrorResult(err error) ToolResult {
	output := NewOutput()
	output.Error(fmt.Sprintf("Error: %v", err))
	return ToolResult{Output: *output, Error: err.Error()}
}

// 🔺️toolErrorMsg holds the data fields for a toolErrorMsg record.
func toolErrorMsg(msg string) ToolResult {
	output := NewOutput()
	output.Error(fmt.Sprintf("Error: %s", msg))
	return ToolResult{Output: *output, Error: msg}
}

// 🔻️toolResultFromEvents holds the data fields for a toolResultFromEvents record.
func toolResultFromEvents(events []Event, data interface{}) ToolResult {
	text := renderEventsToMarkdown(events)
	output := NewOutput()
	if text != "" {
		output.Plain(text)
	}
	for _, event := range events {
		if event.Kind == KindError && event.Error != nil {
			return ToolResult{Output: *output, Data: data, Error: event.Error.Message}
		}
		if event.Kind == KindDone && event.Done != nil && event.Done.ExitCode != 0 {
			output.ExitCode = event.Done.ExitCode
		}
	}
	return ToolResult{Output: *output, Data: data}
}

// 🌳️toolResultFromTreeList holds the data fields for a toolResultFromTreeList record.
func toolResultFromTreeList(nodeKind TreeNodeKind) ToolResult {
	ctx := context.Background()
	tree := BuildMonorepoTreeCached(ctx, TreeBuildOptions{})
	filter := TreeFilter{OnlyKinds: map[TreeNodeKind]bool{nodeKind: true}}
	tree = FilterMonorepoTree(tree, &filter)
	var nodes []*TreeNode
	flattenTreeNodes(tree, &nodes)
	var sb strings.Builder
	for _, n := range nodes {
		entityKind := treeNodeKindToEntityKind(n.Kind)
		if entityKind == "" {
			entityKind = string(n.Kind)
		}
		if n.Data != nil {
			sb.WriteString(renderEntityMarkdown(entityKind, n.Data) + "\n")
		}
	}
	output := NewOutput()
	output.Plain(sb.String())
	return ToolResult{Output: *output}
}

// ⬛️toolResultFromTreeRender holds the data fields for a toolResultFromTreeRender record.
func toolResultFromTreeRender(nodeKind TreeNodeKind) ToolResult {
	ctx := context.Background()
	tree := BuildMonorepoTreeCached(ctx, TreeBuildOptions{})
	filter := TreeFilter{OnlyKinds: map[TreeNodeKind]bool{nodeKind: true}}
	tree = FilterMonorepoTree(tree, &filter)
	text := RenderMonorepoTreeMarkdown(tree)
	output := NewOutput()
	output.Plain(text)
	return ToolResult{Output: *output}
}

// 🕸️runGraphQL holds the data fields for a runGraphQL record.
func runGraphQL(cmd *command.Command, factory EngineFactory, config *Config, query string, variables map[string]interface{}) error {
	argsPayload := GraphQLArgs{Query: query, Variables: variables}
	payloadBytes, err := json.Marshal(argsPayload)
	if err != nil {
		return err
	}
	engine, err := factory(*config)
	if err != nil {
		return err
	}
	ctx := context.Background()
	if config.Timeout > 0 {
		ctxWithTimeout, cancel := context.WithTimeout(ctx, config.Timeout)
		defer cancel()
		ctx = ctxWithTimeout
	}
	request := Request{Command: CmdGraphQL, Args: payloadBytes, RepoRoot: config.Repo, Verbose: config.Verbose}
	stream := engine.Run(ctx, request)
	return renderStream(cmd, config, stream)
}
