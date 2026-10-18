// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Todo tracking.

// #endregion 🧲️Header

package client

import (
	"fmt"
	repopkg "github.com/usalu/semio/repo/go"
	"io/fs"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

// 📨️GetTodos MUST retrieve the requested value or return an error.
// 📖️GetTodos retrieves and returns the todos.
func (c *repoContext) GetTodos(filter *FilterInput) ([]*Todo, error) {
	allTodos, err := ScanTodos(c.rootDir)
	if err != nil {
		return nil, err
	}
	if filter != nil && filter.Filter != nil {
		search := strings.ToLower(*filter.Filter)
		var match []*Todo
		for _, t := range allTodos {
			if strings.Contains(strings.ToLower(t.Name), search) || strings.Contains(strings.ToLower(t.Description), search) {
				match = append(match, t)
			}
		}
		return match, nil
	}
	return allTodos, nil
}

// 🎯️ScanTodos MUST scan the input completely and collect all matches.
// 🔬️ScanTodos scans and collects todos from the input.
func ScanTodos(rootDir string) ([]*Todo, error) {
	var todos []*Todo
	err := filepath.WalkDir(rootDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if d.IsDir() {
			if strings.HasPrefix(d.Name(), ".") && d.Name() != "." && d.Name() != ".🧬semio" {
				return fs.SkipDir
			}
			if d.Name() == "node_modules" || d.Name() == "dist" || d.Name() == "build" {
				return fs.SkipDir
			}

			todoPath := filepath.Join(path, ".todos.md")
			if _, err := os.Stat(todoPath); err == nil {
				content, _ := os.ReadFile(todoPath)
				todos = append(todos, ParseTodoMarkdown(string(content), path)...)
			}
		} else {
			ext := filepath.Ext(path)

			switch ext {
			case ".ts", ".js", ".tsx", ".jsx", ".go", ".cs", ".py", ".md", ".json":
				content, _ := os.ReadFile(path)
				todos = append(todos, ParseTodoComments(string(content), path)...)
			}
		}
		return nil
	})
	return todos, err
}

// 📰️ParseTodoMarkdown MUST return the parsed result or an error for invalid input.
// 📰️ParseTodoMarkdown parses and returns the todo markdown from the input.
func ParseTodoMarkdown(content string, parentPath string) []*Todo {
	var todos []*Todo
	lines := strings.Split(content, "\n")
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "- TODO ") {
			rem := strings.TrimPrefix(trimmed, "- TODO ")
			parts := strings.SplitN(rem, ":", 2)
			name := strings.TrimSpace(parts[0])
			desc := ""
			if len(parts) > 1 {
				desc = strings.TrimSpace(parts[1])
			}
			id := Slugify(name)
			todos = append(todos, &Todo{
				ID:          id,
				Name:        name,
				Description: desc,
				ParentID:    parentPath,
				Location:    &Location{FilePath: filepath.Join(parentPath, ".todos.md")},
			})
		}
	}
	return todos
}

// 💬️ParseTodoComments MUST return the parsed result or an error for invalid input.
// 💾️ParseTodoComments parses and returns the todo comments from the input.
func ParseTodoComments(content string, filePath string) []*Todo {
	var todos []*Todo
	lines := strings.Split(content, "\n")

	re := regexp.MustCompile(`^\s*(//|#|--)\s*TODO\s+([^:]+):\s*(.*)$`)
	for i, line := range lines {
		matches := re.FindStringSubmatch(line)
		if len(matches) > 3 {
			name := strings.TrimSpace(matches[2])
			desc := strings.TrimSpace(matches[3])
			id := Slugify(name)
			todos = append(todos, &Todo{
				ID:          id,
				Name:        name,
				Description: desc,
				ParentID:    filePath,
				Location:    &Location{FilePath: filePath, Line: i + 1, Column: 1},
			})
		}
	}
	return todos
}

