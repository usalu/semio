// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Package graphql owns the repository's hand-rolled GraphQL document grammar, its schema
// vocabulary and the deterministic executor the repo CLI answers `repo://` queries with.

// #endregion 🧲️Header

package graphql

import (
	context "context"
	json "encoding/json"
	errors "errors"
	fmt "fmt"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	slices "slices"
	sort "sort"
	strconv "strconv"
	strings "strings"
	unicode "unicode"

	codebase "github.com/usalu/semio/repo/codebase"
	contributorspkg "github.com/usalu/semio/repo/contributors"
	events "github.com/usalu/semio/repo/events"
	goalspkg "github.com/usalu/semio/repo/goals"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	move "github.com/usalu/semio/repo/move"
	providers "github.com/usalu/semio/repo/providers"
	statutespkg "github.com/usalu/semio/repo/statutes"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	todospkg "github.com/usalu/semio/repo/todos"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 📜️Schema

/** 🗺️ How a type reference wraps the type it names. */
type TypeRefKind int

const (
	// 🏷️ A named type, resolved against the schema's type table.
	TypeRefNamed TypeRefKind = iota
	// ❗️ A wrapper asserting the value is never null.
	TypeRefNonNull
	// 📃️ A wrapper for a homogeneous sequence.
	TypeRefList
)

/** 🗺️ A type reference in a field, argument or input-field position. The name is resolved against
 * [Schema.Types] instead of a pointer, so the graph may be cyclic without a thunk. */
type TypeRef struct {
	Kind TypeRefKind
	Name string
	Of   *TypeRef
}

/** 🏷️ A named type reference. */
func Ty(name string) TypeRef { return TypeRef{Kind: TypeRefNamed, Name: name} }

/** ❗️ Wraps a reference as non-null. */
func NN(inner TypeRef) TypeRef { return TypeRef{Kind: TypeRefNonNull, Of: &inner} }

/** 📃️ Wraps a reference as a list. */
func Lst(inner TypeRef) TypeRef { return TypeRef{Kind: TypeRefList, Of: &inner} }

/** 📃️ The `[T!]!` shape every collection field of this schema uses. */
func NNList(inner TypeRef) TypeRef { return NN(Lst(NN(inner))) }

/** 🏷️ The innermost named type, ignoring every wrapper. */
func (reference TypeRef) TypeName() string {
	if reference.Kind == TypeRefNamed {
		return reference.Name
	}
	return reference.Of.TypeName()
}

/** 📜️ The SDL rendering of this reference. */
func (reference TypeRef) Render() string {
	switch reference.Kind {
	case TypeRefNonNull:
		return reference.Of.Render() + "!"
	case TypeRefList:
		return "[" + reference.Of.Render() + "]"
	default:
		return reference.Name
	}
}

/** 🎚️ One declared field argument and its default. */
type Argument struct {
	Name    string
	Type    TypeRef
	Default interface{}
}

/** 🧩️ One selectable field of an object or interface type. */
type Field struct {
	Name     string
	Type     TypeRef
	Args     []Argument
	Resolver string
}

/** 🧩️ A field resolved by reading the source object. */
func NewField(name string, kind TypeRef) Field { return Field{Name: name, Type: kind} }

/** 🔑️ A field resolved by the named resolver. */
func ResolvedField(name string, kind TypeRef, resolver string) Field {
	return Field{Name: name, Type: kind, Resolver: resolver}
}

/** 🎚️ Declares one argument without a default. */
func (field Field) Arg(name string, kind TypeRef) Field {
	field.Args = append(append([]Argument{}, field.Args...), Argument{Name: name, Type: kind})
	return field
}

/** 📜️ Anything nameable in the owned schema vocabulary. */
type NamedType interface{ TypeName() string }

/** 🔤️ A leaf type serialised straight into the result. */
type ScalarType struct{ Name string }

func (declared *ScalarType) TypeName() string { return declared.Name }

/** 🏛️ A composite type with a field table. */
type ObjectType struct {
	Name       string
	Interfaces []string
	Fields     []Field
}

func (declared *ObjectType) TypeName() string { return declared.Name }

/** 🧩️ Looks one field up by name. */
func (declared *ObjectType) Field(name string) *Field {
	for index := range declared.Fields {
		if declared.Fields[index].Name == name {
			return &declared.Fields[index]
		}
	}
	return nil
}

/** 🎭️ An abstract type resolved to a concrete object at execution time. */
type InterfaceType struct {
	Name   string
	Fields []Field
}

func (declared *InterfaceType) TypeName() string { return declared.Name }

/** 🎭️ A closed set of object types resolved at execution time. */
type UnionType struct {
	Name  string
	Types []string
}

func (declared *UnionType) TypeName() string { return declared.Name }

/** 🔢️ One member of an enum type and the wire value it stands for. */
type EnumValue struct {
	Name string
	Wire string
}

/** 🔢️ A closed set of named constants. */
type EnumType struct {
	Name   string
	Values []EnumValue
}

func (declared *EnumType) TypeName() string { return declared.Name }

/** 📥️ One field of an input object type. */
type InputField struct {
	Name string
	Type TypeRef
}

/** 📥️ A composite type accepted as an argument. */
type InputObjectType struct {
	Name   string
	Fields []InputField
}

func (declared *InputObjectType) TypeName() string { return declared.Name }

/** 🗺️ The executable schema: a type table plus the operation roots. */
type Schema struct {
	Types    map[string]NamedType
	Query    string
	Mutation string
}

/** 🏛️ Looks one object type up by name. */
func (schema Schema) Object(name string) *ObjectType {
	if object, ok := schema.Types[name].(*ObjectType); ok {
		return object
	}
	return nil
}

/** 🔤️ The five scalar names the repo schema is built from. */
var Scalars = []string{"String", "Int", "Boolean", "ID", "DateTime"}

// #endregion 📜️Schema

// #region 🔤️Lexer

/** 🔤️ The five token classes the grammar distinguishes. */
type tokenKind int

const (
	tokenEOF tokenKind = iota
	tokenName
	tokenString
	tokenNumber
	tokenPunct
)

/** 🔤️ One lexed token and the byte offset it starts at. */
type token struct {
	kind   tokenKind
	text   string
	offset int
}

/** 🔤️ A byte-oriented scanner over one document source. */
type lexer struct {
	source string
	offset int
}

/** 🔤️ Reads the next token, skipping insignificant bytes, commas and comments. */
func (lexer *lexer) next() (token, error) {
	for lexer.offset < len(lexer.source) {
		r := rune(lexer.source[lexer.offset])
		if unicode.IsSpace(r) || r == ',' {
			lexer.offset++
			continue
		}
		if r == '#' {
			for lexer.offset < len(lexer.source) && lexer.source[lexer.offset] != '\n' {
				lexer.offset++
			}
			continue
		}
		break
	}
	if lexer.offset >= len(lexer.source) {
		return token{kind: tokenEOF, offset: lexer.offset}, nil
	}
	start := lexer.offset
	ch := lexer.source[lexer.offset]
	if strings.ContainsRune("{}()[]:$!@=", rune(ch)) {
		lexer.offset++
		return token{kind: tokenPunct, text: string(ch), offset: start}, nil
	}
	if ch == '"' {
		lexer.offset++
		for lexer.offset < len(lexer.source) {
			if lexer.source[lexer.offset] == '\\' {
				lexer.offset += 2
				continue
			}
			if lexer.source[lexer.offset] == '"' {
				lexer.offset++
				raw := lexer.source[start:lexer.offset]
				value, err := strconv.Unquote(raw)
				return token{kind: tokenString, text: value, offset: start}, err
			}
			lexer.offset++
		}
		return token{}, fmt.Errorf("unterminated string at %d", start)
	}
	if ch == '-' || ch >= '0' && ch <= '9' {
		lexer.offset++
		for lexer.offset < len(lexer.source) && strings.ContainsRune("0123456789.eE+-", rune(lexer.source[lexer.offset])) {
			lexer.offset++
		}
		return token{kind: tokenNumber, text: lexer.source[start:lexer.offset], offset: start}, nil
	}
	if ch == '_' || unicode.IsLetter(rune(ch)) {
		lexer.offset++
		for lexer.offset < len(lexer.source) {
			r := rune(lexer.source[lexer.offset])
			if r != '_' && !unicode.IsLetter(r) && !unicode.IsDigit(r) {
				break
			}
			lexer.offset++
		}
		return token{kind: tokenName, text: lexer.source[start:lexer.offset], offset: start}, nil
	}
	return token{}, fmt.Errorf("unexpected character %q at %d", ch, start)
}

// #endregion 🔤️Lexer

// #region 🌳️Ast

/** 🌳️ One parsed operation: its kind and its root selection set. */
type Document struct {
	Operation  string
	Selections []Selection
}

/** 🌳️ One selected field, its alias, its arguments and its sub-selection. */
type Selection struct {
	Alias     string
	Name      string
	Arguments map[string]Value
	Fields    []Selection
}

/** 🌳️ One argument value: a variable reference, a list, an input object or a literal. */
type Value struct {
	Literal  interface{}
	Variable string
	List     []Value
	Object   map[string]Value
}

/** 🔢️ Substitutes variables into a parsed value, recursing through lists and input objects. */
func (value Value) Resolve(variables map[string]interface{}) interface{} {
	if value.Variable != "" {
		return variables[value.Variable]
	}
	if value.List != nil {
		result := make([]interface{}, len(value.List))
		for index := range value.List {
			result[index] = value.List[index].Resolve(variables)
		}
		return result
	}
	if value.Object != nil {
		result := map[string]interface{}{}
		for key, item := range value.Object {
			result[key] = item.Resolve(variables)
		}
		return result
	}
	return value.Literal
}

/** 🔢️ Coerces one selection's arguments, filling declared defaults only for absent argument names. */
func CoerceArguments(arguments map[string]Value, defaults map[string]interface{}, variables map[string]interface{}) map[string]interface{} {
	args := map[string]interface{}{}
	for name, raw := range arguments {
		args[name] = raw.Resolve(variables)
	}
	for name, fallback := range defaults {
		if _, ok := args[name]; !ok && fallback != nil {
			args[name] = fallback
		}
	}
	return args
}

/** 🖼️ Canonical, language-neutral projection of one value, as `🧬️schema/🔣️.json` defines it. */
func (value Value) Projection() map[string]interface{} {
	if value.Variable != "" {
		return map[string]interface{}{"kind": "variable", "name": value.Variable}
	}
	if value.List != nil {
		items := make([]interface{}, len(value.List))
		for index := range value.List {
			items[index] = value.List[index].Projection()
		}
		return map[string]interface{}{"kind": "list", "items": items}
	}
	if value.Object != nil {
		names := make([]string, 0, len(value.Object))
		for name := range value.Object {
			names = append(names, name)
		}
		sort.Strings(names)
		fields := make([]interface{}, 0, len(names))
		for _, name := range names {
			fields = append(fields, map[string]interface{}{"name": name, "value": value.Object[name].Projection()})
		}
		return map[string]interface{}{"kind": "object", "fields": fields}
	}
	if value.Literal == nil {
		return map[string]interface{}{"kind": "null"}
	}
	return map[string]interface{}{"kind": "literal", "value": value.Literal}
}

/** 🖼️ Canonical projection of one selection, with arguments sorted by name. */
func (selection Selection) Projection() map[string]interface{} {
	names := make([]string, 0, len(selection.Arguments))
	for name := range selection.Arguments {
		names = append(names, name)
	}
	sort.Strings(names)
	arguments := make([]interface{}, 0, len(names))
	for _, name := range names {
		arguments = append(arguments, map[string]interface{}{"name": name, "value": selection.Arguments[name].Projection()})
	}
	fields := make([]interface{}, 0, len(selection.Fields))
	for _, field := range selection.Fields {
		fields = append(fields, field.Projection())
	}
	var alias interface{}
	if selection.Alias != "" {
		alias = selection.Alias
	}
	return map[string]interface{}{"name": selection.Name, "alias": alias, "arguments": arguments, "fields": fields}
}

/** 🖼️ Canonical projection of one document — the shape every implementation and the oracle emit. */
func (document Document) Projection() map[string]interface{} {
	selections := make([]interface{}, 0, len(document.Selections))
	for _, selection := range document.Selections {
		selections = append(selections, selection.Projection())
	}
	return map[string]interface{}{"operation": document.Operation, "selections": selections}
}

/** 🔑️ The result key one selection writes under — its alias when it has one. */
func (selection Selection) Key() string {
	if selection.Alias != "" {
		return selection.Alias
	}
	return selection.Name
}

// #endregion 🌳️Ast

// #region 🧩️Parser

/** 🧩️ A single-token-lookahead recursive descent parser over the lexer. */
type parser struct {
	lexer   lexer
	current token
	err     error
}

/** 🧩️ Starts a parser primed with the first token. */
func newParser(source string) *parser {
	parser := &parser{lexer: lexer{source: source}}
	parser.advance()
	return parser
}

/** 🧩️ Reads the next token unless the parser already failed. */
func (parser *parser) advance() {
	if parser.err != nil {
		return
	}
	parser.current, parser.err = parser.lexer.next()
}

/** 🧩️ Consumes an exact punctuation or keyword, reporting the offset when it is missing. */
func (parser *parser) take(text string) error {
	if parser.err != nil {
		return parser.err
	}
	if parser.current.text != text {
		return fmt.Errorf("expected %q at %d, got %q", text, parser.current.offset, parser.current.text)
	}
	parser.advance()
	return nil
}

/** 🧩️ Skips a balanced parenthesised group — variable definitions and directive arguments. */
func (parser *parser) skipParenthesized() {
	depth := 0
	for parser.current.kind != tokenEOF {
		if parser.current.text == "(" {
			depth++
		}
		if parser.current.text == ")" {
			depth--
			parser.advance()
			if depth == 0 {
				break
			}
			continue
		}
		parser.advance()
	}
}

/** 🧩️ Parses one document: an optional operation header followed by the root selection set. */
func Parse(source string) (Document, error) {
	parser := newParser(source)
	doc := Document{Operation: "query"}
	if parser.current.kind == tokenName && (parser.current.text == "query" || parser.current.text == "mutation") {
		doc.Operation = parser.current.text
		parser.advance()
		if parser.current.kind == tokenName {
			parser.advance()
		}
		if parser.current.text == "(" {
			parser.skipParenthesized()
		}
	}
	selections, err := parser.selectionSet()
	if err != nil {
		return Document{}, err
	}
	doc.Selections = selections
	if parser.err != nil {
		return Document{}, parser.err
	}
	if parser.current.kind != tokenEOF {
		return Document{}, fmt.Errorf("unexpected token %q at %d", parser.current.text, parser.current.offset)
	}
	return doc, nil
}

/** 🧩️ Parses a brace-delimited selection set, including aliases, arguments and dropped directives. */
func (parser *parser) selectionSet() ([]Selection, error) {
	if err := parser.take("{"); err != nil {
		return nil, err
	}
	var selections []Selection
	for parser.current.text != "}" {
		if parser.current.kind == tokenEOF {
			return nil, fmt.Errorf("unterminated selection set")
		}
		if parser.current.kind != tokenName {
			return nil, fmt.Errorf("expected field name at %d", parser.current.offset)
		}
		item := Selection{Name: parser.current.text, Arguments: map[string]Value{}}
		parser.advance()
		if parser.current.text == ":" {
			parser.advance()
			item.Alias = item.Name
			if parser.current.kind != tokenName {
				return nil, fmt.Errorf("expected aliased field name")
			}
			item.Name = parser.current.text
			parser.advance()
		}
		if parser.current.text == "(" {
			parser.advance()
			for parser.current.text != ")" {
				if parser.current.kind != tokenName {
					return nil, fmt.Errorf("expected argument name at %d", parser.current.offset)
				}
				name := parser.current.text
				parser.advance()
				if err := parser.take(":"); err != nil {
					return nil, err
				}
				argument, err := parser.value()
				if err != nil {
					return nil, err
				}
				item.Arguments[name] = argument
			}
			parser.advance()
		}
		for parser.current.text == "@" {
			parser.advance()
			if parser.current.kind == tokenName {
				parser.advance()
			}
			if parser.current.text == "(" {
				parser.skipParenthesized()
			}
		}
		if parser.current.text == "{" {
			fields, err := parser.selectionSet()
			if err != nil {
				return nil, err
			}
			item.Fields = fields
		}
		selections = append(selections, item)
	}
	parser.advance()
	return selections, nil
}

/** 🧩️ Parses one argument value: variable, list, input object, string, number, boolean, null or enum. */
func (parser *parser) value() (Value, error) {
	current := parser.current
	switch {
	case current.text == "$":
		parser.advance()
		if parser.current.kind != tokenName {
			return Value{}, fmt.Errorf("expected variable name")
		}
		result := Value{Variable: parser.current.text}
		parser.advance()
		return result, nil
	case current.text == "[":
		parser.advance()
		result := Value{}
		for parser.current.text != "]" {
			item, err := parser.value()
			if err != nil {
				return Value{}, err
			}
			result.List = append(result.List, item)
		}
		parser.advance()
		return result, nil
	case current.text == "{":
		parser.advance()
		result := Value{Object: map[string]Value{}}
		for parser.current.text != "}" {
			if parser.current.kind != tokenName {
				return Value{}, fmt.Errorf("expected object field")
			}
			name := parser.current.text
			parser.advance()
			if err := parser.take(":"); err != nil {
				return Value{}, err
			}
			item, err := parser.value()
			if err != nil {
				return Value{}, err
			}
			result.Object[name] = item
		}
		parser.advance()
		return result, nil
	case current.kind == tokenString:
		parser.advance()
		return Value{Literal: current.text}, nil
	case current.kind == tokenNumber:
		parser.advance()
		if strings.ContainsAny(current.text, ".eE") {
			parsed, err := strconv.ParseFloat(current.text, 64)
			return Value{Literal: parsed}, err
		}
		parsed, err := strconv.Atoi(current.text)
		return Value{Literal: parsed}, err
	case current.kind == tokenName:
		parser.advance()
		switch current.text {
		case "true":
			return Value{Literal: true}, nil
		case "false":
			return Value{Literal: false}, nil
		case "null":
			return Value{Literal: nil}, nil
		}
		return Value{Literal: current.text}, nil
	default:
		return Value{}, fmt.Errorf("invalid value at %d", current.offset)
	}
}

// #endregion 🧩️Parser

// #region ✅️Validation

/** ✅️ Reports whether a request string is a document this grammar accepts. */
func Validate(source string) error { _, err := Parse(source); return err }

/** ✅️ The operation kind of a request string, without executing it. */
func OperationType(source string) (string, error) {
	doc, err := Parse(source)
	return doc.Operation, err
}

// #endregion ✅️Validation

// #region ⚡️Execution

/** ❌️ One execution failure, carrying the reference implementation's error text. */
type ExecutionError struct{ Message string }

func (failure *ExecutionError) Error() string { return failure.Message }

/** ❌️ Builds an execution failure. */
func NewExecutionError(message string) *ExecutionError { return &ExecutionError{Message: message} }

/** 🔗️ Prefixes this failure with the selection it happened under, as `field: cause`. */
func (failure *ExecutionError) under(field string) *ExecutionError {
	return &ExecutionError{Message: field + ": " + failure.Message}
}

/** ❌️ Every failure a context operation can report. */
type ContextError struct{ Message string }

func (failure *ContextError) Error() string { return failure.Message }

/** ⚡️ One execution request. */
type Params struct {
	Context        context.Context
	Schema         Schema
	Repo           model.RepoContext
	RequestString  string
	VariableValues map[string]interface{}
}

/** ⚡️ The result of one execution. */
type Result struct {
	Data   interface{} `json:"data,omitempty"`
	Errors []error     `json:"errors,omitempty"`
}

/** ⚡️ Parses and executes one request against the schema. */
func Do(params Params) *Result {
	document, err := Parse(params.RequestString)
	if err != nil {
		return &Result{Errors: []error{err}}
	}
	rootName := params.Schema.Query
	if document.Operation == "mutation" {
		rootName = params.Schema.Mutation
	}
	root := params.Schema.Object(rootName)
	if root == nil {
		return &Result{Errors: []error{fmt.Errorf("%s root is not configured", document.Operation)}}
	}
	repo := params.Repo
	if repo == nil && RepoResolverInstance != nil {
		repo = RepoResolverInstance.context()
	}
	if repo == nil {
		return &Result{Errors: []error{fmt.Errorf("repository context is not configured")}}
	}
	ctx := params.Context
	if ctx == nil {
		ctx = context.Background()
	}
	execution := newExecution(ctx, params.Schema, repo, params.VariableValues)
	data, failure := execution.executeSelections(nil, root, document.Selections)
	if failure != nil {
		return &Result{Errors: []error{failure}}
	}
	return &Result{Data: data}
}

/** ⚡️ Case- and separator-insensitive field name the default resolver matches on. */
func canonicalName(value string) string {
	return strings.ToLower(strings.NewReplacer("_", "", "-", "").Replace(value))
}

/** ⚡️ Reads a field off a source object when no resolver was declared. */
func resolveDefault(source interface{}, name string) interface{} {
	fields, ok := source.(map[string]interface{})
	if !ok {
		return nil
	}
	if value, found := fields[name]; found {
		return value
	}
	canonical := canonicalName(name)
	keys := make([]string, 0, len(fields))
	for key := range fields {
		keys = append(keys, key)
	}
	sort.Strings(keys)
	for _, key := range keys {
		if canonicalName(key) == canonical {
			return fields[key]
		}
	}
	return nil
}

/** 🧹️ Whether a text survives one filter input. */
func matchesFilterInput(text string, filter *model.FilterInput) bool {
	if filter == nil || filter.Filter == nil || *filter.Filter == "" {
		return true
	}
	haystack, needle := text, *filter.Filter
	if filter.MatchCase == nil || !*filter.MatchCase {
		haystack = strings.ToLower(haystack)
		needle = strings.ToLower(needle)
	}
	if filter.MatchWholeWord != nil && *filter.MatchWholeWord {
		for _, word := range strings.FieldsFunc(haystack, func(character rune) bool {
			return !unicode.IsLetter(character) && !unicode.IsDigit(character)
		}) {
			if word == needle {
				return true
			}
		}
		return false
	}
	return strings.Contains(haystack, needle)
}

/** ⚡️ One execution of one document against one schema and one context. */
type execution struct {
	ctx          context.Context
	schema       Schema
	repo         model.RepoContext
	variables    map[string]interface{}
	rootDir      string
	statutes     []*model.StatuteMeta
	technologies []*model.Technology
}

/** ⚡️ Binds one execution to its schema and context. */
func newExecution(ctx context.Context, schema Schema, repo model.RepoContext, variables map[string]interface{}) *execution {
	if variables == nil {
		variables = map[string]interface{}{}
	}
	return &execution{
		ctx:          ctx,
		schema:       schema,
		repo:         repo,
		variables:    variables,
		rootDir:      repo.GetRootDir(),
		statutes:     repo.GetStatutes(),
		technologies: repo.GetTechnologies(),
	}
}

/** 📥️ Coerces one resolved argument value against its declared type. */
func (run *execution) coerceInput(value interface{}, declared TypeRef) interface{} {
	switch declared.Kind {
	case TypeRefNonNull:
		return run.coerceInput(value, *declared.Of)
	case TypeRefList:
		if value == nil {
			return nil
		}
		if items, ok := value.([]interface{}); ok {
			coerced := make([]interface{}, len(items))
			for index, item := range items {
				coerced[index] = run.coerceInput(item, *declared.Of)
			}
			return coerced
		}
		return []interface{}{run.coerceInput(value, *declared.Of)}
	default:
		switch named := run.schema.Types[declared.Name].(type) {
		case *EnumType:
			member, ok := value.(string)
			if !ok {
				return value
			}
			for _, declaredValue := range named.Values {
				if declaredValue.Name == member {
					return declaredValue.Wire
				}
			}
			return value
		case *InputObjectType:
			fields, ok := value.(map[string]interface{})
			if !ok {
				return value
			}
			coerced := map[string]interface{}{}
			for _, field := range named.Fields {
				if item, found := fields[field.Name]; found {
					coerced[field.Name] = run.coerceInput(item, field.Type)
				}
			}
			return coerced
		default:
			return value
		}
	}
}

/** ⚡️ Resolves every selection of one object, honouring aliases, defaults and `__typename`. */
func (run *execution) executeSelections(source interface{}, object *ObjectType, selections []Selection) (map[string]interface{}, *ExecutionError) {
	if err := run.ctx.Err(); err != nil {
		return nil, NewExecutionError(err.Error())
	}
	result := map[string]interface{}{}
	for _, selected := range selections {
		if selected.Name == "__typename" {
			result[selected.Key()] = object.Name
			continue
		}
		field := object.Field(selected.Name)
		if field == nil {
			return nil, NewExecutionError(fmt.Sprintf("unknown field %q on %s", selected.Name, object.Name))
		}
		defaults := map[string]interface{}{}
		for _, argument := range field.Args {
			defaults[argument.Name] = argument.Default
		}
		raw := CoerceArguments(selected.Arguments, defaults, run.variables)
		args := map[string]interface{}{}
		for name, value := range raw {
			args[name] = value
			for index := range field.Args {
				if field.Args[index].Name == name {
					args[name] = run.coerceInput(value, field.Args[index].Type)
					break
				}
			}
		}
		var value interface{}
		if field.Resolver != "" {
			resolved, failure := run.dispatch(field.Resolver, source, args)
			if failure != nil {
				return nil, failure.under(selected.Name)
			}
			value = resolved
		} else {
			value = resolveDefault(source, selected.Name)
		}
		projected, failure := run.project(value, field.Type, selected.Fields)
		if failure != nil {
			return nil, failure.under(selected.Name)
		}
		result[selected.Key()] = projected
	}
	return result, nil
}

/** ⚡️ Shapes one resolved value to its declared type and sub-selection. */
func (run *execution) project(value interface{}, declared TypeRef, selections []Selection) (interface{}, *ExecutionError) {
	if value == nil {
		return nil, nil
	}
	switch declared.Kind {
	case TypeRefNonNull:
		return run.project(value, *declared.Of, selections)
	case TypeRefList:
		items, ok := value.([]interface{})
		if !ok {
			return nil, NewExecutionError(fmt.Sprintf("expected list, got %s", renderJSON(value)))
		}
		projected := make([]interface{}, len(items))
		for index, item := range items {
			shaped, failure := run.project(item, *declared.Of, selections)
			if failure != nil {
				return nil, failure
			}
			projected[index] = shaped
		}
		return projected, nil
	default:
		switch named := run.schema.Types[declared.Name].(type) {
		case *ObjectType:
			if len(selections) == 0 {
				return value, nil
			}
			return run.executeSelectionsAsValue(value, named, selections)
		case *InterfaceType:
			object := run.concrete(value)
			if object == nil {
				return nil, NewExecutionError("cannot resolve interface " + named.Name)
			}
			return run.executeSelectionsAsValue(value, object, selections)
		case *UnionType:
			object := run.concrete(value)
			if object == nil {
				return nil, NewExecutionError("cannot resolve union " + named.Name)
			}
			return run.executeSelectionsAsValue(value, object, selections)
		case *EnumType:
			wire, ok := value.(string)
			if !ok {
				return value, nil
			}
			for _, member := range named.Values {
				if member.Wire == wire {
					return member.Name, nil
				}
			}
			return value, nil
		default:
			return value, nil
		}
	}
}

/** ⚡️ Executes a sub-selection and hands the map back as a plain value. */
func (run *execution) executeSelectionsAsValue(source interface{}, object *ObjectType, selections []Selection) (interface{}, *ExecutionError) {
	fields, failure := run.executeSelections(source, object, selections)
	if failure != nil {
		return nil, failure
	}
	return fields, nil
}

/** 🎭️ The concrete object a source discriminates itself as. */
func (run *execution) concrete(value interface{}) *ObjectType {
	fields, ok := value.(map[string]interface{})
	if !ok {
		return nil
	}
	name, ok := fields["__typename"].(string)
	if !ok {
		return nil
	}
	return run.schema.Object(name)
}

/** 🔣️ Renders one JSON value the way the reference implementation prints it in a diagnostic. */
func renderJSON(value interface{}) string {
	encoded, err := json.Marshal(value)
	if err != nil {
		return fmt.Sprintf("%v", value)
	}
	return string(encoded)
}

// #endregion ⚡️Execution

// #region 🚚️Split

// #region 📦️Utils

var executor *Executor

// 🕸️ensureExecutor lazily initializes the GraphQL executor on first use.
// This avoids the expensive initialization (LoadBundles, buildSchema) for
// 🕸️commands that don't need GraphQL (e.g. hook calls).
func ensureExecutor() {
	workspace.ExecutorOnce.Do(func() {
		var e error
		executor, e = NewExecutorWithContext(workspace.RootDir, NewRepoContext(workspace.RootDir))
		if e != nil {
			fmt.Fprintf(os.Stderr, "Warning: Failed to initialize GraphQL executor: %v\n", e)
			executor = nil
		}
	})
}

// #endregion 📦️Utils

// #region 💡️GraphQL Resolver

// 💿️Resolver holds the data fields for a resolver record.
type Resolver struct {
	RootDir string
	Ctx     model.RepoContext
}

// 🔷️NewResolver MUST initialize all required fields and return a valid resolver.
// 🆕️NewResolver creates and returns a new resolver instance.
func NewResolver(rootDir string) *Resolver {
	return &Resolver{RootDir: rootDir, Ctx: NewRepoContext(rootDir)}
}

// 📝️NewResolverWithContext MUST initialize all required fields and return a valid resolver with context.
// 🔤️NewResolverWithContext creates and returns a new resolver with context instance.
func NewResolverWithContext(rootDir string, ctx model.RepoContext) *Resolver {
	return &Resolver{RootDir: rootDir, Ctx: ctx}
}

// 🔶️context holds the data fields for a context record.
func (r *Resolver) context() model.RepoContext {
	return r.Ctx
}

// #endregion 💡️GraphQL Resolver

// #region 🩻️Default Context

// 📝️defaultContext holds the data fields for a defaultContext record.
type defaultContext struct {
	rootDir string
}

// 🔷️NewDefaultContext MUST initialize all required fields and return a valid default context.
// 🟧️NewDefaultContext creates and returns a new default context instance.
func NewDefaultContext(rootDir string) model.RepoContext {
	return &defaultContext{rootDir: rootDir}
}

// 💿️repoContext holds the data fields for a repoContext record.
type RepoContext struct {
	rootDir            string
	bundles            []model.Bundle
	managementProvider providers.ManagementProvider
}

// 🔶️NewRepoContext MUST initialize all required fields and return a valid repo context.
// 🐙️NewRepoContext creates and returns a new repo context instance.
func NewRepoContext(rootDir string) model.RepoContext {
	resolvedRoot := rootDir
	if strings.TrimSpace(resolvedRoot) == "" {
		resolvedRoot = workspace.FindRepoRoot("")
	}
	workspace.SetRootDir(resolvedRoot)
	ctx := &RepoContext{rootDir: resolvedRoot, managementProvider: providers.DefaultManagementProvider()}
	ctx.bundles = codebase.LoadBundles()
	return ctx
}

// 🔹️ManagementProv holds the data fields for a ManagementProv record.
func (c *RepoContext) ManagementProv() providers.ManagementProvider { return c.managementProvider }

// 📨️GetRootDir MUST retrieve the requested value or return an error.
// 🟨️GetRootDir retrieves and returns the root dir.
func (c *RepoContext) GetRootDir() string { return c.rootDir }

// ❌️GetFileID MUST retrieve the requested value or return an error.
// 🔑️GetFileID retrieves and returns the file i d.
func (c *RepoContext) GetFileID(path string) string {
	ctx := &codebase.CodebaseContext{RootDir: c.rootDir, Bundles: c.bundles}
	return ctx.GetFileID(path)
}

// 📁️GetFolderID MUST retrieve the requested value or return an error.
// 🟦️GetFolderID retrieves and returns the folder i d.
func (c *RepoContext) GetFolderID(path string) string {
	ctx := &codebase.CodebaseContext{RootDir: c.rootDir, Bundles: c.bundles}
	return ctx.GetFolderID(path)
}

// 📦️GetBundles MUST retrieve the requested value or return an error.
// 🟪️GetBundles retrieves and returns the bundles.
func (c *RepoContext) GetBundles() []*model.Bundle {
	result := make([]*model.Bundle, len(c.bundles))
	for i := range c.bundles {
		result[i] = &c.bundles[i]
	}
	return result
}

// 📜️GetTechnologies MUST retrieve the requested value or return an error.
// 📺️GetTechnologies retrieves and returns the technologies.
func (c *RepoContext) GetTechnologies() []*model.Technology {
	technologies := codebase.LoadTechnologies()
	res := make([]*model.Technology, len(technologies))
	for i := range technologies {
		res[i] = &technologies[i]
	}
	return res
}

// 💾️GetCheckpoints MUST retrieve the requested value or return an error.
// ✔️GetCheckpoints retrieves and returns the checkpoints.
func (c *RepoContext) GetCheckpoints(limit *int) ([]*model.Checkpoint, error) {
	checkpoints := contributorspkg.LoadCheckpoints(limit)
	res := make([]*model.Checkpoint, len(checkpoints))
	for i := range checkpoints {
		res[i] = &checkpoints[i]
	}
	return res, nil
}

// 🔸️GetFolders MUST retrieve the requested value or return an error.
// 🟫️GetFolders retrieves and returns the folders.
func (c *RepoContext) GetFolders() []*model.Folder {
	ctx := codebase.NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return []*model.Folder{}
	}
	folders := codebase.BuildCodebaseFolders(ctx)
	results := make([]*model.Folder, 0, len(folders))
	for _, entry := range folders {
		results = append(results, &model.Folder{
			ID:        entry.ID,
			Path:      entry.Path,
			URI:       entry.URI,
			Name:      entry.Name,
			ParentID:  entry.ParentID,
			BundleID:  entry.BundleID,
			Kind:      model.DeriveFolderKind(entry.Path),
			Generated: model.IsGeneratedFolder(entry.Path),
		})
	}
	return results
}

