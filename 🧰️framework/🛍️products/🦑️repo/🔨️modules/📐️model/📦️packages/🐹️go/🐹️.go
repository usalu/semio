// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

// 📐️Package model holds the pure data shapes of the 🦑️repo product together with the slug
// 📐️vocabularies and the derivations that follow from the shape alone. Nothing in it touches
// 📐️the filesystem, a process or a clock. The wire contract is 🧬️schema/🔣️.json and the twin
// 📐️implementation is 📦️packages/🦀️rust.
package model

import (
	json "encoding/json"
	errors "errors"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	regexp "regexp"
	runtime "runtime"
	strconv "strconv"
	strings "strings"
	sync "sync"
	template "text/template"
	time "time"
	utf8 "unicode/utf8"

	identity "github.com/usalu/semio/repo/identity"
	search "github.com/usalu/semio/repo/search"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #endregion 🔌️Adapters

// #region 📋️Allowed Values

// 📋️AllowedValuesEnvironment overrides the module-relative lookup of the shared vocabulary table.
const AllowedValuesEnvironment = "SEMIO_REPO_MODEL_ALLOWED_VALUES"

// 📋️AllowedValues mirrors 🧬️schema/🔣️allowed-values.json, the vocabulary table the Rust twin
// 📋️embeds with include_str!. Neither implementation re-declares these strings in source.
type AllowedValues struct {
	LLMs    []string `json:"llms"`
	Efforts []string `json:"efforts"`
	Clients []string `json:"clients"`
}

var (
	allowedOnce  sync.Once
	allowedTable AllowedValues
	allowedError error
)

// 🧭️allowedValuesPath resolves the table relative to this source file, because `go:embed` cannot
// reach a parent directory and the table is owned by the module, not by the Go package.
func allowedValuesPath() (string, error) {
	if override := strings.TrimSpace(os.Getenv(AllowedValuesEnvironment)); override != "" {
		return override, nil
	}
	if _, file, _, ok := runtime.Caller(0); ok {
		candidate := filepath.Join(filepath.Dir(file), "..", "..", "🧬️schema", "🔣️allowed-values.json")
		if _, err := os.Stat(candidate); err == nil {
			return filepath.Clean(candidate), nil
		}
	}
	suffix := filepath.Join("🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📐️model", "🧬️schema", "🔣️allowed-values.json")
	directory, err := os.Getwd()
	if err != nil {
		return "", err
	}
	for {
		candidate := filepath.Join(directory, suffix)
		if _, err := os.Stat(candidate); err == nil {
			return candidate, nil
		}
		parent := filepath.Dir(directory)
		if parent == directory {
			return "", errors.New("model: allowed-value table not found")
		}
		directory = parent
	}
}

// 📋️Allowed loads the shared vocabulary table once per process.
func Allowed() (AllowedValues, error) {
	allowedOnce.Do(func() {
		path, err := allowedValuesPath()
		if err != nil {
			allowedError = err
			return
		}
		data, err := os.ReadFile(path)
		if err != nil {
			allowedError = err
			return
		}
		allowedError = json.Unmarshal(data, &allowedTable)
	})
	return allowedTable, allowedError
}

// 🤖️AllowedLLMs returns the allowed LLM slugs, in declaration order.
func AllowedLLMs() ([]string, error) {
	table, err := Allowed()
	return table.LLMs, err
}

// 🏋️AllowedEfforts returns the allowed reasoning-effort slugs, in declaration order.
func AllowedEfforts() ([]string, error) {
	table, err := Allowed()
	return table.Efforts, err
}

// 💻️AllowedClients returns the allowed client slugs, in declaration order.
func AllowedClients() ([]string, error) {
	table, err := Allowed()
	return table.Clients, err
}

// #endregion 📋️Allowed Values

// #region 🚚️Split

// #region 🎁️Templates

var textTemplateContent = `{{- define "text/entity" -}}
{{- if .ID }}{{ colorize (sanitizeProp .ID) "bold" .IsTTY }}{{ end -}}
{{- range $i, $p := .Props -}}
{{- if or (ne $.ID "") (gt $i 0) }} {{ end -}}
{{- colorize $p (propColor $i) $.IsTTY -}}
{{- end -}}
{{- end -}}
{{- define "text/analyze_summary" -}}
{{ colorize "->" "blue" .IsTTY }} found {{ .Total }} breachs{{ if .Autofixable }} ({{ .Autofixable }} autofixable){{ end }}
{{- end -}}
{{- define "text/breach" -}}
{{ printf "  " }}{{ colorize "breach" "red" .IsTTY }} {{ .Kind }} {{ colorize (printf "%s:%s" .Scope .Line) "dim" .IsTTY }} {{ .Summary }}
{{- end -}}
{{- define "text/fix" -}}
{{ colorize "->" "blue" .IsTTY }} fixed {{ .Fixed }} breachs ({{ .Remaining }} remaining)
{{- end -}}
{{- define "text/id_path" -}}
{{ colorize "->" "blue" .IsTTY }} {{ .ID }} {{ .Path }}
{{- end -}}
{{- define "text/id_only" -}}
{{ colorize "->" "blue" .IsTTY }} item {{ .ID }}
{{- end -}}
{{- define "text/done_success" -}}
{{- colorize "ok" "green" .IsTTY }} done  {{ .Command }}  {{ .Duration -}}
{{- end -}}
{{- define "text/done_failure" -}}
{{- colorize "failed" "red" .IsTTY }} failed {{ .Command }}  {{ .Duration }} (exit: {{ .ExitCode }})
{{- end -}}
{{- define "text/error" -}}
{{ colorize "error" "red" .IsTTY }} error: {{ .Message }}
{{- end -}}
{{- define "text/error_detail" -}}
{{ .Detail }}
{{- end -}}
{{- define "text/log" -}}
{{ colorize "-" "dim" .IsTTY }} {{ .Message }}
{{- end -}}
{{- define "text/progress_tty" -}}
{{- printf "\r" }}{{ colorize "..." "blue" true }} {{ .Percent }}% ({{ .Current }}/{{ .Total }}) {{ .Step -}}
{{- end -}}
{{- define "text/progress" -}}
progress: {{ .Percent }}% {{ .Step }}
{{- end -}}
{{- define "text/result_fallback" -}}
{{ colorize "->" "blue" .IsTTY }} {{ .Data }}
{{- end -}}`

var markdownTemplateContent = "{{- define \"md/entity_link\" -}}\n[{{ sanitizeProp .ID }}]({{ .URI }}){{ range .Props }} - `{{ . }}`{{ end }}\n{{- end -}}\n" +
	"{{- define \"md/entity_item\" -}}\n- [{{ sanitizeProp .ID }}]({{ .URI }}){{ range .Props }} - `{{ . }}`{{ end }}\n{{- end -}}\n" +
	"{{- define \"md/analyze_total\" -}}\n- **Total Breachs**: {{ .Total }}\n{{- end -}}\n" +
	"{{- define \"md/analyze_autofixable\" -}}\n- **Autofixable**: {{ .Autofixable }}\n{{- end -}}\n" +
	"{{- define \"md/breach\" -}}\n- [{{ .Kind }}](repo://statute/{{ pathToUriPath .Kind }}) - {{ .Scope }}:{{ .Line }} - {{ .Summary }}\n{{- end -}}\n" +
	"{{- define \"md/error\" -}}\n**Error: {{ .Message }}**\n{{- end -}}\n" +
	"{{- define \"md/error_detail\" -}}\n> {{ .Detail }}\n{{- end -}}\n" +
	"{{- define \"md/result_fallback\" -}}\n[{{ sanitizeProp .Name }}]({{ .URI }})\n{{- end -}}\n" +
	"{{- define \"md/tree_node_category\" -}}\n{{- .Indent }}- {{ if .URI }}[{{ .Label }}]({{ .URI }}){{ else }}{{ .Label }}{{ end }}\n{{- end -}}\n" +
	"{{- define \"md/tree_node_entity\" -}}\n{{- .Indent }}{{ .Content }}\n{{- end -}}"

var TextTpl *template.Template

var MdTpl *template.Template

// 💿️templateFuncMap holds the data fields for a templateFuncMap record.
func templateFuncMap() template.FuncMap {
	fm := TxtFuncMap()
	fm["colorize"] = func(s, color string, isTTY bool) string {
		return Colorize(s, colorNameToANSI(color), isTTY)
	}
	fm["propColor"] = func(i int) string {
		switch i {
		case 0:
			return "yellow"
		case 1:
			return "blue"
		case 2:
			return "green"
		default:
			return "dim"
		}
	}
	fm["sanitizeProp"] = sanitizeProp
	fm["pathToUriPath"] = workspace.PathToUriPath
	return fm
}

// 🎨️colorNameToANSI holds the data fields for a colorNameToANSI record.
func colorNameToANSI(name string) string {
	switch name {
	case "red":
		return ColorRed
	case "green":
		return ColorGreen
	case "yellow":
		return ColorYellow
	case "blue":
		return ColorBlue
	case "dim":
		return ColorDim
	case "bold":
		return ColorBold
	default:
		return ""
	}
}

func initTemplates() {
	TextTpl = template.Must(template.New("text").Funcs(templateFuncMap()).Parse(textTemplateContent))
	MdTpl = template.Must(template.New("md").Funcs(templateFuncMap()).Parse(markdownTemplateContent))
}

func RenderTemplate(tpl *template.Template, name string, data interface{}) string {
	var sb strings.Builder
	if err := tpl.ExecuteTemplate(&sb, name, data); err != nil {
		return fmt.Sprintf("[template error: %s: %v]", name, err)
	}
	return sb.String()
}

// 🔸️init holds the data fields for a init record.
func init() {
	initTemplates()
}

// #endregion 🎁️Templates

// #region 🔑️Engine Events

// 🔷️inferEntityKindFromStatute holds the data fields for a inferEntityKindFromStatute record.
func InferEntityKindFromStatute(statute Statute) string {
	path := string(statute)
	switch {
	case strings.Contains(path, "/definition/"):
		return "definition"
	case strings.Contains(path, "/section/"):
		return "section"
	case strings.Contains(path, "/file/"):
		return "file"
	case strings.Contains(path, "/folder/"):
		return "folder"
	case strings.Contains(path, "/bundle/"):
		return "bundle"
	case strings.Contains(path, "/technology/"):
		return "technology"
	default:
		return "repo"
	}
}

// #endregion 🔑️Engine Events

// #region 🖥️Entity Emojis Command

// #region 🖥️Entity Emojis Command
func AllEntityEmojis() []string {
	seen := map[string]bool{}
	var result []string
	add := func(e string) {
		normalized := EmojiText(e)
		if normalized == "" || seen[normalized] {
			return
		}
		seen[normalized] = true
		result = append(result, normalized)
	}

	add(EmojiTechnologyUser)
	add(EmojiTechnologyInfra)
	add(EmojiTechnologyResearch)
	add(EmojiTechnologyMono)
	add(EmojiBundleLibrary)
	add(EmojiBundleSchema)
	add(EmojiBundleBinary)
	add(EmojiBundleUI)
	add(EmojiBundleExample)
	add(EmojiBundleSite)
	add(EmojiBundleAssets)
	add(EmojiBundleRepo)
	add(EmojiFolderOrg)
	add(EmojiFolderRequired)
	add(EmojiFileCode)
	add(EmojiFileLab)
	add(EmojiFileScript)
	add(EmojiFileDocs)
	add(EmojiFileConfig)
	add(EmojiFileResource)
	add(EmojiFileLicense)
	add(EmojiLine)
	add(EmojiSection)
	add(EmojiDefinitionImpl)
	add(EmojiDefinitionInterface)
	add(EmojiDefinitionConstant)
	add(EmojiDefinitionTest)
	add(EmojiYear)
	add(EmojiMonth)
	add(EmojiDay)
	add(EmojiHour)
	add(EmojiMinute)
	add(EmojiSecond)
	add(EmojiGoal)
	add(EmojiTicket)
	add(EmojiDraft)
	add(EmojiTodo)
	add(EmojiPolicy)
	add(EmojiBreach)
	add(EmojiBreachScope)
	add(EmojiContributor)
	add(EmojiCheckpoint)
	add(EmojiInteractionStarted)
	add(EmojiInteractionEdited)
	add(EmojiInteractionFinished)
	add(EmojiInteractionRestarted)
	add(EmojiInteractionDeleted)
	add(EmojiSession)
	add(EmojiSessionRunning)
	add(EmojiSessionCompleted)
	add(EmojiSessionInterrupted)

	add(EmojiCodebase)
	add(EmojiTechnologies)
	add(EmojiBundles)
	add(EmojiFolders)
	add(EmojiFiles)
	add(EmojiDefinitions)
	return result
}

// #endregion 🖥️Entity Emojis Command

// #region 🎺️Models

// 🎫️TicketNode holds the data fields for a ticket node record.
type TicketNode struct {
	ID, Slug, Status string
	Title, URI       string
	GoalID, ParentID string
	Children         []*TicketNode
	Created          string
	Finished         string
	Description      string
	Summary          string
}

// 💿️GoalNode holds the data fields for a goal node record.
type GoalNode struct {
	ID, Title, Status  string
	DueDate, CreatedAt string
	Description        string
	Children           []*GoalNode
	Tickets            []*TicketNode
}

// #endregion 🎺️Models

// #region 🧊️ANSI

const ColorReset = "\033[0m"

const ColorRed = "\033[31m"

const ColorGreen = "\033[32m"

const ColorYellow = "\033[33m"

const ColorBlue = "\033[34m"

const ColorDim = "\033[2m"

const ColorBold = "\033[1m"

// 💿️colorize holds the data fields for a colorize record.
func Colorize(s string, color string, enabled bool) string {
	if !enabled {
		return s
	}
	return color + s + ColorReset
}

// #endregion 🧊️ANSI

// #region 💡️GraphQL Types

// 🔌️Node defines the interface contract for node operations.
type Node interface {
	IsNode()
	GetID() string
}

// 📖️DefinitionKind represents a definition kind value.
type DefinitionKind string

const DefinitionKindImplementation DefinitionKind = "implementation"

const DefinitionKindInterface DefinitionKind = "interface"

const DefinitionKindConstant DefinitionKind = "constant"

const DefinitionKindTest DefinitionKind = "test"

// 🔷️IsValid MUST return true only when the condition is met.
// ▶️IsValid reports whether the DefinitionKind is valid.
func (e DefinitionKind) IsValid() bool {
	switch e {
	case DefinitionKindImplementation, DefinitionKindInterface, DefinitionKindConstant, DefinitionKindTest:
		return true
	}
	return false
}

// 🔤️String MUST return the canonical string value.
// 🔤️String returns the string representation of the DefinitionKind.
func (e DefinitionKind) String() string {
	return string(e)
}

// 📝️DeriveDefinitionKind MUST return a valid value for any recognized input.
// 💾️DeriveDefinitionKind infers and returns the definition kind from the given input.
func DeriveDefinitionKind(rawKind string) DefinitionKind {
	switch strings.ToLower(rawKind) {
	case "interface", "type", "trait", "abstract",
		"extend type", "extend interface", "extend enum", "extend union", "extend input",
		"input", "union", "scalar",
		"delegate", "record":
		return DefinitionKindInterface
	case "const", "constant", "enum", "var", "let", "static":
		return DefinitionKindConstant
	case "test":
		return DefinitionKindTest
	default:
		return DefinitionKindImplementation
	}
}

// 🎫️TicketStatus represents a ticket status value.
type TicketStatus string

const TicketStatusOpen TicketStatus = "open"

const TicketStatusClosed TicketStatus = "closed"

// 🔶️IsValid MUST return true only when the condition is met.
// 🔑️IsValid reports whether the TicketStatus is valid.
func (e TicketStatus) IsValid() bool {
	switch e {
	case TicketStatusOpen, TicketStatusClosed:
		return true
	}
	return false
}

// 🔹️String MUST return the canonical string value.
// 📩️String returns the string representation of the TicketStatus.
func (e TicketStatus) String() string {
	return string(e)
}

// 🔸️BreachPriority represents a breach priority value.
type BreachPriority string

const BreachPriorityHigh BreachPriority = "high"

const BreachPriorityMedium BreachPriority = "medium"

const BreachPriorityLow BreachPriority = "low"

// 🔺️IsValid MUST return true only when the condition is met.
// ⚠️IsValid reports whether the BreachPriority is valid.
func (e BreachPriority) IsValid() bool {
	switch e {
	case BreachPriorityHigh, BreachPriorityMedium, BreachPriorityLow:
		return true
	}
	return false
}

// 🔻️String MUST return the canonical string value.
// 🔷️String returns the string representation of the BreachPriority.
func (e BreachPriority) String() string {
	return string(e)
}

// ⬜️NormalizeLLMSlug MUST be idempotent for already-normalized values.
// 📝️NormalizeLLMSlug normalizes the l l m slug to its canonical form.
func NormalizeLLMSlug(llm string) string {
	return strings.ToLower(workspace.Slugify(llm))
}

// 🏋️NormalizeEffortSlug normalizes the reasoning effort slug to its canonical form.
func NormalizeEffortSlug(effort string) string {
	return strings.ToLower(workspace.Slugify(effort))
}

// 🟥️NormalizeClientSlug MUST be idempotent for already-normalized values.
// ❓️NormalizeClientSlug normalizes the client slug to its canonical form.
func NormalizeClientSlug(client string) string {
	return strings.ToLower(workspace.Slugify(client))
}

// ❌️ResolveAllowedLLM MUST return an error for unrecognized values.
// 🔶️ResolveAllowedLLM resolves and validates the allowed l l m against known values.
func ResolveAllowedLLM(llm string) (string, error) {
	llmSlug := NormalizeLLMSlug(llm)
	bestMatch := ""
	for _, allowed := range AllowedLLMList() {
		if strings.Contains(llmSlug, NormalizeLLMSlug(allowed)) {
			if len(allowed) > len(bestMatch) {
				bestMatch = allowed
			}
		}
	}
	if bestMatch == "" {
		return "", fmt.Errorf("llm '%s' is not allowed. Please use one of: %s", llmSlug, strings.Join(AllowedLLMList(), ", "))
	}
	return bestMatch, nil
}

// 🏋️ResolveAllowedEffort resolves and validates the allowed reasoning effort against known values.
func ResolveAllowedEffort(effort string) (string, error) {
	if strings.TrimSpace(effort) == "" {
		return "", nil
	}
	effortSlug := NormalizeEffortSlug(effort)
	bestMatch := ""
	for _, allowed := range AllowedEffortList() {
		if effortSlug == NormalizeEffortSlug(allowed) || strings.Contains(effortSlug, NormalizeEffortSlug(allowed)) {
			if len(allowed) > len(bestMatch) {
				bestMatch = allowed
			}
		}
	}
	if bestMatch == "" {
		return "", fmt.Errorf("effort '%s' is not allowed. Please use one of: %s", effortSlug, strings.Join(AllowedEffortList(), ", "))
	}
	return bestMatch, nil
}

// 🟧️ResolveAllowedClient MUST return an error for unrecognized values.
// 🔹️ResolveAllowedClient resolves and validates the allowed client against known values.
func ResolveAllowedClient(client string) (string, error) {
	uiSlug := NormalizeClientSlug(client)
	bestMatch := ""
	for _, allowed := range AllowedClientList() {
		if strings.Contains(uiSlug, NormalizeClientSlug(allowed)) {
			if len(allowed) > len(bestMatch) {
				bestMatch = allowed
			}
		}
	}
	if bestMatch == "" {
		return "", fmt.Errorf("client '%s' is not allowed. Please use one of: %s", uiSlug, strings.Join(AllowedClientList(), ", "))
	}
	return bestMatch, nil
}

// 💿️Range holds the data fields for a range record.
type Range struct {
	Start int `json:"start"`
	End   int `json:"end"`
}

// 🟨️LineMetrics holds the data fields for a line metrics record.
type LineMetrics struct {
	Added   int `yaml:"added" json:"added"`
	Removed int `yaml:"removed" json:"removed"`
}

// 🟩️DiffLines holds the data fields for a diff lines record.
type DiffLines struct {
	Added   []int
	Removed []int
}

// 🟦️CountMetrics holds the data fields for a count metrics record.
type CountMetrics struct {
	Added   int `json:"added"`
	Updated int `json:"updated"`
	Removed int `json:"removed"`
}

// 🤝️ContributorIcons holds the data fields for a contributor icons record.
type ContributorIcons struct {
	Avatar      *string `json:"avatar,omitempty"`
	AvatarRound *string `json:"avatarRound,omitempty"`
	Github      *string `json:"github,omitempty"`
}

// 🔗️ContributorLink holds the data fields for a contributor link record.
type ContributorLink struct {
	Name string `json:"name"`
	URL  string `json:"url"`
}

// 🟪️TicketDate holds the data fields for a ticket date record.
type TicketDate struct {
	Created  time.Time  `json:"created"`
	Finished *time.Time `json:"finished,omitempty"`
}

// 📑️TicketSectionMetrics holds the data fields for a ticket section metrics record.
type TicketSectionMetrics struct {
	Range       *Range       `json:"range,omitempty"`
	Definitions []string     `json:"definitions,omitempty"`
	Lines       *LineMetrics `json:"lines,omitempty"`
}

// 📍️TicketFileMetricsEntry holds the data fields for a ticket file metrics entry record.
type TicketFileMetricsEntry struct {
	Path     string                          `json:"path"`
	Lines    *LineMetrics                    `json:"lines,omitempty"`
	Sections map[string]TicketSectionMetrics `json:"sections,omitempty"`
}

// 🔬️AnalyzeMetrics holds the data fields for a analyze metrics record.
type AnalyzeMetrics struct {
	Total       int            `json:"total"`
	ByPriority  *PriorityCount `json:"byPriority"`
	Autofixable int            `json:"autofixable"`
}

// 🟫️PriorityCount holds the data fields for a priority count record.
type PriorityCount struct {
	High   int `json:"high"`
	Medium int `json:"medium"`
	Low    int `json:"low"`
}

// 💠️Repo holds the data fields for a repo record.
type Repo struct {
	ID           string       `json:"id"`
	Name         string       `json:"name"`
	Path         string       `json:"path"`
	Technologies []Technology `json:"technologies"`
	Bundles      []Bundle     `json:"bundles"`
}

// 🌿️IsNode MUST return true only when the condition is met.
// 🐙️IsNode reports whether the Repo is node.
func (r *Repo) IsNode() {}

// 🏪️GetID MUST return the stored value without modification.
// 🔺️GetID returns the i d of the Repo.
func (r *Repo) GetID() string { return EmojiText(EmojiRepo) }

// 🔳️GetURI MUST return the stored value without modification.
// 🌐️GetURI returns the u r i of the Repo.
func (r *Repo) GetURI() string { return "repo://root" }

// 🛠️TechnologyKind represents a technology kind value.
type TechnologyKind string

const TechnologyKindUser TechnologyKind = "👤️"

const TechnologyKindInfrastructure TechnologyKind = "🧰️"

const TechnologyKindResearch TechnologyKind = "🔬️"

const TechnologyKindMono TechnologyKind = "🌱️"

// 🔲️String MUST return the canonical string value.
// 📺️String returns the string representation of the TechnologyKind.
func (e TechnologyKind) String() string {
	return string(e)
}

// 📦️BundleKind represents a bundle kind value.
type BundleKind string

const BundleKindLibrary BundleKind = "library"

const BundleKindSchema BundleKind = "schema"

const BundleKindBinary BundleKind = "binary"

const BundleKindUI BundleKind = "ui"

const BundleKindSite BundleKind = "site"

const BundleKindAssets BundleKind = "assets"

const BundleKindRepo BundleKind = "repo"

// ▪️IsValid MUST return true only when the condition is met.
// 🔻️IsValid reports whether the BundleKind is valid.
func (e BundleKind) IsValid() bool {
	switch e {
	case BundleKindLibrary, BundleKindSchema, BundleKindBinary, BundleKindUI, BundleKindSite, BundleKindAssets, BundleKindRepo:
		return true
	}
	return false
}

// ▫️String MUST return the canonical string value.
// ⬜️String returns the string representation of the BundleKind.
func (e BundleKind) String() string {
	return string(e)
}

// 🏷️DeriveTechnologyKind MUST return a valid value for any recognized input.
// 🟥️DeriveTechnologyKind infers and returns the technology kind from the given input.
func DeriveTechnologyKind(name string) TechnologyKind {
	switch name {
	case "compose":
		return TechnologyKindUser
	case "repo":
		return TechnologyKindInfrastructure
	case "coda":
		return TechnologyKindResearch
	}
	if strings.HasPrefix(name, "@") {
		return DeriveTechnologyKind(strings.TrimPrefix(name, "@"))
	}
	return TechnologyKindUser
}

// ◾DeriveBundleKind MUST return a valid value for any recognized input.
// 🟧️DeriveBundleKind infers and returns the bundle kind from the given input.
func DeriveBundleKind(name string, root string) BundleKind {
	absRoot := root
	if !filepath.IsAbs(absRoot) {
		absRoot = filepath.Join(workspace.GetRootDir(), root)
	}
	for _, configName := range []string{"package.json", "project.json"} {
		configPath := filepath.Join(absRoot, configName)
		if workspace.FileExists(configPath) {
			content, err := workspace.ReadTextFile(configPath)
			if err == nil {
				var meta struct {
					BundleKind string `json:"bundleKind"`
				}
				if json.Unmarshal([]byte(content), &meta) == nil && meta.BundleKind != "" {
					kind := BundleKind(strings.ToLower(meta.BundleKind))
					if kind.IsValid() {
						return kind
					}
				}
			}
		}
	}
	return BundleKindLibrary
}

// 📜️Technology holds the data fields for a technology record.
type Technology struct {
	Name    string         `json:"name"`
	Root    string         `json:"root"`
	Kind    TechnologyKind `json:"kind"`
	Emoji   string         `json:"emoji"`
	Bundles []Bundle       `json:"bundles"`
}

// ◽IsNode MUST return true only when the condition is met.
// 🔳️IsNode reports whether the Technology is node.
func (p *Technology) IsNode() {}

// ◻GetID MUST return the stored value without modification.
// 🔲️GetID returns the i d of the Technology.
func (p *Technology) GetID() string {
	emoji := p.Emoji
	if emoji == "" {
		emoji = string(p.Kind)
	}
	if emoji == "" {
		emoji = string(DeriveTechnologyKind(p.Name))
	}
	return EmojiText(emoji) + workspace.Flat(p.Name)
}

// ◼GetURI MUST return the stored value without modification.
// ▪️GetURI returns the u r i of the Technology.
func (p *Technology) GetURI() string {
	return "repo://technology/" + p.GetID()
}

// 🔵️Bundle holds the data fields for a bundle record.
type Bundle struct {
	Name           string     `json:"name"`
	Root           string     `json:"root"`
	SourceRoot     string     `json:"sourceRoot,omitempty"`
	TechnologyName string     `json:"technologyName"`
	Tags           []string   `json:"tags,omitempty"`
	Kind           BundleKind `json:"kind"`
	Emoji          string     `json:"emoji"`
	Packages       []Package  `json:"packages,omitempty"`
}

// 🔴️Package holds the data fields for a package record.
type Package struct {
	Name    string `json:"name"`
	Version string `json:"version"`
	Path    string `json:"path"`
	Kind    string `json:"kind"`
}

// 🟠️IsNode MUST return true only when the condition is met.
// ▫️IsNode reports whether the Bundle is node.
func (b *Bundle) IsNode() {}

// 🟡️GetID MUST return the stored value without modification.
// ◾GetID returns the i d of the Bundle.
func (b *Bundle) GetID() string {
	emoji := b.Emoji
	if emoji == "" {
		switch string(b.Kind) {
		case "schema":
			emoji = EmojiBundleSchema
		case "binary":
			emoji = EmojiBundleBinary
		case "ui":
			emoji = EmojiBundleUI
		case "site":
			emoji = EmojiBundleSite
		case "assets":
			emoji = EmojiBundleAssets
		case "library":
			emoji = EmojiBundleLibrary
		case "example":
			emoji = EmojiBundleExample
		case "repo":
			emoji = EmojiBundleRepo
		}
	}
	parts := strings.SplitN(b.Name, "/", 2)
	technologyCode := parts[0]
	bundleCode := technologyCode
	if len(parts) > 1 {
		bundleCode = parts[1]
	}
	techEmoji := resolveTechnologyEmoji(technologyCode)
	return EmojiText(techEmoji) + workspace.Flat(technologyCode) + EmojiText(emoji) + workspace.Flat(bundleCode)
}

// 🟢️GetURI MUST return the stored value without modification.
// ◽GetURI returns the u r i of the Bundle.
func (b *Bundle) GetURI() string {
	return "repo://bundle/" + b.GetID()
}

// 🟣️normalizeBundleLabel holds the data fields for a normalizeBundleLabel record.
func NormalizeBundleLabel(name string) string {
	if name == "" {
		return ""
	}
	if strings.HasPrefix(name, "@") {
		return name
	}
	if name == "vscode" {
		return "repo/vscode"
	}
	if name == "repo" {
		return "repo/go"
	}
	return "compose/" + name
}

// 🟤️normalizeBundleID holds the data fields for a normalizeBundleID record.
func NormalizeBundleID(name string) string {
	return NormalizeBundleLabel(name)
}

// 🔧️bundlePathPrefix holds the data fields for a bundlePathPrefix record.
func bundlePathPrefix(name string) string {
	if name == "" {
		return ""
	}
	if name == "repo" {
		return ""
	}
	if strings.HasPrefix(name, "compose/") {
		return strings.TrimPrefix(name, "compose/") + "/"
	}
	return name + "/"
}

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
var folderKindCache sync.Map

// 🩵️DeriveFolderKind holds the data fields for a DeriveFolderKind record.
func DeriveFolderKind(path string) FolderKind {
	base := filepath.Base(path)
	if strings.HasPrefix(base, ".") {
		return FolderKindRequired
	}

	if val, ok := folderKindCache.Load(path); ok {
		return val.(FolderKind)
	}

	kind := FolderKindOrganization
	requiredIndicators := []string{"package.json", "pyproject.toml", "go.mod", "Cargo.toml"}
	for _, indicator := range requiredIndicators {
		if workspace.FileExists(filepath.Join(workspace.GetRootDir(), path, indicator)) {
			kind = FolderKindRequired
			break
		}
	}

	if kind == FolderKindOrganization {
		dir := filepath.Join(workspace.GetRootDir(), path)
		if entries, err := os.ReadDir(dir); err == nil {
			for _, entry := range entries {
				if !entry.IsDir() {
					ext := filepath.Ext(entry.Name())
					if ext == ".csproj" || ext == ".sln" {
						kind = FolderKindRequired
						break
					}
				}
			}
		}
	}

	folderKindCache.Store(path, kind)
	return kind
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
	return BuildFolderID(f.Path, f.BundleID)
}

// 💚️GetURI MUST return the stored value without modification.
// ⚫️GetURI returns the u r i of the Folder.
func (f *Folder) GetURI() string {
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

const EmojiTechnologyUser = "👤️"

const EmojiTechnologyInfra = "🧰️"

const EmojiTechnologyResearch = "🔬️"

const EmojiTechnologyMono = "🌱️"

const EmojiBundleLibrary = "📚️"

const EmojiBundleSchema = "🛂️"

const EmojiBundleBinary = "⌨️"

const EmojiBundleUI = "🖱️"

const EmojiBundleExample = "📔️"

const EmojiBundleSite = "🌐️"

const EmojiBundleAssets = "🏪️"

const EmojiBundleRepo = "🪆️"

const EmojiFolderOrg = "🗃️"

const EmojiFolderRequired = "🛅️"

const EmojiFolderRoot = "🌱️"

const EmojiFileCode = "💻️"

const EmojiFileLab = "🥼️"

const EmojiFileScript = "📜️"

const EmojiFileDocs = "📃️"

const EmojiFileConfig = "⚙️"

const EmojiFileResource = "💾️"

const EmojiFileTemplate = "📋️"

const EmojiFileLicense = "⚖️"

const EmojiLine = "📌️"

const EmojiSection = "🔖️"

const EmojiDefinitionImpl = "🛠️"

const EmojiDefinitionInterface = "✂️"

const EmojiDefinitionConstant = "🪨️"

const EmojiDefinitionTest = "🧪️"

const EmojiYear = "🎆️"

const EmojiMonth = "🌙️"

const EmojiDay = "☀️"

const EmojiHour = "⏰️"

const EmojiMinute = "⌚️"

const EmojiSecond = "⏱️"

const EmojiGoal = "🎯️"

const EmojiTicket = "🎫️"

const EmojiDraft = "📝️"

const EmojiTodo = "📝️"

const EmojiPolicy = "👮️"

const EmojiBreach = "🚫️"

const EmojiBreachScope = "🔍️"

const EmojiContributor = "🧑️‍💻️"

const EmojiCheckpoint = "🔀️"

const EmojiInteractionStarted = "🌱️"

const EmojiInteractionEdited = "✏️"

const EmojiInteractionFinished = "✅️"

const EmojiInteractionRestarted = "🔁️"

const EmojiInteractionDeleted = "🗑️"

const EmojiSession = "⚪️"

const EmojiSessionRunning = "🟡️"

const EmojiSessionCompleted = "🟢️"

const EmojiSessionInterrupted = "🔴️"

var EmojiRepo = ""

var EmojiCodebase = "🖥️"

var EmojiTechnologies = "🏗️"

var EmojiBundles = "📦️"

var EmojiFolders = "📁️"

var EmojiFiles = "📄️"

var EmojiSections = "🔖️"

var EmojiDefinitions = "🏷️"

var EmojiYears = "🎆️"

var EmojiMonths = "🌙️"

var EmojiDays = "☀️"

var EmojiHours = "⏰️"

var EmojiMinutes = "⌚️"

var EmojiSeconds = "⏱️"

var EmojiTickets = "🎫️"

var EmojiGoals = "🎯️"

var EmojiDrafts = "📝️"

var EmojiTodos = "📝️"

var EmojiPolicies = "👮️"

var EmojiBreaches = "🚫️"

var EmojiContributors = "🧑️‍💻️"

var EmojiCheckpoints = "🔀️"

var EmojiSessions = "⚪️"

var EmojiInteractions = ""

var EmojiTerritorys = ""

var EmojiTerritory = ""

var EmojiStatutes = ""

var EmojiStatute = ""

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
	return BuildFileID(f.Path, f.BundleID)
}

// 🤎️GetURI MUST return the stored value without modification.
// 💛️GetURI returns the u r i of the File.
func (f *File) GetURI() string {
	return BuildFileUriFromPath(f.Path)
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
		fileID := BuildFileID(s.FilePath, nil)
		if s.Path != "" {
			return BuildSectionID(fileID, strings.Split(strings.ReplaceAll(s.Path, "#", "/"), "/"))
		}
		segment := s.Name
		if s.Emoji != "" {
			segment = s.Emoji + s.Name
		}
		return BuildSectionID(fileID, []string{segment})
	}
	emoji := s.Emoji
	if emoji == "" {
		emoji = EmojiSection
	}
	return EmojiText(emoji) + workspace.Flat(s.Name)
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
		fileID := BuildFileID(d.FilePath, nil)
		var sectionParts []string
		if d.SectionPath != "" {
			sectionParts = strings.Split(strings.ReplaceAll(d.SectionPath, "#", "/"), "/")
		}
		return BuildDefinitionID(fileID, sectionParts, d.Name, d.Kind)
	}
	data := map[string]interface{}{"kind": string(d.Kind)}
	return definitionKindEmoji(data) + workspace.Flat(d.Name)
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

