// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Dependency update command.

// #endregion 🧲️Header

package client

import (
	"fmt"
	command "github.com/usalu/semio/repo/client/internal/command"
	yaml "github.com/usalu/semio/repo/yaml"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
	"sync"
)

// Update command implementation for dependency updates.
// ✏️updateCmd holds the data fields for a updateCmd record.
var updateCmd = &command.Command{
	Use:   "update [target]",
	Short: "Update dependencies (npm, python, rust, go, dotnet)",
	RunE:  runUpdate,
}

// 🔁️updateDryRun holds the data fields for a updateDryRun record.
var updateDryRun bool

// 🔸️updateApply holds the data fields for a updateApply record.
var updateApply bool

// 🔺️init holds the data fields for a init record.
func init() {
	updateCmd.Flags().BoolVar(&updateDryRun, "dry-run", false, "Show what would be updated without making changes")
	updateCmd.Flags().BoolVar(&updateApply, "apply", false, "Apply updates (default is dry-run)")
	benchmarkCmd.Flags().BoolVar(&benchmarkDryRun, "dry-run", false, "Initialize and exit without running benchmarks")
}

// 🔻️DependabotConfig holds the data fields for a dependabot config record.
type DependabotConfig struct {
	Version int `yaml:"version"`
	Updates []struct {
		PackageEcosystem string `yaml:"package-ecosystem"`
		Directory        string `yaml:"directory"`
		Ignore           []struct {
			DependencyName string   `yaml:"dependency-name"`
			Versions       []string `yaml:"versions"`
		} `yaml:"ignore"`
	} `yaml:"updates"`
	XComposeConfig struct {
		PreserveLocalVersions struct {
			Npm struct {
				Pattern string `yaml:"pattern"`
			} `yaml:"npm"`
		} `yaml:"preserveLocalVersions"`
	} `yaml:"x-compose-config"`
}

// ⬛️UpdateConfig holds the data fields for a update config record.
type UpdateConfig struct {
	Exclude               map[string][]string
	Constraints           map[string][]Constraint
	PreserveLocalVersions struct {
		Npm struct {
			Pattern string
		}
	}
	Paths struct {
		Npm    []string
		Python []string
		Rust   []string
		Go     []string
		Dotnet []string
	}
}

// 🔒️Constraint holds the data fields for a constraint record.
type Constraint struct {
	Dependency string
	MaxMajor   int
}

// ⬜️runUpdate holds the data fields for a runUpdate record.
func runUpdate(cmd *command.Command, args []string) error {
	target := "all"
	if len(args) > 0 {
		target = args[0]
	}
	if !updateApply {
		updateDryRun = true
	}
	if updateDryRun {
		fmt.Println("=== Dependency Update Script ===")
		fmt.Println("Running in DRY RUN mode - no changes will be made.")
		fmt.Printf("Target: %s\n", target)
		fmt.Println("\n=== Update Complete ===")
		return nil
	}

	rootDir := findRepoRoot(".")
	config, err := loadUpdateConfig(rootDir)
	if err != nil {
		return err
	}

	fmt.Println("=== Dependency Update Script ===")
	if updateDryRun {
		fmt.Println("Running in DRY RUN mode - no changes will be made.")
	}
	fmt.Printf("Target: %s\n", target)

	var wg sync.WaitGroup

	if target == "all" || target == "npm" {
		wg.Add(1)
		go func() {
			defer wg.Done()
			updateNpm(rootDir, config, updateDryRun)
		}()
	}

	if target == "all" || target == "python" {
		wg.Add(1)
		go func() {
			defer wg.Done()
			updatePython(rootDir, config, updateDryRun)
		}()
	}

	if target == "all" || target == "rust" {
		wg.Add(1)
		go func() {
			updateRust(rootDir, config, updateDryRun)
		}()
	}
	if target == "all" || target == "go" {
		wg.Add(1)
		go func() {
			defer wg.Done()
			updateGo(rootDir, config, updateDryRun)
		}()
	}

	if target == "all" || target == "dotnet" {
		wg.Add(1)
		go func() {
			defer wg.Done()
			updateDotNet(rootDir, config, updateDryRun)
		}()
	}

	wg.Wait()
	fmt.Println("\n=== Update Complete ===")
	return nil
}

