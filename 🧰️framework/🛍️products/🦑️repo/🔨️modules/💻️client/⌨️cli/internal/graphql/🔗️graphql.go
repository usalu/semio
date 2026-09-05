// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Package graphql provides the owned schema and deterministic query executor for the CLI.

// #endregion 🧲️Header

package graphql

import (
	"context"
	"fmt"
	"reflect"
	"strconv"
	"strings"
	"time"
	"unicode"
)

// #region 📜️Schema

type Type interface{ typeName() string }

type Scalar struct{ Name string }

func (scalar *Scalar) typeName() string { return scalar.Name }

var (
	String   = &Scalar{Name: "String"}
	Int      = &Scalar{Name: "Int"}
	Boolean  = &Scalar{Name: "Boolean"}
	ID       = &Scalar{Name: "ID"}
	DateTime = &Scalar{Name: "DateTime"}
)

type NonNull struct{ OfType Type }

func (value *NonNull) typeName() string { return value.OfType.typeName() }
func NewNonNull(value Type) *NonNull    { return &NonNull{OfType: value} }

type List struct{ OfType Type }

func (value *List) typeName() string { return value.OfType.typeName() }
func NewList(value Type) *List       { return &List{OfType: value} }

type ResolveParams struct {
	Source  interface{}
	Args    map[string]interface{}
	Context context.Context
}

type ResolveTypeParams struct {
	Value   interface{}
	Info    interface{}
	Context context.Context
}

type Field struct {
	Type    Type
	Args    FieldConfigArgument
	Resolve func(ResolveParams) (interface{}, error)
}

type Fields map[string]*Field
type FieldsThunk func() Fields

type ObjectConfig struct {
	Name       string
	Fields     interface{}
	Interfaces []*Interface
}
type Object struct {
	Name   string
	fields interface{}
}

func (object *Object) typeName() string     { return object.Name }
func NewObject(config ObjectConfig) *Object { return &Object{Name: config.Name, fields: config.Fields} }
func (object *Object) Fields() Fields {
	switch fields := object.fields.(type) {
	case Fields:
		return fields
	case FieldsThunk:
		return fields()
	default:
		return Fields{}
	}
}

type InterfaceConfig struct {
	Name        string
	Fields      interface{}
	ResolveType func(ResolveTypeParams) *Object
}
type Interface struct {
	Name        string
	fields      interface{}
	ResolveType func(ResolveTypeParams) *Object
}

func (value *Interface) typeName() string { return value.Name }
func NewInterface(config InterfaceConfig) *Interface {
	return &Interface{Name: config.Name, fields: config.Fields, ResolveType: config.ResolveType}
}

type UnionConfig struct {
	Name        string
	Types       []*Object
	ResolveType func(ResolveTypeParams) *Object
}
type Union struct {
	Name        string
	Types       []*Object
	ResolveType func(ResolveTypeParams) *Object
}

func (value *Union) typeName() string { return value.Name }
func NewUnion(config UnionConfig) *Union {
	return &Union{Name: config.Name, Types: config.Types, ResolveType: config.ResolveType}
}

type EnumValueConfig struct{ Value interface{} }
type EnumValueConfigMap map[string]*EnumValueConfig
type EnumConfig struct {
	Name   string
	Values EnumValueConfigMap
}
type Enum struct {
	Name   string
	Values EnumValueConfigMap
}

func (value *Enum) typeName() string  { return value.Name }
func NewEnum(config EnumConfig) *Enum { return &Enum{Name: config.Name, Values: config.Values} }

type InputObjectFieldConfig struct{ Type Type }
type InputObjectConfigFieldMap map[string]*InputObjectFieldConfig
type InputObjectConfig struct {
	Name   string
	Fields InputObjectConfigFieldMap
}
type InputObject struct {
	Name   string
	Fields InputObjectConfigFieldMap
}

func (value *InputObject) typeName() string { return value.Name }
func NewInputObject(config InputObjectConfig) *InputObject {
	return &InputObject{Name: config.Name, Fields: config.Fields}
}

type ArgumentConfig struct {
	Type         Type
	DefaultValue interface{}
}
type FieldConfigArgument map[string]*ArgumentConfig

type SchemaConfig struct{ Query, Mutation *Object }
type Schema struct{ Query, Mutation *Object }

func NewSchema(config SchemaConfig) (Schema, error) {
	if config.Query == nil {
		return Schema{}, fmt.Errorf("query schema is required")
	}
	return Schema{Query: config.Query, Mutation: config.Mutation}, nil
}

// #endregion 📜️Schema

// #region 🔤️Syntax

type tokenKind int

const (
	tokenEOF tokenKind = iota
	tokenName
	tokenString
	tokenNumber
	tokenPunct
)

type token struct {
	kind   tokenKind
	text   string
	offset int
}

type lexer struct {
	source string
	offset int
}

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

type parser struct {
	lexer   lexer
	current token
	err     error
}

