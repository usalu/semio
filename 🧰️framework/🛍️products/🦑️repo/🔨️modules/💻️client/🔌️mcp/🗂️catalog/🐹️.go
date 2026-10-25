// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// The build surface derived from the tool-neutral `repo` operations catalogue: one MCP tool per exposed operation or subcommand, its input schema, its argument vector and its bilingual description. Nothing in a tool is hand-written; the authored exposure policy only says which rows are exposed and how.

// #endregion 🧲️Header

package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

// #region 📖️CatalogueModel

// 🗣️Text is a text in both languages, keyed by language code.
type Text map[string]string

func (text Text) English() string { return text["en"] }
func (text Text) German() string  { return text["de"] }

// 🗣️Bilingual renders both languages, English first.
func (text Text) Bilingual() string {
	return "EN: " + text.English() + "\nDE: " + text.German()
}

// 🚩️CatalogueFlag is one flag row of the neutral catalogue.
type CatalogueFlag struct {
	Name        string   `json:"name"`
	Aliases     []string `json:"aliases"`
	Value       string   `json:"value"`
	ValueName   string   `json:"valueName"`
	Operations  []string `json:"operations"`
	Description Text     `json:"description"`
}

// 🧩️CatalogueSubcommand is one subcommand row; `Flags` is nil when the catalogue names no flags for it.
type CatalogueSubcommand struct {
	Name     string    `json:"name"`
	Synopsis string    `json:"synopsis"`
	Summary  Text      `json:"summary"`
	Flags    *[]string `json:"flags"`
}

// ⚙️CatalogueOperation is one operation row.
type CatalogueOperation struct {
	Name            string                `json:"name"`
	Requires        string                `json:"requires"`
	Synopsis        string                `json:"synopsis"`
	Summary         Text                  `json:"summary"`
	Implementations map[string]string     `json:"implementations"`
	DryRun          bool                  `json:"dryRun"`
	Progress        bool                  `json:"progress"`
	Cancellable     bool                  `json:"cancellable"`
	Flags           []string              `json:"flags"`
	Subcommands     []CatalogueSubcommand `json:"subcommands"`
}

// 📖️Catalogue is the neutral operations catalogue; members the model does not read are ignored so the catalogue may grow.
type Catalogue struct {
	Version    int                  `json:"version"`
	Languages  []string             `json:"languages"`
	Flags      []CatalogueFlag      `json:"flags"`
	Operations []CatalogueOperation `json:"operations"`
}

// 🔌️ParseBackendIds reads the ids of the build backend registry in registry order.
func ParseBackendIds(data []byte) ([]string, error) {
	var registry struct {
		Backends []struct {
			ID string `json:"id"`
		} `json:"backends"`
	}
	if err := json.Unmarshal(data, &registry); err != nil {
		return nil, fmt.Errorf("mcp: backend registry: %w", err)
	}
	ids := make([]string, 0, len(registry.Backends))
	for _, backend := range registry.Backends {
		if backend.ID == "" || strings.ContainsAny(backend.ID, "|{}[]<>=, \"") {
			return nil, fmt.Errorf("mcp: backend registry: unusable id %q", backend.ID)
		}
		ids = append(ids, backend.ID)
	}
	if len(ids) == 0 {
		return nil, errors.New("mcp: backend registry: no backends")
	}
	return ids, nil
}

// 📖️ParseCatalogue reads the catalogue document, expands the `{backends}` placeholder to the registered backend ids and refuses an unknown version or a missing language.
func ParseCatalogue(data []byte, backends []string) (Catalogue, error) {
	var catalogue Catalogue
	data = bytes.ReplaceAll(data, []byte("{backends}"), []byte(strings.Join(backends, "|")))
	if err := json.Unmarshal(data, &catalogue); err != nil {
		return Catalogue{}, fmt.Errorf("mcp: catalogue: %w", err)
	}
	if catalogue.Version != 1 || len(catalogue.Operations) == 0 {
		return Catalogue{}, errors.New("mcp: catalogue: unsupported version or no operations")
	}
	return catalogue, nil
}

func (catalogue Catalogue) operation(name string) (CatalogueOperation, bool) {
	for _, operation := range catalogue.Operations {
		if operation.Name == name {
			return operation, true
		}
	}
	return CatalogueOperation{}, false
}

func (catalogue Catalogue) flag(name, operation string) (CatalogueFlag, bool) {
	var fallback *CatalogueFlag
	for index, flag := range catalogue.Flags {
		if flag.Name != name {
			continue
		}
		if contains(flag.Operations, operation) {
			return flag, true
		}
		if fallback == nil || (len(fallback.Operations) > 0 && len(flag.Operations) == 0) {
			fallback = &catalogue.Flags[index]
		}
	}
	if fallback == nil {
		return CatalogueFlag{}, false
	}
	return *fallback, true
}

