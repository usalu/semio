package main

import (
	"errors"
	"regexp"
	"strings"
	"unicode"
)

var reserved = regexp.MustCompile(`(?i)^(con|prn|aux|nul|com[1-9¹²³]|lpt[1-9¹²³])(\.|$)`)

func segment(value string, directory bool) (string, error) {
	if directory {
		value = strings.SplitN(value, ":", 2)[0]
	}
	parts := strings.Split(value, "|")
	if len(parts) > 2 {
		return "", errors.New("invalid MSI name")
	}
	name := parts[len(parts)-1]
	if directory && name == "." {
		return "", nil
	}
	if name == "" || name == "." || name == ".." || strings.ContainsAny(name, "<>:\"/\\|?*") || strings.IndexFunc(name, unicode.IsControl) >= 0 || strings.HasSuffix(name, ".") || strings.HasSuffix(name, " ") || reserved.MatchString(name) {
		return "", errors.New("unsafe MSI path")
	}
	return name, nil
}

func archivePath(value string) (string, error) {
	parts := strings.Split(value, "/")
	if len(parts) > 64 {
		return "", errors.New("archive path depth")
	}
	for i, part := range parts {
		name, err := segment(part, false)
		if err != nil || name != part {
			return "", errors.New("unsafe archive path")
		}
		parts[i] = name
	}
	return strings.Join(parts, "/"), nil
}

func destination(rows [][3]string, directory, file string) (string, error) {
	names := map[string][3]string{}
	for _, row := range rows {
		if row[0] == "" {
			return "", errors.New("empty MSI directory")
		}
		if _, ok := names[row[0]]; ok {
			return "", errors.New("duplicate MSI directory")
		}
		names[row[0]] = row
	}
	name, err := segment(file, false)
	if err != nil {
		return "", err
	}
	parts := []string{name}
	seen := map[string]bool{}
	reached := false
	for directory != "" {
		if seen[directory] || len(seen) >= 64 {
			return "", errors.New("MSI directory cycle or depth")
		}
		seen[directory] = true
		row, ok := names[directory]
		if !ok {
			return "", errors.New("missing MSI directory")
		}
		if directory == "TARGETDIR" {
			if row[1] != "" || row[2] != "SourceDir" {
				return "", errors.New("invalid MSI target root")
			}
			reached = true
			break
		}
		name, err = segment(row[2], true)
		if err != nil {
			return "", err
		}
		if name != "" {
			parts = append([]string{name}, parts...)
		}
		directory = row[1]
	}
	if !reached {
		return "", errors.New("MSI directory does not reach target root")
	}
	return strings.Join(parts, "/"), nil
}
