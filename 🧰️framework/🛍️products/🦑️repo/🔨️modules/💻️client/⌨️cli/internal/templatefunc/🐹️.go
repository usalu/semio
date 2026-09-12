// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Package templatefunc defines the owned text-template function vocabulary.

// #endregion 🧲️Header

package templatefunc

import (
	"encoding/json"
	"fmt"
	"strconv"
	"strings"
	"text/template"
)

// #region 🎁️Functions

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

// #endregion 🎁️Functions