// #endregion 📖️CatalogueModel

// #region 🏗️PolicyModel

type PolicyIdentity struct {
	Argument    string           `json:"argument"`
	Pattern     string           `json:"pattern"`
	Description Text             `json:"description"`
	Environment AgentEnvironment `json:"environment"`
}

type PolicyGlobal struct {
	Flag        string `json:"flag"`
	Input       string `json:"input"`
	Environment string `json:"environment"`
	Description Text   `json:"description"`
}

type PolicyOmitted struct {
	Flag   string `json:"flag"`
	Reason Text   `json:"reason"`
}

type PolicyFlagType struct {
	Type    string   `json:"type"`
	Minimum *int64   `json:"minimum"`
	Maximum *int64   `json:"maximum"`
	Items   []string `json:"items"`
	Closed  bool     `json:"closed"`
}

type PolicyEnvironment struct {
	Variable    string   `json:"variable"`
	Enum        []string `json:"enum"`
	Description Text     `json:"description"`
}

type PolicyKind struct {
	Annotations ToolAnnotationsPolicy `json:"annotations"`
	TaskSupport string                `json:"taskSupport"`
	Note        Text                  `json:"note"`
}

// 🏷️ToolAnnotationsPolicy are the authored hints; an unset hint stays unset.
type ToolAnnotationsPolicy struct {
	ReadOnlyHint    *bool `json:"readOnlyHint"`
	DestructiveHint *bool `json:"destructiveHint"`
	IdempotentHint  *bool `json:"idempotentHint"`
	OpenWorldHint   *bool `json:"openWorldHint"`
}

type PolicyLimits struct {
	Concurrent         int `json:"concurrent"`
	CancelGraceMs      int `json:"cancelGraceMs"`
	ProgressIntervalMs int `json:"progressIntervalMs"`
	HeartbeatMs        int `json:"heartbeatMs"`
	CaptureBytes       int `json:"captureBytes"`
	StdoutBytes        int `json:"stdoutBytes"`
	StderrBytes        int `json:"stderrBytes"`
	DataBytes          int `json:"dataBytes"`
	TextBytes          int `json:"textBytes"`
	MaxArguments       int `json:"maxArguments"`
	MaxArgumentBytes   int `json:"maxArgumentBytes"`
}

type PolicyPositional struct {
	Name        string   `json:"name"`
	Description Text     `json:"description"`
	Pattern     string   `json:"pattern"`
	Suffix      string   `json:"suffix"`
	Enum        []string `json:"enum"`
	Optional    bool     `json:"optional"`
	Argv        bool     `json:"argv"`
}

type PolicyTool struct {
	Name        string                     `json:"name"`
	Operation   string                     `json:"operation"`
	Subcommand  string                     `json:"subcommand"`
	Kind        string                     `json:"kind"`
	Summary     Text                       `json:"summary"`
	Flags       *[]string                  `json:"flags"`
	Required    []string                   `json:"required"`
	AnyOf       [][]string                 `json:"anyOf"`
	Bind        map[string]json.RawMessage `json:"bind"`
	Defaults    map[string]json.RawMessage `json:"defaults"`
	Positional  []PolicyPositional         `json:"positional"`
	Environment []string                   `json:"environment"`
	Annotations *ToolAnnotationsPolicy     `json:"annotations"`
}

type PolicyExcluded struct {
	Operation  string `json:"operation"`
	Subcommand string `json:"subcommand"`
	Reason     Text   `json:"reason"`
}

// 🏗️Policy is the authored exposure policy of the build surface.
type Policy struct {
	Schema       string                       `json:"$schema"`
	Version      int                          `json:"version"`
	Catalogue    string                       `json:"catalogue"`
	BuildPolicy  string                       `json:"buildPolicy"`
	Backends     string                       `json:"backends"`
	Identity     PolicyIdentity               `json:"identity"`
	Globals      []PolicyGlobal               `json:"globals"`
	OmittedFlags []PolicyOmitted              `json:"omittedFlags"`
	Strip        []string                     `json:"strip"`
	Set          map[string]string            `json:"set"`
	FlagTypes    map[string]PolicyFlagType    `json:"flagTypes"`
	Environment  map[string]PolicyEnvironment `json:"environment"`
	Kinds        map[string]PolicyKind        `json:"kinds"`
	Limits       PolicyLimits                 `json:"limits"`
	Tools        []PolicyTool                 `json:"tools"`
	Excluded     []PolicyExcluded             `json:"excluded"`
}

