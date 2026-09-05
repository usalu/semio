// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Package yaml provides a bounded YAML data decoder and deterministic YAML-compatible encoder.

// #endregion 🧲️Header

package yaml

import (
	"bufio"
	"bytes"
	"encoding/json"
	"fmt"
	"reflect"
	"strconv"
	"strings"
)

// #region 📤️Encoding

func Marshal(value interface{}) ([]byte, error) {
	normalized := normalize(reflect.ValueOf(value))
	var output strings.Builder
	if err := emit(&output, normalized, 0); err != nil {
		return nil, err
	}
	return []byte(output.String()), nil
}

func normalize(value reflect.Value) interface{} {
	if !value.IsValid() {
		return nil
	}
	if value.Kind() == reflect.Pointer {
		if value.IsNil() {
			return nil
		}
		return normalize(value.Elem())
	}
	switch value.Kind() {
	case reflect.Struct:
		result := map[string]interface{}{}
		typeInfo := value.Type()
		for index := 0; index < value.NumField(); index++ {
			field := typeInfo.Field(index)
			name, options := fieldName(field)
			if name == "-" || field.PkgPath != "" {
				continue
			}
			fieldValue := value.Field(index)
			if options["omitempty"] && fieldValue.IsZero() {
				continue
			}
			result[name] = normalize(fieldValue)
		}
		return result
	case reflect.Map:
		result := map[string]interface{}{}
		iterator := value.MapRange()
		for iterator.Next() {
			result[fmt.Sprint(iterator.Key().Interface())] = normalize(iterator.Value())
		}
		return result
	case reflect.Slice, reflect.Array:
		result := make([]interface{}, value.Len())
		for index := range result {
			result[index] = normalize(value.Index(index))
		}
		return result
	case reflect.String:
		return value.String()
	case reflect.Bool:
		return value.Bool()
	case reflect.Int, reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64:
		return value.Int()
	case reflect.Uint, reflect.Uint8, reflect.Uint16, reflect.Uint32, reflect.Uint64:
		return value.Uint()
	case reflect.Float32, reflect.Float64:
		return value.Float()
	case reflect.Interface:
		if value.IsNil() {
			return nil
		}
		return normalize(value.Elem())
	default:
		return value.Interface()
	}
}

func emit(output *strings.Builder, value interface{}, indent int) error {
	prefix := strings.Repeat(" ", indent)
	switch item := value.(type) {
	case map[string]interface{}:
		keys := make([]string, 0, len(item))
		for key := range item {
			keys = append(keys, key)
		}
		slicesSort(keys)
		for _, key := range keys {
			child := item[key]
			if isContainer(child) {
				fmt.Fprintf(output, "%s%s:\n", prefix, key)
				if err := emit(output, child, indent+2); err != nil {
					return err
				}
			} else {
				fmt.Fprintf(output, "%s%s: %s\n", prefix, key, scalarString(child))
			}
		}
	case []interface{}:
		for _, child := range item {
			if isContainer(child) {
				fmt.Fprintf(output, "%s-\n", prefix)
				if err := emit(output, child, indent+2); err != nil {
					return err
				}
			} else {
				fmt.Fprintf(output, "%s- %s\n", prefix, scalarString(child))
			}
		}
	default:
		fmt.Fprintf(output, "%s%s\n", prefix, scalarString(item))
	}
	return nil
}

func isContainer(value interface{}) bool {
	switch value.(type) {
	case map[string]interface{}, []interface{}:
		return true
	}
	return false
}

func scalarString(value interface{}) string {
	if value == nil {
		return "null"
	}
	if text, ok := value.(string); ok {
		if text == "" {
			return `""`
		}
		if strings.ContainsAny(text, ":#{}[],&*!|>'\"%@`\n\r\t") || strings.TrimSpace(text) != text {
			encoded, _ := json.Marshal(text)
			return string(encoded)
		}
		return text
	}
	return fmt.Sprint(value)
}

// #endregion 📤️Encoding

// #region 📥️Decoding

type line struct {
	indent int
	text   string
}

func Unmarshal(data []byte, target interface{}) error {
	if target == nil || reflect.ValueOf(target).Kind() != reflect.Pointer {
		return fmt.Errorf("yaml target must be a non-nil pointer")
	}
	var node interface{}
	if err := json.Unmarshal(data, &node); err != nil {
		lines, scanErr := scan(data)
		if scanErr != nil {
			return scanErr
		}
		if len(lines) == 0 {
			node = map[string]interface{}{}
		} else {
			var next int
			node, next, err = parseBlock(lines, 0, lines[0].indent)
			if err != nil {
				return err
			}
			if next != len(lines) {
				return fmt.Errorf("yaml contains an unexpected block at line %d", next+1)
			}
		}
	}
	return assign(reflect.ValueOf(target).Elem(), node)
}

