/** 📋️ What a session talks to and what it reports: failures, refresh outcomes, submission phases, connection and snapshot, options, the deputy's patience and the small deciders around a command. */

import { type Command, type Event, type LearnerRegisteredEvent, type Rejection } from "@semio-tech/course";
import type { Deputy } from "../../🫡️deputy/🟦️.ts";
import { type LocalStore } from "../../💾️persistence/🟦️.ts";
import { type OutboxActivity } from "../../📮️outbox/🟦️.ts";
import { type CommandVerdict, type ProctorClient, type ProctorContract, type ProctorReachability, type RetryTiming } from "../../🛂️proctor/🟦️.ts";
import { type CourseState } from "../🧮️state/🟦️.ts";

/** 🚫️ Why an interactive command did not go through: the proctor rejected it by a rule of the quiz, refused it for a
 * reason of its own, or — `waiting` — takes none of its kind from this client's address until the `allowance` it names
 * has room again, in `retryAfterMs` when it said how long. */
export type SessionFailure = { readonly kind: "rejected"; readonly rejection: Rejection } | { readonly kind: "refused"; readonly detail: string } | { readonly kind: "waiting"; readonly allowance: string; readonly retryAfterMs?: number };

/** 🔄️ How a background refresh went: the proctor answered, or it did not — then with the wait it asked for, if it
 * said one (a rate limit). */
export type RefreshOutcome = { readonly answered: true } | { readonly answered: false; readonly retryAfterMs?: number };

/** 📊️ The progress of a submission. */
export type SubmissionPhase = { readonly phase: "saving"; readonly done: number; readonly total: number } | { readonly phase: "submitting" } | { readonly phase: "results" };

/** 📶️ What the learner sees of the connection; with a `deputy` the device goes on deciding while the proctor is away —
 * also while it speaks another `contract`, which is named as long as it does. */
export interface CourseConnection {
  readonly reachability: ProctorReachability;
  readonly online: boolean;
  readonly pending: number;
  readonly activity: OutboxActivity;
  readonly deputy: boolean;
  readonly contract?: ProctorContract;
}

/** 📸️ One consistent render input: the state and the connection. */
export interface CourseSnapshot {
  readonly state: CourseState;
  readonly connection: CourseConnection;
}

/** ⚙️ What a {@link CourseSession} talks to; with a `deputy` it decides by itself while the proctor is away, and a
 * learner waits `patienceMs` for the proctor's own decision before the deputy takes it. */
export interface CourseSessionOptions {
  readonly proctor: ProctorClient;
  readonly store: LocalStore;
  readonly deputy?: Deputy;
  readonly patienceMs?: number;
  readonly timing?: RetryTiming;
  readonly now?: () => number;
}

/** 🪑️ Where one session keeps its deputy: seated when the session is made, or later once the site's material has arrived. */
export interface DeputySeat {
  current: Deputy | undefined;
}

/** ⏳️ How long a learner waits for the proctor to decide a command before the deputy decides it: long enough for a
 * proctor that answers at all, far shorter than the time a request to a silent host takes to give up. */
export const DEPUTY_PATIENCE_MS = 3_000;

/** ⚖️ A command decided, and by whom: the proctor, or the deputy on the device. */
export interface Decided {
  readonly verdict: CommandVerdict;
  readonly by: "proctor" | "deputy";
}

/** 🧑‍⚖️ Whether `command` asks for a decision the deputy can take; an answer is recorded, not decided. */
export function decision(command: Command): boolean {
  return command.type !== "record-answer";
}

/** 🕐️ How long a view this client itself caused may still be missing from the proctor's projections. */
export const PROJECTION_GRACE_MS = 10_000;

export function stoppedLifetime(): AbortController {
  const lifetime = new AbortController();
  lifetime.abort();
  return lifetime;
}

export function describe(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

export function failureOf(verdict: Exclude<CommandVerdict, { readonly kind: "accepted" }>): SessionFailure {
  return verdict.kind === "rejected" ? { kind: "rejected", rejection: verdict.rejection } : { kind: "refused", detail: verdict.detail };
}

export function registration(event: Event): event is LearnerRegisteredEvent {
  return event.type === "learner-registered";
}

/** 🔁️ How often a registration is tried again after the handle (or, for an anonymous learner, the fresh id) turned out
 * to be taken between asking and registering. */
export const IDENTIFY_ATTEMPTS = 3;

