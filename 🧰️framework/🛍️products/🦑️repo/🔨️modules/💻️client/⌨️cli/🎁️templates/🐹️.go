// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Runtime text and markdown templates with their initialization.

// #endregion 🧲️Header

package client

import (
	"fmt"
	templatefunc "github.com/usalu/semio/repo/client/internal/templatefunc"
	"strings"
	"text/template"
)

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
  {{ colorize "breach" "red" .IsTTY }} {{ .Kind }} {{ colorize (printf "%s:%s" .Scope .Line) "dim" .IsTTY }} {{ .Summary }}
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

var (
	textTpl *template.Template
	mdTpl   *template.Template
)

// 💿️templateFuncMap holds the data fields for a templateFuncMap record.
func templateFuncMap() template.FuncMap {
	fm := templatefunc.TxtFuncMap()
	fm["colorize"] = func(s, color string, isTTY bool) string {
		return colorize(s, colorNameToANSI(color), isTTY)
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
	fm["pathToUriPath"] = PathToUriPath
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
	textTpl = template.Must(template.New("text").Funcs(templateFuncMap()).Parse(textTemplateContent))
	mdTpl = template.Must(template.New("md").Funcs(templateFuncMap()).Parse(markdownTemplateContent))
}

func renderTemplate(tpl *template.Template, name string, data interface{}) string {
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