func scan(data []byte) ([]line, error) {
	scanner := bufio.NewScanner(bytes.NewReader(data))
	scanner.Buffer(make([]byte, 64*1024), 8*1024*1024)
	var lines []line
	for scanner.Scan() {
		raw := strings.TrimRight(scanner.Text(), " \t\r")
		trimmed := strings.TrimSpace(raw)
		if trimmed == "" || strings.HasPrefix(trimmed, "#") || trimmed == "---" || trimmed == "..." {
			continue
		}
		indent := len(raw) - len(strings.TrimLeft(raw, " "))
		if strings.Contains(raw[:indent], "\t") {
			return nil, fmt.Errorf("yaml tabs are not allowed")
		}
		lines = append(lines, line{indent: indent, text: strings.TrimSpace(stripComment(strings.TrimSpace(raw)))})
	}
	return lines, scanner.Err()
}

func stripComment(value string) string {
	quoted := byte(0)
	for index := 0; index < len(value); index++ {
		if value[index] == '\'' || value[index] == '"' {
			if quoted == 0 {
				quoted = value[index]
			} else if quoted == value[index] {
				quoted = 0
			}
		}
		if value[index] == '#' && quoted == 0 && (index == 0 || value[index-1] == ' ') {
			return strings.TrimSpace(value[:index])
		}
	}
	return value
}

func parseBlock(lines []line, start, indent int) (interface{}, int, error) {
	if start >= len(lines) {
		return map[string]interface{}{}, start, nil
	}
	if strings.HasPrefix(lines[start].text, "-") {
		return parseSequence(lines, start, indent)
	}
	result := map[string]interface{}{}
	index := start
	for index < len(lines) && lines[index].indent == indent && !strings.HasPrefix(lines[index].text, "-") {
		key, raw, found := strings.Cut(lines[index].text, ":")
		if !found || strings.TrimSpace(key) == "" {
			return nil, index, fmt.Errorf("invalid yaml mapping at line %d", index+1)
		}
		key, raw = strings.TrimSpace(key), strings.TrimSpace(raw)
		index++
		if raw == "" {
			if index < len(lines) && lines[index].indent > indent {
				child, next, err := parseBlock(lines, index, lines[index].indent)
				if err != nil {
					return nil, next, err
				}
				result[key], index = child, next
			} else {
				result[key] = map[string]interface{}{}
			}
		} else {
			result[key] = parseScalar(raw)
		}
	}
	return result, index, nil
}

func parseSequence(lines []line, start, indent int) (interface{}, int, error) {
	var result []interface{}
	index := start
	for index < len(lines) && lines[index].indent == indent && strings.HasPrefix(lines[index].text, "-") {
		raw := strings.TrimSpace(strings.TrimPrefix(lines[index].text, "-"))
		index++
		if raw == "" {
			if index < len(lines) && lines[index].indent > indent {
				child, next, err := parseBlock(lines, index, lines[index].indent)
				if err != nil {
					return nil, next, err
				}
				result, index = append(result, child), next
			} else {
				result = append(result, nil)
			}
			continue
		}
		if key, value, found := strings.Cut(raw, ":"); found {
			object := map[string]interface{}{strings.TrimSpace(key): parseScalar(strings.TrimSpace(value))}
			if index < len(lines) && lines[index].indent > indent {
				child, next, err := parseBlock(lines, index, lines[index].indent)
				if err != nil {
					return nil, next, err
				}
				if fields, ok := child.(map[string]interface{}); ok {
					for childKey, childValue := range fields {
						object[childKey] = childValue
					}
				}
				index = next
			}
			result = append(result, object)
		} else {
			result = append(result, parseScalar(raw))
		}
	}
	return result, index, nil
}

func parseScalar(raw string) interface{} {
	if raw == "" {
		return ""
	}
	if (strings.HasPrefix(raw, `"`) && strings.HasSuffix(raw, `"`)) || (strings.HasPrefix(raw, `'`) && strings.HasSuffix(raw, `'`)) {
		if raw[0] == '\'' {
			return strings.ReplaceAll(raw[1:len(raw)-1], "''", "'")
		}
		var value string
		if json.Unmarshal([]byte(raw), &value) == nil {
			return value
		}
	}
	if strings.HasPrefix(raw, "[") && strings.HasSuffix(raw, "]") {
		body := strings.TrimSpace(raw[1 : len(raw)-1])
		if body == "" {
			return []interface{}{}
		}
		parts := strings.Split(body, ",")
		result := make([]interface{}, len(parts))
		for index, part := range parts {
			result[index] = parseScalar(strings.TrimSpace(part))
		}
		return result
	}
	switch strings.ToLower(raw) {
	case "true":
		return true
	case "false":
		return false
	case "null", "~":
		return nil
	}
	if value, err := strconv.ParseInt(raw, 10, 64); err == nil {
		return value
	}
	if value, err := strconv.ParseFloat(raw, 64); err == nil {
		return value
	}
	return raw
}

