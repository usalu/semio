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
	"os/signal"
	"syscall"
	"time"
 cli "github.com/usalu/semio/repo/cli"
	"github.com/usalu/semio/repo/providers"
	"github.com/usalu/semio/repo/workspace"
)

// #endregion 🤸️Preamble

// #region 🦀️Mcp
// MCP process bootstrap.

const MCPProfileEnvironment = "SEMIO_REPO_MCP_CLIENT"

func resolveMCPProfile(arguments []string, lookupEnvironment func(string) (string, bool)) (providers.McpClientKind, error) {
	if len(arguments) != 1 || arguments[0] != "mcp" {
		return "", errors.New("Expected repo mcp without additional arguments")
	}
	raw, _ := lookupEnvironment(MCPProfileEnvironment)
	return providers.ParseMcpClientKind(raw)
}

func main() {
 if len(os.Args) < 2 || os.Args[1] != "mcp" {
  if err := cli.RunCLI(); err != nil { var exited workspace.ExitError; if errors.As(err, &exited) { os.Exit(exited.Code) }; fmt.Fprintln(os.Stderr, err); os.Exit(1) }; return
 }
	profile, err := resolveMCPProfile(os.Args[1:], os.LookupEnv)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	if err := workspace.BindWorkingDirectory(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()
	if _, err := runMCPForProfile(ctx, stdioTransport{reader: os.Stdin, writer: os.Stdout}, NewClientRepository(profile), profile); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func runMCP(ctx context.Context, transport Transport, repository RepositoryHandlers) (*Server, error) {
	return runMCPForProfile(ctx, transport, repository, providers.McpClientGeneric)
}

func runMCPForProfile(ctx context.Context, transport Transport, repository RepositoryHandlers, profile providers.McpClientKind) (*Server, error) {
	started := time.Now()
	server, err := NewRepositoryServerFor(repository, profile)
	if err != nil {
		return nil, err
	}
	buildTools := registerBuildSurface(server)
	fmt.Fprintf(os.Stderr, "[repo-mcp] ready pid=%d profile=%s tools=%d buildTools=%d in %s\n", os.Getpid(), profile, len(server.tools), buildTools, time.Since(started).Round(time.Millisecond))
	return serveMCP(ctx, transport, server)
}

// 🏗️registerBuildSurface adds the tools derived from the neutral catalogue and reports how many; when the repository has no policy or the catalogue does not derive, the server keeps serving tickets and goals and says why on stderr.
func registerBuildSurface(server *Server) int {
	root, bound := workspace.LookupRootDir()
	if !bound {
		return 0
	}
	surface, err := LoadBuildSurface(root)
	if err != nil {
		if !errors.Is(err, os.ErrNotExist) {
			fmt.Fprintf(os.Stderr, "[repo-mcp] build tools unavailable: %v\n", err)
		}
		return 0
	}
	if err := surface.Register(server); err != nil {
		fmt.Fprintf(os.Stderr, "[repo-mcp] build tools unavailable: %v\n", err)
		return 0
	}
	return len(surface.Tools)
}

func serveMCP(ctx context.Context, transport Transport, server *Server) (*Server, error) {
	err := server.Serve(ctx, "stdio", transport)
	if errors.Is(err, ErrPeerDropped) {
		err = nil
	}
	return server, err
}

// #endregion 🦀️Mcp