// 🏗️ParsePolicy reads the exposure policy; an unknown member is refused because the policy is authored here.
func ParsePolicy(data []byte) (Policy, error) {
	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	var policy Policy
	if err := decoder.Decode(&policy); err != nil {
		return Policy{}, fmt.Errorf("mcp: build surface policy: %w", err)
	}
	if policy.Version != 1 || len(policy.Tools) == 0 {
		return Policy{}, errors.New("mcp: build surface policy: unsupported version or no tools")
	}
	return policy, nil
}

// #endregion 🏗️PolicyModel

// #region 🛠️Derivation

const (
	InputBoolean = "boolean"
	InputString  = "string"
	InputInteger = "integer"
	InputEnum    = "enum"
	InputList    = "list"
	InputArgv    = "argv"
)

// 🧾️Input is one input of a derived tool and the way it reaches the child.
type Input struct {
	Name        string
	Flag        string
	Separate    bool
	Positional  bool
	Kind        string
	Items       []string
	Minimum     *int64
	Maximum     *int64
	Pattern     *regexp.Regexp
	Suffix      string
	Required    bool
	Default     json.RawMessage
	Description string
	Variable    string
	Agent       bool
}

// 📌️Bound is a flag the server fixes: it is not an input.
type Bound struct {
	Flag  string
	Value json.RawMessage
}

// 🛠️BuildTool is one derived tool.
type BuildTool struct {
	Name       string
	Operation  string
	Subcommand string
	Kind       string
	Title      string
	Inputs     []Input
	Bound      []Bound
	AnyOf      [][]string
	Schema     Tool
	Limits     PolicyLimits
}

// 🌐️flagProperty names the input of a flag: `--dry-run` is `dry_run`.
func flagProperty(flag string) string {
	return strings.ReplaceAll(strings.TrimPrefix(flag, "--"), "-", "_")
}

// 🛠️ToolName names the tool of an operation row or a subcommand row.
func ToolName(operation, subcommand string) string {
	name := strings.ReplaceAll(operation, "-", "_")
	if subcommand != "" {
		name += "_" + strings.ReplaceAll(subcommand, "-", "_")
	}
	return name
}

// ⚙️effectiveFlags names the flags the catalogue gives an operation row or a subcommand row: the subcommand's own list when it has one, else the operation's.
func (catalogue Catalogue) effectiveFlags(operation CatalogueOperation, subcommand string) ([]string, error) {
	if subcommand == "" {
		return operation.Flags, nil
	}
	for _, candidate := range operation.Subcommands {
		if candidate.Name == subcommand {
			if candidate.Flags != nil {
				return *candidate.Flags, nil
			}
			return operation.Flags, nil
		}
	}
	return nil, fmt.Errorf("mcp: catalogue has no subcommand %s %s", operation.Name, subcommand)
}

func (catalogue Catalogue) summary(operation CatalogueOperation, subcommand string) (Text, string) {
	if subcommand == "" {
		return operation.Summary, operation.Synopsis
	}
	for _, candidate := range operation.Subcommands {
		if candidate.Name == subcommand {
			return candidate.Summary, candidate.Synopsis
		}
	}
	return operation.Summary, operation.Synopsis
}

func enumOf(valueName string) []string {
	if !strings.Contains(valueName, "|") {
		return nil
	}
	values := strings.Split(valueName, "|")
	for _, value := range values {
		if value == "" || strings.ContainsAny(value, "[]<>=, ") {
			return nil
		}
	}
	return values
}

