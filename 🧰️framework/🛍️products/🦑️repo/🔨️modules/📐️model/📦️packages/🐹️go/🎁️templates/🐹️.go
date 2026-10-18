// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎁️templates carries the text and markdown templates and their function maps.

// #endregion 🧲️Header

package model

import (
	json "encoding/json"
	fmt "fmt"
	strconv "strconv"
	strings "strings"
	template "text/template"

	workspace "github.com/usalu/semio/repo/workspace"
)

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

// #endregion 🎁️Templates
