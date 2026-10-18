// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// The repo coordinator entrypoint: environment configuration, the route mux, the service lifecycle and Main.

// #endregion 🧲️Header

// Package coordinator owns the repo coordinator service: storage, projections and HTTP.
package coordinator

import (
	"errors"
	"fmt"
	"log"
	"math/rand"
	"net"
	"net/http"
	"os"
	"strconv"
	"strings"
	"time"
)

// #region 🚀️Entrypoint

// Server configuration loading from environment variables. MUST provide sensible defaults.

// ⚙️Config holds all server configuration values.
type Config struct {
	Address          string
	DatabasePath     string
	RepoRoot         string
	Token            string
	GitHubSecret     string
	DiscordWebhook   string
	RequestBodyLimit int64
}

// 🖥️loadConfig reads server configuration from environment variables with fallback defaults.
func LoadConfig() Config {
	cwd, _ := os.Getwd()
	return Config{
		Address:          envOrDefault("COMPOSE_SERVER_ADDR", "127.0.0.1:8787"),
		DatabasePath:     envOrDefault("COMPOSE_SERVER_DB", "compose-server.db"),
		RepoRoot:         envOrDefault("COMPOSE_SERVER_REPO_ROOT", cwd),
		Token:            envOrDefault("COMPOSE_SERVER_TOKEN", ""),
		GitHubSecret:     envOrDefault("COMPOSE_SERVER_GITHUB_SECRET", ""),
		DiscordWebhook:   envOrDefault("COMPOSE_SERVER_DISCORD_WEBHOOK", ""),
		RequestBodyLimit: envOrDefaultInt64("COMPOSE_SERVER_BODY_LIMIT", 10*1024*1024),
	}
}

// 📦️envOrDefault returns the environment variable value or the fallback if empty.
func envOrDefault(key, fallback string) string {
	if value := strings.TrimSpace(os.Getenv(key)); value != "" {
		return value
	}
	return fallback
}

// 🔬️envOrDefaultInt64 returns the parsed int64 environment variable or the fallback.
func envOrDefaultInt64(key string, fallback int64) int64 {
	if value := strings.TrimSpace(os.Getenv(key)); value != "" {
		if parsed, err := strconv.ParseInt(value, 10, 64); err == nil {
			return parsed
		}
	}
	return fallback
}

// 🆔️newID mints a monotonic-prefixed unique identifier for an in-process event.
func newID() string {
	return fmt.Sprintf("%d-%d", time.Now().UTC().UnixNano(), rand.Int63())
}

// 🧭️NewMux wires every coordinator route onto a fresh multiplexer.
func NewMux(server *Server) *http.ServeMux {
	mux := http.NewServeMux()
	mux.HandleFunc("/healthz", server.handleHealth)
	mux.HandleFunc("/ticket/open", server.handleTicketOpen)
	mux.HandleFunc("/ticket/close", server.handleTicketClose)
	mux.HandleFunc("/ticket/reopen", server.handleTicketReopen)
	mux.HandleFunc("/tickets", server.handleTicketsQuery)
	mux.HandleFunc("/ticket/", server.handleTicketDetail)
	mux.HandleFunc("/diff/ingest", server.handleDiffIngest)
	mux.HandleFunc("/repo/reindex", server.handleReindex)
	mux.HandleFunc("/repo/index-file", server.handleIndexFile)
	mux.HandleFunc("/warnings", server.handleWarnings)
	mux.HandleFunc("/breachs", server.handleBreachs)
	mux.HandleFunc("/scopes", server.handleScopes)
	mux.HandleFunc("/events", server.handleEvents)
	mux.HandleFunc("/api/v1/events", server.handleEvents)
	mux.HandleFunc("/webhooks/github", server.handleGitHubWebhook)
	return mux
}

// 🛎️Service is one running coordinator, owning its store, bus and listener.
type Service struct {
	Address    string
	repository *EventRepository
	bus        *EventBus
	listener   net.Listener
}

// 🚀️Start opens the store, wires the routes and serves them on the configured address.
func Start(config Config) (*Service, error) {
	repository, err := OpenDatabase(config.DatabasePath)
	if err != nil {
		return nil, err
	}
	bus := NewEventBus(repository)
	server := NewServer(config, repository, bus)
	server.registerNotifications()
	bus.Start()
	listener, err := net.Listen("tcp", config.Address)
	if err != nil {
		bus.Stop()
		return nil, errors.Join(err, repository.Close())
	}
	service := &Service{Address: listener.Addr().String(), repository: repository, bus: bus, listener: listener}
	go func() {
		_ = http.Serve(listener, NewMux(server))
	}()
	return service, nil
}

// 🛑️Stop closes the listener, drains the bus and releases the store.
func (service *Service) Stop() error {
	closeErr := service.listener.Close()
	service.bus.Stop()
	return errors.Join(closeErr, service.repository.Close())
}

// ▶️Main is the coordinator entry point the binary delegates to.
func Main() {
	config := LoadConfig()
	service, err := Start(config)
	if err != nil {
		log.Fatal(err)
	}
	log.Printf("repo coordinator listening on %s", service.Address)
	select {}
}

// #endregion 🚀️Entrypoint