// 🌳️ContributorContributionsTree holds the data fields for a contributor contributions tree record.
type ContributorContributionsTree struct {
	Checkpoints []*Checkpoint
	Tickets     []*Ticket
	Bundles     []*ContributorBundle
}

// 🔖️ContributorBundle holds the data fields for a contributor bundle record.
type ContributorBundle struct {
	Name    string
	Lines   LineMetrics
	Folders []*ContributorFolder
}

// 🔖️ContributorFolder holds the data fields for a contributor folder record.
type ContributorFolder struct {
	Name  string
	Lines LineMetrics
	Files []*ContributorFile
}

// 🔖️ContributorFile holds the data fields for a contributor file record.
type ContributorFile struct {
	Name     string
	Lines    LineMetrics
	Sections []*ContributorSection
}

// 🔖️ContributorSection holds the data fields for a contributor section record.
type ContributorSection struct {
	Name        string
	Lines       LineMetrics
	Definitions []*ContributorDefinition
}

// 🔖️ContributorDefinition holds the data fields for a contributor definition record.
type ContributorDefinition struct {
	Name  string
	Lines LineMetrics
}

// 🔖️IsNode MUST return true only when the condition is met.
// 💝️IsNode reports whether the Contributor is node.
func (c *Contributor) IsNode() {}

// 🔖️GetID MUST return the stored value without modification.
// 💘️GetID returns the i d of the Contributor.
func (c *Contributor) GetID() string {
	return EmojiText(EmojiContributor) + workspace.Flat(c.Alias)
}

// 🔖️GetURI MUST return the stored value without modification.
// 🏵️GetURI returns the u r i of the Contributor.
func (c *Contributor) GetURI() string {
	return "repo://contributor/" + c.GetID()
}

// ✔️Checkpoint holds the data fields for a checkpoint record.
type Checkpoint struct {
	ID       string  `json:"id"`
	SHA      string  `json:"sha"`
	Title    string  `json:"title"`
	AuthorID *string `json:"authorId,omitempty"`
	Date     string  `json:"date"`
}

// 🔖️IsNode MUST return true only when the condition is met.
// 🔢️IsNode reports whether the Checkpoint is node.
func (c *Checkpoint) IsNode() {}

// #endregion 💡️GraphQL Types

// #region 🎨️Drafts

// 💿️Draft holds the data fields for a draft record.
type Draft struct {
	ID string `json:"id"`
}

// 🏪️GetID MUST return the stored value without modification.
// 📖️GetID returns the i d of the Draft.
func (d *Draft) GetID() string {
	return EmojiText(EmojiDraft) + workspace.Flat(d.ID)
}

// 🔗️GetURI MUST return the stored value without modification.
// 🌐️GetURI returns the u r i of the Draft.
func (d *Draft) GetURI() string {
	return "repo://draft/" + d.GetID()
}

// #endregion 🎨️Drafts

// #region 💡️GraphQL Types

// 🔖️GetID MUST return the stored value without modification.
// 🌸️GetID returns the i d of the Checkpoint.
func (c *Checkpoint) GetID() string { return EmojiText(EmojiCheckpoint) + c.SHA }

// 🔖️GetURI MUST return the stored value without modification.
// 🌺️GetURI returns the u r i of the Checkpoint.
func (c *Checkpoint) GetURI() string {
	return "repo://checkpoint/" + c.GetID()
}

// 🧷️TicketPlan records an IDE-native plan or spec file to archive into the ticket folder on close.
type TicketPlan struct {
	Client string `json:"client,omitempty" yaml:"client,omitempty"`
	ID     string `json:"id,omitempty" yaml:"id,omitempty"`
	Source string `json:"source,omitempty" yaml:"source,omitempty"`
	Local  string `json:"local,omitempty" yaml:"local,omitempty"`
}

// 🔖️Ticket holds the data fields for a ticket record.
type Ticket struct {
	Year          int                   `json:"-" yaml:"-"`
	Month         int                   `json:"-" yaml:"-"`
	Day           int                   `json:"-" yaml:"-"`
	Slug          string                `json:"-" yaml:"-"`
	Title         string                `json:"title" yaml:"title"`
	Emoji         string                `json:"emoji,omitempty" yaml:"emoji,omitempty"`
	Status        TicketStatus          `json:"status" yaml:"status"`
	Description   string                `json:"description,omitempty" yaml:"description,omitempty"`
	Summary       string                `json:"summary,omitempty" yaml:"summary,omitempty"`
	Management    *TicketManagementData `json:"github,omitempty" yaml:"github,omitempty"`
	Goal          string                `json:"goal,omitempty" yaml:"goal,omitempty"`
	Parent        string                `json:"-" yaml:"-"`
	Plan          *TicketPlan           `json:"plan,omitempty" yaml:"plan,omitempty"`
	Sessions      []string              `json:"sessions,omitempty" yaml:"sessions,omitempty"`
	Interactions  []Interaction         `json:"-" yaml:"-"`
	Agents        []TicketAgent         `json:"-" yaml:"-"`
	FolderPath    string                `json:"-" yaml:"-"`
	JsonPath      string                `json:"-" yaml:"-"`
	ImportantPath string                `json:"-" yaml:"-"`
}

func AppendTicketSessionID(ticket *Ticket, sessionID string) {
	sessionID = strings.TrimSpace(sessionID)
	if ticket == nil || sessionID == "" {
		return
	}
	for _, existing := range ticket.Sessions {
		if existing == sessionID {
			return
		}
	}
	ticket.Sessions = append(ticket.Sessions, sessionID)
}

func isTicketInteractionKind(kind string, expected string) bool {
	kind = strings.TrimSpace(kind)
	expected = strings.TrimSpace(expected)
	if kind == expected {
		return true
	}
	if strings.HasSuffix(kind, ".ended") {
		return strings.TrimSuffix(kind, ".ended") == expected
	}
	return false
}

// 📋️UnmarshalJSON MUST require an explicit valid ticket status. Every member is read exactly as
// the document states it: the goal reference, the summary and every contributor reference stay the
// text `🧬️schema/🔣️.json` declares, so a decode never depends on the developer's own checkout.
func (t *Ticket) UnmarshalJSON(data []byte) error {
	type TicketAlias struct {
		Title       string                `json:"title"`
		Emoji       string                `json:"emoji,omitempty"`
		Description string                `json:"description,omitempty"`
		Status      TicketStatus          `json:"status"`
		Summary     string                `json:"summary,omitempty"`
		Management  *TicketManagementData `json:"github,omitempty"`
		Goal        string                `json:"goal,omitempty"`
		Parent      string                `json:"parent,omitempty"`
	}
	aux := &struct {
		TicketAlias
		Plan               *TicketPlan     `json:"plan,omitempty"`
		PromptLegacy       string          `json:"prompt"`
		InteractionsLegacy []Interaction   `json:"interactions"`
		AgentsLegacy       []TicketAgent   `json:"agents"`
		SessionsRaw        json.RawMessage `json:"sessions"`
	}{}
	if err := json.Unmarshal(data, aux); err != nil {
		return err
	}
	if !aux.Status.IsValid() {
		return fmt.Errorf("ticket status must be explicitly \"open\" or \"closed\"")
	}
	*t = Ticket{
		Title:       aux.Title,
		Emoji:       aux.Emoji,
		Description: aux.Description,
		Status:      aux.Status,
		Summary:     aux.Summary,
		Management:  aux.Management,
		Goal:        aux.Goal,
		Parent:      aux.Parent,
		Plan:        aux.Plan,
	}
	if t.Description == "" && aux.PromptLegacy != "" {
		t.Description = aux.PromptLegacy
	}
	if len(aux.InteractionsLegacy) > 0 {
		t.Interactions = aux.InteractionsLegacy
	}
	if len(aux.AgentsLegacy) > 0 {
		t.Agents = aux.AgentsLegacy
	}
	if len(aux.SessionsRaw) > 0 {
		var sessionIDs []string
		if err := json.Unmarshal(aux.SessionsRaw, &sessionIDs); err == nil {
			for _, sessionID := range sessionIDs {
				AppendTicketSessionID(t, sessionID)
			}
		} else {
			var legacyAgents []TicketAgent
			if err := json.Unmarshal(aux.SessionsRaw, &legacyAgents); err == nil {
				for _, legacyAgent := range legacyAgents {
					AppendTicketSessionID(t, legacyAgent.Session)
				}
				if len(t.Agents) == 0 {
					t.Agents = legacyAgents
				}
			}
		}
	}
	for _, agent := range t.Agents {
		AppendTicketSessionID(t, agent.Session)
	}
	return nil
}

// 😀️MarshalJSON writes every member back exactly as it stands, refusing a ticket whose status is
// not one of the two the schema allows.
func (t Ticket) MarshalJSON() ([]byte, error) {
	if !t.Status.IsValid() {
		return nil, fmt.Errorf("ticket status must be explicitly \"open\" or \"closed\"")
	}
	type TicketAlias Ticket
	return json.Marshal(TicketAlias(t))
}

// 🔖️IsNode MUST return true only when the condition is met.
// 🌻️IsNode reports whether the Ticket is node.
func (t *Ticket) IsNode() {}

// 🔖️GetID MUST return the stored value without modification.
// 🌼️GetID returns the i d of the Ticket.
func (t *Ticket) GetID() string {
	return EmojiText(EmojiTicket) + workspace.Flat(t.Slug)
}

// 🔖️GetURI MUST return the stored value without modification.
// 🌷️GetURI returns the u r i of the Ticket.
func (t *Ticket) GetURI() string {
	data := map[string]interface{}{"slug": t.Slug}
	if t.Goal != "" {
		data["goalId"] = t.Goal
	}
	return GetArtifactURI("ticket", data)
}

// 📌️GetTitle MUST return the stored value without modification.
// 🌹️GetTitle returns the title of the Ticket.
func (t *Ticket) GetTitle() string {
	return t.Title
}

// 🔖️GetPrompt MUST return the description or the first interaction prompt.
// 🥀️GetPrompt returns the prompt of the Ticket.
func (t *Ticket) GetPrompt() string {
	if t.Description != "" {
		return t.Description
	}
	if len(t.Interactions) > 0 {
		return t.Interactions[0].Prompt
	}
	return ""
}

// 🪪️GetLatestPrompt MUST return the latest prompt from sessions or interactions.
// 🧪️GetLatestPrompt returns the latest prompt of the Ticket.
func (t *Ticket) GetLatestPrompt() string {
	if len(t.Interactions) > 0 {
		return t.Interactions[len(t.Interactions)-1].Prompt
	}
	return t.Description
}

// 🧪️GetLLM MUST return the LLM from the latest session or interaction.
// 🪻️GetLLM returns the l l m of the Ticket.
func (t *Ticket) GetLLM() string {
	if len(t.Interactions) > 0 {
		return t.Interactions[len(t.Interactions)-1].LLM
	}
	if len(t.Agents) > 0 {
		return t.Agents[len(t.Agents)-1].LLM
	}
	return ""
}

// 🏋️GetEffort MUST return the reasoning effort from the latest session or interaction.
// 🏋️GetEffort returns the reasoning effort of the Ticket.
func (t *Ticket) GetEffort() string {
	if len(t.Interactions) > 0 {
		return t.Interactions[len(t.Interactions)-1].Effort
	}
	if len(t.Agents) > 0 {
		return t.Agents[len(t.Agents)-1].Effort
	}
	return ""
}

// 🔖️GetClient MUST return the client from the latest session or interaction.
// 🪷️GetClient returns the client of the Ticket.
func (t *Ticket) GetClient() string {
	if len(t.Interactions) > 0 {
		return t.Interactions[len(t.Interactions)-1].Client
	}
	if len(t.Agents) > 0 {
		return t.Agents[len(t.Agents)-1].Client
	}
	return ""
}

// 🔖️GetStatus MUST return the stored value without modification.
// 🍁️GetStatus returns the status of the Ticket.
func (t *Ticket) GetStatus() TicketStatus {
	return t.Status
}

// ✍️GetAuthor MUST return the author from the first session or interaction.
// 🔐️GetAuthor returns the author of the Ticket.
func (t *Ticket) GetAuthor() string {
	if len(t.Interactions) > 0 {
		return t.Interactions[0].Author
	}
	if len(t.Agents) > 0 {
		return t.Agents[0].Contributor
	}
	return ""
}

// 🔖️GetCheckpoint MUST return the checkpoint from the first interaction.
// 🍂️GetCheckpoint returns the checkpoint of the Ticket.
func (t *Ticket) GetCheckpoint() string {
	if len(t.Interactions) > 0 {
		return t.Interactions[0].Checkpoint
	}
	return ""
}

// 🔖️GetSummary MUST return the stored value without modification.
// 🍃️GetSummary returns the summary of the Ticket.
func (t *Ticket) GetSummary() string {
	if t.Summary != "" {
		return t.Summary
	}
	for i := len(t.Interactions) - 1; i >= 0; i-- {
		if isTicketInteractionKind(t.Interactions[i].Kind, "ticket.close") && t.Interactions[i].Summary != "" {
			return t.Interactions[i].Summary
		}
	}
	return ""
}

// ▶️GetDateStarted MUST return the earliest date from interactions or sessions.
// 🌿️GetDateStarted returns the date started of the Ticket.
func (t *Ticket) GetDateStarted() time.Time {
	for _, interaction := range t.Interactions {
		if isTicketInteractionKind(interaction.Kind, "ticket.open") && interaction.Date != "" {
			if parsed, err := time.Parse("2006-01-02 15:04:05", interaction.Date); err == nil {
				return parsed
			}
			if parsed, err := time.Parse(time.RFC3339, interaction.Date); err == nil {
				return parsed
			}
		}
	}
	if len(t.Interactions) > 0 && t.Interactions[0].Date != "" {
		if parsed, err := time.Parse("2006-01-02 15:04:05", t.Interactions[0].Date); err == nil {
			return parsed
		}
		if parsed, err := time.Parse(time.RFC3339, t.Interactions[0].Date); err == nil {
			return parsed
		}
	}
	return time.Date(t.Year, time.Month(t.Month), t.Day, 0, 0, 0, 0, time.UTC)
}

// 🔖️GetDateFinished MUST return the stored value without modification.
// ⏹️GetDateFinished returns the date finished of the Ticket.
func (t *Ticket) GetDateFinished() *time.Time {
	for i := len(t.Interactions) - 1; i >= 0; i-- {
		if isTicketInteractionKind(t.Interactions[i].Kind, "ticket.close") && t.Interactions[i].Date != "" {
			if parsed, err := time.Parse("2006-01-02 15:04:05", t.Interactions[i].Date); err == nil {
				return &parsed
			}
			if parsed, err := time.Parse(time.RFC3339, t.Interactions[i].Date); err == nil {
				return &parsed
			}
		}
	}
	return nil
}

// 🔖️GetInteractionFiles MUST return the stored value without modification.
// ☘️GetInteractionFiles returns all unique InteractionFile entries across all interactions.
func (t *Ticket) GetInteractionFiles() []InteractionFile {
	seen := make(map[string]struct{})
	var result []InteractionFile
	for _, interaction := range t.Interactions {
		for _, f := range interaction.Files {
			if _, ok := seen[f.Path]; ok {
				continue
			}
			seen[f.Path] = struct{}{}
			result = append(result, f)
		}
	}
	return result
}

// 🔖️TicketBundleContrib holds the data fields for a ticket bundle contrib record.
type TicketBundleContrib struct {
	BundleID string              `json:"bundleId"`
	Files    []TicketFileContrib `json:"files"`
}

// 🔖️TicketFileContrib holds the data fields for a ticket file contrib record.
type TicketFileContrib struct {
	FileID   string                 `json:"fileId"`
	Sections []TicketSectionContrib `json:"sections"`
}

// 🔖️TicketSectionContrib holds the data fields for a ticket section contrib record.
type TicketSectionContrib struct {
	SectionID   string       `json:"sectionId"`
	Definitions []string     `json:"definitions"`
	Metrics     *LineMetrics `json:"metrics"`
}

// 🔖️Policy holds the data fields for a policy record.
type Policy struct {
	ID          string         `json:"id"`
	Name        string         `json:"name"`
	Description *string        `json:"description,omitempty"`
	Scopes      []string       `json:"scopes"`
	Groups      []Territory    `json:"groups"`
	Statutes    []*StatuteMeta `json:"statutes"`
}

// 🔖️IsNode MUST return true only when the condition is met.
// 🍀️IsNode reports whether the Policy is node.
func (p *Policy) IsNode() {}

// 🔖️GetID MUST return the stored value without modification.
// 🪴️GetID returns the i d of the Policy.
func (p *Policy) GetID() string {
	slug := strings.TrimPrefix(p.ID, "/")
	return EmojiText(EmojiPolicy) + workspace.Flat(slug)
}

// 🔖️GetURI MUST return the stored value without modification.
// 🌱️GetURI returns the u r i of the Policy.
func (p *Policy) GetURI() string {
	return "repo://policy/" + p.GetID()
}

// 🔖️StatuteMeta holds the data fields for a statute meta record.
type StatuteMeta struct {
	Kind        Statute        `json:"kind"`
	PolicyID    string         `json:"policyId"`
	Priority    BreachPriority `json:"priority"`
	Reason      string         `json:"reason"`
	Solution    string         `json:"solution"`
	Autofixable bool           `json:"autofixable"`
}

func NormalizeStatuteMeta(meta *StatuteMeta) {
	if meta == nil {
		return
	}
	if meta.Priority == "" {
		meta.Priority = BreachPriorityLow
	}
}

// 🔖️IsNode MUST return true only when the condition is met.
// 📰️IsNode reports whether the StatuteMeta is node.
func (v *StatuteMeta) IsNode() {}

// 🔖️GetID MUST return the stored value without modification.
// 🌲️GetID returns the i d of the StatuteMeta.
func (v *StatuteMeta) GetID() string {
	return fmt.Sprintf("%s%s", EmojiText(EmojiStatute), workspace.StatutePathToIdValue(string(v.Kind)))
}

// 🔖️GetURI MUST return the stored value without modification.
// 🔖️GetURI returns the u r i of the StatuteMeta.
func (v *StatuteMeta) GetURI() string {
	return "repo://statute/" + v.GetID()
}

// 🔖️AnalyzeResult holds the data fields for a analyze result record.
type AnalyzeResult struct {
	Breachs []*Breach       `json:"breachs"`
	Metrics *AnalyzeMetrics `json:"metrics"`
}

// 🔖️FixResult holds the data fields for a fix result record.
type FixResult struct {
	Fixed     int       `json:"fixed"`
	Remaining int       `json:"remaining"`
	Breachs   []*Breach `json:"breachs"`
}

// 🔖️ContributorContributions holds the data fields for a contributor contributions record.
type ContributorContributions struct {
	Bundles     []ContributionBundle     `json:"bundles"`
	Folders     []ContributionFolder     `json:"folders"`
	Files       []ContributionFile       `json:"files"`
	Sections    []ContributionSection    `json:"sections"`
	Definitions []ContributionDefinition `json:"definitions"`
}

// 🔖️ContributionBundle holds the data fields for a contribution bundle record.
type ContributionBundle struct {
	BundleID string        `json:"bundleId"`
	Metrics  *CountMetrics `json:"metrics"`
}

// 🔖️ContributionFolder holds the data fields for a contribution folder record.
type ContributionFolder struct {
	FolderID string        `json:"folderId"`
	Metrics  *CountMetrics `json:"metrics"`
}

// 🔖️ContributionFile holds the data fields for a contribution file record.
type ContributionFile struct {
	FileID  string       `json:"fileId"`
	Metrics *LineMetrics `json:"metrics"`
}

// 🔖️ContributionSection holds the data fields for a contribution section record.
type ContributionSection struct {
	SectionID string       `json:"sectionId"`
	Metrics   *LineMetrics `json:"metrics"`
}

// 🔖️ContributionDefinition holds the data fields for a contribution definition record.
type ContributionDefinition struct {
	DefinitionID string       `json:"definitionId"`
	Metrics      *LineMetrics `json:"metrics"`
}

// ♻️SemanticChangeType represents a semantic change type value.
type SemanticChangeType string

const SemanticChangeAdded SemanticChangeType = "added"

const SemanticChangeDeleted SemanticChangeType = "deleted"

const SemanticChangeModified SemanticChangeType = "modified"

const SemanticChangeRenamed SemanticChangeType = "renamed"

// 🔖️SemanticChange holds the data fields for a semantic change record.
type SemanticChange struct {
	Kind     string
	Status   SemanticChangeType
	Path     string
	FromPath string
	ToPath   string
	Lines    LineMetrics
}

// 🔖️mergeLineMetrics holds the data fields for a mergeLineMetrics record.
func mergeLineMetrics(target *LineMetrics, add LineMetrics) {
	if target == nil {
		return
	}
	target.Added += add.Added
	target.Removed += add.Removed
}

func buildCodebasePathSet(codebase *Codebase) map[string]struct{} {
	result := make(map[string]struct{})
	if codebase == nil {
		return result
	}
	for _, bundle := range codebase.Bundles {
		result[bundle.ID] = struct{}{}
	}
	for _, folder := range codebase.Folders {
		result[folder.Path] = struct{}{}
	}
	for _, file := range codebase.Files {
		result[file.Path] = struct{}{}
	}
	for _, section := range codebase.Sections {
		result[section.Path] = struct{}{}
	}
	for _, def := range codebase.Definitions {
		result[def.Path] = struct{}{}
	}
	return result
}

// 🔖️extractFilePrefix holds the data fields for a extractFilePrefix record.
func ExtractFilePrefix(path string) string {
	if path == "" {
		return ""
	}
	if idx := strings.Index(path, "#"); idx != -1 {
		return path[:idx]
	}
	if idx := strings.Index(path, "§"); idx != -1 {
		return path[:idx]
	}
	return path
}

// #endregion 💡️GraphQL Types

// #region 🎡️GraphQL Input Types

// 💿️FileListInput holds the data fields for a file list input record.
type FileListInput struct {
	Updated []string `json:"updated,omitempty"`
	Created []string `json:"created,omitempty"`
	Removed []string `json:"removed,omitempty"`
}

// 🎫️TicketOpenInput holds the data fields for a ticket open input record.
type TicketOpenInput struct {
	Emoji        string `json:"emoji"`
	Title        string `json:"title"`
	Prompt       string `json:"prompt"`
	LLM          string `json:"llm,omitempty"`
	Effort       string `json:"effort,omitempty"`
	Client       string `json:"client"`
	NoIssue      bool   `json:"noIssue,omitempty"`
	Draft        string `json:"draft,omitempty"`
	Goal         string `json:"goal"`
	Parent       string `json:"parent,omitempty"`
	NoManagement bool   `json:"noManagement,omitempty"`
	Issue        string `json:"issue,omitempty"`
	PlanID       string `json:"planId,omitempty"`
	SpecID       string `json:"specId,omitempty"`
}

// 🆕️DraftCreateInput holds the data fields for a draft create input record.
type DraftCreateInput struct {
	Title string   `json:"title"`
	Files []string `json:"files,omitempty"`
}

