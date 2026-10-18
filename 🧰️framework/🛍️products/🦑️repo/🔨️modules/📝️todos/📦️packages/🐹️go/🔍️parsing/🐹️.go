package todos

import (
	strings "strings"
	unicode "unicode"
	utf8 "unicode/utf8"

	identity "github.com/usalu/semio/repo/identity"
	model "github.com/usalu/semio/repo/model"
)

// #region 🔍️Parsing

// 📄️TodoMarkdownName is the file a directory records its own todos in.
const TodoMarkdownName = ".todos.md"

// 🏷️TodoMarkdownPrefix is the item prefix a todo markdown entry carries.
const TodoMarkdownPrefix = "- TODO "

// 🗂️TodoScanExtensions are the file extensions a scan reads todo comments out of.
var TodoScanExtensions = []string{".ts", ".js", ".tsx", ".jsx", ".go", ".cs", ".py", ".md", ".json"}

// 💬️TodoCommentOpeners are the comment openers a todo comment may start with.
var TodoCommentOpeners = []string{"//", "#", "--"}

// 🗑️TodoSkippedDirectories are the directory names a scan never descends into.
var TodoSkippedDirectories = []string{"node_modules", "dist", "build"}

// ✂️SplitTodoCommentParts returns the untouched prefix, the name and the description of a todo
// comment line, so a rewrite can put a new name and description back without disturbing the
// indentation or the comment opener.
func SplitTodoCommentParts(line string) (string, string, string, bool) {
	rest := strings.TrimLeft(line, " \t\r\n\v\f")
	opener := ""
	for _, candidate := range TodoCommentOpeners {
		if strings.HasPrefix(rest, candidate) {
			opener = candidate
			break
		}
	}
	if opener == "" {
		return "", "", "", false
	}
	afterOpener := strings.TrimLeft(rest[len(opener):], " \t\r\n\v\f")
	if !strings.HasPrefix(afterOpener, "TODO") {
		return "", "", "", false
	}
	keyword := afterOpener[len("TODO"):]
	if keyword == "" {
		return "", "", "", false
	}
	first, size := utf8.DecodeRuneInString(keyword)
	if size == 0 || !unicode.IsSpace(first) {
		return "", "", "", false
	}
	afterKeyword := strings.TrimLeft(keyword, " \t\r\n\v\f")
	if afterKeyword == "" {
		return "", "", "", false
	}
	colon := strings.Index(afterKeyword, ":")
	if colon < 0 {
		return "", "", "", false
	}
	name := afterKeyword[:colon]
	description := afterKeyword[colon+1:]
	prefix := line[:len(line)-len(afterKeyword)]
	return prefix, strings.TrimSpace(name), strings.TrimSpace(description), true
}

// 📰️ParseTodoMarkdown parses a todo markdown document. Every item becomes a todo whose name is the
// text before the first colon and whose description is everything after it.
func ParseTodoMarkdown(content string, parentPath string) []*model.Todo {
	found := make([]*model.Todo, 0)
	for _, line := range strings.Split(content, "\n") {
		trimmed := strings.TrimSpace(line)
		if !strings.HasPrefix(trimmed, TodoMarkdownPrefix) {
			continue
		}
		rest := trimmed[len(TodoMarkdownPrefix):]
		name, description := rest, ""
		if colon := strings.Index(rest, ":"); colon >= 0 {
			name, description = rest[:colon], rest[colon+1:]
		}
		name = strings.TrimSpace(name)
		description = strings.TrimSpace(description)
		found = append(found, &model.Todo{
			ID:          identity.Slugify(name),
			Name:        name,
			Description: description,
			ParentID:    parentPath,
			Location:    &model.Location{FilePath: joinTodoPath(parentPath, TodoMarkdownName)},
		})
	}
	return found
}

// 💬️ParseTodoComments parses the todo comments of a source file. The line number is one based.
func ParseTodoComments(content string, filePath string) []*model.Todo {
	found := make([]*model.Todo, 0)
	for index, line := range strings.Split(content, "\n") {
		_, name, description, ok := SplitTodoCommentParts(line)
		if !ok {
			continue
		}
		found = append(found, &model.Todo{
			ID:          identity.Slugify(name),
			Name:        name,
			Description: description,
			ParentID:    filePath,
			Location:    &model.Location{FilePath: filePath, Line: index + 1, Column: 1},
		})
	}
	return found
}

// 💬️TodoCommentOpener returns the comment opener a todo written into a file gets, by extension.
func TodoCommentOpener(path string) string {
	switch todoExtensionOf(path) {
	case ".py", ".sh", ".yaml", ".yml":
		return "#"
	case ".sql", ".lua":
		return "--"
	}
	return "//"
}

// 🗂️todoExtensionOf returns the extension of the last segment of a path.
func todoExtensionOf(path string) string {
	name := path
	if index := strings.LastIndexAny(path, "/\\"); index >= 0 {
		name = path[index+1:]
	}
	if index := strings.LastIndex(name, "."); index > 0 {
		return name[index:]
	}
	return ""
}

// 🛤️joinTodoPath joins a parent path and a name with a forward slash.
func joinTodoPath(parent, name string) string {
	if parent == "" || parent == "." {
		return name
	}
	return strings.TrimRight(parent, "/") + "/" + name
}

// #endregion 🔍️Parsing