// 🆕️TodoCreate MUST return a non-nil error when the operation fails.
// 🆕️TodoCreate performs the todo create operation on the repo context.
func (c *repoContext) TodoCreate(input TodoCreateInput) (*Todo, error) {

	info, err := os.Stat(input.ParentID)
	if err == nil && info.IsDir() {
		todoPath := filepath.Join(input.ParentID, ".todos.md")
		line := fmt.Sprintf("- TODO %s: %s\n", input.Name, input.Description)
		f, err := os.OpenFile(todoPath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
		if err != nil {
			return nil, err
		}
		defer f.Close()
		if _, err := f.WriteString(line); err != nil {
			return nil, err
		}
		todo := &Todo{ID: Slugify(input.Name), Name: input.Name, Description: input.Description, ParentID: input.ParentID}
		repopkg.Emit(repopkg.EventTodoCreateEnded, "repo-cli", repopkg.TodoCreatePayload{
			TodoPayload: repopkg.TodoPayload{ID: todo.ID, ParentID: input.ParentID, Name: input.Name, Author: GetGitAuthorAlias()},
		})
		return todo, nil
	}

	if err == nil && !info.IsDir() {
		ext := filepath.Ext(input.ParentID)
		prefix := "//"
		if ext == ".py" || ext == ".sh" || ext == ".yaml" || ext == ".yml" {
			prefix = "#"
		} else if ext == ".sql" || ext == ".lua" {
			prefix = "--"
		}
		line := fmt.Sprintf("\n%s TODO %s: %s\n", prefix, input.Name, input.Description)
		f, err := os.OpenFile(input.ParentID, os.O_APPEND|os.O_WRONLY, 0644)
		if err != nil {
			return nil, err
		}
		defer f.Close()
		if _, err := f.WriteString(line); err != nil {
			return nil, err
		}
		todo := &Todo{ID: Slugify(input.Name), Name: input.Name, Description: input.Description, ParentID: input.ParentID, Location: &Location{FilePath: input.ParentID}}
		repopkg.Emit(repopkg.EventTodoCreateEnded, "repo-cli", repopkg.TodoCreatePayload{
			TodoPayload: repopkg.TodoPayload{ID: todo.ID, ParentID: input.ParentID, Name: input.Name, Author: GetGitAuthorAlias()},
		})
		return todo, nil
	}
	return nil, fmt.Errorf("invalid parent id (must be path to folder or file)")
}

// ♻️TodoChange MUST return a non-nil error when the operation fails.
// ✏️TodoChange performs the todo change operation on the repo context.
func (c *repoContext) TodoChange(input TodoChangeInput) (*Todo, error) {
	todos, err := ScanTodos(c.rootDir)
	if err != nil {
		return nil, err
	}
	var todo *Todo
	for _, t := range todos {
		if t.ID == input.ID {
			todo = t
			break
		}
	}
	if todo == nil {
		return nil, fmt.Errorf("todo not found")
	}
	if todo.Location == nil {
		return nil, fmt.Errorf("todo has no location")
	}
	name := todo.Name
	if input.Name != nil {
		name = *input.Name
	}
	description := todo.Description
	if input.Description != nil {
		description = *input.Description
	}
	if strings.HasSuffix(todo.Location.FilePath, ".todos.md") {
		if err := replaceLineInMarkdown(todo.Location.FilePath, todo.Name, name, description); err != nil {
			return nil, err
		}
	} else if todo.Location.Line > 0 {
		if err := replaceLineInFile(todo.Location.FilePath, todo.Location.Line, name, description); err != nil {
			return nil, err
		}
	} else {
		return nil, fmt.Errorf("unable to locate todo source line")
	}
	updated := &Todo{ID: Slugify(name), Name: name, Description: description, ParentID: todo.ParentID, Location: todo.Location}
	repopkg.Emit(repopkg.EventTodoChangeEnded, "repo-cli", repopkg.TodoChangePayload{
		TodoPayload: repopkg.TodoPayload{ID: updated.ID, ParentID: todo.ParentID, Name: name, Author: GetGitAuthorAlias()},
		Name:        input.Name, Description: input.Description,
	})
	return updated, nil
}

// 🗑️TodoDelete MUST return a non-nil error when the operation fails.
// 🗑️TodoDelete performs the todo delete operation on the repo context.
func (c *repoContext) TodoDelete(id string) (bool, error) {
	todos, err := ScanTodos(c.rootDir)
	if err != nil {
		return false, err
	}
	for _, t := range todos {
		if t.ID == id {
			if strings.HasSuffix(t.Location.FilePath, ".todos.md") {
				removeLineFromMarkdown(t.Location.FilePath, t.Name)
				repopkg.Emit(repopkg.EventTodoDeleteEnded, "repo-cli", repopkg.TodoDeletePayload{
					TodoPayload: repopkg.TodoPayload{ID: t.ID, ParentID: t.ParentID, Name: t.Name, Author: GetGitAuthorAlias()},
				})
				return true, nil
			}
			if t.Location.Line > 0 {
				removeLineFromFile(t.Location.FilePath, t.Location.Line)
				repopkg.Emit(repopkg.EventTodoDeleteEnded, "repo-cli", repopkg.TodoDeletePayload{
					TodoPayload: repopkg.TodoPayload{ID: t.ID, ParentID: t.ParentID, Name: t.Name, Author: GetGitAuthorAlias()},
				})
				return true, nil
			}
		}
	}
	return false, fmt.Errorf("todo not found")
}

// 🎫️TodoToTicket MUST return a non-nil error when the operation fails.
// 🔤️TodoToTicket performs the todo to ticket operation on the repo context.
func (c *repoContext) TodoToTicket(id string, input TicketOpenInput) (*Ticket, error) {
	todos, _ := ScanTodos(c.rootDir)
	var todo *Todo
	for _, t := range todos {
		if t.ID == id {
			todo = t
			break
		}
	}
	if todo == nil {
		return nil, fmt.Errorf("todo not found")
	}

	if input.Title == "" {
	}
	input.Prompt = fmt.Sprintf("%s\n\n%s", todo.Description, input.Prompt)

	ticket, err := c.TicketOpen(input)
	if err != nil {
		return nil, err
	}
	c.TodoDelete(id)
	return ticket, nil
}

// 🚚️removeLineFromMarkdown holds the data fields for a removeLineFromMarkdown record.
func removeLineFromMarkdown(path, name string) {
	content, _ := os.ReadFile(path)
	lines := strings.Split(string(content), "\n")
	var newLines []string
	prefix := "- TODO " + name + ":"
	for _, line := range lines {
		if strings.HasPrefix(strings.TrimSpace(line), prefix) {
			continue
		}
		newLines = append(newLines, line)
	}
	os.WriteFile(path, []byte(strings.Join(newLines, "\n")), 0644)
}

func removeLineFromFile(path string, lineNum int) {
	content, _ := os.ReadFile(path)
	lines := strings.Split(string(content), "\n")
	if lineNum > 0 && lineNum <= len(lines) {
		newLines := append(lines[:lineNum-1], lines[lineNum:]...)
		os.WriteFile(path, []byte(strings.Join(newLines, "\n")), 0644)
	}
}

// ✏️replaceLineInMarkdown MUST rewrite exactly the matching TODO line or return an error.
// 📝️replaceLineInMarkdown rewrites a `.todos.md` TODO entry with a new name/description.
func replaceLineInMarkdown(path, oldName, newName, newDescription string) error {
	content, err := os.ReadFile(path)
	if err != nil {
		return err
	}
	lines := strings.Split(string(content), "\n")
	prefix := "- TODO " + oldName + ":"
	found := false
	for i, line := range lines {
		if strings.HasPrefix(strings.TrimSpace(line), prefix) {
			lines[i] = "- TODO " + newName + ": " + newDescription
			found = true
			break
		}
	}
	if !found {
		return fmt.Errorf("todo entry not found in %s", path)
	}
	return os.WriteFile(path, []byte(strings.Join(lines, "\n")), 0644)
}

// ✏️replaceLineInFile MUST rewrite exactly the matching TODO comment line or return an error.
// 📝️replaceLineInFile rewrites an inline TODO comment with a new name/description.
func replaceLineInFile(path string, lineNum int, newName, newDescription string) error {
	content, err := os.ReadFile(path)
	if err != nil {
		return err
	}
	lines := strings.Split(string(content), "\n")
	if lineNum <= 0 || lineNum > len(lines) {
		return fmt.Errorf("todo line %d out of range in %s", lineNum, path)
	}
	re := regexp.MustCompile(`^(\s*(?://|#|--)\s*TODO\s+)([^:]+):\s*(.*)$`)
	matches := re.FindStringSubmatch(lines[lineNum-1])
	if len(matches) < 4 {
		return fmt.Errorf("todo comment not found at %s:%d", path, lineNum)
	}
	lines[lineNum-1] = matches[1] + newName + ": " + newDescription
	return os.WriteFile(path, []byte(strings.Join(lines, "\n")), 0644)
}
