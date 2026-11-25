// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🫥️hidden-children proves that a captured helper of the launcher starts without a console window on Windows (CREATE_NO_WINDOW 0x08000000 and a hidden window, the values of the Win32 documentation) and unchanged elsewhere.

// #endregion 🧲️Header

package launcher

import (
	"os/exec"
	"reflect"
	"runtime"
	"testing"
)

// 🫥️A captured helper is hidden on Windows and untouched on Unix.
func TestCapturedHelperIsHidden(t *testing.T) {
	command := exec.Command("helper")
	hideChild(command, false)
	if runtime.GOOS != "windows" {
		if command.SysProcAttr != nil {
			t.Fatal("a Unix helper must keep its default attributes")
		}
		return
	}
	attributes := reflect.ValueOf(command.SysProcAttr)
	if attributes.IsNil() {
		t.Fatal("a Windows helper needs attributes")
	}
	if !attributes.Elem().FieldByName("HideWindow").Bool() {
		t.Fatal("the helper window must be hidden")
	}
	if attributes.Elem().FieldByName("CreationFlags").Uint()&0x08000000 == 0 {
		t.Fatal("the helper must start with CREATE_NO_WINDOW")
	}
}
