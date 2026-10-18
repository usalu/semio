// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// MCP server factory and hook entry points.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"fmt"
	mcp "github.com/usalu/semio/repo/client/internal/mcp"
	server "github.com/usalu/semio/repo/client/internal/mcpserver"
	"os"
	"time"
)

func wrapMcpToolHandler(timeout time.Duration, handler func(context.Context, mcp.CallToolRequest) (*mcp.CallToolResult, error)) func(context.Context, mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	return func(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
		if timeout <= 0 {
			return handler(ctx, request)
		}
		toolCtx, cancel := context.WithTimeout(ctx, timeout)
		defer cancel()
		type outcome struct {
			result *mcp.CallToolResult
			err    error
		}
		ch := make(chan outcome, 1)
		go func() {
			r, e := handler(toolCtx, request)
			ch <- outcome{r, e}
		}()
		select {
		case o := <-ch:
			return o.result, o.err
		case <-toolCtx.Done():
			return nil, fmt.Errorf("tool call timed out after %s: %w", timeout, toolCtx.Err())
		}
	}
}

// 🦀️CreateMcpServer builds the MCP server for the given IDE kind (stdio).
func CreateMcpServer(kind McpClientKind, toolTimeout time.Duration) *server.MCPServer {
	if kind == "" {
		kind = McpClientGeneric
	}
	s := server.NewMCPServer(
		McpServerName(kind),
		"1.0.0",
		server.WithToolCapabilities(true),
		server.WithPromptCapabilities(true),
	)
	s.AddPrompt(
		mcp.NewPrompt("enhance",
			mcp.WithPromptDescription(mcpDesc(kind, "prompt_enhance")),
			mcp.WithArgument("prompt", mcp.ArgumentDescription("Context the agent must honor while enhancing."), mcp.RequiredArgument()),
		),
		handleEnhancePrompt,
	)
	s.AddPrompt(
		mcp.NewPrompt("refactor",
			mcp.WithPromptDescription(mcpDesc(kind, "prompt_refactor")),
			mcp.WithArgument("prompt", mcp.ArgumentDescription("Refactor constraints the agent must honor."), mcp.RequiredArgument()),
		),
		handleRefactorPrompt,
	)
	s.AddPrompt(
		mcp.NewPrompt("test",
			mcp.WithPromptDescription(mcpDesc(kind, "prompt_test")),
			mcp.WithArgument("prompt", mcp.ArgumentDescription("Coverage targets the agent must honor."), mcp.RequiredArgument()),
		),
		handleTestPrompt,
	)
	s.AddPrompt(
		mcp.NewPrompt("comply",
			mcp.WithPromptDescription(mcpDesc(kind, "prompt_comply")),
			mcp.WithArgument("prompt", mcp.ArgumentDescription("Compliance constraints the agent must honor."), mcp.RequiredArgument()),
		),
		handleComplyPrompt,
	)
	s.AddResource(
		mcp.NewResource("repo://", mcpDesc(kind, "res_root"), mcp.WithMIMEType("text/plain")),
		handleRepoResource,
	)
	s.AddResource(
		mcp.NewResource("repo://bundles", mcpDesc(kind, "res_bundles"), mcp.WithMIMEType("text/plain")),
		handleBundlesResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://bundle/{id}", mcpDesc(kind, "res_bundle_one")),
		handleBundleResource,
	)
	s.AddResource(
		mcp.NewResource("repo://folders", mcpDesc(kind, "res_folders"), mcp.WithMIMEType("text/plain")),
		handleFoldersResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://folder/{id}", mcpDesc(kind, "res_folder_one")),
		handleFolderResource,
	)
	s.AddResource(
		mcp.NewResource("repo://files", mcpDesc(kind, "res_files"), mcp.WithMIMEType("text/plain")),
		handleFilesResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://file/{id}", mcpDesc(kind, "res_file_one")),
		handleFileResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://sections/{id}", mcpDesc(kind, "res_sections")),
		handleSectionsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://section/{id}", mcpDesc(kind, "res_section_one")),
		handleSectionResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://definitions/{id}", mcpDesc(kind, "res_definitions")),
		handleDefinitionsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://definition/{id}", mcpDesc(kind, "res_definition_one")),
		handleDefinitionResource,
	)
	s.AddResource(
		mcp.NewResource("repo://tickets", mcpDesc(kind, "res_tickets"), mcp.WithMIMEType("text/plain")),
		handleTicketsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://ticket/{id}", mcpDesc(kind, "res_ticket_one")),
		handleTicketResource,
	)
	s.AddResource(
		mcp.NewResource("repo://goals", mcpDesc(kind, "res_goals"), mcp.WithMIMEType("text/plain")),
		handleGoalsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://goal/{id}", mcpDesc(kind, "res_goal_one")),
		handleGoalResource,
	)
	s.AddResource(
		mcp.NewResource("repo://policies", mcpDesc(kind, "res_policies"), mcp.WithMIMEType("text/plain")),
		handlePoliciesResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://policy/{id}", mcpDesc(kind, "res_policy_one")),
		handlePolicyResource,
	)
	s.AddResource(
		mcp.NewResource("repo://statutes", mcpDesc(kind, "res_statutes"), mcp.WithMIMEType("text/plain")),
		handleStatutesResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://statute/{id}", mcpDesc(kind, "res_statute_one")),
		handleStatuteResource,
	)
	s.AddResource(
		mcp.NewResource("repo://contributors", mcpDesc(kind, "res_contributors"), mcp.WithMIMEType("text/plain")),
		handleContributorsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://contributor/{id}", mcpDesc(kind, "res_contributor_one")),
		handleContributorResource,
	)
	s.AddResource(
		mcp.NewResource("repo://checkpoints", mcpDesc(kind, "res_checkpoints"), mcp.WithMIMEType("text/plain")),
		handleCheckpointsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://checkpoint/{id}", mcpDesc(kind, "res_checkpoint_one")),
		handleCheckpointResource,
	)

	openOpts := []mcp.ToolOption{
		mcp.WithDescription(mcpDesc(kind, "tool_ticket_open")),
		mcp.WithString("emoji", mcp.Required(), mcp.Description("Single emoji representing the ticket (e.g. '🎫️', '🐛️', '✨️').")),
		mcp.WithString("title", mcp.Required(), mcp.Description("Short title for the ticket. Shape is not enforced.")),
		mcp.WithString("prompt", mcp.Required(), mcp.Description("Full description of the task.")),
		mcp.WithString("goal", mcp.Required(), mcp.Description("Goal id to associate the ticket with (from repo://goals).")),
		mcp.WithString("client", mcp.Description("Agent client used for this ticket.")),
		mcp.WithString("llm", mcp.Description("LLM used for this ticket.")),
		mcp.WithString("effort", mcp.Description("Reasoning effort for the LLM (low, medium, high, max).")),
		mcp.WithString("draft", mcp.Description("Draft slug to seed the ticket workspace.")),
		mcp.WithString("parent", mcp.Description("Parent ticket slug for nested tickets.")),
		mcp.WithString("issue", mcp.Description("Existing GitHub issue URL to link instead of creating a new one.")),
	}
	switch kind {
	case McpClientCursor, McpClientCopilot, McpClientClaude, McpClientCodex:
		openOpts = append(openOpts, mcp.WithString("plan_id", mcp.Description(mcpDesc(kind, "arg_plan_id"))))
	case McpClientKiro:
		openOpts = append(openOpts, mcp.WithString("spec_id", mcp.Description(mcpDesc(kind, "arg_spec_id"))))
	}
	s.AddTool(mcp.NewTool("ticket_open", openOpts...), wrapMcpToolHandler(toolTimeout, newTicketOpenHandler(kind)))

	closeOpts := []mcp.ToolOption{
		mcp.WithDescription(mcpDesc(kind, "tool_ticket_close")),
		mcp.WithString("summary", mcp.Required(), mcp.Description("Summary of the work done.")),
		mcp.WithArray("files", mcp.Description("Files created, updated, or removed during the ticket."), mcp.WithStringItems()),
		mcp.WithString("path", mcp.Description("Ticket path as returned by ticket_open (e.g. '26/03/27/FIX-MCP-DESCRIPTIONS'). Omit to close the latest open ticket.")),
		mcp.WithString("title", mcp.Description("Updated title for the ticket.")),
		mcp.WithBoolean("no_management", mcp.Description("Skip updating the GitHub issue.")),
	}
	s.AddTool(mcp.NewTool("ticket_close", closeOpts...), wrapMcpToolHandler(toolTimeout, newTicketCloseHandler(kind)))

	reopenOpts := []mcp.ToolOption{
		mcp.WithDescription(mcpDesc(kind, "tool_ticket_reopen")),
		mcp.WithString("path", mcp.Description("Ticket path as returned by ticket_open (e.g. '26/03/27/FIX-MCP-DESCRIPTIONS'). Omit to reopen the latest closed ticket.")),
		mcp.WithString("prompt", mcp.Description("Updated or additional task description.")),
		mcp.WithString("llm", mcp.Description("LLM to use for the reopened ticket.")),
		mcp.WithString("effort", mcp.Description("Reasoning effort for the LLM (low, medium, high, max).")),
		mcp.WithString("client", mcp.Description("Agent client to use for the reopened ticket.")),
		mcp.WithString("title", mcp.Description("Updated title for the ticket.")),
		mcp.WithString("draft", mcp.Description("Draft slug to seed the ticket workspace.")),
		mcp.WithBoolean("no_management", mcp.Description("Skip updating the GitHub issue.")),
	}
	switch kind {
	case McpClientCursor, McpClientCopilot, McpClientClaude, McpClientCodex:
		reopenOpts = append(reopenOpts, mcp.WithString("plan_id", mcp.Description(mcpDesc(kind, "arg_plan_id"))))
	case McpClientKiro:
		reopenOpts = append(reopenOpts, mcp.WithString("spec_id", mcp.Description(mcpDesc(kind, "arg_spec_id"))))
	}
	s.AddTool(mcp.NewTool("ticket_reopen", reopenOpts...), wrapMcpToolHandler(toolTimeout, newTicketReopenHandler(kind)))

	s.AddTool(
		mcp.NewTool("section_move",
			mcp.WithDescription(mcpDesc(kind, "tool_section_move")),
			mcp.WithString("file", mcp.Required(), mcp.Description("Path to the file containing the section.")),
			mcp.WithString("old_name", mcp.Required(), mcp.Description("Current name of the section.")),
			mcp.WithString("new_name", mcp.Required(), mcp.Description("New name for the section.")),
		),
		wrapMcpToolHandler(toolTimeout, sectionMove),
	)
	s.AddTool(
		mcp.NewTool("file_integrate",
			mcp.WithDescription(mcpDesc(kind, "tool_file_integrate")),
			mcp.WithString("source", mcp.Required(), mcp.Description("Path to the source file.")),
			mcp.WithString("target_section", mcp.Required(), mcp.Description("Name of the section in the target file to integrate into.")),
			mcp.WithString("target_file", mcp.Required(), mcp.Description("Path to the target file.")),
			mcp.WithString("target_parent_section", mcp.Description("Name of the parent section in the target file.")),
		),
		wrapMcpToolHandler(toolTimeout, sectionIntegrate),
	)
	s.AddTool(
		mcp.NewTool("section_extract",
			mcp.WithDescription(mcpDesc(kind, "tool_section_extract")),
			mcp.WithString("source_file", mcp.Required(), mcp.Description("Path to the source file.")),
			mcp.WithString("source_section", mcp.Required(), mcp.Description("Name of the section to extract.")),
			mcp.WithString("target_file", mcp.Required(), mcp.Description("Path to the target file where the section will be written.")),
		),
		wrapMcpToolHandler(toolTimeout, sectionExtract),
	)
	return s
}