// 📄️GetFiles MUST retrieve the requested value or return an error.
// 💠️GetFiles retrieves and returns the files.
func (c *RepoContext) GetFiles() []*model.File {
	ctx := codebase.NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return []*model.File{}
	}
	files := codebase.BuildCodebaseFiles(ctx)
	results := make([]*model.File, 0, len(files))
	for _, entry := range files {
		name := filepath.Base(entry.Path)
		results = append(results, &model.File{
			ID:        entry.ID,
			Path:      entry.Path,
			URI:       entry.URI,
			Name:      name,
			Extension: filepath.Ext(entry.Path),
			FolderID:  entry.FolderID,
			BundleID:  entry.BundleID,
			Kind:      model.DeriveFileKind(name),
		})
	}
	return results
}

// 📖️GetDefinitions MUST retrieve the requested value or return an error.
// 🔳️GetDefinitions retrieves and returns the definitions.
func (c *RepoContext) GetDefinitions() []*model.Definition {
	ctx := codebase.NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return []*model.Definition{}
	}
	var results []*model.Definition
	for _, file := range ctx.Files {
		tool := move.ToolDefinitionList(file)
		definitions, ok := tool.Data.([]model.Definition)
		if !ok {
			continue
		}
		fileID := ctx.GetFileID(file)
		for i := range definitions {
			def := definitions[i]

			def.ID = codebase.DefinitionID(fileID, def)
			results = append(results, &def)
		}
	}
	return results
}

// 📑️GetSections MUST retrieve the requested value or return an error.
// 🔲️GetSections retrieves and returns the sections.
func (c *RepoContext) GetSections() []*model.Section {
	ctx := codebase.NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return []*model.Section{}
	}
	var results []*model.Section
	for _, file := range ctx.Files {
		tool := move.ToolSectionList(file)
		sections, ok := tool.Data.([]model.Section)
		if !ok {
			continue
		}
		fileID := ctx.GetFileID(file)
		for i := range sections {
			sec := sections[i]

			sec.FilePath = file
			sec.ID = model.BuildSectionID(fileID, languages.NormalizeSectionPath(sec.Path))
			results = append(results, &sec)
		}
	}
	return results
}

// 🤝️GetContributors MUST retrieve the requested value or return an error.
// ▪️GetContributors retrieves and returns the contributors.
func (c *RepoContext) GetContributors() ([]*model.Contributor, error) {
	contributors := contributorspkg.ListContributors(contributorspkg.NewFsContributorStore(c.rootDir))
	result := make([]*model.Contributor, len(contributors))
	for i := range contributors {
		result[i] = &contributors[i]
	}
	return result, nil
}

// 🎫️GetTickets MUST retrieve the requested value or return an error.
// ▫️GetTickets retrieves and returns the tickets.
func (c *RepoContext) GetTickets(year, month, day *int, status *model.TicketStatus) ([]*model.Ticket, error) {
	tickets, err := ticketspkg.ListTickets(year, month, day)
	if err != nil {
		return nil, err
	}
	var result []*model.Ticket
	for i := range tickets {
		if status == nil || tickets[i].GetStatus() == *status {
			result = append(result, &tickets[i])
		}
	}
	return result, nil
}

// ⛳️GetGoals MUST retrieve the requested value or return an error.
// ◾GetGoals retrieves and returns the goals.
func (c *RepoContext) GetGoals() ([]*model.Goal, error) {
	return goalspkg.ListGoals()
}

// 🆕️GoalCreate MUST return a non-nil error when the operation fails.
// ◽GoalCreate performs the goal create operation on the repo context.
func (c *RepoContext) GoalCreate(input model.GoalCreateInput) (*model.Goal, error) {

	if input.Title == "" {
		return nil, fmt.Errorf("missing title")
	}
	if input.Description == "" {
		return nil, fmt.Errorf("missing description")
	}
	if input.Prompt == "" {
		return nil, fmt.Errorf("missing prompt")
	}
	if input.DueDate == "" {
		return nil, fmt.Errorf("missing due date")
	}
	if input.LLM == "" {
		return nil, fmt.Errorf("missing llm")
	}
	if input.Client == "" {
		return nil, fmt.Errorf("missing client")
	}

	llmSlug, err := model.ResolveAllowedLLM(input.LLM)
	if err != nil {
		return nil, err
	}
	var effortSlug string
	if input.Effort != "" {
		effortSlug, err = model.ResolveAllowedEffort(input.Effort)
		if err != nil {
			return nil, err
		}
	}
	uiSlug, err := model.ResolveAllowedClient(input.Client)
	if err != nil {
		return nil, err
	}

	slug := workspace.Slugify(input.Title)
	id := slug
	if input.Parent != "" {
		id = input.Parent + "/" + slug
	}

	dir := workspace.GetRepoGoalsDir()
	path := filepath.Join(dir, filepath.FromSlash(id), "🎯️goal.json")
	if workspace.FileExists(path) {
		return nil, fmt.Errorf("goal with id %s already exists", id)
	}

	goal := model.Goal{
		ID:          id,
		Title:       input.Title,
		Parent:      input.Parent,
		Description: input.Description,
		Prompt:      input.Prompt,
		Status:      "open",
		Dates:       model.GoalDates{Due: input.DueDate},
		Client:      uiSlug,
		LLM:         llmSlug,
		Effort:      effortSlug,
	}

	if !input.NoManagement {
		if goalspkg.IsRootGoal(goal.ID) {
			if input.Milestone != "" {
				goal.Management = &model.GoalManagementData{Milestone: input.Milestone}
			} else {
				milestoneNumber, err := c.managementProvider.CreateMilestone(input.Title, input.Description)
				if err != nil {
					return nil, err
				}
				if input.DueDate != "" {
					_ = c.managementProvider.UpdateMilestone(milestoneNumber, "", "", "", input.DueDate)
				}
				repoUrl, _ := getGhRepoUrl()
				goal.Management = &model.GoalManagementData{
					Milestone: fmt.Sprintf("%s/milestone/%d", repoUrl, milestoneNumber),
				}
			}
		} else if goalspkg.IsFirstGenGoal(goal.ID) {
			milestone, _ := goalspkg.GetRootGoalMilestone(id)
			issueURL, err := c.managementProvider.CreateGoalIssue(input.Title, input.Description, milestone)
			if err != nil {
				return nil, err
			}
			goal.Management = &model.GoalManagementData{Issue: issueURL}
		} else {
			issueURL, err := c.managementProvider.CreateGoalIssue(input.Title, input.Description, nil)
			if err != nil {
				return nil, err
			}
			goal.Management = &model.GoalManagementData{Issue: issueURL}
			parentID := goalspkg.GetParentGoalID(id)
			parentGoal, parentErr := goalspkg.ReadGoal(parentID)
			if parentErr == nil && parentGoal.Management != nil && parentGoal.Management.Issue != "" {
				if err := c.managementProvider.AddSubIssue(parentGoal.Management.Issue, issueURL); err != nil {
					workspace.WriteWarningf("Failed to link sub-issue to parent goal %s: %v", parentID, err)
				}
			}
		}
	}

	if err := goalspkg.SaveGoal(goal); err != nil {
		return nil, err
	}
	events.Emit(events.EventGoalOpenEnded, "repo-cli", events.GoalOpenPayload{
		GoalPayload: events.GoalPayload{ID: goal.ID},
		Title:       goal.Title, Description: goal.Description, LLM: llmSlug, Effort: effortSlug, Parent: goal.Parent, Author: contributorspkg.GetGitAuthorAlias(),
	})
	return &goal, nil
}

// 🌐️getGhRepoUrl holds the data fields for a getGhRepoUrl record.
func getGhRepoUrl() (string, error) {
	out, err := exec.Command("gh", "repo", "view", "--json", "url", "--jq", ".url").Output()
	if err != nil {
		return "", err
	}
	return strings.TrimSpace(string(out)), nil
}

// 🔁️UpdateGoalTitle MUST apply the update and return an error if the target is missing.
// ❓️UpdateGoalTitle modifies an existing goal title entry.
func UpdateGoalTitle(goal *model.Goal, title string) error {
	title = strings.TrimSpace(title)
	if title == "" {
		return fmt.Errorf("goal title is required")
	}
	newSlug := workspace.Slugify(title)
	if title == newSlug {
		return fmt.Errorf("goal title must be titleized (e.g. \"Some Title on Something\") and NOT an all-caps slug")
	}
	if title == strings.ToLower(newSlug) {
		return fmt.Errorf("goal title must be titleized (e.g. \"Some Title on Something\") and NOT a slug")
	}

	var newID string
	if goal.Parent != "" {
		newID = goal.Parent + "/" + newSlug
	} else {
		newID = newSlug
	}

	if newID != goal.ID {
		dir := workspace.GetRepoGoalsDir()
		oldPath := filepath.Join(dir, filepath.FromSlash(goal.ID))
		newPath := filepath.Join(dir, filepath.FromSlash(newID))
		if workspace.FileExists(newPath) {
			return fmt.Errorf("goal folder already exists: %s", newPath)
		}
		if err := os.Rename(oldPath, newPath); err != nil {
			return err
		}
		goal.ID = newID
	}
	goal.Title = title
	return nil
}

// ♻️GoalChange MUST return a non-nil error when the operation fails.
// 📐️GoalChange performs the goal change operation on the repo context.
func (c *RepoContext) GoalChange(input model.GoalChangeInput) (*model.Goal, error) {
	dir := workspace.GetRepoGoalsDir()
	path := filepath.Join(dir, input.ID, "🎯️goal.json")
	content, err := workspace.ReadTextFile(path)
	if err != nil {
		return nil, err
	}
	var goal model.Goal
	if err := json.Unmarshal([]byte(content), &goal); err != nil {
		return nil, err
	}
	goal.ID = input.ID

	if input.Title != nil {
		if err := UpdateGoalTitle(&goal, *input.Title); err != nil {
			return nil, err
		}
	}
	if input.Description != nil {
		goal.Description = *input.Description
	}
	if input.DueDate != nil {
		goal.Dates.Due = *input.DueDate
	}
	if input.LLM != nil {
		llmSlug, err := model.ResolveAllowedLLM(*input.LLM)
		if err != nil {
			return nil, err
		}
		goal.LLM = llmSlug
	}
	if input.Effort != nil {
		effortSlug, err := model.ResolveAllowedEffort(*input.Effort)
		if err != nil {
			return nil, err
		}
		goal.Effort = effortSlug
	}
	if input.Parent != nil {
		goal.Parent = *input.Parent

		slug := goal.ID
		if idx := strings.LastIndex(goal.ID, "/"); idx != -1 {
			slug = goal.ID[idx+1:]
		}

		var newID string
		if goal.Parent != "" {
			newID = goal.Parent + "/" + slug
		} else {
			newID = slug
		}

		if newID != goal.ID {
			oldPath := filepath.Join(dir, filepath.FromSlash(goal.ID))
			newPath := filepath.Join(dir, filepath.FromSlash(newID))

			if workspace.FileExists(newPath) {
				return nil, fmt.Errorf("goal folder already exists: %s", newPath)
			}

			if err := os.MkdirAll(filepath.Dir(newPath), 0755); err != nil {
				return nil, err
			}

			if err := os.Rename(oldPath, newPath); err != nil {
				return nil, err
			}
			goal.ID = newID
		}
	}

	if goal.Management != nil && !input.NoManagement {
		if goalspkg.IsRootGoal(goal.ID) && goal.Management.Milestone != "" {
			number, err := model.ParseMilestoneNumber(goal.Management.Milestone)
			if err == nil {
				if err := c.managementProvider.UpdateMilestone(number, goal.Title, goal.Description, goal.Status, goal.Dates.Due); err != nil {
					return nil, err
				}
			}
		} else if !goalspkg.IsRootGoal(goal.ID) && goal.Management.Issue != "" {
			if err := c.managementProvider.UpdateGoalIssue(goal.Management.Issue, goal.Title, goal.Description); err != nil {
				return nil, err
			}
		}
	}

	if err := goalspkg.SaveGoal(goal); err != nil {
		return nil, err
	}
	events.Emit(events.EventGoalChangeEnded, "repo-cli", events.GoalChangePayload{
		GoalPayload: events.GoalPayload{ID: goal.ID},
		Title:       input.Title, Description: input.Description, LLM: input.LLM, Effort: input.Effort, Parent: input.Parent, Author: contributorspkg.GetGitAuthorAlias(),
	})
	return &goal, nil
}

// 📪️GoalClose MUST return a non-nil error when the operation fails.
// 🏁️GoalClose performs the goal close operation on the repo context.
func (c *RepoContext) GoalClose(input model.GoalCloseInput) (*model.Goal, error) {
	dir := workspace.GetRepoGoalsDir()
	path := filepath.Join(dir, input.ID, "🎯️goal.json")
	content, err := workspace.ReadTextFile(path)
	if err != nil {
		return nil, err
	}
	var goal model.Goal
	if err := json.Unmarshal([]byte(content), &goal); err != nil {
		return nil, err
	}
	goal.ID = input.ID

	if goal.Status == "closed" {
		return nil, fmt.Errorf("goal is already closed")
	}

	goal.Status = "closed"
	goal.Summary = input.Summary

	if goal.Management != nil && !input.NoManagement {
		if goalspkg.IsRootGoal(goal.ID) && goal.Management.Milestone != "" {
			number, err := model.ParseMilestoneNumber(goal.Management.Milestone)
			if err == nil {
				if err := c.managementProvider.UpdateMilestone(number, goal.Title, goal.Description, "closed", ""); err != nil {
					return nil, err
				}
			}
		} else if !goalspkg.IsRootGoal(goal.ID) && goal.Management.Issue != "" {
			if err := c.managementProvider.CloseIssue(goal.Management.Issue); err != nil {
				return nil, err
			}
		}
	}

	if err := goalspkg.SaveGoal(goal); err != nil {
		return nil, err
	}
	events.Emit(events.EventGoalCloseEnded, "repo-cli", events.GoalClosePayload{
		GoalPayload: events.GoalPayload{ID: goal.ID},
		Summary:     goal.Summary, Author: contributorspkg.GetGitAuthorAlias(),
	})
	return &goal, nil
}

// 🔓️GoalReopen MUST return a non-nil error when the operation fails.
// 🔄️GoalReopen performs the goal reopen operation on the repo context.
func (c *RepoContext) GoalReopen(input model.GoalReopenInput) (*model.Goal, error) {
	dir := workspace.GetRepoGoalsDir()
	path := filepath.Join(dir, input.ID, "🎯️goal.json")
	content, err := workspace.ReadTextFile(path)
	if err != nil {
		return nil, err
	}
	var goal model.Goal
	if err := json.Unmarshal([]byte(content), &goal); err != nil {
		return nil, err
	}
	goal.ID = input.ID

	if goal.Status == "open" {
		return nil, fmt.Errorf("goal is already open")
	}

	goal.Status = "open"

	if input.Title != nil {
		if err := UpdateGoalTitle(&goal, *input.Title); err != nil {
			return nil, err
		}
	}
	if input.Description != nil {
		goal.Description = *input.Description
	}
	if input.DueDate != nil {
		goal.Dates.Due = *input.DueDate
	}
	if input.Parent != nil {
		goal.Parent = *input.Parent
	}
	goal.Prompt = input.Prompt
	goal.LLM = input.LLM
	goal.Client = input.Client
	if input.Effort != "" {
		effortSlug, err := model.ResolveAllowedEffort(input.Effort)
		if err != nil {
			return nil, err
		}
		goal.Effort = effortSlug
	}

	if goal.Management != nil && !input.NoManagement {
		if goalspkg.IsRootGoal(goal.ID) && goal.Management.Milestone != "" {
			number, err := model.ParseMilestoneNumber(goal.Management.Milestone)
			if err == nil {
				if err := c.managementProvider.UpdateMilestone(number, goal.Title, goal.Description, "open", goal.Dates.Due); err != nil {
					return nil, err
				}
			}
		} else if !goalspkg.IsRootGoal(goal.ID) && goal.Management.Issue != "" {
			if err := c.managementProvider.ReopenIssue(goal.Management.Issue); err != nil {
				return nil, err
			}
		}
	}

	if err := goalspkg.SaveGoal(goal); err != nil {
		return nil, err
	}
	events.Emit(events.EventGoalReopenEnded, "repo-cli", events.GoalReopenPayload{
		GoalPayload: events.GoalPayload{ID: goal.ID},
		Prompt:      input.Prompt, Client: input.Client, LLM: input.LLM, Effort: goal.Effort, Author: contributorspkg.GetGitAuthorAlias(),
	})
	return &goal, nil
}

// 🔺️TicketChange MUST return a non-nil error when the operation fails.
// ♻️TicketChange performs the ticket change operation on the repo context.
func (c *RepoContext) TicketChange(input model.TicketChangeInput) (*model.Ticket, error) {
	ticket, err := ticketspkg.ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}

	if input.Title != nil && *input.Title != "" {
		if err := ticketspkg.UpdateTicketTitle(ticket, *input.Title); err != nil {
			return nil, err
		}

		if ticket.Management != nil && ticket.Management.Issue != "" && !input.NoManagement {
			if err := c.managementProvider.UpdateIssueTitle(ticket.Management.Issue, *input.Title); err != nil {
				workspace.WriteWarningf("Failed to update GitHub issue title: %v", err)
			}
		}
	}

	changed := false
	if input.Prompt != nil {
		ticket.Description = *input.Prompt
		if len(ticket.Interactions) > 0 {
			ticket.Interactions[len(ticket.Interactions)-1].Prompt = *input.Prompt
		}
		changed = true
	}

	if len(ticket.Interactions) > 0 {
		if input.LLM != nil {
			llmSlug, err := model.ResolveAllowedLLM(*input.LLM)
			if err != nil {
				return nil, err
			}
			ticket.Interactions[len(ticket.Interactions)-1].LLM = llmSlug
			changed = true
		}
		if input.Effort != nil {
			effortSlug, err := model.ResolveAllowedEffort(*input.Effort)
			if err != nil {
				return nil, err
			}
			ticket.Interactions[len(ticket.Interactions)-1].Effort = effortSlug
			changed = true
		}
		if input.Client != nil {
			uiSlug, err := model.ResolveAllowedClient(*input.Client)
			if err != nil {
				return nil, err
			}
			ticket.Interactions[len(ticket.Interactions)-1].Client = uiSlug
			changed = true
		}
	}
	if input.Goal != nil {
		ticket.Goal = *input.Goal
		changed = true
	}
	if input.Parent != nil {
		ticket.Parent = *input.Parent
		changed = true
	}

	if changed {
		if err := ticketspkg.SaveTicket(ticket); err != nil {
			return nil, err
		}
		ticketID := model.FormatTicketRelPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
		events.Emit(events.EventTicketChangeEnded, "repo-cli", events.TicketChangePayload{
			TicketPayload: events.TicketPayload{ID: ticketID, Year: ticket.Year, Month: ticket.Month, Day: ticket.Day, Slug: ticket.Slug},
			Title:         input.Title, Prompt: input.Prompt, LLM: input.LLM, Effort: input.Effort, Goal: input.Goal, Parent: input.Parent, Author: contributorspkg.GetGitAuthorAlias(),
		})
	}

	return ticket, nil
}

// 🗑️GoalDelete MUST return a non-nil error when the operation fails.
// ◼GoalDelete performs the goal delete operation on the repo context.
func (c *RepoContext) GoalDelete(input model.GoalDeleteInput) (bool, error) {
	dir := workspace.GetRepoGoalsDir()
	path := filepath.Join(dir, input.ID, "🎯️goal.json")
	content, err := workspace.ReadTextFile(path)
	if err != nil {
		return false, err
	}
	var goal model.Goal
	if err := json.Unmarshal([]byte(content), &goal); err != nil {
		return false, err
	}

	if !input.NoManagement && goal.Management != nil {
		if goalspkg.IsRootGoal(goal.ID) && goal.Management.Milestone != "" {
			number, err := model.ParseMilestoneNumber(goal.Management.Milestone)
			if err == nil {
				if err := c.managementProvider.DeleteMilestone(number); err != nil {
					return false, err
				}
			}
		} else if !goalspkg.IsRootGoal(goal.ID) && goal.Management.Issue != "" {
			parts := strings.Split(goal.Management.Issue, "/")
			number := parts[len(parts)-1]
			if err := c.managementProvider.DeleteIssue(number); err != nil {
				return false, err
			}
		}
	}

	if err := os.RemoveAll(filepath.Dir(path)); err != nil {
		return false, err
	}
	return true, nil
}

// 🔻️TicketDelete MUST return a non-nil error when the operation fails.
// 🔴️TicketDelete performs the ticket delete operation on the repo context.
func (c *RepoContext) TicketDelete(input model.TicketDeleteInput) (bool, error) {
	ticket, err := ticketspkg.ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return false, err
	}

	if !input.NoManagement && ticket.Management != nil {
		if ticket.Management.Issue != "" {
			parts := strings.Split(ticket.Management.Issue, "/")
			number := parts[len(parts)-1]
			if err := c.managementProvider.DeleteIssue(number); err != nil {
				return false, err
			}
		}
	}

	path := ticket.FolderPath
	if path == "" {
		path = ticketspkg.GetTicketPath(input.Year, input.Month, input.Day, input.Slug)
	}

	if err := os.RemoveAll(path); err != nil {
		return false, err
	}

	return true, nil
}

// 🔬️scanTodos returns every todo of the repository tree the context reads, at host paths.
func (c *RepoContext) scanTodos() []*model.Todo {
	return todospkg.LocateTodosOnDisk(c.rootDir, todospkg.ScanTodos(todospkg.NewFsTodoTree(c.rootDir)))
}

// 🎨️draftStore returns the store the notes directory of the repository keeps drafts in.
func (c *RepoContext) draftStore() todospkg.DraftStore {
	return todospkg.NewFsDraftStore(workspace.GetDraftsPath())
}

// ⬛️GetDrafts MUST retrieve the requested value or return an error.
// 🟠️GetDrafts retrieves and returns the drafts.
func (c *RepoContext) GetDrafts() ([]*model.Draft, error) {
	return todospkg.ListDrafts(c.draftStore()), nil
}

// ⬜️DraftCreate MUST return a non-nil error when the operation fails.
// 🟡️DraftCreate performs the draft create operation on the repo context.
func (c *RepoContext) DraftCreate(input model.DraftCreateInput) (*model.Draft, error) {
	files, err := todospkg.LoadTreeFiles(input.Files)
	if err != nil {
		return nil, err
	}
	return todospkg.CreateDraft(c.draftStore(), input.Title, files)
}

// 🟥️DraftDelete MUST return a non-nil error when the operation fails.
// 🟢️DraftDelete performs the draft delete operation on the repo context.
func (c *RepoContext) DraftDelete(id string) (bool, error) {
	return true, todospkg.DeleteDraft(c.draftStore(), id)
}

// 🟧️GetPolicies MUST retrieve the requested value or return an error.
// 🟣️GetPolicies retrieves and returns the policies.
func (c *RepoContext) GetPolicies() []*model.Policy {
	policies := statutespkg.GetRegisteredPolicies()
	result := make([]*model.Policy, len(policies))
	for i := range policies {
		var descPtr *string
		if policies[i].Description != "" {
			d := policies[i].Description
			descPtr = &d
		}
		var statutes []*model.StatuteMeta
		for _, kind := range policies[i].AllKinds() {
			meta := kind.Info()
			meta.PolicyID = policies[i].ID
			model.NormalizeStatuteMeta(&meta)
			statutes = append(statutes, &meta)
		}
		result[i] = &model.Policy{
			ID:          policies[i].ID,
			Name:        policies[i].Name,
			Description: descPtr,
			Scopes:      policies[i].Scopes,
			Groups:      policies[i].Groups,
			Statutes:    statutes,
		}
	}
	return result
}

// 🟨️GetStatutes MUST retrieve the requested value or return an error.
// 🟤️GetStatutes retrieves and returns the statutes.
// ⛳️GetInteractions aggregates every interaction recorded on a ticket or a goal.
func (c *RepoContext) GetInteractions() ([]model.InteractionResource, error) {
	return ticketspkg.ListInteractions()
}

func (c *RepoContext) GetStatutes() []*model.StatuteMeta {
	var result []*model.StatuteMeta
	for _, meta := range model.StatuteInfoTable {
		m := meta
		model.NormalizeStatuteMeta(&m)
		result = append(result, &m)
	}
	return result
}

// 🔬️Analyze MUST return a non-nil error when the operation fails.
// ⚪️Analyze performs the analyze operation on the repo context.
func (c *RepoContext) Analyze(scope *string) (*model.AnalyzeResult, error) {
	scopeStr := "compose"
	if scope != nil {
		scopeStr = *scope
	}
	s := workspace.ParseScope(scopeStr)
	all, err := statutespkg.LoadBreachsFromCache(workspace.RootDir)
	if err != nil {
		return nil, err
	}
	var filtered []model.Breach
	for i := range all {
		if statutespkg.BreachMatchesScope(&all[i], s) {
			filtered = append(filtered, all[i])
		}
	}
	pc := &model.PriorityCount{}
	autofixN := 0
	result := make([]*model.Breach, len(filtered))
	for i := range filtered {
		if filtered[i].ID == "" {
			fmt.Printf("[DEBUG] Analyze found breach with empty id: %+v\n", filtered[i])
		}
		result[i] = &filtered[i]
		switch filtered[i].Priority() {
		case model.BreachPriorityHigh:
			pc.High++
		case model.BreachPriorityMedium:
			pc.Medium++
		default:
			pc.Low++
		}
		if filtered[i].Autofixable() {
			autofixN++
		}
	}
	return &model.AnalyzeResult{
		Breachs: result,
		Metrics: &model.AnalyzeMetrics{
			Total:       len(filtered),
			ByPriority:  pc,
			Autofixable: autofixN,
		},
	}, nil
}

// 🏷️inferDefinitionKindFromLine holds the data fields for a inferDefinitionKindFromLine record.
func inferDefinitionKindFromLine(line string) model.DefinitionKind {
	lower := strings.ToLower(strings.TrimSpace(line))
	switch {
	case strings.HasPrefix(lower, "type "),
		strings.HasPrefix(lower, "interface "),
		strings.HasPrefix(lower, "trait "),
		strings.HasPrefix(lower, "protocol "),
		strings.HasPrefix(lower, "input "),
		strings.HasPrefix(lower, "union "),
		strings.HasPrefix(lower, "scalar "),
		strings.HasPrefix(lower, "delegate "),
		strings.HasPrefix(lower, "record "):
		return model.DefinitionKindInterface
	case strings.HasPrefix(lower, "const "),
		strings.HasPrefix(lower, "export const "),
		strings.HasPrefix(lower, "enum "),
		strings.HasPrefix(lower, "var "),
		strings.HasPrefix(lower, "let "),
		strings.HasPrefix(lower, "static "):
		return model.DefinitionKindConstant
	default:
		return model.DefinitionKindImplementation
	}
}

