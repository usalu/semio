package main

import (
	"errors"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

type sdkFile struct {
	ID, Path, Cabinet string
	Bytes             int64
}
type cabinetInput struct {
	Name string
	archiveInput
}
type sdkMedium struct {
	disk, last int
	cabinet    string
}

var fileID = regexp.MustCompile(`^[a-zA-Z0-9_]+$`)
var cabinetID = regexp.MustCompile(`^[a-f0-9]{32}\.cab$`)

func sdkFiles(directories [][3]string, components [][2]string, files [][5]string, media [][3]string) ([]sdkFile, error) {
	dirs := map[string]string{}
	for _, row := range components {
		if row[0] == "" || dirs[row[0]] != "" {
			return nil, errors.New("duplicate MSI component")
		}
		dirs[row[0]] = row[1]
	}
	volumes := []sdkMedium{}
	for _, row := range media {
		disk, e1 := strconv.Atoi(row[0])
		last, e2 := strconv.Atoi(row[1])
		if e1 != nil || e2 != nil || disk < 1 || last < 0 {
			return nil, errors.New("invalid MSI media")
		}
		volumes = append(volumes, sdkMedium{disk, last, row[2]})
	}
	sort.Slice(volumes, func(i, j int) bool { return volumes[i].disk < volumes[j].disk })
	ids, paths := map[string]bool{}, map[string]bool{}
	result := make([]sdkFile, 0, len(files))
	for _, row := range files {
		size, e1 := strconv.ParseInt(row[3], 10, 64)
		sequence, e2 := strconv.Atoi(row[4])
		directory, ok := dirs[row[1]]
		if !ok || !fileID.MatchString(row[0]) || ids[strings.ToLower(row[0])] || e1 != nil || size < 0 || size > 100000000 || e2 != nil || sequence < 1 {
			return nil, errors.New("invalid MSI file")
		}
		ids[strings.ToLower(row[0])] = true
		path, err := destination(directories, directory, row[2])
		if err != nil {
			return nil, err
		}
		if !strings.HasPrefix(path, "Windows Kits/10/") {
			return nil, errors.New("file outside SDK root")
		}
		path = "sdk/" + strings.TrimPrefix(path, "Windows Kits/10/")
		key := strings.ToLower(path)
		if paths[key] {
			return nil, errors.New("duplicate SDK path")
		}
		paths[key] = true
		cabinet := ""
		for _, medium := range volumes {
			if medium.last >= sequence {
				cabinet = medium.cabinet
				break
			}
		}
		if !cabinetID.MatchString(cabinet) {
			return nil, errors.New("invalid cabinet reference")
		}
		result = append(result, sdkFile{row[0], path, cabinet, size})
	}
	return result, nil
}