// 📝️GoalCreateInput holds the data fields for a goal create input record.
type GoalCreateInput struct {
	Title        string `json:"title"`
	Description  string `json:"description"`
	Prompt       string `json:"prompt"`
	DueDate      string `json:"dueDate"`
	LLM          string `json:"llm"`
	Effort       string `json:"effort,omitempty"`
	Client       string `json:"client"`
	NoManagement bool   `json:"noManagement,omitempty"`
	Parent       string `json:"parent,omitempty"`
	Milestone    string `json:"milestone,omitempty"`
}

// ♻️GoalChangeInput holds the data fields for a goal change input record.
type GoalChangeInput struct {
	ID           string  `json:"id"`
	Title        *string `json:"title,omitempty"`
	Description  *string `json:"description,omitempty"`
	DueDate      *string `json:"dueDate,omitempty"`
	LLM          *string `json:"llm,omitempty"`
	Effort       *string `json:"effort,omitempty"`
	Parent       *string `json:"parent,omitempty"`
	NoManagement bool    `json:"noManagement,omitempty"`
}

// 📪️GoalCloseInput holds the data fields for a goal close input record.
type GoalCloseInput struct {
	ID           string `json:"id"`
	Summary      string `json:"summary"`
	NoManagement bool   `json:"noManagement,omitempty"`
}

// 🔓️GoalReopenInput holds the data fields for a goal reopen input record.
type GoalReopenInput struct {
	ID           string  `json:"id"`
	Prompt       string  `json:"prompt"`
	Client       string  `json:"client"`
	LLM          string  `json:"llm"`
	Effort       string  `json:"effort,omitempty"`
	Title        *string `json:"title,omitempty"`
	Description  *string `json:"description,omitempty"`
	DueDate      *string `json:"dueDate,omitempty"`
	Parent       *string `json:"parent,omitempty"`
	NoManagement bool    `json:"noManagement,omitempty"`
}

// 🗑️GoalDeleteInput holds the data fields for a goal delete input record.
type GoalDeleteInput struct {
	ID           string `json:"id"`
	NoManagement bool   `json:"noManagement,omitempty"`
}

// 🔷️TicketDeleteInput holds the data fields for a ticket delete input record.
type TicketDeleteInput struct {
	Year         int    `json:"year"`
	Month        int    `json:"month"`
	Day          int    `json:"day"`
	Slug         string `json:"slug"`
	NoManagement bool   `json:"noManagement,omitempty"`
}

// 🔶️TicketCloseInput holds the data fields for a ticket close input record.
type TicketCloseInput struct {
	Year         int      `json:"year"`
	Month        int      `json:"month"`
	Day          int      `json:"day"`
	Slug         string   `json:"slug"`
	Summary      string   `json:"summary"`
	Files        []string `json:"files"`
	Title        *string  `json:"title,omitempty"`
	NoManagement bool     `json:"noManagement,omitempty"`
	All          bool     `json:"all,omitempty"`
}

// 📬️TicketReopenInput holds the data fields for a ticket reopen input record.
type TicketReopenInput struct {
	Year         int     `json:"year"`
	Month        int     `json:"month"`
	Day          int     `json:"day"`
	Slug         string  `json:"slug"`
	Prompt       string  `json:"prompt"`
	LLM          string  `json:"llm,omitempty"`
	Effort       string  `json:"effort,omitempty"`
	Client       string  `json:"client"`
	Title        *string `json:"title,omitempty"`
	Draft        string  `json:"draft,omitempty"`
	Goal         string  `json:"goal,omitempty"`
	Parent       string  `json:"parent,omitempty"`
	NoManagement bool    `json:"noManagement,omitempty"`
	PlanID       string  `json:"planId,omitempty"`
	SpecID       string  `json:"specId,omitempty"`
}

// 🔹️TicketChangeInput holds the data fields for a ticket change input record.
type TicketChangeInput struct {
	Year         int     `json:"year"`
	Month        int     `json:"month"`
	Day          int     `json:"day"`
	Slug         string  `json:"slug"`
	Title        *string `json:"title,omitempty"`
	Prompt       *string `json:"prompt,omitempty"`
	LLM          *string `json:"llm,omitempty"`
	Effort       *string `json:"effort,omitempty"`
	Client       *string `json:"client,omitempty"`
	Goal         *string `json:"goal,omitempty"`
	Parent       *string `json:"parent,omitempty"`
	NoManagement bool    `json:"noManagement,omitempty"`
}

// 🤝️ContributorAddInput holds the data fields for a contributor add input record.
type ContributorAddInput struct {
	Github       string   `json:"github"`
	Name         *string  `json:"name,omitempty"`
	Names        []string `json:"names,omitempty"`
	Email        *string  `json:"email,omitempty"`
	Emails       []string `json:"emails,omitempty"`
	Fingerprint  *string  `json:"fingerprint,omitempty"`
	Fingerprints []string `json:"fingerprints,omitempty"`
}

// 🧹️FilterInput holds the data fields for a filter input record.
type FilterInput struct {
	Filter         *string  `json:"filter,omitempty"`
	Regex          *bool    `json:"regex,omitempty"`
	MatchCase      *bool    `json:"matchCase,omitempty"`
	MatchWholeWord *bool    `json:"matchWholeWord,omitempty"`
	ShowIgnored    *bool    `json:"showIgnored,omitempty"`
	ShowGenerated  *bool    `json:"showGenerated,omitempty"`
	ExcludeKinds   []string `json:"excludeKinds,omitempty"`
	IncludeKinds   []string `json:"includeKinds,omitempty"`
}

// 🗺️ToStreamOptions MUST map all filter input fields to stream options.
// 🔍️ToStreamOptions converts the filter input into stream options.
func (f *FilterInput) ToStreamOptions() StreamOptions {
	if f == nil {
		return StreamOptions{}
	}
	opts := StreamOptions{}
	if f.Filter != nil {
		opts.Filter = *f.Filter
	}
	if f.Regex != nil {
		opts.Regex = *f.Regex
	}
	if f.MatchCase != nil {
		opts.MatchCase = *f.MatchCase
	}
	if f.MatchWholeWord != nil {
		opts.MatchWholeWord = *f.MatchWholeWord
	}
	if f.ShowIgnored != nil {
		opts.ShowIgnored = *f.ShowIgnored
	}
	if f.ShowGenerated != nil {
		opts.ShowGenerated = *f.ShowGenerated
	}
	opts.ExcludeKinds = f.ExcludeKinds
	opts.IncludeKinds = f.IncludeKinds
	return opts
}

// #endregion 🎡️GraphQL Input Types

// #region ⚙️Types

// 🏷️ToolKind represents a categorized tool execution kind.
type ToolKind string

const ToolKindGeneric ToolKind = "generic"

const ToolKindPlan ToolKind = "plan"

const ToolKindCodeSearch ToolKind = "code-search"

const ToolKindCodeEdit ToolKind = "code-edit"

const ToolKindTest ToolKind = "test"

const ToolKindBuild ToolKind = "build"

const ToolKindTerminal ToolKind = "terminal"

// 🆕️TodoCreateInput holds the data fields for a todo create input record.
type TodoCreateInput struct {
	ParentID    string `json:"parentId"`
	Name        string `json:"name"`
	Description string `json:"description"`
}

// ♻️TodoChangeInput holds the data fields for a todo change input record.
type TodoChangeInput struct {
	ID          string  `json:"id"`
	Name        *string `json:"name"`
	Description *string `json:"description"`
}

// ✅️Todo holds the data fields for a todo record.
type Todo struct {
	ID          string    `json:"id"`
	Name        string    `json:"name"`
	Description string    `json:"description,omitempty"`
	ParentID    string    `json:"parentId"`
	Location    *Location `json:"location,omitempty"`
}

// 🌿️IsNode MUST return true only when the condition is met.
// ❓️IsNode reports whether the Todo is node.
func (t *Todo) IsNode() {}

// 🏪️GetID MUST return the stored value without modification.
// 📖️GetID returns the i d of the Todo.
func (t *Todo) GetID() string { return EmojiText(EmojiTodo) + workspace.Flat(t.ID) }

// 🔗️GetURI MUST return the stored value without modification.
// 🌐️GetURI returns the u r i of the Todo.
func (t *Todo) GetURI() string {
	return "repo://todo/" + t.GetID()
}

// 🔷️Location holds the data fields for a location record.
type Location struct {
	FilePath string `json:"filePath"`
	Line     int    `json:"line"`
	Column   int    `json:"column"`
}

// 🔶️Breach holds the data fields for a breach record.
type Breach struct {
	ID      string  `json:"id"`
	Summary string  `json:"summary"`
	Kind    Statute `json:"kind"`
	Scope   string  `json:"scope"`
	Line    int     `json:"line,omitempty"`
	Column  int     `json:"column,omitempty"`
	Excerpt string  `json:"excerpt,omitempty"`
	// Optional fields from lint script JSON (`.🦑️repo/⚡️cache/breach/*.json`).
	LintPriority    BreachPriority `json:"priority,omitempty"`
	LintAutofixable *bool          `json:"autofixable,omitempty"`
	Reason          string         `json:"reason,omitempty"`
	Solution        string         `json:"solution,omitempty"`
}

// 🔹️IsNode MUST return true only when the condition is met.
// ⚠️IsNode reports whether the Breach is node.
func (v *Breach) IsNode() {}

// 🔸️GetID MUST return the stored value without modification.
// 🔑️GetID returns the i d of the Breach.
func (v *Breach) GetID() string { return fmt.Sprintf("%s%s", EmojiText(EmojiBreach), v.ID) }

// 🔺️GetURI MUST return the stored value without modification.
// 🔹️GetURI returns the u r i of the Breach.
func (v *Breach) GetURI() string {
	return "repo://breach/" + v.GetID()
}

// 🔻️Priority MUST derive the value from the statute metadata.
// 📰️Priority returns the priority of the breach from its kind metadata.
func (v *Breach) Priority() BreachPriority {
	if v.LintPriority != "" {
		return v.LintPriority
	}
	return v.Kind.Info().Priority
}

// 🔧️Autofixable MUST return true only for statutes that support auto-fix.
// 📜️Autofixable reports whether the statute supports automatic fixing.
func (v *Breach) Autofixable() bool {
	if v.LintAutofixable != nil {
		return *v.LintAutofixable
	}
	return v.Kind.Info().Autofixable
}

// 🎫️TicketFileMetrics holds the data fields for a ticket file metrics record.
type TicketFileMetrics struct {
	Sections map[string]TicketSectionMetrics `yaml:"sections" json:"sections"`
}

// 📦️TicketBundleMetrics holds the data fields for a ticket bundle metrics record.
type TicketBundleMetrics struct {
	Files map[string]TicketFileMetrics `yaml:"files" json:"files"`
}

// ⬛️TicketBundles represents a ticket bundles value.
type TicketBundles map[string]TicketBundleMetrics

// #endregion ⚙️Types

// #region 🎽️Languages

// 📄️InteractionFile holds a file reference with path, id and uri.
type InteractionFile struct {
	Path string `json:"path" yaml:"path"`
	ID   string `json:"id,omitempty" yaml:"id,omitempty"`
	URI  string `json:"uri,omitempty" yaml:"uri,omitempty"`
}

// 🟫️Interaction holds the data fields for a interaction record.
type Interaction struct {
	Kind       string            `json:"kind" yaml:"kind"`
	Date       string            `json:"date" yaml:"date"`
	Author     string            `json:"author" yaml:"author"`
	System     string            `json:"system" yaml:"system"`
	Client     string            `json:"client" yaml:"client"`
	Checkpoint string            `json:"checkpoint" yaml:"checkpoint"`
	Prompt     string            `json:"prompt,omitempty" yaml:"prompt,omitempty"`
	Summary    string            `json:"summary,omitempty" yaml:"summary,omitempty"`
	LLM        string            `json:"llm,omitempty" yaml:"llm,omitempty"`
	Effort     string            `json:"effort,omitempty" yaml:"effort,omitempty"`
	Files      []InteractionFile `json:"files,omitempty" yaml:"files,omitempty"`
}

// 💾️CheckpointDiffRename holds a from/to pair for renamed entities.
type CheckpointDiffRename struct {
	From string `json:"from" yaml:"from"`
	To   string `json:"to" yaml:"to"`
}

// 🆕️CheckpointDiffStats holds deleted/renamed/modified/created lists for a diff category.
type CheckpointDiffStats struct {
	Deleted  []string               `json:"deleted,omitempty" yaml:"deleted,omitempty"`
	Renamed  []CheckpointDiffRename `json:"renamed,omitempty" yaml:"renamed,omitempty"`
	Modified []string               `json:"modified,omitempty" yaml:"modified,omitempty"`
	Created  []string               `json:"created,omitempty" yaml:"created,omitempty"`
}

// 📁️CheckpointDiff holds diff stats for technologies, bundles, folders, files, sections, and definitions.
type CheckpointDiff struct {
	Technologies CheckpointDiffStats `json:"technologies,omitempty" yaml:"technologies,omitempty"`
	Bundles      CheckpointDiffStats `json:"bundles,omitempty" yaml:"bundles,omitempty"`
	Folders      CheckpointDiffStats `json:"folders,omitempty" yaml:"folders,omitempty"`
	Files        CheckpointDiffStats `json:"files,omitempty" yaml:"files,omitempty"`
	Sections     CheckpointDiffStats `json:"sections,omitempty" yaml:"sections,omitempty"`
	Definitions  CheckpointDiffStats `json:"definitions,omitempty" yaml:"definitions,omitempty"`
}

// 🎫️TicketAgentPlanStep holds a single step in an agent plan with optional timestamps.
type TicketAgentPlanStep struct {
	ID          string `json:"id" yaml:"id"`
	Name        string `json:"name" yaml:"name"`
	Description string `json:"description,omitempty" yaml:"description,omitempty"`
	Status      string `json:"status,omitempty" yaml:"status,omitempty"`
	Ideated     string `json:"ideated,omitempty" yaml:"ideated,omitempty"`
	Started     string `json:"started,omitempty" yaml:"started,omitempty"`
	Completed   string `json:"completed,omitempty" yaml:"completed,omitempty"`
	Abandoned   string `json:"abandoned,omitempty" yaml:"abandoned,omitempty"`
}

// 💠️TicketAgentPlan holds the plan steps for an agent.
type TicketAgentPlan struct {
	Steps []TicketAgentPlanStep `json:"steps,omitempty" yaml:"steps,omitempty"`
}

// 🔳️TicketAgent holds an agent record with contributor and plan.
type TicketAgent struct {
	Session     string           `json:"session" yaml:"session"`
	Contributor string           `json:"contributor,omitempty" yaml:"contributor,omitempty"`
	System      string           `json:"system,omitempty" yaml:"system,omitempty"`
	Client      string           `json:"client,omitempty" yaml:"client,omitempty"`
	LLM         string           `json:"llm,omitempty" yaml:"llm,omitempty"`
	Effort      string           `json:"effort,omitempty" yaml:"effort,omitempty"`
	Transcript  string           `json:"transcript,omitempty" yaml:"transcript,omitempty"`
	Plan        *TicketAgentPlan `json:"plan,omitempty" yaml:"plan,omitempty"`
}

// 🔲️UnmarshalJSON MUST handle both legacy and current JSON layouts.
func (i *Interaction) UnmarshalJSON(data []byte) error {
	type Alias Interaction
	raw := struct {
		Alias
		RawAuthor json.RawMessage `json:"author"`
		RawSystem json.RawMessage `json:"system"`
		RawDates  json.RawMessage `json:"dates"`
	}{}
	if err := json.Unmarshal(data, &raw); err != nil {
		return err
	}
	*i = Interaction(raw.Alias)
	if i.Date == "" && len(raw.RawDates) > 0 {
		var legacy struct {
			Created string `json:"created"`
		}
		if err := json.Unmarshal(raw.RawDates, &legacy); err == nil && legacy.Created != "" {
			i.Date = legacy.Created
		}
	}
	if len(raw.RawAuthor) > 0 {
		if raw.RawAuthor[0] == '"' {
			json.Unmarshal(raw.RawAuthor, &i.Author)
		} else if raw.RawAuthor[0] == '{' {
			var obj struct {
				Name   string `json:"name"`
				Email  string `json:"email"`
				Github string `json:"github"`
			}
			if err := json.Unmarshal(raw.RawAuthor, &obj); err == nil {
				if obj.Name != "" && obj.Email != "" {
					i.Author = fmt.Sprintf("%s <%s>", obj.Name, obj.Email)
				} else if obj.Name != "" {
					i.Author = obj.Name
				} else if obj.Email != "" {
					i.Author = obj.Email
				}
			}
		}
	}
	if len(raw.RawSystem) > 0 {
		if raw.RawSystem[0] == '"' {
			json.Unmarshal(raw.RawSystem, &i.System)
		} else if raw.RawSystem[0] == '{' {
			var obj struct {
				Version string `json:"version"`
				Client  string `json:"client"`
			}
			if err := json.Unmarshal(raw.RawSystem, &obj); err == nil {
				i.System = obj.Version
				if obj.Client != "" && i.Client == "" {
					i.Client = obj.Client
				}
			}
		}
	}
	return nil
}

// 🎁️InteractionResource holds a flat interaction enriched with its source context.
type InteractionResource struct {
	Interaction
	SourceKind string `json:"sourceKind"`
	SourceID   string `json:"sourceId"`
	GoalID     string `json:"goalId,omitempty"`
	TicketID   string `json:"ticketId,omitempty"`
}

// ▫️TicketSection holds the data fields for a ticket section record.
type TicketSection struct {
	Name        string       `json:"name"`
	Range       *Range       `json:"range,omitempty"`
	Definitions []string     `json:"definitions,omitempty"`
	Lines       *LineMetrics `json:"lines,omitempty"`
}

// ◾TicketFile holds the data fields for a ticket file record.
type TicketFile struct {
	Path  string       `json:"path"`
	Lines *LineMetrics `json:"lines,omitempty"`
}

// 🐙️TicketManagementData holds the data fields for a ticket github data record.
type TicketManagementData struct {
	Issue string `json:"issue,omitempty"`
}

// ◽TicketFileRenamed holds the data fields for a ticket file renamed record.
type TicketFileRenamed struct {
	From  string       `json:"from"`
	To    string       `json:"to"`
	Lines *LineMetrics `json:"lines,omitempty"`
}

// 🗃️TicketDiffSet holds the data fields for a ticket diff set record.
type TicketDiffSet struct {
	Deleted  []TicketFile        `json:"deleted"`
	Renamed  []TicketFileRenamed `json:"renamed"`
	Modified []TicketFile        `json:"modified"`
	Added    []TicketFile        `json:"added"`
}

// ◻TicketDiffs holds the data fields for a ticket diffs record.
type TicketDiffs struct {
	Bundles     TicketDiffSet `json:"bundles"`
	Folders     TicketDiffSet `json:"folders"`
	Files       TicketDiffSet `json:"files"`
	Sections    TicketDiffSet `json:"sections"`
	Definitions TicketDiffSet `json:"definitions"`
}

// ◼TicketData holds the data fields for a ticket data record.
type TicketData struct {
	Title      string                `json:"title"`
	Status     TicketStatus          `json:"status"`
	Summary    string                `json:"summary,omitempty"`
	Management *TicketManagementData `json:"github,omitempty"`
	Goal       string                `json:"goal,omitempty"`
	Parent     string                `json:"parent,omitempty"`
}

// 🔵️Goal holds the data fields for a goal record.
type Goal struct {
	Title       string              `json:"title"`
	Description string              `json:"description"`
	Prompt      string              `json:"prompt"`
	Status      string              `json:"status"`
	Summary     string              `json:"summary,omitempty"`
	DueDate     string              `json:"dueDate,omitempty"`
	Dates       GoalDates           `json:"dates"`
	Client      string              `json:"client"`
	LLM         string              `json:"llm"`
	Effort      string              `json:"effort,omitempty"`
	Parent      string              `json:"parent,omitempty"`
	Management  *GoalManagementData `json:"github,omitempty"`

	ID   string `json:"-"`
	Path string `json:"-"`
}

// 😀️MarshalJSON converts internal filesystem paths to repo emoji IDs for Goal serialization.
func (g Goal) MarshalJSON() ([]byte, error) {
	type GoalAlias Goal
	alias := GoalAlias(g)

	if alias.Parent != "" && !strings.Contains(alias.Parent, EmojiText(EmojiGoal)) {
		alias.Parent = GoalPathToComposeID(alias.Parent)
	}
	return json.Marshal(alias)
}

// 🌿️IsNode MUST return true only when the condition is met.
// 🔖️IsNode reports whether the Goal is node.
func (g *Goal) IsNode() {}

// 🔴️GetID MUST return the stored value without modification.
// 🔑️GetID returns the i d of the Goal.
func (g *Goal) GetID() string { return goalArtifactID(g.ID) }

// 🔗️GetURI MUST return the stored value without modification.
// 🌐️GetURI returns the u r i of the Goal.
func (g *Goal) GetURI() string {
	return GetArtifactURI("goal", map[string]interface{}{"id": g.ID})
}

// 🟠️GoalDates holds the data fields for a goal dates record.
type GoalDates struct {
	Due string `json:"due,omitempty"`
}

// 🟡️GoalManagementData holds the data fields for a goal github data record.
type GoalManagementData struct {
	Milestone string `json:"milestone,omitempty"`
	Issue     string `json:"issue,omitempty"`
}

// 🟢️Statute represents a statute value.
type Statute string

const BreachCodeFileMissingHeaderRegion Statute = "code/file/missing-header-region"

const BreachCodeFileWrongHeaderRegionFormat Statute = "code/file/wrong-header-region-format"

const BreachCodeFileMissingContributors Statute = "code/file/missing-contributors"

const BreachCodeFileMissingSummary Statute = "code/file/missing-summary"

const BreachCodeFileMissingLicense Statute = "code/file/missing-license"

const BreachCodeFileWrongLicense Statute = "code/file/wrong-license"

const BreachCodeFileMissingRequirements Statute = "code/file/missing-requirements"

const BreachCodeFileMissingDocs Statute = "code/file/missing-docs"

const BreachCodeSectionEmpty Statute = "code/section/empty"

const BreachCodeSectionOrphanDefinition Statute = "code/section/orphan-definition"

const BreachCodeSectionMissingStartName Statute = "code/section/missing-start-name"

const BreachCodeSectionMissingEndName Statute = "code/section/missing-end-name"

const BreachCodeSectionNameMismatch Statute = "code/section/name-mismatch"

const BreachCodeSectionWrongFormat Statute = "code/section/wrong-format"

const BreachCodeSectionWrongFormatSummaryTooLong Statute = "code/section/wrong-format/summary/too-long-summary"

const BreachCodeSectionWrongFormatNewlineAfterRegion Statute = "code/section/wrong-format/newline-after-region"

const BreachCodeSectionWrongFormatRequirementsSplitBlock Statute = "code/section/wrong-format/requirements/split-block"

const BreachCodeSectionWrongFormatDocs Statute = "code/section/wrong-format/docs"

const BreachCodeSectionMissingSummary Statute = "code/section/missing-summary"

const BreachCodeSectionMissingRequirements Statute = "code/section/missing-requirements"

const BreachCodeSectionMissingDocs Statute = "code/section/missing-docs"

const BreachCodeDefWrongFormat Statute = "code/definition/wrong-format"

const BreachCodeDefNotNativeDocstring Statute = "code/definition/wrong-format/not-native-docstring"

const BreachCodeDefMissingSummary Statute = "code/definition/missing-summary"

const BreachCodeDefMissingRequirements Statute = "code/definition/missing-requirements"

const BreachCodeDefMissingDocs Statute = "code/definition/missing-docs"

const BreachCodeCommentInline Statute = "code/comment/inline"

const BreachCodeCommentBlock Statute = "code/comment/block"

const BreachCodeCommentJSDoc Statute = "code/comment/jsdoc"

const BreachCodeRequirementsSyntax Statute = "code/requirements/implementation-syntax"

const BreachCodeDocsMissingReadme Statute = "code/doc/missing-readme"

const BreachDevDocsMissingFile Statute = "dev-docs/missing-file"

const BreachDevDocsMissingFolder Statute = "dev-docs/missing-folder"

const BreachDevDocsWrongFilePath Statute = "dev-docs/wrong-file-path"

const BreachDevDocsWrongFolderPath Statute = "dev-docs/wrong-folder-path"

const BreachDevDocsWrongFileName Statute = "dev-docs/wrong-file-name"

const BreachDevDocsWrongFolderName Statute = "dev-docs/wrong-folder-name"

const BreachDevDocsWrongFileOrder Statute = "dev-docs/wrong-file-order"

const BreachDevDocsWrongFolderOrder Statute = "dev-docs/wrong-folder-order"

const BreachDevDocsMissingComponent Statute = "dev-docs/missing-component"

const BreachDevDocsWrongComponentName Statute = "dev-docs/wrong-component-name"

const BreachDevDocsWrongComponentOrder Statute = "dev-docs/wrong-component-order"

const BreachSketchpadImportThirdParty Statute = "sketchpad/import/third-party-outside-elements"

const BreachSketchpadStateMultipleMachines Statute = "sketchpad/state/multiple-machines"

const BreachSketchpadStateCreateActor Statute = "sketchpad/state/create-actor-usage"

const BreachSketchpadStateYjsAppState Statute = "sketchpad/state/yjs-app-state"

const BreachSketchpadStateForbiddenStore Statute = "sketchpad/state/forbidden-store"

const BreachSketchpadHooksNonTriadic Statute = "sketchpad/hook/non-triadic"

const BreachCodeUnicodeEmojiVariation Statute = "code/unicode/emoji-variation"

const BreachRepoMissingCommand Statute = "repo/missing-command"

const BreachRepoMissingTicketTracking Statute = "repo/missing-ticket-tracking"

const BreachSystemDevcontainerVscodeSettingsOutside Statute = "system/devcontainer/vscode/settings-outside-devcontainer"

const BreachSystemDevcontainerVscodeExtensionsOutside Statute = "system/devcontainer/vscode/extensions-outside-devcontainer"

const BreachFolderIllegalEmpty Statute = "folder/illegal/empty"

const BreachFolderNameMissingEmoji Statute = "folder/name/missing-emoji"

const BreachFolderNameGenericEmoji Statute = "folder/name/generic-emoji"

const BreachFolderNameNoncanonicalEmojiPresentation Statute = "folder/name/noncanonical-emoji-presentation"

const BreachFolderNameWhitespaceAfterEmoji Statute = "folder/name/whitespace-after-emoji"

const BreachFolderNameEmojiNotUnique Statute = "folder/name/emoji-not-unique"

const BreachFolderNameMultipleEmojis Statute = "folder/name/multiple-emojis"

const BreachFileIllegalUseGodfile Statute = "file/illegal/use-godfile"

const BreachFileNameMissingEmoji Statute = "file/name/missing-emoji"

const BreachFileNameGenericEmoji Statute = "file/name/generic-emoji"

const BreachFileNameNoncanonicalEmojiPresentation Statute = "file/name/noncanonical-emoji-presentation"

const BreachFileNameWhitespaceAfterEmoji Statute = "file/name/whitespace-after-emoji"

const BreachFileNameEmojiNotUnique Statute = "file/name/emoji-not-unique"

const BreachFileNameMultipleEmojis Statute = "file/name/multiple-emojis"

const BreachFileNameReservedEmoji Statute = "file/name/reserved-name-has-emoji"

const BreachComposeNoUiDependency Statute = "compose/import/no-ui-dependency"

const BreachDependencyBoundaryDirectImport Statute = "dependency-boundary/import/direct-third-party"

const BreachComposeDescriptionMissingEmoji Statute = "compose/description/missing-emoji"

const BreachComposeDescriptionEmojiNotUnique Statute = "compose/description/emoji-not-unique"

const BreachCodeRustRegionComment Statute = "code/rust/region-comment-instead-of-mod"

// 🔏️pathEmojiStatuteMeta returns the shared high-priority contract for path identity statutes.
func pathEmojiStatuteMeta(kind Statute) StatuteMeta {
	return StatuteMeta{
		Kind:        kind,
		Priority:    BreachPriorityHigh,
		Reason:      "Every non-reserved file and folder requires exactly one handpicked, meaningful emoji unique among all siblings; reserved filenames remain literal",
		Solution:    "Handpick one meaningful sibling-unique emoji, or restore the unprefixed reserved name, and update only resolved path references",
		Autofixable: false,
	}
}