// RunMcpServerFor starts the MCP stdio server for the given kind.
func RunMcpServerFor(kind McpClientKind, toolTimeout time.Duration) error {
	s := CreateMcpServer(kind, toolTimeout)
	return server.ServeStdio(s)
}

func newTicketOpenHandler(kind McpClientKind) func(context.Context, mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	return func(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
		return ticketOpenWithKind(ctx, request, kind)
	}
}

func newTicketCloseHandler(kind McpClientKind) func(context.Context, mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	return func(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
		_ = kind
		return ticketClose(ctx, request)
	}
}

func newTicketReopenHandler(kind McpClientKind) func(context.Context, mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	return func(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
		return ticketReopenWithKind(ctx, request, kind)
	}
}

// RunHookFor runs a hook for the IDE kind (client is implied; stdin is hook JSON payload).
func RunHookFor(kind McpClientKind, eventStr string, stdin []byte) error {
	client := HookClientForMcpKind(kind)
	if client == "" {
		return fmt.Errorf("hooks are not available for mcp kind %q", kind)
	}
	cwd, err := os.Getwd()
	if err != nil {
		return err
	}
	repoRoot := findRepoRoot(cwd)
	SetRootDir(repoRoot)
	var input json.RawMessage
	if len(stdin) > 0 {
		input = json.RawMessage(stdin)
	}
	ctx, cancel := context.WithTimeout(context.Background(), DefaultCommandTimeout)
	defer cancel()
	return runHookExecutionCtx(ctx, client, eventStr, "", "", "", "", repoRoot, input, false, os.Stdout, os.Stderr)
}

// RunMCPFor starts the MCP stdio server for the given IDE kind.
func RunMCPFor(kind McpClientKind) error {
	return RunMcpServerFor(kind, DefaultCommandTimeout)
}