// 🟥️loadUpdateConfig holds the data fields for a loadUpdateConfig record.
func loadUpdateConfig(rootDir string) (*UpdateConfig, error) {
	dependabotPath := filepath.Join(rootDir, ".github", "dependabot.yml")
	data, err := os.ReadFile(dependabotPath)
	if err != nil {
		return nil, fmt.Errorf("dependabot.yml not found: %w", err)
	}

	var dependabot DependabotConfig
	if err := yaml.Unmarshal(data, &dependabot); err != nil {
		return nil, err
	}

	config := &UpdateConfig{
		Exclude:     make(map[string][]string),
		Constraints: make(map[string][]Constraint),
	}
	config.Paths.Npm = []string{}
	config.Paths.Python = []string{}
	config.Paths.Rust = []string{}
	config.Paths.Go = []string{}
	config.Paths.Dotnet = []string{}

	config.PreserveLocalVersions.Npm.Pattern = "*"
	if dependabot.XComposeConfig.PreserveLocalVersions.Npm.Pattern != "" {
		config.PreserveLocalVersions.Npm.Pattern = dependabot.XComposeConfig.PreserveLocalVersions.Npm.Pattern
	}

	for _, update := range dependabot.Updates {
		dir := strings.TrimPrefix(update.Directory, "/")
		ecosystem := update.PackageEcosystem

		switch ecosystem {
		case "npm":
			config.Paths.Npm = append(config.Paths.Npm, dir)
		case "uv":
			config.Paths.Python = append(config.Paths.Python, dir)
		case "cargo":
			config.Paths.Rust = append(config.Paths.Rust, dir)
		case "gomod":
			config.Paths.Go = append(config.Paths.Go, dir)
		case "nuget":
			files := findCsprojFiles(rootDir, dir)
			for _, file := range files {
				config.Paths.Dotnet = append(config.Paths.Dotnet, file)
				if len(update.Ignore) > 0 {
					for _, ignore := range update.Ignore {
						if len(ignore.Versions) > 0 {
							for _, v := range ignore.Versions {
								re := regexp.MustCompile(`>=\s*(\d+)\.`)
								match := re.FindStringSubmatch(v)
								if len(match) > 1 {
									maxMajor, _ := strconv.Atoi(match[1])
									maxMajor = maxMajor - 1
									config.Constraints[file] = append(config.Constraints[file], Constraint{
										Dependency: ignore.DependencyName,
										MaxMajor:   maxMajor,
									})
								}
							}
						} else {
							config.Exclude[file] = append(config.Exclude[file], ignore.DependencyName)
						}
					}
				}
			}
		}
	}
	return config, nil
}

// 📄️findCsprojFiles holds the data fields for a findCsprojFiles record.
func findCsprojFiles(rootDir, dir string) []string {
	fullDir := filepath.Join(rootDir, dir)
	var files []string
	entries, err := os.ReadDir(fullDir)
	if err != nil {
		return files
	}
	for _, entry := range entries {
		if !entry.IsDir() && strings.HasSuffix(entry.Name(), ".csproj") {
			files = append(files, filepath.Join(dir, entry.Name()))
		}
	}
	return files
}

// 🟧️runCommand holds the data fields for a runCommand record.
func runCommand(dir, name string, args ...string) error {
	cmd := exec.Command(name, args...)
	cmd.Dir = dir
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	fmt.Printf("  Running: %s %s in %s\n", name, strings.Join(args, " "), dir)
	return cmd.Run()
}

func runCommandQuiet(dir, name string, args ...string) (string, error) {
	cmd := exec.Command(name, args...)
	cmd.Dir = dir
	output, err := cmd.Output()
	return string(output), err
}

// 🟨️updateNpm holds the data fields for a updateNpm record.
func updateNpm(rootDir string, config *UpdateConfig, dryRun bool) {
	fmt.Println("\n[NPM] Updating npm packages...")
	if dryRun {
		fmt.Println("  [DRY RUN] Would run: npm update -S")
		return
	}

	if err := runCommand(rootDir, "npm", "update", "-S"); err != nil {
		fmt.Printf("Error updating npm: %v\n", err)
	}
	fmt.Println("[NPM] Done.")
}

