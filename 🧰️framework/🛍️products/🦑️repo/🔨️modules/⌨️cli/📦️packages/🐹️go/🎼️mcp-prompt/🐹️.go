// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎼️mcp_prompt carries the MCP prompt template rendering and prompt handlers of the cli.

// #endregion 🧲️Header

package cli

import (
	context "context"
	os "os"
	filepath "path/filepath"
	strings "strings"
	template "text/template"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔌️Mcp Surface

// #region 🪄️Handlers

// 🌐️Request handler functions for CLI and MCP operations.
// ⏹️renderPromptTemplate holds the data fields for a renderPromptTemplate record.
func (repository *mcpRepository) renderPromptTemplate(name string, data map[string]string) (string, error) {
	path := workspace.RepoMetaPathForRoot(repository.root,filepath.Join("💬️prompts", name+".tpl"))
	content, err := os.ReadFile(path)
	if err != nil {
		return "", err
	}
	tmpl, err := template.New(name).Funcs(model.TxtFuncMap()).Parse(string(content))
	if err != nil {
		return "", err
	}
	var out strings.Builder
	if err := tmpl.Execute(&out, data); err != nil {
		return "", err
	}
	return out.String(), nil
}

// 💿️handleEnhancePrompt holds the data fields for a handleEnhancePrompt record.
func (repository *mcpRepository) handleEnhancePrompt(ctx context.Context, request GetPromptRequest) (*GetPromptResult, error) {
	prompt := request.Params.Arguments["prompt"]
	content, err := repository.renderPromptTemplate("enhance", map[string]string{"prompt": prompt})
	if err != nil {
		return nil, err
	}
	return NewGetPromptResult(
		"Enhance the implementation by adding more features and enhance the existing tests to cover the new features.",
		[]PromptMessage{
			NewPromptMessage(RoleUser, NewTextContent(content)),
		},
	), nil
}

func (repository *mcpRepository) handleRefactorPrompt(ctx context.Context, request GetPromptRequest) (*GetPromptResult, error) {
	prompt := request.Params.Arguments["prompt"]
	content, err := repository.renderPromptTemplate("refactor", map[string]string{"prompt": prompt})
	if err != nil {
		return nil, err
	}
	return NewGetPromptResult(
		"Refactor the implementation and dont stop until all tests pass.",
		[]PromptMessage{
			NewPromptMessage(RoleUser, NewTextContent(content)),
		},
	), nil
}

// 🧪️handleTestPrompt holds the data fields for a handleTestPrompt record.
func (repository *mcpRepository) handleTestPrompt(ctx context.Context, request GetPromptRequest) (*GetPromptResult, error) {
	prompt := request.Params.Arguments["prompt"]
	content, err := repository.renderPromptTemplate("test", map[string]string{"prompt": prompt})
	if err != nil {
		return nil, err
	}
	return NewGetPromptResult(
		"Extend the current tests by testing more features.",
		[]PromptMessage{
			NewPromptMessage(RoleUser, NewTextContent(content)),
		},
	), nil
}

// 🔷️handleComplyPrompt holds the data fields for a handleComplyPrompt record.
func (repository *mcpRepository) handleComplyPrompt(ctx context.Context, request GetPromptRequest) (*GetPromptResult, error) {
	prompt := request.Params.Arguments["prompt"]
	content, err := repository.renderPromptTemplate("comply", map[string]string{"prompt": prompt})
	if err != nil {
		return nil, err
	}
	return NewGetPromptResult(
		"Get the implementation to comply the a set of tests. Dont remove any functionality from the tests.",
		[]PromptMessage{
			NewPromptMessage(RoleUser, NewTextContent(content)),
		},
	), nil
}

// #endregion 🪄️Handlers

// #endregion 🔌️Mcp Surface
