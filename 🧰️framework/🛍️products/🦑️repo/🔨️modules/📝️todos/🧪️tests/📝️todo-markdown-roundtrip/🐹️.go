// 🐹️ Go side of the todo line grammar case.
package adapter

import (
	"encoding/json"
	"fmt"
	"strings"

	model "github.com/usalu/semio/repo/model"
	todos "github.com/usalu/semio/repo/todos"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type markdownRewrite struct {
	OldName        string `json:"oldName"`
	NewName        string `json:"newName"`
	NewDescription string `json:"newDescription"`
}

type sourceRewrite struct {
	Line           int64  `json:"line"`
	NewName        string `json:"newName"`
	NewDescription string `json:"newDescription"`
}

type lineVectors struct {
	Schema           string            `json:"schema"`
	Markdown         string            `json:"markdown"`
	MarkdownPath     string            `json:"markdownPath"`
	Source           string            `json:"source"`
	SourcePath       string            `json:"sourcePath"`
	MarkdownRewrites []markdownRewrite `json:"markdownRewrites"`
	SourceRewrites   []sourceRewrite   `json:"sourceRewrites"`
	OpenerPaths      []string          `json:"openerPaths"`
}

func renderLineTodo(todo *model.Todo) string {
	location := "-"
	if todo.Location != nil {
		location = fmt.Sprintf("%s:%d:%d", todo.Location.FilePath, todo.Location.Line, todo.Location.Column)
	}
	return fmt.Sprintf("%s|%s|%s|%s", todo.Name, todo.Description, todo.ParentID, location)
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func rewritingALineAndReadingItBackAgrees(ctx *host.Context) (host.Outcome, error) {
	data, err := ctx.FixtureBytes("shared://📝️line-vectors.json")
	if err != nil {
		return host.Outcome{}, err
	}
	var vectors lineVectors
	if err := json.Unmarshal(data, &vectors); err != nil {
		return host.Outcome{}, err
	}
	parent := ""
	if index := strings.LastIndex(vectors.MarkdownPath, "/"); index != -1 {
		parent = vectors.MarkdownPath[:index]
	}
	names := func(document string, inMarkdown bool) string {
		var parsed []*model.Todo
		if inMarkdown {
			parsed = todos.ParseTodoMarkdown(document, parent)
		} else {
			parsed = todos.ParseTodoComments(document, vectors.SourcePath)
		}
		rendered := make([]string, 0, len(parsed))
		for _, todo := range parsed {
			rendered = append(rendered, fmt.Sprintf("%s:%s", todo.Name, todo.Description))
		}
		return strings.Join(rendered, ",")
	}
	render := func(parsed []*model.Todo) []string {
		rendered := make([]string, 0, len(parsed))
		for _, todo := range parsed {
			rendered = append(rendered, renderLineTodo(todo))
		}
		return rendered
	}

	markdownRewrites := make([]string, 0, len(vectors.MarkdownRewrites))
	markdownReparsed := make([]string, 0, len(vectors.MarkdownRewrites))
	markdownRemovals := make([]string, 0, len(vectors.MarkdownRewrites))
	for _, rewrite := range vectors.MarkdownRewrites {
		document, err := todos.ReplaceInMarkdown(vectors.Markdown, rewrite.OldName, rewrite.NewName, rewrite.NewDescription)
		if err != nil {
			markdownRewrites = append(markdownRewrites, fmt.Sprintf("err:%s", todos.ErrorClass(err)))
			markdownReparsed = append(markdownReparsed, "-")
		} else {
			markdownRewrites = append(markdownRewrites, fmt.Sprintf("ok:%s", document))
			markdownReparsed = append(markdownReparsed, names(document, true))
		}
		markdownRemovals = append(markdownRemovals, todos.RemoveFromMarkdown(vectors.Markdown, rewrite.OldName))
	}

	sourceRewrites := make([]string, 0, len(vectors.SourceRewrites))
	sourceReparsed := make([]string, 0, len(vectors.SourceRewrites))
	sourceRemovals := make([]string, 0, len(vectors.SourceRewrites))
	for _, rewrite := range vectors.SourceRewrites {
		document, err := todos.ReplaceInFile(vectors.Source, rewrite.Line, rewrite.NewName, rewrite.NewDescription)
		if err != nil {
			sourceRewrites = append(sourceRewrites, fmt.Sprintf("err:%s", todos.ErrorClass(err)))
			sourceReparsed = append(sourceReparsed, "-")
		} else {
			sourceRewrites = append(sourceRewrites, fmt.Sprintf("ok:%s", document))
			sourceReparsed = append(sourceReparsed, names(document, false))
		}
		sourceRemovals = append(sourceRemovals, todos.RemoveFromFile(vectors.Source, rewrite.Line))
	}

	lines := strings.Split(vectors.Source, "\n")
	parts := make([]string, 0, len(lines))
	for _, line := range lines {
		prefix, name, description, ok := todos.SplitTodoCommentParts(line)
		if !ok {
			parts = append(parts, "-")
			continue
		}
		parts = append(parts, fmt.Sprintf("%s|%s|%s", prefix, name, description))
	}

	openers := make([]string, 0, len(vectors.OpenerPaths))
	for _, path := range vectors.OpenerPaths {
		openers = append(openers, fmt.Sprintf("%s=%s", path, todos.TodoCommentOpener(path)))
	}

	return host.Outcome{Projection: map[string]any{
		"markdownParse":    render(todos.ParseTodoMarkdown(vectors.Markdown, parent)),
		"sourceParse":      render(todos.ParseTodoComments(vectors.Source, vectors.SourcePath)),
		"markdownRewrites": markdownRewrites,
		"markdownReparsed": markdownReparsed,
		"sourceRewrites":   sourceRewrites,
		"sourceReparsed":   sourceReparsed,
		"markdownRemovals": markdownRemovals,
		"sourceRemovals":   sourceRemovals,
		"parts":            parts,
		"openers":          openers,
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("rewriting-a-line-and-reading-it-back-agrees", rewritingALineAndReadingItBackAgrees)
}

// endregion 🔖️Registration