// 🧮️Derive builds every exposed tool from the catalogue and the policy, and refuses a policy that names something the catalogue does not have, a flag no row accepts, or a row that is neither exposed nor excluded.
func Derive(catalogue Catalogue, policy Policy) ([]BuildTool, error) {
	var tools []BuildTool
	seen := map[string]bool{}
	identity, err := regexp.Compile(policy.Identity.Pattern)
	if err != nil {
		return nil, fmt.Errorf("mcp: identity pattern: %w", err)
	}
	globals := map[string]PolicyGlobal{}
	for _, global := range policy.Globals {
		globals[global.Flag] = global
	}
	omitted := map[string]bool{}
	for _, row := range policy.OmittedFlags {
		omitted[row.Flag] = true
	}
	for _, spec := range policy.Tools {
		operation, found := catalogue.operation(spec.Operation)
		if !found {
			return nil, fmt.Errorf("mcp: policy exposes unknown operation %q", spec.Operation)
		}
		name := spec.Name
		if name == "" {
			name = ToolName(spec.Operation, spec.Subcommand)
		}
		if seen[name] {
			return nil, fmt.Errorf("mcp: duplicate tool %q", name)
		}
		seen[name] = true
		kind, known := policy.Kinds[spec.Kind]
		if !known {
			return nil, fmt.Errorf("mcp: tool %q has unknown kind %q", name, spec.Kind)
		}
		catalogued, err := catalogue.effectiveFlags(operation, spec.Subcommand)
		if err != nil {
			return nil, err
		}
		flags := catalogued
		if spec.Flags != nil {
			flags = *spec.Flags
			for _, flag := range flags {
				if !contains(operation.Flags, flag) {
					return nil, fmt.Errorf("mcp: tool %q accepts %s, which the catalogue does not give %s", name, flag, spec.Operation)
				}
			}
		}
		summary, synopsis := catalogue.summary(operation, spec.Subcommand)
		if len(spec.Summary) > 0 {
			summary = spec.Summary
		}
		if summary.English() == "" || summary.German() == "" {
			return nil, fmt.Errorf("mcp: tool %q has no summary in both languages", name)
		}
		tool := BuildTool{Name: name, Operation: spec.Operation, Subcommand: spec.Subcommand, Kind: spec.Kind, Title: strings.TrimSpace("repo " + spec.Operation + " " + spec.Subcommand), Limits: policy.Limits}
		properties := map[string]Schema{}
		var required []string
		add := func(input Input, schema Schema) error {
			if _, duplicate := properties[input.Name]; duplicate {
				return fmt.Errorf("mcp: tool %q has two inputs named %q", name, input.Name)
			}
			properties[input.Name] = schema
			tool.Inputs = append(tool.Inputs, input)
			if input.Required {
				required = append(required, input.Name)
			}
			return nil
		}
		for _, positional := range spec.Positional {
			input := Input{Name: positional.Name, Positional: true, Kind: InputString, Required: !positional.Optional, Suffix: positional.Suffix, Description: positional.Description.Bilingual()}
			schema := Schema{Description: input.Description}
			switch {
			case positional.Argv:
				input.Kind = InputArgv
				schema.Type, schema.Items, schema.MinItems, schema.MaxItems = "array", &Schema{Type: "string", MinLength: 1}, 1, policy.Limits.MaxArguments
			case len(positional.Enum) > 0:
				input.Kind, input.Items = InputEnum, positional.Enum
				schema.Type, schema.Enum = "string", positional.Enum
			default:
				schema.Type, schema.MinLength = "string", 1
				if positional.Pattern != "" {
					compiled, err := regexp.Compile(positional.Pattern)
					if err != nil {
						return nil, fmt.Errorf("mcp: tool %q input %q: %w", name, positional.Name, err)
					}
					input.Pattern, schema.Pattern = compiled, positional.Pattern
				}
			}
			if err := add(input, schema); err != nil {
				return nil, err
			}
		}
		bound := map[string]json.RawMessage{}
		for flag, value := range spec.Bind {
			if !contains(flags, flag) {
				return nil, fmt.Errorf("mcp: tool %q binds %s, which it does not accept", name, flag)
			}
			bound[flag] = value
		}
		flagNames := make([]string, 0, len(flags))
		flagNames = append(flagNames, flags...)
		sort.Strings(flagNames)
		for _, flag := range flagNames {
			if value, isBound := bound[flag]; isBound {
				tool.Bound = append(tool.Bound, Bound{Flag: flag, Value: value})
				continue
			}
			if omitted[flag] {
				continue
			}
			row, ok := catalogue.flag(flag, spec.Operation)
			if !ok {
				return nil, fmt.Errorf("mcp: tool %q accepts %s, which the catalogue does not define", name, flag)
			}
			input, schema, err := flagInput(row, policy.FlagTypes[flag], contains(spec.Required, flag), spec.Defaults[flag], policy)
			if err != nil {
				return nil, fmt.Errorf("mcp: tool %q: %w", name, err)
			}
			if err := add(input, schema); err != nil {
				return nil, err
			}
		}
		for _, flag := range spec.Required {
			if !contains(flags, flag) {
				return nil, fmt.Errorf("mcp: tool %q requires %s, which it does not accept", name, flag)
			}
		}
		for _, group := range spec.AnyOf {
			names := make([]string, 0, len(group))
			for _, flag := range group {
				if !contains(flags, flag) {
					return nil, fmt.Errorf("mcp: tool %q anyOf names %s, which it does not accept", name, flag)
				}
				names = append(names, flagProperty(flag))
			}
			tool.AnyOf = append(tool.AnyOf, names)
		}
		for _, variable := range spec.Environment {
			row, ok := policy.Environment[variable]
			if !ok {
				return nil, fmt.Errorf("mcp: tool %q names unknown environment input %q", name, variable)
			}
			if err := add(Input{Name: variable, Kind: InputEnum, Items: row.Enum, Variable: row.Variable, Description: row.Description.Bilingual()}, Schema{Type: "string", Enum: row.Enum, Description: row.Description.Bilingual()}); err != nil {
				return nil, err
			}
		}
		for _, global := range policy.Globals {
			row, ok := catalogue.flag(global.Flag, "")
			if !ok {
				return nil, fmt.Errorf("mcp: policy global %s is not in the catalogue", global.Flag)
			}
			items := enumOf(row.ValueName)
			if len(items) == 0 {
				return nil, fmt.Errorf("mcp: global %s has no closed value set", global.Flag)
			}
			description := global.Description.Bilingual()
			if global.Description.English() == "" || global.Description.German() == "" || !strings.Contains(row.Description.English(), global.Environment) {
				return nil, fmt.Errorf("mcp: global %s needs a description in both languages and the catalogue must name %s", global.Flag, global.Environment)
			}
			if err := add(Input{Name: global.Input, Flag: global.Flag, Kind: InputEnum, Items: items, Variable: global.Environment, Description: description}, Schema{Type: "string", Enum: items, Description: description}); err != nil {
				return nil, err
			}
		}
		if err := add(Input{Name: policy.Identity.Argument, Kind: InputString, Agent: true, Pattern: identity, Description: policy.Identity.Description.Bilingual()}, Schema{Type: "string", Pattern: policy.Identity.Pattern, MinLength: 1, Description: policy.Identity.Description.Bilingual()}); err != nil {
			return nil, err
		}
		sort.Strings(required)
		additional := false
		schema := Schema{Type: "object", Properties: properties, Required: required, AdditionalProperties: &additional}
		for _, group := range tool.AnyOf {
			sorted := append([]string(nil), group...)
			sort.Strings(sorted)
			schema.AnyOf = append(schema.AnyOf, Schema{Required: sorted})
		}
		annotations := policyAnnotations(kind.Annotations, spec.Annotations)
		annotations.Title = tool.Title
		tool.Schema = Tool{Name: name, Title: tool.Title, Description: describe(summary, synopsis, operation, kind, spec), InputSchema: schema, OutputSchema: BuildResultOutputSchema(), Annotations: &annotations, Execution: &ToolExecution{TaskSupport: kind.TaskSupport}}
		tools = append(tools, tool)
	}
	if err := checkCoverage(catalogue, policy, tools); err != nil {
		return nil, err
	}
	sort.Slice(tools, func(left, right int) bool { return tools[left].Name < tools[right].Name })
	return tools, nil
}

