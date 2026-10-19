/** 🏛️ Who decides a command: the proctor, or — when it is not the proctor's to decide right now — the deputy on the device. The routing knows when the device is ahead of the proctor or its views behind, and waits for the answers of a run to reach the proctor.
 *
 * @see ../../🫡️deputy/🟦️.ts — the deciders on the device
 */

import { type Command, type HandleView, type Id } from "@semio-tech/course";
import { commandRun, type Outbox } from "../../📮️outbox/🟦️.ts";
import { isTransient } from "../../🛂️proctor/🟦️.ts";
import { type HeldState } from "../🗃️held-state/🟦️.ts";
import { type Lifecycle } from "../🔌️lifecycle/🟦️.ts";
import { type ProctorLink } from "../📡️proctor-link/🟦️.ts";
import { type Deputising } from "../🫡️deputising/🟦️.ts";
import { decision, type Decided, type SubmissionPhase } from "../📋️session-model/🟦️.ts";

//#region 🏛️DecisionRouting
/** 🚦️ The routing of one session between its proctor and its deputy. */
export class DecisionRouting {
  constructor(
    private readonly state: HeldState,
    private readonly link: ProctorLink,
    private readonly lifecycle: Lifecycle,
    private readonly outbox: Outbox,
    private readonly deputising: Deputising,
  ) {}

  /** 🏎️ Whether the device is ahead of the proctor: a decision the deputy took still waits in the outbox, so the
   * proctor's views lack it. */
  ahead(): boolean {
    return this.deputising.present && this.outbox.waiting(decision);
  }

  /** 🐢️ Whether the proctor's view of `run` lacks a decision the deputy took that still waits in the outbox: one about
   * the run itself, a start of a run of its quiz — which may void it — or the learner's registration. Decisions about
   * other runs leave the proctor's view of this one as it is. */
  behind(run: Id): boolean {
    if (!this.deputising.present) return false;
    const { runs, learnerView } = this.state.read();
    const quiz = runs[run]?.quiz ?? learnerView?.runs.find((summary) => summary.run === run)?.quiz;
    return this.outbox.waiting((command) => decision(command) && (command.type === "identify-learner" || commandRun(command) === run || (command.type === "start-run" && (quiz === undefined || command.quiz === quiz))));
  }

  /** 🛂️ Whether a command is the proctor's to decide right now: always without a deputy; with one unless the proctor
   * is known not to answer (it did not, it asked to slow down, it speaks another contract, or the browser is offline)
   * or the device is ahead —
   * then the command would overtake what waits. A proctor nobody has heard from yet is asked. */
  direct(): boolean {
    const reached = this.link.proctor.reachability();
    return !this.deputising.present || (this.lifecycle.isOnline() && (reached === "reachable" || reached === "unknown") && !this.ahead());
  }

  /** 🔎️ Whether the deputy may decide `command` from what the device holds: an opening or a submission of an open run
   * only when its run view is held — a listing alone knows neither the run's answers nor when its tasks were opened, so
   * such a command waits for the proctor. */
  decidable(command: Command): boolean {
    if (command.type !== "open-task" && command.type !== "submit-run") return true;
    const { runs, learnerView } = this.state.read();
    return runs[command.run] !== undefined || learnerView?.runs.find((summary) => summary.run === command.run)?.status !== "open";
  }

  /** ⚖️ Has `command` decided. Without a deputy the proctor decides, however long a connection shortage lasts. With
   * one the proctor is asked once while it is the proctor's to decide, and the deputy decides whenever it is not or
   * the proctor did not answer; a sent command the proctor did decide is the same command when it arrives again. */
  async decided(command: Command, signal: AbortSignal): Promise<Decided> {
    if (!this.deputising.present || !this.decidable(command)) return { verdict: await this.link.retrying(() => this.link.proctor.command(command, signal), signal), by: "proctor" };
    if (this.direct()) {
      try {
        return { verdict: await this.link.patient((waiting) => this.link.proctor.command(command, waiting), signal), by: "proctor" };
      } catch (error) {
        if (signal.aborted || !isTransient(error)) throw error;
      }
    }
    return { verdict: this.deputising.deputise(command), by: "deputy" };
  }

  /** 🔦️ Who holds `handle`: the proctor's answer — or, when it is not the proctor's to say right now, the deputy's,
   * which knows no holder. */
  async holding(handle: string, signal: AbortSignal): Promise<HandleView> {
    if (!this.deputising.present) return this.link.retrying(() => this.link.proctor.handle(handle, signal), signal);
    if (this.direct()) {
      try {
        return await this.link.patient((waiting) => this.link.proctor.handle(handle, waiting), signal);
      } catch (error) {
        if (signal.aborted || !isTransient(error)) throw error;
      }
    }
    return this.deputising.handle(handle);
  }

  /** 📤️ Waits until every answer of `run` reached the proctor, reporting the progress, and says whether they all did.
   * With a deputy the wait ends as soon as the run is no longer the proctor's to decide, and at the latest when the
   * learner's patience is spent: the answers go on waiting, and the submission must wait behind them. */
  async saved(run: Id, signal: AbortSignal, onPhase: (phase: SubmissionPhase) => void): Promise<boolean> {
    const progress = (done: number, total: number): void => onPhase({ phase: "saving", done, total });
    if (!this.deputising.present) return this.outbox.settled(run, signal, progress).then(() => true);
    const away = new AbortController();
    const leave = (): void => away.abort();
    const unsubscribe = this.link.proctor.subscribe(() => this.direct() || leave());
    const spent = setTimeout(leave, this.link.patienceMs);
    signal.addEventListener("abort", leave, { once: true });
    try {
      await this.outbox.settled(run, away.signal, progress);
      return true;
    } catch (error) {
      if (signal.aborted) throw signal.reason ?? error;
      return false;
    } finally {
      unsubscribe();
      clearTimeout(spent);
      signal.removeEventListener("abort", leave);
    }
  }
}
//#endregion 🏛️DecisionRouting
