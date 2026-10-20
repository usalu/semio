// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Tasks: long tool calls that outlive their request. A task-augmented `tools/call` answers at once with a task; the work runs on, reports progress, can be polled, read and cancelled.

// #endregion 🧲️Header

package main

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"encoding/json"
	"errors"
	"sort"
	"sync"
	"time"
)

// #region ⏳️TaskStore

const (
	DefaultTaskTTL      int64 = 60 * 60 * 1000
	MaximumTaskTTL      int64 = 24 * 60 * 60 * 1000
	TaskPollIntervalMs  int64 = 2000
	taskTimestampFormat       = "2006-01-02T15:04:05.000Z07:00"
)

type taskEntry struct {
	task    Task
	owner   *Session
	cancel  context.CancelCauseFunc
	done    chan struct{}
	result  *CallToolResult
	failure *RPCError
	expires time.Time
	order   uint64
}

// 🗂️TaskStore keeps the tasks of one server. A working task is never forgotten; a finished one is kept for its ttl and forgotten oldest first once the store is full.
type TaskStore struct {
	mu      sync.Mutex
	entries map[string]*taskEntry
	maximum int
	counter uint64
	now     func() time.Time
}

func NewTaskStore(maximum int) *TaskStore {
	if maximum <= 0 {
		maximum = DefaultLimits().MaxTasks
	}
	return &TaskStore{entries: map[string]*taskEntry{}, maximum: maximum, now: time.Now}
}

func (store *TaskStore) stamp() string { return store.now().UTC().Format(taskTimestampFormat) }

func newTaskID() (string, error) {
	random := make([]byte, 16)
	if _, err := rand.Read(random); err != nil {
		return "", err
	}
	return hex.EncodeToString(random), nil
}

func terminalTask(status string) bool {
	return status == TaskCompleted || status == TaskFailed || status == TaskCancelled
}

// 🧹️sweep forgets finished tasks past their ttl. The caller holds the lock.
func (store *TaskStore) sweep() {
	now := store.now()
	for id, entry := range store.entries {
		if terminalTask(entry.task.Status) && !entry.expires.IsZero() && now.After(entry.expires) {
			delete(store.entries, id)
		}
	}
}

// ➕️create registers a working task owned by one session.
func (store *TaskStore) create(owner *Session, requested *int64) (*taskEntry, error) {
	id, err := newTaskID()
	if err != nil {
		return nil, err
	}
	ttl := DefaultTaskTTL
	if requested != nil && *requested > 0 {
		ttl = min(*requested, MaximumTaskTTL)
	}
	store.mu.Lock()
	defer store.mu.Unlock()
	store.sweep()
	if len(store.entries) >= store.maximum {
		var oldest *taskEntry
		for _, entry := range store.entries {
			if terminalTask(entry.task.Status) && (oldest == nil || entry.order < oldest.order) {
				oldest = entry
			}
		}
		if oldest == nil {
			return nil, ErrLimit
		}
		delete(store.entries, oldest.task.TaskID)
	}
	store.counter++
	stamp := store.stamp()
	ttlValue := ttl
	entry := &taskEntry{task: Task{TaskID: id, Status: TaskWorking, StatusMessage: "started", CreatedAt: stamp, LastUpdatedAt: stamp, TTL: &ttlValue, PollInterval: TaskPollIntervalMs}, owner: owner, done: make(chan struct{}), order: store.counter}
	store.entries[id] = entry
	return entry, nil
}

func (store *TaskStore) snapshot(entry *taskEntry) Task {
	store.mu.Lock()
	defer store.mu.Unlock()
	return entry.task
}

func (store *TaskStore) lookup(id string) (*taskEntry, bool) {
	store.mu.Lock()
	defer store.mu.Unlock()
	store.sweep()
	entry, ok := store.entries[id]
	return entry, ok
}

// 💬️message records what a working task is doing now without a notification.
func (store *TaskStore) message(entry *taskEntry, text string) {
	store.mu.Lock()
	defer store.mu.Unlock()
	if !terminalTask(entry.task.Status) {
		entry.task.StatusMessage, entry.task.LastUpdatedAt = text, store.stamp()
	}
}

// 🏁️finish ends a working task with the result or failure of its call; a task that was cancelled keeps its state. It reports whether this call ended the task.
func (store *TaskStore) finish(entry *taskEntry, result *CallToolResult, failure *RPCError) bool {
	store.mu.Lock()
	defer store.mu.Unlock()
	if terminalTask(entry.task.Status) {
		return false
	}
	entry.result, entry.failure = result, failure
	entry.task.Status = TaskCompleted
	entry.task.StatusMessage = "completed"
	if failure != nil || (result != nil && result.IsError) {
		entry.task.Status = TaskFailed
		entry.task.StatusMessage = "failed"
	}
	entry.task.LastUpdatedAt = store.stamp()
	entry.expires = store.now().Add(time.Duration(*entry.task.TTL) * time.Millisecond)
	close(entry.done)
	return true
}

// 🛑️cancel ends a working task as cancelled and stops its call. It reports whether the task was still working.
func (store *TaskStore) cancel(entry *taskEntry, cause error) bool {
	store.mu.Lock()
	if terminalTask(entry.task.Status) {
		store.mu.Unlock()
		return false
	}
	entry.task.Status, entry.task.StatusMessage, entry.task.LastUpdatedAt = TaskCancelled, "cancelled", store.stamp()
	entry.failure = &RPCError{Code: CodeRequestCancelled, Message: "task cancelled"}
	entry.expires = store.now().Add(time.Duration(*entry.task.TTL) * time.Millisecond)
	cancel := entry.cancel
	close(entry.done)
	store.mu.Unlock()
	if cancel != nil {
		cancel(cause)
	}
	return true
}

