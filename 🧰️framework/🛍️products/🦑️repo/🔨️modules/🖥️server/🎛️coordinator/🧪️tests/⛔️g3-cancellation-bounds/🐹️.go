// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Phase 9 G3 event-store cancellation, bounds and unavailable-store contract tests.

// #endregion 🧲️Header

package main

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"testing"
	"time"
)

// #region ⛔️G3CancellationBounds

func TestG3CancellationBeforeDuringAndAfterCommit(t *testing.T) {
	t.Run("before", func(t *testing.T) {
		store, path := testStore(t)
		ctx, cancel := context.WithCancel(context.Background())
		cancel()
		_, err := store.Append(ctx, 0, []EventInput{testInput("before", `{}`)}, nil)
		if !errors.Is(err, context.Canceled) {
			t.Fatalf("error=%v", err)
		}
		if _, statErr := os.Stat(path); !os.IsNotExist(statErr) {
			t.Fatalf("cancel-before created log: %v", statErr)
		}
	})
	t.Run("during", func(t *testing.T) {
		store, path := testStore(t)
		ctx, cancel := context.WithCancel(context.Background())
		_, err := store.Append(ctx, 0, []EventInput{testInput("during", `{}`)}, func(progress StoreProgress) {
			if progress.Phase == "stage-synced" {
				cancel()
			}
		})
		if !errors.Is(err, context.Canceled) {
			t.Fatalf("error=%v", err)
		}
		if _, statErr := os.Stat(path); !os.IsNotExist(statErr) {
			t.Fatalf("cancel-during created log: %v", statErr)
		}
	})
	t.Run("after", func(t *testing.T) {
		store, _ := testStore(t)
		ctx, cancel := context.WithCancel(context.Background())
		result, err := store.Append(ctx, 0, []EventInput{testInput("after", `{}`)}, func(progress StoreProgress) {
			if progress.Phase == "committed" {
				cancel()
			}
		})
		if err != nil || len(result.Events) != 1 {
			t.Fatalf("committed append result=%+v err=%v", result, err)
		}
		events, replayErr := store.Replay(context.Background(), nil)
		if replayErr != nil || len(events) != 1 {
			t.Fatalf("events=%+v err=%v", events, replayErr)
		}
	})
	t.Run("during-replay", func(t *testing.T) {
		store, _ := testStore(t)
		_, err := store.Append(context.Background(), 0, []EventInput{testInput("replay-a", `{}`), testInput("replay-b", `{}`)}, nil)
		if err != nil {
			t.Fatal(err)
		}
		ctx, cancel := context.WithCancel(context.Background())
		_, err = store.Replay(ctx, func(progress StoreProgress) {
			if progress.Phase == "replayed" && progress.Current == 1 {
				cancel()
			}
		})
		if !errors.Is(err, context.Canceled) {
			t.Fatalf("replay cancellation error=%v", err)
		}
	})
}

