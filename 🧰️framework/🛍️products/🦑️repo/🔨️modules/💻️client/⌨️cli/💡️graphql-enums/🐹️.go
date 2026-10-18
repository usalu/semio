// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// GraphQL domain enums, allowed values, and normalization.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"strings"
)

// 🔌️Node defines the interface contract for node operations.
type Node interface {
	IsNode()
	GetID() string
}

// 📖️DefinitionKind represents a definition kind value.
type DefinitionKind string

const (
	DefinitionKindImplementation DefinitionKind = "implementation"
	DefinitionKindInterface      DefinitionKind = "interface"
	DefinitionKindConstant       DefinitionKind = "constant"
	DefinitionKindTest           DefinitionKind = "test"
)

// 🔷️IsValid MUST return true only when the condition is met.
// ▶️IsValid reports whether the DefinitionKind is valid.
func (e DefinitionKind) IsValid() bool {
	switch e {
	case DefinitionKindImplementation, DefinitionKindInterface, DefinitionKindConstant, DefinitionKindTest:
		return true
	}
	return false
}

// 🔤️String MUST return the canonical string value.
// 🔤️String returns the string representation of the DefinitionKind.
func (e DefinitionKind) String() string {
	return string(e)
}

// 📝️DeriveDefinitionKind MUST return a valid value for any recognized input.
// 💾️DeriveDefinitionKind infers and returns the definition kind from the given input.
func DeriveDefinitionKind(rawKind string) DefinitionKind {
	switch strings.ToLower(rawKind) {
	case "interface", "type", "trait", "abstract",
		"extend type", "extend interface", "extend enum", "extend union", "extend input",
		"input", "union", "scalar",
		"delegate", "record":
		return DefinitionKindInterface
	case "const", "constant", "enum", "var", "let", "static":
		return DefinitionKindConstant
	case "test":
		return DefinitionKindTest
	default:
		return DefinitionKindImplementation
	}
}

// 🎫️TicketStatus represents a ticket status value.
type TicketStatus string

const (
	TicketStatusOpen   TicketStatus = "open"
	TicketStatusClosed TicketStatus = "closed"
)

// 🔶️IsValid MUST return true only when the condition is met.
// 🔑️IsValid reports whether the TicketStatus is valid.
func (e TicketStatus) IsValid() bool {
	switch e {
	case TicketStatusOpen, TicketStatusClosed:
		return true
	}
	return false
}

// 🔹️String MUST return the canonical string value.
// 📩️String returns the string representation of the TicketStatus.
func (e TicketStatus) String() string {
	return string(e)
}

// 🔸️BreachPriority represents a breach priority value.
type BreachPriority string

const (
	BreachPriorityHigh   BreachPriority = "high"
	BreachPriorityMedium BreachPriority = "medium"
	BreachPriorityLow    BreachPriority = "low"
)

// 🔺️IsValid MUST return true only when the condition is met.
// ⚠️IsValid reports whether the BreachPriority is valid.
func (e BreachPriority) IsValid() bool {
	switch e {
	case BreachPriorityHigh, BreachPriorityMedium, BreachPriorityLow:
		return true
	}
	return false
}

// 🔻️String MUST return the canonical string value.
// 🔷️String returns the string representation of the BreachPriority.
func (e BreachPriority) String() string {
	return string(e)
}

