// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

// Repo MCP entry point.

// #endregion 🧲️Header

// #region 🤸️Preamble
// Package declaration and dependency imports for the repo MCP entry point.

package main

import (
	"context"
	"errors"
	"fmt"
	"os"

	"github.com/usalu/semio/repo/client"
)

// #endregion 🤸️Preamble

// #region 🦀️Mcp
// MCP process bootstrap.

const MCPProfileEnvironment = "SEMIO_REPO_MCP_CLIENT"

func resolveMCPProfile(arguments []string, lookupEnvironment func(string) (string, bool)) (client.McpClientKind, error) {
	if len(arguments) != 0 {
		return "", errors.New("repo MCP accepts no command arguments")
	}
	raw, _ := lookupEnvironment(MCPProfileEnvironment)
	return client.ParseMcpClientKind(raw)
}

func main() {
	profile, err := resolveMCPProfile(os.Args[1:], os.LookupEnv)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	if _, err := runMCPForProfile(context.Background(), stdioTransport{reader: os.Stdin, writer: os.Stdout}, NewClientRepository(profile), profile); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func runMCP(ctx context.Context, transport Transport, repository RepositoryHandlers) (*Server, error) {
	return runMCPForProfile(ctx, transport, repository, client.McpClientGeneric)
}

func runMCPForProfile(ctx context.Context, transport Transport, repository RepositoryHandlers, profile client.McpClientKind) (*Server, error) {
	server, err := NewRepositoryServerFor(repository, profile)
	if err != nil {
		return nil, err
	}
	return serveMCP(ctx, transport, server)
}

func serveMCP(ctx context.Context, transport Transport, server *Server) (*Server, error) {
	err := server.Serve(ctx, "stdio", transport)
	if errors.Is(err, ErrPeerDropped) {
		err = nil
	}
	return server, err
}

// #endregion 🦀️Mcp
