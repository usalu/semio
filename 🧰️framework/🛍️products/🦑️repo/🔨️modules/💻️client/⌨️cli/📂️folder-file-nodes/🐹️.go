// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Folder, file, section, definition, contributor, and checkpoint nodes.

// #endregion 🧲️Header

package client

import (
	"path/filepath"
	"strings"
	"time"
)

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
	return buildFolderID(f.Path, f.BundleID)
}

// 💚️GetURI MUST return the stored value without modification.
// ⚫️GetURI returns the u r i of the Folder.
func (f *Folder) GetURI() string {
	return buildFolderUriFromPath(f.Path)
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

const (
	FileKindCode     = "code"
	FileKindScript   = "script"
	FileKindConfig   = "config"
	FileKindLab      = "lab"
	FileKindDocs     = "docs"
	FileKindResource = "resource"
	FileKindTemplate = "template"
	FileKindLicense  = "license"
)

const (
	EmojiTechnologyUser       = "👤️"
	EmojiTechnologyInfra      = "🧰️"
	EmojiTechnologyResearch   = "🔬️"
	EmojiTechnologyMono       = "🌱️"
	EmojiBundleLibrary        = "📚️"
	EmojiBundleSchema         = "🛂️"
	EmojiBundleBinary         = "⌨️"
	EmojiBundleUI             = "🖱️"
	EmojiBundleExample        = "📔️"
	EmojiBundleSite           = "🌐️"
	EmojiBundleAssets         = "🏪️"
	EmojiBundleRepo           = "🪆️"
	EmojiFolderOrg            = "🗃️"
	EmojiFolderRequired       = "🛅️"
	EmojiFolderRoot           = "🌱️"
	EmojiFileCode             = "💻️"
	EmojiFileLab              = "🥼️"
	EmojiFileScript           = "📜️"
	EmojiFileDocs             = "📃️"
	EmojiFileConfig           = "⚙️"
	EmojiFileResource         = "💾️"
	EmojiFileTemplate         = "📋️"
	EmojiFileLicense          = "⚖️"
	EmojiLine                 = "📌️"
	EmojiSection              = "🔖️"
	EmojiDefinitionImpl       = "🛠️"
	EmojiDefinitionInterface  = "✂️"
	EmojiDefinitionConstant   = "🪨️"
	EmojiDefinitionTest       = "🧪️"
	EmojiYear                 = "🎆️"
	EmojiMonth                = "🌙️"
	EmojiDay                  = "☀️"
	EmojiHour                 = "⏰️"
	EmojiMinute               = "⌚️"
	EmojiSecond               = "⏱️"
	EmojiGoal                 = "🎯️"
	EmojiTicket               = "🎫️"
	EmojiDraft                = "📝️"
	EmojiTodo                 = "📝️"
	EmojiPolicy               = "👮️"
	EmojiBreach               = "🚫️"
	EmojiBreachScope          = "🔍️"
	EmojiContributor          = "🧑️‍💻️"
	EmojiCheckpoint           = "🔀️"
	EmojiInteractionStarted   = "🌱️"
	EmojiInteractionEdited    = "✏️"
	EmojiInteractionFinished  = "✅️"
	EmojiInteractionRestarted = "🔁️"
	EmojiInteractionDeleted   = "🗑️"
	EmojiSession              = "⚪️"
	EmojiSessionRunning       = "🟡️"
	EmojiSessionCompleted     = "🟢️"
	EmojiSessionInterrupted   = "🔴️"
)

var (
	EmojiRepo         = ""
	EmojiCodebase     = "🖥️"
	EmojiTechnologies = "🏗️"
	EmojiBundles      = "📦️"
	EmojiFolders      = "📁️"
	EmojiFiles        = "📄️"
	EmojiSections     = "🔖️"
	EmojiDefinitions  = "🏷️"
	EmojiYears        = "🎆️"
	EmojiMonths       = "🌙️"
	EmojiDays         = "☀️"
	EmojiHours        = "⏰️"
	EmojiMinutes      = "⌚️"
	EmojiSeconds      = "⏱️"
	EmojiTickets      = "🎫️"
	EmojiGoals        = "🎯️"
	EmojiDrafts       = "📝️"
	EmojiTodos        = "📝️"
	EmojiPolicies     = "👮️"
	EmojiBreaches     = "🚫️"
	EmojiContributors = "🧑️‍💻️"
	EmojiCheckpoints  = "🔀️"
	EmojiSessions     = "⚪️"
	EmojiInteractions = ""
	EmojiTerritorys   = ""
	EmojiTerritory    = ""
	EmojiStatutes     = ""
	EmojiStatute      = ""
)

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
	return buildFileID(f.Path, f.BundleID)
}

// 🤎️GetURI MUST return the stored value without modification.
// 💛️GetURI returns the u r i of the File.
func (f *File) GetURI() string {
	return buildFileUriFromPath(f.Path)
}

// 💗️Section holds the data fields for a section record.
type Section struct {
	ID          string       `json:"id,omitempty"`
	Name        string       `json:"name"`
	Path        string       `json:"path,omitempty"`
	FilePath    string       `json:"filePath,omitempty"`
	Emoji       string       `json:"emoji"`
	StartLine   int          `json:"startLine"`
	EndLine     int          `json:"endLine"`
	StartIndex  int          `json:"startIndex"`
	EndIndex    int          `json:"endIndex"`
	Children    []Section    `json:"children,omitempty"`
	Definitions []Definition `json:"definitions,omitempty"`
}