// 🔌️CancelOwner cancels every working task of a closing session.
func (store *TaskStore) CancelOwner(owner *Session, cause error) {
	store.mu.Lock()
	var owned []*taskEntry
	for _, entry := range store.entries {
		if entry.owner == owner && !terminalTask(entry.task.Status) {
			owned = append(owned, entry)
		}
	}
	store.mu.Unlock()
	for _, entry := range owned {
		store.cancel(entry, cause)
	}
}

func (store *TaskStore) list() []Task {
	store.mu.Lock()
	defer store.mu.Unlock()
	store.sweep()
	entries := make([]*taskEntry, 0, len(store.entries))
	for _, entry := range store.entries {
		entries = append(entries, entry)
	}
	sort.Slice(entries, func(left, right int) bool { return entries[left].order < entries[right].order })
	tasks := make([]Task, len(entries))
	for index, entry := range entries {
		tasks[index] = entry.task
	}
	return tasks
}

// #endregion ⏳️TaskStore

// #region 🔐️TaskSession

type taskProgress struct {
	inner ProgressReporter
	store *TaskStore
	entry *taskEntry
}

func (reporter taskProgress) Report(ctx context.Context, value float64, total *float64, message string) error {
	if message != "" {
		reporter.store.message(reporter.entry, message)
	}
	return reporter.inner.Report(ctx, value, total, message)
}

// ▶️startTask answers a task-augmented `tools/call` at once and runs the tool on.
func (session *Session) startTask(registration toolRegistration, params CallToolParams) (any, *RPCError) {
	store := session.server.tasks
	entry, err := store.create(session, params.Task.TTL)
	if err != nil {
		if errors.Is(err, ErrLimit) {
			return nil, rpcError(CodeServerBusy, "task store full")
		}
		return nil, rpcError(CodeInternalError, "internal error")
	}
	taskContext, cancel := context.WithCancelCause(context.WithValue(context.Background(), sessionContextKey{}, session))
	store.mu.Lock()
	entry.cancel = cancel
	store.mu.Unlock()
	reporter := taskProgress{inner: progress{session: session, token: params.Meta.ProgressToken()}, store: store, entry: entry}
	go func() {
		result, callError := guarded(func() (CallToolResult, error) { return registration.handler(taskContext, params, reporter) })
		var failure *RPCError
		if callError != nil {
			failure = handlerRPCError(taskContext, callError)
			result = CallToolResult{}
		}
		if store.finish(entry, &result, failure) {
			session.announce(store, entry)
		}
		cancel(nil)
	}()
	return CreateTaskResult{Task: store.snapshot(entry)}, nil
}

// 📢️announce sends `notifications/tasks/status` for the current state of a task.
func (session *Session) announce(store *TaskStore, entry *taskEntry) {
	_ = session.notify(context.Background(), "notifications/tasks/status", store.snapshot(entry))
}

func (session *Session) taskOf(raw json.RawMessage) (*taskEntry, *RPCError) {
	var params TaskIDParams
	if err := DecodeParams(raw, &params); err != nil || params.TaskID == "" {
		return nil, rpcError(CodeInvalidParams, "invalid task params")
	}
	entry, ok := session.server.tasks.lookup(params.TaskID)
	if !ok {
		return nil, rpcError(CodeInvalidParams, "task not found")
	}
	return entry, nil
}

func (session *Session) getTask(raw json.RawMessage) (any, *RPCError) {
	entry, failure := session.taskOf(raw)
	if failure != nil {
		return nil, failure
	}
	return session.server.tasks.snapshot(entry), nil
}

// 📥️taskResult waits for the task to finish and answers with the result of the original call.
func (session *Session) taskResult(ctx context.Context, raw json.RawMessage) (any, *RPCError) {
	entry, failure := session.taskOf(raw)
	if failure != nil {
		return nil, failure
	}
	select {
	case <-entry.done:
	case <-ctx.Done():
		return nil, rpcError(CodeRequestCancelled, "request cancelled")
	}
	store := session.server.tasks
	store.mu.Lock()
	result, callFailure := entry.result, entry.failure
	id := entry.task.TaskID
	store.mu.Unlock()
	if callFailure != nil {
		return nil, callFailure
	}
	related := map[string]any{RelatedTaskMeta: map[string]string{"taskId": id}}
	copied := *result
	copied.Meta = related
	return copied, nil
}

func (session *Session) listTasks(raw json.RawMessage) (any, *RPCError) {
	_, offset, callError := session.listOffset(raw)
	if callError != nil {
		return nil, callError
	}
	tasks := session.server.tasks.list()
	end, next, err := page(offset, len(tasks), session.server.config.Limits.MaxPageItems)
	if err != nil {
		return nil, rpcError(CodeInvalidParams, "invalid cursor")
	}
	return ListTasksResult{Tasks: tasks[offset:end], NextCursor: next}, nil
}

func (session *Session) cancelTask(raw json.RawMessage) (any, *RPCError) {
	entry, failure := session.taskOf(raw)
	if failure != nil {
		return nil, failure
	}
	store := session.server.tasks
	if !store.cancel(entry, errors.New("task cancelled by peer")) {
		return nil, rpcError(CodeInvalidParams, "task already finished: "+store.snapshot(entry).Status)
	}
	session.announce(store, entry)
	return store.snapshot(entry), nil
}

// #endregion 🔐️TaskSession
