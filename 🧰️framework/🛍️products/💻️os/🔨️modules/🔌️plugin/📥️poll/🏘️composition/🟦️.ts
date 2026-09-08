import type { ResidentCapacity } from "../../../../../../🔨️modules/🌱️value/💾️resident/🟦️.ts";

type Counts<T extends number | bigint> = { readonly bytes: T; readonly slots: T; readonly owners: T };
type Capacity<T extends number | bigint> = Counts<T> & { readonly control: Counts<T> };
export type PluginPollCompositionWit = Capacity<bigint>;

function field(value: unknown, key: string): unknown {
  if (value === null || typeof value !== "object") throw new Error("plugin-composition.field");
  const descriptor = Object.getOwnPropertyDescriptor(value, key);
  if (!descriptor || !("value" in descriptor)) throw new Error("plugin-composition.field");
  return descriptor.value;
}
function wireCount(value: unknown): bigint {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0) throw new Error("plugin-composition.number");
  return BigInt(value);
}
function hostCount(value: unknown): number {
  if (typeof value !== "bigint" || value < 0n || value > 9007199254740991n) throw new Error("plugin-composition.u64");
  return Number(value);
}
function mapCapacity<T extends number | bigint>(value: unknown, scalar: (value: unknown) => T): Capacity<T> {
  const bytes = scalar(field(value, "bytes")); const slots = scalar(field(value, "slots")); const owners = scalar(field(value, "owners")); const source = field(value, "control");
  const control = { bytes: scalar(field(source, "bytes")), slots: scalar(field(source, "slots")), owners: scalar(field(source, "owners")) };
  if (control.bytes > bytes || control.slots > slots || control.owners > owners) throw new Error("plugin-composition.partition");
  return Object.freeze({ bytes, slots, owners, control: Object.freeze(control) });
}

/** 🏘️ Maps the existing required poll field; original owner, raw roots and allocation admission remain with the caller. */
export function pluginPollCompositionToWit(value: unknown): PluginPollCompositionWit { return mapCapacity(value, wireCount); }

/** 🏘️ Checks WIT u64 values before host conversion; equal configuration never certifies private composition identity. */
export function pluginPollCompositionFromWit(value: unknown): ResidentCapacity { return mapCapacity(value, hostCount); }

//#region 🧪️CompositionMapping
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️pluginpollcompositionwit-preserves-the-canonical-six-scalars-and-exact-n/🟦️.ts");
  await registerTests1(import.meta.vitest, { pluginPollCompositionFromWit, pluginPollCompositionToWit }, { url: import.meta.url });
}
//#endregion 🧪️CompositionMapping
