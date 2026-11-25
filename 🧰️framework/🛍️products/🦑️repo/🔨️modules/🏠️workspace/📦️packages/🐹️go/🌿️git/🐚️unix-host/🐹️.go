//go:build !windows

package workspace

import "os/exec"

// 🫥️hideConsoleWindow is a no-op: Unix children open no console window.
func hideConsoleWindow(command *exec.Cmd) {}

// 🧭️setRegisteredBatchCommandLine is unreachable from ordinary non-Windows routing.
func setRegisteredBatchCommandLine(command *exec.Cmd, fragment string) {}
