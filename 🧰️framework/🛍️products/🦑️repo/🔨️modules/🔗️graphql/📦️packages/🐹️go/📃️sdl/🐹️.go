// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📃️sdl carries the schema definition language printer.

// #endregion 🧲️Header

package graphql

import (
	slices "slices"
	sort "sort"
	strings "strings"
)

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
