//go:build !windows

package client

import "os/exec"

// 🫥️hideConsoleWindow is a no-op: Unix children open no console window.
func hideConsoleWindow(command *exec.Cmd) *exec.Cmd { return command }