// 📊️statuteInfoTable holds the data fields for a statuteInfoTable record.
var StatuteInfoTable = map[Statute]StatuteMeta{
	BreachRepoMissingCommand: {
		Kind:        BreachRepoMissingCommand,
		Priority:    BreachPriorityHigh,
		Reason:      "Command is missing from parity implementation (CLI, MCP, VS Code)",
		Solution:    "Implement the command in the missing platform",
		Autofixable: false,
	},
	BreachRepoMissingTicketTracking: {
		Kind:        BreachRepoMissingTicketTracking,
		Priority:    BreachPriorityHigh,
		Reason:      "Ticket tracking code is missing or incomplete",
		Solution:    "Implement strict ticket tracking (open/close/log)",
		Autofixable: false,
	},
	BreachCodeFileMissingHeaderRegion: {
		Kind:        BreachCodeFileMissingHeaderRegion,
		Priority:    BreachPriorityLow,
		Reason:      "Header region with license, filename, and contributors is required",
		Solution:    "Add header region with SPDX license, filename, and contributors",
		Autofixable: true,
	},
	BreachCodeFileWrongHeaderRegionFormat: {
		Kind:        BreachCodeFileWrongHeaderRegionFormat,
		Priority:    BreachPriorityLow,
		Reason:      "Header region format is incorrect (missing License or Requirements subregion)",
		Solution:    "Add License and Requirements subregions inside Header",
		Autofixable: false,
	},
	BreachCodeFileMissingContributors: {
		Kind:        BreachCodeFileMissingContributors,
		Priority:    BreachPriorityLow,
		Reason:      "Contributors must be documented in header",
		Solution:    "Add contributor line in header region",
		Autofixable: false,
	},
	BreachCodeFileMissingSummary: {
		Kind:        BreachCodeFileMissingSummary,
		Priority:    BreachPriorityLow,
		Reason:      "Summary must be documented in header",
		Solution:    "Add summary line in header region after the file ID",
		Autofixable: false,
	},
	BreachCodeFileMissingLicense: {
		Kind:        BreachCodeFileMissingLicense,
		Priority:    BreachPriorityLow,
		Reason:      "License text is required in header License subregion",
		Solution:    "Add AGPL license text in License subregion",
		Autofixable: true,
	},
	BreachCodeFileWrongLicense: {
		Kind:        BreachCodeFileWrongLicense,
		Priority:    BreachPriorityLow,
		Reason:      "License must be AGPL-3.0-or-later",
		Solution:    "Replace license with AGPL-3.0-or-later",
		Autofixable: true,
	},
	BreachCodeFileMissingRequirements: {
		Kind:        BreachCodeFileMissingRequirements,
		Priority:    BreachPriorityLow,
		Reason:      "Requirements subregion is required inside Header",
		Solution:    "Add Requirements subregion inside Header",
		Autofixable: false,
	},
	BreachCodeFileMissingDocs: {
		Kind:        BreachCodeFileMissingDocs,
		Priority:    BreachPriorityLow,
		Reason:      "File must be documented in bundle README.md Docs section",
		Solution:    "Add file documentation to bundle README.md",
		Autofixable: false,
	},
	BreachCodeSectionEmpty: {
		Kind:        BreachCodeSectionEmpty,
		Priority:    BreachPriorityLow,
		Reason:      "Empty sections should be removed",
		Solution:    "Remove empty section or add content",
		Autofixable: true,
	},
	BreachCodeSectionOrphanDefinition: {
		Kind:        BreachCodeSectionOrphanDefinition,
		Priority:    BreachPriorityLow,
		Reason:      "All code must be inside named sections",
		Solution:    "Move code into an existing section or add a new section",
		Autofixable: false,
	},
	BreachCodeSectionMissingStartName: {
		Kind:        BreachCodeSectionMissingStartName,
		Priority:    BreachPriorityLow,
		Reason:      "Section start marker must have a name",
		Solution:    "Add name to section start marker",
		Autofixable: false,
	},
	BreachCodeSectionMissingEndName: {
		Kind:        BreachCodeSectionMissingEndName,
		Priority:    BreachPriorityLow,
		Reason:      "Section end marker should have matching name",
		Solution:    "Add matching name to section end marker",
		Autofixable: true,
	},
	BreachCodeSectionNameMismatch: {
		Kind:        BreachCodeSectionNameMismatch,
		Priority:    BreachPriorityLow,
		Reason:      "Section start and end names must match",
		Solution:    "Fix section end name to match start name",
		Autofixable: true,
	},
	BreachCodeSectionWrongFormat: {
		Kind:        BreachCodeSectionWrongFormat,
		Priority:    BreachPriorityLow,
		Reason:      "Section region marker format is incorrect",
		Solution:    "Use correct region marker format with emoji bookmark",
		Autofixable: false,
	},
	BreachCodeSectionWrongFormatSummaryTooLong: {
		Kind:        BreachCodeSectionWrongFormatSummaryTooLong,
		Priority:    BreachPriorityLow,
		Reason:      "Section summary must not exceed 256 characters",
		Solution:    "Shorten the section summary to 256 characters or less",
		Autofixable: false,
	},
	BreachCodeSectionWrongFormatNewlineAfterRegion: {
		Kind:        BreachCodeSectionWrongFormatNewlineAfterRegion,
		Priority:    BreachPriorityLow,
		Reason:      "No blank lines allowed between region start marker and region information comments",
		Solution:    "Remove blank lines between region start marker and the first comment line",
		Autofixable: true,
	},
	BreachCodeSectionWrongFormatRequirementsSplitBlock: {
		Kind:        BreachCodeSectionWrongFormatRequirementsSplitBlock,
		Priority:    BreachPriorityLow,
		Reason:      "Spec comment block must be contiguous without blank lines",
		Solution:    "Remove blank lines between spec comment lines",
		Autofixable: true,
	},
	BreachCodeSectionWrongFormatDocs: {
		Kind:        BreachCodeSectionWrongFormatDocs,
		Priority:    BreachPriorityLow,
		Reason:      "Section docs format is incorrect",
		Solution:    "Fix the section docs format",
		Autofixable: false,
	},
	BreachCodeSectionMissingSummary: {
		Kind:        BreachCodeSectionMissingSummary,
		Priority:    BreachPriorityLow,
		Reason:      "Section must have a summary comment after the region start",
		Solution:    "Add summary comment after section region start marker",
		Autofixable: true,
	},
	BreachCodeSectionMissingRequirements: {
		Kind:        BreachCodeSectionMissingRequirements,
		Priority:    BreachPriorityLow,
		Reason:      "Section must have requirements comments after the summary",
		Solution:    "Add requirements comments after section summary",
		Autofixable: false,
	},
	BreachCodeSectionMissingDocs: {
		Kind:        BreachCodeSectionMissingDocs,
		Priority:    BreachPriorityLow,
		Reason:      "Section must be documented in bundle README.md Docs section",
		Solution:    "Add section documentation to bundle README.md",
		Autofixable: false,
	},
	BreachCodeDefWrongFormat: {
		Kind:        BreachCodeDefWrongFormat,
		Priority:    BreachPriorityLow,
		Reason:      "Definition does not have a proper docstring",
		Solution:    "Add language-native docstring to definition",
		Autofixable: false,
	},
	BreachCodeDefNotNativeDocstring: {
		Kind:        BreachCodeDefNotNativeDocstring,
		Priority:    BreachPriorityLow,
		Reason:      "Definition must use language-native docstring format (JSDoc for TS/JS, /// for C#/Rust)",
		Solution:    "Convert line comments to language-native docstring format",
		Autofixable: true,
	},
	BreachCodeDefMissingSummary: {
		Kind:        BreachCodeDefMissingSummary,
		Priority:    BreachPriorityLow,
		Reason:      "Definition must have a summary in its docstring",
		Solution:    "Add summary line to definition docstring",
		Autofixable: true,
	},
	BreachCodeDefMissingRequirements: {
		Kind:        BreachCodeDefMissingRequirements,
		Priority:    BreachPriorityLow,
		Reason:      "Definition must have requirements in its docstring",
		Solution:    "Add requirements to definition docstring",
		Autofixable: true,
	},
	BreachCodeDefMissingDocs: {
		Kind:        BreachCodeDefMissingDocs,
		Priority:    BreachPriorityLow,
		Reason:      "Definition must be documented in bundle README.md Docs section",
		Solution:    "Add definition documentation to bundle README.md",
		Autofixable: false,
	},
	BreachCodeCommentInline: {
		Kind:        BreachCodeCommentInline,
		Priority:    BreachPriorityLow,
		Reason:      "Inline comments are forbidden",
		Solution:    "Remove inline comment",
		Autofixable: true,
	},
	BreachCodeCommentBlock: {
		Kind:        BreachCodeCommentBlock,
		Priority:    BreachPriorityLow,
		Reason:      "Block comments are forbidden",
		Solution:    "Remove block comment",
		Autofixable: true,
	},
	BreachCodeCommentJSDoc: {
		Kind:        BreachCodeCommentJSDoc,
		Priority:    BreachPriorityLow,
		Reason:      "JSDoc comments are forbidden",
		Solution:    "Remove JSDoc comment",
		Autofixable: true,
	},
	BreachCodeRequirementsSyntax: {
		Kind:        BreachCodeRequirementsSyntax,
		Priority:    BreachPriorityLow,
		Reason:      "Requirements must be implementation-agnostic and must not contain code syntax",
		Solution:    "Remove backticks, function calls, and other code syntax from spec text",
		Autofixable: false,
	},
	BreachCodeDocsMissingReadme: {
		Kind:        BreachCodeDocsMissingReadme,
		Priority:    BreachPriorityLow,
		Reason:      "Bundle or folder is missing a README.md with summary and requirements",
		Solution:    "Add a README.md with # Summary and # 💯️Requirements sections",
		Autofixable: false,
	},
	BreachCodeUnicodeEmojiVariation: {
		Kind:        BreachCodeUnicodeEmojiVariation,
		Priority:    BreachPriorityLow,
		Reason:      "Emoji variation selectors (VS15/VS16) are forbidden",
		Solution:    "Strip variation selectors from emoji",
		Autofixable: true,
	},
	BreachDevDocsMissingFile: {
		Kind:        BreachDevDocsMissingFile,
		Priority:    BreachPriorityLow,
		Reason:      "File exists but has no section in AGENTS.md Codebase",
		Solution:    "Add ## 📄️PATH section in AGENTS.md",
		Autofixable: true,
	},
	BreachDevDocsMissingFolder: {
		Kind:        BreachDevDocsMissingFolder,
		Priority:    BreachPriorityLow,
		Reason:      "Folder exists but has no section in AGENTS.md Codebase",
		Solution:    "Add ## 📁️PATH section in AGENTS.md",
		Autofixable: true,
	},
	BreachDevDocsWrongFilePath: {
		Kind:        BreachDevDocsWrongFilePath,
		Priority:    BreachPriorityLow,
		Reason:      "File section path does not match actual file path",
		Solution:    "Update file section path to match actual path",
		Autofixable: true,
	},
	BreachDevDocsWrongFolderPath: {
		Kind:        BreachDevDocsWrongFolderPath,
		Priority:    BreachPriorityLow,
		Reason:      "Folder section path does not match actual folder path",
		Solution:    "Update folder section path to match actual path",
		Autofixable: true,
	},
	BreachDevDocsWrongFileName: {
		Kind:        BreachDevDocsWrongFileName,
		Priority:    BreachPriorityLow,
		Reason:      "File section name format is incorrect (should be ## 📄️PATH)",
		Solution:    "Rename section to ## 📄️PATH",
		Autofixable: true,
	},
	BreachDevDocsWrongFolderName: {
		Kind:        BreachDevDocsWrongFolderName,
		Priority:    BreachPriorityLow,
		Reason:      "Folder section name format is incorrect (should be ## 📁️PATH/)",
		Solution:    "Rename section to ## 📁️PATH/",
		Autofixable: true,
	},
	BreachDevDocsWrongFileOrder: {
		Kind:        BreachDevDocsWrongFileOrder,
		Priority:    BreachPriorityLow,
		Reason:      "File sections are not in alphabetical order",
		Solution:    "Reorder file sections alphabetically",
		Autofixable: true,
	},
	BreachDevDocsWrongFolderOrder: {
		Kind:        BreachDevDocsWrongFolderOrder,
		Priority:    BreachPriorityLow,
		Reason:      "Folder sections are not in alphabetical order",
		Solution:    "Reorder folder sections alphabetically",
		Autofixable: true,
	},
	BreachDevDocsMissingComponent: {
		Kind:        BreachDevDocsMissingComponent,
		Priority:    BreachPriorityLow,
		Reason:      "Package.json workspace has no corresponding component in README.md",
		Solution:    "Add component section in README.md Components",
		Autofixable: true,
	},
	BreachDevDocsWrongComponentName: {
		Kind:        BreachDevDocsWrongComponentName,
		Priority:    BreachPriorityLow,
		Reason:      "Component section name does not match workspace name",
		Solution:    "Rename component section to match workspace",
		Autofixable: true,
	},
	BreachDevDocsWrongComponentOrder: {
		Kind:        BreachDevDocsWrongComponentOrder,
		Priority:    BreachPriorityLow,
		Reason:      "Component sections are not in package.json workspaces order",
		Solution:    "Reorder components to match package.json workspaces",
		Autofixable: true,
	},
	BreachSketchpadImportThirdParty: {
		Kind:        BreachSketchpadImportThirdParty,
		Priority:    BreachPriorityHigh,
		Reason:      "Third party imports must only be in elements.tsx",
		Solution:    "Move third party import to elements.tsx and re-export from there",
		Autofixable: false,
	},
	BreachSketchpadStateMultipleMachines: {
		Kind:        BreachSketchpadStateMultipleMachines,
		Priority:    BreachPriorityHigh,
		Reason:      "Only one state machine is allowed (createMachine can only be used once)",
		Solution:    "Consolidate state management into a single state machine",
		Autofixable: false,
	},
	BreachSketchpadStateCreateActor: {
		Kind:        BreachSketchpadStateCreateActor,
		Priority:    BreachPriorityHigh,
		Reason:      "createActor is forbidden in sketchpad",
		Solution:    "Remove createActor usage and use the single state machine instead",
		Autofixable: false,
	},
	BreachSketchpadStateYjsAppState: {
		Kind:        BreachSketchpadStateYjsAppState,
		Priority:    BreachPriorityHigh,
		Reason:      "Yjs app state is forbidden in sketchpad state management",
		Solution:    "Keep app state in the single state machine and use Yjs only for synchronized document data",
		Autofixable: false,
	},
	BreachSketchpadStateForbiddenStore: {
		Kind:        BreachSketchpadStateForbiddenStore,
		Priority:    BreachPriorityHigh,
		Reason:      "Stores outside of State Management sections are forbidden",
		Solution:    "Move store to a State Management section or remove it",
		Autofixable: false,
	},
	BreachSketchpadHooksNonTriadic: {
		Kind:        BreachSketchpadHooksNonTriadic,
		Priority:    BreachPriorityHigh,
		Reason:      "Client elements must use triadic hooks pattern [state, setState, canSetState]=useSELECTOR()",
		Solution:    "Refactor to use triadic hook pattern with useSELECTOR",
		Autofixable: false,
	},
	BreachSystemDevcontainerVscodeSettingsOutside: {
		Kind:        BreachSystemDevcontainerVscodeSettingsOutside,
		Priority:    BreachPriorityHigh,
		Reason:      "VSCode settings must be inside devcontainer.json customizations, not in .vscode/settings.json",
		Solution:    "Move .vscode/settings.json to customizations.vscode.settings inside .devcontainer/devcontainer.json",
		Autofixable: true,
	},
	BreachSystemDevcontainerVscodeExtensionsOutside: {
		Kind:        BreachSystemDevcontainerVscodeExtensionsOutside,
		Priority:    BreachPriorityHigh,
		Reason:      "Host editors (Cursor, VS Code) read .vscode/extensions.json for workspace recommendations; it must include every extension from devcontainer customizations.vscode.extensions",
		Solution:    "Sync .vscode/extensions.json recommendations with customizations.vscode.extensions in .devcontainer/devcontainer.json (host-only extras such as Dev Containers are allowed)",
		Autofixable: true,
	},
	BreachFolderIllegalEmpty: {
		Kind:        BreachFolderIllegalEmpty,
		Priority:    BreachPriorityLow,
		Reason:      "Empty folders are not allowed",
		Solution:    "Remove the empty folder",
		Autofixable: true,
	},
	BreachFolderNameMissingEmoji:                  pathEmojiStatuteMeta(BreachFolderNameMissingEmoji),
	BreachFolderNameGenericEmoji:                  pathEmojiStatuteMeta(BreachFolderNameGenericEmoji),
	BreachFolderNameNoncanonicalEmojiPresentation: pathEmojiStatuteMeta(BreachFolderNameNoncanonicalEmojiPresentation),
	BreachFolderNameWhitespaceAfterEmoji:          pathEmojiStatuteMeta(BreachFolderNameWhitespaceAfterEmoji),
	BreachFolderNameEmojiNotUnique:                pathEmojiStatuteMeta(BreachFolderNameEmojiNotUnique),
	BreachFolderNameMultipleEmojis:                pathEmojiStatuteMeta(BreachFolderNameMultipleEmojis),
	BreachFileIllegalUseGodfile: {
		Kind:        BreachFileIllegalUseGodfile,
		Priority:    BreachPriorityHigh,
		Reason:      "File is not listed in .🧬semio/🦑️repo/📁️files.json godfile",
		Solution:    "Add the file to .🧬semio/🦑️repo/📁️files.json or remove it",
		Autofixable: false,
	},
	BreachFileNameMissingEmoji:                  pathEmojiStatuteMeta(BreachFileNameMissingEmoji),
	BreachFileNameGenericEmoji:                  pathEmojiStatuteMeta(BreachFileNameGenericEmoji),
	BreachFileNameNoncanonicalEmojiPresentation: pathEmojiStatuteMeta(BreachFileNameNoncanonicalEmojiPresentation),
	BreachFileNameWhitespaceAfterEmoji:          pathEmojiStatuteMeta(BreachFileNameWhitespaceAfterEmoji),
	BreachFileNameEmojiNotUnique:                pathEmojiStatuteMeta(BreachFileNameEmojiNotUnique),
	BreachFileNameMultipleEmojis:                pathEmojiStatuteMeta(BreachFileNameMultipleEmojis),
	BreachFileNameReservedEmoji:                 pathEmojiStatuteMeta(BreachFileNameReservedEmoji),
	BreachComposeNoUiDependency: {
		Kind:        BreachComposeNoUiDependency,
		Priority:    BreachPriorityHigh,
		Reason:      "compose.* files must be self-contained and not import from UI dependencies",
		Solution:    "Remove the UI dependency import and use only non-UI alternatives",
		Autofixable: false,
	},
	BreachDependencyBoundaryDirectImport: {
		Kind:        BreachDependencyBoundaryDirectImport,
		Priority:    BreachPriorityHigh,
		Reason:      "Third-party packages must only be imported in adapter regions or adapter paths",
		Solution:    "Move the import into a //#region 🔌️Adapter (or /adapters/) module and depend on a first-party port interface elsewhere",
		Autofixable: false,
	},
	BreachComposeDescriptionMissingEmoji: {
		Kind:        BreachComposeDescriptionMissingEmoji,
		Priority:    BreachPriorityMedium,
		Reason:      "Every entity description must start with an emoji as the first symbol",
		Solution:    "Add a leading emoji to the description",
		Autofixable: false,
	},
	BreachComposeDescriptionEmojiNotUnique: {
		Kind:        BreachComposeDescriptionEmojiNotUnique,
		Priority:    BreachPriorityMedium,
		Reason:      "Every entity must have a unique emoji among its siblings",
		Solution:    "Change the leading emoji to one that is not already used by a sibling entity",
		Autofixable: false,
	},
	BreachCodeRustRegionComment: {
		Kind:        BreachCodeRustRegionComment,
		Priority:    BreachPriorityHigh,
		Reason:      "Rust files must use mod blocks for sections instead of region comments",
		Solution:    "Replace // #region with mod section_name { // 🔖️Name and // #endregion with } // 🔖️Name",
		Autofixable: false,
	},
}

// 📍️Info MUST return the metadata entry for the statute.
// ℹInfo returns the metadata for the statute.
func (k Statute) Info() StatuteMeta {
	if info, ok := StatuteInfoTable[k]; ok {
		return info
	}
	return StatuteMeta{
		Kind:        k,
		Priority:    BreachPriorityLow,
		Reason:      "Unknown breach",
		Solution:    "Fix the breach",
		Autofixable: false,
	}
}

// 🟣️Territory holds the data fields for a statute group record.
type Territory struct {
	Name        string      `json:"name"`
	Description string      `json:"description"`
	Scopes      []string    `json:"scopes,omitempty"`
	Groups      []Territory `json:"groups,omitempty"`
	Kinds       []Statute   `json:"kinds,omitempty"`
}

// 🟤️AllKinds MUST include all statutes from the group and its children.
// 🟦️AllKinds returns all statutes associated with the group.
func (g *Territory) AllKinds() []Statute {
	var result []Statute
	result = append(result, g.Kinds...)
	for _, child := range g.Groups {
		result = append(result, child.AllKinds()...)
	}
	return result
}

// ⚪️GetID MUST return the stored value without modification.
// 🗺️GetID returns the i d of the Territory.
func (g *Territory) GetID() string {
	return fmt.Sprintf("%s%s", EmojiText(EmojiTerritory), g.Name)
}

// ⚫️GetURI MUST return the stored value without modification.
// ❌️GetURI returns the u r i of the Territory.
func (g *Territory) GetURI() string {
	return "repo://territory/" + g.GetID()
}

// 🤍️ContributorTicket holds the data fields for a contributor ticket record.
type ContributorTicket struct {
	Year     int          `json:"year"`
	Month    int          `json:"month"`
	Day      int          `json:"day"`
	Slug     string       `json:"slug"`
	Status   TicketStatus `json:"status"`
	FilePath string       `json:"filePath,omitempty"`
}

type ContributorCheckpoint struct {
	Title string `json:"title"`
	Sha   string `json:"sha"`
}

// 🖤️ContributorContributionsStorage holds the data fields for a contributor contributions storage record.
type ContributorContributionsStorage struct {
	Bundles     []string                `json:"bundles,omitempty"`
	Folders     []string                `json:"folders,omitempty"`
	Files       []string                `json:"files,omitempty"`
	Regions     []string                `json:"regions,omitempty"`
	Definitions []string                `json:"definitions,omitempty"`
	Tickets     []ContributorTicket     `json:"tickets,omitempty"`
	Checkpoints []ContributorCheckpoint `json:"checkpoints,omitempty"`
	Lines       *LineMetrics            `json:"lines,omitempty"`
}

// #endregion 🎽️Languages

// #region ⭐️Codebase Types

// 💿️BundleMetricsInternal holds the data fields for a bundle metrics internal record.
type BundleMetricsInternal struct {
	Folders     int `json:"folders"`
	Files       int `json:"files"`
	Sections    int `json:"sections"`
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
	Breachs     int `json:"breachs"`
}

// 📁️FolderMetricsInternal holds the data fields for a folder metrics internal record.
type FolderMetricsInternal struct {
	Files   int `json:"files"`
	Lines   int `json:"lines"`
	Breachs int `json:"breachs"`
}

// 📄️FileMetricsInternal holds the data fields for a file metrics internal record.
type FileMetricsInternal struct {
	Sections    int `json:"sections"`
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
}

// 📑️SectionMetricsInternal holds the data fields for a section metrics internal record.
type SectionMetricsInternal struct {
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
	Breachs     int `json:"breachs"`
}

// 📖️DefinitionMetricsInternal holds the data fields for a definition metrics internal record.
type DefinitionMetricsInternal struct {
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
	Breachs     int `json:"breachs"`
}

// 🔷️RangePosition holds the data fields for a range position record.
type RangePosition struct {
	Line   int `json:"line"`
	Column int `json:"column"`
}

// 🔶️FileRange holds the data fields for a file range record.
type FileRange struct {
	Start RangePosition `json:"start"`
	End   RangePosition `json:"end"`
}

// 🔹️BreachFile holds the data fields for a breach file record.
type BreachFile struct {
	ID    string     `json:"id"`
	Path  string     `json:"path"`
	URI   string     `json:"uri"`
	Range *FileRange `json:"range,omitempty"`
}

// 🔸️BreachFolder holds the data fields for a breach folder record.
type BreachFolder struct {
	ID   string `json:"id"`
	Path string `json:"path"`
	URI  string `json:"uri"`
}

// 🔺️CodebaseBreach holds the data fields for a codebase breach record.
type CodebaseBreach struct {
	ID          string         `json:"id"`
	Folders     []BreachFolder `json:"folders,omitempty"`
	Files       []BreachFile   `json:"files,omitempty"`
	Kind        Statute        `json:"kind"`
	Priority    BreachPriority `json:"priority"`
	Autofixable bool           `json:"autofixable"`
	Reason      string         `json:"reason"`
	Solution    string         `json:"solution"`
}

// 📦️CodebaseBundle holds the data fields for a codebase bundle record.
type CodebaseBundle struct {
	ID           string                 `json:"id"`
	Folder       string                 `json:"folder"`
	URI          string                 `json:"uri"`
	Contributors []string               `json:"contributors,omitempty"`
	Tickets      []string               `json:"tickets,omitempty"`
	Metrics      *BundleMetricsInternal `json:"metrics,omitempty"`
}

// 🔻️CodebaseFolder holds the data fields for a codebase folder record.
type CodebaseFolder struct {
	ID       string                 `json:"id"`
	Path     string                 `json:"path"`
	URI      string                 `json:"uri"`
	Name     string                 `json:"name"`
	ParentID *string                `json:"parentId,omitempty"`
	BundleID *string                `json:"bundleId,omitempty"`
	Metrics  *FolderMetricsInternal `json:"metrics,omitempty"`
}

// 🔗️FileBreachRef holds the data fields for a file breach ref record.
type FileBreachRef struct {
	Kind        Statute        `json:"kind"`
	Priority    BreachPriority `json:"priority"`
	Autofixable bool           `json:"autofixable"`
	Solution    string         `json:"solution"`
}

// ⬛️CodebaseFile holds the data fields for a codebase file record.
type CodebaseFile struct {
	ID       string               `json:"id"`
	Path     string               `json:"path"`
	URI      string               `json:"uri"`
	FolderID *string              `json:"folderId,omitempty"`
	BundleID *string              `json:"bundleId,omitempty"`
	Metrics  *FileMetricsInternal `json:"metrics,omitempty"`
	Breachs  []FileBreachRef      `json:"breachs,omitempty"`
}

// ⬜️CodebaseSection holds the data fields for a codebase section record.
type CodebaseSection struct {
	ID      string                  `json:"id"`
	Path    string                  `json:"path"`
	URI     string                  `json:"uri"`
	Metrics *SectionMetricsInternal `json:"metrics,omitempty"`
}

// 🟥️CodebaseDefinition holds the data fields for a codebase definition record.
type CodebaseDefinition struct {
	ID      string                     `json:"id"`
	Path    string                     `json:"path"`
	URI     string                     `json:"uri"`
	Metrics *DefinitionMetricsInternal `json:"metrics,omitempty"`
}

// 🤝️ContributorBundleContrib holds the data fields for a contributor bundle contrib record.
type ContributorBundleContrib struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

// 🟧️ContributorFolderContrib holds the data fields for a contributor folder contrib record.
type ContributorFolderContrib struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

// 🟨️ContributorFileContrib holds the data fields for a contributor file contrib record.
type ContributorFileContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

// 🟩️ContributorSectionContrib holds the data fields for a contributor section contrib record.
type ContributorSectionContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

// 🟦️ContributorDefinitionContrib holds the data fields for a contributor definition contrib record.
type ContributorDefinitionContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

// 🟪️ContributorContributionsInternal holds the data fields for a contributor contributions internal record.
type ContributorContributionsInternal struct {
	Bundles     []ContributorBundleContrib     `json:"bundles,omitempty"`
	Folders     []ContributorFolderContrib     `json:"folders,omitempty"`
	Files       []ContributorFileContrib       `json:"files,omitempty"`
	Sections    []ContributorSectionContrib    `json:"sections,omitempty"`
	Definitions []ContributorDefinitionContrib `json:"definitions,omitempty"`
}

// 🟫️ContributorMetricsInternal holds the data fields for a contributor metrics internal record.
type ContributorMetricsInternal struct {
	Checkpoints int `json:"checkpoints"`
	Tickets     int `json:"tickets"`
	Bundles     int `json:"bundles"`
	Folders     int `json:"folders"`
	Files       int `json:"files"`
	Lines       int `json:"lines"`
	Sections    int `json:"sections"`
	Definitions int `json:"definitions"`
}

// 💠️CodebaseContributor holds the data fields for a codebase contributor record.
type CodebaseContributor struct {
	ID            string                            `json:"id"`
	URI           string                            `json:"uri"`
	Path          string                            `json:"path"`
	Name          string                            `json:"name,omitempty"`
	Icons         *ContributorIcons                 `json:"icons,omitempty"`
	Emails        []string                          `json:"emails,omitempty"`
	Links         map[string]string                 `json:"links,omitempty"`
	Contributions *ContributorContributionsInternal `json:"contributions,omitempty"`
	Metrics       *ContributorMetricsInternal       `json:"metrics,omitempty"`
}

// 🎫️TicketDateInfo holds the data fields for a ticket date info record.
type TicketDateInfo struct {
	Created  string `json:"created,omitempty"`
	Finished string `json:"finished,omitempty"`
}

// 🔳️TicketBundleContribInfo holds the data fields for a ticket bundle contrib info record.
type TicketBundleContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

// 🔲️TicketFolderContribInfo holds the data fields for a ticket folder contrib info record.
type TicketFolderContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

// ▪️TicketFileContribInfo holds the data fields for a ticket file contrib info record.
type TicketFileContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

// ▫️TicketSectionContribInfo holds the data fields for a ticket section contrib info record.
type TicketSectionContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

// ◾TicketDefinitionContrib holds the data fields for a ticket definition contrib record.
type TicketDefinitionContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

