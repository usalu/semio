// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// #endregion 🧲️Header

package main

import (
	"bufio"
	"context"
	"errors"
	"io"
	"sync"
)

// #region 🔌️Transport

type Transport interface {
	io.Reader
	io.Writer
	io.Closer
}

func (server *Server) Serve(ctx context.Context, peer string, transport Transport) error {
	var writeMu sync.Mutex
	write := func(ctx context.Context, payload []byte) error {
		if err := ctx.Err(); err != nil {
			return err
		}
		writeMu.Lock()
		defer writeMu.Unlock()
		if _, err := transport.Write(append(append([]byte(nil), payload...), '\n')); err != nil {
			return ErrPeerDropped
		}
		return nil
	}
	session, err := server.Connect(peer, write)
	if err != nil {
		return err
	}
	defer transport.Close()
	jobs := make(chan []byte, server.config.Limits.MaxHandlers)
	workerErrors := make(chan error, 1)
	var workers sync.WaitGroup
	for range server.config.Limits.MaxHandlers {
		workers.Add(1)
		server.handlerWorkers.Add(1)
		go func() {
			defer workers.Done()
			defer server.handlerWorkers.Add(-1)
			for payload := range jobs {
				server.handlerQueued.Add(-1)
				server.handlerActive.Add(1)
				response, err := session.Dispatch(ctx, payload)
				if err == nil && len(response) > 0 {
					err = write(ctx, response)
				}
				server.handlerActive.Add(-1)
				if err != nil {
					select {
					case workerErrors <- err:
					default:
					}
					session.drop(err)
				}
			}
		}()
	}
	finish := func(cause error) error {
		session.drop(cause)
		close(jobs)
		workers.Wait()
		select {
		case workerErr := <-workerErrors:
			return workerErr
		default:
			return cause
		}
	}
	scanner := bufio.NewScanner(transport)
	scanner.Buffer(make([]byte, 64*1024), server.config.Limits.MaxPayloadBytes+1)
	for scanner.Scan() {
		if err := ctx.Err(); err != nil {
			return finish(err)
		}
		payload := append([]byte(nil), scanner.Bytes()...)
		request, _, protocolError := session.decodeRequest(payload)
		if protocolError == nil && request.ID == nil {
			if _, err := session.Dispatch(ctx, payload); err != nil && !errors.Is(err, ErrStaleSession) {
				return finish(err)
			}
			continue
		}
		server.handlerQueued.Add(1)
		select {
		case jobs <- payload:
		default:
			server.handlerQueued.Add(-1)
			response, err := session.reject(payload, rpcError(CodeServerBusy, "handler queue full"))
			if err != nil {
				return finish(err)
			}
			if err := write(ctx, response); err != nil {
				return finish(err)
			}
		}
	}
	if err := scanner.Err(); err != nil {
		if errors.Is(err, bufio.ErrTooLong) {
			return finish(ErrPayloadTooLarge)
		}
		return finish(err)
	}
	if err := ctx.Err(); err != nil {
		return finish(err)
	}
	return finish(ErrPeerDropped)
}

func (session *Session) reject(payload []byte, rejection *RPCError) ([]byte, error) {
	request, id, protocolError := session.decodeRequest(payload)
	if protocolError != nil {
		rejection = protocolError
	}
	response, err := session.encodeError(id, rejection)
	if err != nil {
		return nil, err
	}
	key := ""
	if request.ID != nil {
		key = request.ID.String()
	}
	if err := session.commitExchange(key, payload, response); err != nil {
		return nil, err
	}
	return response, nil
}

// #endregion 🔌️Transport
