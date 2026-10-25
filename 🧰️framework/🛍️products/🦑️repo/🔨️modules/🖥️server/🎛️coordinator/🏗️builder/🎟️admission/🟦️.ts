import {countedCompilerCapacityRequestV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { acquireCompilerCapacityLeaseV1, type CompilerAdmissionControl } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { createRequestedIssuedResourceControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🎟️ One development rebuild shares its host claim across overlapping compilers and releases it while idle. */
export function createNextRebuildAdmissionV1(options: Readonly<{ directory: string; environment: Readonly<Record<string, string | undefined>>; run: (work: (control: CompilerAdmissionControl) => Promise<void>) => Promise<void> }>): Readonly<{ begin: (id: string) => Promise<void>; finish: (id: string) => Promise<void> }> {
  if (schema.definitions.Protocol.const.idleClaims !== 0) throw Error("Development rebuild must release idle capacity");
  type Group = { participants: Set<string>; admitted: boolean; controller: AbortController; granted: Promise<void>; accept: () => void; refuse: (error: unknown) => void; end: () => void; operation: Promise<void> };
  let current: Group | undefined;
  const participants = new Map<string, Group>();
  const create = (): Group => {
    let accept!: () => void, refuse!: (error: unknown) => void, end!: () => void;
    const granted = new Promise<void>((resolve, reject) => { accept = resolve; refuse = reject; }), ended = new Promise<void>(resolve => { end = resolve; });
    granted.catch(() => undefined);
    const group: Group = { participants: new Set(), admitted: false, controller: new AbortController(), granted, accept, refuse, end, operation: Promise.resolve() };
    group.operation = options.run(async original => {
      const control: CompilerAdmissionControl = createRequestedIssuedResourceControlV1(original, 0, group.controller.signal);
      const lease = await acquireCompilerCapacityLeaseV1({request:countedCompilerCapacityRequestV1(), directory: options.directory, control, environment: options.environment });
      try { group.admitted = true; group.accept(); await ended; }
      finally { lease.release(); }
    }).catch(group.refuse);
    return group;
  };
  return Object.freeze({
    async begin(id: string): Promise<void> {
      if (!id || participants.has(id)) throw Error("Development compiler already owns its rebuild phase");
      const group = current ??= create();
      group.participants.add(id); participants.set(id, group);
      await group.granted;
    },
    async finish(id: string): Promise<void> {
      const group = participants.get(id);
      if (!group) return;
      participants.delete(id); group.participants.delete(id);
      if (group.participants.size) return;
      if (current === group) current = undefined;
      if (!group.admitted) group.controller.abort(Error("Queued development compilation closed"));
      group.end(); await group.operation;
    },
  });
}

/** 🪝️ Binds only the public Webpack compiler lifecycle, without exposing Webpack types through the owner API. */
export function nextRebuildPluginV1(gate: ReturnType<typeof createNextRebuildAdmissionV1>): Readonly<{ apply: (compiler: unknown) => void }> {
  type AsyncHook = { tapPromise: (name: string, run: (...args: unknown[]) => Promise<void>) => void };
  type SyncHook = { tap: (name: string, run: (...args: unknown[]) => void) => void };
  let sequence = 0;
  return Object.freeze({ apply(value: unknown): void {
    const compiler = value as { name?: string; hooks: { beforeCompile: AsyncHook; done: AsyncHook; failed: SyncHook; watchClose: SyncHook } };
    if (!compiler?.hooks || !compiler.hooks.beforeCompile?.tapPromise || !compiler.hooks.done?.tapPromise || !compiler.hooks.failed?.tap || !compiler.hooks.watchClose?.tap) throw Error("Complete development compiler lifecycle required");
    const name = "SemioDevelopmentRebuildAdmission", id = `${compiler.name ?? "compiler"}:${++sequence}`;
    compiler.hooks.beforeCompile.tapPromise(name, () => gate.begin(id));
    compiler.hooks.done.tapPromise(name, () => gate.finish(id));
    compiler.hooks.failed.tap(name, () => { void gate.finish(id); });
    compiler.hooks.watchClose.tap(name, () => { void gate.finish(id); });
  } });
}
