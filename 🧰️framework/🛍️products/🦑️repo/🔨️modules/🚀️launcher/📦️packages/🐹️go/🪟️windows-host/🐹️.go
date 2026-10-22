//go:build windows

// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🪟️windows-host runs the runtime as a child that shares the console, forwards interrupts and settles with its exit code, and reads the user locale.

// #endregion 🧲️Header

package launcher

import (
	"syscall"
	"unsafe"
)

// #region 🪟️Host

// 🔄️handOff starts the runtime with the inherited streams and waits for its terminal result.
func handOff(executable string, argv []string, env []string) error { return RunChild(executable, argv, env) }

// 🖥️systemLanguages lists the user locale name of Windows.
func systemLanguages() []string {
	buffer := make([]uint16, 85)
	found, _, _ := syscall.NewLazyDLL("kernel32.dll").NewProc("GetUserDefaultLocaleName").Call(uintptr(unsafe.Pointer(&buffer[0])), uintptr(len(buffer)))
	if found == 0 {
		return nil
	}
	return []string{syscall.UTF16ToString(buffer)}
}

// #endregion 🪟️Host
