//go:build windows

// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🪟️windows-contract proves the Windows host on a real Windows machine: the job that ends the runtime with the launcher, the worktree name from the kernel's final path compared with Bun's own answer, the account check and busy wait of the service pipe, and the transient rename refusals.

// #endregion 🧲️Header

package launcher

import (
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"
	"syscall"
	"testing"
	"time"
	"unsafe"
)

const windowsHelperRole = "SEMIO_LAUNCHER_WINDOWS_HELPER"

// 🧪️TestWindowsHelper is no test of its own: started with a role it plays the launcher (runs the sleeper through RunChild) or the sleeper (writes its pid, then sleeps).
func TestWindowsHelper(t *testing.T) {
	switch os.Getenv(windowsHelperRole) {
	case "launcher":
		_ = RunChild(os.Args[0], []string{os.Args[0], "-test.run=^TestWindowsHelper$"}, append(os.Environ(), windowsHelperRole+"=sleeper"))
		os.Exit(0)
	case "sleeper":
		_ = os.WriteFile(os.Getenv("SEMIO_LAUNCHER_WINDOWS_PID"), []byte(strconv.Itoa(os.Getpid())), 0o644)
		time.Sleep(60 * time.Second)
		os.Exit(0)
	}
}

func processAlive(pid int) bool {
	handle, err := syscall.OpenProcess(0x1000, false, uint32(pid))
	if err != nil {
		return false
	}
	defer syscall.CloseHandle(handle)
	var code uint32
	return syscall.GetExitCodeProcess(handle, &code) == nil && code == 259
}

// 📐️The job limit block has the size and field offsets of JOBOBJECT_EXTENDED_LIMIT_INFORMATION on 64-bit Windows.
func TestWindowsJobLayout(t *testing.T) {
	var limit jobExtendedLimit
	if unsafe.Sizeof(limit) != 144 || unsafe.Offsetof(limit.LimitFlags) != 16 || unsafe.Offsetof(limit.IoCounters) != 64 || unsafe.Offsetof(limit.ProcessMemoryLimit) != 112 {
		t.Fatalf("layout %d/%d/%d/%d", unsafe.Sizeof(limit), unsafe.Offsetof(limit.LimitFlags), unsafe.Offsetof(limit.IoCounters), unsafe.Offsetof(limit.ProcessMemoryLimit))
	}
}

// 🪢️Terminating the launcher from outside ends the runtime it started.
func TestWindowsRuntimeEndsWithLauncher(t *testing.T) {
	pidFile := filepath.Join(t.TempDir(), "pid")
	launcher := exec.Command(os.Args[0], "-test.run=^TestWindowsHelper$")
	launcher.Env = append(os.Environ(), windowsHelperRole+"=launcher", "SEMIO_LAUNCHER_WINDOWS_PID="+pidFile)
	if err := launcher.Start(); err != nil {
		t.Fatal(err)
	}
	sleeper := 0
	for deadline := time.Now().Add(30 * time.Second); sleeper == 0 && time.Now().Before(deadline); time.Sleep(50 * time.Millisecond) {
		raw, _ := os.ReadFile(pidFile)
		sleeper, _ = strconv.Atoi(strings.TrimSpace(string(raw)))
	}
	if sleeper == 0 || !processAlive(sleeper) {
		_ = launcher.Process.Kill()
		t.Fatal("the runtime did not start")
	}
	_ = launcher.Process.Kill()
	_ = launcher.Wait()
	for deadline := time.Now().Add(5 * time.Second); processAlive(sleeper) && time.Now().Before(deadline); time.Sleep(50 * time.Millisecond) {
	}
	if processAlive(sleeper) {
		if process, err := os.FindProcess(sleeper); err == nil {
			_ = process.Kill()
		}
		t.Fatal("the runtime outlived the terminated launcher")
	}
}

// 🧭️Every spelling of one directory gets the kernel's name, and Bun's `realpathSync.native` answers the same text.
func TestWindowsCanonicalRootEqualsBun(t *testing.T) {
	base := t.TempDir()
	directory := filepath.Join(base, "Mixed Case Root")
	if err := os.Mkdir(directory, 0o755); err != nil {
		t.Fatal(err)
	}
	want := canonicalRoot(directory)
	variant := strings.ToLower(directory[:1]) + strings.ToUpper(directory[1:])
	if got := canonicalRoot(variant); got != want {
		t.Errorf("variant %s: %s, want %s", variant, got, want)
	}
	bun, err := exec.LookPath("bun")
	if err != nil {
		t.Fatal("bun is required for the cross-runtime law of the worktree name")
	}
	output, err := exec.Command(bun, "-e", "process.stdout.write(require('node:fs').realpathSync.native(process.argv[1]))", variant).Output()
	if err != nil || string(output) != want {
		t.Errorf("bun realpathSync.native(%s) = %q (%v), launcher %q", variant, output, err, want)
	}
}

// 🔐️A pipe served by the caller's own process is kept; while its only instance is busy the dial waits about 250 ms and fails.
func TestWindowsResidentPipeOwnerAndBusy(t *testing.T) {
	endpoint := `\\.\pipe\semio-repo-test-` + strconv.Itoa(os.Getpid())
	name, _ := syscall.UTF16PtrFromString(endpoint)
	server, _, callErr := kernel32.NewProc("CreateNamedPipeW").Call(uintptr(unsafe.Pointer(name)), 3, 0, 1, 4096, 4096, 0, 0)
	if syscall.Handle(server) == syscall.InvalidHandle {
		t.Fatal(callErr)
	}
	defer syscall.CloseHandle(syscall.Handle(server))
	first, err := residentDialEndpoint(endpoint)
	if err != nil {
		t.Fatalf("own pipe refused: %v", err)
	}
	defer first.Close()
	if owner, err := pipeServerUser(syscall.Handle(first.(*os.File).Fd())); err != nil || owner != currentUser() || owner == "?" {
		t.Errorf("server account %q (%v), caller %q", owner, err, currentUser())
	}
	started := time.Now()
	if _, err := residentDialEndpoint(endpoint); !errors.Is(err, errorPipeBusy) || time.Since(started) < 200*time.Millisecond {
		t.Errorf("busy pipe: %v after %v", err, time.Since(started))
	}
}

// 🔁️Access denied and a sharing violation are transient for a rename; a missing file or an existing target are not.
func TestWindowsRenameTransient(t *testing.T) {
	for code, want := range map[syscall.Errno]bool{5: true, 32: true, 2: false, 3: false, 183: false} {
		if got := renameTransient(&os.LinkError{Op: "rename", Err: code}); got != want {
			t.Errorf("errno %d: %v", code, got)
		}
	}
}
