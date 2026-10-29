import type { ReservationSession } from "../🗃️state/🟦️.ts";
import type { ReservationHub } from "../🧩️hub/🟦️.ts";
import type { BrowserActorChildValue } from "@semio-tech/framework-plugin/browser-bundle/child";
import type { ShardCommandIngressPage } from "@semio-tech/framework-actor/shard-client";
import { driveSpawnedJob, spawnedJobCompletedEvent } from "@semio-tech/framework-actor/wire-turn";
import { type BrowserActorSpawnedJob, type DocumentBrowserActorChild, browserActorTurnBudget } from "../../🧩️actor-wire-helpers/🟦️.ts";

declare module "../🧩️hub/🟦️.ts" {
  interface ReservationCollaborators { lanes: ReservationPollLanes }
}

/** 🛤️ Serializes every guest `poll` and job crossing on the poll lane and every turn on the turn lane, so a closed or replaced child never receives late work. */
export class ReservationPollLanes {
  private readonly session: ReservationSession;
  constructor(hub: ReservationHub) { this.session = hub.session; }

  private invokeWithPages(child: DocumentBrowserActorChild, events: BrowserActorChildValue[], commandPage: ShardCommandIngressPage | null, coldPairPage: BrowserActorChildValue | null, assertCurrent: () => void, allowClosing = false): Promise<BrowserActorChildValue> {
    const work = this.session.pollTail.then(async () => {
      if ((!allowClosing && this.session.closed) || this.session.child !== child) throw new Error("document browser actor: closed poll lane");
      if (events.length > 1) throw new Error("document browser actor: a reactor turn admits at most one event");
      assertCurrent();
      if (commandPage) await child.invoke(["reactor", "stageCommandPage"], [commandPage.cursor as unknown as BrowserActorChildValue, commandPage.bytes as unknown as BrowserActorChildValue]);
      if (coldPairPage) await child.invoke(["reactor", "stageColdPairPage"], [coldPairPage]);
      const value = await child.invoke(["reactor", "poll"], [events, browserActorTurnBudget()]);
      assertCurrent();
      return value;
    });
    this.session.pollTail = work.then(() => {}, () => {});
    return work;
  }

  private invokeJobs(child: DocumentBrowserActorChild, name: "startJob" | "stepJob" | "cancelJob", args: BrowserActorChildValue[], assertCurrent: () => void): Promise<BrowserActorChildValue> {
    const work = this.session.pollTail.then(async () => {
      if (this.session.closed || this.session.child !== child) throw new Error("document browser actor: closed poll lane");
      assertCurrent();
      const value = await child.invoke(["jobs", name], args);
      assertCurrent();
      return value;
    });
    this.session.pollTail = work.then(() => {}, () => {});
    return work;
  }

  /** 🧵️ Runs each spawned job to a terminal step through the shared {@link driveSpawnedJob} pump and returns the
   * `job-completed` events the guest is owed, in spawn order. */
  async driveSpawnedJobs(child: DocumentBrowserActorChild, jobs: readonly BrowserActorSpawnedJob[], assertCurrent: () => void): Promise<BrowserActorChildValue[]> {
    const events: BrowserActorChildValue[] = [];
    for (const spawned of jobs) {
      const completion = await driveSpawnedJob({
        ...spawned,
        port: {
          startJob: async (job, kind, input) => void (await this.invokeJobs(child, "startJob", [job, kind, input], assertCurrent)),
          stepJob: (job, budget) => this.invokeJobs(child, "stepJob", [job, { fuel: budget.fuel, deadlineMs: budget.deadlineMs }], assertCurrent),
        },
      });
      const event = spawnedJobCompletedEvent(spawned.job, completion);
      events.push({ tag: event.kind, val: event.payload } as unknown as BrowserActorChildValue);
    }
    return events;
  }

  /** 🎟️ One guest turn serves at most one event (`reactor.wit` `poll`), so a batch is a host defect refused before it crosses the child boundary; callers queue and deliver one event per poll. */
  invokePoll(child: DocumentBrowserActorChild, events: BrowserActorChildValue[], coldPairPage: BrowserActorChildValue | null, assertCurrent: () => void, allowClosing = false): Promise<BrowserActorChildValue> {
    return this.invokeWithPages(child, events, null, coldPairPage, assertCurrent, allowClosing);
  }

  invokeCommandPage(child: DocumentBrowserActorChild, page: ShardCommandIngressPage | null, assertCurrent: () => void): Promise<BrowserActorChildValue> {
    return this.invokeWithPages(child, [], page, null, assertCurrent);
  }

  enqueueTurn<T>(work: () => Promise<T>, allowClosing = false): Promise<T> {
    const operation = this.session.turnTail.then(async () => {
      if (this.session.closed && !allowClosing) throw new Error("document browser actor: closed turn lane");
      return work();
    });
    this.session.turnTail = operation.then(() => {}, () => {});
    return operation;
  }
}
