package client

import (
	"os/exec"
	"runtime"
	"sort"
	"strconv"
	"strings"
)

type ownedCommandProcess struct {
	pid   int
	ppid  int
	pgid  int
	depth int
}

func ownedCommandProcessTree(root int) []ownedCommandProcess {
	if runtime.GOOS == "windows" {
		return nil
	}
	output, err := exec.Command("ps", "-axo", "pid=,ppid=,pgid=").Output()
	if err != nil {
		return []ownedCommandProcess{{pid: root, pgid: root}}
	}
	rows := make([]ownedCommandProcess, 0)
	for _, line := range strings.Split(string(output), "\n") {
		fields := strings.Fields(line)
		if len(fields) != 3 {
			continue
		}
		pid, pidErr := strconv.Atoi(fields[0])
		ppid, ppidErr := strconv.Atoi(fields[1])
		pgid, pgidErr := strconv.Atoi(fields[2])
		if pidErr == nil && ppidErr == nil && pgidErr == nil && pid > 0 && ppid > 0 && pgid > 0 {
			rows = append(rows, ownedCommandProcess{pid: pid, ppid: ppid, pgid: pgid})
		}
	}
	depths := map[int]int{root: 0}
	for changed := true; changed; {
		changed = false
		for _, row := range rows {
			if _, exists := depths[row.pid]; exists {
				continue
			}
			parentDepth, owned := depths[row.ppid]
			if !owned {
				continue
			}
			depths[row.pid] = parentDepth + 1
			changed = true
		}
	}
	owned := make([]ownedCommandProcess, 0, len(depths))
	for _, row := range rows {
		if depth, exists := depths[row.pid]; exists {
			row.depth = depth
			owned = append(owned, row)
		}
	}
	if !slicesContainsOwnedProcess(owned, root) {
		owned = append(owned, ownedCommandProcess{pid: root, pgid: root})
	}
	sort.Slice(owned, func(left, right int) bool { return owned[left].depth > owned[right].depth })
	return owned
}

func slicesContainsOwnedProcess(processes []ownedCommandProcess, pid int) bool {
	for _, process := range processes {
		if process.pid == pid {
			return true
		}
	}
	return false
}

func terminateOwnedCommandProcessTree(root int) {
	if root <= 0 {
		return
	}
	if runtime.GOOS == "windows" {
		_ = exec.Command("taskkill", "/pid", strconv.Itoa(root), "/T", "/F").Run()
		return
	}
	owned := ownedCommandProcessTree(root)
	pids := make(map[int]bool, len(owned))
	for _, process := range owned {
		pids[process.pid] = true
	}
	groups := make(map[int]bool)
	for _, process := range owned {
		if pids[process.pgid] {
			groups[process.pgid] = true
		}
	}
	for group := range groups {
		_ = exec.Command("kill", "-KILL", "-"+strconv.Itoa(group)).Run()
	}
	for _, process := range owned {
		_ = exec.Command("kill", "-KILL", strconv.Itoa(process.pid)).Run()
	}
}
