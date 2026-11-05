//go:build windows

// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🪟️windows-host runs the runtime as a child that shares the console, forwards interrupts and settles with its exit code, ties the child's life to the launcher with a job object, names a worktree by the kernel's final path, and reads the user locale.

// #endregion 🧲️Header

package launcher

import (
	"encoding/binary"
	"errors"
	"os"
	"strings"
	"syscall"
	"unicode/utf16"
	"unsafe"
)

// #region 🪟️Host

var (
	kernel32                 = syscall.NewLazyDLL("kernel32.dll")
	createJobObject          = kernel32.NewProc("CreateJobObjectW")
	setInformationJobObject  = kernel32.NewProc("SetInformationJobObject")
	assignProcessToJobObject = kernel32.NewProc("AssignProcessToJobObject")
	getFinalPathNameByHandle = kernel32.NewProc("GetFinalPathNameByHandleW")
)

// 🧾️Win32 constants of the job, process and file calls below. See https://learn.microsoft.com/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information
const (
	jobObjectExtendedLimitInformation = 9
	jobLimitKillOnJobClose            = 0x2000
	jobLimitSilentBreakawayOK         = 0x1000
	processSetQuota                   = 0x0100
	processTerminate                  = 0x0001
	errorSharingViolation             = syscall.Errno(32)
)

// 📐️jobExtendedLimit is JOBOBJECT_EXTENDED_LIMIT_INFORMATION; Go lays the fields out as C does (144 bytes on 64-bit Windows).
type jobExtendedLimit struct {
	PerProcessUserTimeLimit, PerJobUserTimeLimit int64
	LimitFlags                                   uint32
	MinimumWorkingSetSize, MaximumWorkingSetSize uintptr
	ActiveProcessLimit                           uint32
	Affinity                                     uintptr
	PriorityClass, SchedulingClass               uint32
	IoCounters                                   [6]uint64
	ProcessMemoryLimit, JobMemoryLimit           uintptr
	PeakProcessMemoryUsed, PeakJobMemoryUsed     uintptr
}

// 🔄️handOff starts the runtime with the inherited streams and waits for its terminal result.
func handOff(executable string, argv []string, env []string) error {
	return RunChild(executable, argv, env)
}

// 🪢️containChild puts the started runtime into a job that ends it when the launcher ends by any means (as `exec` makes the PID one on POSIX), with silent breakaway so only the runtime itself belongs to the job: the runtime's own children keep the job of the runtime, and the services it detaches outlive both. The returned release closes the job after the runtime settled; a host that refuses the job runs the child without it. See https://learn.microsoft.com/windows/win32/procthread/job-objects
func containChild(pid int) func() {
	job, _, _ := createJobObject.Call(0, 0)
	if job == 0 {
		return func() {}
	}
	limit := jobExtendedLimit{LimitFlags: jobLimitKillOnJobClose | jobLimitSilentBreakawayOK}
	process, err := syscall.OpenProcess(processSetQuota|processTerminate, false, uint32(pid))
	if err == nil {
		defer syscall.CloseHandle(process)
	}
	if ok, _, _ := setInformationJobObject.Call(job, jobObjectExtendedLimitInformation, uintptr(unsafe.Pointer(&limit)), unsafe.Sizeof(limit)); ok == 0 || err != nil {
		syscall.CloseHandle(syscall.Handle(job))
		return func() {}
	}
	if ok, _, _ := assignProcessToJobObject.Call(job, uintptr(process)); ok == 0 {
		syscall.CloseHandle(syscall.Handle(job))
		return func() {}
	}
	return func() { syscall.CloseHandle(syscall.Handle(job)) }
}

