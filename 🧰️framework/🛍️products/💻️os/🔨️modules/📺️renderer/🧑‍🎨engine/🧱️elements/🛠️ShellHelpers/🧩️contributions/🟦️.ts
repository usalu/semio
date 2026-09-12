// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/ShellHelpers/contributions/component.ts
/** @emoji 🧩️ The host→guest contributions push as an owned, per-instance cancellable unit — kept in
 * its OWN module (no React, no shell imports) so a law can drive it without pulling the shell's
 * element graph, the way `🏛️ShellHost/🩺️fault` is kept apart from `🏛️ShellHost` itself.
 * Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.
 */
// #endregion 🧲️Header

/** 🧩️ The session a contributions closure is installed INTO — the receiver, never the contributor. */
export type ContributionsSessionKey = { readonly pluginId: string; readonly instanceId: number };

/** 🕸️ The operator kinds one session's document (or, when the document is still genesis, its app's
 * published examples) reaches — the cut `scopeContributionsJson` applies to the loaded closure. */
export type ContributionsOperatorScope = { readonly status: "resolved"; readonly kinds: readonly string[] } | { readonly status: "unresolved"; readonly reason: string };

/** 🧩️ What ONE `publish` call actually did, so a caller logs a decision instead of inferring one. */
export type ContributionsPublishOutcome =
  | { readonly status: "installed"; readonly chars: number; readonly kinds: readonly string[] }
  | { readonly status: "unchanged" }
  | { readonly status: "unresolved"; readonly reason: string }
  | { readonly status: "empty" }
  | { readonly status: "retired" }
  | { readonly status: "failed"; readonly reason: string };

/** 🔌️ Everything the publisher needs from its host, as four ports over an `environment` the caller
 * captures per publish (the loaded plugin closure, the host/focused mode, the disabled extensions).
 * The environment is captured by the FIRST caller of a run — a later caller that finds the same
 * registry generation JOINS that run instead of starting a second one. */
export type ContributionsPublisherPorts<E> = {
  readonly registryGeneration: (environment: E) => string;
  readonly resolveScope: (session: ContributionsSessionKey, environment: E) => Promise<ContributionsOperatorScope>;
  readonly buildPack: (session: ContributionsSessionKey, kinds: readonly string[], environment: E) => string;
  readonly install: (session: ContributionsSessionKey, json: string, kinds: readonly string[], environment: E) => Promise<void>;
};

export type ContributionsPublisher<E> = {
  readonly publish: (session: ContributionsSessionKey, environment: E) => Promise<ContributionsPublishOutcome>;
  readonly retire: (instanceId: number) => void;
  readonly installedKey: () => string | null;
};

/**
 * 🧩️ The host→guest contributions push as its OWN owned, per-instance cancellable unit.
 *
 * 🏁️ It used to live inside `ShellHost.refreshUi`, behind that function's refresh-generation guard:
 * the push `await`ed a guest document read and then returned when the generation had moved. A guest
 * whose pre-contribution `flowEvalTick` faults `flow.extension-not-contributed` re-arms itself, every
 * settle triggers a refresh, and every refresh bumps the generation — so each push was superseded
 * inside its own document read and the closure never crossed. The starvation is structural, not a
 * race to widen a window on: a push is not a projection of one refresh's UI state, so a superseded
 * refresh must not abort one (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 *
 * The unit is keyed by `(pluginId, instanceId)`, joins concurrent callers on an unmoved registry
 * generation, resolves the operator scope ONCE per session, and installs `setContributions` keyed by
 * `(instanceId, content)` — claimed BEFORE the guest crossing, so two overlapping publishes can never
 * push the same content twice, and restored when the crossing throws so a failure can be retried.
 * `retire(instanceId)` is the cancellation: a session switch abandons whatever is still resolving for
 * that instance, and its result is discarded rather than pushed into a session nobody is looking at.
 */
export function createContributionsPublisher<E>(ports: ContributionsPublisherPorts<E>): ContributionsPublisher<E> {
  const inFlightByKey = new Map<string, { readonly generation: string; readonly run: Promise<ContributionsPublishOutcome> }>();
  const kindsByKey = new Map<string, readonly string[]>();
  // 📦️ The scoped pack is a quarter-megabyte cut of the whole loaded closure and depends on nothing
  // but `(receiver, registry generation, kinds)` — cutting it again on every refresh burned real CPU
  // for a byte-identical answer (71 rebuilds in one 60 s boot, measured).
  const packByKey = new Map<string, string>();
  const epochByInstance = new Map<number, number>();
  let installedKey: string | null = null;
  const keyOf = (session: ContributionsSessionKey): string => `${session.pluginId}::${session.instanceId}`;
  const epochOf = (instanceId: number): number => epochByInstance.get(instanceId) ?? 0;
  const run = async (session: ContributionsSessionKey, environment: E, epoch: number, generation: string): Promise<ContributionsPublishOutcome> => {
    const key = keyOf(session);
    try {
      let kinds = kindsByKey.get(key);
      if (kinds === undefined) {
        const scope = await ports.resolveScope(session, environment);
        if (epochOf(session.instanceId) !== epoch) return { status: "retired" };
        if (scope.status === "unresolved") return { status: "unresolved", reason: scope.reason };
        kinds = scope.kinds;
        kindsByKey.set(key, kinds);
      }
      const packKey = `${session.pluginId}::${generation}::${kinds.join(",")}`;
      let json = packByKey.get(packKey);
      if (json === undefined) {
        json = ports.buildPack(session, kinds, environment);
        packByKey.set(packKey, json);
      }
      if (json.length === 0 || json === "[]") return { status: "empty" };
      const pushKey = `${session.instanceId}::${json}`;
      if (pushKey === installedKey) return { status: "unchanged" };
      const displaced = installedKey;
      installedKey = pushKey;
      try {
        await ports.install(session, json, kinds, environment);
      } catch (error) {
        installedKey = displaced;
        throw error;
      }
      return { status: "installed", chars: json.length, kinds };
    } catch (error) {
      return { status: "failed", reason: error instanceof Error ? error.message : String(error) };
    }
  };
  return {
    publish(session, environment) {
      const key = keyOf(session);
      const generation = ports.registryGeneration(environment);
      const live = inFlightByKey.get(key);
      if (live && live.generation === generation) return live.run;
      const entry = { generation, run: run(session, environment, epochOf(session.instanceId), generation) };
      inFlightByKey.set(key, entry);
      void entry.run.then(() => {
        if (inFlightByKey.get(key) === entry) inFlightByKey.delete(key);
      });
      return entry.run;
    },
    retire(instanceId) {
      epochByInstance.set(instanceId, epochOf(instanceId) + 1);
      for (const [key, entry] of [...inFlightByKey]) {
        if (key.endsWith(`::${instanceId}`) && inFlightByKey.get(key) === entry) inFlightByKey.delete(key);
      }
      for (const key of [...kindsByKey.keys()]) {
        if (key.endsWith(`::${instanceId}`)) kindsByKey.delete(key);
      }
      packByKey.clear();
    },
    installedKey: () => installedKey,
  };
}
