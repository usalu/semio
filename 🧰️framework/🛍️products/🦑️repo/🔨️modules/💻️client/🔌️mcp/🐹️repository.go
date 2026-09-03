// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// #endregion 🧲️Header

package main

import (
	"context"
	"encoding/json"
	"errors"
	"io"
	"strconv"
	"strings"

	"github.com/usalu/semio/repo/client"
)

// #region 🔗️DomainBoundary

type RepositoryResult struct {
	Text       string
	Structured json.RawMessage
	IsError    bool
}

type RepositoryHandlers interface {
	Call(context.Context, string, json.RawMessage) (RepositoryResult, error)
	Read(context.Context, string) (ResourceContent, error)
	Prompt(context.Context, string, map[string]string) (GetPromptResult, error)
}

type ClientRepository struct {
	profile client.McpClientKind
}

func NewClientRepository(profile client.McpClientKind) ClientRepository {
	if profile == "" {
		profile = client.McpClientGeneric
	}
	return ClientRepository{profile: profile}
}

func (repository ClientRepository) Call(ctx context.Context, name string, raw json.RawMessage) (RepositoryResult, error) {
	if err := ctx.Err(); err != nil {
		return RepositoryResult{}, err
	}
	var result client.ToolResult
	switch name {
	case "ticket_open":
		var params struct {
			Emoji        string `json:"emoji"`
			Title        string `json:"title"`
			Prompt       string `json:"prompt"`
			Goal         string `json:"goal"`
			Client       string `json:"client"`
			LLM          string `json:"llm"`
			Effort       string `json:"effort"`
			Draft        string `json:"draft"`
			Parent       string `json:"parent"`
			Issue        string `json:"issue"`
			PlanID       string `json:"plan_id"`
			SpecID       string `json:"spec_id"`
			NoIssue      bool   `json:"no_issue"`
			NoManagement bool   `json:"no_management"`
		}
		if err := DecodeParams(raw, &params); err != nil || params.Emoji == "" || params.Title == "" || params.Prompt == "" || params.Goal == "" {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid ticket_open arguments"}
		}
		result = client.ToolTicketOpen(params.Emoji, params.Title, params.Prompt, params.LLM, params.Effort, params.Client, params.Draft, params.NoIssue, params.Goal, params.Parent, params.NoManagement, params.Issue, repository.profile, params.PlanID, params.SpecID)
	case "ticket_close":
		var params struct {
			Path         string   `json:"path"`
			Summary      string   `json:"summary"`
			Files        []string `json:"files"`
			Title        string   `json:"title"`
			NoManagement bool     `json:"no_management"`
		}
		if err := DecodeParams(raw, &params); err != nil || params.Summary == "" {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid ticket_close arguments"}
		}
		year, month, day, slug, err := resolveTicketPath(params.Path)
		if err != nil {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid ticket path"}
		}
		result = client.ToolTicketClose(year, month, day, slug, params.Summary, params.Files, params.Title, params.NoManagement)
	case "ticket_reopen":
		var params struct {
			Path         string `json:"path"`
			Prompt       string `json:"prompt"`
			LLM          string `json:"llm"`
			Effort       string `json:"effort"`
			Client       string `json:"client"`
			Draft        string `json:"draft"`
			Title        string `json:"title"`
			Goal         string `json:"goal"`
			Parent       string `json:"parent"`
			PlanID       string `json:"plan_id"`
			SpecID       string `json:"spec_id"`
			NoManagement bool   `json:"no_management"`
		}
		if err := DecodeParams(raw, &params); err != nil {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid ticket_reopen arguments"}
		}
		year, month, day, slug, err := resolveTicketPath(params.Path)
		if err != nil {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid ticket path"}
		}
		result = client.ToolTicketReopen(year, month, day, slug, params.Prompt, params.LLM, params.Effort, params.Client, params.Draft, params.Title, params.Goal, params.Parent, params.NoManagement, repository.profile, params.PlanID, params.SpecID)
	case "section_move":
		var params struct {
			File    string `json:"file"`
			OldName string `json:"old_name"`
			NewName string `json:"new_name"`
		}
		if err := DecodeParams(raw, &params); err != nil || params.File == "" || params.OldName == "" || params.NewName == "" {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid section_move arguments"}
		}
		result = client.ToolSectionMove(params.File, params.OldName, params.NewName)
	case "file_integrate":
		var params struct {
			Source              string `json:"source"`
			TargetSection       string `json:"target_section"`
			TargetFile          string `json:"target_file"`
			TargetParentSection string `json:"target_parent_section"`
		}
		if err := DecodeParams(raw, &params); err != nil || params.Source == "" || params.TargetSection == "" || params.TargetFile == "" {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid file_integrate arguments"}
		}
		result = client.ToolIntegrate(params.Source, params.TargetSection, params.TargetFile, params.TargetParentSection)
	case "section_extract":
		var params struct {
			SourceFile    string `json:"source_file"`
			SourceSection string `json:"source_section"`
			TargetFile    string `json:"target_file"`
		}
		if err := DecodeParams(raw, &params); err != nil || params.SourceFile == "" || params.SourceSection == "" || params.TargetFile == "" {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid section_extract arguments"}
		}
		result = client.ToolExtract(params.SourceFile, params.SourceSection, params.TargetFile)
	default:
		return RepositoryResult{}, &HandlerError{Code: -32010, Message: "tool not found"}
	}
	if err := ctx.Err(); err != nil {
		return RepositoryResult{}, err
	}
	return repositoryResult(result)
}