// 🗃️CodebaseTicket holds the data fields for a codebase ticket record.
type CodebaseTicket struct {
	ID          string                     `json:"id"`
	Path        string                     `json:"path"`
	URI         string                     `json:"uri"`
	Date        *TicketDateInfo            `json:"date,omitempty"`
	Checkpoint  string                     `json:"checkpoint,omitempty"`
	Year        string                     `json:"year"`
	Month       string                     `json:"month"`
	Day         string                     `json:"day"`
	Slug        string                     `json:"slug"`
	Prompt      string                     `json:"prompt,omitempty"`
	LLM         string                     `json:"llm,omitempty"`
	Author      string                     `json:"author,omitempty"`
	Status      TicketStatus               `json:"status"`
	Bundles     []TicketBundleContribInfo  `json:"bundles,omitempty"`
	Folders     []TicketFolderContribInfo  `json:"folders,omitempty"`
	Files       []TicketFileContribInfo    `json:"files,omitempty"`
	Sections    []TicketSectionContribInfo `json:"sections,omitempty"`
	Definitions []TicketDefinitionContrib  `json:"definitions,omitempty"`
}

// 📜️PolicyBreachRef holds the data fields for a policy breach ref record.
type PolicyBreachRef struct {
	Kind        Statute        `json:"kind"`
	Priority    BreachPriority `json:"priority"`
	Autofixable bool           `json:"autofixable"`
	Solution    string         `json:"solution"`
}

// ◽CodebasePolicy holds the data fields for a codebase policy record.
type CodebasePolicy struct {
	ID      string            `json:"id"`
	Name    string            `json:"name"`
	Scopes  []string          `json:"scopes,omitempty"`
	Breachs []PolicyBreachRef `json:"breachs,omitempty"`
}

// 🌳️CbTreeNodeKind represents a cb tree node kind value.
type CbTreeNodeKind string

const CbTreeNodeRepo CbTreeNodeKind = "repo"

const CbTreeNodeBundle CbTreeNodeKind = "bundle"

const CbTreeNodeFolder CbTreeNodeKind = "folder"

const CbTreeNodeFile CbTreeNodeKind = "file"

const CbTreeNodeSection CbTreeNodeKind = "section"

const CbTreeNodeDefinition CbTreeNodeKind = "definition"

// 🌿️CbTreeNode holds the data fields for a cb tree node record.
type CbTreeNode struct {
	Kind     CbTreeNodeKind         `json:"kind"`
	Children map[string]*CbTreeNode `json:"children,omitempty"`
}

// ◻Codebase holds the data fields for a codebase record.
type Codebase struct {
	Bundles      []CodebaseBundle       `json:"bundles"`
	Folders      []CodebaseFolder       `json:"folders"`
	Files        []CodebaseFile         `json:"files"`
	Sections     []CodebaseSection      `json:"sections"`
	Definitions  []CodebaseDefinition   `json:"definitions"`
	Contributors []CodebaseContributor  `json:"contributors"`
	Tickets      []CodebaseTicket       `json:"tickets"`
	Policies     []CodebasePolicy       `json:"policies"`
	Breachs      []CodebaseBreach       `json:"breachs"`
	Tree         map[string]*CbTreeNode `json:"tree"`
}

// #endregion ⭐️Codebase Types

// #region 📦️Utils

// 📜️policyAppliesToScope holds the data fields for a policyAppliesToScope record.
func policyAppliesToScope(policyID string, scope workspace.Scope) bool {
	switch policyID {
	case "code":
		return scope.Kind == workspace.ScopeFile && workspace.IsSourceFile(scope.FilePath)
	case "dev-docs":
		return scope.Kind == workspace.ScopeRepo || scope.Kind == workspace.ScopeFolder || scope.Kind == workspace.ScopeFile
	default:
		return true
	}
}

// 🎆️FormatYearDir returns the emoji-prefixed year directory segment.
func FormatYearDir(year int) string {
	return EmojiYear + workspace.PadNumber(year, 2)
}

// 🌙️FormatMonthDir returns the emoji-prefixed month directory segment.
func FormatMonthDir(month int) string {
	return EmojiMonth + workspace.PadNumber(month, 2)
}

// ☀️FormatDayDir returns the emoji-prefixed day directory segment.
func FormatDayDir(day int) string {
	return EmojiDay + workspace.PadNumber(day, 2)
}

// 🎫️FormatTicketRelPath returns YY/MM/DD/SLUG with emoji date segments.
func FormatTicketRelPath(year, month, day int, slug string) string {
	return FormatYearDir(year) + "/" + FormatMonthDir(month) + "/" + FormatDayDir(day) + "/" + slug
}

// #endregion 📦️Utils

// #region 📋️Tickets

// 🟣️StreamOptions holds the data fields for a stream options record.
type StreamOptions struct {
	ShowIgnored    bool
	ShowGenerated  bool
	ExcludeKinds   []string
	IncludeKinds   []string
	Filter         string
	Query          string
	Regex          bool
	MatchCase      bool
	MatchWholeWord bool

	ExcludeBundleKinds     []BundleKind
	IncludeBundleKinds     []BundleKind
	ExcludeFolderKinds     []FolderKind
	IncludeFolderKinds     []FolderKind
	ExcludeDefinitionKinds []DefinitionKind
	IncludeDefinitionKinds []DefinitionKind

	ExcludeYears        []int
	IncludeYears        []int
	ExcludeMonths       []int
	IncludeMonths       []int
	ExcludeDays         []int
	IncludeDays         []int
	ExcludeContributors []string
	IncludeContributors []string
	ExcludePolicies     []string
	IncludePolicies     []string
	ExcludeBreachs      []string
	IncludeBreachs      []string
}

// 🟤️matchesFilter holds the data fields for a matchesFilter record.
func MatchesFilter(name string, opts StreamOptions) bool {
	if opts.Filter == "" {
		return true
	}

	target := name
	pattern := opts.Filter

	if !opts.MatchCase && !opts.Regex {
		target = strings.ToLower(target)
		pattern = strings.ToLower(pattern)
	}

	if opts.Regex {
		re, err := regexp.Compile(opts.Filter)
		if err != nil {
			return false
		}
		return re.MatchString(name)
	}

	if opts.MatchWholeWord {
		words := strings.FieldsFunc(target, func(r rune) bool {
			return !((r >= 'a' && r <= 'z') || (r >= 'A' && r <= 'Z') || (r >= '0' && r <= '9') || r == '_')
		})
		for _, w := range words {
			if w == pattern {
				return true
			}
		}
		return false
	}

	return strings.Contains(target, pattern)
}

// 🔍️matchesQuery holds the data fields for a matchesQuery record.
func MatchesQuery(text string, opts StreamOptions) bool {
	if opts.Query == "" {
		return true
	}
	lowerText := strings.ToLower(text)
	lowerQuery := strings.ToLower(opts.Query)
	if strings.Contains(lowerText, lowerQuery) {
		return true
	}
	mapping := search.NewIndexMapping()
	index, err := search.NewMemOnly(mapping)
	if err != nil {
		return true
	}
	defer index.Close()
	doc := map[string]interface{}{"text": text}
	index.Index("item", doc)
	mq := search.NewMatchQuery(opts.Query)
	mq.SetFuzziness(2)
	searchRequest := search.NewSearchRequest(mq)
	searchRequest.Size = 1
	results, err := index.Search(searchRequest)
	if err != nil {
		return true
	}
	return results.Total > 0
}

// #endregion 📋️Tickets

// #region 🎥️GraphQL Context Port

// 🔌️RepoContext defines the interface for repo context operations.
type RepoContext interface {
	GetRootDir() string
	GetBundles() []*Bundle
	GetTechnologies() []*Technology
	GetCheckpoints(limit *int) ([]*Checkpoint, error)
	GetFolders() []*Folder
	GetFiles() []*File
	GetSections() []*Section
	GetDefinitions() []*Definition
	GetContributors() ([]*Contributor, error)
	GetGoals() ([]*Goal, error)
	GetTickets(year, month, day *int, status *TicketStatus) ([]*Ticket, error)
	GetPolicies() []*Policy
	GetDrafts() ([]*Draft, error)
	GetTodos(filter *FilterInput) ([]*Todo, error)
	GetStatutes() []*StatuteMeta
	GetInteractions() ([]InteractionResource, error)
	Analyze(scope *string) (*AnalyzeResult, error)
	GoalCreate(input GoalCreateInput) (*Goal, error)
	GoalChange(input GoalChangeInput) (*Goal, error)
	GoalClose(input GoalCloseInput) (*Goal, error)
	GoalReopen(input GoalReopenInput) (*Goal, error)
	GoalDelete(input GoalDeleteInput) (bool, error)
	TodoCreate(input TodoCreateInput) (*Todo, error)
	TodoChange(input TodoChangeInput) (*Todo, error)
	TodoDelete(id string) (bool, error)
	DraftCreate(input DraftCreateInput) (*Draft, error)
	DraftDelete(id string) (bool, error)
	TicketOpen(input TicketOpenInput) (*Ticket, error)
	TicketClose(input TicketCloseInput) (*Ticket, error)
	TicketReopen(input TicketReopenInput) (*Ticket, error)
	TicketChange(input TicketChangeInput) (*Ticket, error)
	TicketDelete(input TicketDeleteInput) (bool, error)
	FolderCreate(path string) (*Folder, error)
	FolderMove(src, dst string) (*Folder, error)
	FolderDelete(path string) error
	FileCreate(path string) (*File, error)
	FileMove(src, dst string) (*File, error)
	FileDelete(path string) error
	SectionCreate(file, name string, parent *string) (*Section, error)
	SectionMove(file, oldName, newName string) (*Section, error)
	SectionDelete(file, name string) error
	Integrate(source, targetSection, targetFile, targetParent *string) (*File, error)
	Extract(sourceFile, sourceSection, targetFile *string) (*File, error)
	ContributorAdd(input ContributorAddInput) (*Contributor, error)
	ContributorRemove(github string) error
	SyncManagement() (bool, error)
}

// #endregion 🎥️GraphQL Context Port

// #region 🩻️Default Context

// 🎯️parseMilestoneNumber holds the data fields for a parseMilestoneNumber record.
func ParseMilestoneNumber(milestone string) (int, error) {
	if n, err := strconv.Atoi(milestone); err == nil {
		return n, nil
	}
	parts := strings.Split(milestone, "/")
	if len(parts) > 0 {
		if n, err := strconv.Atoi(parts[len(parts)-1]); err == nil {
			return n, nil
		}
	}
	return 0, fmt.Errorf("could not parse milestone number from %s", milestone)
}

// #endregion 🩻️Default Context

// #region 🧬️Missing Utilities

// ❌️ScopeToFiles MUST return a non-nil error when the operation fails.
func ScopeToFiles(scope workspace.Scope, bundles []Bundle) ([]string, error) {
	ignorePatterns := []string{"**/node_modules/**", "**/.venv/**"}
	var files []string
	var err error
	switch scope.Kind {
	case workspace.ScopeRepo:
		files, err = workspace.GlobByExtension(workspace.RootDir, "**/*", []string{"ts", "tsx", "py", "cs", "go", "rs"}, ignorePatterns, true)
	case workspace.ScopeTechnology:
		for _, proj := range bundles {
			if proj.Name == scope.TechnologyName {
				files, err = workspace.GlobByExtension(workspace.RootDir, proj.Root+"/**/*", []string{"ts", "tsx", "py", "cs", "go", "rs"}, ignorePatterns, true)
				break
			}
		}
	case workspace.ScopeFolder:
		if scope.FilePath != "" {
			files, err = workspace.GlobByExtension(workspace.RootDir, scope.FilePath+"**/*", []string{"ts", "tsx", "py", "cs", "go", "rs"}, ignorePatterns, true)
		}
	case workspace.ScopeFile, workspace.ScopeSection, workspace.ScopeDefinition:
		if scope.FilePath != "" {
			files = []string{scope.FilePath}
		}
	}
	if err != nil {
		return nil, err
	}
	files = FilterConsideredFiles(files)
	files = FilterGitIgnored(files)
	return files, nil
}

// 🧹️filterConsideredFiles holds the data fields for a filterConsideredFiles record.
func FilterConsideredFiles(files []string) []string {
	if len(files) == 0 {
		return files
	}
	filtered := make([]string, 0, len(files))
	for _, filePath := range files {
		if workspace.IsRepoExcludedPath(filePath) {
			continue
		}
		if workspace.IsGitIgnored(filePath) {
			continue
		}
		filtered = append(filtered, filePath)
	}
	return filtered
}

// 🏗️buildBreachID holds the data fields for a buildBreachID record.
func BuildBreachID(scope string, line int, col int) string {
	if line > 0 && col > 0 {
		return fmt.Sprintf("repo/breach/%s#%d:%d", scope, line, col)
	}
	if line > 0 {
		return fmt.Sprintf("repo/breach/%s#%d", scope, line)
	}
	return fmt.Sprintf("repo/breach/%s", scope)
}

// 📦️GetBundleByPath MUST retrieve the requested value or return an error.
// 🔻️GetBundleByPath retrieves and returns the bundle by path.
func GetBundleByPath(path string) *Bundle {
	bundles := LookupBundles()
	normalizedPath := workspace.NormalizePath(path)
	var bestMatch *Bundle
	var matchedLen int
	for i := range bundles {
		bundle := &bundles[i]
		root := workspace.NormalizePath(bundle.Root)
		if strings.HasPrefix(normalizedPath, root+"/") || normalizedPath == root {
			if len(root) > matchedLen {
				bestMatch = bundle
				matchedLen = len(root)
			}
		}
	}
	return bestMatch
}

// 🔷️findBundleInfo holds the data fields for a findBundleInfo record.
func findBundleInfo(path string) (name, root string, ok bool) {
	bundles := LookupBundles()
	normalizedPath := workspace.NormalizePath(path)
	var matchedBundle string
	var matchedRoot string
	var matchedLen int
	for _, bundle := range bundles {
		root := workspace.NormalizePath(bundle.Root)
		if strings.HasPrefix(normalizedPath, root+"/") || normalizedPath == root {
			if len(root) > matchedLen {
				matchedBundle = bundle.Name
				matchedRoot = root
				matchedLen = len(root)
			}
		}
	}
	if matchedLen > 0 || matchedBundle != "" {
		return matchedBundle, matchedRoot, true
	}
	return "", "", false
}

// 🛤️resolveParentIDFromPath holds the data fields for a resolveParentIDFromPath record.
func ResolveParentIDFromPath(dirPath string) string {
	normalized := workspace.NormalizePath(dirPath)
	if normalized == "." || normalized == "" {
		bundle := GetBundleByPath(".")
		if bundle != nil {
			return bundle.GetID()
		}
		return ""
	}
	bundle := GetBundleByPath(normalized)
	if bundle != nil {
		bundleRoot := workspace.NormalizePath(bundle.Root)
		if normalized == bundleRoot {
			return bundle.GetID()
		}
		if strings.HasPrefix(normalized, bundleRoot+"/") {
			relPath := normalized[len(bundleRoot)+1:]
			parts := strings.Split(relPath, "/")
			parentID := bundle.GetID()
			currentPath := bundleRoot
			for _, part := range parts {
				currentPath += "/" + part
				kind := DeriveFolderKind(currentPath)
				if kind == FolderKindOrganization {
					parentID += EmojiText(EmojiFolderOrg) + workspace.Flat(part)
				} else {
					parentID += EmojiText(EmojiFolderRequired) + workspace.Flat(part)
				}
			}
			return parentID
		}
	}
	parts := strings.Split(normalized, "/")
	parentID := ""
	currentPath := ""
	for _, part := range parts {
		if currentPath == "" {
			currentPath = part
		} else {
			currentPath += "/" + part
		}
		kind := DeriveFolderKind(currentPath)
		if kind == FolderKindOrganization {
			parentID += EmojiText(EmojiFolderOrg) + workspace.Flat(part)
		} else {
			parentID += EmojiText(EmojiFolderRequired) + workspace.Flat(part)
		}
	}
	return parentID
}

// 📁️buildFolderID holds the data fields for a buildFolderID record.
func BuildFolderID(path string, bundleID *string) string {
	normalized := workspace.NormalizePath(path)
	if normalized == "." || normalized == "" {
		bundle := GetBundleByPath(".")
		if bundle != nil {
			return bundle.GetID()
		}
		return ""
	}
	dir := filepath.Dir(normalized)
	name := filepath.Base(normalized)
	parentID := ""
	bundle := GetBundleByPath(normalized)
	if bundle != nil {
		bundleRoot := workspace.NormalizePath(bundle.Root)
		if normalized == bundleRoot {
			return bundle.GetID()
		}
		if dir == bundleRoot {
			parentID = bundle.GetID()
		} else if strings.HasPrefix(dir, bundleRoot+"/") {
			parentID = ResolveParentIDFromPath(dir)
		} else {
			parentID = ResolveParentIDFromPath(dir)
		}
	} else if dir != "." && dir != "" {
		parentID = ResolveParentIDFromPath(dir)
	} else {
		repoBundle := GetBundleByPath(".")
		if repoBundle != nil {
			parentID = repoBundle.GetID()
		}
	}
	kind := DeriveFolderKind(normalized)
	if kind == FolderKindRoot {
		if bundle != nil {
			return bundle.GetID()
		}
	}
	if kind == FolderKindOrganization {
		return parentID + EmojiText(EmojiFolderOrg) + workspace.Flat(name)
	}
	return parentID + EmojiText(EmojiFolderRequired) + workspace.Flat(name)
}

// 📄️buildFileID holds the data fields for a buildFileID record.
func BuildFileID(path string, bundleID *string) string {
	normalized := workspace.NormalizePath(path)
	name := filepath.Base(normalized)
	kind := DeriveFileKind(name)
	dir := filepath.Dir(normalized)
	parentID := ""
	if dir != "." && dir != "" {
		bundle := GetBundleByPath(normalized)
		if bundle != nil && workspace.NormalizePath(bundle.Root) == workspace.NormalizePath(dir) {
			parentID = bundle.GetID()
		} else {
			parentID = ResolveParentIDFromPath(dir)
		}
	} else {
		bundle := GetBundleByPath(normalized)
		if bundle != nil {
			parentID = bundle.GetID()
		}
	}
	return GetArtifactID("file", map[string]interface{}{"path": normalized, "kind": kind, "parentId": parentID})
}

// 📑️buildSectionID holds the data fields for a buildSectionID record.
func BuildSectionID(fileID string, sectionPath []string) string {
	if len(sectionPath) == 0 || (len(sectionPath) == 1 && sectionPath[0] == "") {
		return fileID
	}
	var segments []string
	for _, s := range sectionPath {
		if s != "" {
			segments = append(segments, s)
		}
	}
	if len(segments) == 0 {
		return fileID
	}
	result := fileID
	for _, segment := range segments {
		emoji, name := ExtractEntityEmoji(segment)
		if emoji == "" {
			emoji = EmojiSection
		}
		result += EmojiText(emoji) + workspace.Flat(name)
	}
	return result
}

// 🧪️isTestFunctionName holds the data fields for a isTestFunctionName record.
func isTestFunctionName(name string) bool {

	for _, prefix := range []string{"Test", "Benchmark", "Fuzz"} {
		if strings.HasPrefix(name, prefix) && len(name) > len(prefix) && name[len(prefix)] >= 'A' && name[len(prefix)] <= 'Z' {
			return true
		}
	}

	if strings.HasPrefix(name, "test_") {
		return true
	}
	return false
}

// 📖️buildDefinitionID holds the data fields for a buildDefinitionID record.
func BuildDefinitionID(fileID string, sectionPath []string, name string, kind DefinitionKind) string {
	effectiveKind := kind
	if kind == DefinitionKindImplementation && strings.Contains(fileID, EmojiText(EmojiFileLab)) && isTestFunctionName(name) {
		effectiveKind = DefinitionKindTest
	}
	data := map[string]interface{}{"kind": string(effectiveKind)}
	return BuildSectionID(fileID, sectionPath) + definitionKindEmoji(data) + workspace.Flat(name)
}

// #endregion 🧬️Missing Utilities

// #region 🔊️Cli

// 🤝️ParseContributorIdentity MUST return the parsed result or an error for invalid input.
// 🔖️ParseContributorIdentity parses and returns the contributor identity from the input.
func ParseContributorIdentity(line string) (name, email string, ok bool) {
	re := regexp.MustCompile(`\d{4}\s+(.+?)\s*<([^>]+)>`)
	m := re.FindStringSubmatch(line)
	if m == nil {
		return "", "", false
	}
	return strings.TrimSpace(m[1]), strings.TrimSpace(m[2]), true
}

// 🧹️filterGitIgnored holds the data fields for a filterGitIgnored record.
func FilterGitIgnored(files []string) []string {
	if len(files) == 0 {
		return files
	}
	relPaths := make([]string, len(files))
	for i, filePath := range files {
		relPaths[i] = workspace.NormalizeRepoPath(filePath)
	}
	ignored := workspace.GetGitIgnoredSet(relPaths)
	filtered := make([]string, 0, len(files))
	for i, filePath := range files {
		normalized := workspace.NormalizeRepoPath(filePath)
		if normalized != "" && ignored[relPaths[i]] {
			continue
		}
		if normalized != "" && workspace.IsGitIgnored(normalized) {
			continue
		}
		filtered = append(filtered, filePath)
	}
	return filtered
}

// #endregion 🔊️Cli

// #region 🦀️Hooks

// 📡️HookEvent represents a lifecycle event kind for hooks.
type HookEvent string

const HookVersionCheckpointStarting HookEvent = "version.checkpoint.starting"

const HookVersionCheckpointEnded HookEvent = "version.checkpoint.ended"

const HookVersionCheckinStarting HookEvent = "version.checkin.starting"

const HookVersionCheckinEnded HookEvent = "version.checkin.ended"

const HookVersionCheckoutStarting HookEvent = "version.checkout.starting"

const HookVersionCheckoutEnded HookEvent = "version.checkout.ended"

const HookAgentStarted HookEvent = "agent.started"

const HookAgentEnded HookEvent = "agent.ended"

const HookAgentPromptSubmitting HookEvent = "agent.prompt.submitting"

const HookAgentCompacting HookEvent = "agent.compacting"

const HookAgentToolStarting HookEvent = "agent.tool.starting"

const HookAgentToolEnded HookEvent = "agent.tool.ended"

const HookAgentToolPlanUpdatingStarting HookEvent = "agent.tool.plan.updating.starting"

const HookAgentToolPlanUpdatingEnded HookEvent = "agent.tool.plan.updating.ended"

const HookAgentToolSearchStarting HookEvent = "agent.file.read.starting"

const HookAgentToolSearchEnded HookEvent = "agent.file.read.ended"

const HookAgentToolCodeEditStarting HookEvent = "agent.tool.code.edit.starting"

const HookAgentToolCodeEditEnded HookEvent = "agent.tool.code.edit.ended"

const HookAgentToolTestStarting HookEvent = "agent.tool.test.starting"

const HookAgentToolTestEnded HookEvent = "agent.tool.test.ended"

const HookAgentToolBuildStarting HookEvent = "agent.tool.build.starting"

const HookAgentToolBuildEnded HookEvent = "agent.tool.build.ended"

const HookAgentToolTerminalStarting HookEvent = "agent.tool.terminal.starting"

const HookAgentToolTerminalEnded HookEvent = "agent.tool.terminal.ended"

const HookAgentThinkingStarting HookEvent = "agent.thinking.starting"

const HookAgentThinkingEnded HookEvent = "agent.thinking.ended"

// 📋️AllHookEvents lists every valid hook event slug.
var AllHookEvents = []HookEvent{
	HookVersionCheckpointStarting,
	HookVersionCheckpointEnded,
	HookVersionCheckinStarting,
	HookVersionCheckinEnded,
	HookVersionCheckoutStarting,
	HookVersionCheckoutEnded,
	HookAgentStarted,
	HookAgentEnded,
	HookAgentPromptSubmitting,
	HookAgentCompacting,
	HookAgentToolStarting,
	HookAgentToolEnded,
	HookAgentToolPlanUpdatingStarting,
	HookAgentToolPlanUpdatingEnded,
	HookAgentToolSearchStarting,
	HookAgentToolSearchEnded,
	HookAgentToolCodeEditStarting,
	HookAgentToolCodeEditEnded,
	HookAgentToolTestStarting,
	HookAgentToolTestEnded,
	HookAgentToolBuildStarting,
	HookAgentToolBuildEnded,
	HookAgentToolTerminalStarting,
	HookAgentToolTerminalEnded,
	HookAgentThinkingStarting,
	HookAgentThinkingEnded,
}

// 🏷️HookKind categorizes a hook as either a git hook or an agent hook.
type HookKind string

const HookKindVersion HookKind = "version"

const HookKindAgent HookKind = "agent"

// 🎯️HookContext carries event metadata and a codebase handle for hook handlers.
type HookContext struct {
	Event      HookEvent         `json:"event"`
	Client     string            `json:"client"`
	Second     string            `json:"second"`
	RepoRoot   string            `json:"repoRoot"`
	ToolName   string            `json:"toolName,omitempty"`
	ToolArgs   string            `json:"toolArgs,omitempty"`
	FilePath   string            `json:"filePath,omitempty"`
	ParentInfo string            `json:"parentInfo,omitempty"`
	Extra      map[string]string `json:"extra,omitempty"`
	Input      json.RawMessage   `json:"input,omitempty"`
}

// 🔁️HookPlanStep represents a single step in a plan/task list update event.
type HookPlanStep struct {
	Name   string `json:"name"`
	Status string `json:"status"`
}

// 🔶️HookResult represents the outcome of a hook invocation with event-specific data.
type HookResult interface {
	IsAllowed() bool
	GetMessage() string
}

// 💻️HookResultBase provides common fields for all hook results.
type HookResultBase struct {
	Allowed bool   `json:"allowed"`
	Message string `json:"message,omitempty"`
	Raw     any    `json:"-"`
}

// 💿️IsAllowed holds the data fields for a IsAllowed record.
func (h HookResultBase) IsAllowed() bool { return h.Allowed }

// 🔹️GetMessage holds the data fields for a GetMessage record.
func (h HookResultBase) GetMessage() string { return h.Message }

// 🔸️HookResultAgentBase provides shared fields for all agent hook results.
type HookResultAgentBase struct {
	HookResultBase
	Checkpoint string `json:"checkpoint,omitempty"`
	Session    string `json:"session,omitempty"`
	Second     string `json:"second,omitempty"`
	Client     string `json:"client,omitempty"`
	LLM        string `json:"llm,omitempty"`
	Effort     string `json:"effort,omitempty"`
	Transcript string `json:"transcript,omitempty"`
	MessageID  string `json:"message,omitempty"`
	Parent     string `json:"parent"`
}

type HookResultVersionCheckpointStarting struct {
	HookResultBase
	Checkpoint  string `json:"checkpoint,omitempty"`
	Description string `json:"description,omitempty"`
}

type HookResultVersionCheckpointEnded struct {
	HookResultBase
	Checkpoint  string `json:"checkpoint,omitempty"`
	Description string `json:"description,omitempty"`
}

// ▶️HookResultAgentStarted represents the result of an agent started event.
type HookResultAgentStarted struct {
	HookResultAgentBase
}

// 🔺️HookResultAgentEnded represents the result of an agent ended event.
type HookResultAgentEnded struct {
	HookResultAgentBase
	Report string `json:"report,omitempty"`
}

// 🔻️HookResultAgentPromptSubmitting represents the result of an agent prompt submitting event.
type HookResultAgentPromptSubmitting struct {
	HookResultAgentBase
	Prompt string `json:"prompt,omitempty"`
}

// ⬛️HookResultAgentCompacting represents the result of an agent compacting event.
type HookResultAgentCompacting struct {
	HookResultAgentBase
	Chat string `json:"chat,omitempty"`
}

// ⬜️HookResultAgentToolStarting represents the result of an agent tool starting event.
type HookResultAgentToolStarting struct {
	HookResultAgentBase
	Name  string          `json:"name,omitempty"`
	Input json.RawMessage `json:"input,omitempty"`
}

// 🟥️HookResultAgentToolEnded represents the result of an agent tool ended event.
type HookResultAgentToolEnded struct {
	HookResultAgentBase
	Name     string          `json:"name,omitempty"`
	Input    json.RawMessage `json:"input,omitempty"`
	Response json.RawMessage `json:"response,omitempty"`
}

// 🟧️HookResultAgentToolPlanUpdating represents the result of an agent tool plan updating event.
type HookResultAgentToolPlanUpdating struct {
	HookResultAgentBase
	Steps []HookPlanStep `json:"steps,omitempty"`
}

// 🔎️HookResultAgentToolSearchStarting represents the result of an agent tool search starting event.
type HookResultAgentToolSearchStarting struct {
	HookResultAgentBase
	Pages       []string               `json:"pages,omitempty"`
	Ranges      []string               `json:"ranges,omitempty"`
	Definitions []HookSearchDefinition `json:"definitions,omitempty"`
}

// 🟨️HookResultAgentToolSearchEnded represents the result of an agent tool search ended event.
type HookResultAgentToolSearchEnded struct {
	HookResultAgentBase
	Pages       []string               `json:"pages,omitempty"`
	Ranges      []string               `json:"ranges,omitempty"`
	Definitions []HookSearchDefinition `json:"definitions,omitempty"`
	Error       string                 `json:"error,omitempty"`
}

