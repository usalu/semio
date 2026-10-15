/** 🪦️ The retired-instance ledger and the typed predicates that tell a teardown race from a real failure.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | PluginRuntime}, which re-exports every public name below so import specifiers stay unchanged.
 */

//#region 🔖️plugin-runtime
//#region 🪦️InstanceRetirement
/** 🪦️ An instance the host has RETIRED — its dispatch queue is closed and its actor is gone or going.
 *
 * Every late arrival for such an instance is a teardown race by construction, not a failure: a canvas
 * host that remounts and republishes its selection reset, a gesture already in flight when a role
 * switch or a hot-swap tore the instance down, an operation-completion subscription an effect pass
 * re-establishes one render too late. The shell answers those by DROPPING them typed — one line naming
 * the instance — instead of surfacing a stack, which is what turned a whole journey red on a single
 * `interactionSelect` (`📓️react-current-tree-battery-2026-09-14.md` §6).
 *
 * The message text of the two throwing gates below is deliberately left verbatim — several probes count
 * `no actor for instance` / `no channel for instance` as their own evidence — so the RETIREMENT is
 * carried as a property on the error rather than by rewording it. */
export const PLUGIN_INSTANCE_RETIRED = "pluginInstanceRetired" as const;

export function markPluginInstanceRetiredV1<TError extends Error>(error: TError): TError {
  Object.defineProperty(error, PLUGIN_INSTANCE_RETIRED, { value: true, enumerable: false });
  return error;
}

/** 🪦️ Every instance any handle of a plugin has destroyed, keyed `pluginId#instanceId`.
 *
 * 🐛️ The ledger used to be a `Set<number>` owned by ONE `adaptPluginHandle` closure, which answers
 * correctly for as long as that handle lives and loses the answer the moment it does not: a hot swap
 * replaces the handle, the replacement's ledger is empty, and every lane still addressed to the
 * destroyed instance — `subscribeOperationCompletions`, `subscribeOperationProgress`, the remounted-
 * window refresh — got an UNMARKED `no channel for instance N`, which `dropForRetiredInstance` cannot
 * recognise and which therefore surfaced as a console error and an unhandled rejection (measured on
 * :6023, `🗑️generated/react-sweep2/panel-i18n/console.txt` at +77.7 s, right after the swap's own
 * `destroyApp`). Instance ids are minted per plugin and never reused, so the plugin-qualified id is
 * the honest key, and "nothing ever created instance 99" stays exactly what it was: unmarked. */
const retiredPluginInstances = new Set<string>();
export function markPluginInstanceRetiredForPluginV1(pluginId: string, instanceId: number): void {
  retiredPluginInstances.add(`${pluginId}#${instanceId}`);
}
export function pluginInstanceWasRetiredV1(pluginId: string, instanceId: number): boolean {
  return retiredPluginInstances.has(`${pluginId}#${instanceId}`);
}
/** 🪦️ A live instance under that id supersedes any retirement recorded for it — the ledger states what
 * is GONE, and a `createApp` that answers with the id is the one fact that unsays it. */
export function forgetPluginInstanceRetirementV1(pluginId: string, instanceId: number): void {
  retiredPluginInstances.delete(`${pluginId}#${instanceId}`);
}

/** 🪦️ Whether this failure means "the instance it addressed is retired", covering both throwing gates, the two
 * channel terminals `AppChannelClient` itself raises once its queue is closed, and the shard client's
 * `actor-activation.revoked` — the activation a turn captured was closed under it (a session switch sealing the
 * instance, a hot swap, a suspend). Such a turn is cancelled WITH its activation: a tree-window refresh that crossed a
 * `/spaces/<id>` load used to surface as `tree window refresh failed … actor-activation.revoked` (ticket 26/09/23 U5,
 * C10 relay). */
export function isPluginInstanceRetiredV1(error: unknown): boolean {
  if (typeof error === "object" && error !== null && (error as Record<string, unknown>)[PLUGIN_INSTANCE_RETIRED] === true) return true;
  const message = error instanceof Error ? error.message : String(error ?? "");
  return message.includes("app-channel.disposed") || message.includes("app-channel.closed") || message.includes("plugin-handle.closed") || message.includes("actor-activation.revoked");
}
//#endregion 🪦️InstanceRetirement
//#endregion 🔖️plugin-runtime
