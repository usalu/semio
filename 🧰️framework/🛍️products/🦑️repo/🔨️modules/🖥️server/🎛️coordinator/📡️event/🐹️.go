// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// The asynchronous in-process event bus.

// #endregion 🧲️Header

package main

import (
	"context"
	"encoding/json"
	"errors"
	"sync"
	"time"
)

// #region 📡️Event

// Asynchronous in-process event bus for decoupled event publishing and subscription. MUST persist events to the database before dispatching.

// 🎯️EventHandler is a callback invoked when an event of a subscribed type is published.
type EventHandler func(context.Context, Event) error

type eventDispatch struct {
	ctx    context.Context
	event  Event
	result chan error
}

// 📡️EventBus is a buffered channel-based event dispatcher with persistent storage.
type EventBus struct {
	ch       chan eventDispatch
	handlers map[string][]EventHandler
	db       CoordinatorRepository
	ctx      context.Context
	cancel   context.CancelFunc
	wg       sync.WaitGroup
}

// 🗄️NewEventBus creates a new event bus backed by the given database.
// 🆕️MUST initialize the channel buffer to 256 and create a cancellable context.
func NewEventBus(db CoordinatorRepository) *EventBus {
	ctx, cancel := context.WithCancel(context.Background())
	return &EventBus{
		ch:       make(chan eventDispatch, 256),
		handlers: map[string][]EventHandler{},
		db:       db,
		ctx:      ctx,
		cancel:   cancel,
	}
}

// 🏷️Subscribe registers a handler for the given event type.
// ➕️MUST append the handler to the handlers map.
func (b *EventBus) Subscribe(eventType string, handler EventHandler) {
	b.handlers[eventType] = append(b.handlers[eventType], handler)
}

// 📬️Publish persists an event and dispatches it to subscribers.
// 💾️MUST store the event in the database before sending to the channel.
func (b *EventBus) Publish(ctx context.Context, eventType string, source string, payload interface{}) error {
	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		return err
	}
	event := Event{
		ID:        newID(),
		Kind:      eventType,
		Source:    source,
		Payload:   string(payloadBytes),
		CreatedAt: time.Now().UTC(),
	}
	if err := b.db.recordPublishedEvent(ctx, event); err != nil {
		return err
	}
	dispatch := eventDispatch{ctx: ctx, event: event, result: make(chan error, 1)}
	select {
	case b.ch <- dispatch:
	case <-b.ctx.Done():
		return errors.New("event bus closed")
	case <-ctx.Done():
		return ctx.Err()
	}
	select {
	case err := <-dispatch.result:
		return err
	case <-b.ctx.Done():
		return errors.New("event bus closed")
	case <-ctx.Done():
		return ctx.Err()
	}
}

// ▶️Start launches the event dispatch goroutine.
// ▶️MUST consume events from the channel and invoke registered handlers.
func (b *EventBus) Start() {
	b.wg.Add(1)
	go func() {
		defer b.wg.Done()
		for {
			select {
			case dispatch := <-b.ch:
				var handlerErr error
				if handlers := b.handlers[dispatch.event.Kind]; len(handlers) > 0 {
					for _, handler := range handlers {
						handlerErr = errors.Join(handlerErr, handler(dispatch.ctx, dispatch.event))
					}
				}
				dispatch.result <- handlerErr
			case <-b.ctx.Done():
				return
			}
		}
	}()
}

// ⏹️Stop cancels the event bus context and waits for the dispatch goroutine to finish.
// ⏹️MUST block until the goroutine exits.
func (b *EventBus) Stop() {
	b.cancel()
	b.wg.Wait()
}

// #endregion 📡️Event
