//go:build windows

package client

import (
 "os/exec"
 "syscall"
)

// 🫥️hideConsoleWindow starts the helper without a console window: CREATE_NO_WINDOW and a hidden window. See https://learn.microsoft.com/windows/win32/procthread/process-creation-flags
func hideConsoleWindow(command *exec.Cmd) *exec.Cmd {
 if command.SysProcAttr == nil {
  command.SysProcAttr = &syscall.SysProcAttr{}
 }
 command.SysProcAttr.HideWindow = true
 command.SysProcAttr.CreationFlags |= 0x08000000
 return command
}
