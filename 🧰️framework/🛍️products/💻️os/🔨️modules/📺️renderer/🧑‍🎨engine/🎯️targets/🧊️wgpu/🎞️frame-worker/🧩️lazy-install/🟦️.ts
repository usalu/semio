/** 🧩️ The lazy half of the frame Worker's plugin mounting.
 *
 * 🐛️ `mountPluginHandles` eager-mounts exactly the boot plan's plugins and nothing afterwards, and the
 * wasm renderer cannot fetch a module itself — so the browser wgpu shell's `install_plugin` had no
 * module source at all and every cross-plugin open in the browser was refused before it began, while
 * the native shell already re-read its runtime manifest on demand. This is that missing source: ONE
 * plugin, on request, through the SAME `loadPluginModule` + `pluginHandleForBridge` pair the eager
 * mount uses, so the handle the shell admits is one shape, not two.
 *
 * ⏱️ Progress is reported per phase rather than as a fraction: a module load is one fetch+instantiate
 * whose only honest milestones are `resolving` (the catalog row), `loading` (the module itself) and
 * its settlement. The shell's own `ShellPluginInstall` phase machine consumes exactly these.
 *
 * 🛑️ Cancellation settles the CALLER, not the load. Nothing can abort a module fetch already in
 * flight, and killing it would throw away work the next request would repeat; the withdrawn request
 * rejects immediately and the load runs to completion into the loader's own cache, so a re-request
 * finds it warm. A second request for a plugin already in flight joins the first rather than starting
 * a second fetch of the same bytes.
 */

/** 🧩️ What one lazy install reports as it runs — see this module's header for why these and not a
 * fraction. `mounted`/`cancelled`/`failed` are terminal. */
export type LazyPluginInstallPhase = "resolving" | "loading" | "mounted" | "cancelled" | "failed";

/** 🚪️ The door the frame Worker installs on its global as `semioWgpuInstallPlugin` /
 * `semioWgpuCancelPluginInstall`, and the shape this module's own tests drive directly. */
export type LazyPluginInstallDoor = {
  /** 📦️ Mounts `pluginId` and answers its bridge handle. Rejects with `plugin-install.*` for every
   * refusal, so a caller never has to tell a missing catalog row from a failed fetch by message shape. */
  install(pluginId: string): Promise<unknown>;
  /** 🛑️ Withdraws an in-flight request. Answers whether one was actually withdrawn. */
  cancel(pluginId: string): boolean;
  /** 🔍️ The plugin ids with a request outstanding right now, in request order. */
  pending(): readonly string[];
};

export type LazyPluginInstallOptions = {
  /** 🗂️ The module url for a catalog row, or `undefined` for a plugin the catalog does not carry. */
  readonly moduleUrl: (pluginId: string) => string | undefined;
  /** 📦️ Loads the module and returns the handle the shell admits — the Worker passes
   * `pluginHandleForBridge(await loadPluginModule(...))`, wrapped in its own suspension monitor. */
  readonly mount: (pluginId: string, moduleUrl: string) => Promise<unknown>;
  /** 📣️ Phase sink. Faults inside it are swallowed: reporting progress must never fail an install. */
  readonly progress?: (pluginId: string, phase: LazyPluginInstallPhase) => void;
};

/** 🛑️ One withdrawable arm of the race below. The definite-assignment assertion is the honest
 * spelling: a `Promise` executor runs synchronously, so `reject` IS bound before this returns. */
function createWithdrawal(pluginId: string): { readonly promise: Promise<never>; readonly withdraw: () => void } {
  let reject!: (error: Error) => void;
  const promise = new Promise<never>((_, rejectPromise) => {
    reject = rejectPromise;
  });
  return { promise, withdraw: () => reject(new Error(`plugin-install.cancelled: ${pluginId}`)) };
}

/** 🧩️ Builds one door over injected `moduleUrl`/`mount` seams — the Worker wires the real catalog and
 * loader, tests wire fakes, and neither needs a `Worker` global to exist. */
export function createLazyPluginInstallDoor(options: LazyPluginInstallOptions): LazyPluginInstallDoor {
  const inFlight = new Map<string, { readonly settled: Promise<unknown>; withdraw: (() => void) | null }>();
  const report = (pluginId: string, phase: LazyPluginInstallPhase): void => {
    try {
      options.progress?.(pluginId, phase);
    } catch {
    }
  };
  const install = (pluginId: string): Promise<unknown> => {
    const existing = inFlight.get(pluginId);
    if (existing) return existing.settled;
    report(pluginId, "resolving");
    const moduleUrl = options.moduleUrl(pluginId);
    if (moduleUrl === undefined || moduleUrl === "") {
      report(pluginId, "failed");
      return Promise.reject(new Error(`plugin-install.unknown-plugin: ${pluginId}`));
    }
    const withdrawal = createWithdrawal(pluginId);
    report(pluginId, "loading");
    const mounted = options.mount(pluginId, moduleUrl);
    const settled = Promise.race([mounted, withdrawal.promise]).then(
      (handle) => {
        if (inFlight.get(pluginId)?.settled === settled) inFlight.delete(pluginId);
        report(pluginId, "mounted");
        return handle;
      },
      (error: unknown) => {
        const cancelled = error instanceof Error && error.message.startsWith("plugin-install.cancelled");
        if (inFlight.get(pluginId)?.settled === settled) inFlight.delete(pluginId);
        report(pluginId, cancelled ? "cancelled" : "failed");
        throw error instanceof Error ? error : new Error(`plugin-install.failed: ${pluginId}: ${String(error)}`);
      },
    );
    inFlight.set(pluginId, { settled, withdraw: withdrawal.withdraw });
    return settled;
  };
  return {
    install,
    cancel: (pluginId) => {
      const entry = inFlight.get(pluginId);
      if (!entry?.withdraw) return false;
      const withdraw = entry.withdraw;
      entry.withdraw = null;
      withdraw();
      return true;
    },
    pending: () => [...inFlight.keys()],
  };
}