// 💖️IsNode MUST return true only when the condition is met.
// 🧡️IsNode reports whether the Section is node.
func (s *Section) IsNode() {}

// 💝️GetID MUST return the stored value without modification.
// ❤️GetID returns the i d of the Section.
func (s *Section) GetID() string {
	if s.FilePath != "" {
		fileID := buildFileID(s.FilePath, nil)
		if s.Path != "" {
			return buildSectionID(fileID, strings.Split(strings.ReplaceAll(s.Path, "#", "/"), "/"))
		}
		segment := s.Name
		if s.Emoji != "" {
			segment = s.Emoji + s.Name
		}
		return buildSectionID(fileID, []string{segment})
	}
	emoji := s.Emoji
	if emoji == "" {
		emoji = EmojiSection
	}
	return emojiText(emoji) + Flat(s.Name)
}

// 💘️GetURI MUST return the stored value without modification.
// 🤍️GetURI returns the u r i of the Section.
func (s *Section) GetURI() string {
	return "repo://section/" + s.GetID()
}

// 💕️Definition holds the data fields for a definition record.
type Definition struct {
	ID          string         `json:"id,omitempty"`
	Name        string         `json:"name"`
	Kind        DefinitionKind `json:"kind"`
	FilePath    string         `json:"filePath,omitempty"`
	SectionPath string         `json:"sectionPath,omitempty"`
	Emoji       string         `json:"emoji"`
	StartLine   int            `json:"startLine"`
	EndLine     int            `json:"endLine"`
	StartIndex  int            `json:"startIndex"`
	EndIndex    int            `json:"endIndex"`
}

// 🔖️IsNode MUST return true only when the condition is met.
// 🖤️IsNode reports whether the Definition is node.
func (d *Definition) IsNode() {}

// 🔖️GetID MUST return the stored value without modification.
// 🤎️GetID returns the i d of the Definition.
func (d *Definition) GetID() string {
	if d.FilePath != "" {
		fileID := buildFileID(d.FilePath, nil)
		var sectionParts []string
		if d.SectionPath != "" {
			sectionParts = strings.Split(strings.ReplaceAll(d.SectionPath, "#", "/"), "/")
		}
		return buildDefinitionID(fileID, sectionParts, d.Name, d.Kind)
	}
	data := map[string]interface{}{"kind": string(d.Kind)}
	return definitionKindEmoji(data) + Flat(d.Name)
}

// 🔖️GetURI MUST return the stored value without modification.
// 💖️GetURI returns the u r i of the Definition.
func (d *Definition) GetURI() string {
	return "repo://definition/" + d.GetID()
}

// 🔖️Contributor holds the data fields for a contributor record.
type Contributor struct {
	Alias         string                          `yaml:"alias" json:"alias"`
	Aliases       []string                        `yaml:"aliases,omitempty" json:"aliases,omitempty"`
	Emoji         string                          `yaml:"emoji,omitempty" json:"emoji,omitempty"`
	Github        string                          `yaml:"github" json:"github"`
	Githubs       []string                        `yaml:"githubs,omitempty" json:"githubs,omitempty"`
	Name          string                          `yaml:"name" json:"name"`
	Names         []string                        `yaml:"names,omitempty" json:"names,omitempty"`
	Email         string                          `yaml:"email" json:"email"`
	Emails        []string                        `yaml:"emails,omitempty" json:"emails,omitempty"`
	Links         map[string]string               `yaml:"links,omitempty" json:"links,omitempty"`
	Fingerprint   string                          `yaml:"fingerprint,omitempty" json:"fingerprint,omitempty"`
	Fingerprints  []string                        `yaml:"fingerprints,omitempty" json:"fingerprints,omitempty"`
	Contributions ContributorContributionsStorage `yaml:"contributions,omitempty" json:"contributions,omitempty"`
}

// 🔖️IsNode MUST return true only when the condition is met.
// 💝️IsNode reports whether the Contributor is node.
func (c *Contributor) IsNode() {}

// 🔖️GetID MUST return the stored value without modification.
// 💘️GetID returns the i d of the Contributor.
func (c *Contributor) GetID() string {
	return emojiText(EmojiContributor) + Flat(c.Alias)
}

// 🔖️GetURI MUST return the stored value without modification.
// 🏵️GetURI returns the u r i of the Contributor.
func (c *Contributor) GetURI() string {
	return "repo://contributor/" + c.GetID()
}

// ✔️Checkpoint holds the data fields for a checkpoint record.
type Checkpoint struct {
	ID       string    `json:"id"`
	SHA      string    `json:"sha"`
	Title    string    `json:"title"`
	AuthorID *string   `json:"authorId,omitempty"`
	Date     time.Time `json:"date"`
}

// 🔖️IsNode MUST return true only when the condition is met.
// 🔢️IsNode reports whether the Checkpoint is node.
func (c *Checkpoint) IsNode() {}

// 🔖️GetID MUST return the stored value without modification.
// 🌸️GetID returns the i d of the Checkpoint.
func (c *Checkpoint) GetID() string { return emojiText(EmojiCheckpoint) + c.SHA }

// 🔖️GetURI MUST return the stored value without modification.
// 🌺️GetURI returns the u r i of the Checkpoint.
func (c *Checkpoint) GetURI() string {
	return "repo://checkpoint/" + c.GetID()
}
