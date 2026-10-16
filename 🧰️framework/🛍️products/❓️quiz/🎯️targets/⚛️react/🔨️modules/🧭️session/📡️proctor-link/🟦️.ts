/** 📡️ How the session talks to the proctor in time: the session clock, calls retried through connection shortages, the grace for views that are not projected yet and the learner's patience with a proctor that does not answer. */

import { PROJECTION_GRACE_MS } from "../📋️session-model/🟦️.ts";
import { ProctorUnavailable, isNotFound, retryTransient, type ProctorClient, type RetryTiming } from "../../🛂️proctor/🟦️.ts";

//#region 📡️ProctorLink
/** 🔗️ The proctor with the timing a session asks it by: its `timing` of retries, the `patienceMs` a learner waits for a decision and the clock. */
export class ProctorLink {
  constructor(
    readonly proctor: ProctorClient,
    readonly timing: RetryTiming,
    readonly patienceMs: number,
    private readonly clock: () => number,
  ) {}

  /** 🕰️ The session's clock in milliseconds since the Unix epoch: the instant an answer or an opening is stamped with
   * and the one every countdown is measured against. */
  now(): number {
    return this.clock();
  }

  /** 🔁️ Calls the proctor again through connection shortages, with jittered backoff, until it answers or `signal` aborts. */
  retrying<T>(call: () => Promise<T>, signal: AbortSignal): Promise<T> {
    return retryTransient(call, this.timing, signal);
  }

  /** 🏗️ Calls the proctor like {@link retrying} and waits out the projection lag: a view this client itself caused may be missing for {@link PROJECTION_GRACE_MS}. */
  projected<T>(call: (signal: AbortSignal) => Promise<T>, signal: AbortSignal): Promise<T> {
    const until = this.now() + PROJECTION_GRACE_MS;
    return this.retrying(async () => {
      try {
        return await call(signal);
      } catch (error) {
        if (isNotFound(error) && this.now() < until) throw new ProctorUnavailable("not projected yet");
        throw error;
      }
    }, signal);
  }

  /** 🧘️ Asks the proctor through `call` for as long as the learner's patience lasts; after that the call is given up as
   * a connection shortage and the proctor counts as unreachable until it answers anything again, so the next command
   * does not wait for it once more. */
  async patient<T>(call: (signal: AbortSignal) => Promise<T>, signal: AbortSignal): Promise<T> {
    const waiting = new AbortController();
    const cancel = (): void => waiting.abort(signal.reason);
    const spent = setTimeout(() => {
      this.proctor.silent();
      waiting.abort(new ProctorUnavailable("the proctor did not answer in time"));
    }, this.patienceMs);
    signal.addEventListener("abort", cancel, { once: true });
    try {
      return await call(waiting.signal);
    } finally {
      clearTimeout(spent);
      signal.removeEventListener("abort", cancel);
    }
  }
}
//#endregion 📡️ProctorLink
