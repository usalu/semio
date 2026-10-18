// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// ⚡️execution carries the deterministic GraphQL execution engine.

// #endregion 🧲️Header

package graphql

import (
	context "context"
	json "encoding/json"
	fmt "fmt"
	sort "sort"
	strings "strings"
	unicode "unicode"

	model "github.com/usalu/semio/repo/model"
)

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