func updatePython(rootDir string, config *UpdateConfig, dryRun bool) {
	fmt.Println("\n[Python] Updating Python packages...")
	for _, pyPath := range config.Paths.Python {
		fullPath := filepath.Join(rootDir, pyPath)
		tomlPath := filepath.Join(fullPath, "pyproject.toml")
		if _, err := os.Stat(tomlPath); os.IsNotExist(err) {
			continue
		}

		fmt.Printf("  Updating %s...\n", pyPath)
		if dryRun {
			fmt.Println("  [DRY RUN] Would update pyproject.toml and run uv lock")
			continue
		}

		if err := runCommand(fullPath, "uv", "lock", "--upgrade"); err != nil {
			fmt.Printf("Error updating python in %s: %v\n", pyPath, err)
		}
	}
	fmt.Println("[Python] Done.")
}

// 🟩️updateRust holds the data fields for a updateRust record.
func updateRust(rootDir string, config *UpdateConfig, dryRun bool) {
	fmt.Println("\n[Rust] Updating Rust packages...")
	for _, rsPath := range config.Paths.Rust {
		fullPath := filepath.Join(rootDir, rsPath)
		if _, err := os.Stat(filepath.Join(fullPath, "Cargo.toml")); os.IsNotExist(err) {
			continue
		}

		fmt.Printf("  Updating %s...\n", rsPath)
		if dryRun {
			fmt.Println("  [DRY RUN] Would run cargo update")
			continue
		}

		if err := runCommand(fullPath, "cargo", "update"); err != nil {
			fmt.Printf("Error updating rust in %s: %v\n", rsPath, err)
		}
	}
	fmt.Println("[Rust] Done.")
}

// 🟦️updateGo holds the data fields for a updateGo record.
func updateGo(rootDir string, config *UpdateConfig, dryRun bool) {
	fmt.Println("\n[Go] Updating Go modules...")
	for _, goPath := range config.Paths.Go {
		fullPath := filepath.Join(rootDir, goPath)
		if _, err := os.Stat(filepath.Join(fullPath, "go.mod")); os.IsNotExist(err) {
			continue
		}

		fmt.Printf("  Updating %s...\n", goPath)
		if dryRun {
			fmt.Println("  [DRY RUN] Would run: go get -u ./... && go mod tidy")
			continue
		}

		runCommand(fullPath, "go", "get", "-u", "./...")
		runCommand(fullPath, "go", "mod", "tidy")
	}
	fmt.Println("[Go] Done.")
}

func updateDotNet(rootDir string, config *UpdateConfig, dryRun bool) {
	fmt.Println("\n[.NET] Updating .NET packages...")
	for _, csprojPath := range config.Paths.Dotnet {
		fullPath := filepath.Join(rootDir, csprojPath)
		if _, err := os.Stat(fullPath); os.IsNotExist(err) {
			continue
		}

		fmt.Printf("  Updating %s...\n", csprojPath)
		if dryRun {
			fmt.Println("  [DRY RUN] Would check for package updates")
			continue
		}

		output, err := runCommandQuiet(filepath.Dir(fullPath), "dotnet", "list", fullPath, "package", "--outdated")
		if err != nil {
			continue
		}

		lines := strings.Split(output, "\n")
		for _, line := range lines {
			if strings.Contains(line, ">") {
				parts := strings.Fields(line)
				if len(parts) >= 5 {
					name := parts[1]
					latest := parts[4]

					excluded := false
					if ex, ok := config.Exclude[csprojPath]; ok {
						for _, e := range ex {
							if e == name {
								excluded = true
								break
							}
						}
					}
					if excluded {
						continue
					}

					fmt.Printf("    Updating %s to %s\n", name, latest)
					runCommand(filepath.Dir(fullPath), "dotnet", "add", fullPath, "package", name, "--version", latest)
				}
			}
		}
	}
	fmt.Println("[.NET] Done.")
}