// #endregion 📥️Decoding

// #region 🪞️Reflection

func assign(target reflect.Value, source interface{}) error {
	if !target.CanSet() {
		return nil
	}
	if source == nil {
		target.SetZero()
		return nil
	}
	if target.Kind() == reflect.Pointer {
		if target.IsNil() {
			target.Set(reflect.New(target.Type().Elem()))
		}
		return assign(target.Elem(), source)
	}
	switch target.Kind() {
	case reflect.Interface:
		target.Set(reflect.ValueOf(source))
		return nil
	case reflect.Struct:
		object, ok := source.(map[string]interface{})
		if !ok {
			return fmt.Errorf("cannot decode %T into %s", source, target.Type())
		}
		typeInfo := target.Type()
		for index := 0; index < target.NumField(); index++ {
			field := typeInfo.Field(index)
			name, _ := fieldName(field)
			if name == "-" {
				continue
			}
			value, found := object[name]
			if !found {
				for key, candidate := range object {
					if canonical(key) == canonical(name) || canonical(key) == canonical(field.Name) {
						value, found = candidate, true
						break
					}
				}
			}
			if found {
				if err := assign(target.Field(index), value); err != nil {
					return fmt.Errorf("%s: %w", name, err)
				}
			}
		}
	case reflect.Map:
		object, ok := source.(map[string]interface{})
		if !ok {
			return fmt.Errorf("cannot decode %T into map", source)
		}
		target.Set(reflect.MakeMap(target.Type()))
		for key, value := range object {
			mapKey := reflect.New(target.Type().Key()).Elem()
			mapValue := reflect.New(target.Type().Elem()).Elem()
			if err := assign(mapKey, key); err != nil {
				return err
			}
			if err := assign(mapValue, value); err != nil {
				return err
			}
			target.SetMapIndex(mapKey, mapValue)
		}
	case reflect.Slice:
		sequence, ok := source.([]interface{})
		if !ok {
			return fmt.Errorf("cannot decode %T into slice", source)
		}
		target.Set(reflect.MakeSlice(target.Type(), len(sequence), len(sequence)))
		for index := range sequence {
			if err := assign(target.Index(index), sequence[index]); err != nil {
				return err
			}
		}
	case reflect.String:
		target.SetString(fmt.Sprint(source))
	case reflect.Bool:
		value, ok := source.(bool)
		if !ok {
			return fmt.Errorf("cannot decode %T into bool", source)
		}
		target.SetBool(value)
	case reflect.Int, reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64:
		switch value := source.(type) {
		case int64:
			target.SetInt(value)
		case float64:
			target.SetInt(int64(value))
		default:
			parsed, err := strconv.ParseInt(fmt.Sprint(source), 10, 64)
			if err != nil {
				return err
			}
			target.SetInt(parsed)
		}
	case reflect.Uint, reflect.Uint8, reflect.Uint16, reflect.Uint32, reflect.Uint64:
		parsed, err := strconv.ParseUint(fmt.Sprint(source), 10, 64)
		if err != nil {
			return err
		}
		target.SetUint(parsed)
	case reflect.Float32, reflect.Float64:
		parsed, err := strconv.ParseFloat(fmt.Sprint(source), 64)
		if err != nil {
			return err
		}
		target.SetFloat(parsed)
	default:
		return fmt.Errorf("unsupported yaml target %s", target.Kind())
	}
	return nil
}

func fieldName(field reflect.StructField) (string, map[string]bool) {
	tag := field.Tag.Get("yaml.go")
	if tag == "" {
		tag = field.Tag.Get("json")
	}
	parts := strings.Split(tag, ",")
	name := parts[0]
	if name == "" {
		name = field.Name
	}
	options := map[string]bool{}
	for _, option := range parts[1:] {
		options[option] = true
	}
	return name, options
}

func canonical(value string) string {
	return strings.ToLower(strings.NewReplacer("-", "", "_", "").Replace(value))
}

func slicesSort(values []string) {
	for index := 1; index < len(values); index++ {
		for cursor := index; cursor > 0 && values[cursor] < values[cursor-1]; cursor-- {
			values[cursor], values[cursor-1] = values[cursor-1], values[cursor]
		}
	}
}

// #endregion 🪞️Reflection