func newParser(source string) *parser {
	parser := &parser{lexer: lexer{source: source}}
	parser.advance()
	return parser
}
func (parser *parser) advance() {
	if parser.err != nil {
		return
	}
	parser.current, parser.err = parser.lexer.next()
}
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

type document struct {
	operation  string
	selections []selection
}
type selection struct {
	alias, name string
	arguments   map[string]value
	fields      []selection
}
type value struct {
	literal  interface{}
	variable string
	list     []value
	object   map[string]value
}

func parse(source string) (document, error) {
	parser := newParser(source)
	doc := document{operation: "query"}
	if parser.current.kind == tokenName && (parser.current.text == "query" || parser.current.text == "mutation") {
		doc.operation = parser.current.text
		parser.advance()
		if parser.current.kind == tokenName {
			parser.advance()
		}
		if parser.current.text == "(" {
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
	}
	selections, err := parser.selectionSet()
	if err != nil {
		return document{}, err
	}
	doc.selections = selections
	if parser.err != nil {
		return document{}, parser.err
	}
	if parser.current.kind != tokenEOF {
		return document{}, fmt.Errorf("unexpected token %q at %d", parser.current.text, parser.current.offset)
	}
	return doc, nil
}

func (parser *parser) selectionSet() ([]selection, error) {
	if err := parser.take("{"); err != nil {
		return nil, err
	}
	var selections []selection
	for parser.current.text != "}" {
		if parser.current.kind == tokenEOF {
			return nil, fmt.Errorf("unterminated selection set")
		}
		if parser.current.kind != tokenName {
			return nil, fmt.Errorf("expected field name at %d", parser.current.offset)
		}
		item := selection{name: parser.current.text, arguments: map[string]value{}}
		parser.advance()
		if parser.current.text == ":" {
			parser.advance()
			item.alias = item.name
			if parser.current.kind != tokenName {
				return nil, fmt.Errorf("expected aliased field name")
			}
			item.name = parser.current.text
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
				item.arguments[name] = argument
			}
			parser.advance()
		}
		for parser.current.text == "@" {
			parser.advance()
			if parser.current.kind == tokenName {
				parser.advance()
			}
			if parser.current.text == "(" {
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
		}
		if parser.current.text == "{" {
			fields, err := parser.selectionSet()
			if err != nil {
				return nil, err
			}
			item.fields = fields
		}
		selections = append(selections, item)
	}
	parser.advance()
	return selections, nil
}

func (parser *parser) value() (value, error) {
	current := parser.current
	switch {
	case current.text == "$":
		parser.advance()
		if parser.current.kind != tokenName {
			return value{}, fmt.Errorf("expected variable name")
		}
		result := value{variable: parser.current.text}
		parser.advance()
		return result, nil
	case current.text == "[":
		parser.advance()
		result := value{}
		for parser.current.text != "]" {
			item, err := parser.value()
			if err != nil {
				return value{}, err
			}
			result.list = append(result.list, item)
		}
		parser.advance()
		return result, nil
	case current.text == "{":
		parser.advance()
		result := value{object: map[string]value{}}
		for parser.current.text != "}" {
			if parser.current.kind != tokenName {
				return value{}, fmt.Errorf("expected object field")
			}
			name := parser.current.text
			parser.advance()
			if err := parser.take(":"); err != nil {
				return value{}, err
			}
			item, err := parser.value()
			if err != nil {
				return value{}, err
			}
			result.object[name] = item
		}
		parser.advance()
		return result, nil
	case current.kind == tokenString:
		parser.advance()
		return value{literal: current.text}, nil
	case current.kind == tokenNumber:
		parser.advance()
		if strings.ContainsAny(current.text, ".eE") {
			parsed, err := strconv.ParseFloat(current.text, 64)
			return value{literal: parsed}, err
		}
		parsed, err := strconv.Atoi(current.text)
		return value{literal: parsed}, err
	case current.kind == tokenName:
		parser.advance()
		switch current.text {
		case "true":
			return value{literal: true}, nil
		case "false":
			return value{literal: false}, nil
		case "null":
			return value{literal: nil}, nil
		}
		return value{literal: current.text}, nil
	default:
		return value{}, fmt.Errorf("invalid value at %d", current.offset)
	}
}

func (value value) resolve(variables map[string]interface{}) interface{} {
	if value.variable != "" {
		return variables[value.variable]
	}
	if value.list != nil {
		result := make([]interface{}, len(value.list))
		for index := range value.list {
			result[index] = value.list[index].resolve(variables)
		}
		return result
	}
	if value.object != nil {
		result := map[string]interface{}{}
		for key, item := range value.object {
			result[key] = item.resolve(variables)
		}
		return result
	}
	return value.literal
}

func Validate(source string) error { _, err := parse(source); return err }
func OperationType(source string) (string, error) {
	doc, err := parse(source)
	return doc.operation, err
}

// #endregion 🔤️Syntax

// #region ⚡️Execution

type Params struct {
	Context        context.Context
	Schema         Schema
	RequestString  string
	VariableValues map[string]interface{}
}

type Result struct {
	Data   interface{} `json:"data,omitempty"`
	Errors []error     `json:"errors,omitempty"`
}

func Do(params Params) *Result {
	doc, err := parse(params.RequestString)
	if err != nil {
		return &Result{Errors: []error{err}}
	}
	root := params.Schema.Query
	if doc.operation == "mutation" {
		root = params.Schema.Mutation
	}
	if root == nil {
		return &Result{Errors: []error{fmt.Errorf("%s root is not configured", doc.operation)}}
	}
	ctx := params.Context
	if ctx == nil {
		ctx = context.Background()
	}
	data, err := executeSelections(ctx, nil, root, doc.selections, params.VariableValues)
	if err != nil {
		return &Result{Errors: []error{err}}
	}
	return &Result{Data: data}
}

func executeSelections(ctx context.Context, source interface{}, object *Object, selections []selection, variables map[string]interface{}) (map[string]interface{}, error) {
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	result := map[string]interface{}{}
	fields := object.Fields()
	for _, selected := range selections {
		if selected.name == "__typename" {
			result[fieldKey(selected)] = object.Name
			continue
		}
		field := fields[selected.name]
		if field == nil {
			return nil, fmt.Errorf("unknown field %q on %s", selected.name, object.Name)
		}
		args := map[string]interface{}{}
		for name, raw := range selected.arguments {
			args[name] = raw.resolve(variables)
		}
		for name, config := range field.Args {
			if _, ok := args[name]; !ok && config.DefaultValue != nil {
				args[name] = config.DefaultValue
			}
		}
		var value interface{}
		var err error
		if field.Resolve != nil {
			value, err = field.Resolve(ResolveParams{Source: source, Args: args, Context: ctx})
		} else {
			value = resolveDefault(source, selected.name)
		}
		if err != nil {
			return nil, fmt.Errorf("%s: %w", selected.name, err)
		}
		projected, err := project(ctx, value, field.Type, selected.fields, variables)
		if err != nil {
			return nil, fmt.Errorf("%s: %w", selected.name, err)
		}
		result[fieldKey(selected)] = projected
	}
	return result, nil
}

func fieldKey(selection selection) string {
	if selection.alias != "" {
		return selection.alias
	}
	return selection.name
}

func project(ctx context.Context, value interface{}, kind Type, selections []selection, variables map[string]interface{}) (interface{}, error) {
	if value == nil {
		return nil, nil
	}
	switch typed := kind.(type) {
	case *NonNull:
		return project(ctx, value, typed.OfType, selections, variables)
	case *List:
		reflected := reflect.ValueOf(value)
		if reflected.Kind() != reflect.Slice && reflected.Kind() != reflect.Array {
			return nil, fmt.Errorf("expected list, got %T", value)
		}
		result := make([]interface{}, reflected.Len())
		for index := range result {
			item, err := project(ctx, reflected.Index(index).Interface(), typed.OfType, selections, variables)
			if err != nil {
				return nil, err
			}
			result[index] = item
		}
		return result, nil
	case *Object:
		if len(selections) == 0 {
			return value, nil
		}
		return executeSelections(ctx, value, typed, selections, variables)
	case *Interface:
		object := typed.ResolveType(ResolveTypeParams{Value: value, Context: ctx})
		if object == nil {
			return nil, fmt.Errorf("cannot resolve interface %s", typed.Name)
		}
		return executeSelections(ctx, value, object, selections, variables)
	case *Union:
		object := typed.ResolveType(ResolveTypeParams{Value: value, Context: ctx})
		if object == nil {
			return nil, fmt.Errorf("cannot resolve union %s", typed.Name)
		}
		return executeSelections(ctx, value, object, selections, variables)
	case *Scalar:
		if typed == DateTime {
			if timestamp, ok := value.(time.Time); ok {
				return timestamp.Format(time.RFC3339), nil
			}
		}
		return value, nil
	case *Enum:
		for name, enumValue := range typed.Values {
			if reflect.DeepEqual(value, enumValue.Value) {
				return name, nil
			}
		}
		return value, nil
	default:
		return value, nil
	}
}

func resolveDefault(source interface{}, name string) interface{} {
	if source == nil {
		return nil
	}
	if object, ok := source.(map[string]interface{}); ok {
		return object[name]
	}
	value := reflect.ValueOf(source)
	for value.Kind() == reflect.Pointer {
		if value.IsNil() {
			return nil
		}
		value = value.Elem()
	}
	if value.Kind() != reflect.Struct {
		return nil
	}
	typeInfo := value.Type()
	canonical := canonicalName(name)
	for index := 0; index < value.NumField(); index++ {
		field := typeInfo.Field(index)
		jsonName := strings.Split(field.Tag.Get("json"), ",")[0]
		if canonicalName(field.Name) == canonical || canonicalName(jsonName) == canonical {
			return value.Field(index).Interface()
		}
	}
	return nil
}

func canonicalName(value string) string {
	return strings.ToLower(strings.NewReplacer("_", "", "-", "").Replace(value))
}

// #endregion ⚡️Execution
