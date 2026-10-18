// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

// 📐️Package model holds the pure data shapes of the 🦑️repo product together with the slug
// 📐️vocabularies and the derivations that follow from the shape alone. Nothing in it touches
// 📐️the filesystem, a process or a clock. The wire contract is 🧬️schema/🔣️.json and the twin
// 📐️implementation is 📦️packages/🦀️rust.
package model

import (
	json "encoding/json"
	errors "errors"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	runtime "runtime"
	strings "strings"
	sync "sync"

	workspace "github.com/usalu/semio/repo/workspace"
)

// 📋️AllowedValuesEnvironment overrides the module-relative lookup of the shared vocabulary table.
const AllowedValuesEnvironment = "SEMIO_REPO_MODEL_ALLOWED_VALUES"

// 📋️AllowedValues mirrors 🧬️schema/🔣️allowed-values.json, the vocabulary table the Rust twin
// 📋️embeds with include_str!. Neither implementation re-declares these strings in source.
type AllowedValues struct {
	LLMs    []string `json:"llms"`
	Efforts []string `json:"efforts"`
	Clients []string `json:"clients"`
}

var (
	allowedOnce  sync.Once
	allowedTable AllowedValues
	allowedError error
)

// 🧭️allowedValuesPath resolves the table relative to this source file, because `go:embed` cannot
// reach a parent directory and the table is owned by the module, not by the Go package.
func allowedValuesPath() (string, error) {
	if override := strings.TrimSpace(os.Getenv(AllowedValuesEnvironment)); override != "" {
		return override, nil
	}
	if _, file, _, ok := runtime.Caller(0); ok {
		candidate := filepath.Join(filepath.Dir(file), "..", "..", "🧬️schema", "🔣️allowed-values.json")
		if _, err := os.Stat(candidate); err == nil {
			return filepath.Clean(candidate), nil
		}
	}
	suffix := filepath.Join("🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📐️model", "🧬️schema", "🔣️allowed-values.json")
	directory, err := os.Getwd()
	if err != nil {
		return "", err
	}
	for {
		candidate := filepath.Join(directory, suffix)
		if _, err := os.Stat(candidate); err == nil {
			return candidate, nil
		}
		parent := filepath.Dir(directory)
		if parent == directory {
			return "", errors.New("model: allowed-value table not found")
		}
		directory = parent
	}
}

// 📋️Allowed loads the shared vocabulary table once per process.
func Allowed() (AllowedValues, error) {
	allowedOnce.Do(func() {
		path, err := allowedValuesPath()
		if err != nil {
			allowedError = err
			return
		}
		data, err := os.ReadFile(path)
		if err != nil {
			allowedError = err
			return
		}
		allowedError = json.Unmarshal(data, &allowedTable)
	})
	return allowedTable, allowedError
}

// 🤖️AllowedLLMs returns the allowed LLM slugs, in declaration order.
func AllowedLLMs() ([]string, error) {
	table, err := Allowed()
	return table.LLMs, err
}

// 🏋️AllowedEfforts returns the allowed reasoning-effort slugs, in declaration order.
func AllowedEfforts() ([]string, error) {
	table, err := Allowed()
	return table.Efforts, err
}

// 💻️AllowedClients returns the allowed client slugs, in declaration order.
func AllowedClients() ([]string, error) {
	table, err := Allowed()
	return table.Clients, err
}

// ⬜️NormalizeLLMSlug MUST be idempotent for already-normalized values.
// 📝️NormalizeLLMSlug normalizes the l l m slug to its canonical form.
func NormalizeLLMSlug(llm string) string {
	return strings.ToLower(workspace.Slugify(llm))
}

// 🏋️NormalizeEffortSlug normalizes the reasoning effort slug to its canonical form.
func NormalizeEffortSlug(effort string) string {
	return strings.ToLower(workspace.Slugify(effort))
}

// 🟥️NormalizeClientSlug MUST be idempotent for already-normalized values.
// ❓️NormalizeClientSlug normalizes the client slug to its canonical form.
func NormalizeClientSlug(client string) string {
	return strings.ToLower(workspace.Slugify(client))
}

// ❌️ResolveAllowedLLM MUST return an error for unrecognized values.
// 🔶️ResolveAllowedLLM resolves and validates the allowed l l m against known values.
func ResolveAllowedLLM(llm string) (string, error) {
	llmSlug := NormalizeLLMSlug(llm)
	bestMatch := ""
	for _, allowed := range AllowedLLMList() {
		if strings.Contains(llmSlug, NormalizeLLMSlug(allowed)) {
			if len(allowed) > len(bestMatch) {
				bestMatch = allowed
			}
		}
	}
	if bestMatch == "" {
		return "", fmt.Errorf("llm '%s' is not allowed. Please use one of: %s", llmSlug, strings.Join(AllowedLLMList(), ", "))
	}
	return bestMatch, nil
}

// 🏋️ResolveAllowedEffort resolves and validates the allowed reasoning effort against known values.
func ResolveAllowedEffort(effort string) (string, error) {
	if strings.TrimSpace(effort) == "" {
		return "", nil
	}
	effortSlug := NormalizeEffortSlug(effort)
	bestMatch := ""
	for _, allowed := range AllowedEffortList() {
		if effortSlug == NormalizeEffortSlug(allowed) || strings.Contains(effortSlug, NormalizeEffortSlug(allowed)) {
			if len(allowed) > len(bestMatch) {
				bestMatch = allowed
			}
		}
	}
	if bestMatch == "" {
		return "", fmt.Errorf("effort '%s' is not allowed. Please use one of: %s", effortSlug, strings.Join(AllowedEffortList(), ", "))
	}
	return bestMatch, nil
}

// 🟧️ResolveAllowedClient MUST return an error for unrecognized values.
// 🔹️ResolveAllowedClient resolves and validates the allowed client against known values.
func ResolveAllowedClient(client string) (string, error) {
	uiSlug := NormalizeClientSlug(client)
	bestMatch := ""
	for _, allowed := range AllowedClientList() {
		if strings.Contains(uiSlug, NormalizeClientSlug(allowed)) {
			if len(allowed) > len(bestMatch) {
				bestMatch = allowed
			}
		}
	}
	if bestMatch == "" {
		return "", fmt.Errorf("client '%s' is not allowed. Please use one of: %s", uiSlug, strings.Join(AllowedClientList(), ", "))
	}
	return bestMatch, nil
}

// 🤖️allowedLLMList answers the allowed LLM slugs from the module's schema table.
func AllowedLLMList() []string {
	values, err := AllowedLLMs()
	if err != nil {
		return nil
	}
	return values
}

// 🏋️allowedEffortList answers the allowed reasoning-effort slugs from the module's schema table.
func AllowedEffortList() []string {
	values, err := AllowedEfforts()
	if err != nil {
		return nil
	}
	return values
}

// 💻️allowedClientList answers the allowed client slugs from the module's schema table.
func AllowedClientList() []string {
	values, err := AllowedClients()
	if err != nil {
		return nil
	}
	return values
}