func TestG3PayloadEventDepthAndLogBounds(t *testing.T) {
	limits := DefaultStoreLimits()
	limits.MaxPayloadBytes = len(`{"a":1}`)
	limits.MaxAppendEvents = 2
	limits.MaxJSONDepth = 2
	limits.MaxLogBytes = 4096
	limits.MaxAppendBytes = 2048
	path := filepath.Join(t.TempDir(), "bounded.events")
	store, err := OpenEventStore(context.Background(), path, limits)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := store.Append(context.Background(), 0, []EventInput{testInput("maximum", `{"a":1}`)}, nil); err != nil {
		t.Fatalf("maximum payload rejected: %v", err)
	}
	if _, err := store.Append(context.Background(), 1, []EventInput{testInput("max-plus-one", `{"a":10}`)}, nil); !errors.Is(err, ErrStoreLimit) {
		t.Fatalf("payload max+1 error=%v", err)
	}
	if _, err := store.Append(context.Background(), 1, []EventInput{testInput("one", `{}`), testInput("two", `{}`), testInput("three", `{}`)}, nil); !errors.Is(err, ErrStoreLimit) {
		t.Fatalf("event max+1 error=%v", err)
	}
	if _, err := store.Append(context.Background(), 1, []EventInput{testInput("depth-maximum", `[[]]`)}, nil); err != nil {
		t.Fatalf("maximum depth rejected: %v", err)
	}
	if _, err := store.Append(context.Background(), 2, []EventInput{testInput("depth-plus-one", `[[[]]]`)}, nil); !errors.Is(err, ErrStoreLimit) {
		t.Fatalf("depth max+1 error=%v", err)
	}
	invalidScalar := testInput("invalid\x00id", `{}`)
	if _, err := store.Append(context.Background(), 2, []EventInput{invalidScalar}, nil); err == nil {
		t.Fatal("NUL-containing identity was accepted")
	}
	logLimits := DefaultStoreLimits()
	logLimits.MaxLogBytes = 1
	logStore, err := OpenEventStore(context.Background(), filepath.Join(t.TempDir(), "log.events"), logLimits)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := logStore.Append(context.Background(), 0, []EventInput{testInput("too-large", `{}`)}, nil); !errors.Is(err, ErrStoreLimit) {
		t.Fatalf("log max+1 error=%v", err)
	}
	replayPath := filepath.Join(t.TempDir(), "replay.events")
	replayStore, err := OpenEventStore(context.Background(), replayPath, DefaultStoreLimits())
	if err != nil {
		t.Fatal(err)
	}
	if _, err := replayStore.Append(context.Background(), 0, []EventInput{testInput("replay-one", `{}`), testInput("replay-two", `{}`)}, nil); err != nil {
		t.Fatal(err)
	}
	replayLimits := DefaultStoreLimits()
	replayLimits.MaxReplayEvents = 1
	if _, err := OpenEventStore(context.Background(), replayPath, replayLimits); !errors.Is(err, ErrStoreLimit) {
		t.Fatalf("replay event max+1 error=%v", err)
	}
}

func TestG3UnavailableStoreAndBoundedLockWait(t *testing.T) {
	parent := filepath.Join(t.TempDir(), "not-a-directory")
	if err := os.WriteFile(parent, []byte("x"), 0o600); err != nil {
		t.Fatal(err)
	}
	if _, err := OpenEventStore(context.Background(), filepath.Join(parent, "events"), DefaultStoreLimits()); !errors.Is(err, ErrStoreUnavailable) {
		t.Fatalf("unavailable error=%v", err)
	}
	store, path := testStore(t)
	if err := os.WriteFile(path+".lock", []byte("busy"), 0o600); err != nil {
		t.Fatal(err)
	}
	ctx, cancel := context.WithTimeout(context.Background(), 15*time.Millisecond)
	defer cancel()
	started := time.Now()
	_, err := store.Replay(ctx, nil)
	if !errors.Is(err, context.DeadlineExceeded) {
		t.Fatalf("lock wait error=%v", err)
	}
	if time.Since(started) > time.Second {
		t.Fatal("lock shortage froze past the explicit deadline")
	}
	_ = os.Remove(path + ".lock")
	if events, replayErr := store.Replay(context.Background(), nil); replayErr != nil || len(events) != 0 {
		t.Fatalf("store did not recover after external shortage: events=%v err=%v", events, replayErr)
	}
	unlock, err := store.lock(context.Background())
	if err != nil {
		t.Fatal(err)
	}
	ctx, cancel = context.WithTimeout(context.Background(), 15*time.Millisecond)
	started = time.Now()
	_, err = store.Replay(ctx, nil)
	cancel()
	unlock()
	if !errors.Is(err, context.DeadlineExceeded) || time.Since(started) > time.Second {
		t.Fatalf("in-process shortage error=%v duration=%s", err, time.Since(started))
	}
}

// #endregion ⛔️G3CancellationBounds
