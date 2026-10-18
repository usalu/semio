package todos

import (
	json "encoding/json"
	testing "testing"
)

// #region 📥️Hook Stdin Tests

func TestExtractToolInputFromStdinNativeFormat(t *testing.T) {
	t.Run("native.event.tool_input extracted", func(t *testing.T) {
		input := json.RawMessage(`{"native":{"event":{"tool_name":"Read","tool_input":{"file_path":"/tmp/test.go","limit":100}}}}`)
		result := ExtractToolInputFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil tool input from native format")
		}
		var data map[string]interface{}
		if err := json.Unmarshal(result, &data); err != nil {
			t.Fatal(err)
		}
		if data["file_path"] != "/tmp/test.go" {
			t.Errorf("expected file_path=/tmp/test.go, got %v", data["file_path"])
		}
	})
	t.Run("direct tool_input still works", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"Read","tool_input":{"file_path":"/tmp/test.go"}}`)
		result := ExtractToolInputFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil tool input")
		}
	})
}

func TestExtractToolInputFromStdin(t *testing.T) {
	t.Run("with tool_input", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"test","tool_input":{"key":"val"}}`)
		result := ExtractToolInputFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil tool input")
		}
		var data map[string]interface{}
		if err := json.Unmarshal(result, &data); err != nil {
			t.Fatal(err)
		}
		if data["key"] != "val" {
			t.Errorf("expected key=val, got %v", data["key"])
		}
	})
	t.Run("without tool_input", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"test"}`)
		result := ExtractToolInputFromStdin(input)
		if result != nil {
			t.Error("expected nil tool input")
		}
	})
	t.Run("empty", func(t *testing.T) {
		result := ExtractToolInputFromStdin(nil)
		if result != nil {
			t.Error("expected nil for empty input")
		}
	})
}

func TestExtractToolResponseFromStdin(t *testing.T) {
	t.Run("tool_output", func(t *testing.T) {
		input := json.RawMessage(`{"tool_output":"response data"}`)
		result := ExtractToolResponseFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil response")
		}
	})
	t.Run("tool_response", func(t *testing.T) {
		input := json.RawMessage(`{"tool_response":"data"}`)
		result := ExtractToolResponseFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil response")
		}
	})
	t.Run("no response", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"test"}`)
		result := ExtractToolResponseFromStdin(input)
		if result != nil {
			t.Error("expected nil response")
		}
	})
}

// #endregion 📥️Hook Stdin Tests
