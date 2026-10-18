// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧪️test_command carries the test command of the cli with its llm, effort and client flag extraction.

// #endregion 🧲️Header

package cli

import (
	fmt "fmt"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	testrunner "github.com/usalu/semio/repo/testrunner"
)

// #region 🕸️Test Command

// ⬛️testCommand holds the data fields for a testCommand record.
func testCommand(factory EngineFactory, config *Config) *Command {
	return &Command{
		Use:   "test [testable-id-or-uri]...",
		Short: "Run tests for given entities",
		Long: `Run tests for one or more testable entities identified by their entity IDs or URIs.

With no arguments, runs all tests in the repository.

Supported entity scopes (narrowing order):
  🧰️technology                       - all tests in the technology
  🧰️technology⌨️bundle               - all tests in the bundle
  🧰️technology⌨️bundle🥼️file         - all tests in the test file
  🧰️technology⌨️bundle🥼️file🔖️sec    - all tests in the section
  🧰️technology⌨️bundle🥼️file🔖️sec🧪️fn - a single test function`,
		Args: ArbitraryArgs,
		RunE: func(cmd *Command, args []string) error {
			scopes := testrunner.ResolveTestScopes(args)
			var firstErr error
			for _, scope := range scopes {
				if err := testrunner.RunTestScope(scope, cmd); err != nil && firstErr == nil {
					firstErr = err
				}
			}
			return firstErr
		},
	}
}

// 🧠️extractLLMFromArgs extracts LLM from command flags and positional arguments.
func extractLLMFromArgs(cmd *Command, args []string) (string, []string) {
	if llm, _ := cmd.Flags().GetString("llm"); llm != "" {
		return llm, args
	}

	for _, allowed := range model.AllowedLLMList() {
		flagName := allowed
		if val, _ := cmd.Flags().GetBool(flagName); val {
			return allowed, args
		}
	}

	remaining := []string{}
	foundLLM := ""
	for _, arg := range args {
		if foundLLM != "" {
			remaining = append(remaining, arg)
			continue
		}
		normalized := model.NormalizeLLMSlug(arg)
		matched := false
		bestMatch := ""
		for _, allowed := range model.AllowedLLMList() {
			if strings.Contains(normalized, model.NormalizeLLMSlug(allowed)) {
				if len(allowed) > len(bestMatch) {
					bestMatch = allowed
					matched = true
				}
			}
		}
		if matched {
			foundLLM = bestMatch
		}
		if !matched {
			remaining = append(remaining, arg)
		}
	}
	return foundLLM, remaining
}

// 🏋️extractEffortFromArgs extracts reasoning effort from command flags and positional arguments.
func extractEffortFromArgs(cmd *Command, args []string) (string, []string) {
	if effort, _ := cmd.Flags().GetString("effort"); effort != "" {
		return effort, args
	}

	for _, allowed := range model.AllowedEffortList() {
		flagName := allowed
		if val, _ := cmd.Flags().GetBool(flagName); val {
			return allowed, args
		}
	}

	remaining := []string{}
	foundEffort := ""
	for _, arg := range args {
		if foundEffort != "" {
			remaining = append(remaining, arg)
			continue
		}
		normalized := model.NormalizeEffortSlug(arg)
		matched := false
		bestMatch := ""
		for _, allowed := range model.AllowedEffortList() {
			if normalized == model.NormalizeEffortSlug(allowed) || strings.Contains(normalized, model.NormalizeEffortSlug(allowed)) {
				if len(allowed) > len(bestMatch) {
					bestMatch = allowed
					matched = true
				}
			}
		}
		if matched {
			foundEffort = bestMatch
		}
		if !matched {
			remaining = append(remaining, arg)
		}
	}
	return foundEffort, remaining
}

// 🧲️extractClientFromArgs holds the data fields for a extractClientFromArgs record.
func extractClientFromArgs(cmd *Command, args []string) (string, []string) {

	if client, _ := cmd.Flags().GetString("client"); client != "" {
		return client, args
	}

	for _, allowed := range model.AllowedClientList() {
		flagName := allowed
		if val, _ := cmd.Flags().GetBool(flagName); val {
			return allowed, args
		}
	}

	remaining := []string{}
	foundClient := ""
	for _, arg := range args {
		if foundClient != "" {
			remaining = append(remaining, arg)
			continue
		}
		normalized := model.NormalizeClientSlug(arg)
		matched := false
		bestMatch := ""
		for _, allowed := range model.AllowedClientList() {
			if strings.Contains(normalized, model.NormalizeClientSlug(allowed)) {
				if len(allowed) > len(bestMatch) {
					bestMatch = allowed
					matched = true
				}
			}
		}
		if matched {
			foundClient = bestMatch
		}
		if !matched {
			remaining = append(remaining, arg)
		}
	}
	return foundClient, remaining
}

// ➕️addLLMFlags holds the data fields for a addLLMFlags record.
func addLLMFlags(cmd *Command) {
	for _, llm := range model.AllowedLLMList() {
		if cmd.Flags().Lookup(llm) == nil {
			cmd.Flags().Bool(llm, false, fmt.Sprintf("Use %s as LLM", llm))
		}
	}
}

// 🏋️addEffortFlags adds boolean flags for all allowed effort values.
func addEffortFlags(cmd *Command) {
	for _, effort := range model.AllowedEffortList() {
		if cmd.Flags().Lookup(effort) == nil {
			cmd.Flags().Bool(effort, false, fmt.Sprintf("Use %s as LLM reasoning effort", effort))
		}
	}
}

// 💻️addClientFlags holds the data fields for a addClientFlags record.
func addClientFlags(cmd *Command) {
	for _, client := range model.AllowedClientList() {
		if cmd.Flags().Lookup(client) == nil {
			cmd.Flags().Bool(client, false, fmt.Sprintf("Use %s as Client", client))
		}
	}
}

// #endregion 🕸️Test Command
