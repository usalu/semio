// 🐹️ Go side of the todo scanning case.
package adapter

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	model "github.com/usalu/semio/repo/model"
	todos "github.com/usalu/semio/repo/todos"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type scanTreeFile struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

type scanTreeVectors struct {
	Schema      string         `json:"schema"`
	Files       []scanTreeFile `json:"files"`
	SearchTerms []string       `json:"searchTerms"`
}

func loadScanTree(ctx *host.Context) (scanTreeVectors, error) {
	data, err := ctx.FixtureBytes("shared://🔍️scan-tree.json")
	if err != nil {
		return scanTreeVectors{}, err
	}
	var file scanTreeVectors
	err = json.Unmarshal(data, &file)
	return file, err
}

func seeds(file scanTreeVectors) []todos.TreeFile {
	seeded := make([]todos.TreeFile, 0, len(file.Files))
	for _, entry := range file.Files {
		seeded = append(seeded, todos.TreeFile{Path: entry.Path, Content: entry.Content})
	}
	return seeded
}

func renderTodo(todo *model.Todo) string {
	location := "-"
	if todo.Location != nil {
		location = fmt.Sprintf("%s:%d:%d", todo.Location.FilePath, todo.Location.Line, todo.Location.Column)
	}
	return fmt.Sprintf("%s|%s|%s|%s|%s", todo.ID, todo.Name, todo.Description, todo.ParentID, location)
}

func renderWalk(tree todos.TodoTree) []string {
	entries := tree.Walk()
	walk := make([]string, 0, len(entries))
	for _, entry := range entries {
		walk = append(walk, fmt.Sprintf("%s|%t", entry.Path, entry.IsDir))
	}
	return walk
}

func renderTodos(found []*model.Todo) []string {
	rendered := make([]string, 0, len(found))
	for _, todo := range found {
		rendered = append(rendered, renderTodo(todo))
	}
	return rendered
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func everyTodoInTheTreeIsFound(ctx *host.Context) (host.Outcome, error) {
	file, err := loadScanTree(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	tree := todos.NewMemoryTodoTree(seeds(file))
	searches := make([]string, 0, len(file.SearchTerms))
	for _, term := range file.SearchTerms {
		names := make([]string, 0)
		for _, todo := range todos.SearchTodos(tree, term) {
			names = append(names, todo.Name)
		}
		searches = append(searches, fmt.Sprintf("%s=%s", term, strings.Join(names, ",")))
	}
	return host.Outcome{Projection: map[string]any{
		"todos":    renderTodos(todos.ScanTodos(tree)),
		"searches": searches,
		"walk":     renderWalk(tree),
	}}, nil
}

func theScanRefusesThreeKindsOfDirectory(ctx *host.Context) (host.Outcome, error) {
	file, err := loadScanTree(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	root := filepath.Join(ctx.WorkDir, "scan-tree")
	for _, entry := range file.Files {
		target := filepath.Join(append([]string{root}, strings.Split(entry.Path, "/")...)...)
		if err := os.MkdirAll(filepath.Dir(target), 0o755); err != nil {
			return host.Outcome{}, err
		}
		if err := os.WriteFile(target, []byte(entry.Content), 0o644); err != nil {
			return host.Outcome{}, err
		}
	}
	tree := todos.NewFsTodoTree(root)
	return host.Outcome{Projection: map[string]any{
		"walk":  renderWalk(tree),
		"todos": renderTodos(todos.ScanTodos(tree)),
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("every-todo-in-the-tree-is-found", everyTodoInTheTreeIsFound).
		Subject("the-scan-refuses-three-kinds-of-directory", theScanRefusesThreeKindsOfDirectory)
}

// endregion 🔖️Registration
