// 📋️ Tests of the allowed vocabulary.

package model

import (
	testing "testing"
)

func TestCurrentMcpLLMsAllowed(t *testing.T) {
	for _, llm := range []string{"opus-4-7", "gpt-5-5"} {
		if got, err := ResolveAllowedLLM(llm); err != nil || got != llm {
			t.Fatalf("ResolveAllowedLLM(%q) = %q, %v", llm, got, err)
		}
	}
}

// 🧠️TestAllowedLLMsCoverage tests resolution of all latest LLMs across providers.
func TestAllowedLLMsCoverage(t *testing.T) {
	testModels := []string{
		"opus-4-7", "opus-4-6", "opus-4-5", "opus-4-1", "opus-4", "opus-3",
		"sonnet-5", "sonnet-4-6", "sonnet-4-5", "sonnet-4", "sonnet-3-7", "sonnet-3-5",
		"haiku-4-5", "haiku-4", "haiku-3-5",
		"gemini-3-7-pro", "gemini-3-7-flash", "gemini-3-1-pro", "gemini-3-pro", "gemini-3-flash",
		"gemini-2-5-pro", "gemini-2-5-flash", "gemini-2-5-flash-lite", "gemini-2-pro", "gemini-2-flash", "gemini-2-flash-lite",
		"gpt-5-5", "gpt-5-4", "gpt-5-3-codex", "gpt-5-2-codex", "gpt-5-2", "gpt-5-1-codex", "gpt-5-1", "gpt-5-codex", "gpt-5", "gpt-5-mini",
		"o3-pro", "o3-mini", "o3", "o1-pro", "o1-mini", "o1-preview", "o1",
		"grok-4-5", "grok-4", "grok-3-mini", "grok-3", "grok-2", "cursor-grok-4-5", "cursor-grok-4",
		"composer-2-5", "composer-2", "composer-1-5", "composer",
		"deepseek-r1", "deepseek-v3", "deepseek-coder",
		"llama-4", "llama-3-3", "llama-3-2", "llama-3-1", "llama-3",
		"qwen-3", "qwen-2-5", "swe-1-5", "swe-1",
	}
	for _, model := range testModels {
		slug, err := ResolveAllowedLLM(model)
		if err != nil {
			t.Errorf("ResolveAllowedLLM(%q) failed: %v", model, err)
		}
		if slug == "" {
			t.Errorf("ResolveAllowedLLM(%q) returned empty slug", model)
		}
	}
}

// 🏋️TestResolveAllowedEffort tests reasoning effort resolution and normalization.
func TestResolveAllowedEffort(t *testing.T) {
	cases := []struct {
		input    string
		expected string
		wantErr  bool
	}{
		{"low", "low", false},
		{"Low", "low", false},
		{"MEDIUM", "medium", false},
		{"high", "high", false},
		{"Max", "max", false},
		{"", "", false},
		{"invalid-effort", "", true},
		{"extreme", "", true},
	}
	for _, tc := range cases {
		got, err := ResolveAllowedEffort(tc.input)
		if (err != nil) != tc.wantErr {
			t.Errorf("ResolveAllowedEffort(%q) error = %v, wantErr %v", tc.input, err, tc.wantErr)
		}
		if got != tc.expected {
			t.Errorf("ResolveAllowedEffort(%q) = %q, want %q", tc.input, got, tc.expected)
		}
	}
}
