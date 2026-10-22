package events

import (
	bytes "bytes"
	json "encoding/json"
	fmt "fmt"
	io "io"
	http "net/http"
	os "os"
	strings "strings"
	time "time"
)

// #region 📤️Emit

// 🛰️Emitter delivers an envelope to the coordinator.
type Emitter interface {
	Emit(kind EventKind, source string, payload interface{})
}

// 🌐️HTTPEmitter posts envelopes to COMPOSE_SERVER_ADDR.
type HTTPEmitter struct { url string; token string }

// 🛰️NewHTTPEmitter captures one coordinator endpoint and token.
func NewHTTPEmitter(address, token string) HTTPEmitter { return HTTPEmitter{url:EmitURL(address), token:strings.TrimSpace(token)} }

// 📤️Emit posts an event to the repo server. No-operation when COMPOSE_SERVER_ADDR is unset.


// 🔗️EmitURL renders the coordinator endpoint for an address, empty when the address is empty.
func EmitURL(addr string) string {
	addr = strings.TrimSpace(addr)
	if addr == "" {
		return ""
	}
	url := addr
	if !strings.HasPrefix(addr, "http://") && !strings.HasPrefix(addr, "https://") {
		url = "http://" + addr
	}
	return strings.TrimSuffix(url, "/") + "/api/v1/events"
}

// 📤️Emit posts an event to the repo server. No-operation when COMPOSE_SERVER_ADDR is unset.
func Emit(kind EventKind, source string, payload interface{}) { NewHTTPEmitter(os.Getenv("COMPOSE_SERVER_ADDR"),os.Getenv("COMPOSE_SERVER_TOKEN")).Emit(kind,source,payload) }

// 📮️Emit delivers through the captured endpoint.
func (emitter HTTPEmitter) Emit(kind EventKind, source string, payload interface{}) {
	url := emitter.url
	if url == "" {
		return
	}
	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		return
	}
	body, err := json.Marshal(Event{Kind: kind, Source: source, Payload: payloadBytes})
	if err != nil {
		return
	}
	req, err := http.NewRequest(http.MethodPost, url, bytes.NewReader(body))
	if err != nil {
		return
	}
	req.Header.Set("Content-Type", "application/json")
	if token := emitter.token; token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}
	client := &http.Client{Timeout: 5 * time.Second}
	response, err := client.Do(req)
	if err == nil { response.Body.Close() }
}

// 🖥️getServerAddr returns the server address from COMPOSE_SERVER_ADDR env var.
func GetServerAddr() string {
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
func GetServerToken() string {
	return strings.TrimSpace(os.Getenv("COMPOSE_SERVER_TOKEN"))
}

// 📨️serverRequest sends an authenticated HTTP request to the server.
func ServerRequest(method, path string, body interface{}) (*http.Response, error) {
	addr := GetServerAddr()
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
	if token := GetServerToken(); token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}
	client := &http.Client{Timeout: 10 * time.Second}
	return client.Do(req)
}

// 🔐️serverWhoami calls the auth whoami endpoint and returns developer info.
func ServerWhoami() (map[string]interface{}, error) {
	resp, err := ServerRequest("GET", "/api/v1/auth", nil)
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

// #endregion 📤️Emit