func (ClientRepository) Read(ctx context.Context, uri string) (ResourceContent, error) {
	if err := ctx.Err(); err != nil {
		return ResourceContent{}, err
	}
	var result client.ToolResult
	switch uri {
	case "repo://":
		result = client.ToolCodebase()
	case "repo://bundles":
		result = client.ToolBundleList()
	case "repo://folders":
		result = client.ToolFolderList("")
	case "repo://files":
		result = client.ToolFileList("")
	case "repo://tickets":
		result = client.ToolTicketList(nil, nil, nil)
	case "repo://goals":
		result = client.ToolGoalList()
	case "repo://policies":
		result = client.ToolPolicyList()
	case "repo://contributors":
		result = client.ToolContributorList()
	default:
		return ResourceContent{}, errors.New("mcp: resource not found")
	}
	converted, err := repositoryResult(result)
	if err != nil {
		return ResourceContent{}, err
	}
	if converted.IsError {
		return ResourceContent{}, errors.New("mcp: resource handler failed")
	}
	return ResourceContent{URI: uri, MIMEType: "text/plain", Text: converted.Text}, nil
}

func (ClientRepository) Prompt(ctx context.Context, name string, arguments map[string]string) (GetPromptResult, error) {
	if err := ctx.Err(); err != nil {
		return GetPromptResult{}, err
	}
	prompt := arguments["prompt"]
	var instruction string
	switch name {
	case "enhance":
		instruction = "Enhance the request while preserving its intent and constraints."
	case "refactor":
		instruction = "Refactor the requested scope completely and preserve observable behavior."
	case "test":
		instruction = "Test the requested scope with executable success and hostile cases."
	case "comply":
		instruction = "Apply the repository instructions and resolve every in-scope breach."
	default:
		return GetPromptResult{}, errors.New("mcp: prompt not found")
	}
	return GetPromptResult{Description: instruction, Messages: []PromptMessage{{Role: "user", Content: Content{Type: "text", Text: instruction + "\n\n" + prompt}}}}, nil
}

func repositoryResult(result client.ToolResult) (RepositoryResult, error) {
	structured, err := json.Marshal(result.Data)
	if err != nil {
		return RepositoryResult{}, errors.New("mcp: repository result encoding failed")
	}
	var text strings.Builder
	for index, line := range result.Output.Lines {
		if index > 0 {
			text.WriteByte('\n')
		}
		text.WriteString(line.Text)
	}
	if result.Error != "" {
		if text.Len() > 0 {
			text.WriteByte('\n')
		}
		text.WriteString(result.Error)
	}
	return RepositoryResult{Text: text.String(), Structured: structured, IsError: result.Error != "" || result.Output.ExitCode != 0}, nil
}

