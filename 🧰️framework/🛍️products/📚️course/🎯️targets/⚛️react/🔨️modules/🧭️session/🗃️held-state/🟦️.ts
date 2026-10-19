/** 🗃️ What the session holds for the learner: the one current {@link CourseState}, the frozen snapshot the screens read and the listeners they subscribed. The state only moves by folding a {@link CourseClientEvent} with {@link evolveCourseState}, and what moves is persisted. */

import { type Id } from "@semio-tech/course";
import { type LocalStore } from "../../💾️persistence/🟦️.ts";
import { persistCourseState } from "../💾️restore/🟦️.ts";
import { evolveCourseState, type CourseClientEvent, type CourseState } from "../🧮️state/🟦️.ts";
import { type CourseConnection, type CourseSnapshot } from "../📋️session-model/🟦️.ts";

//#region 🗃️HeldState
/** 🧊️ The state of one session with its snapshot and listeners; `connection` says what the learner sees of the link whenever a snapshot is frozen. */
export class HeldState {
  private state: CourseState;
  private current: CourseSnapshot;
  private readonly listeners = new Set<() => void>();

  constructor(
    private readonly store: LocalStore,
    private readonly connection: () => CourseConnection,
    initial: CourseState,
  ) {
    this.state = initial;
    this.current = this.freeze();
  }

  /** 🎞️ The current snapshot; the same object until something changes. */
  snapshot = (): CourseSnapshot => this.current;

  /** 🔔️ Calls `listener` on every change; returns the unsubscribe function. */
  subscribe = (listener: () => void): (() => void) => {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  };

  /** 📖️ The state now. */
  read(): CourseState {
    return this.state;
  }

  /** 🧑‍🎓️ The id of the learner this device acts as, if any. */
  learnerId(): Id | undefined {
    return this.state.learner?.id;
  }

  /** ⚡️ Folds `event` into the state; a change is written back to the store unless `persist` is off (what another tab stored, or a view the deputy made up) and told to the listeners. */
  dispatch(event: CourseClientEvent, persist = true): void {
    const before = this.state;
    this.state = evolveCourseState(before, event);
    if (this.state === before) return;
    if (persist) persistCourseState(this.store, before, this.state);
    this.notify();
  }

  /** 🔁️ Replaces the state outright, without telling anyone: the caller announces it with {@link notify} once it is done. */
  reset(state: CourseState): void {
    this.state = state;
  }

  /** 📣️ Freezes a new snapshot and calls every listener. */
  notify(): void {
    this.current = this.freeze();
    for (const listener of [...this.listeners]) listener();
  }

  private freeze(): CourseSnapshot {
    return { state: this.state, connection: this.connection() };
  }
}
//#endregion 🗃️HeldState
