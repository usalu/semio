package todos

import (
	fmt "fmt"
	os "os"
	strings "strings"
)

// #region ✏️Markdown Edits

// 🚚️RemoveLineFromMarkdown removes a todo item from a markdown file on disk.
func RemoveLineFromMarkdown(path, name string) {
	content, err := os.ReadFile(path)
	if err != nil {
		return
	}
	os.WriteFile(path, []byte(RemoveFromMarkdown(string(content), name)), 0644)
}

// 🚚️RemoveLineFromFile removes a one-based line from a file on disk.
func RemoveLineFromFile(path string, lineNum int) {
	content, err := os.ReadFile(path)
	if err != nil {
		return
	}
	os.WriteFile(path, []byte(RemoveFromFile(string(content), int64(lineNum))), 0644)
}

// 📝️ReplaceLineInMarkdown rewrites a todo markdown entry of a file on disk.
func ReplaceLineInMarkdown(path, oldName, newName, newDescription string) error {
	content, err := os.ReadFile(path)
	if err != nil {
		return err
	}
	rewritten, err := ReplaceInMarkdown(string(content), oldName, newName, newDescription)
	if err != nil {
		return fmt.Errorf("todo entry not found in %s", path)
	}
	return os.WriteFile(path, []byte(rewritten), 0644)
}

// 📝️ReplaceLineInFile rewrites an inline todo comment of a file on disk.
func ReplaceLineInFile(path string, lineNum int, newName, newDescription string) error {
	content, err := os.ReadFile(path)
	if err != nil {
		return err
	}
	rewritten, err := ReplaceInFile(string(content), int64(lineNum), newName, newDescription)
	if err != nil {
		return fmt.Errorf("todo comment not found at %s:%d", path, lineNum)
	}
	return os.WriteFile(path, []byte(rewritten), 0644)
}

// ✏️ReplaceInMarkdown rewrites the item of a markdown document with a new name and description,
// leaving every other line untouched.
func ReplaceInMarkdown(document, oldName, newName, newDescription string) (string, error) {
	prefix := TodoMarkdownPrefix + oldName + ":"
	lines := strings.Split(document, "\n")
	for index, line := range lines {
		if strings.HasPrefix(strings.TrimLeft(line, " \t\r\n\v\f"), prefix) {
			lines[index] = TodoMarkdownPrefix + newName + ": " + newDescription
			return strings.Join(lines, "\n"), nil
		}
	}
	return "", errTodoNotFound()
}

// 🗑️RemoveFromMarkdown removes the item of a markdown document.
func RemoveFromMarkdown(document, name string) string {
	prefix := TodoMarkdownPrefix + name + ":"
	kept := make([]string, 0)
	for _, line := range strings.Split(document, "\n") {
		if strings.HasPrefix(strings.TrimLeft(line, " \t\r\n\v\f"), prefix) {
			continue
		}
		kept = append(kept, line)
	}
	return strings.Join(kept, "\n")
}

// ✏️ReplaceInFile rewrites the todo comment on a one-based line, keeping its indentation and
// comment opener.
func ReplaceInFile(document string, lineNumber int64, newName, newDescription string) (string, error) {
	lines := strings.Split(document, "\n")
	if lineNumber <= 0 || lineNumber > int64(len(lines)) {
		return "", errNoLocation()
	}
	index := int(lineNumber) - 1
	prefix, _, _, ok := SplitTodoCommentParts(lines[index])
	if !ok {
		return "", errTodoNotFound()
	}
	lines[index] = prefix + newName + ": " + newDescription
	return strings.Join(lines, "\n"), nil
}

// 🗑️RemoveFromFile removes the one-based line from a document.
func RemoveFromFile(document string, lineNumber int64) string {
	lines := strings.Split(document, "\n")
	if lineNumber > 0 && lineNumber <= int64(len(lines)) {
		index := int(lineNumber) - 1
		lines = append(lines[:index], lines[index+1:]...)
	}
	return strings.Join(lines, "\n")
}

// #endregion ✏️Markdown Edits