func resolveTicketPath(path string) (int, int, int, string, error) {
	if path == "" {
		ticket, err := client.LatestTicket()
		if err != nil || ticket == nil {
			return 0, 0, 0, "", errors.New("mcp: ticket not found")
		}
		return ticket.Year, ticket.Month, ticket.Day, ticket.Slug, nil
	}
	parts := strings.Split(strings.Trim(path, "/"), "/")
	if len(parts) != 4 || parts[3] == "" {
		return 0, 0, 0, "", errors.New("mcp: invalid ticket path")
	}
	year, yearErr := strconv.Atoi(parts[0])
	month, monthErr := strconv.Atoi(parts[1])
	day, dayErr := strconv.Atoi(parts[2])
	if yearErr != nil || monthErr != nil || dayErr != nil || year < 0 || year > 99 || month < 1 || month > 12 || day < 1 || day > 31 {
		return 0, 0, 0, "", errors.New("mcp: invalid ticket path")
	}
	return year, month, day, parts[3], nil
}

// #endregion 🔗️DomainBoundary

// #region 🏭️ProductionServer

func NewRepositoryServer(repository RepositoryHandlers) (*Server, error) {
	return NewRepositoryServerFor(repository, client.McpClientGeneric)
}

func NewRepositoryServerFor(repository RepositoryHandlers, profile client.McpClientKind) (*Server, error) {
	return NewRepositoryServerWithLimitsFor(repository, profile, DefaultLimits())
}

func NewRepositoryServerWithLimits(repository RepositoryHandlers, limits Limits) (*Server, error) {
	return NewRepositoryServerWithLimitsFor(repository, client.McpClientGeneric, limits)
}