func contains(values []string, wanted string) bool {
	for _, value := range values {
		if value == wanted {
			return true
		}
	}
	return false
}

func policyAnnotations(kind ToolAnnotationsPolicy, override *ToolAnnotationsPolicy) ToolAnnotations {
	merged := kind
	if override != nil {
		if override.ReadOnlyHint != nil {
			merged.ReadOnlyHint = override.ReadOnlyHint
		}
		if override.DestructiveHint != nil {
			merged.DestructiveHint = override.DestructiveHint
		}
		if override.IdempotentHint != nil {
			merged.IdempotentHint = override.IdempotentHint
		}
		if override.OpenWorldHint != nil {
			merged.OpenWorldHint = override.OpenWorldHint
		}
	}
	return ToolAnnotations{ReadOnlyHint: merged.ReadOnlyHint, DestructiveHint: merged.DestructiveHint, IdempotentHint: merged.IdempotentHint, OpenWorldHint: merged.OpenWorldHint}
}

// 🚩️flagInput derives the input of one catalogue flag.
func flagInput(row CatalogueFlag, override PolicyFlagType, required bool, fallback json.RawMessage, policy Policy) (Input, Schema, error) {
	description := row.Description.Bilingual()
	if len(fallback) > 0 {
		var shown string
		if err := json.Unmarshal(fallback, &shown); err != nil {
			shown = string(fallback)
		}
		description += "\nDefault of this tool / Standard dieses Werkzeugs: " + shown
	}
	input := Input{Name: flagProperty(row.Name), Flag: row.Name, Required: required, Default: fallback, Description: description, Separate: row.Value == "separate"}
	schema := Schema{Description: description, Default: fallback}
	switch {
	case row.Value == "none":
		input.Kind, schema.Type = InputBoolean, "boolean"
	case override.Type == "integer":
		input.Kind, input.Minimum, input.Maximum = InputInteger, override.Minimum, override.Maximum
		schema.Type, schema.Minimum, schema.Maximum = "integer", override.Minimum, override.Maximum
	case override.Type == "list":
		items := override.Items
		if override.Closed {
			items = enumOf(row.ValueName)
			if len(items) == 0 {
				return Input{}, Schema{}, fmt.Errorf("flag %s is a closed list but the catalogue names no value set", row.Name)
			}
		}
		input.Kind, input.Items = InputList, items
		schema.Type, schema.MinItems, schema.UniqueItems = "array", 1, true
		schema.Items = &Schema{Type: "string", MinLength: 1}
		if len(items) > 0 {
			schema.Items.Enum = items
		}
	case row.Value == "optional":
		return Input{}, Schema{}, fmt.Errorf("flag %s takes an optional value; omit it in the policy", row.Name)
	default:
		if items := enumOf(row.ValueName); len(items) > 0 {
			input.Kind, input.Items = InputEnum, items
			schema.Type, schema.Enum = "string", items
		} else {
			input.Kind = InputString
			schema.Type, schema.MinLength = "string", 1
		}
	}
	return input, schema, nil
}

