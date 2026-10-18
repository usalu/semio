// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// ⚙️config carries the repository config settings: logging flags, defaults, loading and parsing.

// #endregion 🧲️Header

package workspace

import (
	os "os"
	strings "strings"
)

// #region ⚙️RepoConfig

// 📋️LoggingConfig holds hook logging switches and detail for `📋️config.toml` `[logging]`.
type LoggingConfig struct {
	Session    bool
	Operations bool
	Plan       bool
	Detail     string
}

// ⚙️RepoConfig holds repo-wide settings loaded from `.🧬semio/🦑️repo/📋️config.toml`.
type RepoConfig struct {
	Logging LoggingConfig
}

// 🔷️DefaultRepoConfig returns the settings used when the config file is missing.
func DefaultRepoConfig() RepoConfig {
	return RepoConfig{
		Logging: LoggingConfig{
			Session:    false,
			Operations: true,
			Plan:       true,
			Detail:     "standard",
		},
	}
}

// ✅️parseRepoConfigBool reads the affirmative spellings a config value may use.
func parseRepoConfigBool(value string) bool {
	switch strings.ToLower(strings.TrimSpace(value)) {
	case "true", "yes", "1", "on":
		return true
	default:
		return false
	}
}

// ✂️unquoteRepoConfigValue strips one matching pair of surrounding quotes.
func unquoteRepoConfigValue(value string) string {
	value = strings.TrimSpace(value)
	if len(value) >= 2 {
		if (value[0] == '"' && value[len(value)-1] == '"') || (value[0] == '\'' && value[len(value)-1] == '\'') {
			return value[1 : len(value)-1]
		}
	}
	return value
}

// 📥️LoadRepoConfig reads `.🧬semio/🦑️repo/📋️config.toml` under repoRoot; a missing file yields defaults.
func LoadRepoConfig(repoRoot string) RepoConfig {
	if strings.TrimSpace(repoRoot) == "" {
		return DefaultRepoConfig()
	}
	data, err := os.ReadFile(RepoMetaPathForRoot(repoRoot, ConfigFileName))
	if err != nil {
		return DefaultRepoConfig()
	}
	return ParseRepoConfig(string(data))
}

// 📜️ParseRepoConfig applies the recognised `[logging]` keys of a config document onto the defaults.
func ParseRepoConfig(document string) RepoConfig {
	config := DefaultRepoConfig()
	section := ""
	for _, line := range strings.Split(document, "\n") {
		trim := strings.TrimSpace(line)
		if trim == "" || strings.HasPrefix(trim, "#") {
			continue
		}
		if strings.HasPrefix(trim, "[") && strings.HasSuffix(trim, "]") {
			section = strings.ToLower(strings.Trim(trim, "[]"))
			continue
		}
		if !strings.Contains(trim, "=") {
			continue
		}
		parts := strings.SplitN(trim, "=", 2)
		key := strings.ToLower(strings.TrimSpace(parts[0]))
		value := unquoteRepoConfigValue(parts[1])
		if section != "logging" {
			continue
		}
		switch key {
		case "session":
			config.Logging.Session = parseRepoConfigBool(value)
		case "operations":
			config.Logging.Operations = parseRepoConfigBool(value)
		case "plan":
			config.Logging.Plan = parseRepoConfigBool(value)
		case "detail":
			if value != "" {
				config.Logging.Detail = value
			}
		}
	}
	return config
}

// 📤️IncludeResponse reports whether hook logs carry the response payload at this detail level.
func (config LoggingConfig) IncludeResponse() bool {
	return strings.ToLower(strings.TrimSpace(config.Detail)) != "minimal"
}

// 🔬️IncludeNative reports whether hook logs carry the native payload at this detail level.
func (config LoggingConfig) IncludeNative() bool {
	return strings.ToLower(strings.TrimSpace(config.Detail)) == "full"
}

// #endregion ⚙️RepoConfig