// ⬛️AllowedLLMs holds the allowed LLM values across providers.
var AllowedLLMs = []string{
	"opus-4-7",
	"opus-4-6",
	"opus-4-5",
	"opus-4-1",
	"opus-4",
	"opus-3",
	"sonnet-5",
	"sonnet-4-6",
	"sonnet-4-5",
	"sonnet-4",
	"sonnet-3-7",
	"sonnet-3-5",
	"haiku-4-5",
	"haiku-4",
	"haiku-3-5",
	"claude-3-7-sonnet",
	"claude-3-5-sonnet",
	"claude-3-5-haiku",
	"claude-3-opus",
	"gemini-3-7-pro",
	"gemini-3-7-flash",
	"gemini-3-1-pro",
	"gemini-3-pro",
	"gemini-3-flash",
	"gemini-2-5-pro",
	"gemini-2-5-flash",
	"gemini-2-5-flash-lite",
	"gemini-2-pro",
	"gemini-2-flash",
	"gemini-2-flash-lite",
	"gemini-1-5-pro",
	"gemini-1-5-flash",
	"gpt-5-5",
	"gpt-5-4",
	"gpt-5-3-codex",
	"gpt-5-2-codex",
	"gpt-5-2",
	"gpt-5-1-codex",
	"gpt-5-1",
	"gpt-5-codex",
	"gpt-5",
	"gpt-5-mini",
	"o3-pro",
	"o3-mini",
	"o3",
	"o1-pro",
	"o1-mini",
	"o1-preview",
	"o1",
	"gpt-4-5",
	"gpt-4o",
	"gpt-4o-mini",
	"gpt-4",
	"codex",
	"grok-4-5",
	"grok-4",
	"grok-3-mini",
	"grok-3",
	"grok-2",
	"cursor-grok-4-5",
	"cursor-grok-4",
	"composer-2-5",
	"composer-2",
	"composer-1-5",
	"composer",
	"deepseek-r1",
	"deepseek-v3",
	"deepseek-coder",
	"llama-4",
	"llama-3-3",
	"llama-3-2",
	"llama-3-1",
	"llama-3",
	"qwen-3",
	"qwen-2-5",
	"swe-1-5",
	"swe-1",
}

// 🏋️AllowedEfforts holds the allowed reasoning effort values for LLMs.
var AllowedEfforts = []string{
	"low",
	"medium",
	"high",
	"max",
}

// 💻️AllowedClients holds the allowed clients values.
var AllowedClients = []string{
	"vscode",
	"copilot-chat",
	"windsurf",
	"windsurf-chat",
	"claude-code",
	"codex",
	"cursor",
	"cursor-chat",
	"antigravity",
	"antigravity-chat",
	"droid",
	"kiro-cli",
}

// ⬜️NormalizeLLMSlug MUST be idempotent for already-normalized values.
// 📝️NormalizeLLMSlug normalizes the l l m slug to its canonical form.
func NormalizeLLMSlug(llm string) string {
	return strings.ToLower(Slugify(llm))
}

// 🏋️NormalizeEffortSlug normalizes the reasoning effort slug to its canonical form.
func NormalizeEffortSlug(effort string) string {
	return strings.ToLower(Slugify(effort))
}

// 🟥️NormalizeClientSlug MUST be idempotent for already-normalized values.
// ❓️NormalizeClientSlug normalizes the client slug to its canonical form.
func NormalizeClientSlug(client string) string {
	return strings.ToLower(Slugify(client))
}

// ❌️ResolveAllowedLLM MUST return an error for unrecognized values.
// 🔶️ResolveAllowedLLM resolves and validates the allowed l l m against known values.
func ResolveAllowedLLM(llm string) (string, error) {
	llmSlug := NormalizeLLMSlug(llm)
	bestMatch := ""
	for _, allowed := range AllowedLLMs {
		if strings.Contains(llmSlug, NormalizeLLMSlug(allowed)) {
			if len(allowed) > len(bestMatch) {
				bestMatch = allowed
			}
		}
	}
	if bestMatch == "" {
		return "", fmt.Errorf("llm '%s' is not allowed. Please use one of: %s", llmSlug, strings.Join(AllowedLLMs, ", "))
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
	for _, allowed := range AllowedEfforts {
		if effortSlug == NormalizeEffortSlug(allowed) || strings.Contains(effortSlug, NormalizeEffortSlug(allowed)) {
			if len(allowed) > len(bestMatch) {
				bestMatch = allowed
			}
		}
	}
	if bestMatch == "" {
		return "", fmt.Errorf("effort '%s' is not allowed. Please use one of: %s", effortSlug, strings.Join(AllowedEfforts, ", "))
	}
	return bestMatch, nil
}

// 🟧️ResolveAllowedClient MUST return an error for unrecognized values.
// 🔹️ResolveAllowedClient resolves and validates the allowed client against known values.
func ResolveAllowedClient(client string) (string, error) {
	uiSlug := NormalizeClientSlug(client)
	bestMatch := ""
	for _, allowed := range AllowedClients {
		if strings.Contains(uiSlug, NormalizeClientSlug(allowed)) {
			if len(allowed) > len(bestMatch) {
				bestMatch = allowed
			}
		}
	}
	if bestMatch == "" {
		return "", fmt.Errorf("client '%s' is not allowed. Please use one of: %s", uiSlug, strings.Join(AllowedClients, ", "))
	}
	return bestMatch, nil
}