// 🧭️canonicalRoot is the spelling the kernel gives the directory (`GetFinalPathNameByHandleW`, normalized, DOS volume name): drive-letter case, component case, short names, links and junctions all map to it, and Bun's `realpathSync.native` answers the same text, so the launcher and the service hash one name. See https://learn.microsoft.com/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlew
func canonicalRoot(path string) string {
	name, err := syscall.UTF16PtrFromString(path)
	if err != nil {
		return path
	}
	handle, err := syscall.CreateFile(name, 0, syscall.FILE_SHARE_READ|syscall.FILE_SHARE_WRITE|syscall.FILE_SHARE_DELETE, nil, syscall.OPEN_EXISTING, syscall.FILE_FLAG_BACKUP_SEMANTICS, 0)
	if err != nil {
		return path
	}
	defer syscall.CloseHandle(handle)
	buffer := make([]uint16, 512)
	for {
		length, _, _ := getFinalPathNameByHandle.Call(uintptr(handle), uintptr(unsafe.Pointer(&buffer[0])), uintptr(len(buffer)), 0)
		if length == 0 {
			return path
		}
		if int(length) < len(buffer) {
			return FinalPathSpelling(syscall.UTF16ToString(buffer[:length]))
		}
		buffer = make([]uint16, length)
	}
}

// 🔁️renameTransient is true for the refusals a scanner or indexer causes for a moment by holding a fresh file open: access denied and sharing violation.
func renameTransient(err error) bool {
	return errors.Is(err, syscall.ERROR_ACCESS_DENIED) || errors.Is(err, errorSharingViolation)
}

// 🔗️Creates an unprivileged NTFS directory junction without invoking a shell or requiring symlink privileges.
func createWorkspaceDirectoryLink(source, destination string) error {
	if err := os.Mkdir(destination, 0o755); err != nil {
		return err
	}
	complete := false
	defer func() {
		if !complete {
			os.Remove(destination)
		}
	}()
	name, err := syscall.UTF16PtrFromString(destination)
	if err != nil {
		return err
	}
	handle, err := syscall.CreateFile(name, syscall.GENERIC_WRITE, syscall.FILE_SHARE_READ|syscall.FILE_SHARE_WRITE|syscall.FILE_SHARE_DELETE, nil, syscall.OPEN_EXISTING, syscall.FILE_FLAG_BACKUP_SEMANTICS|syscall.FILE_FLAG_OPEN_REPARSE_POINT, 0)
	if err != nil {
		return err
	}
	defer syscall.CloseHandle(handle)
	plain := strings.TrimPrefix(source, `\\?\`)
	substitute := `\??\` + plain
	if strings.HasPrefix(plain, `\\`) {
		substitute = `\??\UNC\` + strings.TrimPrefix(plain, `\\`)
	}
	first, last := utf16.Encode([]rune(substitute)), utf16.Encode([]rune(plain))
	buffer := make([]byte, 16+2*(len(first)+len(last)+2))
	binary.LittleEndian.PutUint32(buffer, 0xa0000003)
	binary.LittleEndian.PutUint16(buffer[4:], uint16(len(buffer)-8))
	binary.LittleEndian.PutUint16(buffer[10:], uint16(2*len(first)))
	binary.LittleEndian.PutUint16(buffer[12:], uint16(2*(len(first)+1)))
	binary.LittleEndian.PutUint16(buffer[14:], uint16(2*len(last)))
	for i, value := range first {
		binary.LittleEndian.PutUint16(buffer[16+2*i:], value)
	}
	for i, value := range last {
		binary.LittleEndian.PutUint16(buffer[16+2*(len(first)+1+i):], value)
	}
	var returned uint32
	if err := syscall.DeviceIoControl(handle, 0x000900a4, &buffer[0], uint32(len(buffer)), nil, 0, &returned, nil); err != nil {
		return err
	}
	complete = true
	return nil
}

func publishWorkspaceDirectoryLink(source, destination string) error {
	from, err := syscall.UTF16PtrFromString(source)
	if err != nil {
		return err
	}
	to, err := syscall.UTF16PtrFromString(destination)
	if err != nil {
		return err
	}
	result, _, failure := kernel32.NewProc("MoveFileW").Call(uintptr(unsafe.Pointer(from)), uintptr(unsafe.Pointer(to)))
	if result == 0 {
		return failure
	}
	return nil
}

// 🖥️systemLanguages lists the user locale name of Windows.
func systemLanguages(OsLocale) []string {
	buffer := make([]uint16, 85)
	found, _, _ := kernel32.NewProc("GetUserDefaultLocaleName").Call(uintptr(unsafe.Pointer(&buffer[0])), uintptr(len(buffer)))
	if found == 0 {
		return nil
	}
	return []string{syscall.UTF16ToString(buffer)}
}

// #endregion 🪟️Host