// 📖️HookSearchDefinition holds search line coverage for a resolved definition.
type HookSearchDefinition struct {
	ID  string `json:"id,omitempty"`
	Loc int    `json:"loc,omitempty"`
}

// 🟩️HookResultAgentToolCodeEditStarting represents the result of an agent tool code edit starting event.
type HookResultAgentToolCodeEditStarting struct {
	HookResultAgentBase
	Path string `json:"path,omitempty"`
	Old  string `json:"old,omitempty"`
	New  string `json:"new,omitempty"`
	All  bool   `json:"all,omitempty"`
}

// 🟦️HookResultAgentToolCodeEditEnded represents the result of an agent tool code edit ended event.
type HookResultAgentToolCodeEditEnded struct {
	HookResultAgentBase
	Path string `json:"path,omitempty"`
	Old  string `json:"old,omitempty"`
	New  string `json:"new,omitempty"`
}

// 🧪️HookResultAgentToolTestStarting represents the result of an agent tool test starting event.
type HookResultAgentToolTestStarting struct {
	HookResultAgentBase
	Labs    []string `json:"labs,omitempty"`
	Tests   []string `json:"tests,omitempty"`
	Timeout string   `json:"timeout,omitempty"`
}

// 🟪️HookResultAgentToolTestEnded represents the result of an agent tool test ended event.
type HookResultAgentToolTestEnded struct {
	HookResultAgentBase
	Files     []string `json:"files,omitempty"`
	Succeeded []string `json:"succeeded,omitempty"`
	Failed    []string `json:"failed,omitempty"`
}

// 🔤️ValidateHookEvent checks if the given string is a valid hook event.
func ValidateHookEvent(s string) (HookEvent, error) {
	for _, e := range AllHookEvents {
		if string(e) == s {
			return e, nil
		}
	}
	return "", fmt.Errorf("invalid hook event %q, valid events: %s", s, strings.Join(hookEventStrings(), ", "))
}

func hookEventStrings() []string {
	result := make([]string, len(AllHookEvents))
	for i, e := range AllHookEvents {
		result[i] = string(e)
	}
	return result
}

// #endregion 🦀️Hooks

// #region 🧱️Artifact ID

// 💿️SemanticId holds the data fields for a semantic id record.
type SemanticId struct {
	Emoji string
	Value string
}

// 😀️emojiText holds the data fields for a emojiText record.
func EmojiText(emoji string) string {
	stripped := strings.ReplaceAll(emoji, "\uFE0E", "")
	textDefaultEmojis := []string{
		"\U0001F3D7", "\u2328", "\U0001F5B1", "\U0001F5C3",
		"\u2699", "\u2696", "\U0001F3F7", "\U0001F6E0",
		"\u2702", "\U0001F6E1", "\U0001F5D1",
		"\u2600", "\u23F1", "\u270F", "\U0001F46E",
		"\u2B05", "\u2B06", "\u2B07",
	}
	base := strings.ReplaceAll(stripped, "\uFE0F", "")
	for _, td := range textDefaultEmojis {
		if strings.Contains(base, td) {
			return strings.ReplaceAll(base, td, td+"\uFE0F")
		}
	}
	return base
}

// 🧲️extractEntityEmoji extracts the leading emoji and remaining text from a string.
// 🧲️Returns (emoji, remainingText). If no emoji is found, returns ("", original).
func ExtractEntityEmoji(s string) (string, string) {
	if s == "" {
		return "", ""
	}
	runes := []rune(s)
	if len(runes) == 0 {
		return "", ""
	}
	i := 0
	r := runes[i]
	if strings.ContainsRune("0123456789#*", r) {
		end := 1
		if end < len(runes) && runes[end] == 0xFE0F {
			end++
		}
		if end < len(runes) && runes[end] == 0x20E3 {
			return string(runes[:end+1]), string(runes[end+1:])
		}
		return "", s
	}
	// Check if the first rune is an emoji
	if !isEmojiRune(r) {
		return "", s
	}
	i++
	if r >= 0x1F1E6 && r <= 0x1F1FF && i < len(runes) && runes[i] >= 0x1F1E6 && runes[i] <= 0x1F1FF {
		i++
	}
	// Consume variation selectors and ZWJ sequences
	for i < len(runes) {
		r = runes[i]
		if r == 0xFE0F || r == 0xFE0E { // variation selectors
			i++
		} else if r == 0x20E3 { // combining enclosing keycap
			i++
		} else if r == 0x200D { // ZWJ
			i++
			if i < len(runes) {
				i++ // consume the next emoji after ZWJ
				// Continue consuming variation selectors
				for i < len(runes) && (runes[i] == 0xFE0F || runes[i] == 0xFE0E) {
					i++
				}
			}
		} else if r >= 0x1F3FB && r <= 0x1F3FF { // skin tone modifiers
			i++
		} else {
			break
		}
	}
	emoji := string(runes[:i])
	remaining := string(runes[i:])
	return emoji, remaining
}

// 🔷️isEmojiRune returns true if the rune is likely an emoji base character.
func isEmojiRune(r rune) bool {
	if r >= 0x1F1E6 && r <= 0x1F1FF {
		return true
	}
	// Common emoji ranges
	if r >= 0x1F600 && r <= 0x1F64F {
		return true
	} // Emoticons
	if r >= 0x1F300 && r <= 0x1F5FF {
		return true
	} // Misc Symbols and Pictographs
	if r >= 0x1F680 && r <= 0x1F6FF {
		return true
	} // Transport and Map
	if r >= 0x1F700 && r <= 0x1F77F {
		return true
	} // Alchemical Symbols
	if r >= 0x1F780 && r <= 0x1F7FF {
		return true
	} // Geometric Shapes Extended
	if r >= 0x1F800 && r <= 0x1F8FF {
		return true
	} // Supplemental Arrows-C
	if r >= 0x1F900 && r <= 0x1F9FF {
		return true
	} // Supplemental Symbols and Pictographs
	if r >= 0x1FA00 && r <= 0x1FA6F {
		return true
	} // Chess Symbols
	if r >= 0x1FA70 && r <= 0x1FAFF {
		return true
	} // Symbols and Pictographs Extended-A
	if r >= 0x2600 && r <= 0x26FF {
		return true
	} // Misc Symbols
	if r >= 0x2700 && r <= 0x27BF {
		return true
	} // Dingbats
	if r >= 0x2300 && r <= 0x23FF {
		return true
	} // Misc Technical
	if r >= 0x2B50 && r <= 0x2B55 {
		return true
	} // Stars
	if r >= 0x2B05 && r <= 0x2B07 {
		return true
	}
	if r >= 0x200D && r <= 0x200D {
		return true
	} // ZWJ
	if r >= 0xFE00 && r <= 0xFE0F {
		return true
	} // Variation selectors
	if r == 0x2139 || r == 0x2194 || r == 0x2195 {
		return true
	}
	if r >= 0x2196 && r <= 0x2199 {
		return true
	}
	if r >= 0x21A9 && r <= 0x21AA {
		return true
	}
	if r >= 0x231A && r <= 0x231B {
		return true
	}
	if r >= 0x25AA && r <= 0x25AB {
		return true
	}
	if r == 0x25B6 || r == 0x25C0 || r == 0x25FB || r == 0x25FC || r == 0x25FD || r == 0x25FE {
		return true
	}
	if r >= 0x2614 && r <= 0x2615 {
		return true
	}
	if r >= 0x2648 && r <= 0x2653 {
		return true
	}
	if r == 0x267F || r == 0x2693 || r == 0x26A1 {
		return true
	}
	if r >= 0x26AA && r <= 0x26AB {
		return true
	}
	if r >= 0x26BD && r <= 0x26BE {
		return true
	}
	if r >= 0x26C4 && r <= 0x26C5 {
		return true
	}
	if r == 0x26CE || r == 0x26D4 {
		return true
	}
	if r == 0x26EA || r == 0x26F2 || r == 0x26F3 || r == 0x26F5 || r == 0x26FA || r == 0x26FD {
		return true
	}
	if r == 0x2702 || r == 0x2705 {
		return true
	}
	if r >= 0x2708 && r <= 0x270D {
		return true
	}
	if r == 0x270F {
		return true
	}
	if r == 0x2712 || r == 0x2714 || r == 0x2716 || r == 0x271D || r == 0x2721 {
		return true
	}
	if r == 0x2728 {
		return true
	}
	if r >= 0x2733 && r <= 0x2734 {
		return true
	}
	if r == 0x2744 || r == 0x2747 || r == 0x274C || r == 0x274E {
		return true
	}
	if r >= 0x2753 && r <= 0x2755 {
		return true
	}
	if r == 0x2757 {
		return true
	}
	if r >= 0x2763 && r <= 0x2764 {
		return true
	}
	if r >= 0x2795 && r <= 0x2797 {
		return true
	}
	if r == 0x27A1 || r == 0x27B0 || r == 0x27BF {
		return true
	}
	if r >= 0x2934 && r <= 0x2935 {
		return true
	}
	if r >= 0x3030 && r <= 0x3030 {
		return true
	}
	if r == 0x303D || r == 0x3297 || r == 0x3299 {
		return true
	}
	if r == 0x00A9 || r == 0x00AE {
		return true
	} // © ®
	return false
}

// 🛠️resolveTechnologyEmoji looks up the emoji for a technology by name.
// 📺️Falls back to kind-derived emoji.
func resolveTechnologyEmoji(techName string) string {
	techs := LookupTechnologies()
	for _, t := range techs {
		if t.Name == techName {
			if t.Emoji != "" {
				return t.Emoji
			}
			break
		}
	}
	kind := DeriveTechnologyKind(techName)
	return string(kind)
}

// 🔤️String MUST return a non-empty string representation.
// 🔤️String returns the string representation of the semantic id.
func (s SemanticId) String() string {
	if s.Value == "" {
		return EmojiText(s.Emoji)
	}
	return fmt.Sprintf("%s%s", EmojiText(s.Emoji), s.Value)
}

// 🏷️technologyKindEmoji holds the data fields for a technologyKindEmoji record.
func technologyKindEmoji(data map[string]interface{}) string {
	if emoji, ok := data["emoji"].(string); ok && emoji != "" {
		return EmojiText(emoji)
	}
	if val, ok := data["kind"].(string); ok {
		switch val {
		case "infrastructure":
			return EmojiText(EmojiTechnologyInfra)
		case "research":
			return EmojiText(EmojiTechnologyResearch)
		case "mono":
			return EmojiText(EmojiTechnologyMono)
		case "user":
			return EmojiText(EmojiTechnologyUser)
		}
	}
	name, _ := data["name"].(string)
	if strings.Contains(name, "repo") {
		return EmojiText(EmojiTechnologyInfra)
	}
	if strings.HasPrefix(name, "coda") {
		return EmojiText(EmojiTechnologyResearch)
	}
	return EmojiText(EmojiTechnologyUser)
}

func bundleKindEmoji(data map[string]interface{}) string {
	if emoji, ok := data["emoji"].(string); ok && emoji != "" {
		return EmojiText(emoji)
	}
	bKind, _ := data["kind"].(string)
	switch bKind {
	case "schema":
		return EmojiText(EmojiBundleSchema)
	case "binary":
		return EmojiText(EmojiBundleBinary)
	case "ui":
		return EmojiText(EmojiBundleUI)
	case "site":
		return EmojiText(EmojiBundleSite)
	case "assets":
		return EmojiText(EmojiBundleAssets)
	case "library":
		return EmojiText(EmojiBundleLibrary)
	case "example":
		return EmojiText(EmojiBundleExample)
	}
	return EmojiText(EmojiBundleLibrary)
}

// 📄️fileKindEmoji holds the data fields for a fileKindEmoji record.
func fileKindEmoji(data map[string]interface{}) string {
	fkind, _ := data["kind"].(string)
	switch fkind {
	case "code":
		return EmojiText(EmojiFileCode)
	case "lab":
		return EmojiText(EmojiFileLab)
	case "script":
		return EmojiText(EmojiFileScript)
	case "docs":
		return EmojiText(EmojiFileDocs)
	case "config":
		return EmojiText(EmojiFileConfig)
	case "resource":
		return EmojiText(EmojiFileResource)
	case "template":
		return EmojiText(EmojiFileTemplate)
	case "license":
		return EmojiText(EmojiFileLicense)
	}
	return ""
}

// 📁️folderKindEmoji holds the data fields for a folderKindEmoji record.
func folderKindEmoji(data map[string]interface{}) string {
	fkind, _ := data["kind"].(string)
	if fkind == "organization" {
		return EmojiText(EmojiFolderOrg)
	} else if fkind == "root" {
		return EmojiText(EmojiFolderRoot)
	}
	return EmojiText(EmojiFolderRequired)
}

// 💎️DefinitionKindEmoji MUST return the same emoji for a kind everywhere it is rendered.
// ✂️DefinitionKindEmoji returns the emoji a definition kind renders as.
func DefinitionKindEmoji(kind DefinitionKind) string {
	switch kind {
	case DefinitionKindInterface:
		return EmojiText(EmojiDefinitionInterface)
	case DefinitionKindConstant:
		return EmojiText(EmojiDefinitionConstant)
	case DefinitionKindTest:
		return EmojiText(EmojiDefinitionTest)
	}
	return EmojiText(EmojiDefinitionImpl)
}

func definitionKindEmoji(data map[string]interface{}) string {
	kind, _ := data["kind"].(string)
	return DefinitionKindEmoji(DeriveDefinitionKind(kind))
}

// 🏺️goalArtifactID holds the data fields for a goalArtifactID record.
func goalArtifactID(rawGoalID string) string {
	parts := strings.Split(rawGoalID, "/")
	result := ""
	for _, p := range parts {
		result += EmojiText(EmojiGoal) + workspace.Flat(p)
	}
	return result
}

// ⛳️goalPathToComposeID converts a filesystem goal path (e.g. "AI-OPTIMIZED-REPO/REPO-CLI")
// 🖊️to the repo emoji ID format (e.g. "🎯️aioptimizedrepo🎯️repocli").
func GoalPathToComposeID(goalPath string) string {
	if goalPath == "" {
		return ""
	}
	return goalArtifactID(goalPath)
}

// 🛤️composeIDToGoalPath converts a repo emoji goal ID (e.g. "🎯️aioptimizedrepo🎯️repocli")
// back to its filesystem goal path (e.g. "AI-OPTIMIZED-REPO/REPO-CLI") by scanning the
// 🔑️goals directory and matching each segment's Flat() value.
func ComposeIDToGoalPath(composeID string) string {
	if composeID == "" {
		return ""
	}
	goalEmoji := EmojiText(EmojiGoal)
	trimmed := composeID
	if strings.HasPrefix(trimmed, goalEmoji) {
		trimmed = trimmed[len(goalEmoji):]
	}
	segments := strings.Split(trimmed, goalEmoji)
	if len(segments) == 0 {
		return ""
	}

	dir := workspace.GetRepoGoalsDir()
	var pathParts []string
	currentDir := dir

	for _, seg := range segments {
		if seg == "" {
			continue
		}
		entries, err := os.ReadDir(currentDir)
		if err != nil {
			return composeID
		}
		found := false
		for _, entry := range entries {
			if entry.IsDir() && workspace.Flat(entry.Name()) == seg {
				pathParts = append(pathParts, entry.Name())
				currentDir = filepath.Join(currentDir, entry.Name())
				found = true
				break
			}
		}
		if !found {
			return composeID
		}
	}
	return strings.Join(pathParts, "/")
}

// 🤝️contributorGithubToComposeID converts a contributor identifier (alias or github) to the compose contributor ID format.
func contributorGithubToComposeID(identifier string) string {
	if identifier == "" || identifier == "unknown" {
		return identifier
	}
	prefix := EmojiText(EmojiContributor)
	if strings.HasPrefix(identifier, prefix) {
		return identifier
	}
	return prefix + workspace.Flat(identifier)
}

// 🐙️composeIDToContributorGithub converts a compose contributor ID back to a contributor alias.
func composeIDToContributorGithub(composeID string) string {
	if composeID == "" || composeID == "unknown" {
		return composeID
	}
	prefix := EmojiText(EmojiContributor)
	if !strings.HasPrefix(composeID, prefix) {
		return composeID
	}
	flat := composeID[len(prefix):]
	contributors, err := LookupContributors()
	if err == nil {
		for _, c := range contributors {
			if workspace.Flat(c.Alias) == flat {
				return c.Alias
			}
		}
	}
	return flat
}

func interactionKindEmoji(data map[string]interface{}) string {
	kind, _ := data["kind"].(string)
	switch kind {
	case "edited":
		return EmojiText(EmojiInteractionEdited)
	case "finished":
		return EmojiText(EmojiInteractionFinished)
	case "restarted":
		return EmojiText(EmojiInteractionRestarted)
	case "deleted":
		return EmojiText(EmojiInteractionDeleted)
	}
	return EmojiText(EmojiInteractionStarted)
}

// 🔶️interactionKindFromEmoji holds the data fields for a interactionKindFromEmoji record.
func interactionKindFromEmoji(emoji string) string {
	switch emoji {
	case EmojiInteractionFinished:
		return "finished"
	case EmojiInteractionRestarted:
		return "restarted"
	case EmojiInteractionDeleted:
		return "deleted"
	case EmojiInteractionEdited:
		return "edited"
	}
	return "started"
}

// 📜️technologyKindCodeFromEmoji holds the data fields for a technologyKindCodeFromEmoji record.
func technologyKindCodeFromEmoji(emoji string) string {
	ne := EmojiText(emoji)
	switch ne {
	case EmojiText(EmojiTechnologyInfra):
		return "i"
	case EmojiText(EmojiTechnologyResearch):
		return "r"
	case EmojiText(EmojiTechnologyMono):
		return "m"
	}
	return "u"
}

// 📦️bundleKindCodeFromEmoji holds the data fields for a bundleKindCodeFromEmoji record.
func bundleKindCodeFromEmoji(emoji string) string {
	ne := EmojiText(emoji)
	switch ne {
	case EmojiText(EmojiBundleRepo):
		return "r"
	case EmojiText(EmojiBundleSchema):
		return "s"
	case EmojiText(EmojiBundleBinary):
		return "b"
	case EmojiText(EmojiBundleUI):
		return "u"
	case EmojiText(EmojiBundleExample):
		return "e"
	case EmojiText(EmojiBundleSite):
		return "w"
	case EmojiText(EmojiBundleAssets):
		return "a"
	}
	return "l"
}

// 🔹️folderKindCodeFromEmoji holds the data fields for a folderKindCodeFromEmoji record.
func folderKindCodeFromEmoji(emoji string) string {
	ne := EmojiText(emoji)
	if ne == EmojiText(EmojiFolderRequired) {
		return "req"
	}
	return "org"
}

// 📖️definitionKindCodeFromEmoji holds the data fields for a definitionKindCodeFromEmoji record.
func definitionKindCodeFromEmoji(emoji string) string {
	ne := EmojiText(emoji)
	switch ne {
	case EmojiText(EmojiDefinitionInterface):
		return "f"
	case EmojiText(EmojiDefinitionConstant):
		return "c"
	case EmojiText(EmojiDefinitionTest):
		return "t"
	}
	return "i"
}

// 🔸️technologyKindCode holds the data fields for a technologyKindCode record.
func technologyKindCode(data map[string]interface{}) string {
	if val, ok := data["kind"].(string); ok {
		switch val {
		case "infrastructure":
			return "i"
		case "research":
			return "r"
		case "mono":
			return "m"
		case "user":
			return "u"
		}
	}
	name, _ := data["name"].(string)
	if strings.Contains(name, "repo") {
		return "i"
	}
	if strings.HasPrefix(name, "coda") {
		return "r"
	}
	return "u"
}

// 🔺️TechnologyKindToCode holds the data fields for a TechnologyKindToCode record.
func TechnologyKindToCode(k TechnologyKind) string {
	switch k {
	case TechnologyKindInfrastructure:
		return "i"
	case TechnologyKindResearch:
		return "r"
	case TechnologyKindMono:
		return "m"
	}
	return "u"
}

// 🔻️bundleKindCode holds the data fields for a bundleKindCode record.
func bundleKindCode(data map[string]interface{}) string {
	bKind, _ := data["kind"].(string)
	switch bKind {
	case "repo":
		return "r"
	case "library":
		return "l"
	case "schema":
		return "s"
	case "binary":
		return "b"
	case "ui":
		return "u"
	case "example":
		return "e"
	case "site":
		return "w"
	case "assets":
		return "a"
	}
	return "l"
}

// ⬛️BundleKindToCode holds the data fields for a BundleKindToCode record.
func BundleKindToCode(k BundleKind) string {
	switch k {
	case BundleKindRepo:
		return "r"
	case BundleKindSchema:
		return "s"
	case BundleKindBinary:
		return "b"
	case BundleKindUI:
		return "u"
	case BundleKindSite:
		return "w"
	case BundleKindAssets:
		return "a"
	}
	return "l"
}

// ⬜️folderKindCode holds the data fields for a folderKindCode record.
func folderKindCode(data map[string]interface{}) string {
	fkind, _ := data["kind"].(string)
	if fkind == "organization" {
		return "org"
	}
	return "req"
}

func FolderKindToCode(k FolderKind) string {
	if k == FolderKindOrganization {
		return "org"
	}
	return "req"
}

// 🟥️definitionKindCode holds the data fields for a definitionKindCode record.
func definitionKindCode(data map[string]interface{}) string {
	kind, _ := data["kind"].(string)
	switch DeriveDefinitionKind(kind) {
	case DefinitionKindInterface:
		return "f"
	case DefinitionKindConstant:
		return "c"
	case DefinitionKindTest:
		return "t"
	}
	return "i"
}

// 🟧️interactionKindCode holds the data fields for a interactionKindCode record.
func interactionKindCode(data map[string]interface{}) string {
	kind, _ := data["kind"].(string)
	return kind
}

// 📑️containsUriSection holds the data fields for a containsUriSection record.
func containsUriSection(uri string) bool {
	return strings.HasPrefix(uri, "repo://section/")
}

// 🔗️containsUriDefinition holds the data fields for a containsUriDefinition record.
func containsUriDefinition(uri string) bool {
	return strings.HasPrefix(uri, "repo://definition/")
}

// 🟨️extractFileAndSectionsFromUri extracts the file path and section names from a URI.
// 🆕️Supports new format repo://section/{{id}} where ID encodes file+sections as emojis.
func extractFileAndSectionsFromUri(uri string) (filePath string, sectionSlugs []string) {
	// New format: repo://section/{{id}}
	if strings.HasPrefix(uri, "repo://section/") || strings.HasPrefix(uri, "repo://file/") {
		id := UriToId(uri)
		if id == "" {
			return "", nil
		}
		filePath = IdToPath(id)
		sectionSlugs = IdToSectionPath(id)
		return filePath, sectionSlugs
	}
	// Try the raw string as an ID (for cases where "repo://" was already stripped)
	prefixStripped := strings.TrimPrefix(uri, "repo://")
	if strings.HasPrefix(prefixStripped, "section/") || strings.HasPrefix(prefixStripped, "file/") {
		fullUri := "repo://" + prefixStripped
		id := UriToId(fullUri)
		if id != "" {
			return IdToPath(id), IdToSectionPath(id)
		}
	}
	// Legacy fallback: parse old /f/ and /s/ format
	p := prefixStripped
	var sectionParts []string
	remaining := p
	for {
		idx := strings.Index(remaining, "/s/")
		if idx < 0 {
			break
		}
		after := remaining[idx+3:]
		nextS := strings.Index(after, "/s/")
		nextD := strings.Index(after, "/d/")
		endIdx := len(after)
		if nextS >= 0 && nextS < endIdx {
			endIdx = nextS
		}
		if nextD >= 0 && nextD < endIdx {
			endIdx = nextD
		}
		sectionParts = append(sectionParts, workspace.PathFromUriPath(after[:endIdx]))
		remaining = after[endIdx:]
	}
	fIdx := strings.Index(p, "/f/")
	if fIdx >= 0 {
		rest := p[fIdx+3:]
		nextSlash := strings.Index(rest, "/")
		if nextSlash < 0 {
			filePath = workspace.PathFromUriPath(rest)
		} else {
			filePath = workspace.PathFromUriPath(rest[:nextSlash])
		}
	}
	return filePath, sectionParts
}

// 🟩️extractFileAndDefinitionFromUri extracts the file path and definition name from a URI.
// ▶️Supports new format repo://definition/{{id}} where ID encodes file+sections+definition as emojis.
func extractFileAndDefinitionFromUri(uri string) (filePath string, defName string) {
	// New format: repo://definition/{{id}}
	if strings.HasPrefix(uri, "repo://definition/") {
		id := UriToId(uri)
		if id == "" {
			return "", ""
		}
		filePath = IdToPath(id)
		defName = IdToDefinitionName(id)
		return filePath, defName
	}
	// Try with the raw string (for cases where "repo://" was already stripped)
	prefixStripped := strings.TrimPrefix(uri, "repo://")
	if strings.HasPrefix(prefixStripped, "definition/") {
		fullUri := "repo://" + prefixStripped
		id := UriToId(fullUri)
		if id != "" {
			return IdToPath(id), IdToDefinitionName(id)
		}
	}
	// Legacy fallback: parse old /f/ and /d/ format
	p := prefixStripped
	fIdx := strings.Index(p, "/f/")
	if fIdx >= 0 {
		rest := p[fIdx+3:]
		nextSlash := strings.Index(rest, "/")
		if nextSlash < 0 {
			filePath = workspace.PathFromUriPath(rest)
		} else {
			filePath = workspace.PathFromUriPath(rest[:nextSlash])
		}
	}
	dIdx := strings.LastIndex(p, "/d/")
	if dIdx >= 0 {
		rest := p[dIdx+3:]
		dParts := strings.SplitN(rest, "/", 2)
		if len(dParts) >= 2 {
			defName = workspace.PathFromUriPath(dParts[1])
		}
	}
	return filePath, defName
}

// 🟦️ArtifactRef holds the data fields for a artifact ref record.
type ArtifactRef struct {
	Kind         string
	Path         string
	SectionParts []string
}

// ❌️ParseArtifactRef MUST return the parsed result or an error for invalid input.
// 💾️ParseArtifactRef parses and returns the artifact ref from the input.
func ParseArtifactRef(ref string) ArtifactRef {
	clean := strings.ReplaceAll(ref, "\uFE0E", "")
	clean = strings.ReplaceAll(clean, "\uFE0F", "")
	if uri := IdToUri(clean); uri != "" && strings.HasPrefix(uri, "repo://") {
		if containsUriSection(uri) && !containsUriDefinition(uri) {
			filePath, sectionSlugs := extractFileAndSectionsFromUri(uri)
			return ArtifactRef{Kind: "section", Path: workspace.NormalizePath(filePath), SectionParts: sectionSlugs}
		}
		if containsUriDefinition(uri) {
			filePath, defName := extractFileAndDefinitionFromUri(uri)
			if defName != "" {
				return ArtifactRef{Kind: "definition", Path: workspace.NormalizePath(filePath + "§" + defName)}
			}
			return ArtifactRef{Kind: "definition", Path: workspace.NormalizePath(filePath)}
		}
	}

	sectionEmojis := []string{"🔖️"}
	for _, e := range sectionEmojis {
		if strings.HasPrefix(clean, e) {
			rest := clean[len(e):]
			parts := strings.SplitN(rest, "#", 2)
			filePath := workspace.NormalizePath(parts[0])
			var sectionParts []string
			if len(parts) > 1 {
				sectionParts = strings.Split(parts[1], "#")
			}
			return ArtifactRef{Kind: "section", Path: filePath, SectionParts: sectionParts}
		}
	}

	defEmojis := []string{"🏷️", "✂️", "🪨️", "🛠️"}
	for _, e := range defEmojis {
		if strings.HasPrefix(clean, e) {
			rest := clean[len(e):]
			return ArtifactRef{Kind: "definition", Path: workspace.NormalizePath(rest)}
		}
	}

	folderEmojis := []string{"📁️", "🗃️", "📂️"}
	for _, e := range folderEmojis {
		if strings.HasPrefix(clean, e) {
			rest := clean[len(e):]
			return ArtifactRef{Kind: "folder", Path: workspace.NormalizePath(strings.TrimSuffix(rest, "/"))}
		}
	}

	fileEmojis := []string{"💻️", "🥼️", "📜️", "📃️", "⚙️", "💾️", "⚖️", "📄️"}
	for _, e := range fileEmojis {
		if strings.HasPrefix(clean, e) {
			rest := clean[len(e):]
			return ArtifactRef{Kind: "file", Path: workspace.NormalizePath(rest)}
		}
	}

	if strings.Contains(ref, "#") {
		parts := strings.SplitN(ref, "#", 2)
		filePath := workspace.NormalizePath(parts[0])
		var sectionParts []string
		if len(parts) > 1 {
			sectionParts = strings.Split(parts[1], "#")
		}
		return ArtifactRef{Kind: "section", Path: filePath, SectionParts: sectionParts}
	}
	if strings.HasSuffix(ref, "/") {
		return ArtifactRef{Kind: "folder", Path: workspace.NormalizePath(strings.TrimSuffix(ref, "/"))}
	}
	normalized := workspace.NormalizePath(ref)
	absPath := filepath.Join(workspace.RootDir, normalized)
	if info, err := os.Stat(absPath); err == nil && info.IsDir() {
		return ArtifactRef{Kind: "folder", Path: normalized}
	}
	return ArtifactRef{Kind: "file", Path: normalized}
}

