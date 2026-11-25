// 🫥️ Registered tool commands (git included) start without a console window on Windows.

package workspace

import (
	"reflect"
	"runtime"
	"testing"
)

// 🫥️A registered tool command is hidden on Windows (CREATE_NO_WINDOW 0x08000000 and a hidden window, the values of the Win32 documentation) and untouched on Unix.
func TestToolCommandIsHidden(t *testing.T) {
	command := ToolCommand("git", "--version")
	if runtime.GOOS != "windows" {
		if command.SysProcAttr != nil {
			t.Fatal("a Unix tool command must keep its default attributes")
		}
		return
	}
	attributes := reflect.ValueOf(command.SysProcAttr)
	if attributes.IsNil() || !attributes.Elem().FieldByName("HideWindow").Bool() || attributes.Elem().FieldByName("CreationFlags").Uint()&0x08000000 == 0 {
		t.Fatal("a Windows tool command must be hidden with CREATE_NO_WINDOW")
	}
}
