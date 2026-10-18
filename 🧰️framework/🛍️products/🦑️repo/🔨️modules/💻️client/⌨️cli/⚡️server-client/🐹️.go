// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// File utilities and server client.

// #endregion 🧲️Header

package client

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"strings"
	"time"
)

// 🚚️MoveFile MUST return a non-nil error when the operation fails.
func MoveFile(sourcePath, destPath string) error {
	inputFile, err := os.Open(sourcePath)
	if err != nil {
		return fmt.Errorf("couldn't open source file: %v", err)
	}
	outputFile, err := os.Create(destPath)
	if err != nil {
		inputFile.Close()
		return fmt.Errorf("couldn't open dest file: %v", err)
	}
	defer outputFile.Close()
	_, err = io.Copy(outputFile, inputFile)
	inputFile.Close()
	if err != nil {
		return fmt.Errorf("writing to output file failed: %v", err)
	}

	err = os.Remove(sourcePath)
	if err != nil {
		return fmt.Errorf("failed removing original file: %v", err)
	}
	return nil
}

// ❌️CopyFile MUST return a non-nil error when the operation fails.
func CopyFile(sourcePath, destPath string) error {
	inputFile, err := os.Open(sourcePath)
	if err != nil {
		return fmt.Errorf("couldn't open source file: %v", err)
	}
	defer inputFile.Close()
	outputFile, err := os.Create(destPath)
	if err != nil {
		return fmt.Errorf("couldn't open dest file: %v", err)
	}
	defer outputFile.Close()
	_, err = io.Copy(outputFile, inputFile)
	if err != nil {
		return fmt.Errorf("writing to output file failed: %v", err)
	}
	return nil
}

// 🖥️getServerAddr returns the server address from COMPOSE_SERVER_ADDR env var.
func getServerAddr() string {
	addr := strings.TrimSpace(os.Getenv("COMPOSE_SERVER_ADDR"))
	if addr == "" {
		return ""
	}
	if !strings.HasPrefix(addr, "http://") && !strings.HasPrefix(addr, "https://") {
		addr = "http://" + addr
	}
	return strings.TrimSuffix(addr, "/")
}

// 🎟️getServerToken returns the API key from COMPOSE_SERVER_TOKEN env var.
func getServerToken() string {
	return strings.TrimSpace(os.Getenv("COMPOSE_SERVER_TOKEN"))
}

// 📨️serverRequest sends an authenticated HTTP request to the server.
func serverRequest(method, path string, body interface{}) (*http.Response, error) {
	addr := getServerAddr()
	if addr == "" {
		return nil, fmt.Errorf("COMPOSE_SERVER_ADDR not set")
	}
	url := addr + path
	var reqBody io.Reader
	if body != nil {
		jsonBytes, err := json.Marshal(body)
		if err != nil {
			return nil, err
		}
		reqBody = bytes.NewReader(jsonBytes)
	}
	req, err := http.NewRequest(method, url, reqBody)
	if err != nil {
		return nil, err
	}
	req.Header.Set("Content-Type", "application/json")
	if token := getServerToken(); token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}
	client := &http.Client{Timeout: 10 * time.Second}
	return client.Do(req)
}

// ⚙️syncTicketToServer sends ticket state to the server. No-operation if server is not configured.
func syncTicketToServer(ticket *Ticket, action string) {
	addr := getServerAddr()
	if addr == "" {
		return
	}
	ticketID := FormatTicketRelPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	var payload map[string]interface{}
	switch action {
	case "close":
		payload = map[string]interface{}{"action": action, "ticket_id": ticketID, "summary": ticket.Summary}
	case "reopen":
		payload = map[string]interface{}{"action": action, "ticket_id": ticketID, "prompt": ticket.Description, "title": ticket.Title}
	default:
		var parent *string
		if ticket.Parent != "" {
			parent = &ticket.Parent
		}
		payload = map[string]interface{}{
			"action":    action,
			"ticket_id": ticketID,
			"title":     ticket.Title,
			"prompt":    ticket.Description,
			"goal":      ticket.Goal,
			"parent":    parent,
			"author":    GetGitAuthorAlias(),
		}
		if ticket.Management != nil && ticket.Management.Issue != "" {
			payload["github_issue"] = ticket.Management.Issue
		}
	}
	resp, err := serverRequest("POST", "/api/v1/ticket", payload)
	if err != nil {
		return
	}
	resp.Body.Close()
}

// 🔐️serverWhoami calls the auth whoami endpoint and returns developer info.
func serverWhoami() (map[string]interface{}, error) {
	resp, err := serverRequest("GET", "/api/v1/auth", nil)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()
	if resp.StatusCode != 200 {
		return nil, fmt.Errorf("auth failed: %s", resp.Status)
	}
	var result map[string]interface{}
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, err
	}
	return result, nil
}
