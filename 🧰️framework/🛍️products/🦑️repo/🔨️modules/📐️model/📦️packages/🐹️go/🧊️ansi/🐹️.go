// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧊️ansi carries the ANSI colors and the terminal width helpers.

// #endregion 🧲️Header

package model

import (
	os "os"
	strconv "strconv"
	strings "strings"
	utf8 "unicode/utf8"
)

// #region 🧊️ANSI

const ColorReset = "\033[0m"

const ColorRed = "\033[31m"

const ColorGreen = "\033[32m"

const ColorYellow = "\033[33m"

const ColorBlue = "\033[34m"

const ColorDim = "\033[2m"

const ColorBold = "\033[1m"

// 💿️colorize holds the data fields for a colorize record.
func Colorize(s string, color string, enabled bool) string {
	if !enabled {
		return s
	}
	return color + s + ColorReset
}

// 🔸️getTerminalWidth holds the data fields for a getTerminalWidth record.
func GetTerminalWidth() int {
	w := os.Getenv("COLUMNS")
	if w != "" {
		if n, err := strconv.Atoi(w); err == nil && n > 0 {
			return n
		}
	}
	return 120
}

func TruncateANSI(s string, maxVisible int) string {
	if maxVisible <= 0 {
		return s
	}
	visible := 0
	for i := 0; i < len(s); {
		if s[i] == 0x1b {
			j := i + 1
			for j < len(s) && s[j] != 'm' {
				j++
			}
			if j < len(s) {
				j++
			}
			i = j
			continue
		}
		_, size := utf8.DecodeRuneInString(s[i:])
		if size <= 0 {
			break
		}
		i += size
		visible++
		if visible > maxVisible {
			break
		}
	}
	if visible <= maxVisible {
		return s
	}
	if maxVisible <= 3 {
		maxVisible = 3
	}
	limit := maxVisible - 3
	var b strings.Builder
	visible = 0
	for i := 0; i < len(s); {
		if s[i] == 0x1b {
			j := i + 1
			for j < len(s) && s[j] != 'm' {
				j++
			}
			if j < len(s) {
				j++
			}
			b.WriteString(s[i:j])
			i = j
			continue
		}
		r, size := utf8.DecodeRuneInString(s[i:])
		if size <= 0 {
			break
		}
		if visible >= limit {
			break
		}
		b.WriteRune(r)
		visible++
		i += size
	}
	b.WriteString("...")
	b.WriteString("\x1b[0m")
	return b.String()
}

// #endregion 🧊️ANSI
