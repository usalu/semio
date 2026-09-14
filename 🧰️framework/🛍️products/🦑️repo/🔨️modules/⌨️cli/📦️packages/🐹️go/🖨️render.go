// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🖨️render carries the cli stream renderers: NDJSON, human and markdown projections of the engine event stream.

// #endregion 🧲️Header

package cli

import (
	context "context"
	json "encoding/json"
	fmt "fmt"
	io "io"
	os "os"
	strings "strings"
	time "time"

	model "github.com/usalu/semio/repo/model"
	treepkg "github.com/usalu/semio/repo/tree"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🌩️CLI Renderers

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
				summary = model.RenderTemplate(model.TextTpl, "text/done_failure", map[string]interface{}{
					"IsTTY":    isTTY,
					"Command":  event.Command,
					"Duration": duration,
					"ExitCode": exitCode,
				})
			} else {
				summary = model.RenderTemplate(model.TextTpl, "text/done_success", map[string]interface{}{
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
			fmt.Fprintln(errOut, model.RenderTemplate(model.TextTpl, "text/error", map[string]interface{}{
				"IsTTY":   isTTY,
				"Message": event.Error.Message,
			}))
			if r.Verbose && event.Error.Detail != "" {
				fmt.Fprintln(errOut, model.RenderTemplate(model.TextTpl, "text/error_detail", map[string]interface{}{
					"Detail": event.Error.Detail,
				}))
			}
			continue
		}
		if event.Kind == KindLog && event.Message != "" {
			if isTTY {
				fmt.Fprint(out, "\r\033[K")
			}
			fmt.Fprintln(errOut, model.RenderTemplate(model.TextTpl, "text/log", map[string]interface{}{
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
				fmt.Fprint(out, model.RenderTemplate(model.TextTpl, "text/progress_tty", map[string]interface{}{
					"Percent": event.Progress.Percent,
					"Current": event.Progress.Current,
					"Total":   event.Progress.Total,
					"Step":    event.Progress.Step,
				}))
			} else {
				if event.Progress.Percent%10 == 0 && event.Progress.Percent > 0 {
					fmt.Fprintln(out, model.RenderTemplate(model.TextTpl, "text/progress", map[string]interface{}{
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
		return model.RenderTemplate(model.TextTpl, "text/result_fallback", map[string]interface{}{
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
			sb.WriteString(model.RenderTemplate(model.TextTpl, "text/analyze_summary", map[string]interface{}{
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
					sb.WriteString(model.RenderTemplate(model.TextTpl, "text/breach", map[string]interface{}{
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
		return model.RenderTemplate(model.TextTpl, "text/fix", map[string]interface{}{
			"IsTTY":     isTTY,
			"Fixed":     fixed,
			"Remaining": remaining,
		}) + "\n"
	}

	if repo, ok := payload["repo"].(map[string]interface{}); ok {
		goals, goalsOk := repo["goals"].([]interface{})
		tickets, _ := repo["tickets"].([]interface{})
		if goalsOk {
			return treepkg.RenderGoalTree(goals, tickets, isTTY, false)
		}
	}

	termWidth := 0
	if isTTY {
		termWidth = model.GetTerminalWidth()
	}

	renderList := func(kind string, items []interface{}) (string, bool) {
		if len(items) == 0 {
			return "", true
		}
		var b strings.Builder
		for _, it := range items {
			if m, ok := it.(map[string]interface{}); ok {
				line := model.RenderEntityHuman(kind, m, isTTY)
				if termWidth > 0 {
					line = model.TruncateANSI(line, termWidth)
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
			return model.RenderEntityHuman(key, entity, isTTY) + "\n"
		}
	}

	if file, ok := payload["file"].(map[string]interface{}); ok {
		line := model.RenderEntityHuman("file", file, isTTY)
		if termWidth > 0 {
			line = model.TruncateANSI(line, termWidth)
		}
		sb.WriteString(line + "\n")
		if sections, ok := file["sections"].([]interface{}); ok {
			var printSections func(items []interface{}, indent string)
			printSections = func(items []interface{}, indent string) {
				for _, item := range items {
					if sec, ok := item.(map[string]interface{}); ok {
						secStr := model.RenderEntityHuman("section", sec, isTTY)
						if termWidth > 0 {
							secStr = model.TruncateANSI(secStr, termWidth)
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
					defStr := model.RenderEntityHuman("definition", def, isTTY)
					if termWidth > 0 {
						defStr = model.TruncateANSI(defStr, termWidth)
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
		var s model.Section
		json.Unmarshal(b, &s)
		return treepkg.RenderSectionTree(&s, isTTY, false)
	}

	for key, val := range payload {
		if entityMap, ok := val.(map[string]interface{}); ok {
			kind := model.InferEntityKind(key)
			if kind != "" {
				return model.RenderEntityHuman(kind, entityMap, isTTY) + "\n"
			}
		}
	}

	if id, ok := payload["id"].(string); ok {
		if path, ok := payload["path"].(string); ok {
			return model.RenderTemplate(model.TextTpl, "text/id_path", map[string]interface{}{
				"IsTTY": isTTY,
				"ID":    id,
				"Path":  path,
			}) + "\n"
		}
		return model.RenderTemplate(model.TextTpl, "text/id_only", map[string]interface{}{
			"IsTTY": isTTY,
			"ID":    id,
		}) + "\n"
	}

	return model.RenderEntityHuman("root", payload, isTTY) + "\n"
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
			fmt.Fprintln(errOut, model.RenderTemplate(model.MdTpl, "md/error", map[string]interface{}{
				"Message": event.Error.Message,
			}))
			if event.Error.Detail != "" {
				fmt.Fprintln(errOut, model.RenderTemplate(model.MdTpl, "md/error_detail", map[string]interface{}{
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
		return model.RenderEntityMarkdownLink("root", map[string]interface{}{"name": string(data)}) + "\n"
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
			sb.WriteString(model.RenderTemplate(model.MdTpl, "md/analyze_total", map[string]interface{}{
				"Total": total,
			}) + "\n")
			if autofixable != nil {
				sb.WriteString(model.RenderTemplate(model.MdTpl, "md/analyze_autofixable", map[string]interface{}{
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
					sb.WriteString(model.RenderTemplate(model.MdTpl, "md/breach", map[string]interface{}{
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
			return treepkg.RenderGoalTree(goals, tickets, false, true)
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
					sb.WriteString(model.RenderEntityMarkdown(lk.kind, m) + "\n")
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
				var s model.Section
				json.Unmarshal(b, &s)
				return treepkg.RenderSectionTree(&s, false, true)
			}
			if isList {
				return model.RenderEntityMarkdown(key, entity) + "\n"
			}
			return model.RenderEntityMarkdownLink(key, entity) + "\n"
		}
	}

	if sec, ok := payload["section"].(map[string]interface{}); ok {
		b, _ := json.Marshal(sec)
		var s model.Section
		json.Unmarshal(b, &s)
		return treepkg.RenderSectionTree(&s, false, true)
	}

	for key, val := range payload {
		if entityMap, ok := val.(map[string]interface{}); ok {
			kind := model.InferEntityKind(key)
			if kind != "" {
				return model.RenderEntityMarkdownLink(kind, entityMap) + "\n"
			}
		}
	}

	return model.RenderEntityMarkdownLink("root", payload) + "\n"
}

// 📄️formatMarkdownFile holds the data fields for a formatMarkdownFile record.
func formatMarkdownFile(file map[string]interface{}) string {
	var sb strings.Builder
	sb.WriteString(model.RenderEntityMarkdownLink("file", file) + "\n")

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
					line := model.RenderEntityMarkdown("section", sec)
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
				sb.WriteString("  " + model.RenderEntityMarkdown("definition", def) + "\n")
			}
		}
	}
	return sb.String()
}

// 🔸️renderStream holds the data fields for a renderStream record.
func renderStream(cmd *Command, config *Config, stream <-chan Event) error {
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
		return workspace.ExitError{Code: exitCode}
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
			sb.WriteString(model.RenderTemplate(model.MdTpl, "md/error", map[string]interface{}{
				"Message": event.Error.Message,
			}) + "\n")
		}
	}
	return sb.String()
}

// 🔻️toolResultFromEvents holds the data fields for a toolResultFromEvents record.
func toolResultFromEvents(events []Event, data interface{}) workspace.ToolResult {
	text := renderEventsToMarkdown(events)
	output := workspace.NewOutput()
	if text != "" {
		output.Plain(text)
	}
	for _, event := range events {
		if event.Kind == KindError && event.Error != nil {
			return workspace.ToolResult{Output: *output, Data: data, Error: event.Error.Message}
		}
		if event.Kind == KindDone && event.Done != nil && event.Done.ExitCode != 0 {
			output.ExitCode = event.Done.ExitCode
		}
	}
	return workspace.ToolResult{Output: *output, Data: data}
}

// 🌳️toolResultFromTreeList holds the data fields for a toolResultFromTreeList record.
func toolResultFromTreeList(nodeKind treepkg.TreeNodeKind) workspace.ToolResult {
	ctx := context.Background()
	tree := treepkg.BuildMonorepoTreeCached(ctx, treepkg.TreeBuildOptions{})
	filter := treepkg.TreeFilter{OnlyKinds: map[treepkg.TreeNodeKind]bool{nodeKind: true}}
	tree = treepkg.FilterMonorepoTree(tree, &filter)
	var nodes []*treepkg.TreeNode
	flattenTreeNodes(tree, &nodes)
	var sb strings.Builder
	for _, n := range nodes {
		entityKind := treepkg.TreeNodeKindToEntityKind(n.Kind)
		if entityKind == "" {
			entityKind = string(n.Kind)
		}
		if n.Data != nil {
			sb.WriteString(model.RenderEntityMarkdown(entityKind, n.Data) + "\n")
		}
	}
	output := workspace.NewOutput()
	output.Plain(sb.String())
	return workspace.ToolResult{Output: *output}
}

// ⬛️toolResultFromTreeRender holds the data fields for a toolResultFromTreeRender record.
func toolResultFromTreeRender(nodeKind treepkg.TreeNodeKind) workspace.ToolResult {
	ctx := context.Background()
	tree := treepkg.BuildMonorepoTreeCached(ctx, treepkg.TreeBuildOptions{})
	filter := treepkg.TreeFilter{OnlyKinds: map[treepkg.TreeNodeKind]bool{nodeKind: true}}
	tree = treepkg.FilterMonorepoTree(tree, &filter)
	text := treepkg.RenderMonorepoTreeMarkdown(tree, treepkg.DefaultEntityRenderer{})
	output := workspace.NewOutput()
	output.Plain(text)
	return workspace.ToolResult{Output: *output}
}

// 🕸️runGraphQL holds the data fields for a runGraphQL record.
func runGraphQL(cmd *Command, factory EngineFactory, config *Config, query string, variables map[string]interface{}) error {
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

// #endregion 🌩️CLI Renderers
