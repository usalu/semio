// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🌳️ast carries the GraphQL document syntax tree.

// #endregion 🧲️Header

package graphql

import (
	sort "sort"
)

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