func applyAutofixes(file string, breachs []model.Breach) (int, error) {
	absPath := filepath.Join(workspace.RootDir, file)
	content, err := workspace.ReadTextFile(absPath)
	if err != nil {
		return 0, err
	}
	language := languages.GetLanguage(file)
	fixed := 0
	lines := strings.Split(content, "\n")
	sort.Slice(breachs, func(i, j int) bool {
		return breachs[i].Line > breachs[j].Line
	})
	linesToRemove := map[int]bool{}
	for _, v := range breachs {
		switch v.Kind {
		case model.BreachCodeFileMissingHeaderRegion:
			if language != nil && language.SupportsHeaders() {
				headerContent := move.GenerateFileHeader(file, language)
				if headerContent != "" {
					content = headerContent + "\n" + content
					lines = strings.Split(content, "\n")
					fixed++
				}
			}
		case model.BreachCodeSectionEmpty:
			sectionStartLine := 0
			sectionEndLine := 0
			for i := v.Line - 1; i >= 0; i-- {
				if language != nil {
					if matched, _ := language.PolicySectionStartMatch(lines[i]); matched {
						sectionStartLine = i + 1
						break
					}
				}
			}
			for i := v.Line - 1; i < len(lines); i++ {
				if language != nil {
					if matched, _ := language.PolicySectionEndMatch(lines[i]); matched {
						sectionEndLine = i + 1
						break
					}
				}
			}
			if sectionStartLine > 0 && sectionEndLine > 0 {
				for i := sectionStartLine; i <= sectionEndLine; i++ {
					linesToRemove[i] = true
				}
				hasPrecedingBlank := sectionStartLine > 1 && strings.TrimSpace(lines[sectionStartLine-2]) == ""
				hasFollowingBlank := sectionEndLine < len(lines) && strings.TrimSpace(lines[sectionEndLine]) == ""
				if hasPrecedingBlank {
					linesToRemove[sectionStartLine-1] = true
				}
				if hasFollowingBlank && !hasPrecedingBlank {
					linesToRemove[sectionEndLine+1] = true
				}
				fixed++
			}
		case model.BreachCodeSectionWrongFormatNewlineAfterRegion:
			if v.Line > 0 && v.Line <= len(lines) {
				if strings.TrimSpace(lines[v.Line-1]) == "" {
					linesToRemove[v.Line] = true
					fixed++
				}
			}
		case model.BreachCodeSectionMissingEndName:
			if v.Line > 0 && v.Line <= len(lines) && language != nil {
				line := lines[v.Line-1]
				if matched, _ := language.PolicySectionEndMatch(line); matched {
					startName := findMatchingSectionStartName(lines, v.Line-1, language)
					if startName != "" {
						lines[v.Line-1] = language.FormatSectionEnd(startName)
						fixed++
					}
				}
			}
		case model.BreachCodeSectionNameMismatch:
			if v.Line > 0 && v.Line <= len(lines) && language != nil {
				startName := findMatchingSectionStartName(lines, v.Line-1, language)
				if startName != "" {
					lines[v.Line-1] = language.FormatSectionEnd(startName)
					fixed++
				}
			}
		case model.BreachCodeDefNotNativeDocstring:
			if v.Line > 0 && v.Line <= len(lines) && language != nil {
				langName := language.Name()
				defLineNum := v.Line
				prefix := language.CommentPrefix()
				switch langName {
				case "typescript":
					var commentTexts []string
					commentStartIdx := defLineNum - 1
					for lineIndex := defLineNum - 2; lineIndex >= 0; lineIndex-- {
						line := strings.TrimSpace(lines[lineIndex])
						if line == "" {
							break
						}
						if !strings.HasPrefix(line, prefix) {
							break
						}
						commentStartIdx = lineIndex
						text := strings.TrimSpace(strings.TrimPrefix(line, prefix))
						commentTexts = append([]string{text}, commentTexts...)
					}
					if len(commentTexts) > 0 {
						var summaryLines, specLines, todoLines []string
						var identificationLine string
						inTodo := false
						for _, cl := range commentTexts {
							if strings.HasPrefix(cl, "[") && strings.Contains(cl, "](repo://definition/") {
								identificationLine = cl
								inTodo = false
								continue
							}
							if strings.HasPrefix(cl, "TODO:") || strings.HasPrefix(cl, "TODO ") {
								inTodo = true
								todoLines = append(todoLines, cl)
								continue
							}
							if inTodo {
								todoLines = append(todoLines, cl)
								continue
							}
							if statutespkg.IsSpecText(cl) {
								specLines = append(specLines, cl)
							} else {
								summaryLines = append(summaryLines, cl)
							}
						}
						indent := ""
						if defLineNum-1 < len(lines) {
							raw := lines[defLineNum-1]
							for _, ch := range raw {
								if ch == ' ' || ch == '\t' {
									indent += string(ch)
								} else {
									break
								}
							}
						}
						var jsdocLines []string
						jsdocLines = append(jsdocLines, indent+"/**")
						for _, sl := range summaryLines {
							jsdocLines = append(jsdocLines, indent+" * "+sl)
						}
						if len(specLines) > 0 {
							if len(summaryLines) > 0 {
								jsdocLines = append(jsdocLines, indent+" *")
							}
							for _, sp := range specLines {
								jsdocLines = append(jsdocLines, indent+" * "+sp)
							}
						}
						if len(todoLines) > 0 {
							jsdocLines = append(jsdocLines, indent+" *")
							for _, td := range todoLines {
								jsdocLines = append(jsdocLines, indent+" * "+td)
							}
						}
						if identificationLine != "" {
							jsdocLines = append(jsdocLines, indent+" * "+identificationLine)
						}
						jsdocLines = append(jsdocLines, indent+" **/")
						newLines := make([]string, 0, len(lines)-len(commentTexts)+len(jsdocLines))
						newLines = append(newLines, lines[:commentStartIdx]...)
						newLines = append(newLines, jsdocLines...)
						newLines = append(newLines, lines[defLineNum-1:]...)
						lines = newLines
						fixed++
					}
				case "csharp", "rust":
					for lineIndex := defLineNum - 2; lineIndex >= 0; lineIndex-- {
						line := strings.TrimSpace(lines[lineIndex])
						if line == "" {
							break
						}
						if strings.HasPrefix(line, "///") {
							break
						}
						if strings.HasPrefix(line, "//") {
							lines[lineIndex] = strings.Replace(lines[lineIndex], "// ", "/// ", 1)
							fixed++
						} else {
							break
						}
					}
				case "python":
					var commentTexts []string
					commentStartIdx := defLineNum - 1
					for lineIndex := defLineNum - 2; lineIndex >= 0; lineIndex-- {
						line := strings.TrimSpace(lines[lineIndex])
						if line == "" {
							break
						}
						if !strings.HasPrefix(line, prefix) {
							break
						}
						commentStartIdx = lineIndex
						text := strings.TrimSpace(strings.TrimPrefix(line, prefix))
						commentTexts = append([]string{text}, commentTexts...)
					}
					if len(commentTexts) > 0 {
						var cSummary, cRequirements, cTodos []string
						var cId string
						inTodo := false
						for _, cl := range commentTexts {
							if strings.HasPrefix(cl, "[") && strings.Contains(cl, "](repo://definition/") {
								cId = cl
								inTodo = false
								continue
							}
							if strings.HasPrefix(cl, "TODO:") || strings.HasPrefix(cl, "TODO ") {
								inTodo = true
								cTodos = append(cTodos, cl)
								continue
							}
							if inTodo {
								cTodos = append(cTodos, cl)
								continue
							}
							if statutespkg.IsSpecText(cl) {
								cRequirements = append(cRequirements, cl)
							} else {
								cSummary = append(cSummary, cl)
							}
						}
						bodyIndent := "    "
						parenDepth := 0
						for _, ch := range lines[defLineNum-1] {
							if ch == '(' {
								parenDepth++
							}
							if ch == ')' {
								parenDepth--
							}
						}
						bodyStart := defLineNum
						if parenDepth > 0 {
							for scanIdx := defLineNum; scanIdx < len(lines) && scanIdx < defLineNum+15; scanIdx++ {
								for _, ch := range lines[scanIdx] {
									if ch == '(' {
										parenDepth++
									}
									if ch == ')' {
										parenDepth--
									}
								}
								if parenDepth <= 0 {
									bodyStart = scanIdx + 1
									break
								}
							}
						}
						if bodyStart < len(lines) {
							raw := lines[bodyStart]
							detected := ""
							for _, ch := range raw {
								if ch == ' ' || ch == '\t' {
									detected += string(ch)
								} else {
									break
								}
							}
							if detected != "" {
								bodyIndent = detected
							}
						}
						existingDocStart := -1
						existingDocEnd := -1
						existingQuote := `"""`
						for bodyIdx := bodyStart; bodyIdx < len(lines) && bodyIdx < bodyStart+5; bodyIdx++ {
							trimmed := strings.TrimSpace(lines[bodyIdx])
							if trimmed == "" {
								continue
							}
							if strings.HasPrefix(trimmed, `"""`) || strings.HasPrefix(trimmed, `'''`) {
								existingDocStart = bodyIdx
								if strings.HasPrefix(trimmed, `'''`) {
									existingQuote = `'''`
								}
								afterOpen := strings.TrimPrefix(trimmed, existingQuote)
								closeIdx := strings.Index(afterOpen, existingQuote)
								if closeIdx >= 0 {
									existingDocEnd = bodyIdx
								} else {
									for scanIdx := bodyIdx + 1; scanIdx < len(lines); scanIdx++ {
										sline := strings.TrimSpace(lines[scanIdx])
										if sline == existingQuote || strings.HasSuffix(sline, existingQuote) {
											existingDocEnd = scanIdx
											break
										}
									}
								}
							}
							break
						}
						var eSummary, eRequirements, eTodos []string
						var eId string
						if existingDocStart >= 0 && existingDocEnd >= 0 {
							for lineIdx := existingDocStart; lineIdx <= existingDocEnd; lineIdx++ {
								trimmed := strings.TrimSpace(lines[lineIdx])
								trimmed = strings.TrimPrefix(trimmed, existingQuote)
								trimmed = strings.TrimSuffix(trimmed, existingQuote)
								trimmed = strings.TrimSpace(trimmed)
								if trimmed == "" {
									continue
								}
								if strings.HasPrefix(trimmed, "[") && strings.Contains(trimmed, "](repo://definition/") {
									eId = trimmed
								} else if statutespkg.IsSpecText(trimmed) {
									eRequirements = append(eRequirements, trimmed)
								} else if strings.HasPrefix(trimmed, "TODO:") || strings.HasPrefix(trimmed, "TODO ") {
									eTodos = append(eTodos, trimmed)
								} else {
									eSummary = append(eSummary, trimmed)
								}
							}
						}
						mergedSummary := eSummary
						if len(mergedSummary) == 0 {
							mergedSummary = cSummary
						}
						mergedRequirements := cRequirements
						if len(mergedRequirements) == 0 {
							mergedRequirements = eRequirements
						}
						mergedTodos := cTodos
						if len(mergedTodos) == 0 {
							mergedTodos = eTodos
						}
						mergedId := cId
						if mergedId == "" {
							mergedId = eId
						}
						var docLines []string
						for _, sl := range mergedSummary {
							docLines = append(docLines, sl)
						}
						for _, sp := range mergedRequirements {
							docLines = append(docLines, sp)
						}
						for _, td := range mergedTodos {
							docLines = append(docLines, td)
						}
						if mergedId != "" {
							docLines = append(docLines, mergedId)
						}
						var tripleQuoteLines []string
						if len(docLines) == 1 {
							tripleQuoteLines = append(tripleQuoteLines, bodyIndent+`"""`+docLines[0]+`"""`)
						} else if len(docLines) > 1 {
							tripleQuoteLines = append(tripleQuoteLines, bodyIndent+`"""`+docLines[0])
							for i := 1; i < len(docLines); i++ {
								tripleQuoteLines = append(tripleQuoteLines, bodyIndent+docLines[i])
							}
							tripleQuoteLines = append(tripleQuoteLines, bodyIndent+`"""`)
						}
						if len(tripleQuoteLines) > 0 {
							afterDoc := bodyStart
							if existingDocEnd >= 0 {
								afterDoc = existingDocEnd + 1
							}
							newLines := make([]string, 0, len(lines))
							newLines = append(newLines, lines[:commentStartIdx]...)
							newLines = append(newLines, lines[defLineNum-1:bodyStart]...)
							newLines = append(newLines, tripleQuoteLines...)
							newLines = append(newLines, lines[afterDoc:]...)
							lines = newLines
							fixed++
						}
					}
				}
			}
		case model.BreachCodeDefMissingSummary:
			if v.Line > 0 && v.Line <= len(lines) && language != nil {
				defName := ""
				if idx := strings.Index(v.Scope, "::"); idx >= 0 {
					defName = v.Scope[idx+2:]
				}
				if defName != "" {
					langName := language.Name()
					prefix := language.CommentPrefix()
					summaryText := defName + " holds the data fields for a " + defName + " record."
					defLine := lines[v.Line-1]
					trimmedDef := strings.TrimSpace(defLine)
					noPub := strings.TrimPrefix(trimmedDef, "pub ")
					if strings.HasPrefix(noPub, "(") {
						if cidx := strings.Index(noPub, ") "); cidx >= 0 {
							noPub = strings.TrimSpace(noPub[cidx+2:])
						}
					}
					noExport := strings.TrimPrefix(trimmedDef, "export ")
					noExport = strings.TrimLeft(noExport, "async abstract declare default ")
					if strings.HasPrefix(noPub, "fn ") || strings.HasPrefix(noExport, "function ") || strings.HasPrefix(trimmedDef, "def ") || strings.HasPrefix(trimmedDef, "async def ") || strings.HasPrefix(trimmedDef, "func ") {
						summaryText = defName + " performs the " + defName + " operation."
					}
					if langName == "python" {
						parenDepth := 0
						for _, ch := range defLine {
							if ch == '(' {
								parenDepth++
							}
							if ch == ')' {
								parenDepth--
							}
						}
						bodyStart := v.Line
						if parenDepth > 0 {
							for scanIdx := v.Line; scanIdx < len(lines) && scanIdx < v.Line+15; scanIdx++ {
								for _, ch := range lines[scanIdx] {
									if ch == '(' {
										parenDepth++
									}
									if ch == ')' {
										parenDepth--
									}
								}
								if parenDepth <= 0 {
									bodyStart = scanIdx + 1
									break
								}
							}
						}
						docstringFound := false
						for bodyIdx := bodyStart; bodyIdx < len(lines) && bodyIdx < bodyStart+5; bodyIdx++ {
							trimmed := strings.TrimSpace(lines[bodyIdx])
							if trimmed == "" {
								continue
							}
							if strings.HasPrefix(trimmed, `"""`) || strings.HasPrefix(trimmed, `'''`) {
								docstringFound = true
								quote := `"""`
								if strings.HasPrefix(trimmed, `'''`) {
									quote = `'''`
								}
								afterOpen := strings.TrimPrefix(trimmed, quote)
								closeIdx := strings.Index(afterOpen, quote)
								bodyIndent := ""
								for _, ch := range lines[bodyIdx] {
									if ch == ' ' || ch == '\t' {
										bodyIndent += string(ch)
									} else {
										break
									}
								}
								if closeIdx >= 0 {
									existingContent := strings.TrimSpace(afterOpen[:closeIdx])
									if existingContent == "" || strings.HasPrefix(existingContent, "[") {
										if existingContent != "" {
											lines[bodyIdx] = bodyIndent + quote + summaryText
											newLines := make([]string, 0, len(lines)+2)
											newLines = append(newLines, lines[:bodyIdx+1]...)
											newLines = append(newLines, bodyIndent+existingContent)
											newLines = append(newLines, bodyIndent+quote)
											newLines = append(newLines, lines[bodyIdx+1:]...)
											lines = newLines
										} else {
											lines[bodyIdx] = bodyIndent + quote + summaryText + quote
										}
									}
								} else {
									firstContent := strings.TrimSpace(afterOpen)
									if firstContent == "" || strings.HasPrefix(firstContent, "[") {
										lines[bodyIdx] = bodyIndent + quote + summaryText
										if firstContent != "" {
											newLines := make([]string, 0, len(lines)+1)
											newLines = append(newLines, lines[:bodyIdx+1]...)
											newLines = append(newLines, bodyIndent+firstContent)
											newLines = append(newLines, lines[bodyIdx+1:]...)
											lines = newLines
										}
									}
								}
								fixed++
							}
							break
						}
						if !docstringFound {
							bodyIndent := "    "
							if bodyStart < len(lines) {
								raw := lines[bodyStart]
								detected := ""
								for _, ch := range raw {
									if ch == ' ' || ch == '\t' {
										detected += string(ch)
									} else {
										break
									}
								}
								if detected != "" {
									bodyIndent = detected
								}
							}
							newLines := make([]string, 0, len(lines)+1)
							newLines = append(newLines, lines[:bodyStart]...)
							newLines = append(newLines, bodyIndent+`"""`+summaryText+`"""`)
							newLines = append(newLines, lines[bodyStart:]...)
							lines = newLines
							fixed++
						}
					} else if langName == "typescript" {
						prevIdx := v.Line - 2
						if prevIdx >= 0 {
							prevLine := strings.TrimSpace(lines[prevIdx])
							if strings.HasSuffix(prevLine, "**/") || strings.HasSuffix(prevLine, "*/") {
								indent := ""
								for _, ch := range lines[v.Line-1] {
									if ch == ' ' || ch == '\t' {
										indent += string(ch)
									} else {
										break
									}
								}
								for scanIdx := prevIdx; scanIdx >= 0; scanIdx-- {
									sline := strings.TrimSpace(lines[scanIdx])
									if strings.HasPrefix(sline, "/**") {
										openContent := strings.TrimPrefix(sline, "/**")
										openContent = strings.TrimSpace(openContent)
										if openContent == "" || openContent == "**/" || openContent == "*/" {
											lines[scanIdx] = indent + "/** " + summaryText
										}
										fixed++
										break
									}
								}
								break
							}
						}
						newLine := prefix + " " + summaryText
						insertAt := v.Line - 1
						for insertAt > 0 {
							prev := strings.TrimSpace(lines[insertAt-1])
							if prev == "" || !strings.HasPrefix(prev, prefix) {
								break
							}
							insertAt--
						}
						newLines := make([]string, 0, len(lines)+1)
						newLines = append(newLines, lines[:insertAt]...)
						newLines = append(newLines, newLine)
						newLines = append(newLines, lines[insertAt:]...)
						lines = newLines
						fixed++
					} else if langName == "csharp" || langName == "rust" {
						prevIdx := v.Line - 2
						if prevIdx >= 0 && strings.HasPrefix(strings.TrimSpace(lines[prevIdx]), "///") {
							hasSummaryTag := false
							docStartIdx := prevIdx
							for scanIdx := prevIdx; scanIdx >= 0; scanIdx-- {
								sline := strings.TrimSpace(lines[scanIdx])
								if !strings.HasPrefix(sline, "///") {
									break
								}
								docStartIdx = scanIdx
								if strings.Contains(sline, "<summary>") {
									hasSummaryTag = true
									summaryContent := strings.TrimPrefix(sline, "///")
									summaryContent = strings.TrimSpace(summaryContent)
									summaryContent = strings.TrimPrefix(summaryContent, "<summary>")
									summaryContent = strings.TrimSuffix(summaryContent, "</summary>")
									summaryContent = strings.TrimSpace(summaryContent)
									if summaryContent == "" {
										lines[scanIdx] = "/// <summary>" + summaryText + "</summary>"
										fixed++
									}
									break
								}
							}
							if !hasSummaryTag {
								summaryLine := "/// <summary>" + summaryText + "</summary>"
								newLines := make([]string, 0, len(lines)+1)
								newLines = append(newLines, lines[:docStartIdx]...)
								newLines = append(newLines, summaryLine)
								newLines = append(newLines, lines[docStartIdx:]...)
								lines = newLines
								fixed++
							}
							break
						}
						summaryLine := "/// <summary>" + summaryText + "</summary>"
						newLines := make([]string, 0, len(lines)+1)
						newLines = append(newLines, lines[:v.Line-1]...)
						newLines = append(newLines, summaryLine)
						newLines = append(newLines, lines[v.Line-1:]...)
						lines = newLines
						fixed++
					} else {
						newLine := prefix + " " + summaryText
						insertAt := v.Line - 1
						for insertAt > 0 {
							prev := strings.TrimSpace(lines[insertAt-1])
							if prev == "" || !strings.HasPrefix(prev, prefix) {
								break
							}
							insertAt--
						}
						newLines := make([]string, 0, len(lines)+1)
						newLines = append(newLines, lines[:insertAt]...)
						newLines = append(newLines, newLine)
						newLines = append(newLines, lines[insertAt:]...)
						lines = newLines
						fixed++
					}
				}
			}
		case model.BreachCodeDefMissingRequirements:
			if v.Line > 0 && v.Line <= len(lines) && language != nil {
				defName := ""
				if idx := strings.Index(v.Scope, "::"); idx >= 0 {
					defName = v.Scope[idx+2:]
				}
				if defName != "" {
					langName := language.Name()
					prefix := language.CommentPrefix()
					specText := v.Excerpt
					if langName == "python" {
						parenDepth := 0
						for _, ch := range lines[v.Line-1] {
							if ch == '(' {
								parenDepth++
							}
							if ch == ')' {
								parenDepth--
							}
						}
						bodyStart := v.Line
						if parenDepth > 0 {
							for scanIdx := v.Line; scanIdx < len(lines) && scanIdx < v.Line+15; scanIdx++ {
								for _, ch := range lines[scanIdx] {
									if ch == '(' {
										parenDepth++
									}
									if ch == ')' {
										parenDepth--
									}
								}
								if parenDepth <= 0 {
									bodyStart = scanIdx + 1
									break
								}
							}
						}
						for bodyIdx := bodyStart; bodyIdx < len(lines) && bodyIdx < bodyStart+5; bodyIdx++ {
							trimmed := strings.TrimSpace(lines[bodyIdx])
							if trimmed == "" {
								continue
							}
							if strings.HasPrefix(trimmed, `"""`) || strings.HasPrefix(trimmed, `'''`) {
								quote := `"""`
								if strings.HasPrefix(trimmed, `'''`) {
									quote = `'''`
								}
								bodyIndent := ""
								for _, ch := range lines[bodyIdx] {
									if ch == ' ' || ch == '\t' {
										bodyIndent += string(ch)
									} else {
										break
									}
								}
								afterOpen := strings.TrimPrefix(trimmed, quote)
								closeIdx := strings.Index(afterOpen, quote)
								if closeIdx >= 0 {
									existingContent := strings.TrimSpace(afterOpen[:closeIdx])
									lines[bodyIdx] = bodyIndent + quote + existingContent
									newLines := make([]string, 0, len(lines)+2)
									newLines = append(newLines, lines[:bodyIdx+1]...)
									newLines = append(newLines, bodyIndent+specText)
									newLines = append(newLines, bodyIndent+quote)
									newLines = append(newLines, lines[bodyIdx+1:]...)
									lines = newLines
								} else {
									for scanIdx := bodyIdx + 1; scanIdx < len(lines); scanIdx++ {
										sline := strings.TrimSpace(lines[scanIdx])
										if sline == quote || strings.HasSuffix(sline, quote) {
											insertIdx := scanIdx
											for backIdx := scanIdx - 1; backIdx > bodyIdx; backIdx-- {
												bline := strings.TrimSpace(lines[backIdx])
												if strings.HasPrefix(bline, "[") && strings.Contains(bline, "](repo://definition/") {
													insertIdx = backIdx
													break
												}
											}
											newLines := make([]string, 0, len(lines)+1)
											newLines = append(newLines, lines[:insertIdx]...)
											newLines = append(newLines, bodyIndent+specText)
											newLines = append(newLines, lines[insertIdx:]...)
											lines = newLines
											break
										}
									}
								}
								fixed++
							}
							break
						}
					} else if langName == "typescript" {
						prevIdx := v.Line - 2
						if prevIdx >= 0 {
							prevLine := strings.TrimSpace(lines[prevIdx])
							if strings.HasSuffix(prevLine, "**/") || strings.HasSuffix(prevLine, "*/") {
								indent := ""
								for _, ch := range lines[v.Line-1] {
									if ch == ' ' || ch == '\t' {
										indent += string(ch)
									} else {
										break
									}
								}
								for scanIdx := prevIdx; scanIdx >= 0; scanIdx-- {
									sline := strings.TrimSpace(lines[scanIdx])
									if strings.HasPrefix(sline, "/**") {
										newLines := make([]string, 0, len(lines)+1)
										newLines = append(newLines, lines[:scanIdx+1]...)
										newLines = append(newLines, indent+" * "+specText)
										newLines = append(newLines, lines[scanIdx+1:]...)
										lines = newLines
										fixed++
										break
									}
								}
								break
							}
						}
						newLine := prefix + " " + specText
						newLines := make([]string, 0, len(lines)+1)
						newLines = append(newLines, lines[:v.Line-1]...)
						newLines = append(newLines, newLine)
						newLines = append(newLines, lines[v.Line-1:]...)
						lines = newLines
						fixed++
					} else if langName == "csharp" || langName == "rust" {
						prevIdx := v.Line - 2
						if prevIdx >= 0 && strings.HasPrefix(strings.TrimSpace(lines[prevIdx]), "///") {
							hasRemarks := false
							remarksEnd := -1
							for scanIdx := prevIdx; scanIdx >= 0; scanIdx-- {
								sline := strings.TrimSpace(lines[scanIdx])
								if !strings.HasPrefix(sline, "///") {
									break
								}
								if strings.Contains(sline, "</remarks>") {
									remarksEnd = scanIdx
								}
								if strings.Contains(sline, "<remarks>") {
									hasRemarks = true
									break
								}
							}
							if hasRemarks && remarksEnd >= 0 {
								newLines := make([]string, 0, len(lines)+1)
								newLines = append(newLines, lines[:remarksEnd]...)
								newLines = append(newLines, "/// "+specText)
								newLines = append(newLines, lines[remarksEnd:]...)
								lines = newLines
								fixed++
							} else {
								newLines := make([]string, 0, len(lines)+3)
								newLines = append(newLines, lines[:v.Line-1]...)
								newLines = append(newLines, "/// <remarks>")
								newLines = append(newLines, "/// "+specText)
								newLines = append(newLines, "/// </remarks>")
								newLines = append(newLines, lines[v.Line-1:]...)
								lines = newLines
								fixed++
							}
							break
						}
						newLine := "/// " + specText
						newLines := make([]string, 0, len(lines)+1)
						newLines = append(newLines, lines[:v.Line-1]...)
						newLines = append(newLines, newLine)
						newLines = append(newLines, lines[v.Line-1:]...)
						lines = newLines
						fixed++
					} else {
						newLine := prefix + " " + specText
						newLines := make([]string, 0, len(lines)+1)
						newLines = append(newLines, lines[:v.Line-1]...)
						newLines = append(newLines, newLine)
						newLines = append(newLines, lines[v.Line-1:]...)
						lines = newLines
						fixed++
					}
				}
			}
		case model.BreachCodeSectionMissingSummary:
			if v.Line > 0 && v.Line <= len(lines) && language != nil {
				sectionName := ""
				if idx := strings.Index(v.Scope, "#"); idx >= 0 {
					sectionName = v.Scope[idx+1:]
				}
				if sectionName != "" {
					prefix := language.CommentPrefix()
					summaryLine := prefix + " " + sectionName + " MUST provide the " + strings.ToLower(sectionName) + " functionality."
					insertAt := v.Line
					for i := v.Line; i < len(lines); i++ {
						line := strings.TrimSpace(lines[i])
						if line == "" {
							continue
						}
						if strings.HasPrefix(line, prefix) {
							commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
							if strings.HasPrefix(commentText, "[") && strings.Contains(commentText, "](repo://section/") {
								insertAt = i + 1
								break
							}
						}
						break
					}
					newLines := make([]string, 0, len(lines)+1)
					newLines = append(newLines, lines[:insertAt]...)
					newLines = append(newLines, summaryLine)
					newLines = append(newLines, lines[insertAt:]...)
					lines = newLines
					fixed++
				}
			}
		case model.BreachCodeCommentInline:
			if v.Line > 0 && v.Line <= len(lines) && language != nil {
				startLine := v.Line
				prefix := language.CommentPrefix()
				var pendingBlanks []int
				for i := startLine; i <= len(lines); i++ {
					line := lines[i-1]
					trimmed := strings.TrimSpace(line)
					if trimmed == "" {
						pendingBlanks = append(pendingBlanks, i)
						continue
					}
					if i == startLine && v.Column > 1 {
						if v.Column <= len(line) {
							lines[i-1] = strings.TrimRight(line[:v.Column-1], " \t")
							pendingBlanks = nil
							continue
						}
					}
					if !strings.HasPrefix(trimmed, prefix) {
						break
					}
					if matched, _ := language.PolicySectionStartMatch(line); matched {
						break
					}
					if matched, _ := language.PolicySectionEndMatch(line); matched {
						break
					}
					isSkipDirective := false
					for _, d := range language.SkipDirectives() {
						if strings.HasPrefix(trimmed, prefix+" "+d) {
							isSkipDirective = true
							break
						}
					}
					if isSkipDirective {
						break
					}
					if strings.Contains(lines[i-1], "[DEBUG]") {
						break
					}
					for _, bl := range pendingBlanks {
						linesToRemove[bl] = true
					}
					pendingBlanks = nil
					linesToRemove[i] = true
				}
				fixed++
			}
		case model.BreachCodeCommentBlock, model.BreachCodeCommentJSDoc:
			if v.Line > 0 && v.Line <= len(lines) && language != nil {
				startLine := v.Line
				endPrefix := language.BlockCommentEnd()
				startPrefix := language.BlockCommentStart()
				for i := startLine; i <= len(lines); i++ {
					line := lines[i-1]
					if i == startLine && v.Column > 1 {
						idx := strings.Index(line[v.Column-1:], startPrefix)
						if idx >= 0 {
							idx += v.Column - 1
							left := strings.TrimRight(line[:idx], " \t")
							if strings.Contains(line[idx:], endPrefix) {

								endIdx := strings.Index(line[idx:], endPrefix) + idx
								right := ""
								if endIdx+len(endPrefix) < len(line) {
									right = line[endIdx+len(endPrefix):]
								}
								if left != "" && strings.TrimSpace(right) != "" {
									lines[i-1] = left + " " + strings.TrimLeft(right, " \t")
								} else {
									lines[i-1] = left + right
								}
								if strings.TrimSpace(lines[i-1]) == "" {
									linesToRemove[i] = true
								}
								break
							}
							if left != "" {
								lines[i-1] = left
							} else {
								linesToRemove[i] = true
							}
							continue
						}
					}
					if strings.Contains(line, endPrefix) {
						idx := strings.Index(line, endPrefix)
						if idx+len(endPrefix) < len(line) {
							lines[i-1] = strings.TrimLeft(line[idx+len(endPrefix):], " \t")
							if strings.TrimSpace(lines[i-1]) == "" {
								linesToRemove[i] = true
							}
						} else {
							linesToRemove[i] = true
						}
						break
					}
					linesToRemove[i] = true
				}
				fixed++
			}
		case model.BreachCodeUnicodeEmojiVariation:
			if v.Line > 0 && v.Line <= len(lines) {
				line := lines[v.Line-1]

				line = strings.ReplaceAll(line, "\uFE0E", "\uFE0F")

				textDefaultEmojis := []string{
					"\U0001F3D7",
					"\u2328",
					"\U0001F5B1",
					"\U0001F5C3",
					"\u2699",
					"\u2696",
					"\U0001F3F7",
					"\U0001F6E0",
					"\u2702",
					"\U0001F6E1",
				}
				for _, emoji := range textDefaultEmojis {

					line = strings.ReplaceAll(line, emoji+"\uFE0F", emoji)
					line = strings.ReplaceAll(line, emoji, emoji+"\uFE0F")
				}
				lines[v.Line-1] = line
				fixed++
			}
		case model.BreachCodeFileMissingLicense, model.BreachCodeFileWrongLicense:
			if language != nil {
				sections := language.ParseSections(strings.Join(lines, "\n"))
				var headerSec *model.Section
				for i := range sections {
					if strings.ToLower(sections[i].Name) == "header" {
						headerSec = &sections[i]
						break
					}
				}
				if headerSec != nil {
					var licenseSec *model.Section
					for i := range headerSec.Children {
						if strings.ToLower(headerSec.Children[i].Name) == "license" {
							licenseSec = &headerSec.Children[i]
							break
						}
					}
					prefix := language.CommentPrefix()
					licenseText := workspace.AGPLLicenseText()
					var licenseLines []string
					licenseLines = append(licenseLines, "")
					for _, ll := range strings.Split(licenseText, "\n") {
						if ll == "" {
							licenseLines = append(licenseLines, prefix)
						} else {
							licenseLines = append(licenseLines, prefix+" "+ll)
						}
					}
					licenseLines = append(licenseLines, "")
					if licenseSec != nil {
						newLines := make([]string, 0, len(lines)+len(licenseLines))
						newLines = append(newLines, lines[:licenseSec.StartLine]...)
						newLines = append(newLines, licenseLines...)
						newLines = append(newLines, lines[licenseSec.EndLine-1:]...)
						lines = newLines
					} else {
						requirementsSec := (*model.Section)(nil)
						for i := range headerSec.Children {
							if strings.ToLower(headerSec.Children[i].Name) == "requirements" {
								requirementsSec = &headerSec.Children[i]
								break
							}
						}
						insertBefore := headerSec.EndLine - 1
						if requirementsSec != nil {
							insertBefore = requirementsSec.StartLine - 1
						}
						regionStart := language.FormatSectionStart("License")
						regionEnd := language.FormatSectionEnd("License")
						var block []string
						block = append(block, regionStart)
						block = append(block, licenseLines...)
						block = append(block, regionEnd)
						block = append(block, "")
						newLines := make([]string, 0, len(lines)+len(block))
						newLines = append(newLines, lines[:insertBefore]...)
						newLines = append(newLines, block...)
						newLines = append(newLines, lines[insertBefore:]...)
						lines = newLines
					}
					fixed++
				}
			}
		}
	}
	systemFixed, systemErr := applySystemAutofixes(breachs)
	if systemErr != nil {
		return fixed, systemErr
	}
	fixed += systemFixed
	if len(linesToRemove) > 0 {
		var newLines []string
		for i, line := range lines {
			if !linesToRemove[i+1] {
				newLines = append(newLines, line)
			}
		}
		var collapsed []string
		for i, line := range newLines {
			if strings.TrimSpace(line) == "" && i > 0 && strings.TrimSpace(newLines[i-1]) == "" {
				continue
			}
			collapsed = append(collapsed, line)
		}
		content = strings.Join(collapsed, "\n")
	} else {
		content = strings.Join(lines, "\n")
	}
	if fixed > 0 {
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			return 0, err
		}
		if err := statutespkg.RunFormatterAfterAutofix(file, language); err != nil {
			return 0, err
		}
	}
	return fixed, nil
}

// ­ƒƒ®applySystemAutofixes holds the data fields for a applySystemAutofixes record.
func applySystemAutofixes(breachs []model.Breach) (int, error) {
	fixed := 0
	for _, v := range breachs {
		switch v.Kind {
		case model.BreachSystemDevcontainerVscodeSettingsOutside:
			settingsPath := filepath.Join(workspace.RootDir, ".vscode", "settings.json")
			settingsData, err := os.ReadFile(settingsPath)
			if err != nil {
				continue
			}
			var settings map[string]interface{}
			if err := json.Unmarshal(settingsData, &settings); err != nil {
				continue
			}
			devcontainerPath := filepath.Join(workspace.RootDir, ".devcontainer", "devcontainer.json")
			var devcontainer map[string]interface{}
			if dcData, err := os.ReadFile(devcontainerPath); err == nil {
				_ = json.Unmarshal(dcData, &devcontainer)
			}
			if devcontainer == nil {
				devcontainer = map[string]interface{}{}
			}
			customizations, _ := devcontainer["customizations"].(map[string]interface{})
			if customizations == nil {
				customizations = map[string]interface{}{}
			}
			vscodeCustom, _ := customizations["vscode"].(map[string]interface{})
			if vscodeCustom == nil {
				vscodeCustom = map[string]interface{}{}
			}
			vscodeCustom["settings"] = settings
			customizations["vscode"] = vscodeCustom
			devcontainer["customizations"] = customizations
			dcOut, err := json.MarshalIndent(devcontainer, "", "  ")
			if err != nil {
				continue
			}
			if err := os.MkdirAll(filepath.Join(workspace.RootDir, ".devcontainer"), 0755); err != nil {
				continue
			}
			if err := os.WriteFile(devcontainerPath, append(dcOut, '\n'), 0644); err != nil {
				continue
			}
			_ = os.Remove(settingsPath)
			vscodeDir := filepath.Join(workspace.RootDir, ".vscode")
			if entries, err := os.ReadDir(vscodeDir); err == nil && len(entries) == 0 {
				_ = os.Remove(vscodeDir)
			}
			fixed++
		case model.BreachFolderIllegalEmpty:
			folderPath := filepath.Join(workspace.RootDir, v.Excerpt)
			entries, readErr := os.ReadDir(folderPath)
			if readErr == nil && len(entries) == 0 {
				if err := os.Remove(folderPath); err == nil {
					fixed++
				}
			}
		case model.BreachSystemDevcontainerVscodeExtensionsOutside:
			devcontainerExtensions := statutespkg.ReadDevcontainerVscodeExtensions(workspace.RootDir)
			workspaceRecommendations, _ := statutespkg.ReadWorkspaceExtensionRecommendations(workspace.RootDir)
			merged := statutespkg.MergeWorkspaceExtensionRecommendations(devcontainerExtensions, workspaceRecommendations)
			if len(merged) == 0 {
				continue
			}
			if err := statutespkg.WriteWorkspaceExtensionRecommendations(workspace.RootDir, merged); err != nil {
				continue
			}
			fixed++
		}
	}
	return fixed, nil
}

