/** 🎬️ Statechart runtime: actors, the actor system and command routing. TypeScript twin of `🎬️runtime/🦀️.rs`. */

import { ActorId, type Command, type Machine, type MachineSpec, Snapshot, type StepReport, TimerId } from "../🟦️.ts";
import { NullInspector } from "../🔎️inspect/🟦️.ts";
import { init, macrostep, timerElapsed } from "../🧠️kernel/🟦️.ts";
import type { Host } from "../🌐️host/🟦️.ts";

//#region 🎬️Runtime
class Actor<M extends MachineSpec> {
  readonly id: ActorId;
  snapshot: Snapshot<M>;
  readonly mailbox: M["Event"][] = [];

  constructor(id: ActorId, snapshot: Snapshot<M>) {
    this.id = id;
    this.snapshot = snapshot;
  }
}

/** 🖥️ Owns every spawned actor for one machine type and routes their {@link Command}s to a {@link
 * Host}. Mailboxes drain in round-robin order until quiescent. Rust's `ActorLogic`/`MachineLogic`
 * (a marker trait + its blanket impl, abstracting the actor's associated types) has no TS
 * counterpart here — {@link MachineSpec} already supplies those types structurally, so the marker
 * layer collapses to nothing. */
export class ActorSystem<M extends MachineSpec> {
  readonly host: Host<M>;
  readonly #machine: Machine<M>;
  readonly #actors: Actor<M>[] = [];
  #nextId = 0;

  constructor(host: Host<M>, machine: Machine<M>) {
    this.host = host;
    this.#machine = machine;
  }

  /** 🆕️ Initializes and registers a root actor, routing its initial commands immediately. */
  spawnRoot(input: M["Input"]): ActorId {
    const id = ActorId(this.#nextId);
    this.#nextId += 1;
    const buffer: Command<M>[] = [];
    const snapshot = init(this.#machine, input, buffer);
    this.#actors.push(new Actor(id, snapshot));
    this.#routeCommands(id, buffer);
    return id;
  }

  /** 📸️ The current {@link Snapshot} of an actor, if it exists. */
  snapshot(id: ActorId): Snapshot<M> | undefined {
    return this.#actors.find((a) => a.id === id)?.snapshot;
  }

  /** 📮️ Enqueues an event for delivery on the next {@link ActorSystem.drain}. */
  send(to: ActorId, event: M["Event"]): void {
    this.#actors.find((a) => a.id === to)?.mailbox.push(event);
  }

  /** ⏰️ Delivers a {@link TimerId} elapsed notification straight to `macrostep`'s timer entry point for `to`. */
  timerElapsed(to: ActorId, timer: TimerId): StepReport | undefined {
    const actor = this.#actors.find((a) => a.id === to);
    if (!actor) return undefined;
    const buffer: Command<M>[] = [];
    const report = timerElapsed(this.#machine, actor.snapshot, timer, buffer, new NullInspector());
    this.#routeCommands(to, buffer);
    return report;
  }

  /** 📫️ Drains every actor's mailbox to quiescence, running one macrostep per delivered event. */
  drain(): StepReport[] {
    const reports: StepReport[] = [];
    for (;;) {
      let progressed = false;
      for (const actor of this.#actors) {
        const event = actor.mailbox.shift();
        if (event === undefined) continue;
        progressed = true;
        const buffer: Command<M>[] = [];
        const report = macrostep(this.#machine, actor.snapshot, event, buffer, new NullInspector());
        this.#routeCommands(actor.id, buffer);
        reports.push(report);
      }
      if (!progressed) break;
    }
    return reports;
  }

  #routeCommands(actor: ActorId, commands: readonly Command<M>[]): void {
    const sends: Array<readonly [ActorId, M["Event"]]> = [];
    const found = this.#actors.find((a) => a.id === actor);
    if (found) {
      for (const command of commands) {
        const pair = routeCommand(this.host, found.snapshot, actor, command);
        if (pair) sends.push(pair);
      }
    }
    for (const [to, event] of sends) this.send(to, event);
  }
}

/** 🗺️ Applies one {@link Command} to `host`/`snapshot`; returns a `send` command's `[to, event]` pair
 * for the caller to route on, since a lone {@link Host}+{@link Snapshot} pair (no {@link ActorSystem})
 * has no other actor to deliver it to. */
export function routeCommand<M extends MachineSpec>(host: Host<M>, snapshot: Snapshot<M>, actor: ActorId, command: Command<M>): readonly [ActorId, M["Event"]] | undefined {
  switch (command.kind) {
    case "effect":
      host.executeEffect(actor, command.effect);
      return undefined;
    case "raise":
      return undefined;
    case "send":
      return [command.to, command.event];
    case "emit":
      snapshot.status = { kind: "done", output: command.output };
      return undefined;
    case "startInvoke":
      host.startTask(actor, command.invoke);
      return undefined;
    case "stopInvoke":
      host.cancelTask(actor, command.invoke);
      return undefined;
    case "schedule":
      host.schedule(actor, command.timer, command.delayMs);
      return undefined;
    case "cancelTimer":
      host.cancelTimer(actor, command.timer);
      return undefined;
  }
}
//#endregion 🎬️Runtime
