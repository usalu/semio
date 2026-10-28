//go:build windows

// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🛰️resident-windows opens the named pipe of the service as a file and keeps it only when the process serving it runs as the calling user; the key in the user's profile and that account check are the boundary.

// #endregion 🧲️Header

package launcher

import (
	"errors"
	"io"
	"os"
	"syscall"
	"time"
	"unsafe"
)

// #region 🛰️ResidentWindows

const residentGOOS = "windows"

const (
	processQueryLimitedInformation = 0x1000
	errorPipeBusy                  = syscall.Errno(231)
)

var (
	getNamedPipeServerProcessId = kernel32.NewProc("GetNamedPipeServerProcessId")
	waitNamedPipe               = kernel32.NewProc("WaitNamedPipeW")
)

// 🔌️residentDialEndpoint opens the service pipe, waiting up to 250 ms while every instance is busy (the unix socket dial waits as long), and keeps it only when the process serving it runs as the calling user: pipe names are machine-wide, so another account could serve the name first.
func residentDialEndpoint(endpoint string) (io.ReadWriteCloser, error) {
	name, err := syscall.UTF16PtrFromString(endpoint)
	if err != nil {
		return nil, err
	}
	deadline := time.Now().Add(250 * time.Millisecond)
	for {
		file, err := os.OpenFile(endpoint, os.O_RDWR, 0)
		if err == nil {
			if owner, ownerErr := pipeServerUser(syscall.Handle(file.Fd())); ownerErr != nil || owner != currentUser() {
				file.Close()
				return nil, errors.New("resident pipe served by another account")
			}
			return file, nil
		}
		remaining := time.Until(deadline)
		if !errors.Is(err, errorPipeBusy) || remaining <= 0 {
			return nil, err
		}
		waitNamedPipe.Call(uintptr(unsafe.Pointer(name)), uintptr(remaining.Milliseconds()+1))
	}
}

// 🪪️pipeServerUser is the account SID of the process at the server end of a pipe. See https://learn.microsoft.com/windows/win32/api/winbase/nf-winbase-getnamedpipeserverprocessid
func pipeServerUser(pipe syscall.Handle) (string, error) {
	var pid uint32
	if ok, _, callErr := getNamedPipeServerProcessId.Call(uintptr(pipe), uintptr(unsafe.Pointer(&pid))); ok == 0 {
		return "", callErr
	}
	process, err := syscall.OpenProcess(processQueryLimitedInformation, false, pid)
	if err != nil {
		return "", err
	}
	defer syscall.CloseHandle(process)
	var token syscall.Token
	if err := syscall.OpenProcessToken(process, syscall.TOKEN_QUERY, &token); err != nil {
		return "", err
	}
	defer token.Close()
	return tokenUser(token)
}

// 🪪️currentUser is the account SID of the launcher, or a value no SID equals when the token is unreadable.
func currentUser() string {
	token, err := syscall.OpenCurrentProcessToken()
	if err != nil {
		return "?"
	}
	defer token.Close()
	user, err := tokenUser(token)
	if err != nil {
		return "?"
	}
	return user
}

// 🆔️tokenUser is the string SID of the account a token belongs to.
func tokenUser(token syscall.Token) (string, error) {
	user, err := token.GetTokenUser()
	if err != nil {
		return "", err
	}
	return user.User.Sid.String()
}

// 🔐️residentTrusted needs no directory check: the key lives in the profile of the user, and the dial checks the account serving the pipe.
func residentTrusted(string) bool { return true }

// #endregion 🛰️ResidentWindows