// findMatchingSectionStartName locates the section start name for autofix helpers.
func findMatchingSectionStartName(lines []string, endLineIdx int, language languages.LanguagePlugin) string {
	depth := 0
	for i := endLineIdx - 1; i >= 0; i-- {
		if matched, _ := language.PolicySectionEndMatch(lines[i]); matched {
			depth++
			continue
		}
		if matched, name := language.PolicySectionStartMatch(lines[i]); matched {
			if depth > 0 {
				depth--
				continue
			}
			return name
		}
	}
	return ""
}

// 📬️TicketOpen MUST return a non-nil error when the operation fails.
// ⚫️TicketOpen performs the ticket open operation on the repo context.
func (c *RepoContext) TicketOpen(input model.TicketOpenInput) (*model.Ticket, error) {
	resolvedClient, err := model.ResolveAllowedClient(input.Client)
	if err != nil {
		return nil, err
	}
	kind := providers.McpKindFromResolvedClient(resolvedClient)
	return ticketspkg.OpenTicket(input.Emoji, input.Title, input.Prompt, input.LLM, input.Effort, input.Client, input.Draft, input.NoIssue, input.Goal, input.Parent, input.NoManagement, input.Issue, kind, input.PlanID, input.SpecID)
}

// 🟦️TicketClose MUST return a non-nil error when the operation fails.
// 🩵️TicketClose performs the ticket close operation on the repo context.
func (c *RepoContext) TicketClose(input model.TicketCloseInput) (*model.Ticket, error) {
	if input.All {
		tickets, err := ticketspkg.ListTickets(nil, nil, nil)
		if err != nil {
			return nil, err
		}
		var lastTicket *model.Ticket
		for _, t := range tickets {
			if t.Status == model.TicketStatusOpen {
				ticket := t
				fmt.Printf("Closing ticket %s...\n", ticket.Slug)
				if err := ticketspkg.FinishTicket(&ticket, "Bulk close", []string{}, input.NoManagement, true); err != nil {
					workspace.WriteWarningf("Failed to close ticket %s: %v", ticket.Slug, err)
					continue
				}
				lastTicket = &ticket
			}
		}

		if !input.NoManagement {
			issueURLs, err := c.managementProvider.ListOpenIssuesWithLabel("ticket")
			if err != nil {
				workspace.WriteWarningf("Failed to list GitHub issues with 'ticket' label: %v", err)
			} else {
				for _, issueURL := range issueURLs {
					fmt.Printf("Closing GitHub issue %s...\n", issueURL)
					if err := c.managementProvider.CloseIssue(issueURL); err != nil {
						workspace.WriteWarningf("Failed to close GitHub issue %s: %v", issueURL, err)
					}
				}
			}
		}
		return lastTicket, nil
	}
	ticket, err := ticketspkg.ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}

	if input.Title != nil && *input.Title != "" {
		if err := ticketspkg.UpdateTicketTitle(ticket, *input.Title); err != nil {
			return nil, err
		}

		if ticket.Management != nil && ticket.Management.Issue != "" && !input.NoManagement {
			if err := c.managementProvider.UpdateIssueTitle(ticket.Management.Issue, *input.Title); err != nil {
				workspace.WriteWarningf("Failed to update GitHub issue title: %v", err)
			}
		}
	}
	if err := ticketspkg.FinishTicket(ticket, input.Summary, input.Files, input.NoManagement, false); err != nil {
		return nil, err
	}
	return ticket, nil
}

// 🟪️TicketReopen MUST return a non-nil error when the operation fails.
// 🩶️TicketReopen performs the ticket reopen operation on the repo context.
func (c *RepoContext) TicketReopen(input model.TicketReopenInput) (*model.Ticket, error) {
	ticket, err := ticketspkg.ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}

	if input.Title != nil && *input.Title != "" {
		if err := ticketspkg.UpdateTicketTitle(ticket, *input.Title); err != nil {
			return nil, err
		}

		if ticket.Management != nil && ticket.Management.Issue != "" && !input.NoManagement {
			if err := c.managementProvider.UpdateIssueTitle(ticket.Management.Issue, *input.Title); err != nil {
				workspace.WriteWarningf("Failed to update GitHub issue title: %v", err)
			}
		}
	}
	resolvedClient, err := model.ResolveAllowedClient(input.Client)
	if err != nil {
		return nil, err
	}
	kind := providers.McpKindFromResolvedClient(resolvedClient)
	if err := ticketspkg.ReopenTicket(ticket, input.Prompt, input.LLM, input.Effort, input.Client, input.Draft, input.Goal, input.Parent, input.NoManagement, kind, input.PlanID, input.SpecID); err != nil {
		return nil, err
	}
	return ticket, nil
}

// 🟫️FolderCreate MUST return a non-nil error when the operation fails.
// 🩷️FolderCreate performs the folder create operation on the repo context.
func (c *RepoContext) FolderCreate(path string) (*model.Folder, error) { return nil, nil }

// 🚚️FolderMove MUST return a non-nil error when the operation fails.
// 💜️FolderMove performs the folder move operation on the repo context.
func (c *RepoContext) FolderMove(src, dst string) (*model.Folder, error) {
	result := move.ToolFolderMove(src, dst)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	ctx := &codebase.CodebaseContext{Bundles: c.bundles}
	return &model.Folder{ID: ctx.GetFolderID(dst), Path: dst, Name: filepath.Base(dst)}, nil
}

// 💠️FolderDelete MUST return a non-nil error when the operation fails.
// 💙️FolderDelete performs the folder delete operation on the repo context.
func (c *RepoContext) FolderDelete(path string) error {
	result := move.ToolFolderDelete(path)
	if result.Error != "" {
		return errors.New(result.Error)
	}
	return nil
}

// 🔳️FileCreate MUST return a non-nil error when the operation fails.
// 💚️FileCreate performs the file create operation on the repo context.
func (c *RepoContext) FileCreate(path string) (*model.File, error) {
	result := move.ToolFileCreate(path)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	ctx := &codebase.CodebaseContext{Bundles: c.bundles}
	return &model.File{ID: ctx.GetFileID(path), Path: path, Name: filepath.Base(path), Extension: strings.TrimPrefix(filepath.Ext(path), ".")}, nil
}

// 🔲️FileMove MUST return a non-nil error when the operation fails.
// 💛️FileMove performs the file move operation on the repo context.
func (c *RepoContext) FileMove(src, dst string) (*model.File, error) {
	result := move.ToolFileMove(src, dst)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	ctx := &codebase.CodebaseContext{Bundles: c.bundles}
	return &model.File{ID: ctx.GetFileID(dst), Path: dst, Name: filepath.Base(dst), Extension: strings.TrimPrefix(filepath.Ext(dst), ".")}, nil
}

// ▪️FileDelete MUST return a non-nil error when the operation fails.
// 🧡️FileDelete performs the file delete operation on the repo context.
func (c *RepoContext) FileDelete(path string) error {
	result := move.ToolFileDelete(path)
	if result.Error != "" {
		return errors.New(result.Error)
	}
	return nil
}

// ▫️SectionCreate MUST return a non-nil error when the operation fails.
// ❤️SectionCreate performs the section create operation on the repo context.
func (c *RepoContext) SectionCreate(file, name string, parent *string) (*model.Section, error) {
	sectionPath := name
	if parent != nil && *parent != "" {
		sectionPath = *parent + "/" + name
	}
	result := move.ToolSectionCreate(file, sectionPath)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	fileID := c.GetFileID(file)
	id := model.BuildSectionID(fileID, strings.Split(sectionPath, "/"))
	return &model.Section{ID: id, Name: name, Path: sectionPath, FilePath: file}, nil
}

// ◾SectionMove MUST return a non-nil error when the operation fails.
// 🤍️SectionMove performs the section move operation on the repo context.
func (c *RepoContext) SectionMove(file, oldName, newName string) (*model.Section, error) {
	result := move.ToolSectionMove(file, oldName, newName)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	fileID := c.GetFileID(file)
	id := model.BuildSectionID(fileID, strings.Split(newName, "/"))
	return &model.Section{ID: id, Name: newName, Path: newName, FilePath: file}, nil
}

// ◽SectionDelete MUST return a non-nil error when the operation fails.
// 🖤️SectionDelete performs the section delete operation on the repo context.
func (c *RepoContext) SectionDelete(file, name string) error {
	result := move.ToolSectionDelete(file, name)
	if result.Error != "" {
		return errors.New(result.Error)
	}
	return nil
}

// 🧬️Integrate MUST return a non-nil error when the operation fails.
// 🧬️Integrate performs the integrate operation on the repo context.
func (c *RepoContext) Integrate(source, targetSection, targetFile, targetParent *string) (*model.File, error) {
	s := ""
	if source != nil {
		s = *source
	}
	ts := ""
	if targetSection != nil {
		ts = *targetSection
	}
	tf := ""
	if targetFile != nil {
		tf = *targetFile
	}
	tp := ""
	if targetParent != nil {
		tp = *targetParent
	}
	result := move.ToolIntegrate(s, ts, tf, tp)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	return &model.File{ID: c.GetFileID(tf), Path: tf, Name: filepath.Base(tf)}, nil
}

// 🧲️Extract MUST return the extracted component from the input.
// 🤎️Extract extracts the extract from the source.
func (c *RepoContext) Extract(sourceFile, sourceSection, targetFile *string) (*model.File, error) {
	s := ""
	if sourceFile != nil {
		s = *sourceFile
	}
	ss := ""
	if sourceSection != nil {
		ss = *sourceSection
	}
	tf := ""
	if targetFile != nil {
		tf = *targetFile
	}

	result := move.ToolExtract(s, ss, tf)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	return &model.File{ID: c.GetFileID(tf), Path: tf, Name: filepath.Base(tf)}, nil
}

// ➕️ContributorAdd MUST return a non-nil error when the operation fails.
// ➕️ContributorAdd performs the contributor add operation on the repo context.
func (c *RepoContext) ContributorAdd(input model.ContributorAddInput) (*model.Contributor, error) {
	contributor, err := contributorspkg.LoadContributor(input.Github)
	if err != nil {
		contributor, err = contributorspkg.CreateContributor(input.Github)
		if err != nil {
			return nil, err
		}
	}

	changed := false
	if input.Name != nil && *input.Name != "" && contributor.Name == "" {
		contributor.Name = *input.Name
		changed = true
	}
	for _, n := range input.Names {
		if !slices.Contains(contributor.Names, n) {
			contributor.Names = append(contributor.Names, n)
			changed = true
		}
	}
	if input.Email != nil && *input.Email != "" && contributor.Email == "" {
		contributor.Email = *input.Email
		changed = true
	}
	for _, e := range input.Emails {
		if !slices.Contains(contributor.Emails, e) {
			contributor.Emails = append(contributor.Emails, e)
			changed = true
		}
	}
	if input.Fingerprint != nil && *input.Fingerprint != "" && contributor.Fingerprint == "" {
		contributor.Fingerprint = *input.Fingerprint
		changed = true
	}
	for _, f := range input.Fingerprints {
		if !slices.Contains(contributor.Fingerprints, f) {
			contributor.Fingerprints = append(contributor.Fingerprints, f)
			changed = true
		}
	}

	if changed {
		if err := contributorspkg.SaveContributor(*contributor); err != nil {
			return nil, err
		}
		events.Emit(events.EventContributorAddEnded, "repo-cli", events.ContributorPayload{
			Github: contributor.Github, Author: contributorspkg.GetGitAuthorAlias(),
		})
	}
	return contributor, nil
}

// ➖️ContributorRemove MUST return a non-nil error when the operation fails.
// 💗️ContributorRemove performs the contributor remove operation on the repo context.
func (c *RepoContext) ContributorRemove(github string) error {
	events.Emit(events.EventContributorRemoveEnded, "repo-cli", events.ContributorPayload{
		Github: github, Author: contributorspkg.GetGitAuthorAlias(),
	})
	return nil
}

func updateGoalMilestone(goal *model.Goal, number int) (int, error) {
	repoUrl, err := getGhRepoUrl()
	if err != nil {
		return number, err
	}
	if goal.Management == nil {
		goal.Management = &model.GoalManagementData{}
	}
	milestoneUrl := fmt.Sprintf("%s/milestone/%d", repoUrl, number)
	if goal.Management.Milestone != milestoneUrl {
		goal.Management.Milestone = milestoneUrl
		if err := goalspkg.SaveGoal(*goal); err != nil {
			return number, err
		}
	}
	return number, nil
}

// ◻ensureGoalMilestone holds the data fields for a ensureGoalMilestone record.
func ensureGoalMilestone(goal *model.Goal) (*providers.ManagementMilestone, error) {
	if goal == nil {
		return nil, nil
	}
	if strings.TrimSpace(goal.Title) == "" {
		return nil, nil
	}
	if !goalspkg.IsRootGoal(goal.ID) {
		return nil, nil
	}
	if goal.Management != nil && goal.Management.Milestone != "" {
		if n, err := model.ParseMilestoneNumber(goal.Management.Milestone); err == nil && n > 0 {
			if milestone, err := providers.GetManagementProvider().GetMilestone(n); err == nil {
				return milestone, nil
			}
		}
	}
	found, err := providers.GetManagementProvider().FindMilestoneByTitle(goal.Title)
	if err != nil {
		return nil, err
	}
	if found != nil {
		if _, err := updateGoalMilestone(goal, found.Number); err != nil {
			return nil, err
		}
		return found, nil
	}
	n, err := providers.GetManagementProvider().CreateMilestone(goal.Title, goal.Description)
	if err != nil {
		return nil, err
	}
	if goal.Dates.Due != "" {
		_ = providers.GetManagementProvider().UpdateMilestone(n, "", "", "", goal.Dates.Due)
	}
	if _, err := updateGoalMilestone(goal, n); err != nil {
		return nil, err
	}
	milestone, err := providers.GetManagementProvider().GetMilestone(n)
	if err != nil {
		return &providers.ManagementMilestone{Number: n, Title: goal.Title, Description: goal.Description}, nil
	}
	return milestone, nil
}

// ◼SyncManagement MUST return a non-nil error when the operation fails.
// 💖️SyncManagement performs the sync github operation on the repo context.
func (c *RepoContext) SyncManagement() (bool, error) {
	fmt.Println("Syncing local tickets and goals with GitHub...")

	technologies := codebase.LoadTechnologies()
	validLabels := make(map[string]bool)
	for _, p := range technologies {
		technologyLabel := "@" + strings.TrimPrefix(p.Name, "@")
		validLabels[technologyLabel] = true
		for _, b := range p.Bundles {
			validLabels[model.NormalizeBundleLabel(b.Name)] = true
		}
	}
	validLabels["repo"] = true
	if err := c.managementProvider.SyncRepoLabelCatalog(validLabels); err != nil {
		workspace.WriteWarningf("Failed to sync GitHub label catalog: %v", err)
	}

	goals, err := goalspkg.ListGoals()
	if err != nil {
		workspace.WriteWarningf("Failed to list goals: %v", err)
	} else {

		sort.Slice(goals, func(i, j int) bool {
			return goalspkg.GoalDepth(goals[i].ID) < goalspkg.GoalDepth(goals[j].ID)
		})
		for _, goal := range goals {
			if goalspkg.IsRootGoal(goal.ID) {

				if goal.Management != nil && goal.Management.Issue != "" {
					fmt.Printf("Migrating root goal %s: removing issue reference\n", goal.ID)
					goal.Management.Issue = ""
				}
				milestone, err := ensureGoalMilestone(goal)
				if err != nil {
					workspace.WriteWarningf("Failed to ensure milestone for root goal %s: %v", goal.ID, err)
				} else if milestone != nil {
					status := "open"
					if goal.Status == "closed" {
						status = "closed"
					}
					if err := c.managementProvider.UpdateMilestone(milestone.Number, goal.Title, goal.Description, status, goal.Dates.Due); err != nil {
						workspace.WriteWarningf("Failed to update milestone for root goal %s: %v", goal.ID, err)
					}
				}
			} else {

				if goal.Management != nil && goal.Management.Milestone != "" {

					fmt.Printf("Migrating child goal %s: removing milestone\n", goal.ID)
					number, err := model.ParseMilestoneNumber(goal.Management.Milestone)
					if err == nil {
						if err := c.managementProvider.DeleteMilestone(number); err != nil {
							workspace.WriteWarningf("Failed to delete milestone %d for child goal %s: %v", number, goal.ID, err)
						}
					}
					goal.Management.Milestone = ""
				}

				needsSave := false
				if goal.Management == nil || goal.Management.Issue == "" {

					var milestone *int
					if goalspkg.IsFirstGenGoal(goal.ID) {
						milestone, _ = goalspkg.GetRootGoalMilestone(goal.ID)
					}
					issueURL, err := c.managementProvider.CreateGoalIssue(goal.Title, goal.Description, milestone)
					if err != nil {
						workspace.WriteWarningf("Failed to create issue for child goal %s: %v", goal.ID, err)
						continue
					}
					if goal.Management == nil {
						goal.Management = &model.GoalManagementData{}
					}
					goal.Management.Issue = issueURL
					needsSave = true
					fmt.Printf("Created issue for child goal %s: %s\n", goal.ID, issueURL)

					if goal.Status == "closed" {
						_ = c.managementProvider.CloseIssue(issueURL)
					}

					if goalspkg.IsDeeperGoal(goal.ID) {
						parentID := goalspkg.GetParentGoalID(goal.ID)
						parentGoal, parentErr := goalspkg.ReadGoal(parentID)
						if parentErr == nil && parentGoal.Management != nil && parentGoal.Management.Issue != "" {
							if err := c.managementProvider.AddSubIssue(parentGoal.Management.Issue, issueURL); err != nil {
								workspace.WriteWarningf("Failed to link sub-issue %s to parent %s: %v", goal.ID, parentID, err)
							} else {
								fmt.Printf("Linked sub-issue %s to parent %s\n", goal.ID, parentID)
							}
						}
					}
				} else {

					remoteIssue, err := c.managementProvider.GetIssueDetails(goal.Management.Issue)
					if err != nil {
						workspace.WriteWarningf("Failed to get issue for goal %s: %v", goal.ID, err)
					} else {
						if goal.Status == "closed" && strings.ToUpper(remoteIssue.State) == "OPEN" {
							_ = c.managementProvider.CloseIssue(goal.Management.Issue)
						} else if goal.Status == "open" && strings.ToUpper(remoteIssue.State) == "CLOSED" {
							_ = c.managementProvider.ReopenIssue(goal.Management.Issue)
						}
						if goalspkg.IsFirstGenGoal(goal.ID) {
							milestone, _ := goalspkg.GetRootGoalMilestone(goal.ID)
							if milestone != nil {
								rootGoal, _ := goalspkg.ReadGoal(goalspkg.GetRootGoalID(goal.ID))
								if rootGoal != nil && (remoteIssue.Milestone == nil || remoteIssue.Milestone.Title != rootGoal.Title) {
									_ = c.managementProvider.UpdateIssueMilestone(goal.Management.Issue, rootGoal.Title)
								}
							}
						} else if goalspkg.IsDeeperGoal(goal.ID) {
							if remoteIssue.Milestone != nil {
								if err := c.managementProvider.ClearIssueMilestone(goal.Management.Issue); err != nil {
									workspace.WriteWarningf("Failed to clear milestone for goal %s: %v", goal.ID, err)
								}
							}
							parentID := goalspkg.GetParentGoalID(goal.ID)
							parentGoal, parentErr := goalspkg.ReadGoal(parentID)
							if parentErr == nil && parentGoal.Management != nil && parentGoal.Management.Issue != "" {
								parentURL, parentErr := c.managementProvider.GetIssueParentURL(goal.Management.Issue)
								if parentErr != nil {
									workspace.WriteWarningf("Failed to resolve parent for goal %s: %v", goal.ID, parentErr)
								} else if parentURL == "" || parentURL != parentGoal.Management.Issue {
									if err := c.managementProvider.AddSubIssue(parentGoal.Management.Issue, goal.Management.Issue); err != nil {
										workspace.WriteWarningf("Failed to link sub-issue %s to parent %s: %v", goal.ID, parentID, err)
									} else {
										fmt.Printf("Linked sub-issue %s to parent %s\n", goal.ID, parentID)
									}
								}
							}
						}
						hasGoalLabel := false
						for _, label := range remoteIssue.Labels {
							if label.Name == "goal" {
								hasGoalLabel = true
								break
							}
						}
						if !hasGoalLabel {
							if err := c.managementProvider.AddLabels(goal.Management.Issue, []string{"goal"}); err != nil {
								workspace.WriteWarningf("Failed to add goal label for %s: %v", goal.ID, err)
							}
						}
					}
				}

				if needsSave {
					if err := goalspkg.SaveGoal(*goal); err != nil {
						workspace.WriteWarningf("Failed to save goal %s: %v", goal.ID, err)
					}
				}
			}
		}
	}

	tickets, err := ticketspkg.ListTickets(nil, nil, nil)
	if err != nil {
		return false, err
	}

	for i := range tickets {
		t := &tickets[i]
		if t.Management == nil || t.Management.Issue == "" {
			continue
		}

		issueURL := t.Management.Issue

		remoteIssue, err := c.managementProvider.GetIssueDetails(issueURL)
		if err != nil {
			workspace.WriteWarningf("Failed to get GitHub issue %s: %v", issueURL, err)
			continue
		}

		if t.Status == model.TicketStatusClosed && strings.ToUpper(remoteIssue.State) == "OPEN" {
			fmt.Printf("Closing GitHub issue %s (Ticket is closed locally)\n", issueURL)
			if err := c.managementProvider.CloseIssue(issueURL); err != nil {
				workspace.WriteWarningf("Failed to close GitHub issue %s: %v", issueURL, err)
			}
		}

		if t.Goal != "" {
			rootID := goalspkg.GetRootGoalID(t.Goal)
			rootGoal, err := goalspkg.ReadGoal(rootID)
			if err == nil {
				milestone, err := ensureGoalMilestone(rootGoal)
				if err != nil {
					workspace.WriteWarningf("Failed to resolve goal milestone for issue %s: %v", issueURL, err)
				} else if milestone != nil && milestone.Title != "" {
					if remoteIssue.Milestone == nil || remoteIssue.Milestone.Title != milestone.Title {
						fmt.Printf("Updating milestone for issue %s to %s...\n", issueURL, milestone.Title)
						if err := c.managementProvider.UpdateIssueMilestone(issueURL, milestone.Title); err != nil {
							workspace.WriteWarningf("Failed to update milestone for GitHub issue %s: %v", issueURL, err)
						}
					}
				}
			}
		}

		var labelsToRemove []string
		for _, label := range remoteIssue.Labels {
			if strings.HasPrefix(label.Name, "@") {
				if !validLabels[label.Name] {
					labelsToRemove = append(labelsToRemove, label.Name)
				}
			}
		}
		if len(labelsToRemove) > 0 {
			fmt.Printf("Removing invalid technology labels from issue %s: %v\n", issueURL, labelsToRemove)
			if err := c.managementProvider.RemoveLabels(issueURL, labelsToRemove); err != nil {
				workspace.WriteWarningf("Failed to remove labels from GitHub issue %s: %v", issueURL, err)
			}
		}
	}

	issues, err := c.managementProvider.ListIssuesForLabelSync()
	if err != nil {
		workspace.WriteWarningf("Failed to list GitHub issues for label sync: %v", err)
	} else {
		for _, issue := range issues {
			var labelsToRemove []string
			for _, label := range issue.Labels {
				if strings.HasPrefix(label.Name, "@") && !validLabels[label.Name] {
					labelsToRemove = append(labelsToRemove, label.Name)
				}
			}
			if len(labelsToRemove) == 0 {
				continue
			}
			fmt.Printf("Removing invalid technology labels from issue %s: %v\n", issue.URL, labelsToRemove)
			if err := c.managementProvider.RemoveLabels(issue.URL, labelsToRemove); err != nil {
				workspace.WriteWarningf("Failed to remove labels from GitHub issue %s: %v", issue.URL, err)
			}
		}
	}

	fmt.Println("GitHub sync completed.")
	return true, nil
}

// 🌱️GetRootDir MUST retrieve the requested value or return an error.
// 💝️GetRootDir retrieves and returns the root dir.
func (c *defaultContext) GetRootDir() string { return c.rootDir }

// 🔴️GetBundles MUST retrieve the requested value or return an error.
// 💘️GetBundles retrieves and returns the bundles.
func (c *defaultContext) GetBundles() []*model.Bundle { return []*model.Bundle{} }

// 🟠️GetTechnologies MUST retrieve the requested value or return an error.
// 💕️GetTechnologies retrieves and returns the technologies.
func (c *defaultContext) GetTechnologies() []*model.Technology { return []*model.Technology{} }

// ✔️GetCheckpoints MUST retrieve the requested value or return an error.
// 🔢️GetCheckpoints retrieves and returns the checkpoints.
func (c *defaultContext) GetCheckpoints(limit *int) ([]*model.Checkpoint, error) {
	return []*model.Checkpoint{}, nil
}

// 🟡️GetFolders MUST retrieve the requested value or return an error.
// 🏵️GetFolders retrieves and returns the folders.
func (c *defaultContext) GetFolders() []*model.Folder { return []*model.Folder{} }

// 🟢️GetFiles MUST retrieve the requested value or return an error.
// 🌸️GetFiles retrieves and returns the files.
func (c *defaultContext) GetFiles() []*model.File { return []*model.File{} }

// 🟣️GetDefinitions MUST retrieve the requested value or return an error.
// 🌺️GetDefinitions retrieves and returns the definitions.
func (c *defaultContext) GetDefinitions() []*model.Definition { return []*model.Definition{} }

// 🟤️GetSections MUST retrieve the requested value or return an error.
// 🌻️GetSections retrieves and returns the sections.
func (c *defaultContext) GetSections() []*model.Section { return []*model.Section{} }

// ⚪️GetContributors MUST retrieve the requested value or return an error.
// 🌼️GetContributors retrieves and returns the contributors.
func (c *defaultContext) GetContributors() ([]*model.Contributor, error) {
	return []*model.Contributor{}, nil
}

// ⚫️GetTickets MUST retrieve the requested value or return an error.
// 🌷️GetTickets retrieves and returns the tickets.
func (c *defaultContext) GetTickets(year, month, day *int, status *model.TicketStatus) ([]*model.Ticket, error) {
	return []*model.Ticket{}, nil
}

// 🩵️GetPolicies MUST retrieve the requested value or return an error.
// 🌹️GetPolicies retrieves and returns the policies.
func (c *defaultContext) GetPolicies() []*model.Policy { return []*model.Policy{} }

// 🩶️GetStatutes MUST retrieve the requested value or return an error.
// 🥀️GetStatutes retrieves and returns the statutes.
func (c *defaultContext) GetStatutes() []*model.StatuteMeta { return []*model.StatuteMeta{} }

// ⛳️GetInteractions reports that a bare context carries no interactions.
func (c *defaultContext) GetInteractions() ([]model.InteractionResource, error) {
	return []model.InteractionResource{}, nil
}

// 🩷️Analyze MUST return a non-nil error when the operation fails.
// 🪻️Analyze performs the analyze operation on the default context.
func (c *defaultContext) Analyze(scope *string) (*model.AnalyzeResult, error) {
	return &model.AnalyzeResult{Breachs: []*model.Breach{}, Metrics: &model.AnalyzeMetrics{}}, nil
}

// 💙️TicketOpen MUST return a non-nil error when the operation fails.
// 🍁️TicketOpen performs the ticket open operation on the default context.
func (c *defaultContext) TicketOpen(input model.TicketOpenInput) (*model.Ticket, error) {
	return nil, nil
}

// 💚️TicketClose MUST return a non-nil error when the operation fails.
// 🍂️TicketClose performs the ticket close operation on the default context.
func (c *defaultContext) TicketClose(input model.TicketCloseInput) (*model.Ticket, error) {
	return nil, nil
}

// 💛️TicketReopen MUST return a non-nil error when the operation fails.
// 🍃️TicketReopen performs the ticket reopen operation on the default context.
func (c *defaultContext) TicketReopen(input model.TicketReopenInput) (*model.Ticket, error) {
	return nil, nil
}

// 🧡️TicketChange MUST return a non-nil error when the operation fails.
// ☘️TicketChange performs the ticket change operation on the default context.
func (c *defaultContext) TicketChange(input model.TicketChangeInput) (*model.Ticket, error) {
	return nil, nil
}

// ❤️FolderCreate MUST return a non-nil error when the operation fails.
// 🍀️FolderCreate performs the folder create operation on the default context.
func (c *defaultContext) FolderCreate(path string) (*model.Folder, error) { return nil, nil }

// 🤍️FolderMove MUST return a non-nil error when the operation fails.
// 🪴️FolderMove performs the folder move operation on the default context.
func (c *defaultContext) FolderMove(src, dst string) (*model.Folder, error) { return nil, nil }

// 🖤️FolderDelete MUST return a non-nil error when the operation fails.
// 🌱️FolderDelete performs the folder delete operation on the default context.
func (c *defaultContext) FolderDelete(path string) error { return nil }

// 🤎️FileCreate MUST return a non-nil error when the operation fails.
// 🌲️FileCreate performs the file create operation on the default context.
func (c *defaultContext) FileCreate(path string) (*model.File, error) { return nil, nil }

// 💗️FileMove MUST return a non-nil error when the operation fails.
// 🔖️FileMove performs the file move operation on the default context.
func (c *defaultContext) FileMove(src, dst string) (*model.File, error) { return nil, nil }

// 💖️FileDelete MUST return a non-nil error when the operation fails.
// 🔖️FileDelete performs the file delete operation on the default context.
func (c *defaultContext) FileDelete(path string) error { return nil }

// 💝️SectionCreate MUST return a non-nil error when the operation fails.
// 🔖️SectionCreate performs the section create operation on the default context.
func (c *defaultContext) SectionCreate(file, name string, parent *string) (*model.Section, error) {
	return nil, nil
}

// 💘️SectionMove MUST return a non-nil error when the operation fails.
// 🔖️SectionMove performs the section move operation on the default context.
func (c *defaultContext) SectionMove(file, oldName, newName string) (*model.Section, error) {
	return nil, nil
}

// 💕️SectionDelete MUST return a non-nil error when the operation fails.
// 🔖️SectionDelete performs the section delete operation on the default context.
func (c *defaultContext) SectionDelete(file, name string) error { return nil }

// 🔖️Integrate MUST return a non-nil error when the operation fails.
// 🔖️Integrate performs the integrate operation on the default context.
func (c *defaultContext) Integrate(source, targetSection, targetFile, targetParent *string) (*model.File, error) {
	return nil, nil
}

// 🔖️Extract MUST return the extracted component from the input.
// 🔖️Extract extracts the extract from the source.
func (c *defaultContext) Extract(sourceFile, sourceSection, targetFile *string) (*model.File, error) {
	return nil, nil
}

// 🔖️ContributorAdd MUST return a non-nil error when the operation fails.
// 🔖️ContributorAdd performs the contributor add operation on the default context.
func (c *defaultContext) ContributorAdd(input model.ContributorAddInput) (*model.Contributor, error) {
	return nil, nil
}

// 🔖️ContributorRemove MUST return a non-nil error when the operation fails.
// 🔖️ContributorRemove performs the contributor remove operation on the default context.
func (c *defaultContext) ContributorRemove(github string) error { return nil }

