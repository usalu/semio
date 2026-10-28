// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🏭️mcp_factory carries the MCP server factory of the cli with the ticket handlers and hook and MCP runners.

// #endregion 🧲️Header

package cli

import (
	context "context"
	json "encoding/json"
	fmt "fmt"
	os "os"
	time "time"
 strings "strings"

	graphqlpkg "github.com/usalu/semio/repo/graphql"
 model "github.com/usalu/semio/repo/model"
	hooks "github.com/usalu/semio/repo/hooks"
	providers "github.com/usalu/semio/repo/providers"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔌️Mcp Surface

// #region 🦀️McpServerFactory

// 🪆️wrapMcpToolHandler enforces a wall-clock budget on each MCP tool invocation.
func wrapMcpToolHandler(timeout time.Duration, handler func(context.Context, CallToolRequest) (*CallToolResult, error)) func(context.Context, CallToolRequest) (*CallToolResult, error) {
	return func(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
		if timeout <= 0 {
			return handler(ctx, request)
		}
		toolCtx, cancel := context.WithTimeout(ctx, timeout)
		defer cancel()
		type outcome struct {
			result *CallToolResult
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

// 🧭️newMcpRepository binds one repository to its executor and context.
func newMcpRepository(rootDir string) (*mcpRepository,error) {
 executor,err:=graphqlpkg.NewExecutor(rootDir)
 if err!=nil { return nil,err }
 return &mcpRepository{root:executor.Repository().GetRootDir(),executor:executor,repo:executor.Repository()},nil
}

// 🦀️CreateMcpServer builds the MCP server for the given IDE kind (stdio).
func CreateMcpServer(rootDir string, kind providers.McpClientKind, toolTimeout time.Duration) (*MCPServer,error) {
 repository,err:=newMcpRepository(rootDir)
 if err!=nil { return nil,err }
	if kind == "" {
		kind = providers.McpClientGeneric
	}
	s := NewMCPServer(
		providers.McpServerName(kind),
		"1.0.0",
		WithToolCapabilities(true),
		WithPromptCapabilities(true),
	)
	s.AddPrompt(
		NewPrompt("enhance",
			WithPromptDescription(mcpDesc(kind, "prompt_enhance")),
			WithArgument("prompt", ArgumentDescription("Context the agent must honor while enhancing."), RequiredArgument()),
		),
		repository.handleEnhancePrompt,
	)
	s.AddPrompt(
		NewPrompt("refactor",
			WithPromptDescription(mcpDesc(kind, "prompt_refactor")),
			WithArgument("prompt", ArgumentDescription("Refactor constraints the agent must honor."), RequiredArgument()),
		),
		repository.handleRefactorPrompt,
	)
	s.AddPrompt(
		NewPrompt("test",
			WithPromptDescription(mcpDesc(kind, "prompt_test")),
			WithArgument("prompt", ArgumentDescription("Coverage targets the agent must honor."), RequiredArgument()),
		),
		repository.handleTestPrompt,
	)
	s.AddPrompt(
		NewPrompt("comply",
			WithPromptDescription(mcpDesc(kind, "prompt_comply")),
			WithArgument("prompt", ArgumentDescription("Compliance constraints the agent must honor."), RequiredArgument()),
		),
		repository.handleComplyPrompt,
	)
	s.AddResource(
		NewResource("repo://", mcpDesc(kind, "res_root"), WithMIMEType("text/plain")),
		repository.handleRepoResource,
	)
	s.AddResource(
		NewResource("repo://bundles", mcpDesc(kind, "res_bundles"), WithMIMEType("text/plain")),
		repository.handleBundlesResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://bundle/{id}", mcpDesc(kind, "res_bundle_one")),
		repository.handleBundleResource,
	)
	s.AddResource(
		NewResource("repo://folders", mcpDesc(kind, "res_folders"), WithMIMEType("application/json")),
		repository.handleFoldersResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://folder/{id}", mcpDesc(kind, "res_folder_one")),
		repository.handleFolderResource,
	)
	s.AddResource(
		NewResource("repo://files", mcpDesc(kind, "res_files"), WithMIMEType("application/json")),
		repository.handleFilesResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://file/{id}", mcpDesc(kind, "res_file_one")),
		repository.handleFileResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://sections/{id}", mcpDesc(kind, "res_sections")),
		repository.handleSectionsResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://section/{id}", mcpDesc(kind, "res_section_one")),
		repository.handleSectionResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://definitions/{id}", mcpDesc(kind, "res_definitions")),
		repository.handleDefinitionsResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://definition/{id}", mcpDesc(kind, "res_definition_one")),
		repository.handleDefinitionResource,
	)
	s.AddResource(
		NewResource("repo://tickets", mcpDesc(kind, "res_tickets"), WithMIMEType("text/plain")),
		repository.handleTicketsResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://ticket/{id}", mcpDesc(kind, "res_ticket_one")),
		repository.handleTicketResource,
	)
	s.AddResource(
		NewResource("repo://goals", mcpDesc(kind, "res_goals"), WithMIMEType("text/plain")),
		repository.handleGoalsResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://goal/{id}", mcpDesc(kind, "res_goal_one")),
		repository.handleGoalResource,
	)
	s.AddResource(
		NewResource("repo://policies", mcpDesc(kind, "res_policies"), WithMIMEType("text/plain")),
		repository.handlePoliciesResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://policy/{id}", mcpDesc(kind, "res_policy_one")),
		repository.handlePolicyResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://statute/{id}", mcpDesc(kind, "res_statute_one")),
		repository.handleStatuteResource,
	)
	s.AddResource(
		NewResource("repo://contributors", mcpDesc(kind, "res_contributors"), WithMIMEType("text/plain")),
		repository.handleContributorsResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://contributor/{id}", mcpDesc(kind, "res_contributor_one")),
		repository.handleContributorResource,
	)
	s.AddResourceTemplate(
		NewResourceTemplate("repo://checkpoint/{id}", mcpDesc(kind, "res_checkpoint_one")),
		repository.handleCheckpointResource,
	)

	openOpts := []ToolOption{
		WithDescription(mcpDesc(kind, "tool_ticket_open")),
		WithString("emoji", Required(), Description("Single emoji representing the ticket (e.g. '🎫️', '🐛️', '✨️').")),
		WithString("title", Required(), Description("Short title for the ticket. Shape is not enforced.")),
		WithString("prompt", Required(), Description("Full description of the task.")),
		WithString("goal", Required(), Description("Goal id to associate the ticket with (from repo://goals).")),
		WithString("client", Description("Agent client used for this ticket.")),
		WithString("llm", Description("LLM used for this ticket.")),
		WithString("effort", Description("Reasoning effort for the LLM (low, medium, high, max).")),
		WithString("draft", Description("Draft slug to seed the ticket workspace.")),
		WithString("parent", Description("Parent ticket slug for nested tickets.")),
		WithString("issue", Description("Existing GitHub issue URL to link instead of creating a new one.")),
 WithBoolean("no_issue",Description("Keep the ticket local without creating an issue.")),
 WithBoolean("no_management",Description("Skip management integration.")),
	}
	switch kind {
	case providers.McpClientCopilot, providers.McpClientClaude, providers.McpClientCodex:
		openOpts = append(openOpts, WithString("plan_id", Description(mcpDesc(kind, "arg_plan_id"))))
	case providers.McpClientKiro:
		openOpts = append(openOpts, WithString("spec_id", Description(mcpDesc(kind, "arg_spec_id"))))
	}
	s.AddTool(NewTool("ticket_open", openOpts...), wrapMcpToolHandler(toolTimeout, repository.newTicketOpenHandler(kind)))

	closeOpts := []ToolOption{
		WithDescription(mcpDesc(kind, "tool_ticket_close")),
		WithString("summary", Required(), Description("Summary of the work done.")),
		WithArray("files", Description("Files created, updated, or removed during the ticket."), WithStringItems()),
		WithString("path", Description("Ticket path as returned by ticket_open (e.g. '26/03/27/FIX-MCP-DESCRIPTIONS'). Omit to close the latest open ticket.")),
		WithString("title", Description("Updated title for the ticket.")),
		WithBoolean("no_management", Description("Skip updating the GitHub issue.")),
	}
	s.AddTool(NewTool("ticket_close", closeOpts...), wrapMcpToolHandler(toolTimeout, repository.newTicketCloseHandler(kind)))

	reopenOpts := []ToolOption{
		WithDescription(mcpDesc(kind, "tool_ticket_reopen")),
		WithString("path", Description("Ticket path as returned by ticket_open (e.g. '26/03/27/FIX-MCP-DESCRIPTIONS'). Omit to reopen the latest closed ticket.")),
		WithString("prompt", Description("Updated or additional task description.")),
		WithString("llm", Description("LLM to use for the reopened ticket.")),
		WithString("effort", Description("Reasoning effort for the LLM (low, medium, high, max).")),
		WithString("client", Description("Agent client to use for the reopened ticket.")),
		WithString("title", Description("Updated title for the ticket.")),
		WithString("draft", Description("Draft slug to seed the ticket workspace.")),
		WithBoolean("no_management", Description("Skip updating the GitHub issue.")),
	}
	switch kind {
	case providers.McpClientCopilot, providers.McpClientClaude, providers.McpClientCodex:
		reopenOpts = append(reopenOpts, WithString("plan_id", Description(mcpDesc(kind, "arg_plan_id"))))
	case providers.McpClientKiro:
		reopenOpts = append(reopenOpts, WithString("spec_id", Description(mcpDesc(kind, "arg_spec_id"))))
	}
	s.AddTool(NewTool("ticket_reopen", reopenOpts...), wrapMcpToolHandler(toolTimeout, repository.newTicketReopenHandler(kind)))

	s.AddTool(
		NewTool("goal_open",
			WithDescription(mcpDesc(kind, "tool_goal_open")),
			WithString("title", Required(), Description("Goal title.")),
			WithString("prompt", Required(), Description("Goal prompt.")),
			WithString("description", Description("Goal description.")),
			WithString("due_date", Description("YYYY-MM-DD due date.")),
			WithString("llm", Description("Model.")),
			WithString("client", Description("Agent client.")),
			WithString("parent", Description("Parent goal id.")),
			WithString("milestone", Description("Management milestone.")),
			WithBoolean("no_management", Description("Skip management integration.")),
		),
		wrapMcpToolHandler(toolTimeout, repository.goalOpen),
	)
	s.AddTool(
		NewTool("goal_close",
			WithDescription(mcpDesc(kind, "tool_goal_close")),
			WithString("id", Required(), Description("Goal id.")),
			WithString("summary", Required(), Description("Completion summary.")),
			WithBoolean("no_management", Description("Skip management integration.")),
		),
		wrapMcpToolHandler(toolTimeout, repository.goalClose),
	)
	s.AddTool(
		NewTool("goal_reopen",
			WithDescription(mcpDesc(kind, "tool_goal_reopen")),
			WithString("id", Required(), Description("Goal id.")),
			WithString("prompt", Required(), Description("Additional goal prompt.")),
			WithString("llm", Required(), Description("Model.")),
			WithString("client", Required(), Description("Agent client.")),
			WithString("title", Description("Updated title.")),
			WithString("description", Description("Updated description.")),
			WithString("due_date", Description("YYYY-MM-DD due date.")),
			WithBoolean("no_management", Description("Skip management integration.")),
		),
		wrapMcpToolHandler(toolTimeout, repository.goalReopen),
	)
	return s,nil
}

// 🏃️RunMcpServerFor starts the MCP stdio server for the given kind.
func RunMcpServerFor(ctx context.Context, rootDir string, kind providers.McpClientKind, toolTimeout time.Duration) error {
 server,err:=CreateMcpServer(rootDir,kind,toolTimeout)
 if err!=nil { return err }
 return ServeStdio(ctx,server)
}

// #endregion 🦀️McpServerFactory

// #region 🪝️TicketMcpHandlers
func (repository *mcpRepository) newTicketOpenHandler(kind providers.McpClientKind) func(context.Context, CallToolRequest) (*CallToolResult, error) {
	return func(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
		return repository.ticketOpenWithKind(ctx, request, kind)
	}
}

func (repository *mcpRepository) newTicketCloseHandler(kind providers.McpClientKind) func(context.Context, CallToolRequest) (*CallToolResult, error) {
	return func(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
		_ = kind
		return repository.ticketClose(ctx, request)
	}
}

func (repository *mcpRepository) newTicketReopenHandler(kind providers.McpClientKind) func(context.Context, CallToolRequest) (*CallToolResult, error) {
	return func(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
		return repository.ticketReopenWithKind(ctx, request, kind)
	}
}

// ▶️RunHookFor runs a hook for the IDE kind (client is implied; stdin is hook JSON payload).
func RunHookFor(kind providers.McpClientKind, eventStr string, stdin []byte) error {
	client := providers.HookClientForMcpKind(kind)
	if client == "" {
		return fmt.Errorf("hooks are not available for mcp kind %q", kind)
	}
	if err := workspace.BindWorkingDirectory(); err != nil {
		return err
	}
	repoRoot := workspace.GetRootDir()
	var input json.RawMessage
	if len(stdin) > 0 {
		input = json.RawMessage(stdin)
	}
	ctx, cancel := context.WithTimeout(context.Background(), DefaultCommandTimeout)
	defer cancel()
	return hooks.RunHookExecutionCtx(ctx, client, eventStr, "", "", "", "", repoRoot, input, false, os.Stdout, os.Stderr)
}

// 🚀️RunMCPFor starts the MCP stdio server for the given IDE kind.
func RunMCPFor(ctx context.Context, rootDir string, kind providers.McpClientKind) error {
	return RunMcpServerFor(ctx,rootDir,kind,DefaultCommandTimeout)
}

// #endregion 🪝️TicketMcpHandlers

// #endregion 🔌️Mcp Surface

// 🧭️mcpRepository owns the immutable repository binding of one server.
type mcpRepository struct { root string; executor *graphqlpkg.Executor; repo model.RepoContext }

// 📨️query executes only against this server's repository context.
func (repository *mcpRepository) query(ctx context.Context, query string, variables map[string]interface{}) (string,error) { return repository.executor.ExecuteJSON(ctx,query,variables) }

// 🪪️artifactPath resolves only identities present in this repository.
func (repository *mcpRepository) artifactPath(id string) string {
 normalize:=func(value string) string { return strings.NewReplacer("\uFE0E","","\uFE0F","").Replace(value) }
 wanted:=normalize(id)
 for _, file:=range repository.repo.GetFiles() {
  candidate:=normalize(file.ID)
  if wanted==candidate || strings.HasPrefix(wanted,candidate+normalize(model.EmojiSection)) || strings.HasPrefix(wanted,candidate+normalize(model.EmojiDefinitionImpl)) || strings.HasPrefix(wanted,candidate+normalize(model.EmojiDefinitionInterface)) || strings.HasPrefix(wanted,candidate+normalize(model.EmojiDefinitionConstant)) || strings.HasPrefix(wanted,candidate+normalize(model.EmojiDefinitionTest)) { return file.Path }
 }
 for _, folder:=range repository.repo.GetFolders() { if wanted==normalize(folder.ID) { return folder.Path } }
 return ""
}