// 📝️describe renders the bilingual description of a tool from the catalogue row and the kind.
func describe(summary Text, synopsis string, operation CatalogueOperation, kind PolicyKind, spec PolicyTool) string {
	var out strings.Builder
	out.WriteString(summary.Bilingual())
	out.WriteString("\n\nCLI: repo " + synopsis)
	if len(operation.Implementations) > 0 {
		names := make([]string, 0, len(operation.Implementations))
		for name := range operation.Implementations {
			names = append(names, name)
		}
		sort.Strings(names)
		parts := make([]string, len(names))
		for index, name := range names {
			parts[index] = name + "=" + operation.Implementations[name]
		}
		out.WriteString("\nImplementations: " + strings.Join(parts, ", "))
	}
	out.WriteString("\n" + kind.Note.Bilingual())
	return out.String()
}

// 🧱️BuildResultOutputSchema is the one `outputSchema` of every build tool (see `$defs.Result` of the module schema).
func BuildResultOutputSchema() *Schema {
	additional := true
	text := Schema{Type: "string"}
	flag := Schema{Type: "boolean"}
	whole := Schema{Type: "integer"}
	return &Schema{Type: "object", Description: "Structured result of one build operation.", AdditionalProperties: &additional, Required: []string{"version", "tool", "operation", "ok", "exitCode", "cancelled", "durationMs", "command", "agent", "stdout", "stderr", "truncated", "progressEvents"}, Properties: map[string]Schema{
		"version":        {Type: "integer"},
		"tool":           text,
		"operation":      text,
		"subcommand":     text,
		"buildTool":      {Type: "string", MinLength: 1},
		"ok":             flag,
		"exitCode":       whole,
		"cancelled":      flag,
		"durationMs":     whole,
		"command":        {Type: "array", Items: &Schema{Type: "string"}},
		"agent":          {Type: "object", Properties: map[string]Schema{"id": text, "session": text, "lane": text}, Required: []string{"id", "session", "lane"}, AdditionalProperties: &additional},
		"data":           {Description: "The JSON the operation printed."},
		"dataOmitted":    whole,
		"stdout":         text,
		"stderr":         text,
		"truncated":      {Type: "object", Properties: map[string]Schema{"stdout": flag, "stderr": flag}, Required: []string{"stdout", "stderr"}, AdditionalProperties: &additional},
		"progressEvents": whole,
		"hint":           text,
	}}
}

// 🔎️checkCoverage requires every catalogue row to be exposed or excluded with a reason, and every exclusion to name a real row, so a new operation can never be forgotten silently.
func checkCoverage(catalogue Catalogue, policy Policy, tools []BuildTool) error {
	type row struct{ operation, subcommand string }
	accounted := map[row]string{}
	for _, tool := range tools {
		accounted[row{tool.Operation, tool.Subcommand}] = "exposed"
	}
	for _, exclusion := range policy.Excluded {
		key := row{exclusion.Operation, exclusion.Subcommand}
		if accounted[key] != "" {
			return fmt.Errorf("mcp: %s %s is both exposed and excluded", key.operation, key.subcommand)
		}
		if exclusion.Reason.English() == "" || exclusion.Reason.German() == "" {
			return fmt.Errorf("mcp: exclusion of %s %s has no reason in both languages", key.operation, key.subcommand)
		}
		accounted[key] = "excluded"
	}
	var missing []string
	for _, operation := range catalogue.Operations {
		if len(operation.Subcommands) == 0 {
			if accounted[row{operation.Name, ""}] == "" {
				missing = append(missing, operation.Name)
			}
			continue
		}
		for _, subcommand := range operation.Subcommands {
			if accounted[row{operation.Name, subcommand.Name}] == "" && accounted[row{operation.Name, ""}] == "" {
				missing = append(missing, operation.Name+" "+subcommand.Name)
			}
		}
	}
	for key := range accounted {
		operation, found := catalogue.operation(key.operation)
		if !found {
			return fmt.Errorf("mcp: policy names unknown operation %q", key.operation)
		}
		if key.subcommand != "" {
			if _, err := catalogue.effectiveFlags(operation, key.subcommand); err != nil {
				return err
			}
		}
	}
	if len(missing) > 0 {
		sort.Strings(missing)
		return fmt.Errorf("mcp: catalogue rows neither exposed nor excluded: %s", strings.Join(missing, ", "))
	}
	return nil
}