// 🔖️SyncManagement MUST return a non-nil error when the operation fails.
// 🔖️SyncManagement performs the sync github operation on the default context.
func (c *defaultContext) SyncManagement() (bool, error) { return false, nil }

// 🔖️GetGoals MUST retrieve the requested value or return an error.
// 🔖️GetGoals retrieves and returns the goals.
func (c *defaultContext) GetGoals() ([]*model.Goal, error) { return []*model.Goal{}, nil }

// 🔖️GoalCreate MUST return a non-nil error when the operation fails.
// 🔖️GoalCreate performs the goal create operation on the default context.
func (c *defaultContext) GoalCreate(input model.GoalCreateInput) (*model.Goal, error) {
	return nil, nil
}

// 🔖️GoalChange MUST return a non-nil error when the operation fails.
// 🔖️GoalChange performs the goal change operation on the default context.
func (c *defaultContext) GoalChange(input model.GoalChangeInput) (*model.Goal, error) {
	return nil, nil
}

// 🔖️GoalClose MUST return a non-nil error when the operation fails.
// 🔖️GoalClose performs the goal close operation on the default context.
func (c *defaultContext) GoalClose(input model.GoalCloseInput) (*model.Goal, error) { return nil, nil }

// 🔖️GoalReopen MUST return a non-nil error when the operation fails.
// 🔖️GoalReopen performs the goal reopen operation on the default context.
func (c *defaultContext) GoalReopen(input model.GoalReopenInput) (*model.Goal, error) {
	return nil, nil
}

// 🔖️GoalDelete MUST return a non-nil error when the operation fails.
// 🔖️GoalDelete performs the goal delete operation on the default context.
func (c *defaultContext) GoalDelete(input model.GoalDeleteInput) (bool, error) { return false, nil }

// 🔖️TicketDelete MUST return a non-nil error when the operation fails.
// 🔖️TicketDelete performs the ticket delete operation on the default context.
func (c *defaultContext) TicketDelete(input model.TicketDeleteInput) (bool, error) { return false, nil }

// 🔖️GetDrafts MUST retrieve the requested value or return an error.
// 🔖️GetDrafts retrieves and returns the drafts.
func (c *defaultContext) GetDrafts() ([]*model.Draft, error) { return []*model.Draft{}, nil }

// 🔖️DraftCreate MUST return a non-nil error when the operation fails.
// 🔖️DraftCreate performs the draft create operation on the default context.
func (c *defaultContext) DraftCreate(input model.DraftCreateInput) (*model.Draft, error) {
	return nil, nil
}

// 🔖️DraftDelete MUST return a non-nil error when the operation fails.
// 🔖️DraftDelete performs the draft delete operation on the default context.
func (c *defaultContext) DraftDelete(id string) (bool, error) { return false, nil }

// ✅️GetTodos MUST retrieve the requested value or return an error.
// 🔖️GetTodos retrieves and returns the todos.
func (c *defaultContext) GetTodos(filter *model.FilterInput) ([]*model.Todo, error) {
	return []*model.Todo{}, nil
}

// 🔖️TodoCreate MUST return a non-nil error when the operation fails.
// 🔖️TodoCreate performs the todo create operation on the default context.
func (c *defaultContext) TodoCreate(input model.TodoCreateInput) (*model.Todo, error) {
	return nil, nil
}

// 🔖️TodoChange MUST return a non-nil error when the operation fails.
// 🔖️TodoChange performs the todo change operation on the default context.
func (c *defaultContext) TodoChange(input model.TodoChangeInput) (*model.Todo, error) {
	return nil, nil
}

// 🔖️TodoDelete MUST return a non-nil error when the operation fails.
// 🔖️TodoDelete performs the todo delete operation on the default context.
func (c *defaultContext) TodoDelete(id string) (bool, error) { return false, nil }

// #endregion 🩻️Default Context

// #region 🧱️GraphQL Executor

/** 🧱️ One executable schema bound to one repository context. */
type Executor struct {
	resolver *Resolver
	schema   Schema
}

/** 🔷️ Binds the repo schema to a filesystem-backed context rooted at one directory. */
func NewExecutor(rootDir string) (*Executor, error) {
	return NewExecutorWithContext(rootDir, NewRepoContext(rootDir))
}

/** 🔷️ Binds the repo schema to a context. */
func NewExecutorWithContext(rootDir string, ctx model.RepoContext) (*Executor, error) {
	resolver := NewResolverWithContext(rootDir, ctx)
	schema, err := buildSchema(resolver)
	if err != nil {
		return nil, err
	}
	return &Executor{resolver: resolver, schema: schema}, nil
}

/** 🗺️ The schema this executor serves. */
func (e *Executor) Schema() Schema { return e.schema }

/** ⚡️ Parses and executes one request, returning the `data` payload. */
func (e *Executor) Execute(ctx context.Context, query string, variables map[string]interface{}) (interface{}, error) {
	result := Do(Params{
		Context:        ctx,
		Schema:         e.schema,
		Repo:           e.resolver.context(),
		RequestString:  query,
		VariableValues: variables,
	})
	if len(result.Errors) > 0 {
		return nil, fmt.Errorf("graphql errors: [%s]", result.Errors[0].Error())
	}
	return result.Data, nil
}

/** 📋️ Executes one request and renders the `data` payload the way the CLI prints it. */
func (e *Executor) ExecuteJSON(ctx context.Context, query string, variables map[string]interface{}) (string, error) {
	data, err := e.Execute(ctx, query, variables)
	if err != nil {
		return "", err
	}
	jsonBytes, err := json.MarshalIndent(data, "", "  ")
	if err != nil {
		return "", err
	}
	return string(jsonBytes), nil
}

/** 🔍️ Reports whether a request string is a document this grammar accepts. */
func (e *Executor) ValidateQuery(query string) error {
	return Validate(query)
}

/** 📨️ The operation kind of a request string, without executing it. */
func (e *Executor) GetOperationType(query string) (string, error) {
	return OperationType(query)
}

// #endregion 🧱️GraphQL Executor

// #region 🏗️SchemaCommon

/** 🔢️ Declares one enum type. */
func enumType(name string, values ...[2]string) NamedType {
	members := make([]EnumValue, 0, len(values))
	for _, value := range values {
		members = append(members, EnumValue{Name: value[0], Wire: value[1]})
	}
	return &EnumType{Name: name, Values: members}
}

/** 🔢️ The enum types the repo schema declares. */
func schemaEnums() []NamedType {
	return []NamedType{
		enumType("DefinitionKind", [2]string{"IMPLEMENTATION", "implementation"}, [2]string{"INTERFACE", "interface"}, [2]string{"CONSTANT", "constant"}),
		enumType("BundleKind", [2]string{"LIBRARY", "library"}, [2]string{"SCHEMA", "schema"}, [2]string{"BINARY", "binary"}, [2]string{"UI", "ui"}, [2]string{"SITE", "site"}, [2]string{"ASSETS", "assets"}, [2]string{"REPO", "repo"}),
		enumType("FolderKind", [2]string{"ORGANIZATION", "organization"}, [2]string{"REQUIRED", "required"}),
		enumType("TicketStatus", [2]string{"OPEN", "open"}, [2]string{"CLOSED", "closed"}),
		enumType("TicketClient",
			[2]string{"COPILOT_CHAT", "copilot-chat"}, [2]string{"WINDSURF", "windsurf"}, [2]string{"WINDSURF_CHAT", "windsurf-chat"},
			[2]string{"ANTIGRAVITY", "antigravity"}, [2]string{"ANTIGRAVITY_CHAT", "antigravity-chat"}, [2]string{"CURSOR", "cursor"},
			[2]string{"CURSOR_CHAT", "cursor-chat"}, [2]string{"VSCODE", "vscode"}, [2]string{"CLAUDE_CODE", "claude-code"},
			[2]string{"CODEX", "codex"}, [2]string{"DROID", "droid"}, [2]string{"KIRO_CLI", "kiro-cli"}),
		enumType("BreachPriority", [2]string{"HIGH", "high"}, [2]string{"MEDIUM", "medium"}, [2]string{"LOW", "low"}),
		enumType("FileKind",
			[2]string{"CODE", "code"}, [2]string{"SCRIPT", "script"}, [2]string{"CONFIG", "config"}, [2]string{"LAB", "lab"},
			[2]string{"DOCS", "docs"}, [2]string{"RESOURCE", "resource"}, [2]string{"TEMPLATE", "template"}, [2]string{"LICENSE", "license"}),
	}
}

/** 🔢️ The metric objects every aggregate shares. */
func schemaMetrics() []NamedType {
	return []NamedType{
		&ObjectType{Name: "Range", Fields: []Field{NewField("start", NN(Ty("Int"))), NewField("end", NN(Ty("Int")))}},
		&ObjectType{Name: "CountMetrics", Fields: []Field{NewField("added", NN(Ty("Int"))), NewField("updated", NN(Ty("Int"))), NewField("removed", NN(Ty("Int")))}},
		&ObjectType{Name: "PriorityCount", Fields: []Field{NewField("high", NN(Ty("Int"))), NewField("medium", NN(Ty("Int"))), NewField("low", NN(Ty("Int")))}},
		&ObjectType{Name: "AnalyzeMetrics", Fields: []Field{NewField("total", NN(Ty("Int"))), NewField("byPriority", Ty("PriorityCount")), NewField("autofixable", NN(Ty("Int")))}},
		&ObjectType{Name: "AnalyzeResult", Fields: []Field{NewField("breachs", NNList(Ty("Breach"))), NewField("metrics", NN(Ty("AnalyzeMetrics")))}},
	}
}

// #endregion 🏗️SchemaCommon

// #region 🏗️SchemaRepo

/** 💠️ The repository root type: every aggregate reachable from one query. */
func schemaRepo() []NamedType {
	return []NamedType{&ObjectType{Name: "Repo", Fields: []Field{
		NewField("id", NN(Ty("ID"))),
		NewField("name", NN(Ty("String"))),
		NewField("path", NN(Ty("String"))),
		ResolvedField("technologies", NNList(Ty("Technology")), "technologies"),
		ResolvedField("checkpoints", NNList(Ty("Checkpoint")), "checkpoints").Arg("limit", Ty("Int")),
		ResolvedField("bundles", NNList(Ty("Bundle")), "bundles"),
		ResolvedField("folders", NNList(Ty("Folder")), "folders"),
		ResolvedField("files", NNList(Ty("File")), "files"),
		ResolvedField("sections", NNList(Ty("Section")), "sections"),
		ResolvedField("definitions", NNList(Ty("Definition")), "definitions"),
		ResolvedField("contributors", NNList(Ty("Contributor")), "contributors"),
		ResolvedField("goals", NNList(Ty("Goal")), "goals"),
		ResolvedField("tickets", NNList(Ty("Ticket")), "tickets").Arg("year", Ty("Int")).Arg("month", Ty("Int")).Arg("day", Ty("Int")).Arg("status", Ty("TicketStatus")),
		ResolvedField("policies", NNList(Ty("Policy")), "policies"),
		ResolvedField("statutes", NNList(Ty("Statute")), "statutes"),
		ResolvedField("breachs", NNList(Ty("Breach")), "breachs").Arg("scope", Ty("String")),
	}}}
}

// #endregion 🏗️SchemaRepo

// #region 🏗️SchemaTechnology

/** 📜️ The technology type and the package it publishes. */
func schemaTechnology() []NamedType {
	return []NamedType{
		&ObjectType{Name: "Technology", Fields: []Field{
			NewField("id", NN(Ty("ID"))),
			NewField("name", NN(Ty("String"))),
			NewField("root", NN(Ty("String"))),
			NewField("kind", NN(Ty("String"))),
			NewField("bundles", NNList(Ty("Bundle"))),
			NewField("uri", NN(Ty("String"))),
		}},
		&ObjectType{Name: "Package", Fields: []Field{
			NewField("name", NN(Ty("String"))),
			NewField("version", NN(Ty("String"))),
			NewField("path", NN(Ty("String"))),
			NewField("kind", NN(Ty("String"))),
		}},
	}
}

// #endregion 🏗️SchemaTechnology

// #region 🏗️SchemaBundle

/** 🔵️ The bundle type. */
func schemaBundle() []NamedType {
	return []NamedType{&ObjectType{Name: "Bundle", Fields: []Field{
		NewField("id", NN(Ty("ID"))),
		NewField("name", NN(Ty("String"))),
		NewField("root", NN(Ty("String"))),
		NewField("sourceRoot", Ty("String")),
		NewField("projectType", Ty("String")),
		NewField("tags", NNList(Ty("String"))),
		NewField("packages", NNList(Ty("Package"))),
		NewField("kind", NN(Ty("String"))),
		NewField("uri", NN(Ty("String"))),
		ResolvedField("folders", NNList(Ty("Folder")), "empty-list"),
		ResolvedField("files", NNList(Ty("File")), "empty-list"),
		ResolvedField("breachs", NNList(Ty("Breach")), "empty-list"),
	}}}
}

// #endregion 🏗️SchemaBundle

// #region 🏗️SchemaFolder

/** 🩷️ The folder type. */
func schemaFolder() []NamedType {
	return []NamedType{&ObjectType{Name: "Folder", Fields: []Field{
		NewField("id", NN(Ty("ID"))),
		NewField("path", NN(Ty("String"))),
		NewField("uri", NN(Ty("String"))),
		NewField("name", NN(Ty("String"))),
		NewField("kind", NN(Ty("String"))),
		NewField("parent", Ty("Folder")),
		ResolvedField("children", NNList(Ty("Folder")), "folder.children"),
		ResolvedField("files", NNList(Ty("File")), "folder.files"),
		NewField("ignored", NN(Ty("Boolean"))),
		NewField("generated", NN(Ty("Boolean"))),
		NewField("bundle", Ty("Bundle")),
		ResolvedField("breachs", NNList(Ty("Breach")), "empty-list"),
	}}}
}

// #endregion 🏗️SchemaFolder

// #region 🏗️SchemaFile

/** 📄️ The file type. */
func schemaFile() []NamedType {
	return []NamedType{&ObjectType{Name: "File", Fields: []Field{
		NewField("id", NN(Ty("ID"))),
		NewField("path", NN(Ty("String"))),
		NewField("uri", NN(Ty("String"))),
		NewField("name", NN(Ty("String"))),
		NewField("extension", NN(Ty("String"))),
		NewField("folder", Ty("Folder")),
		NewField("kind", NN(Ty("String"))),
		NewField("ignored", NN(Ty("Boolean"))),
		NewField("generated", NN(Ty("Boolean"))),
		NewField("bundle", Ty("Bundle")),
		ResolvedField("sections", Lst(Ty("Section")), "file.sections"),
		ResolvedField("definitions", Lst(Ty("Definition")), "file.definitions"),
		ResolvedField("breachs", Lst(Ty("Breach")), "empty-list"),
		NewField("content", Ty("String")),
		ResolvedField("contributors", NNList(Ty("Contributor")), "empty-list"),
	}}}
}

// #endregion 🏗️SchemaFile

// #region 🏗️SchemaSection

/** 💗️ The section type and the abstract item a section contains. */
func schemaSection() []NamedType {
	return []NamedType{
		&InterfaceType{Name: "SectionItem", Fields: []Field{NewField("id", NN(Ty("ID"))), NewField("name", NN(Ty("String"))), NewField("range", Ty("Range"))}},
		&ObjectType{Name: "Section", Interfaces: []string{"SectionItem"}, Fields: []Field{
			NewField("id", NN(Ty("ID"))),
			NewField("name", NN(Ty("String"))),
			NewField("path", NN(Ty("String"))),
			ResolvedField("file", Ty("File"), "item.file"),
			NewField("parent", Ty("Section")),
			ResolvedField("children", Lst(Ty("SectionItem")), "section.children"),
			NewField("definitions", Lst(Ty("Definition"))),
			ResolvedField("breachs", Lst(Ty("Breach")), "empty-list"),
			NewField("range", NN(Ty("Range"))),
		}},
	}
}

// #endregion 🏗️SchemaSection

// #region 🏗️SchemaDefinition

/** 💕️ The definition type. */
func schemaDefinition() []NamedType {
	return []NamedType{&ObjectType{Name: "Definition", Interfaces: []string{"SectionItem"}, Fields: []Field{
		NewField("id", NN(Ty("ID"))),
		NewField("name", NN(Ty("String"))),
		NewField("kind", NN(Ty("DefinitionKind"))),
		ResolvedField("file", NN(Ty("File")), "item.file"),
		ResolvedField("section", Ty("Section"), "definition.section"),
		ResolvedField("breachs", NNList(Ty("Breach")), "empty-list"),
		NewField("range", NN(Ty("Range"))),
	}}}
}

// #endregion 🏗️SchemaDefinition

// #region 🏗️SchemaStatute

/** 📜️ The statute type. */
func schemaStatute() []NamedType {
	return []NamedType{&ObjectType{Name: "Statute", Fields: []Field{
		NewField("id", NN(Ty("ID"))),
		ResolvedField("policy", NN(Ty("Policy")), "statute.policy"),
		NewField("priority", NN(Ty("BreachPriority"))),
		NewField("autofixable", NN(Ty("Boolean"))),
		NewField("reason", NN(Ty("String"))),
		NewField("solution", NN(Ty("String"))),
		NewField("breachs", NNList(Ty("Breach"))).Arg("scope", Ty("String")),
	}}}
}

// #endregion 🏗️SchemaStatute

// #region 🏗️SchemaBreach

/** 🔶️ The breach type. */
func schemaBreach() []NamedType {
	return []NamedType{&ObjectType{Name: "Breach", Fields: []Field{
		NewField("id", NN(Ty("ID"))),
		NewField("kindId", NN(Ty("ID"))),
		NewField("kind", NN(Ty("Statute"))),
		NewField("scope", NN(Ty("String"))),
		NewField("file", Ty("File")),
		NewField("folder", Ty("Folder")),
		NewField("line", Ty("Int")),
		NewField("column", Ty("Int")),
		NewField("excerpt", Ty("String")),
		NewField("summary", NN(Ty("String"))),
		NewField("priority", NN(Ty("BreachPriority"))),
		NewField("autofixable", NN(Ty("Boolean"))),
	}}}
}

// #endregion 🏗️SchemaBreach

// #region 🏗️SchemaPolicy

/** 👮️ The policy and territory types. */
func schemaPolicy() []NamedType {
	return []NamedType{
		&ObjectType{Name: "Territory", Fields: []Field{
			NewField("name", NN(Ty("String"))),
			NewField("description", NN(Ty("String"))),
			NewField("scopes", NNList(Ty("String"))),
			NewField("groups", NNList(Ty("Territory"))),
			NewField("kinds", NNList(Ty("Statute"))),
		}},
		&ObjectType{Name: "Policy", Fields: []Field{
			NewField("id", NN(Ty("ID"))),
			NewField("name", NN(Ty("String"))),
			NewField("description", Ty("String")),
			NewField("scopes", NNList(Ty("String"))),
			NewField("groups", NNList(Ty("Territory"))),
			NewField("statutes", NNList(Ty("Statute"))),
		}},
	}
}

// #endregion 🏗️SchemaPolicy

// #region 🏗️SchemaInteraction

/** 💬️ The interaction types. */
func schemaInteraction() []NamedType {
	return []NamedType{
		&ObjectType{Name: "Interaction", Fields: []Field{
			NewField("kind", NN(Ty("String"))),
			NewField("prompt", NN(Ty("String"))),
			NewField("checkpoint", Ty("String")),
			NewField("llm", Ty("String")),
			NewField("effort", Ty("String")),
			NewField("date", NN(Ty("String"))),
			NewField("system", NN(Ty("String"))),
			NewField("client", NN(Ty("String"))),
			NewField("author", NN(Ty("String"))),
		}},
		&ObjectType{Name: "InteractionResource", Fields: []Field{
			NewField("kind", NN(Ty("String"))),
			NewField("prompt", NN(Ty("String"))),
			NewField("checkpoint", Ty("String")),
			NewField("author", NN(Ty("String"))),
			NewField("sourceKind", NN(Ty("String"))),
			NewField("sourceId", NN(Ty("String"))),
			NewField("goalId", Ty("String")),
			NewField("ticketId", Ty("String")),
			NewField("llm", Ty("String")),
			NewField("effort", Ty("String")),
			NewField("date", NN(Ty("String"))),
			NewField("system", NN(Ty("String"))),
			NewField("client", NN(Ty("String"))),
		}},
	}
}

// #endregion 🏗️SchemaInteraction

// #region 🏗️SchemaGoal

/** 🎯️ The goal type. */
func schemaGoal() []NamedType {
	return []NamedType{&ObjectType{Name: "Goal", Fields: []Field{
		NewField("id", NN(Ty("ID"))),
		NewField("title", NN(Ty("String"))),
		NewField("description", Ty("String")),
		NewField("prompt", Ty("String")),
		NewField("dueDate", Ty("String")),
		NewField("createdAt", Ty("String")),
		NewField("client", Ty("String")),
		NewField("llm", Ty("String")),
		NewField("effort", Ty("String")),
		NewField("status", NN(Ty("String"))),
		NewField("milestone", Ty("Int")),
		NewField("issue", Ty("String")),
		NewField("parent", Ty("String")),
		NewField("interactions", NNList(Ty("Interaction"))),
	}}}
}

// #endregion 🏗️SchemaGoal

// #region 🏗️SchemaDraft

/** 📝️ The draft type. */
func schemaDraft() []NamedType {
	return []NamedType{&ObjectType{Name: "Draft", Fields: []Field{NewField("id", NN(Ty("ID")))}}}
}

// #endregion 🏗️SchemaDraft

// #region 🏗️SchemaTodo

/** ✅️ The todo type and the position it points at. */
func schemaTodo() []NamedType {
	return []NamedType{
		&ObjectType{Name: "Location", Fields: []Field{NewField("filePath", Ty("String")), NewField("line", Ty("Int")), NewField("column", Ty("Int"))}},
		&ObjectType{Name: "Todo", Fields: []Field{
			NewField("id", NN(Ty("ID"))),
			NewField("name", NN(Ty("String"))),
			NewField("description", Ty("String")),
			NewField("parentId", NN(Ty("ID"))),
			NewField("location", Ty("Location")),
		}},
	}
}

// #endregion 🏗️SchemaTodo

// #region 🏗️SchemaTicket

/** 🎫️ The ticket type and the date pair it carries. */
func schemaTicket() []NamedType {
	return []NamedType{
		&ObjectType{Name: "TicketDate", Fields: []Field{NewField("started", NN(Ty("DateTime"))), NewField("finished", Ty("DateTime"))}},
		&ObjectType{Name: "Ticket", Fields: []Field{
			NewField("id", NN(Ty("ID"))),
			NewField("year", NN(Ty("Int"))),
			NewField("month", NN(Ty("Int"))),
			NewField("day", NN(Ty("Int"))),
			NewField("slug", NN(Ty("String"))),
			NewField("path", NN(Ty("String"))),
			NewField("llm", Ty("String")),
			NewField("effort", Ty("String")),
			NewField("client", Ty("TicketClient")),
			NewField("checkpoint", Ty("String")),
			NewField("uri", NN(Ty("String"))),
			NewField("title", NN(Ty("String"))),
			NewField("emoji", Ty("String")),
			NewField("prompt", NN(Ty("String"))),
			NewField("summary", Ty("String")),
			NewField("status", NN(Ty("TicketStatus"))),
			NewField("interactions", NNList(Ty("Interaction"))),
			ResolvedField("author", Ty("Contributor"), "ticket.author"),
			NewField("dates", NN(Ty("TicketDate"))),
			NewField("goal", Ty("String")),
			NewField("parent", Ty("String")),
			NewField("bundles", NNList(Ty("Bundle"))),
			NewField("files", NNList(Ty("File"))),
		}},
		&ObjectType{Name: "TicketDay", Fields: []Field{NewField("day", NN(Ty("Int"))), NewField("tickets", NNList(Ty("Ticket")))}},
		&ObjectType{Name: "TicketMonth", Fields: []Field{NewField("month", NN(Ty("Int"))), NewField("days", NNList(Ty("TicketDay")))}},
		&ObjectType{Name: "TicketYear", Fields: []Field{NewField("year", NN(Ty("Int"))), NewField("months", NNList(Ty("TicketMonth")))}},
	}
}

// #endregion 🏗️SchemaTicket

// #region 🏗️SchemaCheckpoint

/** ✔️ The checkpoint type. */
func schemaCheckpoint() []NamedType {
	return []NamedType{&ObjectType{Name: "Checkpoint", Fields: []Field{
		NewField("id", NN(Ty("ID"))),
		NewField("sha", NN(Ty("String"))),
		NewField("title", NN(Ty("String"))),
		NewField("date", NN(Ty("DateTime"))),
	}}}
}

// #endregion 🏗️SchemaCheckpoint

// #region 🏗️SchemaContributor

/** 🧑️‍💻️ The contributor type and its contribution tree. */
func schemaContributor() []NamedType {
	named := func(name string, child string, plural string) NamedType {
		return &ObjectType{Name: name, Fields: []Field{NewField("name", NN(Ty("String"))), NewField(plural, NNList(Ty(child)))}}
	}
	return []NamedType{
		&ObjectType{Name: "ContributorIcons", Fields: []Field{NewField("avatar", Ty("String")), NewField("avatarRound", Ty("String")), NewField("github", Ty("String"))}},
		&ObjectType{Name: "ContributorLink", Fields: []Field{NewField("name", NN(Ty("String"))), NewField("url", NN(Ty("String")))}},
		&ObjectType{Name: "ContributorDefinition", Fields: []Field{NewField("name", NN(Ty("String")))}},
		named("ContributorSection", "ContributorDefinition", "definitions"),
		named("ContributorFile", "ContributorSection", "sections"),
		named("ContributorFolder", "ContributorFile", "files"),
		named("ContributorBundle", "ContributorFolder", "folders"),
		&ObjectType{Name: "ContributorContributions", Fields: []Field{
			NewField("checkpoints", NNList(Ty("Checkpoint"))),
			NewField("tickets", NNList(Ty("TicketYear"))),
			NewField("bundles", NNList(Ty("ContributorBundle"))),
		}},
		&ObjectType{Name: "Contributor", Fields: []Field{
			NewField("id", NN(Ty("ID"))),
			NewField("github", NN(Ty("String"))),
			NewField("emoji", Ty("String")),
			NewField("name", NN(Ty("String"))),
			NewField("names", NNList(Ty("String"))),
			NewField("email", NN(Ty("String"))),
			NewField("emails", NNList(Ty("String"))),
			NewField("fingerprint", Ty("String")),
			NewField("fingerprints", NNList(Ty("String"))),
			NewField("links", NNList(Ty("ContributorLink"))),
			ResolvedField("contributions", Ty("ContributorContributions"), "contributor.contributions"),
			NewField("icons", Ty("ContributorIcons")),
			NewField("bundles", NNList(Ty("Bundle"))),
			NewField("files", NNList(Ty("File"))),
			NewField("tickets", NNList(Ty("Ticket"))),
		}},
	}
}

// #endregion 🏗️SchemaContributor

// #region 🏗️SchemaInputs

/** 📥️ Declares one input object type. */
func inputType(name string, fields ...InputField) NamedType {
	return &InputObjectType{Name: name, Fields: fields}
}

/** 📥️ Declares one input field. */
func inputField(name string, kind TypeRef) InputField { return InputField{Name: name, Type: kind} }

/** 📥️ Every input object the query and mutation roots accept. */
func schemaInputs() []NamedType {
	return []NamedType{
		inputType("FilterInput",
			inputField("filter", Ty("String")), inputField("regex", Ty("Boolean")), inputField("matchCase", Ty("Boolean")),
			inputField("matchWholeWord", Ty("Boolean")), inputField("showIgnored", Ty("Boolean")), inputField("showGenerated", Ty("Boolean")),
			inputField("excludeKinds", Lst(Ty("FileKind"))), inputField("includeKinds", Lst(Ty("FileKind")))),
		inputType("DraftCreateInput", inputField("title", NN(Ty("String"))), inputField("files", Lst(NN(Ty("String"))))),
		inputType("TicketOpenInput",
			inputField("emoji", NN(Ty("String"))), inputField("title", NN(Ty("String"))), inputField("prompt", NN(Ty("String"))),
			inputField("llm", Ty("String")), inputField("effort", Ty("String")), inputField("client", NN(Ty("TicketClient"))),
			inputField("noIssue", Ty("Boolean")), inputField("draft", Ty("String")), inputField("goal", NN(Ty("String"))),
			inputField("parent", Ty("String")), inputField("noManagement", Ty("Boolean")), inputField("issue", Ty("String")),
			inputField("planId", Ty("String")), inputField("specId", Ty("String"))),
		inputType("TicketCloseInput",
			inputField("year", Ty("Int")), inputField("month", Ty("Int")), inputField("day", Ty("Int")), inputField("slug", Ty("String")),
			inputField("summary", Ty("String")), inputField("files", Lst(NN(Ty("String")))), inputField("title", Ty("String")),
			inputField("noManagement", Ty("Boolean")), inputField("all", Ty("Boolean"))),
		inputType("TicketReopenInput",
			inputField("year", NN(Ty("Int"))), inputField("month", NN(Ty("Int"))), inputField("day", NN(Ty("Int"))), inputField("slug", NN(Ty("String"))),
			inputField("prompt", NN(Ty("String"))), inputField("client", NN(Ty("TicketClient"))), inputField("llm", Ty("String")),
			inputField("effort", Ty("String")), inputField("title", Ty("String")), inputField("draft", Ty("String")),
			inputField("goal", Ty("String")), inputField("parent", Ty("String")), inputField("noManagement", Ty("Boolean")),
			inputField("planId", Ty("String")), inputField("specId", Ty("String"))),
		inputType("TicketChangeInput",
			inputField("year", NN(Ty("Int"))), inputField("month", NN(Ty("Int"))), inputField("day", NN(Ty("Int"))), inputField("slug", NN(Ty("String"))),
			inputField("title", Ty("String")), inputField("prompt", Ty("String")), inputField("llm", Ty("String")), inputField("effort", Ty("String")),
			inputField("client", Ty("TicketClient")), inputField("goal", Ty("String")), inputField("parent", Ty("String")), inputField("noManagement", Ty("Boolean"))),
		inputType("TodoCreateInput", inputField("name", NN(Ty("String"))), inputField("description", NN(Ty("String"))), inputField("parentId", NN(Ty("String")))),
		inputType("TodoChangeInput", inputField("id", NN(Ty("String"))), inputField("name", Ty("String")), inputField("description", Ty("String"))),
		inputType("GoalCreateInput",
			inputField("title", NN(Ty("String"))), inputField("description", NN(Ty("String"))), inputField("prompt", NN(Ty("String"))),
			inputField("dueDate", NN(Ty("String"))), inputField("llm", NN(Ty("String"))), inputField("effort", Ty("String")),
			inputField("client", NN(Ty("String"))), inputField("parent", Ty("String")), inputField("noManagement", Ty("Boolean")), inputField("milestone", Ty("String"))),
		inputType("GoalChangeInput",
			inputField("id", NN(Ty("String"))), inputField("title", Ty("String")), inputField("description", Ty("String")),
			inputField("dueDate", Ty("String")), inputField("llm", Ty("String")), inputField("effort", Ty("String")),
			inputField("parent", Ty("String")), inputField("noManagement", Ty("Boolean"))),
		inputType("GoalCloseInput", inputField("id", NN(Ty("String"))), inputField("summary", NN(Ty("String"))), inputField("noManagement", Ty("Boolean"))),
		inputType("GoalReopenInput",
			inputField("id", NN(Ty("String"))), inputField("prompt", NN(Ty("String"))), inputField("client", NN(Ty("String"))),
			inputField("llm", NN(Ty("String"))), inputField("effort", Ty("String")), inputField("title", Ty("String")),
			inputField("description", Ty("String")), inputField("dueDate", Ty("String")), inputField("parent", Ty("String")), inputField("noManagement", Ty("Boolean"))),
		inputType("ContributorAddInput",
			inputField("github", NN(Ty("String"))), inputField("name", Ty("String")), inputField("names", Lst(NN(Ty("String")))),
			inputField("email", Ty("String")), inputField("emails", Lst(NN(Ty("String")))), inputField("fingerprint", Ty("String")),
			inputField("fingerprints", Lst(NN(Ty("String"))))),
	}
}

// #endregion 🏗️SchemaInputs

// #region 🏗️SchemaRoots