func NewRepositoryServerWithLimitsFor(repository RepositoryHandlers, profile client.McpClientKind, limits Limits) (*Server, error) {
	if repository == nil {
		return nil, errors.New("mcp: repository handlers are required")
	}
	resolvedProfile, err := client.ParseMcpClientKind(string(profile))
	if err != nil {
		return nil, err
	}
	server, err := NewServer(Config{ServerInfo: Implementation{Name: client.McpServerName(resolvedProfile), Version: "1.0.0"}, Instructions: "Use repository tools and resources through their owned schemas.", Limits: limits})
	if err != nil {
		return nil, err
	}
	object := func(properties map[string]Schema, required ...string) Schema {
		additional := false
		return Schema{Type: "object", Properties: properties, Required: required, AdditionalProperties: &additional}
	}
	stringField := func(description string) Schema { return Schema{Type: "string", Description: description} }
	booleanField := func(description string) Schema { return Schema{Type: "boolean", Description: description} }
	arrayField := func(description string) Schema {
		return Schema{Type: "array", Description: description, Items: &Schema{Type: "string"}}
	}
	openProperties := map[string]Schema{"emoji": stringField("Ticket emoji."), "title": stringField("Ticket title."), "prompt": stringField("Task description."), "goal": stringField("Goal id."), "client": stringField("Agent client."), "llm": stringField("Model."), "effort": stringField("Reasoning effort."), "draft": stringField("Draft id."), "parent": stringField("Parent ticket."), "issue": stringField("Existing issue URL."), "no_issue": booleanField("Skip issue creation."), "no_management": booleanField("Skip management integration.")}
	reopenProperties := map[string]Schema{"path": stringField("YY/MM/DD/SLUG path."), "prompt": stringField("Additional task description."), "client": stringField("Agent client."), "llm": stringField("Model."), "effort": stringField("Reasoning effort."), "draft": stringField("Draft id."), "title": stringField("Updated title."), "goal": stringField("Goal id."), "parent": stringField("Parent ticket."), "no_management": booleanField("Skip management integration.")}
	switch resolvedProfile {
	case client.McpClientCursor, client.McpClientCopilot, client.McpClientClaude, client.McpClientCodex:
		openProperties["plan_id"] = stringField("Client plan id.")
		reopenProperties["plan_id"] = stringField("Client plan id.")
	case client.McpClientKiro:
		openProperties["spec_id"] = stringField("Kiro spec id.")
		reopenProperties["spec_id"] = stringField("Kiro spec id.")
	}
	tools := []Tool{
		{Name: "ticket_open", Description: "Open a repository ticket.", InputSchema: object(openProperties, "emoji", "title", "prompt", "goal")},
		{Name: "ticket_close", Description: "Close a repository ticket.", InputSchema: object(map[string]Schema{"path": stringField("YY/MM/DD/SLUG path."), "summary": stringField("Completion summary."), "files": arrayField("Changed files."), "title": stringField("Updated title."), "no_management": booleanField("Skip management integration.")}, "summary")},
		{Name: "ticket_reopen", Description: "Reopen a repository ticket.", InputSchema: object(reopenProperties)},
		{Name: "section_move", Description: "Rename or move a section.", InputSchema: object(map[string]Schema{"file": stringField("Source file."), "old_name": stringField("Current section."), "new_name": stringField("New section.")}, "file", "old_name", "new_name")},
		{Name: "file_integrate", Description: "Integrate a source file into a target section.", InputSchema: object(map[string]Schema{"source": stringField("Source file."), "target_section": stringField("Target section."), "target_file": stringField("Target file."), "target_parent_section": stringField("Optional parent section.")}, "source", "target_section", "target_file")},
		{Name: "section_extract", Description: "Extract a section into a target file.", InputSchema: object(map[string]Schema{"source_file": stringField("Source file."), "source_section": stringField("Source section."), "target_file": stringField("Target file.")}, "source_file", "source_section", "target_file")},
	}
	for _, schema := range tools {
		name := schema.Name
		if err := server.RegisterTool(schema, func(ctx context.Context, params CallToolParams, progress ProgressReporter) (CallToolResult, error) {
			if err := progress.Report(ctx, 0, nil, "started"); err != nil {
				return CallToolResult{}, err
			}
			result, err := repository.Call(ctx, name, params.Arguments)
			if err != nil {
				return CallToolResult{}, err
			}
			if err := progress.Report(ctx, 1, nil, "completed"); err != nil {
				return CallToolResult{}, err
			}
			return CallToolResult{Content: []Content{{Type: "text", Text: result.Text}}, StructuredContent: result.Structured, IsError: result.IsError}, nil
		}); err != nil {
			return nil, err
		}
	}
	resources := []Resource{
		{URI: "repo://", Name: "repo", MIMEType: "text/plain"},
		{URI: "repo://bundles", Name: "bundles", MIMEType: "text/plain"},
		{URI: "repo://folders", Name: "folders", MIMEType: "text/plain"},
		{URI: "repo://files", Name: "files", MIMEType: "text/plain"},
		{URI: "repo://tickets", Name: "tickets", MIMEType: "text/plain"},
		{URI: "repo://goals", Name: "goals", MIMEType: "text/plain"},
		{URI: "repo://policies", Name: "policies", MIMEType: "text/plain"},
		{URI: "repo://contributors", Name: "contributors", MIMEType: "text/plain"},
	}
	for _, schema := range resources {
		uri := schema.URI
		if err := server.RegisterResource(schema, func(ctx context.Context, _ ReadResourceParams, progress ProgressReporter) (ReadResourceResult, error) {
			if err := progress.Report(ctx, 0, nil, "started"); err != nil {
				return ReadResourceResult{}, err
			}
			content, err := repository.Read(ctx, uri)
			if err != nil {
				return ReadResourceResult{}, err
			}
			return ReadResourceResult{Contents: []ResourceContent{content}}, nil
		}); err != nil {
			return nil, err
		}
	}
	for _, name := range []string{"enhance", "refactor", "test", "comply"} {
		promptName := name
		schema := Prompt{Name: promptName, Arguments: []PromptArgument{{Name: "prompt", Required: true}}}
		if err := server.RegisterPrompt(schema, func(ctx context.Context, params GetPromptParams, _ ProgressReporter) (GetPromptResult, error) {
			return repository.Prompt(ctx, promptName, params.Arguments)
		}); err != nil {
			return nil, err
		}
	}
	return server, nil
}

type stdioTransport struct {
	reader io.Reader
	writer io.Writer
}

func (transport stdioTransport) Read(data []byte) (int, error)  { return transport.reader.Read(data) }
func (transport stdioTransport) Write(data []byte) (int, error) { return transport.writer.Write(data) }
func (stdioTransport) Close() error                             { return nil }

// #endregion 🏭️ProductionServer
