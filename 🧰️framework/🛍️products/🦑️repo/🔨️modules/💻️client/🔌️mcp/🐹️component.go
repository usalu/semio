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

func main() {
	if len(os.Args) > 1 {
		if err := client.RunCLI(); err != nil {
			var exitErr client.ExitError
			if errors.As(err, &exitErr) {
				os.Exit(exitErr.Code)
			}
			fmt.Fprintln(os.Stderr, err)
			os.Exit(1)
		}
		return
	}
	if _, err := runMCP(context.Background(), stdioTransport{reader: os.Stdin, writer: os.Stdout}, ClientRepository{}); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func runMCP(ctx context.Context, transport Transport, repository RepositoryHandlers) (*Server, error) {
	server, err := NewRepositoryServer(repository)
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