/** 🔍️ The query root and the node union it discriminates. */
func schemaQuery() []NamedType {
	return []NamedType{
		&UnionType{Name: "Node", Types: []string{"Repo", "Bundle", "Folder", "File", "Section", "Definition", "Contributor", "Ticket", "Policy", "Statute", "Breach", "Draft"}},
		&ObjectType{Name: "Query", Fields: []Field{
			ResolvedField("node", NN(Ty("Node")), "query.node").Arg("id", NN(Ty("ID"))),
			ResolvedField("repo", NN(Ty("Repo")), "query.repo"),
			ResolvedField("technologies", NNList(Ty("Technology")), "technologies").Arg("filter", Ty("FilterInput")),
			ResolvedField("bundles", NNList(Ty("Bundle")), "bundles").Arg("filter", Ty("FilterInput")),
			ResolvedField("folders", NNList(Ty("Folder")), "folders"),
			ResolvedField("files", NNList(Ty("File")), "files"),
			ResolvedField("sections", NNList(Ty("Section")), "sections"),
			ResolvedField("definitions", NNList(Ty("Definition")), "definitions"),
			ResolvedField("contributors", NNList(Ty("Contributor")), "contributors").Arg("filter", Ty("FilterInput")),
			ResolvedField("todos", NNList(Ty("Todo")), "query.todos").Arg("filter", Ty("FilterInput")),
			ResolvedField("tickets", NNList(Ty("Ticket")), "tickets").Arg("year", Ty("Int")).Arg("month", Ty("Int")).Arg("day", Ty("Int")).Arg("status", Ty("TicketStatus")).Arg("filter", Ty("FilterInput")),
			ResolvedField("interactions", NNList(Ty("InteractionResource")), "query.interactions"),
			ResolvedField("drafts", NNList(Ty("Draft")), "query.drafts"),
			ResolvedField("policies", NNList(Ty("Policy")), "policies").Arg("filter", Ty("FilterInput")),
			ResolvedField("statutes", NNList(Ty("Statute")), "statutes"),
			ResolvedField("breachs", NNList(Ty("Breach")), "breachs").Arg("scope", Ty("String")),
			ResolvedField("bundle", Ty("Bundle"), "query.bundle").Arg("name", NN(Ty("String"))),
			ResolvedField("folder", Ty("Folder"), "query.folder").Arg("path", NN(Ty("String"))),
			ResolvedField("file", Ty("File"), "query.file").Arg("path", NN(Ty("String"))),
			ResolvedField("section", Ty("Section"), "query.section").Arg("path", NN(Ty("String"))).Arg("sectionPath", NNList(Ty("String"))),
			ResolvedField("definition", Ty("Definition"), "query.definition").Arg("path", NN(Ty("String"))).Arg("name", NN(Ty("String"))),
			ResolvedField("contributor", Ty("Contributor"), "query.contributor").Arg("id", NN(Ty("String"))),
			ResolvedField("ticket", Ty("Ticket"), "query.ticket").Arg("year", NN(Ty("Int"))).Arg("month", NN(Ty("Int"))).Arg("day", NN(Ty("Int"))).Arg("slug", NN(Ty("String"))),
			ResolvedField("policy", Ty("Policy"), "query.policy").Arg("id", NN(Ty("String"))),
			ResolvedField("statute", Ty("Statute"), "query.statute").Arg("id", NN(Ty("String"))),
			ResolvedField("analyze", NN(Ty("AnalyzeResult")), "query.analyze").Arg("scope", Ty("String")),
		}},
	}
}

/** ✍️ The mutation root. */
func schemaMutation() []NamedType {
	return []NamedType{&ObjectType{Name: "Mutation", Fields: []Field{
		ResolvedField("syncManagement", NN(Ty("Boolean")), "mutation.syncManagement"),
		ResolvedField("goalCreate", Ty("Goal"), "mutation.goalCreate").Arg("input", NN(Ty("GoalCreateInput"))),
		ResolvedField("goalChange", Ty("Goal"), "mutation.goalChange").Arg("id", NN(Ty("ID"))).Arg("input", NN(Ty("GoalChangeInput"))),
		ResolvedField("goalClose", Ty("Goal"), "mutation.goalClose").Arg("input", NN(Ty("GoalCloseInput"))),
		ResolvedField("goalReopen", Ty("Goal"), "mutation.goalReopen").Arg("input", NN(Ty("GoalReopenInput"))),
		ResolvedField("draftCreate", Ty("Draft"), "mutation.draftCreate").Arg("input", NN(Ty("DraftCreateInput"))),
		ResolvedField("draftDelete", NN(Ty("Boolean")), "mutation.draftDelete").Arg("id", NN(Ty("String"))),
		ResolvedField("todoCreate", Ty("Todo"), "mutation.todoCreate").Arg("input", NN(Ty("TodoCreateInput"))),
		ResolvedField("todoChange", Ty("Todo"), "mutation.todoChange").Arg("input", NN(Ty("TodoChangeInput"))),
		ResolvedField("todoDelete", Ty("Boolean"), "mutation.todoDelete").Arg("id", NN(Ty("ID"))),
		ResolvedField("ticketOpen", Ty("Ticket"), "mutation.ticketOpen").Arg("input", NN(Ty("TicketOpenInput"))),
		ResolvedField("ticketClose", Ty("Ticket"), "mutation.ticketClose").Arg("input", NN(Ty("TicketCloseInput"))),
		ResolvedField("ticketReopen", Ty("Ticket"), "mutation.ticketReopen").Arg("input", NN(Ty("TicketReopenInput"))),
		ResolvedField("ticketChange", Ty("Ticket"), "mutation.ticketChange").Arg("input", NN(Ty("TicketChangeInput"))),
		ResolvedField("contributorAdd", Ty("Contributor"), "mutation.contributorAdd").Arg("input", NN(Ty("ContributorAddInput"))),
		ResolvedField("contributorRemove", NN(Ty("Boolean")), "mutation.contributorRemove").Arg("github", NN(Ty("String"))),
		ResolvedField("folderCreate", Ty("Folder"), "mutation.folderCreate").Arg("path", NN(Ty("String"))),
		ResolvedField("folderMove", Ty("Folder"), "mutation.folderMove").Arg("src", NN(Ty("String"))).Arg("dst", NN(Ty("String"))),
		ResolvedField("folderDelete", NN(Ty("Boolean")), "mutation.folderDelete").Arg("path", NN(Ty("String"))),
		ResolvedField("fileCreate", Ty("File"), "mutation.fileCreate").Arg("path", NN(Ty("String"))),
		ResolvedField("fileMove", Ty("File"), "mutation.fileMove").Arg("src", NN(Ty("String"))).Arg("dst", NN(Ty("String"))),
		ResolvedField("fileDelete", NN(Ty("Boolean")), "mutation.fileDelete").Arg("path", NN(Ty("String"))),
		ResolvedField("sectionCreate", Ty("Section"), "mutation.sectionCreate").Arg("file", NN(Ty("String"))).Arg("name", NN(Ty("String"))).Arg("parent", Ty("String")),
		ResolvedField("sectionMove", Ty("Section"), "mutation.sectionMove").Arg("file", NN(Ty("String"))).Arg("oldName", NN(Ty("String"))).Arg("newName", NN(Ty("String"))),
		ResolvedField("sectionDelete", NN(Ty("Boolean")), "mutation.sectionDelete").Arg("file", NN(Ty("String"))).Arg("name", NN(Ty("String"))),
		ResolvedField("integrate", Ty("File"), "mutation.integrate").Arg("source", NN(Ty("String"))).Arg("targetSection", NN(Ty("String"))).Arg("targetFile", NN(Ty("String"))).Arg("targetParent", Ty("String")),
		ResolvedField("extract", Ty("File"), "mutation.extract").Arg("sourceFile", NN(Ty("String"))).Arg("sourceSection", NN(Ty("String"))).Arg("targetFile", NN(Ty("String"))),
	}}}
}

/** 🗺️ Assembles the executable schema from every per-aggregate fragment. */
func BuildSchema() Schema {
	types := map[string]NamedType{}
	for _, scalar := range Scalars {
		types[scalar] = &ScalarType{Name: scalar}
	}
	fragments := [][]NamedType{
		schemaEnums(), schemaMetrics(), schemaRepo(), schemaTechnology(), schemaBundle(), schemaFolder(), schemaFile(),
		schemaSection(), schemaDefinition(), schemaStatute(), schemaBreach(), schemaPolicy(), schemaInteraction(),
		schemaGoal(), schemaDraft(), schemaTodo(), schemaTicket(), schemaCheckpoint(), schemaContributor(),
		schemaInputs(), schemaQuery(), schemaMutation(),
	}
	for _, fragment := range fragments {
		for _, declared := range fragment {
			types[declared.TypeName()] = declared
		}
	}
	return Schema{Types: types, Query: "Query", Mutation: "Mutation"}
}

/** 🗺️ Builds the schema one resolver serves, registering it as the process-wide instance. */
func buildSchema(resolver *Resolver) (Schema, error) {
	RepoResolverInstance = resolver
	return BuildSchema(), nil
}

// #endregion 🏗️SchemaRoots

// #region 📜️Sdl

/** 📜️ Collects every type reachable from the operation roots. */
func reachable(schema Schema) []string {
	pending := []string{schema.Query}
	if schema.Mutation != "" {
		pending = append(pending, schema.Mutation)
	}
	seen := []string{}
	for len(pending) > 0 {
		name := pending[len(pending)-1]
		pending = pending[:len(pending)-1]
		if slices.Contains(seen, name) {
			continue
		}
		seen = append(seen, name)
		pushField := func(field Field) {
			pending = append(pending, field.Type.TypeName())
			for _, argument := range field.Args {
				pending = append(pending, argument.Type.TypeName())
			}
		}
		switch declared := schema.Types[name].(type) {
		case *ObjectType:
			pending = append(pending, declared.Interfaces...)
			for _, field := range declared.Fields {
				pushField(field)
			}
		case *InterfaceType:
			for _, field := range declared.Fields {
				pushField(field)
			}
		case *UnionType:
			pending = append(pending, declared.Types...)
		case *InputObjectType:
			for _, field := range declared.Fields {
				pending = append(pending, field.Type.TypeName())
			}
		}
	}
	sort.Strings(seen)
	return seen
}

/** 📜️ Renders one field in SDL. */
func renderSDLField(field Field) string {
	arguments := ""
	if len(field.Args) > 0 {
		rendered := make([]string, 0, len(field.Args))
		for _, argument := range field.Args {
			rendered = append(rendered, argument.Name+": "+argument.Type.Render())
		}
		arguments = "(" + strings.Join(rendered, ", ") + ")"
	}
	return "  " + field.Name + arguments + ": " + field.Type.Render()
}

/** 📜️ Whether a scalar is one of the four GraphQL built-ins the SDL never declares. */
func builtinScalar(name string) bool {
	return name == "String" || name == "Int" || name == "Boolean" || name == "ID"
}

/** 📜️ Renders the schema as the SDL both implementations must serve. */
func RenderSDL(schema Schema) string {
	var out strings.Builder
	out.WriteString("schema {\n")
	out.WriteString("  query: " + schema.Query + "\n")
	if schema.Mutation != "" {
		out.WriteString("  mutation: " + schema.Mutation + "\n")
	}
	out.WriteString("}\n")
	for _, name := range reachable(schema) {
		declared, found := schema.Types[name]
		if !found {
			continue
		}
		out.WriteString("\n")
		switch typed := declared.(type) {
		case *ScalarType:
			if !builtinScalar(typed.Name) {
				out.WriteString("scalar " + typed.Name + "\n")
			}
		case *ObjectType:
			implements := ""
			if len(typed.Interfaces) > 0 {
				implements = " implements " + strings.Join(typed.Interfaces, " & ")
			}
			out.WriteString("type " + typed.Name + implements + " {\n")
			for _, field := range typed.Fields {
				out.WriteString(renderSDLField(field) + "\n")
			}
			out.WriteString("}\n")
		case *InterfaceType:
			out.WriteString("interface " + typed.Name + " {\n")
			for _, field := range typed.Fields {
				out.WriteString(renderSDLField(field) + "\n")
			}
			out.WriteString("}\n")
		case *UnionType:
			out.WriteString("union " + typed.Name + " = " + strings.Join(typed.Types, " | ") + "\n")
		case *EnumType:
			out.WriteString("enum " + typed.Name + " {\n")
			for _, member := range typed.Values {
				out.WriteString("  " + member.Name + "\n")
			}
			out.WriteString("}\n")
		case *InputObjectType:
			out.WriteString("input " + typed.Name + " {\n")
			for _, field := range typed.Fields {
				out.WriteString("  " + field.Name + ": " + field.Type.Render() + "\n")
			}
			out.WriteString("}\n")
		}
	}
	return strings.TrimRight(out.String(), "\n") + "\n"
}

/** 📜️ The language-neutral inventory of the served schema: what a conforming reader finds in the
 * SDL, in the one order both implementations can agree on. */
func SchemaInventory(schema Schema) interface{} {
	fieldRow := func(field Field) interface{} {
		args := make([]interface{}, 0, len(field.Args))
		for _, argument := range field.Args {
			args = append(args, map[string]interface{}{"name": argument.Name, "type": argument.Type.Render()})
		}
		return map[string]interface{}{"name": field.Name, "type": field.Type.Render(), "args": args}
	}
	fieldRows := func(fields []Field) []interface{} {
		rows := make([]interface{}, 0, len(fields))
		for _, field := range fields {
			rows = append(rows, fieldRow(field))
		}
		return rows
	}
	types := make([]interface{}, 0)
	for _, name := range reachable(schema) {
		declared, found := schema.Types[name]
		if !found {
			continue
		}
		switch typed := declared.(type) {
		case *ScalarType:
			if builtinScalar(typed.Name) {
				continue
			}
			types = append(types, map[string]interface{}{"name": typed.Name, "kind": "scalar"})
		case *ObjectType:
			interfaces := make([]interface{}, 0, len(typed.Interfaces))
			for _, name := range typed.Interfaces {
				interfaces = append(interfaces, name)
			}
			types = append(types, map[string]interface{}{"name": typed.Name, "kind": "object", "interfaces": interfaces, "fields": fieldRows(typed.Fields)})
		case *InterfaceType:
			types = append(types, map[string]interface{}{"name": typed.Name, "kind": "interface", "fields": fieldRows(typed.Fields)})
		case *UnionType:
			members := make([]interface{}, 0, len(typed.Types))
			for _, name := range typed.Types {
				members = append(members, name)
			}
			types = append(types, map[string]interface{}{"name": typed.Name, "kind": "union", "possibleTypes": members})
		case *EnumType:
			values := make([]interface{}, 0, len(typed.Values))
			for _, member := range typed.Values {
				values = append(values, member.Name)
			}
			types = append(types, map[string]interface{}{"name": typed.Name, "kind": "enum", "values": values})
		case *InputObjectType:
			fields := make([]interface{}, 0, len(typed.Fields))
			for _, field := range typed.Fields {
				fields = append(fields, map[string]interface{}{"name": field.Name, "type": field.Type.Render()})
			}
			types = append(types, map[string]interface{}{"name": typed.Name, "kind": "input", "fields": fields})
		}
	}
	var mutation interface{}
	if schema.Mutation != "" {
		mutation = schema.Mutation
	}
	return map[string]interface{}{"query": schema.Query, "mutation": mutation, "types": types}
}

// #endregion 📜️Sdl

// #region 🖼️Sources

/** 🖼️ Serialises a domain value into the JSON the executor reads fields off. */
func toJSON(value interface{}) interface{} {
	encoded, err := json.Marshal(value)
	if err != nil {
		return nil
	}
	var decoded interface{}
	if json.Unmarshal(encoded, &decoded) != nil {
		return nil
	}
	return decoded
}

/** 🎫️ The ticket record without the domain type's own inference, so a fixture round-trips verbatim. */
type plainTicket model.Ticket

/** 🎯️ The goal record without the domain type's own inference, so a fixture round-trips verbatim. */
type plainGoal model.Goal

/** 🎫️ The object body of one ticket, read off its declared fields alone. */
func ticketBody(ticket *model.Ticket) map[string]interface{} { return body((*plainTicket)(ticket)) }

/** 🎯️ The object body of one goal, read off its declared fields alone. */
func goalBody(goal *model.Goal) map[string]interface{} { return body((*plainGoal)(goal)) }

/** 🖼️ The object body of a serialised domain value, empty when it is not an object. */
func body(value interface{}) map[string]interface{} {
	if fields, ok := toJSON(value).(map[string]interface{}); ok {
		return fields
	}
	return map[string]interface{}{}
}

/** 🏷️ Stamps the concrete type name an interface or union member is discriminated by. */
func tagged(fields map[string]interface{}, typename string) interface{} {
	fields["__typename"] = typename
	return fields
}

/** 🏷️ The emoji one entity kind is registered under, with its presentation selector normalised. */
func entityEmoji(kind string) string {
	switch kind {
	case "technology-user":
		return model.EmojiTechnologyUser
	case "technology-infrastructure":
		return model.EmojiTechnologyInfra
	case "technology-research":
		return model.EmojiTechnologyResearch
	case "technology-mono":
		return model.EmojiTechnologyMono
	case "bundle-library":
		return model.EmojiBundleLibrary
	case "bundle-schema":
		return model.EmojiBundleSchema
	case "bundle-binary":
		return model.EmojiBundleBinary
	case "bundle-ui":
		return model.EmojiBundleUI
	case "bundle-site":
		return model.EmojiBundleSite
	case "bundle-assets":
		return model.EmojiBundleAssets
	case "bundle-repo":
		return model.EmojiBundleRepo
	case "folder-organization":
		return model.EmojiFolderOrg
	case "folder-required":
		return model.EmojiFolderRequired
	case "folder-root":
		return model.EmojiFolderRoot
	case "file-code":
		return model.EmojiFileCode
	case "file-lab":
		return model.EmojiFileLab
	case "file-script":
		return model.EmojiFileScript
	case "file-docs":
		return model.EmojiFileDocs
	case "file-config":
		return model.EmojiFileConfig
	case "file-resource":
		return model.EmojiFileResource
	case "file-template":
		return model.EmojiFileTemplate
	case "file-license":
		return model.EmojiFileLicense
	case "definition-impl":
		return model.EmojiDefinitionImpl
	case "definition-interface":
		return model.EmojiDefinitionInterface
	case "definition-constant":
		return model.EmojiDefinitionConstant
	case "definition-test":
		return model.EmojiDefinitionTest
	case "section":
		return model.EmojiSection
	case "ticket":
		return model.EmojiTicket
	case "goal":
		return model.EmojiGoal
	case "draft":
		return model.EmojiDraft
	case "todo":
		return model.EmojiTodo
	case "policy":
		return model.EmojiPolicy
	case "breach":
		return model.EmojiBreach
	case "contributor":
		return model.EmojiContributor
	case "checkpoint":
		return model.EmojiCheckpoint
	case "statute":
		return model.EmojiStatute
	default:
		return ""
	}
}

/** 📁️ Joins a repository-relative path onto the root, always with forward slashes. */
func fileURI(rootDir string, relative string) string {
	root := strings.ReplaceAll(rootDir, "\\", "/")
	tail := strings.ReplaceAll(relative, "\\", "/")
	if tail == "" {
		return "file://" + root
	}
	return "file://" + strings.TrimRight(root, "/") + "/" + strings.TrimLeft(tail, "/")
}

/** 🔤️ Title-cases one dash separated slug, word by word. */
func titleizeSlug(slug string) string {
	words := strings.Split(slug, "-")
	titled := make([]string, 0, len(words))
	for _, word := range words {
		if word == "" {
			titled = append(titled, "")
			continue
		}
		runes := []rune(word)
		titled = append(titled, strings.ToUpper(string(runes[0]))+strings.ToLower(string(runes[1:])))
	}
	return strings.Join(titled, " ")
}

/** 📜️ The identifier value of a statute path: every segment titleized, joined with `#`. */
func statutePathToIDValue(path string) string {
	segments := strings.Split(path, "/")
	titled := make([]string, 0, len(segments))
	for _, segment := range segments {
		titled = append(titled, titleizeSlug(segment))
	}
	return strings.Join(titled, "#")
}

/** 🏷️ The emoji a technology name derives when the record carries none. */
func technologyEmoji(name string) string {
	if strings.Contains(name, "repo") {
		return entityEmoji("technology-infrastructure")
	}
	if strings.HasPrefix(name, "coda") {
		return entityEmoji("technology-research")
	}
	return ""
}

/** 📜️ The identifier of a technology. */
func technologyID(technology *model.Technology) string {
	emoji := technology.Emoji
	if emoji == "" {
		emoji = technologyEmoji(technology.Name)
	}
	return model.EmojiText(emoji) + workspace.Flat(technology.Name)
}

/** 🔵️ The emoji a bundle kind stands for. */
func bundleKindEmoji(kind model.BundleKind) string {
	switch kind {
	case model.BundleKindLibrary:
		return entityEmoji("bundle-library")
	case model.BundleKindSchema:
		return entityEmoji("bundle-schema")
	case model.BundleKindBinary:
		return entityEmoji("bundle-binary")
	case model.BundleKindUI:
		return entityEmoji("bundle-ui")
	case model.BundleKindSite:
		return entityEmoji("bundle-site")
	case model.BundleKindAssets:
		return entityEmoji("bundle-assets")
	case model.BundleKindRepo:
		return entityEmoji("bundle-repo")
	default:
		return ""
	}
}

/** 🔵️ The identifier of a bundle: technology emoji and code, then bundle emoji and code. */
func bundleID(bundle *model.Bundle, technologies []*model.Technology) string {
	emoji := bundle.Emoji
	if emoji == "" {
		emoji = bundleKindEmoji(bundle.Kind)
	}
	technologyCode, bundleCode := bundle.Name, bundle.Name
	if head, tail, found := strings.Cut(bundle.Name, "/"); found {
		technologyCode, bundleCode = head, tail
	}
	technologyEmojiValue := string(model.DeriveTechnologyKind(technologyCode))
	for _, candidate := range technologies {
		if candidate.Name == technologyCode && candidate.Emoji != "" {
			technologyEmojiValue = candidate.Emoji
			break
		}
	}
	return model.EmojiText(technologyEmojiValue) + workspace.Flat(technologyCode) + model.EmojiText(emoji) + workspace.Flat(bundleCode)
}

/** 💗️ The identifier of a section, falling back to the emoji form when it carries none. */
func sectionID(section *model.Section) string {
	if section.ID != "" {
		return section.ID
	}
	emoji := section.Emoji
	if emoji == "" {
		emoji = entityEmoji("section")
	}
	return model.EmojiText(emoji) + workspace.Flat(section.Name)
}

/** 💕️ The emoji a definition kind stands for. */
func definitionKindEmojiOf(kind model.DefinitionKind) string {
	switch kind {
	case model.DefinitionKindImplementation:
		return entityEmoji("definition-impl")
	case model.DefinitionKindInterface:
		return entityEmoji("definition-interface")
	case model.DefinitionKindConstant:
		return entityEmoji("definition-constant")
	case model.DefinitionKindTest:
		return entityEmoji("definition-test")
	default:
		return ""
	}
}

/** 💕️ The identifier of a definition, falling back to the emoji form when it carries none. */
func definitionID(definition *model.Definition) string {
	if definition.ID != "" {
		return definition.ID
	}
	return model.EmojiText(definitionKindEmojiOf(definition.Kind)) + workspace.Flat(definition.Name)
}

/** 🕰️ Normalises a recorded timestamp into the RFC 3339 rendering the `DateTime` scalar emits. */
func toRFC3339(raw string) (string, bool) {
	trimmed := strings.TrimSpace(raw)
	if len(trimmed) < 19 || (trimmed[10] != ' ' && trimmed[10] != 'T') {
		return "", false
	}
	date, rest := trimmed[:10], trimmed[11:]
	clock := rest
	if index := strings.IndexAny(rest, "+Z."); index >= 0 {
		clock = rest[:index]
	}
	if len(clock) != 8 {
		return "", false
	}
	return date + "T" + clock + "Z", true
}

/** 🕰️ Midnight UTC of a year/month/day triple, the zero date a ticket falls back to. */
func midnight(year, month, day int) string {
	return fmt.Sprintf("%04d-%02d-%02dT00:00:00Z", year, month, day)
}

/** 🎫️ Whether an interaction kind is the expected one, tolerating the `.ended` suffix. */
func isTicketInteractionKind(kind string, expected string) bool {
	trimmed := strings.TrimSpace(kind)
	want := strings.TrimSpace(expected)
	return trimmed == want || strings.TrimSuffix(trimmed, ".ended") == want && strings.HasSuffix(trimmed, ".ended")
}

/** 🎫️ The date a ticket started: its open interaction, else its first, else midnight. */
func ticketStarted(ticket *model.Ticket) string {
	for index := range ticket.Interactions {
		if !isTicketInteractionKind(ticket.Interactions[index].Kind, "ticket.open") {
			continue
		}
		if date, ok := toRFC3339(ticket.Interactions[index].Date); ok {
			return date
		}
	}
	if len(ticket.Interactions) > 0 {
		if date, ok := toRFC3339(ticket.Interactions[0].Date); ok {
			return date
		}
	}
	return midnight(ticket.Year, ticket.Month, ticket.Day)
}

/** 🎫️ The date a ticket finished: its last close interaction, or none. */
func ticketFinished(ticket *model.Ticket) interface{} {
	for index := len(ticket.Interactions) - 1; index >= 0; index-- {
		if !isTicketInteractionKind(ticket.Interactions[index].Kind, "ticket.close") {
			continue
		}
		if date, ok := toRFC3339(ticket.Interactions[index].Date); ok {
			return date
		}
		return nil
	}
	return nil
}

/** 🎫️ The last interaction's field, then the last agent's, then blank. */
func ticketLatest(ticket *model.Ticket, pick func(*model.Interaction) string, fallback func(*model.TicketAgent) string) string {
	if len(ticket.Interactions) > 0 {
		return pick(&ticket.Interactions[len(ticket.Interactions)-1])
	}
	if len(ticket.Agents) > 0 {
		return fallback(&ticket.Agents[len(ticket.Agents)-1])
	}
	return ""
}

/** 🎫️ The author of a ticket: its first interaction's, then its first agent's. */
func ticketAuthor(ticket *model.Ticket) string {
	if len(ticket.Interactions) > 0 {
		return ticket.Interactions[0].Author
	}
	if len(ticket.Agents) > 0 {
		return ticket.Agents[0].Contributor
	}
	return ""
}

/** 🎫️ The checkpoint of a ticket: its first interaction's. */
func ticketCheckpoint(ticket *model.Ticket) string {
	if len(ticket.Interactions) > 0 {
		return ticket.Interactions[0].Checkpoint
	}
	return ""
}

/** 🔤️ Wraps a non-empty string as JSON, mapping the empty string to null. */
func optional(value string) interface{} {
	if value == "" {
		return nil
	}
	return value
}

/** 🤝️ Splits `Name <email>` the way the reference implementation does. */
func parseGitAuthor(value string) (string, string) {
	name, rest, found := strings.Cut(value, " <")
	if !found {
		return value, ""
	}
	return strings.TrimSpace(name), strings.TrimSuffix(rest, ">")
}

/** 🟡️ The milestone number a management link carries. */
func parseMilestoneNumber(milestone string) interface{} {
	if number, err := strconv.Atoi(milestone); err == nil {
		return number
	}
	segments := strings.Split(milestone, "/")
	if number, err := strconv.Atoi(segments[len(segments)-1]); err == nil {
		return number
	}
	return nil
}

/** 📃️ Wraps a string slice as a JSON array, never as null. */
func stringArray(values []string) []interface{} {
	rows := make([]interface{}, 0, len(values))
	for _, value := range values {
		rows = append(rows, value)
	}
	return rows
}

/** 📜️ The technology source object. */
func technologySource(technology *model.Technology, technologies []*model.Technology, rootDir string) interface{} {
	fields := body(technology)
	id := technologyID(technology)
	fields["uri"] = "repo://technology/" + id
	fields["id"] = id
	fields["kind"] = string(technology.Kind)
	rows := make([]interface{}, 0, len(technology.Bundles))
	for index := range technology.Bundles {
		rows = append(rows, bundleSource(&technology.Bundles[index], technologies, rootDir))
	}
	fields["bundles"] = rows
	return tagged(fields, "Technology")
}

/** 🔵️ The bundle source object. */
func bundleSource(bundle *model.Bundle, technologies []*model.Technology, rootDir string) interface{} {
	fields := body(bundle)
	fields["id"] = bundleID(bundle, technologies)
	fields["uri"] = fileURI(rootDir, bundle.Root)
	fields["kind"] = string(bundle.Kind)
	fields["tags"] = stringArray(bundle.Tags)
	packages := make([]interface{}, 0, len(bundle.Packages))
	for index := range bundle.Packages {
		packages = append(packages, tagged(body(&bundle.Packages[index]), "Package"))
	}
	fields["packages"] = packages
	return tagged(fields, "Bundle")
}

/** 🩷️ The folder source object. */
func folderSource(folder *model.Folder) interface{} {
	fields := body(folder)
	fields["kind"] = string(folder.Kind)
	return tagged(fields, "Folder")
}

/** 📄️ The file source object. */
func fileSource(file *model.File) interface{} {
	return tagged(body(file), "File")
}

/** 📐️ The line range of a start/end pair. */
func rangeSource(start int, end int) interface{} {
	return tagged(body(&model.Range{Start: start, End: end}), "Range")
}

/** 💗️ The section source object, recursing through children and definitions. */
func sectionSource(section *model.Section) interface{} {
	fields := body(section)
	fields["id"] = sectionID(section)
	fields["range"] = rangeSource(section.StartLine, section.EndLine)
	children := make([]interface{}, 0, len(section.Children))
	for index := range section.Children {
		children = append(children, sectionSource(&section.Children[index]))
	}
	fields["children"] = children
	definitions := make([]interface{}, 0, len(section.Definitions))
	for index := range section.Definitions {
		definitions = append(definitions, definitionSource(&section.Definitions[index]))
	}
	fields["definitions"] = definitions
	return tagged(fields, "Section")
}

/** 💕️ The definition source object. */
func definitionSource(definition *model.Definition) interface{} {
	fields := body(definition)
	fields["id"] = definitionID(definition)
	fields["kind"] = string(definition.Kind)
	fields["range"] = rangeSource(definition.StartLine, definition.EndLine)
	return tagged(fields, "Definition")
}

/** 📜️ The declared metadata of one statute, or the reference implementation's unknown default. */
func statuteMetaOf(kind model.Statute, statutes []*model.StatuteMeta) model.StatuteMeta {
	for _, candidate := range statutes {
		if candidate != nil && candidate.Kind == kind {
			return *candidate
		}
	}
	return model.StatuteMeta{Kind: kind, Priority: model.BreachPriorityLow, Reason: "Unknown breach", Solution: "Fix the breach"}
}

/** 📜️ The statute source object. */
func statuteSource(meta *model.StatuteMeta) interface{} {
	fields := body(meta)
	fields["id"] = model.EmojiText(entityEmoji("statute")) + statutePathToIDValue(string(meta.Kind))
	fields["priority"] = string(meta.Priority)
	return tagged(fields, "Statute")
}

/** 🔶️ The breach source object, with the statute metadata overlaid by the breach's own overrides. */
func breachSource(breach *model.Breach, statutes []*model.StatuteMeta) interface{} {
	meta := statuteMetaOf(breach.Kind, statutes)
	if breach.LintPriority != "" {
		meta.Priority = breach.LintPriority
	}
	if breach.LintAutofixable != nil {
		meta.Autofixable = *breach.LintAutofixable
	}
	if breach.Reason != "" {
		meta.Reason = breach.Reason
	}
	if breach.Solution != "" {
		meta.Solution = breach.Solution
	}
	fields := body(breach)
	fields["id"] = model.EmojiText(entityEmoji("breach")) + breach.ID
	fields["kindId"] = string(breach.Kind)
	fields["kind"] = statuteSource(&meta)
	fields["priority"] = string(meta.Priority)
	fields["autofixable"] = meta.Autofixable
	fields["line"] = breach.Line
	fields["column"] = breach.Column
	fields["excerpt"] = optional(breach.Excerpt)
	return tagged(fields, "Breach")
}

/** 🟣️ The territory source object. */
func territorySource(territory *model.Territory, statutes []*model.StatuteMeta) interface{} {
	fields := body(territory)
	groups := make([]interface{}, 0, len(territory.Groups))
	for index := range territory.Groups {
		groups = append(groups, territorySource(&territory.Groups[index], statutes))
	}
	fields["groups"] = groups
	kinds := make([]interface{}, 0, len(territory.Kinds))
	for _, kind := range territory.Kinds {
		meta := statuteMetaOf(kind, statutes)
		kinds = append(kinds, statuteSource(&meta))
	}
	fields["kinds"] = kinds
	return tagged(fields, "Territory")
}

