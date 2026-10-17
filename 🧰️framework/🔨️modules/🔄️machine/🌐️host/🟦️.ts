/** 🌐️ Statechart hosts: the `Host` port a machine runs on, the native host and the deterministic test host. TypeScript twin of `🌐️host/🦀️.rs`. */

import { ActorId, InvokeId, type MachineSpec, TimerId } from "../🟦️.ts";

//#region 🌐️Host
/** 🌐 Executes the side effects a {@link Command} describes. No async method — hosts own their own
 * tasks/timers and report completion back as ordinary events. */
export interface Host<M extends MachineSpec> {
  /** 🎇 Executes a consumer-defined effect requested by a running actor. */
  executeEffect(actor: ActorId, effect: M["Effect"]): void;
  /** ⏱ Schedules a delayed-transition timer for the given actor. */
  schedule(actor: ActorId, timer: TimerId, delayMs: number): void;
  /** ⏱ Cancels a previously scheduled timer (invoked when its owning state exits). */
  cancelTimer(actor: ActorId, timer: TimerId): void;
  /** 🚀 Starts the task/actor backing an `invoke` declaration. */
  startTask(actor: ActorId, invoke: InvokeId): void;
  /** 🛑 Stops a previously started task (invoked when its owning state exits). */
  cancelTask(actor: ActorId, invoke: InvokeId): void;
  /** 🕰 The host's current clock reading, in milliseconds. */
  nowMs(): number;
}

/** 🖥 A synchronous, wall-clock-backed {@link Host} for native (non-browser-driven) callers. Timers
 * are polled by the caller via {@link NativeHost.dueTimers} rather than firing on their own — keeping
 * the whole runtime single-threaded per actor, same as Rust's `NativeHost`. */
export class NativeHost<M extends MachineSpec> implements Host<M> {
  readonly #start = Date.now();
  readonly #effects: Array<[ActorId, M["Effect"]]> = [];
  readonly #pendingTimers: Array<[ActorId, TimerId, number]> = [];
  readonly #startedTasks: Array<[ActorId, InvokeId]> = [];

  /** 🎇 Effects recorded so far, in emission order. */
  effects(): ReadonlyArray<readonly [ActorId, M["Effect"]]> {
    return this.#effects;
  }

  /** 🎇 Drains and returns every recorded effect. */
  drainEffects(): Array<[ActorId, M["Effect"]]> {
    return this.#effects.splice(0, this.#effects.length);
  }

  /** 🚀 Tasks started via `invoke`, still pending cancellation. */
  startedTasks(): ReadonlyArray<readonly [ActorId, InvokeId]> {
    return this.#startedTasks;
  }

  /** ⏱ Removes and returns every timer whose deadline has passed. */
  dueTimers(): Array<[ActorId, TimerId]> {
    const now = this.nowMs();
    const due: Array<[ActorId, TimerId]> = [];
    const remaining = this.#pendingTimers.filter(([actor, timer, at]) => {
      if (at > now) return true;
      due.push([actor, timer]);
      return false;
    });
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...remaining);
    return due;
  }

  executeEffect(actor: ActorId, effect: M["Effect"]): void {
    this.#effects.push([actor, effect]);
  }

  schedule(actor: ActorId, timer: TimerId, delayMs: number): void {
    this.#pendingTimers.push([actor, timer, this.nowMs() + delayMs]);
  }

  cancelTimer(actor: ActorId, timer: TimerId): void {
    const kept = this.#pendingTimers.filter(([a, t]) => !(a === actor && t === timer));
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...kept);
  }

  startTask(actor: ActorId, invoke: InvokeId): void {
    this.#startedTasks.push([actor, invoke]);
  }

  cancelTask(actor: ActorId, invoke: InvokeId): void {
    const kept = this.#startedTasks.filter(([a, i]) => !(a === actor && i === invoke));
    this.#startedTasks.length = 0;
    this.#startedTasks.push(...kept);
  }

  nowMs(): number {
    return Date.now() - this.#start;
  }
}

/** 🧪 A {@link Host} with a caller-driven simulated clock — never sleeps in real time. */
export class TestHost<M extends MachineSpec> implements Host<M> {
  #clockMs = 0;
  readonly #effects: Array<[ActorId, M["Effect"]]> = [];
  readonly #pendingTimers: Array<[ActorId, TimerId, number]> = [];
  readonly #startedTasks: Array<[ActorId, InvokeId]> = [];
  readonly #cancelledTasks: Array<[ActorId, InvokeId]> = [];

  /** 🎇 Effects recorded so far, in emission order. */
  effects(): ReadonlyArray<readonly [ActorId, M["Effect"]]> {
    return this.#effects;
  }

  /** 🚀 Tasks currently started (not yet cancelled), for invoke-lifecycle assertions. */
  startedTasks(): ReadonlyArray<readonly [ActorId, InvokeId]> {
    return this.#startedTasks;
  }

  /** 🛑 Tasks that have been cancelled, for invoke-lifecycle assertions. */
  cancelledTasks(): ReadonlyArray<readonly [ActorId, InvokeId]> {
    return this.#cancelledTasks;
  }

  /** ⏱ Advances the simulated clock and returns timers that became due, removing them. */
  advance(delayMs: number): Array<[ActorId, TimerId]> {
    this.#clockMs += delayMs;
    const now = this.#clockMs;
    const due: Array<[ActorId, TimerId]> = [];
    const remaining = this.#pendingTimers.filter(([actor, timer, at]) => {
      if (at > now) return true;
      due.push([actor, timer]);
      return false;
    });
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...remaining);
    return due;
  }

  executeEffect(actor: ActorId, effect: M["Effect"]): void {
    this.#effects.push([actor, effect]);
  }

  schedule(actor: ActorId, timer: TimerId, delayMs: number): void {
    this.#pendingTimers.push([actor, timer, this.#clockMs + delayMs]);
  }

  cancelTimer(actor: ActorId, timer: TimerId): void {
    const kept = this.#pendingTimers.filter(([a, t]) => !(a === actor && t === timer));
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...kept);
  }

  startTask(actor: ActorId, invoke: InvokeId): void {
    this.#startedTasks.push([actor, invoke]);
  }

  cancelTask(actor: ActorId, invoke: InvokeId): void {
    const kept = this.#startedTasks.filter(([a, i]) => !(a === actor && i === invoke));
    this.#startedTasks.length = 0;
    this.#startedTasks.push(...kept);
    this.#cancelledTasks.push([actor, invoke]);
  }

  nowMs(): number {
    return this.#clockMs;
  }
}
//#endregion 🌐️Host