// #endregion 🛠️Derivation

// #region 🧷️Arguments

// ❌️ArgumentError describes why the arguments of a call are refused; it is shown to the agent as the tool's answer so it can correct the call.
type ArgumentError struct{ Message string }

func (err *ArgumentError) Error() string { return err.Message }

func refuse(format string, arguments ...any) error {
	return &ArgumentError{Message: fmt.Sprintf(format, arguments...)}
}

// 🧷️Invocation is the validated call: the argument vector after `repo` and the variables to set.
type Invocation struct {
	Argv        []string
	Environment map[string]string
	BuildTool   string
	Agent       string
	Values      map[string]json.RawMessage
}

// 🧷️Plan validates the arguments of a call against the derived inputs and returns the exact argument vector.
func (tool BuildTool) Plan(raw json.RawMessage) (Invocation, error) {
	values := map[string]json.RawMessage{}
	trimmed := bytes.TrimSpace(raw)
	if len(trimmed) == 0 {
		trimmed = []byte("{}")
	}
	if trimmed[0] != '{' {
		return Invocation{}, refuse("arguments must be a JSON object")
	}
	if err := json.Unmarshal(trimmed, &values); err != nil {
		return Invocation{}, refuse("arguments are not valid JSON: %v", err)
	}
	known := map[string]Input{}
	for _, input := range tool.Inputs {
		known[input.Name] = input
	}
	var unknown []string
	for name := range values {
		if _, ok := known[name]; !ok {
			unknown = append(unknown, name)
		}
	}
	if len(unknown) > 0 {
		sort.Strings(unknown)
		names := make([]string, 0, len(known))
		for name := range known {
			names = append(names, name)
		}
		sort.Strings(names)
		return Invocation{}, refuse("unknown arguments: %s (accepted: %s)", strings.Join(unknown, ", "), strings.Join(names, ", "))
	}
	invocation := Invocation{Environment: map[string]string{}, Values: values}
	positional, flagged, trailing := []string{}, []string{}, []string{}
	for _, input := range tool.Inputs {
		value, present := values[input.Name]
		if present && bytes.Equal(bytes.TrimSpace(value), []byte("null")) {
			present = false
		}
		if !present && len(input.Default) > 0 && input.Flag != "" && input.Variable == "" {
			value, present = input.Default, true
		}
		if !present {
			if input.Required {
				return Invocation{}, refuse("%s is required", input.Name)
			}
			continue
		}
		switch {
		case input.Positional && input.Kind == InputArgv:
			var words []string
			if err := json.Unmarshal(value, &words); err != nil || len(words) == 0 || len(words) > tool.Limits.MaxArguments {
				return Invocation{}, refuse("%s must be an array of 1 to %d strings", input.Name, tool.Limits.MaxArguments)
			}
			for _, word := range words {
				if err := checkWord(word, tool.Limits.MaxArgumentBytes); err != nil || word == "" {
					return Invocation{}, refuse("%s holds an empty or unsafe argument", input.Name)
				}
			}
			trailing = append(trailing, "--")
			trailing = append(trailing, words...)
		case input.Positional:
			text, err := decodeText(value, input, tool.Limits.MaxArgumentBytes)
			if err != nil {
				return Invocation{}, err
			}
			positional = append(positional, text+input.Suffix)
		case input.Variable != "":
			text, err := decodeText(value, input, tool.Limits.MaxArgumentBytes)
			if err != nil {
				return Invocation{}, err
			}
			invocation.Environment[input.Variable] = text
			if input.Flag == "--tool" {
				invocation.BuildTool = text
			}
		case input.Agent:
			text, err := decodeText(value, input, tool.Limits.MaxArgumentBytes)
			if err != nil {
				return Invocation{}, err
			}
			invocation.Agent = text
		default:
			words, err := flagWords(input, value, tool.Limits.MaxArgumentBytes)
			if err != nil {
				return Invocation{}, err
			}
			flagged = append(flagged, words...)
		}
	}
	if len(tool.AnyOf) > 0 {
		satisfied := false
		for _, group := range tool.AnyOf {
			complete := true
			for _, name := range group {
				if value, present := values[name]; !present || bytes.Equal(bytes.TrimSpace(value), []byte("null")) {
					complete = false
				}
			}
			satisfied = satisfied || complete
		}
		if !satisfied {
			options := make([]string, len(tool.AnyOf))
			for index, group := range tool.AnyOf {
				options[index] = strings.Join(group, "+")
			}
			return Invocation{}, refuse("one of %s is required", strings.Join(options, ", "))
		}
	}
	argv := []string{tool.Operation}
	if tool.Subcommand != "" {
		argv = append(argv, tool.Subcommand)
	}
	argv = append(argv, positional...)
	for _, bound := range tool.Bound {
		if bytes.Equal(bytes.TrimSpace(bound.Value), []byte("true")) {
			argv = append(argv, bound.Flag)
		} else if text, err := strconv.Unquote(string(bound.Value)); err == nil {
			argv = append(argv, bound.Flag+"="+text)
		}
	}
	argv = append(argv, flagged...)
	argv = append(argv, trailing...)
	invocation.Argv = argv
	return invocation, nil
}