/** 👮️ The policy source object. */
func policySource(policy *model.Policy, statutes []*model.StatuteMeta) interface{} {
	fields := body(policy)
	fields["scopes"] = stringArray(policy.Scopes)
	groups := make([]interface{}, 0, len(policy.Groups))
	for index := range policy.Groups {
		groups = append(groups, territorySource(&policy.Groups[index], statutes))
	}
	fields["groups"] = groups
	declared := make([]interface{}, 0, len(policy.Statutes))
	for _, meta := range policy.Statutes {
		if meta == nil {
			continue
		}
		declared = append(declared, statuteSource(meta))
	}
	fields["statutes"] = declared
	return tagged(fields, "Policy")
}

/** 💬️ The interaction source object. */
func interactionSource(interaction *model.Interaction) map[string]interface{} {
	fields := body(interaction)
	fields["prompt"] = interaction.Prompt
	fields["summary"] = interaction.Summary
	fields["llm"] = interaction.LLM
	fields["effort"] = interaction.Effort
	fields["__typename"] = "Interaction"
	return fields
}

/** 🎁️ The flattened interaction-resource source object. */
func interactionResourceSource(resource *model.InteractionResource) interface{} {
	fields := interactionSource(&resource.Interaction)
	fields["goalId"] = resource.GoalID
	fields["ticketId"] = resource.TicketID
	fields["sourceKind"] = resource.SourceKind
	fields["sourceId"] = resource.SourceID
	return tagged(fields, "InteractionResource")
}

/** 🎫️ The ticket source object, with every derived field the reference resolvers compute. */
func ticketSource(ticket *model.Ticket, rootDir string) interface{} {
	fields := ticketBody(ticket)
	fields["id"] = model.EmojiText(entityEmoji("ticket")) + workspace.Flat(ticket.Slug)
	fields["year"] = ticket.Year
	fields["month"] = ticket.Month
	fields["day"] = ticket.Day
	fields["slug"] = ticket.Slug
	path := ticket.FolderPath
	if path == "" {
		path = ticket.JsonPath
	}
	fields["path"] = workspace.NormalizePath(path)
	fields["uri"] = fileURI(rootDir, ticket.FolderPath)
	fields["title"] = ticket.Title
	fields["emoji"] = optional(ticket.Emoji)
	prompt := ticket.Description
	if prompt == "" && len(ticket.Interactions) > 0 {
		prompt = ticket.Interactions[0].Prompt
	}
	fields["prompt"] = prompt
	fields["summary"] = nil
	fields["status"] = string(ticket.Status)
	fields["llm"] = optional(ticketLatest(ticket, func(value *model.Interaction) string { return value.LLM }, func(value *model.TicketAgent) string { return value.LLM }))
	fields["effort"] = optional(ticketLatest(ticket, func(value *model.Interaction) string { return value.Effort }, func(value *model.TicketAgent) string { return value.Effort }))
	fields["client"] = optional(ticketLatest(ticket, func(value *model.Interaction) string { return value.Client }, func(value *model.TicketAgent) string { return value.Client }))
	fields["checkpoint"] = optional(ticketCheckpoint(ticket))
	fields["goal"] = optional(ticket.Goal)
	fields["parent"] = optional(ticket.Parent)
	interactions := make([]interface{}, 0, len(ticket.Interactions))
	for index := range ticket.Interactions {
		interactions = append(interactions, interactionSource(&ticket.Interactions[index]))
	}
	fields["interactions"] = interactions
	fields["dates"] = tagged(map[string]interface{}{"started": ticketStarted(ticket), "finished": ticketFinished(ticket)}, "TicketDate")
	return tagged(fields, "Ticket")
}

/** 🎯️ The goal source object. */
func goalSource(goal *model.Goal) interface{} {
	fields := goalBody(goal)
	fields["id"] = goal.ID
	fields["dueDate"] = optional(goal.Dates.Due)
	fields["createdAt"] = nil
	fields["status"] = goal.Status
	management := model.GoalManagementData{}
	if goal.Management != nil {
		management = *goal.Management
	}
	fields["milestone"] = parseMilestoneNumber(management.Milestone)
	fields["issue"] = optional(management.Issue)
	fields["parent"] = optional(goal.Parent)
	fields["interactions"] = []interface{}{}
	return tagged(fields, "Goal")
}

/** 📝️ The draft source object. */
func draftSource(draft *model.Draft) interface{} {
	return tagged(body(draft), "Draft")
}

/** ✅️ The todo source object. */
func todoSource(todo *model.Todo) interface{} {
	fields := body(todo)
	fields["id"] = model.EmojiText(entityEmoji("todo")) + workspace.Flat(todo.ID)
	fields["description"] = optional(todo.Description)
	if todo.Location == nil {
		fields["location"] = nil
	} else {
		fields["location"] = tagged(body(todo.Location), "Location")
	}
	return tagged(fields, "Todo")
}

/** ✔️ The checkpoint source object. */
func checkpointSource(checkpoint *model.Checkpoint) interface{} {
	return tagged(body(checkpoint), "Checkpoint")
}

/** 🧑️‍💻️ The contributor source object. */
func contributorSource(contributor *model.Contributor) interface{} {
	fields := body(contributor)
	fields["id"] = model.EmojiText(entityEmoji("contributor")) + workspace.Flat(contributor.Alias)
	names := make([]string, 0, len(contributor.Links))
	for name := range contributor.Links {
		names = append(names, name)
	}
	sort.Strings(names)
	links := make([]interface{}, 0, len(names))
	for _, name := range names {
		links = append(links, tagged(body(&model.ContributorLink{Name: name, URL: contributor.Links[name]}), "ContributorLink"))
	}
	fields["links"] = links
	fields["emoji"] = optional(contributor.Emoji)
	fields["fingerprint"] = optional(contributor.Fingerprint)
	fields["names"] = stringArray(contributor.Names)
	fields["emails"] = stringArray(contributor.Emails)
	fields["fingerprints"] = stringArray(contributor.Fingerprints)
	fields["githubs"] = stringArray(contributor.Githubs)
	fields["aliases"] = stringArray(contributor.Aliases)
	fields["icons"] = nil
	return tagged(fields, "Contributor")
}

/** 🔬️ The analyze-result source object. */
func analyzeResultSource(result *model.AnalyzeResult, statutes []*model.StatuteMeta) interface{} {
	breachs := make([]interface{}, 0)
	if result != nil {
		for _, breach := range result.Breachs {
			if breach == nil {
				continue
			}
			breachs = append(breachs, breachSource(breach, statutes))
		}
	}
	metrics := model.AnalyzeMetrics{}
	if result != nil && result.Metrics != nil {
		metrics = *result.Metrics
	}
	priority := model.PriorityCount{}
	if metrics.ByPriority != nil {
		priority = *metrics.ByPriority
	}
	metricFields := map[string]interface{}{
		"total":       metrics.Total,
		"autofixable": metrics.Autofixable,
		"byPriority":  tagged(body(&priority), "PriorityCount"),
	}
	return tagged(map[string]interface{}{"breachs": breachs, "metrics": tagged(metricFields, "AnalyzeMetrics")}, "AnalyzeResult")
}

// #endregion 🖼️Sources

// #region 🗂️QueryResolvers

/** 📥️ Decodes one coerced input object into a domain input type. */
func decodeInput(value interface{}, target interface{}) *ExecutionError {
	if value == nil {
		value = map[string]interface{}{}
	}
	encoded, err := json.Marshal(value)
	if err != nil {
		return NewExecutionError(err.Error())
	}
	if err := json.Unmarshal(encoded, target); err != nil {
		return NewExecutionError(err.Error())
	}
	return nil
}

/** ❌️ Lifts one context failure into the execution error chain. */
func lift(err error) *ExecutionError {
	if err == nil {
		return nil
	}
	return NewExecutionError(err.Error())
}

/** 🔤️ The string an argument carries, or the empty string. */
func argText(args map[string]interface{}, name string) string {
	if value, ok := args[name].(string); ok {
		return value
	}
	return ""
}

/** 🔤️ The string an argument carries, or nil when it is absent. */
func argOptionalText(args map[string]interface{}, name string) *string {
	if value, ok := args[name].(string); ok {
		return &value
	}
	return nil
}

/** 🔢️ The integer an argument carries, or nil when it is absent. */
func argInt(args map[string]interface{}, name string) *int {
	switch value := args[name].(type) {
	case int:
		return &value
	case int64:
		converted := int(value)
		return &converted
	case float64:
		converted := int(value)
		return &converted
	default:
		return nil
	}
}

/** 🧹️ The filter argument of one field, decoded. */
func (run *execution) filterOf(args map[string]interface{}) *model.FilterInput {
	raw, found := args["filter"]
	if !found || raw == nil {
		return nil
	}
	filter := &model.FilterInput{}
	if decodeInput(raw, filter) != nil {
		return nil
	}
	return filter
}

/** 📜️ Every technology, filtered and sorted by name. */
func (run *execution) technologyRows(args map[string]interface{}) interface{} {
	filter := run.filterOf(args)
	technologies := append([]*model.Technology{}, run.technologies...)
	sort.SliceStable(technologies, func(left, right int) bool { return technologies[left].Name < technologies[right].Name })
	rows := make([]interface{}, 0, len(technologies))
	for _, technology := range technologies {
		if !matchesFilterInput(technology.Name, filter) && !matchesFilterInput(technologyID(technology), filter) {
			continue
		}
		rows = append(rows, technologySource(technology, run.technologies, run.rootDir))
	}
	return rows
}

/** 🔵️ Every bundle, filtered. */
func (run *execution) bundles(args map[string]interface{}) interface{} {
	filter := run.filterOf(args)
	rows := make([]interface{}, 0)
	for _, bundle := range run.repo.GetBundles() {
		if !matchesFilterInput(bundle.Name, filter) {
			continue
		}
		rows = append(rows, bundleSource(bundle, run.technologies, run.rootDir))
	}
	return rows
}

/** 🎫️ Every ticket matching the date, status and text filters. */
func (run *execution) tickets(args map[string]interface{}) (interface{}, *ExecutionError) {
	var status *model.TicketStatus
	if wire, ok := args["status"].(string); ok {
		switch wire {
		case "open":
			value := model.TicketStatusOpen
			status = &value
		case "closed":
			value := model.TicketStatusClosed
			status = &value
		}
	}
	filter := run.filterOf(args)
	tickets, err := run.repo.GetTickets(argInt(args, "year"), argInt(args, "month"), argInt(args, "day"), status)
	if err != nil {
		return nil, lift(err)
	}
	rows := make([]interface{}, 0, len(tickets))
	for _, ticket := range tickets {
		if !matchesFilterInput(ticket.Slug, filter) {
			continue
		}
		rows = append(rows, ticketSource(ticket, run.rootDir))
	}
	return rows, nil
}

/** 🌿️ Strips one entity emoji prefix, ignoring presentation selectors. */
func stripEntityPrefix(value string, emoji string) (string, bool) {
	prefix := strings.NewReplacer("︎", "", "️", "").Replace(emoji)
	if prefix == "" {
		return "", false
	}
	rest, found := strings.CutPrefix(value, prefix)
	return rest, found
}

/** 🌿️ Routes one artifact identifier to the aggregate it names. */
func (run *execution) node(args map[string]interface{}) (interface{}, *ExecutionError) {
	raw := argText(args, "id")
	clean := strings.NewReplacer("︎", "", "️", "").Replace(raw)
	if clean == "" {
		return run.repoSource(), nil
	}
	for _, kind := range []string{"technology-user", "technology-infrastructure", "technology-research", "technology-mono"} {
		rest, found := stripEntityPrefix(clean, entityEmoji(kind))
		if !found {
			continue
		}
		for _, technology := range run.technologies {
			if workspace.Flat(technology.Name) == rest || technology.Name == rest {
				return technologySource(technology, run.technologies, run.rootDir), nil
			}
		}
		return nil, nil
	}
	for _, kind := range []string{"folder-organization", "folder-required", "folder-root"} {
		if rest, found := stripEntityPrefix(clean, entityEmoji(kind)); found {
			return run.folderByPath(rest), nil
		}
	}
	for _, kind := range []string{"file-code", "file-lab", "file-script", "file-docs", "file-config", "file-resource", "file-template", "file-license"} {
		if rest, found := stripEntityPrefix(clean, entityEmoji(kind)); found {
			return run.fileByPath(rest), nil
		}
	}
	for _, kind := range []string{"definition-impl", "definition-interface", "definition-constant", "definition-test"} {
		rest, found := stripEntityPrefix(clean, entityEmoji(kind))
		if !found {
			continue
		}
		for _, definition := range run.repo.GetDefinitions() {
			if workspace.Flat(definition.Name) == rest || definition.Name == rest {
				return definitionSource(definition), nil
			}
		}
		return nil, nil
	}
	if rest, found := stripEntityPrefix(clean, entityEmoji("ticket")); found {
		slug := strings.SplitN(rest, "?", 2)[0]
		tickets, err := run.repo.GetTickets(nil, nil, nil, nil)
		if err != nil {
			return nil, lift(err)
		}
		for _, ticket := range tickets {
			if workspace.Flat(ticket.Slug) == slug || ticket.Slug == slug {
				return ticketSource(ticket, run.rootDir), nil
			}
		}
		return nil, nil
	}
	if rest, found := stripEntityPrefix(clean, entityEmoji("contributor")); found {
		contributors, err := run.repo.GetContributors()
		if err != nil {
			return nil, lift(err)
		}
		for _, contributor := range contributors {
			if workspace.Flat(contributor.Alias) == rest || contributor.Github == rest {
				return contributorSource(contributor), nil
			}
		}
		return nil, nil
	}
	if rest, found := stripEntityPrefix(clean, entityEmoji("policy")); found {
		return run.policyByID(rest), nil
	}
	for _, prefix := range []string{"repo:", "bundle:", "folder:", "file:", "contributor:", "policy:"} {
		rest, found := strings.CutPrefix(raw, prefix)
		if !found {
			continue
		}
		switch prefix {
		case "repo:":
			return run.repoSource(), nil
		case "bundle:":
			return run.bundleByName(rest), nil
		case "folder:":
			return run.folderByPath(rest), nil
		case "file:":
			return run.fileByPath(rest), nil
		case "contributor:":
			return run.contributorByID(rest)
		default:
			return run.policyByID(rest), nil
		}
	}
	return nil, NewExecutionError("invalid node id format: " + raw)
}

/** 💠️ The repository root object. */
func (run *execution) repoSource() interface{} {
	return tagged(map[string]interface{}{"id": "repo:compose", "name": "compose", "path": run.rootDir}, "Repo")
}

/** 🔵️ One bundle by name or identifier. */
func (run *execution) bundleByName(name string) interface{} {
	bundles := run.repo.GetBundles()
	for _, bundle := range bundles {
		if bundle.Name == name || bundleID(bundle, run.technologies) == name {
			return bundleSource(bundle, run.technologies, run.rootDir)
		}
	}
	return bundleSource(&model.Bundle{Name: name, Kind: model.BundleKindLibrary}, run.technologies, run.rootDir)
}

/** 🩷️ One folder by path. */
func (run *execution) folderByPath(path string) interface{} {
	normalized := strings.ReplaceAll(path, "\\", "/")
	for _, folder := range run.repo.GetFolders() {
		if folder.Path == normalized {
			return folderSource(folder)
		}
	}
	return nil
}

/** 📄️ One file by path. */
func (run *execution) fileByPath(path string) interface{} {
	normalized := strings.ReplaceAll(path, "\\", "/")
	for _, file := range run.repo.GetFiles() {
		if file.Path == normalized {
			return fileSource(file)
		}
	}
	return nil
}

/** 🧑️‍💻️ One contributor by github handle, falling back to a bare record. */
func (run *execution) contributorByID(id string) (interface{}, *ExecutionError) {
	contributors, err := run.repo.GetContributors()
	if err != nil {
		return nil, lift(err)
	}
	for _, contributor := range contributors {
		if contributor.Github == id {
			return contributorSource(contributor), nil
		}
	}
	return contributorSource(&model.Contributor{Alias: id, Github: id}), nil
}

/** 👮️ One policy by name or identifier, falling back to a bare record. */
func (run *execution) policyByID(id string) interface{} {
	for _, policy := range run.repo.GetPolicies() {
		if policy.Name == id || policy.ID == id || strings.EqualFold(policy.Name, id) || strings.EqualFold(policy.ID, id) {
			return policySource(policy, run.statutes)
		}
	}
	return policySource(&model.Policy{ID: "repo/policy/" + id, Name: id, Scopes: []string{}, Groups: []model.Territory{}, Statutes: []*model.StatuteMeta{}}, run.statutes)
}

// #endregion 🗂️QueryResolvers

// #region 🧪️EntityResolvers

/** 📅️ Tickets grouped by one calendar level. */
type ticketBucket struct {
	value    int
	children []*ticketBucket
	rows     []interface{}
}

/** 📅️ Finds or appends one bucket of a calendar level. */
func bucketOf(buckets *[]*ticketBucket, value int) *ticketBucket {
	for _, bucket := range *buckets {
		if bucket.value == value {
			return bucket
		}
	}
	bucket := &ticketBucket{value: value}
	*buckets = append(*buckets, bucket)
	return bucket
}

/** 🧑️‍💻️ The contribution tree of one contributor: their checkpoints and their tickets by date. */
func (run *execution) contributions(source interface{}) (interface{}, *ExecutionError) {
	fields, _ := source.(map[string]interface{})
	github, _ := fields["github"].(string)
	all, err := run.repo.GetTickets(nil, nil, nil, nil)
	if err != nil {
		return nil, lift(err)
	}
	tickets := make([]*model.Ticket, 0, len(all))
	for _, ticket := range all {
		if strings.EqualFold(ticketAuthor(ticket), github) {
			tickets = append(tickets, ticket)
		}
	}
	sort.SliceStable(tickets, func(left, right int) bool { return ticketStarted(tickets[left]) > ticketStarted(tickets[right]) })
	checkpoints := make([]interface{}, 0)
	seen := map[string]bool{}
	for _, ticket := range tickets {
		sha := ticketCheckpoint(ticket)
		if sha == "" || seen[sha] {
			continue
		}
		seen[sha] = true
		checkpoints = append(checkpoints, tagged(map[string]interface{}{
			"id": "repo/checkpoint/" + sha, "sha": sha, "title": ticket.Title, "date": ticketStarted(ticket),
		}, "Checkpoint"))
	}
	years := []*ticketBucket{}
	for _, ticket := range tickets {
		year := bucketOf(&years, ticket.Year)
		month := bucketOf(&year.children, ticket.Month)
		day := bucketOf(&month.children, ticket.Day)
		day.rows = append(day.rows, ticketSource(ticket, run.rootDir))
	}
	ticketYears := make([]interface{}, 0, len(years))
	for _, year := range years {
		months := make([]interface{}, 0, len(year.children))
		for _, month := range year.children {
			days := make([]interface{}, 0, len(month.children))
			for _, day := range month.children {
				days = append(days, tagged(map[string]interface{}{"day": day.value, "tickets": day.rows}, "TicketDay"))
			}
			months = append(months, tagged(map[string]interface{}{"month": month.value, "days": days}, "TicketMonth"))
		}
		ticketYears = append(ticketYears, tagged(map[string]interface{}{"year": year.value, "months": months}, "TicketYear"))
	}
	return tagged(map[string]interface{}{"checkpoints": checkpoints, "tickets": ticketYears, "bundles": []interface{}{}}, "ContributorContributions"), nil
}

/** 🎫️ The contributor a ticket's last interaction names. */
func (run *execution) ticketAuthorOf(source interface{}) (interface{}, *ExecutionError) {
	fields, ok := source.(map[string]interface{})
	if !ok {
		return nil, nil
	}
	interactions, ok := fields["interactions"].([]interface{})
	if !ok || len(interactions) == 0 {
		return nil, nil
	}
	last, ok := interactions[len(interactions)-1].(map[string]interface{})
	if !ok {
		return nil, nil
	}
	raw, _ := last["author"].(string)
	name, email := parseGitAuthor(raw)
	author := email
	if author == "" {
		author = name
	}
	contributors, err := run.repo.GetContributors()
	if err != nil {
		return nil, lift(err)
	}
	for _, contributor := range contributors {
		matched := contributor.Name == author
		for _, candidate := range contributor.Emails {
			if candidate == author || strings.Contains(author, candidate) {
				matched = true
				break
			}
		}
		if matched {
			return contributorSource(contributor), nil
		}
	}
	return contributorSource(&model.Contributor{Alias: author, Github: author, Name: author, Emails: []string{author}}), nil
}

/** 💗️ The children of a section: its nested sections and its definitions, in source order. */
func sectionChildren(source interface{}) interface{} {
	fields, ok := source.(map[string]interface{})
	items := make([]interface{}, 0)
	if !ok {
		return items
	}
	for _, key := range []string{"children", "definitions"} {
		if rows, ok := fields[key].([]interface{}); ok {
			items = append(items, rows...)
		}
	}
	position := func(item interface{}) (float64, float64) {
		row, ok := item.(map[string]interface{})
		if !ok {
			return 0, 0
		}
		start, _ := row["startLine"].(float64)
		index, _ := row["startIndex"].(float64)
		return start, index
	}
	sort.SliceStable(items, func(left, right int) bool {
		leftLine, leftIndex := position(items[left])
		rightLine, rightIndex := position(items[right])
		if leftLine != rightLine {
			return leftLine < rightLine
		}
		return leftIndex < rightIndex
	})
	return items
}

// #endregion 🧪️EntityResolvers

// #region 💻️MutationResolvers

