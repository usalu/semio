// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📁️folder_file carries the folder and file shapes with their kind derivations.

// #endregion 🧲️Header

package model

import (
	os "os"
	filepath "path/filepath"
	strings "strings"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 📁️Folder File

// 📁️FolderKind represents a folder kind value.
type FolderKind string

const FolderKindOrganization FolderKind = "organization"

const FolderKindRequired FolderKind = "required"

const FolderKindRoot FolderKind = "root"

// ⚪️IsValid MUST return true only when the condition is met.
// ◻IsValid reports whether the FolderKind is valid.
func (e FolderKind) IsValid() bool {
	switch e {
	case FolderKindOrganization, FolderKindRequired, FolderKindRoot:
		return true
	}
	return false
}

// ⚫️String MUST return the canonical string value.
// ◼String returns the string representation of the FolderKind.
func (e FolderKind) String() string {
	return string(e)
}

// 💾️DeriveFolderKind MUST return a valid value for any recognized input.
// 🟠️DeriveFolderKind infers and returns the folder kind from the given input.

// 🩵️DeriveFolderKind holds the data fields for a DeriveFolderKind record.
func DeriveFolderKind(path string) FolderKind {
	return NewArtifactIndex(workspace.GetRootDir(), nil).FolderKind(path)
}

// 🧭️FolderKind classifies a directory within this index's repository.
func (index *ArtifactLayout) FolderKind(path string) FolderKind {
	index.mutex.Lock()
	defer index.mutex.Unlock()
	return index.folderKind(path)
}
func (index *ArtifactLayout) folderKind(path string) FolderKind {
	normalized, valid := index.namespacePath(path)
	if !valid {
		return FolderKindOrganization
	}
	if cached, ok := index.folderKinds[normalized]; ok {
		return cached
	}
	kind := FolderKindOrganization
	if strings.HasPrefix(filepath.Base(normalized), ".") {
		kind = FolderKindRequired
	} else if entries, err := index.readDirectory(filepath.Join(index.rootDir, normalized)); err == nil {
		for _, entry := range entries {
			if entry.Type()&os.ModeSymlink != 0 || entry.IsDir() {
				continue
			}
			if entry.Type() != 0 {
				continue
			}
			name := entry.Name()
			extension := filepath.Ext(name)
			if name == "package.json" || name == "pyproject.toml" || name == "go.mod" || name == "Cargo.toml" || extension == ".csproj" || extension == ".sln" {
				kind = FolderKindRequired
				break
			}
		}
	}
	index.folderKinds[normalized] = kind
	return kind
}

func readArtifactDirectory(path string) ([]os.DirEntry, error) {
	info, err := os.Lstat(path)
	if err != nil {
		return nil, err
	}
	if !info.IsDir() || info.Mode()&os.ModeSymlink != 0 {
		return nil, os.ErrInvalid
	}
	return os.ReadDir(path)
}

// 🩶️IsGeneratedFolder MUST return true only when the condition is met.
// 🟡️IsGeneratedFolder reports whether the value is generated folder.
func IsGeneratedFolder(path string) bool {
	parts := strings.Split(filepath.ToSlash(path), "/")
	for _, part := range parts {
		if part == "generated" || part == "dist" || part == "build" || part == "node_modules" || part == "__pycache__" || part == ".next" || part == "coverage" {
			return true
		}
	}
	generatedFolders := []string{
		"js/vscode/generated",
		"js/compose/generated",
	}
	normalized := filepath.ToSlash(path)
	for _, gen := range generatedFolders {
		if normalized == gen || strings.HasPrefix(normalized, gen+"/") {
			return true
		}
	}
	return false
}

// 🩷️Folder holds the data fields for a folder record.
type Folder struct {
	ID        string     `json:"id"`
	Path      string     `json:"path"`
	URI       string     `json:"uri"`
	Name      string     `json:"name"`
	ParentID  *string    `json:"parentId,omitempty"`
	BundleID  *string    `json:"bundleId,omitempty"`
	Kind      FolderKind `json:"kind"`
	Emoji     string     `json:"emoji"`
	Ignored   bool       `json:"ignored"`
	Generated bool       `json:"generated"`
}

// 💜️IsNode MUST return true only when the condition is met.
// 🟢️IsNode reports whether the Folder is node.
func (f *Folder) IsNode() {}

// 💙️GetID MUST return the stored value without modification.
// ⚪️GetID returns the i d of the Folder.
func (f *Folder) GetID() string {
	if f.ID != "" {
		return f.ID
	}
	return BuildFolderID(f.Path, f.BundleID)
}

// 💚️GetURI MUST return the stored value without modification.
// ⚫️GetURI returns the u r i of the Folder.
func (f *Folder) GetURI() string {
	if f.URI != "" {
		return f.URI
	}
	return BuildFolderUriFromPath(f.Path)
}

// 📄️File holds the data fields for a file record.
type File struct {
	ID        string  `json:"id"`
	Path      string  `json:"path"`
	URI       string  `json:"uri"`
	Name      string  `json:"name"`
	Extension string  `json:"extension"`
	FolderID  *string `json:"folderId,omitempty"`
	BundleID  *string `json:"bundleId,omitempty"`
	Kind      string  `json:"kind"`
	Emoji     string  `json:"emoji"`
	Ignored   bool    `json:"ignored"`
	Generated bool    `json:"generated"`
}

const FileKindCode = "code"

const FileKindScript = "script"

const FileKindConfig = "config"

const FileKindLab = "lab"

const FileKindDocs = "docs"

const FileKindResource = "resource"

const FileKindTemplate = "template"

const FileKindLicense = "license"

// 💛️DeriveFileKind MUST return a valid value for any recognized input.
// 🩶️DeriveFileKind infers and returns the file kind from the given input.
func DeriveFileKind(name string) string {
	ext := strings.ToLower(filepath.Ext(name))
	nameLower := strings.ToLower(name)
	nameNoExt := strings.TrimSuffix(nameLower, ext)

	if strings.Contains(nameLower, "license") || strings.Contains(nameLower, "licence") {
		return FileKindLicense
	}

	if strings.HasSuffix(nameNoExt, ".test") || strings.HasSuffix(nameNoExt, ".spec") ||
		strings.HasSuffix(nameNoExt, ".tests") || strings.HasSuffix(nameNoExt, ".requirements") ||
		strings.HasSuffix(nameNoExt, "_test") || strings.HasSuffix(nameNoExt, "_tests") ||
		strings.HasSuffix(nameNoExt, "_spec") || strings.HasSuffix(nameNoExt, "_benchmark") ||
		strings.HasSuffix(nameNoExt, ".benchmark") ||
		strings.HasSuffix(nameNoExt, ".stories") || strings.HasSuffix(nameNoExt, ".story") ||
		strings.HasPrefix(nameLower, "test_") || strings.HasPrefix(nameLower, "test.") || nameLower == "conftest.py" {
		return FileKindLab
	}

	if strings.HasSuffix(nameNoExt, ".config") || strings.HasSuffix(nameNoExt, ".conf") ||
		strings.HasSuffix(nameNoExt, ".rc") || strings.HasSuffix(nameNoExt, ".cfg") {
		return FileKindConfig
	}

	switch ext {
	case ".sh", ".bash", ".zsh", ".fish", ".bat", ".cmd", ".ps1", ".psm1":
		return FileKindScript
	case ".json", ".yaml", ".yml", ".toml", ".xml", ".ini", ".conf", ".config",
		".env", ".properties", ".cfg", ".hcl", ".tf", ".tfvars":
		return FileKindConfig
	case ".gitignore", ".gitattributes", ".gitmodules",
		".dockerignore", ".editorconfig", ".eslintignore",
		".prettierignore", ".npmignore", ".browserslistrc":
		return FileKindConfig
	case ".md", ".txt", ".rst", ".adoc", ".asciidoc", ".org", ".rdoc":
		return FileKindDocs
	case ".tpl", ".tmpl", ".gotmpl", ".mustache", ".hbs", ".handlebars",
		".ejs", ".njk", ".nunjucks", ".jinja", ".jinja2", ".j2",
		".liquid", ".pug", ".jade", ".slim", ".haml":
		return FileKindTemplate
	case ".png", ".jpg", ".jpeg", ".gif", ".svg", ".ico", ".webp", ".bmp", ".tiff", ".tif",
		".ttf", ".woff", ".woff2", ".eot", ".otf",
		".mp3", ".mp4", ".wav", ".ogg", ".webm", ".avi", ".mov",
		".pdf", ".zip", ".tar", ".gz", ".bz2", ".xz", ".7z", ".rar",
		".bin", ".dat", ".db", ".sqlite", ".sqlite3",
		".wasm", ".map":
		return FileKindResource
	case ".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs", ".mts", ".cts",
		".py", ".pyi", ".pyw",
		".cs", ".csx",
		".go",
		".c", ".cpp", ".cc", ".cxx", ".h", ".hpp", ".hxx",
		".rs",
		".java", ".kt", ".kts", ".scala",
		".swift", ".m", ".mm",
		".rb", ".erb",
		".php",
		".lua",
		".r", ".R",
		".jl",
		".ex", ".exs",
		".hs", ".lhs",
		".ml", ".mli",
		".clj", ".cljs", ".cljc",
		".dart", ".v", ".sv", ".vhd", ".vhdl",
		".zig", ".nim", ".cr", ".d",
		".f", ".f90", ".f95", ".f03",
		".pl", ".pm",
		".css", ".scss", ".less", ".sass", ".styl",
		".html", ".htm", ".vue", ".svelte", ".astro",
		".sql", ".graphql", ".gql",
		".proto", ".thrift", ".avsc",
		".g4":
		return FileKindCode
	}

	if strings.HasPrefix(nameLower, "dockerfile") || nameLower == "makefile" || nameLower == "justfile" ||
		nameLower == "rakefile" || nameLower == "gemfile" || nameLower == "vagrantfile" ||
		nameLower == "procfile" || nameLower == "brewfile" || nameLower == "cmakelists.txt" ||
		nameLower == ".babelrc" || nameLower == ".eslintrc" || nameLower == ".prettierrc" ||
		nameLower == ".stylelintrc" || nameLower == ".nycrc" || nameLower == ".npmrc" ||
		nameLower == ".nvmrc" || nameLower == ".yarnrc" || nameLower == ".ruby-version" ||
		nameLower == ".python-version" || nameLower == ".node-version" ||
		nameLower == ".tool-versions" || nameLower == ".cursorrules" || nameLower == ".clinerules" ||
		nameLower == "nx.json" || nameLower == "tsconfig.json" || nameLower == "tslint.json" ||
		nameLower == "composer.json" || nameLower == "cargo.lock" ||
		nameLower == "package-lock.json" || nameLower == "yarn.lock" || nameLower == "pnpm-lock.yaml" ||
		nameLower == "go.sum" || nameLower == "uv.lock" {
		return FileKindConfig
	}

	return FileKindResource
}

// 🧡️IsNode MUST return true only when the condition is met.
// 💜️IsNode reports whether the File is node.
func (f *File) IsNode() {}

// ❤️IsGenerated MUST return true only when the condition is met.
// 💙️IsGenerated reports whether the value is generated.
func IsGenerated(path string) bool {
	base := strings.ToLower(filepath.Base(path))
	if base == "package-lock.json" || base == "yarn.lock" || base == "pnpm-lock.yaml" || base == "go.sum" || base == "uv.lock" {
		return true
	}
	if base == "🎫️ticket.json" || base == "🎯️goal.json" {
		return true
	}
	if strings.HasSuffix(base, ".generated.go") || strings.HasSuffix(base, ".pb.go") {
		return true
	}

	if strings.Contains(path, "generated") {
	}
	return false
}

// 🤍️IsSemanticallyIgnored MUST return true only when the condition is met.
// ⚡️IsSemanticallyIgnored reports whether the value is semantically ignored.
func IsSemanticallyIgnored(path string) bool {
	base := filepath.Base(path)
	if strings.HasPrefix(base, ".") && base != ".gitignore" && base != ".env" {

		return true
	}

	if base == "dist" || base == "build" || base == "coverage" || base == "__pycache__" || base == "node_modules" {
		return true
	}
	return false
}

// 🖤️GetID MUST return the stored value without modification.
// 💚️GetID returns the i d of the File.
func (f *File) GetID() string {
	if f.ID != "" {
		return f.ID
	}
	return BuildFileID(f.Path, f.BundleID)
}

// 🤎️GetURI MUST return the stored value without modification.
// 💛️GetURI returns the u r i of the File.
func (f *File) GetURI() string {
	if f.URI != "" {
		return f.URI
	}
	return BuildFileUriFromPath(f.Path)
}

// #endregion 📁️Folder File