func checkWord(word string, limit int) error {
	if len(word) > limit || strings.ContainsAny(word, "\x00\r\n") {
		return errors.New("unsafe argument")
	}
	return nil
}

func decodeText(value json.RawMessage, input Input, limit int) (string, error) {
	var text string
	if err := json.Unmarshal(value, &text); err != nil {
		return "", refuse("%s must be a string", input.Name)
	}
	if text == "" || checkWord(text, limit) != nil {
		return "", refuse("%s must be a non-empty string without control characters, at most %d bytes", input.Name, limit)
	}
	if input.Kind == InputEnum && !contains(input.Items, text) {
		return "", refuse("%s must be one of %s", input.Name, strings.Join(input.Items, ", "))
	}
	if input.Pattern != nil && !input.Pattern.MatchString(text) {
		return "", refuse("%s does not match %s", input.Name, input.Pattern.String())
	}
	if input.Positional && input.Pattern == nil && strings.HasPrefix(text, "-") {
		return "", refuse("%s must not start with -", input.Name)
	}
	return text, nil
}

func flagWords(input Input, value json.RawMessage, limit int) ([]string, error) {
	switch input.Kind {
	case InputBoolean:
		var enabled bool
		if err := json.Unmarshal(value, &enabled); err != nil {
			return nil, refuse("%s must be true or false", input.Name)
		}
		if enabled {
			return []string{input.Flag}, nil
		}
		return nil, nil
	case InputInteger:
		var number int64
		decoder := json.NewDecoder(bytes.NewReader(value))
		decoder.UseNumber()
		var raw json.Number
		if err := decoder.Decode(&raw); err != nil {
			return nil, refuse("%s must be an integer", input.Name)
		}
		parsed, err := strconv.ParseInt(raw.String(), 10, 64)
		if err != nil {
			return nil, refuse("%s must be an integer", input.Name)
		}
		number = parsed
		if (input.Minimum != nil && number < *input.Minimum) || (input.Maximum != nil && number > *input.Maximum) {
			return nil, refuse("%s is outside its range", input.Name)
		}
		return []string{input.Flag + "=" + strconv.FormatInt(number, 10)}, nil
	case InputList:
		var items []string
		if err := json.Unmarshal(value, &items); err != nil || len(items) == 0 {
			return nil, refuse("%s must be a non-empty array of strings", input.Name)
		}
		seen := map[string]bool{}
		for _, item := range items {
			if item == "" || strings.Contains(item, ",") || checkWord(item, limit) != nil || seen[item] || (len(input.Items) > 0 && !contains(input.Items, item)) {
				return nil, refuse("%s holds an empty, repeated, unsupported or unsafe item", input.Name)
			}
			seen[item] = true
		}
		return []string{input.Flag + "=" + strings.Join(items, ",")}, nil
	default:
		text, err := decodeText(value, input, limit)
		if err != nil {
			return nil, err
		}
		if input.Separate {
			if strings.HasPrefix(text, "-") {
				return nil, refuse("%s must not start with -", input.Name)
			}
			return []string{input.Flag, text}, nil
		}
		return []string{input.Flag + "=" + text}, nil
	}
}

// #endregion 🧷️Arguments