/** ⚡️ Runs one named resolver. */
func (run *execution) dispatch(key string, source interface{}, args map[string]interface{}) (interface{}, *ExecutionError) {
	text := func(name string) string { return argText(args, name) }
	optionalText := func(name string) *string { return argOptionalText(args, name) }
	sourceText := func(name string) string {
		fields, ok := source.(map[string]interface{})
		if !ok {
			return ""
		}
		value, _ := fields[name].(string)
		return value
	}
	input := func() interface{} { return args["input"] }
	switch key {
	case "empty-list":
		return []interface{}{}, nil
	case "technologies":
		return run.technologyRows(args), nil
	case "bundles":
		return run.bundles(args), nil
	case "folders":
		rows := make([]interface{}, 0)
		for _, folder := range run.repo.GetFolders() {
			rows = append(rows, folderSource(folder))
		}
		return rows, nil
	case "files":
		rows := make([]interface{}, 0)
		for _, file := range run.repo.GetFiles() {
			rows = append(rows, fileSource(file))
		}
		return rows, nil
	case "sections":
		rows := make([]interface{}, 0)
		for _, section := range run.repo.GetSections() {
			rows = append(rows, sectionSource(section))
		}
		return rows, nil
	case "definitions":
		rows := make([]interface{}, 0)
		for _, definition := range run.repo.GetDefinitions() {
			rows = append(rows, definitionSource(definition))
		}
		return rows, nil
	case "contributors":
		filter := run.filterOf(args)
		contributors, err := run.repo.GetContributors()
		if err != nil {
			return nil, lift(err)
		}
		rows := make([]interface{}, 0, len(contributors))
		for _, contributor := range contributors {
			if !matchesFilterInput(contributor.Github, filter) {
				continue
			}
			rows = append(rows, contributorSource(contributor))
		}
		return rows, nil
	case "goals":
		goals, err := run.repo.GetGoals()
		if err != nil {
			return nil, lift(err)
		}
		rows := make([]interface{}, 0, len(goals))
		for _, goal := range goals {
			rows = append(rows, goalSource(goal))
		}
		return rows, nil
	case "tickets":
		return run.tickets(args)
	case "policies":
		filter := run.filterOf(args)
		rows := make([]interface{}, 0)
		for _, policy := range run.repo.GetPolicies() {
			if !matchesFilterInput(policy.Name, filter) {
				continue
			}
			rows = append(rows, policySource(policy, run.statutes))
		}
		return rows, nil
	case "statutes":
		rows := make([]interface{}, 0, len(run.statutes))
		for _, meta := range run.statutes {
			rows = append(rows, statuteSource(meta))
		}
		return rows, nil
	case "breachs":
		result, err := run.repo.Analyze(optionalText("scope"))
		if err != nil {
			return nil, lift(err)
		}
		rows := make([]interface{}, 0)
		if result != nil {
			for _, breach := range result.Breachs {
				rows = append(rows, breachSource(breach, run.statutes))
			}
		}
		return rows, nil
	case "checkpoints":
		checkpoints, err := run.repo.GetCheckpoints(argInt(args, "limit"))
		if err != nil {
			return nil, lift(err)
		}
		rows := make([]interface{}, 0, len(checkpoints))
		for _, checkpoint := range checkpoints {
			rows = append(rows, checkpointSource(checkpoint))
		}
		return rows, nil
	case "folder.children":
		id := sourceText("id")
		rows := make([]interface{}, 0)
		for _, folder := range run.repo.GetFolders() {
			if folder.ParentID != nil && *folder.ParentID == id {
				rows = append(rows, folderSource(folder))
			}
		}
		return rows, nil
	case "folder.files":
		id := sourceText("id")
		rows := make([]interface{}, 0)
		for _, file := range run.repo.GetFiles() {
			if file.FolderID != nil && *file.FolderID == id {
				rows = append(rows, fileSource(file))
			}
		}
		return rows, nil
	case "file.sections":
		path := sourceText("path")
		rows := make([]interface{}, 0)
		for _, section := range run.repo.GetSections() {
			if section.FilePath == path {
				rows = append(rows, sectionSource(section))
			}
		}
		return rows, nil
	case "file.definitions":
		path := sourceText("path")
		rows := make([]interface{}, 0)
		for _, definition := range run.repo.GetDefinitions() {
			if definition.FilePath == path {
				rows = append(rows, definitionSource(definition))
			}
		}
		return rows, nil
	case "item.file":
		path := sourceText("filePath")
		if path == "" {
			return nil, nil
		}
		return run.fileByPath(path), nil
	case "definition.section":
		path := sourceText("sectionPath")
		if path == "" {
			return nil, nil
		}
		return sectionSource(&model.Section{Name: path, Path: path, FilePath: sourceText("filePath")}), nil
	case "section.children":
		return sectionChildren(source), nil
	case "statute.policy":
		return policySource(&model.Policy{
			ID: "/policies/lint-scripts", Name: "Lint scripts", Scopes: []string{"**/*"}, Groups: []model.Territory{}, Statutes: []*model.StatuteMeta{},
		}, run.statutes), nil
	case "ticket.author":
		return run.ticketAuthorOf(source)
	case "contributor.contributions":
		return run.contributions(source)
	case "query.node":
		return run.node(args)
	case "query.repo":
		return run.repoSource(), nil
	case "query.todos":
		todos, err := run.repo.GetTodos(run.filterOf(args))
		if err != nil {
			return nil, lift(err)
		}
		rows := make([]interface{}, 0, len(todos))
		for _, todo := range todos {
			rows = append(rows, todoSource(todo))
		}
		return rows, nil
	case "query.interactions":
		interactions, err := run.repo.GetInteractions()
		if err != nil {
			return nil, lift(err)
		}
		rows := make([]interface{}, 0, len(interactions))
		for index := range interactions {
			rows = append(rows, interactionResourceSource(&interactions[index]))
		}
		return rows, nil
	case "query.drafts":
		drafts, err := run.repo.GetDrafts()
		if err != nil {
			return nil, lift(err)
		}
		rows := make([]interface{}, 0, len(drafts))
		for _, draft := range drafts {
			rows = append(rows, draftSource(draft))
		}
		return rows, nil
	case "query.bundle":
		return run.bundleByName(text("name")), nil
	case "query.folder":
		return run.folderByPath(text("path")), nil
	case "query.file":
		return run.fileByPath(text("path")), nil
	case "query.section":
		path := text("path")
		segments, ok := args["sectionPath"].([]interface{})
		if !ok {
			return nil, nil
		}
		parts := make([]string, 0, len(segments))
		for _, segment := range segments {
			if value, ok := segment.(string); ok {
				parts = append(parts, value)
			}
		}
		joined := strings.Join(parts, "#")
		for _, section := range run.repo.GetSections() {
			if section.FilePath == path && section.Path == joined {
				return sectionSource(section), nil
			}
		}
		return sectionSource(&model.Section{Name: joined}), nil
	case "query.definition":
		path, name := text("path"), text("name")
		for _, definition := range run.repo.GetDefinitions() {
			if definition.FilePath == path && definition.Name == name {
				return definitionSource(definition), nil
			}
		}
		return definitionSource(&model.Definition{Name: name, Kind: model.DefinitionKindImplementation}), nil
	case "query.contributor":
		return run.contributorByID(text("id"))
	case "query.ticket":
		slug := text("slug")
		tickets, err := run.repo.GetTickets(argInt(args, "year"), argInt(args, "month"), argInt(args, "day"), nil)
		if err != nil {
			return nil, lift(err)
		}
		for _, ticket := range tickets {
			if ticket.Slug == slug {
				return ticketSource(ticket, run.rootDir), nil
			}
		}
		return nil, nil
	case "query.policy":
		return run.policyByID(text("id")), nil
	case "query.statute":
		meta := statuteMetaOf(model.Statute(text("id")), run.statutes)
		return statuteSource(&meta), nil
	case "query.analyze":
		result, err := run.repo.Analyze(optionalText("scope"))
		if err != nil {
			return nil, lift(err)
		}
		return analyzeResultSource(result, run.statutes), nil
	case "mutation.syncManagement":
		done, err := run.repo.SyncManagement()
		if err != nil {
			return nil, lift(err)
		}
		return done, nil
	case "mutation.goalCreate":
		decoded := model.GoalCreateInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		goal, err := run.repo.GoalCreate(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return goalSource(goal), nil
	case "mutation.goalChange":
		decoded := model.GoalChangeInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		decoded.ID = text("id")
		goal, err := run.repo.GoalChange(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return goalSource(goal), nil
	case "mutation.goalClose":
		decoded := model.GoalCloseInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		goal, err := run.repo.GoalClose(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return goalSource(goal), nil
	case "mutation.goalReopen":
		decoded := model.GoalReopenInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		goal, err := run.repo.GoalReopen(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return goalSource(goal), nil
	case "mutation.draftCreate":
		decoded := model.DraftCreateInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		draft, err := run.repo.DraftCreate(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return draftSource(draft), nil
	case "mutation.draftDelete":
		deleted, err := run.repo.DraftDelete(text("id"))
		if err != nil {
			return nil, lift(err)
		}
		return deleted, nil
	case "mutation.todoCreate":
		decoded := model.TodoCreateInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		todo, err := run.repo.TodoCreate(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return todoSource(todo), nil
	case "mutation.todoChange":
		decoded := model.TodoChangeInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		todo, err := run.repo.TodoChange(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return todoSource(todo), nil
	case "mutation.todoDelete":
		deleted, err := run.repo.TodoDelete(text("id"))
		if err != nil {
			return nil, lift(err)
		}
		return deleted, nil
	case "mutation.ticketOpen":
		decoded := model.TicketOpenInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		ticket, err := run.repo.TicketOpen(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return ticketSource(ticket, run.rootDir), nil
	case "mutation.ticketClose":
		decoded := model.TicketCloseInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		ticket, err := run.repo.TicketClose(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return ticketSource(ticket, run.rootDir), nil
	case "mutation.ticketReopen":
		decoded := model.TicketReopenInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		ticket, err := run.repo.TicketReopen(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return ticketSource(ticket, run.rootDir), nil
	case "mutation.ticketChange":
		decoded := model.TicketChangeInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		ticket, err := run.repo.TicketChange(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return ticketSource(ticket, run.rootDir), nil
	case "mutation.contributorAdd":
		decoded := model.ContributorAddInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		contributor, err := run.repo.ContributorAdd(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return contributorSource(contributor), nil
	case "mutation.contributorRemove":
		if err := run.repo.ContributorRemove(text("github")); err != nil {
			return nil, lift(err)
		}
		return true, nil
	case "mutation.folderCreate":
		folder, err := run.repo.FolderCreate(text("path"))
		if err != nil {
			return nil, lift(err)
		}
		return folderSource(folder), nil
	case "mutation.folderMove":
		folder, err := run.repo.FolderMove(text("src"), text("dst"))
		if err != nil {
			return nil, lift(err)
		}
		return folderSource(folder), nil
	case "mutation.folderDelete":
		if err := run.repo.FolderDelete(text("path")); err != nil {
			return nil, lift(err)
		}
		return true, nil
	case "mutation.fileCreate":
		file, err := run.repo.FileCreate(text("path"))
		if err != nil {
			return nil, lift(err)
		}
		return fileSource(file), nil
	case "mutation.fileMove":
		file, err := run.repo.FileMove(text("src"), text("dst"))
		if err != nil {
			return nil, lift(err)
		}
		return fileSource(file), nil
	case "mutation.fileDelete":
		if err := run.repo.FileDelete(text("path")); err != nil {
			return nil, lift(err)
		}
		return true, nil
	case "mutation.sectionCreate":
		section, err := run.repo.SectionCreate(text("file"), text("name"), optionalText("parent"))
		if err != nil {
			return nil, lift(err)
		}
		return sectionSource(section), nil
	case "mutation.sectionMove":
		section, err := run.repo.SectionMove(text("file"), text("oldName"), text("newName"))
		if err != nil {
			return nil, lift(err)
		}
		return sectionSource(section), nil
	case "mutation.sectionDelete":
		if err := run.repo.SectionDelete(text("file"), text("name")); err != nil {
			return nil, lift(err)
		}
		return true, nil
	case "mutation.integrate":
		file, err := run.repo.Integrate(optionalText("source"), optionalText("targetSection"), optionalText("targetFile"), optionalText("targetParent"))
		if err != nil {
			return nil, lift(err)
		}
		return fileSource(file), nil
	case "mutation.extract":
		file, err := run.repo.Extract(optionalText("sourceFile"), optionalText("sourceSection"), optionalText("targetFile"))
		if err != nil {
			return nil, lift(err)
		}
		return fileSource(file), nil
	default:
		return nil, NewExecutionError("no resolver registered for " + key)
	}
}

// #endregion 💻️MutationResolvers

// #region 🎙️GraphQL

// 💿️gql holds the data fields for a gql record.
func Gql(query string, variables map[string]interface{}) (string, error) {
	ensureExecutor()
	if executor == nil {
		return "", fmt.Errorf("GraphQL executor not initialized")
	}
	return executor.ExecuteJSON(context.Background(), query, variables)
}

// #endregion 🎙️GraphQL

// #region 🌦️GraphQL Helpers

// 💿️printGQL holds the data fields for a printGQL record.
func PrintGQL(query string, variables map[string]interface{}) error {
	result, err := Gql(query, variables)
	if err != nil {
		return err
	}
	fmt.Println(result)
	return nil
}

// #endregion 🌦️GraphQL Helpers

// #region 🔊️Cli

// 💿️repoResolverInstance holds the data fields for a repoResolverInstance record.
var RepoResolverInstance *Resolver

// #endregion 🔊️Cli

// #region 📰️Todos

// 📨️GetTodos MUST retrieve the requested value or return an error.
// 📖️GetTodos retrieves and returns the todos.
func (c *RepoContext) GetTodos(filter *model.FilterInput) ([]*model.Todo, error) {
	allTodos := c.scanTodos()
	if filter != nil && filter.Filter != nil {
		search := strings.ToLower(*filter.Filter)
		var match []*model.Todo
		for _, t := range allTodos {
			if strings.Contains(strings.ToLower(t.Name), search) || strings.Contains(strings.ToLower(t.Description), search) {
				match = append(match, t)
			}
		}
		return match, nil
	}
	return allTodos, nil
}

// 🆕️TodoCreate MUST return a non-nil error when the operation fails.
// 🆕️TodoCreate performs the todo create operation on the repo context.
func (c *RepoContext) TodoCreate(input model.TodoCreateInput) (*model.Todo, error) {

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
		todo := &model.Todo{ID: workspace.Slugify(input.Name), Name: input.Name, Description: input.Description, ParentID: input.ParentID}
		events.Emit(events.EventTodoCreateEnded, "repo-cli", events.TodoCreatePayload{
			TodoPayload: events.TodoPayload{ID: todo.ID, ParentID: input.ParentID, Name: input.Name, Author: contributorspkg.GetGitAuthorAlias()},
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
		todo := &model.Todo{ID: workspace.Slugify(input.Name), Name: input.Name, Description: input.Description, ParentID: input.ParentID, Location: &model.Location{FilePath: input.ParentID}}
		events.Emit(events.EventTodoCreateEnded, "repo-cli", events.TodoCreatePayload{
			TodoPayload: events.TodoPayload{ID: todo.ID, ParentID: input.ParentID, Name: input.Name, Author: contributorspkg.GetGitAuthorAlias()},
		})
		return todo, nil
	}
	return nil, fmt.Errorf("invalid parent id (must be path to folder or file)")
}

// ♻️TodoChange MUST return a non-nil error when the operation fails.
// ✏️TodoChange performs the todo change operation on the repo context.
func (c *RepoContext) TodoChange(input model.TodoChangeInput) (*model.Todo, error) {
	todos := c.scanTodos()
	var todo *model.Todo
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
		if err := todospkg.ReplaceLineInMarkdown(todo.Location.FilePath, todo.Name, name, description); err != nil {
			return nil, err
		}
	} else if todo.Location.Line > 0 {
		if err := todospkg.ReplaceLineInFile(todo.Location.FilePath, todo.Location.Line, name, description); err != nil {
			return nil, err
		}
	} else {
		return nil, fmt.Errorf("unable to locate todo source line")
	}
	updated := &model.Todo{ID: workspace.Slugify(name), Name: name, Description: description, ParentID: todo.ParentID, Location: todo.Location}
	events.Emit(events.EventTodoChangeEnded, "repo-cli", events.TodoChangePayload{
		TodoPayload: events.TodoPayload{ID: updated.ID, ParentID: todo.ParentID, Name: name, Author: contributorspkg.GetGitAuthorAlias()},
		Name:        input.Name, Description: input.Description,
	})
	return updated, nil
}

// 🗑️TodoDelete MUST return a non-nil error when the operation fails.
// 🗑️TodoDelete performs the todo delete operation on the repo context.
func (c *RepoContext) TodoDelete(id string) (bool, error) {
	todos := c.scanTodos()
	for _, t := range todos {
		if t.ID == id {
			if strings.HasSuffix(t.Location.FilePath, ".todos.md") {
				todospkg.RemoveLineFromMarkdown(t.Location.FilePath, t.Name)
				events.Emit(events.EventTodoDeleteEnded, "repo-cli", events.TodoDeletePayload{
					TodoPayload: events.TodoPayload{ID: t.ID, ParentID: t.ParentID, Name: t.Name, Author: contributorspkg.GetGitAuthorAlias()},
				})
				return true, nil
			}
			if t.Location.Line > 0 {
				todospkg.RemoveLineFromFile(t.Location.FilePath, t.Location.Line)
				events.Emit(events.EventTodoDeleteEnded, "repo-cli", events.TodoDeletePayload{
					TodoPayload: events.TodoPayload{ID: t.ID, ParentID: t.ParentID, Name: t.Name, Author: contributorspkg.GetGitAuthorAlias()},
				})
				return true, nil
			}
		}
	}
	return false, fmt.Errorf("todo not found")
}

// 🎫️TodoToTicket MUST return a non-nil error when the operation fails.
// 🔤️TodoToTicket performs the todo to ticket operation on the repo context.
func (c *RepoContext) TodoToTicket(id string, input model.TicketOpenInput) (*model.Ticket, error) {
	todos := c.scanTodos()
	var todo *model.Todo
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

// #endregion 📰️Todos

// #region 🔌️GraphQL Ports

// 🔌️init wires the repository port 📐️model declares but cannot implement.
func init() {
	model.LookupRepoContext = NewRepoContext
}

// #endregion 🔌️GraphQL Ports

// #region 🩻️RecordingContext

/** 🗄️ The mutable record set a [RecordingContext] answers from. */
type recordingRecords struct {
	RootDir      string                      `json:"rootDir"`
	Now          string                      `json:"now"`
	Technologies []*model.Technology         `json:"technologies"`
	Bundles      []*model.Bundle             `json:"bundles"`
	Checkpoints  []*model.Checkpoint         `json:"checkpoints"`
	Folders      []*model.Folder             `json:"folders"`
	Files        []*model.File               `json:"files"`
	Sections     []*model.Section            `json:"sections"`
	Definitions  []*model.Definition         `json:"definitions"`
	Contributors []*model.Contributor        `json:"contributors"`
	Policies     []*model.Policy             `json:"policies"`
	Drafts       []*model.Draft              `json:"drafts"`
	Todos        []*model.Todo               `json:"todos"`
	Statutes     []*model.StatuteMeta        `json:"statutes"`
	Interactions []model.InteractionResource `json:"-"`
	Analyze      *model.AnalyzeResult        `json:"analyze"`
	Goals        []*model.Goal               `json:"-"`
	Tickets      []*model.Ticket             `json:"-"`
	Events       []map[string]interface{}    `json:"-"`
}

/** 🎫️ The ticket fields the wire shape carries but the domain type derives from its path. */
type recordedTicketIdentity struct {
	Year          int                 `json:"year"`
	Month         int                 `json:"month"`
	Day           int                 `json:"day"`
	Slug          string              `json:"slug"`
	Parent        string              `json:"parent"`
	FolderPath    string              `json:"folderPath"`
	JsonPath      string              `json:"jsonPath"`
	ImportantPath string              `json:"importantPath"`
	Interactions  []model.Interaction `json:"interactions"`
	Agents        []model.TicketAgent `json:"agents"`
}

/** 🎯️ The goal fields the wire shape carries but the domain type derives from its path. */
type recordedGoalIdentity struct {
	ID   string `json:"id"`
	Path string `json:"path"`
}

/** 🩻️ A fixture-backed context that answers every read from a record set and records every write.
 *
 * It is the executor's language-agnostic test double: the same record file drives every subject and
 * the oracle, so what two implementations disagree about is the executor, never the filesystem. */
type RecordingContext struct {
	records *recordingRecords
}

/** 📥️ Builds a context from one record document's bytes. */
func NewRecordingContext(source []byte) (*RecordingContext, error) {
	records := &recordingRecords{Now: "2026-01-01 00:00:00"}
	if err := json.Unmarshal(source, records); err != nil {
		return nil, &ContextError{Message: err.Error()}
	}
	var identities struct {
		Tickets []recordedTicketIdentity `json:"tickets"`
		Goals   []recordedGoalIdentity   `json:"goals"`
	}
	if err := json.Unmarshal(source, &identities); err != nil {
		return nil, &ContextError{Message: err.Error()}
	}
	var shapes struct {
		Tickets      []*plainTicket      `json:"tickets"`
		Goals        []*plainGoal        `json:"goals"`
		Interactions []model.Interaction `json:"interactions"`
	}
	if err := json.Unmarshal(source, &shapes); err != nil {
		return nil, &ContextError{Message: err.Error()}
	}
	var resources struct {
		Interactions []struct {
			SourceKind string `json:"sourceKind"`
			SourceID   string `json:"sourceId"`
			GoalID     string `json:"goalId"`
			TicketID   string `json:"ticketId"`
		} `json:"interactions"`
	}
	if err := json.Unmarshal(source, &resources); err != nil {
		return nil, &ContextError{Message: err.Error()}
	}
	for index, interaction := range shapes.Interactions {
		resource := model.InteractionResource{Interaction: interaction}
		if index < len(resources.Interactions) {
			extra := resources.Interactions[index]
			resource.SourceKind, resource.SourceID = extra.SourceKind, extra.SourceID
			resource.GoalID, resource.TicketID = extra.GoalID, extra.TicketID
		}
		records.Interactions = append(records.Interactions, resource)
	}
	for index, ticket := range shapes.Tickets {
		if index >= len(identities.Tickets) {
			break
		}
		identity := identities.Tickets[index]
		ticket.Year, ticket.Month, ticket.Day, ticket.Slug = identity.Year, identity.Month, identity.Day, identity.Slug
		ticket.Parent, ticket.FolderPath, ticket.JsonPath, ticket.ImportantPath = identity.Parent, identity.FolderPath, identity.JsonPath, identity.ImportantPath
		ticket.Interactions, ticket.Agents = identity.Interactions, identity.Agents
	}
	for index, goal := range shapes.Goals {
		if index >= len(identities.Goals) {
			break
		}
		goal.ID, goal.Path = identities.Goals[index].ID, identities.Goals[index].Path
	}
	for _, ticket := range shapes.Tickets {
		records.Tickets = append(records.Tickets, (*model.Ticket)(ticket))
	}
	for _, goal := range shapes.Goals {
		records.Goals = append(records.Goals, (*model.Goal)(goal))
	}
	if records.Now == "" {
		records.Now = "2026-01-01 00:00:00"
	}
	return &RecordingContext{records: records}, nil
}

/** 📥️ Builds a context from one record document's text. */
func NewRecordingContextFromText(source string) (*RecordingContext, error) {
	return NewRecordingContext([]byte(source))
}

/** 📜️ The events every mutation appended, in the order they happened. */
func (recording *RecordingContext) Events() []interface{} {
	rows := make([]interface{}, 0, len(recording.records.Events))
	for _, event := range recording.records.Events {
		rows = append(rows, event)
	}
	return rows
}

/** 🎯️ Every goal record, read off its declared fields alone. */
func plainGoals(goals []*model.Goal) []interface{} {
	rows := make([]interface{}, 0, len(goals))
	for _, goal := range goals {
		rows = append(rows, goalBody(goal))
	}
	return rows
}

/** 🗄️ The mutable record set as it stands now, for a mutation case to assert on. */
func (recording *RecordingContext) Snapshot() interface{} {
	tickets := make([]interface{}, 0, len(recording.records.Tickets))
	for _, ticket := range recording.records.Tickets {
		row := ticketBody(ticket)
		row["year"], row["month"], row["day"], row["slug"] = ticket.Year, ticket.Month, ticket.Day, ticket.Slug
		row["interactions"] = toJSON(ticket.Interactions)
		tickets = append(tickets, row)
	}
	return map[string]interface{}{
		"goals":        plainGoals(recording.records.Goals),
		"tickets":      tickets,
		"todos":        toJSON(recording.records.Todos),
		"drafts":       toJSON(recording.records.Drafts),
		"contributors": toJSON(recording.records.Contributors),
		"folders":      toJSON(recording.records.Folders),
		"files":        toJSON(recording.records.Files),
		"sections":     toJSON(recording.records.Sections),
	}
}

/** 📝️ Appends one event. */
func (recording *RecordingContext) record(kind string, payload interface{}) {
	recording.records.Events = append(recording.records.Events, map[string]interface{}{"kind": kind, "payload": toJSON(payload)})
}

/** 🔤️ The identifier a created aggregate derives from its title. */
func deriveRecordID(title string) string {
	parts := strings.FieldsFunc(strings.ToLower(title), func(character rune) bool {
		return !unicode.IsLetter(character) && !unicode.IsDigit(character)
	})
	return strings.Join(parts, "-")
}

func (recording *RecordingContext) GetRootDir() string { return recording.records.RootDir }

func (recording *RecordingContext) GetTechnologies() []*model.Technology {
	return recording.records.Technologies
}

func (recording *RecordingContext) GetBundles() []*model.Bundle { return recording.records.Bundles }

func (recording *RecordingContext) GetCheckpoints(limit *int) ([]*model.Checkpoint, error) {
	checkpoints := recording.records.Checkpoints
	if limit != nil && *limit >= 0 && *limit < len(checkpoints) {
		return checkpoints[:*limit], nil
	}
	return checkpoints, nil
}

func (recording *RecordingContext) GetFolders() []*model.Folder { return recording.records.Folders }

func (recording *RecordingContext) GetFiles() []*model.File { return recording.records.Files }

func (recording *RecordingContext) GetSections() []*model.Section { return recording.records.Sections }

func (recording *RecordingContext) GetDefinitions() []*model.Definition {
	return recording.records.Definitions
}

func (recording *RecordingContext) GetContributors() ([]*model.Contributor, error) {
	return recording.records.Contributors, nil
}

func (recording *RecordingContext) GetGoals() ([]*model.Goal, error) {
	return recording.records.Goals, nil
}

func (recording *RecordingContext) GetTickets(year, month, day *int, status *model.TicketStatus) ([]*model.Ticket, error) {
	rows := make([]*model.Ticket, 0, len(recording.records.Tickets))
	for _, ticket := range recording.records.Tickets {
		if year != nil && ticket.Year != *year {
			continue
		}
		if month != nil && ticket.Month != *month {
			continue
		}
		if day != nil && ticket.Day != *day {
			continue
		}
		if status != nil && ticket.Status != *status {
			continue
		}
		rows = append(rows, ticket)
	}
	return rows, nil
}

func (recording *RecordingContext) GetPolicies() []*model.Policy { return recording.records.Policies }

func (recording *RecordingContext) GetDrafts() ([]*model.Draft, error) {
	return recording.records.Drafts, nil
}

func (recording *RecordingContext) GetTodos(filter *model.FilterInput) ([]*model.Todo, error) {
	rows := make([]*model.Todo, 0, len(recording.records.Todos))
	for _, todo := range recording.records.Todos {
		if !matchesFilterInput(todo.Name, filter) {
			continue
		}
		rows = append(rows, todo)
	}
	return rows, nil
}

func (recording *RecordingContext) GetStatutes() []*model.StatuteMeta {
	return recording.records.Statutes
}

func (recording *RecordingContext) GetInteractions() ([]model.InteractionResource, error) {
	return recording.records.Interactions, nil
}

func (recording *RecordingContext) Analyze(scope *string) (*model.AnalyzeResult, error) {
	result := recording.records.Analyze
	if result == nil {
		return &model.AnalyzeResult{Breachs: []*model.Breach{}, Metrics: &model.AnalyzeMetrics{ByPriority: &model.PriorityCount{}}}, nil
	}
	if scope == nil {
		return result, nil
	}
	breachs := make([]*model.Breach, 0, len(result.Breachs))
	for _, breach := range result.Breachs {
		if strings.HasPrefix(breach.Scope, *scope) {
			breachs = append(breachs, breach)
		}
	}
	return &model.AnalyzeResult{Breachs: breachs, Metrics: result.Metrics}, nil
}

func (recording *RecordingContext) GoalCreate(input model.GoalCreateInput) (*model.Goal, error) {
	goal := &model.Goal{
		Title: input.Title, Description: input.Description, Prompt: input.Prompt, Status: "open",
		DueDate: input.DueDate, Dates: model.GoalDates{Due: input.DueDate}, Client: input.Client,
		LLM: input.LLM, Effort: input.Effort, Parent: input.Parent,
		Management: &model.GoalManagementData{Milestone: input.Milestone}, ID: deriveRecordID(input.Title),
	}
	recording.records.Goals = append(recording.records.Goals, goal)
	recording.record("goal.create", input)
	return goal, nil
}

/** 🎯️ The recorded goal one identifier names. */
func (recording *RecordingContext) goal(id string) (*model.Goal, error) {
	for _, goal := range recording.records.Goals {
		if goal.ID == id {
			return goal, nil
		}
	}
	return nil, &ContextError{Message: fmt.Sprintf("goal %s not found", id)}
}

func (recording *RecordingContext) GoalChange(input model.GoalChangeInput) (*model.Goal, error) {
	goal, err := recording.goal(input.ID)
	if err != nil {
		return nil, err
	}
	if input.Title != nil {
		goal.Title = *input.Title
	}
	if input.Description != nil {
		goal.Description = *input.Description
	}
	if input.DueDate != nil {
		goal.DueDate = *input.DueDate
		goal.Dates.Due = *input.DueDate
	}
	if input.LLM != nil {
		goal.LLM = *input.LLM
	}
	if input.Effort != nil {
		goal.Effort = *input.Effort
	}
	if input.Parent != nil {
		goal.Parent = *input.Parent
	}
	recording.record("goal.change", input)
	return goal, nil
}

func (recording *RecordingContext) GoalClose(input model.GoalCloseInput) (*model.Goal, error) {
	goal, err := recording.goal(input.ID)
	if err != nil {
		return nil, err
	}
	goal.Status = "closed"
	goal.Summary = input.Summary
	recording.record("goal.close", input)
	return goal, nil
}

func (recording *RecordingContext) GoalReopen(input model.GoalReopenInput) (*model.Goal, error) {
	goal, err := recording.goal(input.ID)
	if err != nil {
		return nil, err
	}
	goal.Status = "open"
	goal.Prompt = input.Prompt
	goal.Client = input.Client
	goal.LLM = input.LLM
	recording.record("goal.reopen", input)
	return goal, nil
}

func (recording *RecordingContext) GoalDelete(input model.GoalDeleteInput) (bool, error) {
	before := len(recording.records.Goals)
	kept := make([]*model.Goal, 0, before)
	for _, goal := range recording.records.Goals {
		if goal.ID != input.ID {
			kept = append(kept, goal)
		}
	}
	recording.records.Goals = kept
	recording.record("goal.delete", input)
	return len(kept) < before, nil
}

func (recording *RecordingContext) TodoCreate(input model.TodoCreateInput) (*model.Todo, error) {
	todo := &model.Todo{ID: deriveRecordID(input.Name), Name: input.Name, Description: input.Description, ParentID: input.ParentID}
	recording.records.Todos = append(recording.records.Todos, todo)
	recording.record("todo.create", input)
	return todo, nil
}

func (recording *RecordingContext) TodoChange(input model.TodoChangeInput) (*model.Todo, error) {
	for _, todo := range recording.records.Todos {
		if todo.ID != input.ID {
			continue
		}
		if input.Name != nil {
			todo.Name = *input.Name
		}
		if input.Description != nil {
			todo.Description = *input.Description
		}
		recording.record("todo.change", input)
		return todo, nil
	}
	return nil, &ContextError{Message: fmt.Sprintf("todo %s not found", input.ID)}
}

func (recording *RecordingContext) TodoDelete(id string) (bool, error) {
	before := len(recording.records.Todos)
	kept := make([]*model.Todo, 0, before)
	for _, todo := range recording.records.Todos {
		if todo.ID != id {
			kept = append(kept, todo)
		}
	}
	recording.records.Todos = kept
	recording.record("todo.delete", id)
	return len(kept) < before, nil
}

func (recording *RecordingContext) DraftCreate(input model.DraftCreateInput) (*model.Draft, error) {
	draft := &model.Draft{ID: deriveRecordID(input.Title)}
	recording.records.Drafts = append(recording.records.Drafts, draft)
	recording.record("draft.create", input)
	return draft, nil
}

func (recording *RecordingContext) DraftDelete(id string) (bool, error) {
	before := len(recording.records.Drafts)
	kept := make([]*model.Draft, 0, before)
	for _, draft := range recording.records.Drafts {
		if draft.ID != id {
			kept = append(kept, draft)
		}
	}
	recording.records.Drafts = kept
	recording.record("draft.delete", id)
	return len(kept) < before, nil
}

/** 🎫️ The recorded ticket one date and slug name. */
func (recording *RecordingContext) ticket(year, month, day int, slug string) (*model.Ticket, error) {
	for _, ticket := range recording.records.Tickets {
		if ticket.Year == year && ticket.Month == month && ticket.Day == day && ticket.Slug == slug {
			return ticket, nil
		}
	}
	return nil, &ContextError{Message: fmt.Sprintf("ticket %d/%d/%d/%s not found", year, month, day, slug)}
}

func (recording *RecordingContext) TicketOpen(input model.TicketOpenInput) (*model.Ticket, error) {
	now := recording.records.Now
	year, month, day := 0, 0, 0
	if len(now) >= 10 {
		year, _ = strconv.Atoi(now[0:4])
		month, _ = strconv.Atoi(now[5:7])
		day, _ = strconv.Atoi(now[8:10])
	}
	slug := strings.ToUpper(deriveRecordID(input.Title))
	ticket := &model.Ticket{
		Year: year, Month: month, Day: day, Slug: slug, Title: input.Title, Emoji: input.Emoji,
		Status: model.TicketStatusOpen, Description: input.Prompt,
		Management: &model.TicketManagementData{Issue: input.Issue}, Goal: input.Goal, Parent: input.Parent,
		Interactions: []model.Interaction{{
			Kind: "ticket.open", Date: now, Author: input.Client, Client: input.Client,
			Prompt: input.Prompt, LLM: input.LLM, Effort: input.Effort,
		}},
		FolderPath: fmt.Sprintf("%04d/%02d/%02d/%s", year, month, day, slug),
	}
	recording.records.Tickets = append(recording.records.Tickets, ticket)
	recording.record("ticket.open", input)
	return ticket, nil
}

func (recording *RecordingContext) TicketClose(input model.TicketCloseInput) (*model.Ticket, error) {
	ticket, err := recording.ticket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}
	ticket.Status = model.TicketStatusClosed
	ticket.Summary = input.Summary
	if input.Title != nil {
		ticket.Title = *input.Title
	}
	files := make([]model.InteractionFile, 0, len(input.Files))
	for _, path := range input.Files {
		files = append(files, model.InteractionFile{Path: path})
	}
	ticket.Interactions = append(ticket.Interactions, model.Interaction{Kind: "ticket.close", Date: recording.records.Now, Summary: input.Summary, Files: files})
	recording.record("ticket.close", input)
	return ticket, nil
}

func (recording *RecordingContext) TicketReopen(input model.TicketReopenInput) (*model.Ticket, error) {
	ticket, err := recording.ticket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}
	ticket.Status = model.TicketStatusOpen
	if input.Title != nil {
		ticket.Title = *input.Title
	}
	ticket.Interactions = append(ticket.Interactions, model.Interaction{
		Kind: "ticket.open", Date: recording.records.Now, Author: input.Client, Client: input.Client,
		Prompt: input.Prompt, LLM: input.LLM, Effort: input.Effort,
	})
	recording.record("ticket.reopen", input)
	return ticket, nil
}

func (recording *RecordingContext) TicketChange(input model.TicketChangeInput) (*model.Ticket, error) {
	ticket, err := recording.ticket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}
	if input.Title != nil {
		ticket.Title = *input.Title
	}
	if input.Prompt != nil {
		ticket.Description = *input.Prompt
	}
	if input.Goal != nil {
		ticket.Goal = *input.Goal
	}
	if input.Parent != nil {
		ticket.Parent = *input.Parent
	}
	recording.record("ticket.change", input)
	return ticket, nil
}

func (recording *RecordingContext) TicketDelete(input model.TicketDeleteInput) (bool, error) {
	before := len(recording.records.Tickets)
	kept := make([]*model.Ticket, 0, before)
	for _, ticket := range recording.records.Tickets {
		if ticket.Year == input.Year && ticket.Month == input.Month && ticket.Day == input.Day && ticket.Slug == input.Slug {
			continue
		}
		kept = append(kept, ticket)
	}
	recording.records.Tickets = kept
	recording.record("ticket.delete", input)
	return len(kept) < before, nil
}

func (recording *RecordingContext) ContributorAdd(input model.ContributorAddInput) (*model.Contributor, error) {
	text := func(value *string) string {
		if value == nil {
			return ""
		}
		return *value
	}
	contributor := &model.Contributor{
		Alias: input.Github, Github: input.Github, Name: text(input.Name), Names: input.Names,
		Email: text(input.Email), Emails: input.Emails, Fingerprint: text(input.Fingerprint), Fingerprints: input.Fingerprints,
	}
	recording.records.Contributors = append(recording.records.Contributors, contributor)
	recording.record("contributor.add", input)
	return contributor, nil
}

func (recording *RecordingContext) ContributorRemove(github string) error {
	kept := make([]*model.Contributor, 0, len(recording.records.Contributors))
	for _, contributor := range recording.records.Contributors {
		if contributor.Github != github {
			kept = append(kept, contributor)
		}
	}
	recording.records.Contributors = kept
	recording.record("contributor.remove", github)
	return nil
}

/** 📁️ The last path segment of one repository-relative path. */
func lastSegment(path string) string {
	if index := strings.LastIndex(path, "/"); index >= 0 {
		return path[index+1:]
	}
	return path
}

func (recording *RecordingContext) FolderCreate(path string) (*model.Folder, error) {
	folder := &model.Folder{
		ID: deriveRecordID(path), Path: path, URI: fileURI(recording.GetRootDir(), path),
		Name: lastSegment(path), Kind: model.FolderKindOrganization,
	}
	recording.records.Folders = append(recording.records.Folders, folder)
	recording.record("folder.create", path)
	return folder, nil
}

func (recording *RecordingContext) FolderMove(src, dst string) (*model.Folder, error) {
	for _, folder := range recording.records.Folders {
		if folder.Path != src {
			continue
		}
		folder.Path = dst
		folder.URI = fileURI(recording.GetRootDir(), dst)
		folder.Name = lastSegment(dst)
		recording.record("folder.move", []string{src, dst})
		return folder, nil
	}
	return nil, &ContextError{Message: fmt.Sprintf("folder %s not found", src)}
}

func (recording *RecordingContext) FolderDelete(path string) error {
	kept := make([]*model.Folder, 0, len(recording.records.Folders))
	for _, folder := range recording.records.Folders {
		if folder.Path != path {
			kept = append(kept, folder)
		}
	}
	recording.records.Folders = kept
	recording.record("folder.delete", path)
	return nil
}

func (recording *RecordingContext) FileCreate(path string) (*model.File, error) {
	name := lastSegment(path)
	extension := ""
	if index := strings.LastIndex(name, "."); index >= 0 {
		extension = name[index:]
	}
	file := &model.File{
		ID: deriveRecordID(path), Path: path, URI: fileURI(recording.GetRootDir(), path),
		Name: name, Extension: extension, Kind: "code",
	}
	recording.records.Files = append(recording.records.Files, file)
	recording.record("file.create", path)
	return file, nil
}

func (recording *RecordingContext) FileMove(src, dst string) (*model.File, error) {
	for _, file := range recording.records.Files {
		if file.Path != src {
			continue
		}
		file.Path = dst
		file.URI = fileURI(recording.GetRootDir(), dst)
		file.Name = lastSegment(dst)
		recording.record("file.move", []string{src, dst})
		return file, nil
	}
	return nil, &ContextError{Message: fmt.Sprintf("file %s not found", src)}
}

func (recording *RecordingContext) FileDelete(path string) error {
	kept := make([]*model.File, 0, len(recording.records.Files))
	for _, file := range recording.records.Files {
		if file.Path != path {
			kept = append(kept, file)
		}
	}
	recording.records.Files = kept
	recording.record("file.delete", path)
	return nil
}

func (recording *RecordingContext) SectionCreate(file, name string, parent *string) (*model.Section, error) {
	path := name
	if parent != nil {
		path = *parent + "#" + name
	}
	section := &model.Section{Name: name, Path: path, FilePath: file}
	recording.records.Sections = append(recording.records.Sections, section)
	recording.record("section.create", []string{file, name})
	return section, nil
}

func (recording *RecordingContext) SectionMove(file, oldName, newName string) (*model.Section, error) {
	for _, section := range recording.records.Sections {
		if section.FilePath != file || section.Name != oldName {
			continue
		}
		section.Name = newName
		section.Path = newName
		recording.record("section.move", []string{file, oldName, newName})
		return section, nil
	}
	return nil, &ContextError{Message: fmt.Sprintf("section %s not found in %s", oldName, file)}
}

func (recording *RecordingContext) SectionDelete(file, name string) error {
	kept := make([]*model.Section, 0, len(recording.records.Sections))
	for _, section := range recording.records.Sections {
		if section.FilePath == file && section.Name == name {
			continue
		}
		kept = append(kept, section)
	}
	recording.records.Sections = kept
	recording.record("section.delete", []string{file, name})
	return nil
}

/** 🔤️ The optional strings one recorded write reports verbatim. */
func recordedOptionals(values ...*string) []interface{} {
	rows := make([]interface{}, 0, len(values))
	for _, value := range values {
		if value == nil {
			rows = append(rows, nil)
			continue
		}
		rows = append(rows, *value)
	}
	return rows
}

func (recording *RecordingContext) Integrate(source, targetSection, targetFile, targetParent *string) (*model.File, error) {
	recording.record("integrate", recordedOptionals(source, targetSection, targetFile, targetParent))
	target := ""
	if targetFile != nil {
		target = *targetFile
	}
	for _, file := range recording.records.Files {
		if file.Path == target {
			return file, nil
		}
	}
	return nil, &ContextError{Message: fmt.Sprintf("file %s not found", target)}
}

func (recording *RecordingContext) Extract(sourceFile, sourceSection, targetFile *string) (*model.File, error) {
	recording.record("extract", recordedOptionals(sourceFile, sourceSection, targetFile))
	target := ""
	if targetFile != nil {
		target = *targetFile
	}
	for _, file := range recording.records.Files {
		if file.Path == target {
			return file, nil
		}
	}
	return recording.FileCreate(target)
}

func (recording *RecordingContext) SyncManagement() (bool, error) {
	recording.record("sync.management", nil)
	return true, nil
}

// #endregion 🩻️RecordingContext

// #endregion 🚚️Split