// 🟪️UnSlugify MUST complete the operation successfully.
func UnSlugify(slug string) string {
	parts := strings.Split(slug, "-")
	for i, p := range parts {
		if len(p) > 0 {
			parts[i] = strings.ToUpper(p[:1]) + strings.ToLower(p[1:])
		}
	}
	return strings.Join(parts, " ")
}

// 💠️SectionIdValueToUriPath MUST complete the operation successfully.
func SectionIdValueToUriPath(value string) string {
	hashIdx := strings.Index(value, "#")
	if hashIdx < 0 {
		return workspace.PathToUriPath(value)
	}
	filePath := value[:hashIdx]
	rest := value[hashIdx+1:]
	sectionParts := strings.Split(rest, "#")
	result := workspace.PathToUriPath(filePath)
	for _, p := range sectionParts {
		result += "/" + workspace.PathToUriPath(p)
	}
	return result
}

// 🔳️DefinitionIdValueToUriPath MUST complete the operation successfully.
func DefinitionIdValueToUriPath(value string) string {
	hashIdx := strings.Index(value, "#")
	paragraphIdx := strings.Index(value, "§")
	if hashIdx < 0 && paragraphIdx < 0 {
		return workspace.PathToUriPath(value)
	}
	if hashIdx < 0 && paragraphIdx >= 0 {
		filePath := value[:paragraphIdx]
		defName := value[paragraphIdx+len("§"):]
		return workspace.PathToUriPath(filePath) + "/" + workspace.PathToUriPath(defName)
	}
	filePath := value[:hashIdx]
	rest := value[hashIdx+1:]
	parts := strings.Split(rest, "§")
	result := workspace.PathToUriPath(filePath)
	for _, p := range parts {
		subParts := strings.Split(p, "#")
		for _, sp := range subParts {
			result += "/" + workspace.PathToUriPath(sp)
		}
	}
	return result
}

// 🔬️ParseSectionUriPath MUST return the parsed result or an error for invalid input.
// 📝️ParseSectionUriPath parses and returns the section uri path from the input.
func ParseSectionUriPath(uriPath string) (filePath string, sectionSlugs []string) {
	parts := strings.Split(uriPath, "/")
	fileEnd := -1
	for i, p := range parts {
		if strings.Contains(p, ".") {
			fileEnd = i
		}
	}
	if fileEnd < 0 {
		return uriPath, nil
	}
	filePath = strings.Join(parts[:fileEnd+1], "/")
	if fileEnd+1 < len(parts) {
		sectionSlugs = parts[fileEnd+1:]
	}
	return
}

// 🔲️StatuteIdToUriPath MUST complete the operation successfully.
func StatuteIdToUriPath(id string) string {
	parts := strings.Split(id, "/")
	for i, p := range parts {
		parts[i] = workspace.PathToUriPath(p)
	}
	return strings.Join(parts, "/")
}

// ▪️StatuteUriPathToId MUST complete the operation successfully.
func StatuteUriPathToId(uriPath string) string {
	parts := strings.Split(uriPath, "/")
	for i, p := range parts {
		parts[i] = workspace.PathFromUriPath(p)
	}
	return strings.Join(parts, "/")
}

// 📨️GetArtifactID MUST retrieve the requested value or return an error.
// 🟨️GetArtifactID retrieves and returns the artifact i d.
func GetArtifactID(kind string, data map[string]interface{}) string {
	parentId, _ := data["parentId"].(string)
	switch kind {
	case "root":
		return ""
	case "years":
		return parentId + EmojiText(EmojiYears)
	case "year":
		yy, _ := data["yy"].(string)
		return parentId + EmojiText(EmojiYear) + yy
	case "months":
		return parentId + EmojiText(EmojiMonths)
	case "month":
		mm, _ := data["mm"].(string)
		return parentId + EmojiText(EmojiMonth) + mm
	case "days":
		return parentId + EmojiText(EmojiDays)
	case "day":
		dd, _ := data["dd"].(string)
		return parentId + EmojiText(EmojiDay) + dd
	case "hours":
		return parentId + EmojiText(EmojiHours)
	case "hour":
		hh, _ := data["hh"].(string)
		return parentId + EmojiText(EmojiHour) + hh
	case "minutes":
		return parentId + EmojiText(EmojiMinutes)
	case "minute":
		mm, _ := data["mm"].(string)
		return parentId + EmojiText(EmojiMinute) + mm
	case "seconds":
		return parentId + EmojiText(EmojiSeconds)
	case "second":
		ss, _ := data["ss"].(string)
		return parentId + EmojiText(EmojiSecond) + ss
	case "codebase":
		return parentId + EmojiText(EmojiCodebase)
	case "technologies":
		return parentId + EmojiText(EmojiTechnologies)
	case "technology":
		name, _ := data["name"].(string)
		return technologyKindEmoji(data) + workspace.Flat(name)
	case "bundles":
		return parentId + EmojiText(EmojiBundles)
	case "bundle":
		name, _ := data["name"].(string)
		parts := strings.SplitN(name, "/", 2)
		technologyCode := parts[0]
		bundleCode := technologyCode
		if len(parts) > 1 {
			bundleCode = parts[1]
		}
		techEmoji := resolveTechnologyEmoji(technologyCode)
		return EmojiText(techEmoji) + workspace.Flat(technologyCode) + bundleKindEmoji(data) + workspace.Flat(bundleCode)
	case "folders":
		return parentId + EmojiText(EmojiFolders)
	case "folder":
		path, _ := data["path"].(string)
		name := filepath.Base(path)
		return parentId + folderKindEmoji(data) + workspace.Flat(name)
	case "files":
		return parentId + EmojiText(EmojiFiles)
	case "file":
		path, _ := data["path"].(string)
		if path == "" {
			path, _ = data["id"].(string)
		}
		name := filepath.Base(path)
		ext := filepath.Ext(name)
		if ext != "" {
			name = strings.TrimSuffix(name, ext)
		}
		return parentId + fileKindEmoji(data) + workspace.Flat(name)
	case "sections":
		return parentId + EmojiText(EmojiSections)
	case "section":
		path, _ := data["path"].(string)
		if path == "" {
			filePath, _ := data["filePath"].(string)
			name, _ := data["name"].(string)
			if filePath != "" && name != "" {
				path = filePath + "#" + name
			} else if name != "" {
				path = name
			}
		}
		if parentId != "" {
			hashIdx := strings.LastIndex(path, "#")
			sectionName := path
			if hashIdx >= 0 {
				sectionName = path[hashIdx+1:]
			}
			return parentId + EmojiText(EmojiSection) + workspace.Flat(sectionName)
		}
		hashIdx := strings.Index(path, "#")
		if hashIdx >= 0 {
			filePath := workspace.NormalizePath(path[:hashIdx])
			sectionPart := path[hashIdx+1:]
			sectionPath := strings.Split(sectionPart, "#")
			fileId := BuildFileID(filePath, nil)
			return BuildSectionID(fileId, sectionPath)
		}
		name, _ := data["name"].(string)
		if name != "" {
			return EmojiText(EmojiSection) + workspace.Flat(name)
		}
		return EmojiText(EmojiSection) + workspace.Flat(path)
	case "definitions":
		return parentId + EmojiText(EmojiDefinitions)
	case "definition":
		k := definitionKindEmoji(data)
		name, _ := data["name"].(string)
		if name == "" {
			id, _ := data["id"].(string)
			if id != "" {
				paragraphIdx := strings.LastIndex(id, "§")
				if paragraphIdx >= 0 {
					name = id[paragraphIdx+len("§"):]
				} else {
					hashIdx := strings.LastIndex(id, "#")
					if hashIdx >= 0 {
						name = id[hashIdx+1:]
					} else {
						name = id
					}
				}
			}
		}
		if parentId != "" {
			return parentId + k + workspace.Flat(name)
		}
		sectionPath, _ := data["sectionPath"].(string)
		filePath, _ := data["filePath"].(string)
		if filePath == "" {
			id, _ := data["id"].(string)
			if id != "" {
				if paragraphIdx := strings.LastIndex(id, "§"); paragraphIdx >= 0 {
					name = id[paragraphIdx+len("§"):]
					if hashIdx := strings.Index(id[:paragraphIdx], "#"); hashIdx >= 0 {
						filePath = id[:hashIdx]
						sectionPath = strings.ReplaceAll(id[hashIdx+1:paragraphIdx], "#", "/")
					} else {
						filePath = id[:paragraphIdx]
					}
				} else if hashIdx := strings.LastIndex(id, "#"); hashIdx >= 0 {
					name = id[hashIdx+1:]
					filePath = id[:hashIdx]
				}
			}
		}
		if filePath != "" {
			fileId := BuildFileID(filePath, nil)
			sectionId := fileId
			if sectionPath != "" {
				sections := strings.Split(strings.ReplaceAll(sectionPath, "#", "/"), "/")
				sectionId = BuildSectionID(fileId, sections)
			}
			return sectionId + k + workspace.Flat(name)
		}
		return k + workspace.Flat(name)
	case "tickets":
		return parentId + EmojiText(EmojiTickets)
	case "ticket":
		if parentId == "" {
			if goalId, ok := data["goalId"].(string); ok && goalId != "" {
				parentId = goalArtifactID(goalId)
			}
		}
		slug, _ := data["slug"].(string)
		title, _ := data["title"].(string)
		if slug == "" && title != "" {
			slug = title
		}
		if slug == "" {
			if id, ok := data["id"].(string); ok && id != "" {
				return id
			}
		}
		return parentId + EmojiText(EmojiTicket) + workspace.Flat(slug)
	case "goals":
		return parentId + EmojiText(EmojiGoals)
	case "goal":
		id, _ := data["id"].(string)
		name := id
		if idx := strings.LastIndex(id, "/"); idx >= 0 {
			name = id[idx+1:]
		}
		return parentId + EmojiText(EmojiGoal) + workspace.Flat(name)
	case "drafts":
		return parentId + EmojiText(EmojiDrafts)
	case "draft":
		slug, _ := data["slug"].(string)
		if slug == "" {
			slug, _ = data["id"].(string)
		}
		return parentId + EmojiText(EmojiDraft) + workspace.Flat(slug)
	case "todos":
		return parentId + EmojiText(EmojiTodos)
	case "todo":
		slug, _ := data["id"].(string)
		return parentId + EmojiText(EmojiTodo) + workspace.Flat(slug)
	case "policies":
		return parentId + EmojiText(EmojiPolicies)
	case "policy":
		slug, _ := data["id"].(string)
		slug = strings.TrimPrefix(slug, "/")
		return parentId + EmojiText(EmojiPolicy) + workspace.Flat(slug)
	case "statutes":
		return ""
	case "statute":
		id, _ := data["id"].(string)
		return workspace.Flat(id)
	case "contributors":
		return parentId + EmojiText(EmojiContributors)
	case "contributor":
		alias, _ := data["alias"].(string)
		if alias == "" {
			alias, _ = data["github"].(string)
		}
		return EmojiText(EmojiContributor) + workspace.Flat(alias)
	case "checkpoints":
		return parentId + EmojiText(EmojiCheckpoints)
	case "checkpoint":
		sha, _ := data["sha"].(string)
		contributorId, _ := data["contributorId"].(string)
		if contributorId == "" {
			if authorId, ok := data["authorId"].(string); ok && authorId != "" {
				contributorId = EmojiText(EmojiContributor) + workspace.Flat(authorId)
			}
		}
		return contributorId + EmojiText(EmojiCheckpoint) + sha
	case "line":
		lineNum, _ := data["line"].(float64)
		return parentId + EmojiText(EmojiLine) + fmt.Sprintf("%d", int(lineNum))
	case "range":
		startLine, _ := data["startLine"].(float64)
		endLine, _ := data["endLine"].(float64)
		return parentId + EmojiText(EmojiLine) + fmt.Sprintf("%d", int(startLine)) + EmojiText(EmojiLine) + fmt.Sprintf("%d", int(endLine))
	case "breach":
		policyId := parentId
		affected, _ := data["affected"].(string)
		lineId, _ := data["lineId"].(string)
		secondId, _ := data["secondId"].(string)
		return policyId + EmojiText(EmojiBreach) + affected + EmojiText(EmojiBreachScope) + lineId + secondId
	case "breaches":
		return parentId + EmojiText(EmojiBreaches)
	case "interactions":
		return parentId + EmojiText(EmojiInteractions)
	case "interaction":
		k := interactionKindEmoji(data)
		secondId, _ := data["secondId"].(string)
		contributorId, _ := data["contributorId"].(string)
		entityID, _ := data["entityId"].(string)
		return secondId + contributorId + entityID + k
	case "sessions":
		return parentId + EmojiText(EmojiSessions)
	case "session":
		uuid, _ := data["uuid"].(string)
		if uuid == "" {
			uuid, _ = data["id"].(string)
		}
		return parentId + EmojiText(EmojiSession) + workspace.Flat(uuid)
	}
	return ""
}

// ▫️GetArtifactURI MUST retrieve the requested value or return an error.
// 🌐️GetArtifactURI retrieves and returns the artifact URI using the format repo://{{entity-kind}}/{{id}}.
func GetArtifactURI(kind string, data map[string]interface{}) string {
	id := GetArtifactID(kind, data)
	if id == "" {
		return "repo://" + kind
	}
	return "repo://" + kind + "/" + id
}

// 🏗️buildFolderUriFromPath holds the data fields for a buildFolderUriFromPath record.
func BuildFolderUriFromPath(path string) string {
	return GetArtifactURI("folder", map[string]interface{}{"path": path})
}

// ◾extractPathFromFolderUri extracts the filesystem path from a folder URI.
// 🟩️With the new repo://folder/{{id}} format, it resolves the emoji ID back to a path.
func extractPathFromFolderUri(uri string) string {
	id := UriToId("repo://" + uri)
	if id == "" {
		id = UriToId(uri)
	}
	if id == "" {
		return ""
	}
	return IdToPath(id)
}

// ◽extractPathFromFileUri extracts the filesystem path from a file URI.
// 🟫️With the new repo://file/{{id}} format, it resolves the emoji ID back to a path.
func extractPathFromFileUri(uri string) string {
	id := UriToId("repo://" + uri)
	if id == "" {
		id = UriToId(uri)
	}
	if id == "" {
		return ""
	}
	return IdToPath(id)
}

// ◻buildFileUriFromPath builds a file URI from a filesystem path.
func BuildFileUriFromPath(path string) string {
	id := BuildFileID(path, nil)
	if id == "" {
		return GetArtifactURI("file", map[string]interface{}{"path": path})
	}
	return "repo://file/" + id
}

// 📋️buildSectionUriFromPath builds a section URI from a path#section format.
func buildSectionUriFromPath(path string) string {
	return GetArtifactURI("section", map[string]interface{}{"path": path})
}

// ◼buildDefinitionUriFromIdValue builds a definition URI from an id value.
func buildDefinitionUriFromIdValue(id string, dkc string) string {
	return GetArtifactURI("definition", map[string]interface{}{"id": id})
}

// 🔵️IdToUri converts an emoji-based ID to a repo:// URI.
// ▫️New format: repo://{{entity-kind}}/{{id}}
func IdToUri(id string) string {
	normalized := strings.ReplaceAll(id, "\uFE0E", "")
	normalized = strings.ReplaceAll(normalized, "\uFE0F", "")
	normalized = strings.TrimSpace(normalized)
	if normalized == "" {
		return ""
	}
	kind := DetectEntityKindFromId(normalized)
	if kind == "" {
		return ""
	}
	return "repo://" + kind + "/" + id
}

// 🔴️DetectEntityKindFromId determines the entity kind from an emoji-based ID string.
func DetectEntityKindFromId(id string) string {
	normalized := strings.ReplaceAll(id, "\uFE0E", "")
	normalized = strings.ReplaceAll(normalized, "\uFE0F", "")
	norm := func(e string) string {
		r := strings.ReplaceAll(e, "\uFE0F", "")
		return strings.ReplaceAll(r, "\uFE0E", "")
	}
	hasPrefix := func(s, emoji string) bool {
		p := norm(emoji)
		return p != "" && strings.HasPrefix(s, p)
	}
	hasSuffix := func(s, emoji string) bool {
		p := norm(emoji)
		return p != "" && strings.HasSuffix(s, p)
	}
	contains := func(s, emoji string) bool {
		p := norm(emoji)
		return p != "" && strings.Contains(s, p)
	}
	// Check interaction suffixes first
	interactionEmojis := []string{EmojiInteractionStarted, EmojiInteractionEdited, EmojiInteractionFinished, EmojiInteractionRestarted, EmojiInteractionDeleted}
	for _, ie := range interactionEmojis {
		if hasSuffix(normalized, ie) {
			return "interaction"
		}
	}
	// Check definition emojis (must check before section since definitions contain sections)
	defEmojis := []string{EmojiDefinitionImpl, EmojiDefinitionInterface, EmojiDefinitionConstant, EmojiDefinitionTest}
	for _, de := range defEmojis {
		if contains(normalized, de) {
			return "definition"
		}
	}
	// Check section emoji
	if contains(normalized, EmojiSection) {
		return "section"
	}
	// Check contributor before file emojis (contributor emoji 🧑️‍💻️ contains 💻️ which matches EmojiFileCode)
	if contains(normalized, EmojiContributor) {
		return "contributor"
	}
	// Check file emojis
	fileEmojis := []string{EmojiFileCode, EmojiFileLab, EmojiFileScript, EmojiFileDocs, EmojiFileConfig, EmojiFileResource, EmojiFileLicense}
	for _, fe := range fileEmojis {
		if contains(normalized, fe) {
			return "file"
		}
	}
	// Check folder emojis
	folderEmojis := []string{EmojiFolderOrg, EmojiFolderRequired}
	for _, fe := range folderEmojis {
		if contains(normalized, fe) {
			return "folder"
		}
	}
	// Check technology/bundle emojis
	technologyEmojis := []string{EmojiTechnologyUser, EmojiTechnologyInfra, EmojiTechnologyResearch, EmojiTechnologyMono}
	bundleEmojis := []string{EmojiBundleLibrary, EmojiBundleSchema, EmojiBundleBinary, EmojiBundleUI, EmojiBundleExample, EmojiBundleSite, EmojiBundleAssets, EmojiBundleRepo}
	for _, pe := range technologyEmojis {
		if hasPrefix(normalized, pe) {
			for _, be := range bundleEmojis {
				if contains(normalized, be) {
					return "bundle"
				}
			}
			return "technology"
		}
	}

	// singularOrPlural is a helper for entity kinds where singular and plural emojis are the same.
	// If the ID has text after the emoji prefix → singular kind, otherwise → plural kind.
	singularOrPlural := func(emoji, singular, plural string) string {
		e := norm(emoji)
		if e != "" && strings.HasPrefix(normalized, e) {
			if len(normalized) > len(e) {
				return singular
			}
			return plural
		}
		return ""
	}

	// Collection emojis where singular/plural differ
	if hasPrefix(normalized, EmojiYears) && norm(EmojiYears) != norm(EmojiYear) {
		return "years"
	}
	if hasPrefix(normalized, EmojiYear) {
		return "year"
	}
	if hasPrefix(normalized, EmojiMonths) && norm(EmojiMonths) != norm(EmojiMonth) {
		return "months"
	}
	if hasPrefix(normalized, EmojiMonth) {
		return "month"
	}
	if hasPrefix(normalized, EmojiDays) && norm(EmojiDays) != norm(EmojiDay) {
		return "days"
	}
	if hasPrefix(normalized, EmojiDay) {
		return "day"
	}
	if hasPrefix(normalized, EmojiHours) && norm(EmojiHours) != norm(EmojiHour) {
		return "hours"
	}
	if hasPrefix(normalized, EmojiHour) {
		return "hour"
	}
	if hasPrefix(normalized, EmojiMinutes) && norm(EmojiMinutes) != norm(EmojiMinute) {
		return "minutes"
	}
	if hasPrefix(normalized, EmojiMinute) {
		return "minute"
	}
	if hasPrefix(normalized, EmojiSeconds) && norm(EmojiSeconds) != norm(EmojiSecond) {
		return "seconds"
	}
	if hasPrefix(normalized, EmojiSecond) {
		return "second"
	}
	if hasPrefix(normalized, EmojiTechnologies) {
		return "technologies"
	}
	if hasPrefix(normalized, EmojiBundles) {
		return "bundles"
	}
	if hasPrefix(normalized, EmojiFolders) {
		return "folders"
	}
	if hasPrefix(normalized, EmojiFiles) {
		return "files"
	}
	if hasPrefix(normalized, EmojiSections) {
		return "sections"
	}
	if hasPrefix(normalized, EmojiDefinitions) {
		return "definitions"
	}
	if hasPrefix(normalized, EmojiCodebase) {
		return "codebase"
	}
	// Entity kinds where singular and plural emojis are the same
	if k := singularOrPlural(EmojiTicket, "ticket", "tickets"); k != "" {
		return k
	}
	if k := singularOrPlural(EmojiGoal, "goal", "goals"); k != "" {
		return k
	}
	if k := singularOrPlural(EmojiDraft, "draft", "drafts"); k != "" {
		return k
	}
	if k := singularOrPlural(EmojiTodo, "todo", "todos"); k != "" {
		return k
	}
	if k := singularOrPlural(EmojiPolicy, "policy", "policies"); k != "" {
		return k
	}
	if k := singularOrPlural(EmojiBreach, "breach", "breaches"); k != "" {
		return k
	}
	// contributor already handled above via contains
	if hasPrefix(normalized, EmojiContributors) {
		return "contributors"
	}
	if k := singularOrPlural(EmojiCheckpoint, "checkpoint", "checkpoints"); k != "" {
		return k
	}
	if hasPrefix(normalized, EmojiInteractions) {
		return "interactions"
	}
	if k := singularOrPlural(EmojiSession, "session", "sessions"); k != "" {
		return k
	}
	return ""
}

// 🟠️UriToId converts a repo:// URI to an emoji-based ID.
// ◾New format: repo://{{entity-kind}}/{{id}} → {{id}}
func UriToId(uri string) string {
	if !strings.HasPrefix(uri, "repo://") {
		return ""
	}
	p := strings.TrimPrefix(uri, "repo://")
	if p == "" || p == "root" {
		return ""
	}
	// Find the first / to split kind from id
	slashIdx := strings.Index(p, "/")
	if slashIdx < 0 {
		// No id part, just the kind — return empty for entity kinds
		return ""
	}
	id := p[slashIdx+1:]
	return id
}

// 🟡️IdToPath converts an emoji-based artifact ID back to a filesystem path.
// It walks all tracked bundles and files, computes their IDs, and matches.
// ◽For file/folder/section/definition IDs, it returns the file path.
func IdToPath(id string) string {
	normalized := strings.ReplaceAll(id, "\uFE0E", "")
	normalized = strings.ReplaceAll(normalized, "\uFE0F", "")
	if normalized == "" {
		return ""
	}
	norm := func(s string) string {
		r := strings.ReplaceAll(s, "\uFE0E", "")
		return strings.ReplaceAll(r, "\uFE0F", "")
	}
	// Strip section and definition suffixes to get to the file-level ID.
	fileId := normalized
	sectionEmojis := []string{EmojiSection}
	definitionEmojis := []string{EmojiDefinitionImpl, EmojiDefinitionInterface, EmojiDefinitionConstant, EmojiDefinitionTest}
	// Find the first section or definition emoji to truncate to file ID.
	firstSectionIdx := -1
	for _, se := range sectionEmojis {
		nse := norm(se)
		idx := strings.Index(fileId, nse)
		if idx >= 0 && (firstSectionIdx < 0 || idx < firstSectionIdx) {
			firstSectionIdx = idx
		}
	}
	if firstSectionIdx < 0 {
		for _, de := range definitionEmojis {
			nde := norm(de)
			idx := strings.Index(fileId, nde)
			if idx >= 0 && (firstSectionIdx < 0 || idx < firstSectionIdx) {
				firstSectionIdx = idx
			}
		}
	}
	if firstSectionIdx > 0 {
		fileId = fileId[:firstSectionIdx]
	}
	// Now fileId should be a bundle+folder+file ID like "🧰️repo⌨️cli💻️main"
	// Walk all bundles and their files to find a match.
	bundles := LookupBundles()
	for _, b := range bundles {
		bundleId := norm(b.GetID())
		if !strings.HasPrefix(fileId, bundleId) {
			continue
		}
		bundleRoot := filepath.Join(workspace.RootDir, workspace.NormalizePath(b.Root))
		// Walk files in this bundle
		filepath.Walk(bundleRoot, func(path string, info os.FileInfo, err error) error {
			if err != nil || info.IsDir() {
				return nil
			}
			if workspace.IsRepoExcludedPath(path) || workspace.IsGitIgnored(path) {
				return nil
			}
			return nil
		})
		// Instead of walking, compute ID for the remainder after bundle ID.
		// The remainder encodes folder + file emojis.
		rest := fileId[len(bundleId):]
		if rest == "" {
			// This is a bundle-level ID, return bundle root
			return workspace.NormalizePath(b.Root)
		}
		// 📂️Walk the bundle directory and match file IDs
		var matchedPath string
		filepath.Walk(bundleRoot, func(path string, info os.FileInfo, err error) error {
			if err != nil {
				return nil
			}
			if info.IsDir() {
				return nil
			}
			if workspace.IsRepoExcludedPath(path) || workspace.IsGitIgnored(path) {
				return nil
			}
			relPath, _ := filepath.Rel(workspace.RootDir, path)
			relPath = workspace.NormalizePath(relPath)
			computedId := norm(BuildFileID(relPath, nil))
			if computedId == fileId {
				matchedPath = relPath
				return filepath.SkipAll
			}
			return nil
		})
		if matchedPath != "" {
			return matchedPath
		}
	}
	// Fallback: try to match folder IDs
	for _, b := range bundles {
		bundleId := norm(b.GetID())
		if !strings.HasPrefix(fileId, bundleId) {
			continue
		}
		bundleRoot := filepath.Join(workspace.RootDir, workspace.NormalizePath(b.Root))
		var matchedPath string
		filepath.Walk(bundleRoot, func(path string, info os.FileInfo, err error) error {
			if err != nil {
				return nil
			}
			if !info.IsDir() {
				return nil
			}
			if workspace.IsRepoExcludedPath(path) {
				return filepath.SkipDir
			}
			relPath, _ := filepath.Rel(workspace.RootDir, path)
			relPath = workspace.NormalizePath(relPath)
			computedId := norm(BuildFolderID(relPath, nil))
			if computedId == fileId {
				matchedPath = relPath
				return filepath.SkipAll
			}
			return nil
		})
		if matchedPath != "" {
			return matchedPath
		}
	}
	return ""
}

// 🟢️IdToSectionPath extracts the section path parts from an emoji-based ID.
// 🔵️Returns the section names as a slice.
func IdToSectionPath(id string) []string {
	normalized := strings.ReplaceAll(id, "\uFE0E", "")
	normalized = strings.ReplaceAll(normalized, "\uFE0F", "")
	norm := func(s string) string {
		r := strings.ReplaceAll(s, "\uFE0E", "")
		return strings.ReplaceAll(r, "\uFE0F", "")
	}
	sectionEmoji := norm(EmojiSection)
	definitionEmojis := []string{
		norm(EmojiDefinitionImpl),
		norm(EmojiDefinitionInterface),
		norm(EmojiDefinitionConstant),
		norm(EmojiDefinitionTest),
	}
	// Strip definition suffix if present
	forSections := normalized
	for _, de := range definitionEmojis {
		idx := strings.LastIndex(forSections, de)
		if idx >= 0 {
			forSections = forSections[:idx]
			break
		}
	}
	// 🟣️Find all section emoji occurrences
	var sections []string
	remaining := forSections
	for {
		idx := strings.Index(remaining, sectionEmoji)
		if idx < 0 {
			break
		}
		remaining = remaining[idx+len(sectionEmoji):]
		// Find the end of this section name (next section emoji or end of string)
		nextIdx := strings.Index(remaining, sectionEmoji)
		if nextIdx < 0 {
			sections = append(sections, remaining)
			break
		}
		sections = append(sections, remaining[:nextIdx])
		remaining = remaining[nextIdx:]
	}
	return sections
}

// 🟤️IdToDefinitionName extracts the definition name from an emoji-based ID.
func IdToDefinitionName(id string) string {
	normalized := strings.ReplaceAll(id, "\uFE0E", "")
	normalized = strings.ReplaceAll(normalized, "\uFE0F", "")
	norm := func(s string) string {
		r := strings.ReplaceAll(s, "\uFE0E", "")
		return strings.ReplaceAll(r, "\uFE0F", "")
	}
	definitionEmojis := []string{
		norm(EmojiDefinitionImpl),
		norm(EmojiDefinitionInterface),
		norm(EmojiDefinitionConstant),
		norm(EmojiDefinitionTest),
	}
	for _, de := range definitionEmojis {
		idx := strings.LastIndex(normalized, de)
		if idx >= 0 {
			return normalized[idx+len(de):]
		}
	}
	return ""
}

