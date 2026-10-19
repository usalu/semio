/** 🧰️ The module-wide instance id counter every `loadPluginModule` call draws from.
 *
 * 🧱️ Child module of {@link ../../🟦️.ts | loadPluginModule}: each function builds one group of the handle's closure state and helpers and returns them for the later groups. */
/** 🪪️ Instance ids are unique across EVERY plugin `loadPluginModule` loads, not just within one call —
 * `pendingTurnEffects` is keyed by `instanceId` alone and shared module-wide, mirroring the kernel's own
 * single global `next_instance_id`. */
export const globalInstanceIds = { next: 1 };