// ⚪️technologyEmojiFromCode holds the data fields for a technologyEmojiFromCode record.
func technologyEmojiFromCode(code string) string {
	switch code {
	case "i":
		return EmojiTechnologyInfra
	case "r":
		return EmojiTechnologyResearch
	case "m":
		return EmojiTechnologyMono
	}
	return EmojiTechnologyUser
}

func bundleEmojiFromCode(code string) string {
	switch code {
	case "r":
		return EmojiBundleRepo
	case "s":
		return EmojiBundleSchema
	case "b":
		return EmojiBundleBinary
	case "u":
		return EmojiBundleUI
	case "e":
		return EmojiBundleExample
	case "w":
		return EmojiBundleSite
	case "a":
		return EmojiBundleAssets
	}
	return EmojiBundleLibrary
}

// ⚫️folderEmojiFromCode holds the data fields for a folderEmojiFromCode record.
func folderEmojiFromCode(code string) string {
	if code == "req" {
		return EmojiFolderRequired
	}
	return EmojiFolderOrg
}

// 🩵️definitionEmojiFromCode holds the data fields for a definitionEmojiFromCode record.
func definitionEmojiFromCode(code string) string {
	switch code {
	case "f":
		return EmojiDefinitionInterface
	case "c":
		return EmojiDefinitionConstant
	case "t":
		return EmojiDefinitionTest
	}
	return EmojiDefinitionImpl
}

// 🩶️uriSubPathToId holds the data fields for a uriSubPathToId record.
func uriSubPathToId(parentId string, subPath string) string {
	if subPath == "fds" {
		return EmojiText(EmojiFolders)
	}
	if strings.HasPrefix(subPath, "fd/") {
		fdRest := strings.TrimPrefix(subPath, "fd/")
		fdParts := strings.SplitN(fdRest, "/", 3)
		if len(fdParts) < 2 {
			return ""
		}
		fEmoji := folderEmojiFromCode(fdParts[0])
		fName := fdParts[1]
		folderId := parentId
		_ = fEmoji
		_ = fName
		if len(fdParts) == 2 {
			return EmojiText(fEmoji) + workspace.Flat(fName)
		}
		return uriSubPathToId(folderId, fdParts[2])
	}
	if subPath == "fis" {
		return EmojiText(EmojiFiles)
	}
	if strings.HasPrefix(subPath, "f/") {
		fRest := strings.TrimPrefix(subPath, "f/")
		fParts := strings.SplitN(fRest, "/", 2)
		fileName := workspace.PathFromUriPath(fParts[0])
		ext := filepath.Ext(fileName)
		if ext != "" {
			fileName = strings.TrimSuffix(fileName, ext)
		}
		fileId := EmojiText(EmojiFileCode) + workspace.Flat(fileName)
		if len(fParts) == 1 {
			return fileId
		}
		return uriSubPathToIdFromFile(fileId, fParts[1])
	}
	if subPath == "ss" {
		return EmojiText(EmojiSections)
	}
	if strings.HasPrefix(subPath, "s/") {
		sRest := strings.TrimPrefix(subPath, "s/")
		sectionParts := splitSectionUri(sRest)
		result := parentId
		for _, sp := range sectionParts {
			result += EmojiText(EmojiSection) + workspace.PathFromUriPath(sp)
		}
		return result
	}
	if subPath == "ds" {
		return EmojiText(EmojiDefinitions)
	}
	if strings.HasPrefix(subPath, "d/") {
		dRest := strings.TrimPrefix(subPath, "d/")
		dParts := strings.SplitN(dRest, "/", 2)
		if len(dParts) < 2 {
			return ""
		}
		dEmoji := definitionEmojiFromCode(dParts[0])
		dName := workspace.PathFromUriPath(dParts[1])
		return parentId + EmojiText(dEmoji) + workspace.Flat(dName)
	}
	return ""
}

// 🩷️uriSubPathToIdFromFile holds the data fields for a uriSubPathToIdFromFile record.
func uriSubPathToIdFromFile(fileId string, subPath string) string {
	if subPath == "ss" {
		return EmojiText(EmojiSections)
	}
	if strings.HasPrefix(subPath, "s/") {
		sRest := strings.TrimPrefix(subPath, "s/")
		return parseSectionAndRest(fileId, sRest)
	}
	if subPath == "ds" {
		return EmojiText(EmojiDefinitions)
	}
	if strings.HasPrefix(subPath, "d/") {
		dRest := strings.TrimPrefix(subPath, "d/")
		dParts := strings.SplitN(dRest, "/", 2)
		if len(dParts) < 2 {
			return ""
		}
		dEmoji := definitionEmojiFromCode(dParts[0])
		dName := workspace.PathFromUriPath(dParts[1])
		return fileId + EmojiText(dEmoji) + workspace.Flat(dName)
	}
	return ""
}

func parseSectionAndRest(parentId string, sRest string) string {
	parts := strings.SplitN(sRest, "/s/", 2)
	sectionName := workspace.PathFromUriPath(parts[0])
	if dIdx := strings.Index(parts[0], "/d/"); dIdx >= 0 {
		sectionName = workspace.PathFromUriPath(parts[0][:dIdx])
		sectionId := parentId + EmojiText(EmojiSection) + sectionName
		dRest := parts[0][dIdx+3:]
		dParts := strings.SplitN(dRest, "/", 2)
		if len(dParts) < 2 {
			return ""
		}
		dEmoji := definitionEmojiFromCode(dParts[0])
		dName := workspace.PathFromUriPath(dParts[1])
		return sectionId + EmojiText(dEmoji) + workspace.Flat(dName)
	}
	sectionId := parentId + EmojiText(EmojiSection) + sectionName
	if len(parts) == 1 {
		return sectionId
	}
	return parseSectionAndRest(sectionId, parts[1])
}

// 💜️splitSectionUri holds the data fields for a splitSectionUri record.
func splitSectionUri(path string) []string {
	return strings.Split(path, "/s/")
}

func parseGoalUri(rest string) string {
	parts := strings.SplitN(rest, "/", 2)
	goalName := parts[0]
	result := EmojiText(EmojiGoal) + workspace.Flat(goalName)
	if len(parts) == 1 {
		return result
	}
	sub := parts[1]
	if strings.HasPrefix(sub, "g/") {
		return result + parseGoalUri(strings.TrimPrefix(sub, "g/"))
	}
	if strings.HasPrefix(sub, "tk/") {
		return EmojiText(EmojiTicket) + workspace.Flat(strings.TrimPrefix(sub, "tk/"))
	}
	if sub == "tks" {
		return EmojiText(EmojiTickets)
	}
	if sub == "gs" {
		return EmojiText(EmojiGoals)
	}
	return result
}

// #endregion 🧱️Artifact ID

// #region 🪨️Entity Rendering

// #region 🪨️Entity Rendering
// 🖊️Entity rendering functions for formatted output generation.
func extractCreatedStr(data map[string]interface{}) string {
	createdStr := ""
	if dates, ok := data["dates"].(map[string]interface{}); ok {
		createdStr, _ = dates["created"].(string)
		if createdStr == "" {
			createdStr, _ = dates["started"].(string)
		}
	}
	if createdStr == "" {
		if dates, ok := data["date"].(map[string]interface{}); ok {
			createdStr, _ = dates["created"].(string)
			if createdStr == "" {
				createdStr, _ = dates["started"].(string)
			}
		}
	}
	if createdStr == "" {
		createdStr, _ = data["createdAt"].(string)
	}
	if createdStr == "" {
		if started, ok := data["started"].(string); ok {
			createdStr = started
		}
	}
	return createdStr
}

// 💿️extractFinishedStr holds the data fields for a extractFinishedStr record.
func extractFinishedStr(data map[string]interface{}) string {
	finishedStr := ""
	if dates, ok := data["dates"].(map[string]interface{}); ok {
		finishedStr, _ = dates["finished"].(string)
	}
	if finishedStr == "" {
		if dates, ok := data["date"].(map[string]interface{}); ok {
			finishedStr, _ = dates["finished"].(string)
		}
	}
	if finishedStr == "" {
		finishedStr, _ = data["finishedAt"].(string)
	}
	if finishedStr == "" {
		if finished, ok := data["finished"].(string); ok {
			finishedStr = finished
		}
	}
	return finishedStr
}

func sanitizeProp(v string) string {
	v = strings.ReplaceAll(v, "\r\n", " ")
	v = strings.ReplaceAll(v, "\n", " ")
	v = strings.ReplaceAll(v, "\r", " ")
	v = strings.ReplaceAll(v, "`", "'")
	for strings.Contains(v, "  ") {
		v = strings.ReplaceAll(v, "  ", " ")
	}
	return strings.TrimSpace(v)
}

// 🔷️sanitizeSingleLine holds the data fields for a sanitizeSingleLine record.
func sanitizeSingleLine(v string) string {
	v = strings.ReplaceAll(v, "\r\n", " ")
	v = strings.ReplaceAll(v, "\n", " ")
	v = strings.ReplaceAll(v, "\r", " ")
	return v
}

// 🔶️collectEntityProps holds the data fields for a collectEntityProps record.
func CollectEntityProps(kind string, data map[string]interface{}, truncateDesc bool) []string {
	var props []string
	appendNonEmpty := func(vals ...string) {
		for _, v := range vals {
			if v != "" {
				v = sanitizeProp(v)
				if v != "" {
					props = append(props, v)
				}
			}
		}
	}
	switch kind {
	case "goal":
		title, _ := data["title"].(string)
		status, _ := data["status"].(string)
		createdStr := extractCreatedStr(data)
		createdDisplay := ""
		if createdStr != "" {
			if tParsed, err := workspace.ParseFlexibleTime(createdStr); err == nil {
				createdDisplay = "created " + identity.Time(tParsed)
			} else {
				createdDisplay = "created " + createdStr
			}
		}
		dueDate, _ := data["dueDate"].(string)
		if dueDate == "" {
			if dates, ok := data["dates"].(map[string]interface{}); ok {
				dueDate, _ = dates["due"].(string)
			}
		}
		dueDisplay := ""
		if dueDate != "" {
			if tParsed, err := workspace.ParseFlexibleTime(dueDate); err == nil {
				dueDisplay = identity.Time(tParsed)
			} else {
				dueDisplay = dueDate
			}
		}
		description, _ := data["description"].(string)
		if truncateDesc && len(description) > 80 {
			description = description[:80] + "..."
		}
		appendNonEmpty(title, status, createdDisplay, dueDisplay, description)
	case "ticket":
		title, _ := data["title"].(string)
		status, _ := data["status"].(string)
		createdStr := extractCreatedStr(data)
		finishedStr := extractFinishedStr(data)
		dateDisplay := ""
		if status == "open" || status == "OPEN" {
			if createdStr != "" {
				if tParsed, err := workspace.ParseFlexibleTime(createdStr); err == nil {
					dateDisplay = "opened " + identity.Time(tParsed)
				} else {
					dateDisplay = "opened " + createdStr
				}
			}
		} else {
			if finishedStr != "" {
				if tParsed, err := workspace.ParseFlexibleTime(finishedStr); err == nil {
					dateDisplay = "closed " + identity.Time(tParsed)
				} else {
					dateDisplay = "closed " + finishedStr
				}
			} else if createdStr != "" {
				if tParsed, err := workspace.ParseFlexibleTime(createdStr); err == nil {
					dateDisplay = identity.Time(tParsed)
				}
			}
		}
		prompt, _ := data["prompt"].(string)
		summary, _ := data["summary"].(string)
		content := ""
		if status == "open" || status == "OPEN" {
			content = prompt
		} else {
			content = summary
		}
		if truncateDesc && len(content) > 80 {
			content = content[:80] + "..."
		}
		appendNonEmpty(title, status, dateDisplay, content)
	case "bundle":
		root, _ := data["root"].(string)
		ptype, _ := data["projectType"].(string)
		if ptype == "" {
			ptype, _ = data["type"].(string)
		}
		appendNonEmpty(root, ptype)
	case "folder":
	case "file":
	case "section":
		start, _ := data["startLine"].(float64)
		end, _ := data["endLine"].(float64)
		if start > 0 || end > 0 {
			appendNonEmpty(fmt.Sprintf(":%d-%d", int(start), int(end)))
		}
	case "definition":
		name, _ := data["name"].(string)
		start, _ := data["startLine"].(float64)
		end, _ := data["endLine"].(float64)
		appendNonEmpty(name)
		if start > 0 || end > 0 {
			appendNonEmpty(fmt.Sprintf(":%d-%d", int(start), int(end)))
		}
	case "contributor":
		name, _ := data["name"].(string)
		appendNonEmpty(name)
	case "todo":
		name, _ := data["name"].(string)
		appendNonEmpty(name)
	case "draft":
	case "policy":
		desc, _ := data["description"].(string)
		appendNonEmpty(desc)
	case "statute":
		desc, _ := data["description"].(string)
		appendNonEmpty(desc)
	case "technology":
		desc, _ := data["description"].(string)
		appendNonEmpty(desc)
	case "checkpoint":
		msg, _ := data["message"].(string)
		appendNonEmpty(msg)
	case "root":
		name, _ := data["name"].(string)
		appendNonEmpty(name)
	}
	return props
}

// #endregion 🪨️Entity Rendering
func RenderEntityHuman(kind string, data map[string]interface{}, isTTY bool) string {
	id := GetArtifactID(kind, data)
	props := CollectEntityProps(kind, data, false)
	return sanitizeSingleLine(RenderTemplate(TextTpl, "text/entity", map[string]interface{}{
		"ID":    id,
		"Props": props,
		"IsTTY": isTTY,
	}))
}

func InferEntityKind(key string) string {
	key = strings.ToLower(key)
	prefixes := []struct {
		prefix string
		kind   string
	}{
		{"ticket", "ticket"},
		{"goal", "goal"},
		{"file", "file"},
		{"folder", "folder"},
		{"section", "section"},
		{"definition", "definition"},
		{"contributor", "contributor"},
		{"todo", "todo"},
		{"draft", "draft"},
		{"policy", "policy"},
		{"breachkind", "statute"},
		{"bundle", "bundle"},
		{"technology", "technology"},
		{"checkpoint", "checkpoint"},
		{"repo", "root"},
		{"syncgithub", "root"},
		{"syncmanagement", "root"},
		{"integrate", "file"},
		{"extract", "file"},
		{"fix", "root"},
	}
	for _, p := range prefixes {
		if strings.HasPrefix(key, p.prefix) {
			return p.kind
		}
	}
	return ""
}

// 📰️renderEntityMarkdownLink holds the data fields for a renderEntityMarkdownLink record.
func RenderEntityMarkdownLink(kind string, data map[string]interface{}) string {
	id := GetArtifactID(kind, data)
	uri := GetArtifactURI(kind, data)
	props := CollectEntityProps(kind, data, false)
	return sanitizeSingleLine(RenderTemplate(MdTpl, "md/entity_link", map[string]interface{}{
		"ID":    id,
		"URI":   uri,
		"Props": props,
	}))
}

// 🔹️renderEntityMarkdown holds the data fields for a renderEntityMarkdown record.
func RenderEntityMarkdown(kind string, data map[string]interface{}) string {
	id := GetArtifactID(kind, data)
	uri := GetArtifactURI(kind, data)
	props := CollectEntityProps(kind, data, false)
	return sanitizeSingleLine(RenderTemplate(MdTpl, "md/entity_item", map[string]interface{}{
		"ID":    id,
		"URI":   uri,
		"Props": props,
	}))
}

// 🔸️getTerminalWidth holds the data fields for a getTerminalWidth record.
func GetTerminalWidth() int {
	w := os.Getenv("COLUMNS")
	if w != "" {
		if n, err := strconv.Atoi(w); err == nil && n > 0 {
			return n
		}
	}
	return 120
}

func TruncateANSI(s string, maxVisible int) string {
	if maxVisible <= 0 {
		return s
	}
	visible := 0
	for i := 0; i < len(s); {
		if s[i] == 0x1b {
			j := i + 1
			for j < len(s) && s[j] != 'm' {
				j++
			}
			if j < len(s) {
				j++
			}
			i = j
			continue
		}
		_, size := utf8.DecodeRuneInString(s[i:])
		if size <= 0 {
			break
		}
		i += size
		visible++
		if visible > maxVisible {
			break
		}
	}
	if visible <= maxVisible {
		return s
	}
	if maxVisible <= 3 {
		maxVisible = 3
	}
	limit := maxVisible - 3
	var b strings.Builder
	visible = 0
	for i := 0; i < len(s); {
		if s[i] == 0x1b {
			j := i + 1
			for j < len(s) && s[j] != 'm' {
				j++
			}
			if j < len(s) {
				j++
			}
			b.WriteString(s[i:j])
			i = j
			continue
		}
		r, size := utf8.DecodeRuneInString(s[i:])
		if size <= 0 {
			break
		}
		if visible >= limit {
			break
		}
		b.WriteRune(r)
		visible++
		i += size
	}
	b.WriteString("...")
	b.WriteString("\x1b[0m")
	return b.String()
}

// #endregion 🪨️Entity Rendering

// #region 🔭️Missing Hook Functions

// 🪪️normalizeTicketSessionID normalizes a ticket session ID.
func normalizeTicketSessionID(sessionID string) string {
	sessionID = strings.TrimSpace(sessionID)
	if sessionID == "" {
		return ""
	}
	sessionEmoji := EmojiText(EmojiSession)
	sessionStateEmojis := []string{
		EmojiText(EmojiSession),
		EmojiText(EmojiSessionRunning),
		EmojiText(EmojiSessionCompleted),
		EmojiText(EmojiSessionInterrupted),
	}
	prefix := ""
	payload := sessionID
	markerIndex := -1
	markerLen := 0
	for _, marker := range sessionStateEmojis {
		if marker == "" {
			continue
		}
		index := strings.Index(sessionID, marker)
		if index >= 0 && (markerIndex == -1 || index < markerIndex) {
			markerIndex = index
			markerLen = len(marker)
		}
	}
	if markerIndex >= 0 {
		prefix = sessionID[:markerIndex]
		payload = sessionID[markerIndex+markerLen:]
	}
	payload = workspace.Flat(payload)
	if payload == "" {
		return strings.TrimSpace(prefix)
	}
	return prefix + sessionEmoji + payload
}

// #endregion 🔭️Missing Hook Functions

// #region 💾️Missing Utility Functions

// 🏷️HookResultAgentThinkingEnded represents the result of an agent thinking ended event.
type HookResultAgentThinkingEnded struct {
	HookResultAgentBase
}

// ▶️HookResultAgentThinkingStarting represents the result of an agent thinking starting event.
type HookResultAgentThinkingStarting struct {
	HookResultAgentBase
}

// #endregion 💾️Missing Utility Functions

// #region 📰️Todos

// 🔸️SessionMeta holds session metadata.
type SessionMeta struct {
	ID          string           `json:"id"`
	URI         string           `json:"uri,omitempty"`
	Client      string           `json:"client,omitempty"`
	Second      string           `json:"second,omitempty"`
	Checkpoint  string           `json:"checkpoint,omitempty"`
	Contributor string           `json:"contributor,omitempty"`
	Transcript  string           `json:"transcript,omitempty"`
	Events      []EventEntry     `json:"events,omitempty"`
	Plan        *TicketAgentPlan `json:"plan,omitempty"`
}

// 📍️EventEntry represents an event entry in session metadata.
type EventEntry struct {
	Event  json.RawMessage `json:"event"`
	Native *struct {
		Event json.RawMessage `json:"event"`
	} `json:"native,omitempty"`
	Response *struct {
		Blocked *bool  `json:"blocked,omitempty"`
		Message string `json:"message,omitempty"`
		Reason  string `json:"reason,omitempty"`
	} `json:"response,omitempty"`
}

// 🔤️classifyCommandKind classifies a command string into a ToolKind.
func ClassifyCommandKind(command string) ToolKind {
	trimmed := strings.TrimSpace(command)
	if trimmed == "" {
		return ToolKindTerminal
	}
	segments := workspace.SplitCommandSegments(trimmed)
	if len(segments) > 1 {
		best := ToolKindTerminal
		for _, segment := range segments {
			kind := ClassifyCommandKind(segment)
			if kind == ToolKindTest || kind == ToolKindBuild || kind == ToolKindCodeEdit || kind == ToolKindPlan {
				return kind
			}
			if kind == ToolKindCodeSearch {
				best = ToolKindCodeSearch
			}
		}
		if best != ToolKindTerminal {
			return best
		}
	}

	parts := strings.Fields(trimmed)
	if len(parts) == 0 {
		return ToolKindTerminal
	}

	cmd := parts[0]
	baseCmd := filepath.Base(cmd)
	if baseCmd == "test" || baseCmd == "[" {
		return ToolKindCodeSearch
	}
	if strings.HasPrefix(baseCmd, "phpunit") || baseCmd == "rspec" || baseCmd == "pytest" || baseCmd == "py.test" || baseCmd == "jest" || baseCmd == "vitest" || baseCmd == "mocha" || baseCmd == "gradle" || baseCmd == "mvn" || baseCmd == "bundle" {
		return ToolKindTest
	}
	if strings.Contains(trimmed, "cargo nextest") || strings.HasPrefix(trimmed, "cargo nextest") {
		return ToolKindTest
	}
	testCmds := []string{"pytest", "jest", "mocha", "vitest", "rspec", "phpunit", "ctest", "bats"}
	for _, testCmd := range testCmds {
		if strings.Contains(trimmed, testCmd) {
			return ToolKindTest
		}
	}

	if strings.HasPrefix(trimmed, "npm test") || strings.HasPrefix(trimmed, "npm run test") ||
		strings.HasPrefix(trimmed, "pnpm test") || strings.HasPrefix(trimmed, "yarn test") ||
		strings.HasPrefix(trimmed, "bun test") {
		return ToolKindTest
	}
	if strings.HasPrefix(trimmed, "go test") {
		return ToolKindTest
	}
	if strings.HasPrefix(trimmed, "cargo test") || strings.HasPrefix(trimmed, "cargo nextest") {
		return ToolKindTest
	}
	if strings.Contains(trimmed, "pytest") || strings.Contains(trimmed, "unittest") {
		return ToolKindTest
	}
	if strings.HasPrefix(trimmed, "make test") || strings.HasPrefix(trimmed, "make check") {
		return ToolKindTest
	}
	if strings.HasPrefix(trimmed, "dotnet test") {
		return ToolKindTest
	}
	if strings.HasPrefix(trimmed, "swift test") || strings.HasPrefix(trimmed, "dart test") ||
		strings.HasPrefix(trimmed, "flutter test") || strings.HasPrefix(trimmed, "mix test") ||
		strings.HasPrefix(trimmed, "mvn test") || strings.HasPrefix(trimmed, "mvn verify") ||
		strings.HasPrefix(trimmed, "gradle test") || strings.HasPrefix(trimmed, "./gradlew test") || strings.HasPrefix(trimmed, "gradlew test") ||
		strings.HasPrefix(trimmed, "cabal test") || strings.HasPrefix(trimmed, "stack test") ||
		strings.HasPrefix(trimmed, "lein test") || strings.HasPrefix(trimmed, "sbt test") ||
		strings.HasPrefix(trimmed, "bundle exec rspec") || strings.HasPrefix(trimmed, "tox") || strings.Contains(trimmed, "phpunit") || strings.HasPrefix(trimmed, "rspec ") {
		return ToolKindTest
	}

	searchCmds := []string{"grep", "rg", "ripgrep", "ag", "ack", "ack-grep", "find", "fd", "fdfind",
		"locate", "mlocate", "ls", "exa", "eza", "tree", "dir", "cat", "bat", "batcat", "less",
		"more", "head", "tail", "wc", "file", "stat", "du", "which", "whereis", "type", "command",
		"hash", "diff", "cmp", "comm", "strings", "od", "xxd", "hexdump", "readlink", "realpath",
		"basename", "dirname", "jq", "yq", "xq", "sort", "uniq", "cut", "tr", "paste", "column",
		"rev", "fold", "fmt", "nl", "expand", "unexpand", "echo", "printf", "env", "printenv",
		"set", "export", "pwd", "id", "whoami", "hostname", "uname", "date", "uptime", "free",
		"df", "ps", "top", "htop", "lsof", "netstat", "ss", "test"}
	for _, searchCmd := range searchCmds {
		if baseCmd == searchCmd {
			return ToolKindCodeSearch
		}
	}
	if baseCmd == "sed" && !strings.Contains(trimmed, "-i") {
		return ToolKindCodeSearch
	}
	if (baseCmd == "awk" || baseCmd == "gawk" || baseCmd == "mawk" || baseCmd == "nawk") && !strings.Contains(trimmed, "-i") {
		return ToolKindCodeSearch
	}

	editCmds := []string{"rm", "mv", "cp", "install", "mkdir", "rmdir", "touch", "chmod", "chown",
		"chgrp", "ln", "tee", "patch", "truncate", "dd", "shred", "tar", "zip", "unzip", "gzip",
		"gunzip", "bzip2", "bunzip2", "xz", "unxz", "zstd"}
	for _, editCmd := range editCmds {
		if baseCmd == editCmd {
			return ToolKindCodeEdit
		}
	}
	if baseCmd == "sed" && strings.Contains(trimmed, "-i") {
		return ToolKindCodeEdit
	}
	if (baseCmd == "awk" || baseCmd == "gawk" || baseCmd == "mawk" || baseCmd == "nawk") && strings.Contains(trimmed, "-i") {
		return ToolKindCodeEdit
	}

	return ToolKindTerminal
}

// 📜️HookLogEntry is an alias for EventEntry for hook log entries.
type HookLogEntry = EventEntry

// ▶️HookResultAgentToolTerminalStarting represents the result of an agent terminal tool starting event.
type HookResultAgentToolTerminalStarting struct {
	HookResultAgentBase
	Name    string          `json:"name,omitempty"`
	Input   json.RawMessage `json:"input,omitempty"`
	Command string          `json:"command,omitempty"`
}

// ⬜️HookResultAgentToolTerminalEnded represents the result of an agent terminal tool ended event.
type HookResultAgentToolTerminalEnded struct {
	HookResultAgentBase
	Name       string          `json:"name,omitempty"`
	Input      json.RawMessage `json:"input,omitempty"`
	Output     json.RawMessage `json:"output,omitempty"`
	Command    string          `json:"command,omitempty"`
	PID        int             `json:"pid,omitempty"`
	Terminated bool            `json:"terminated,omitempty"`
	Stdout     json.RawMessage `json:"stdout,omitempty"`
	Stderr     json.RawMessage `json:"stderr,omitempty"`
}

// #endregion 📰️Todos

// #region 🔌️Model Ports

// 🧭️LookupBundles answers every known bundle. 🗂️codebase installs the real walk in its init.
var LookupBundles = func() []Bundle { return nil }

// 🧬️LookupTechnologies answers every known technology. 🗂️codebase installs the real walk.
var LookupTechnologies = func() []Technology { return nil }

// 🧑️LookupContributors answers every known contributor. 🧑️contributors installs the real read.
var LookupContributors = func() ([]Contributor, error) { return nil, nil }

// 🔗️LookupRepoContext builds the repository port for a root. 🔗️graphql installs the real one.
var LookupRepoContext = func(root string) RepoContext { return nil }

// #endregion 🔌️Model Ports

// #region 📋️Allowed Vocabulary

// 🤖️allowedLLMList answers the allowed LLM slugs from the module's schema table.
func AllowedLLMList() []string {
	values, err := AllowedLLMs()
	if err != nil {
		return nil
	}
	return values
}

// 🏋️allowedEffortList answers the allowed reasoning-effort slugs from the module's schema table.
func AllowedEffortList() []string {
	values, err := AllowedEfforts()
	if err != nil {
		return nil
	}
	return values
}

// 💻️allowedClientList answers the allowed client slugs from the module's schema table.
func AllowedClientList() []string {
	values, err := AllowedClients()
	if err != nil {
		return nil
	}
	return values
}

// #endregion 📋️Allowed Vocabulary

// #region 🪄️Template Functions

func TxtFuncMap() template.FuncMap {
	return template.FuncMap{
		"default": func(fallback, value interface{}) interface{} {
			if empty(value) {
				return fallback
			}
			return value
		},
		"upper":  strings.ToUpper,
		"lower":  strings.ToLower,
		"trim":   strings.TrimSpace,
		"join":   strings.Join,
		"quote":  strconv.Quote,
		"toJson": func(value interface{}) string { encoded, _ := json.Marshal(value); return string(encoded) },
		"indent": func(count int, value string) string {
			prefix := strings.Repeat(" ", count)
			return prefix + strings.ReplaceAll(value, "\n", "\n"+prefix)
		},
		"nindent": func(count int, value string) string {
			prefix := strings.Repeat(" ", count)
			return "\n" + prefix + strings.ReplaceAll(value, "\n", "\n"+prefix)
		},
		"replace":   strings.ReplaceAll,
		"contains":  strings.Contains,
		"hasPrefix": strings.HasPrefix,
		"hasSuffix": strings.HasSuffix,
		"list":      func(values ...interface{}) []interface{} { return values },
	}
}

func empty(value interface{}) bool {
	if value == nil {
		return true
	}
	switch item := value.(type) {
	case string:
		return item == ""
	case bool:
		return !item
	case int:
		return item == 0
	case []interface{}:
		return len(item) == 0
	default:
		return fmt.Sprint(item) == ""
	}
}

// #endregion 🪄️Template Functions

// #endregion 🚚️Split
