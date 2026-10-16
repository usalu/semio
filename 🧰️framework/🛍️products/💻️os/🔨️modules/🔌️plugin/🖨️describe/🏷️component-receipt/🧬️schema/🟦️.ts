import schema from "./🔣️.json";
import type { ComponentProfileV1, ComponentReceiptV1 } from "../../../../../../../🔨️modules/🏷️component-receipt/🟦️.ts";

/** 🏷️ The schema tag every component receipt carries (`ComponentReceiptV1.schema`). */
export const COMPONENT_RECEIPT_SCHEMA_ID = schema.$defs.ComponentReceiptV1.properties.schema.const;

/** 🗂️ The receipt's file name beside the component bytes it names: in a profile's deliverable directory and in a plugin module. */
export const COMPONENT_RECEIPT_FILE = "🧾️receipt.json";

/** 🎯️ The one component target triple (`ComponentReceiptV1.target`). */
export const COMPONENT_TARGET = schema.$defs.ComponentReceiptV1.properties.target.const;

/** 🪶️ The ONE shipped component profile: describe, provenance, receipts, materialization and the trusted catalog act on its bytes. */
export const SHIPPED_COMPONENT_PROFILE: ComponentProfileV1 = "release";

/** 🔁️ The local-only profile of `components iterate <plugin-id>`; its bytes are never described for commit or shipped. */
export const ITERATE_COMPONENT_PROFILE: ComponentProfileV1 = "prototype";

/** 📐️ One profile's fixed deliverable directory (crate-relative), byte cap and shipping admission. */
export type ComponentProfilePolicyV1 = Readonly<{ directory: string; maximumBytes: number; shipped: boolean }>;

/** 📏️ The schema-owned profile policy table (`ComponentProfilePolicies`). */
export const COMPONENT_PROFILE_POLICIES = Object.freeze(Object.fromEntries(Object.entries(schema.$defs.ComponentProfilePolicies.const).map(([profile, policy]) => [profile, Object.freeze({ directory: policy.directory, maximumBytes: policy.maximumBytes, shipped: policy.shipped })]))) as Readonly<Record<ComponentProfileV1, ComponentProfilePolicyV1>>;

/** 🧱️ The 256 MiB bound every shipped component meets before it reaches a receipt. */
export const SHIPPED_COMPONENT_MAX_BYTES = COMPONENT_PROFILE_POLICIES["release"].maximumBytes;

/** 🧯️ The local-only 512 MiB cap of an unoptimized, unstripped `prototype` link. */
export const ITERATE_COMPONENT_MAX_BYTES = COMPONENT_PROFILE_POLICIES["prototype"].maximumBytes;

/** 📏️ The most bytes one canonical receipt file may have. */
export const COMPONENT_RECEIPT_MAX_BYTES = 4096;

/** 🔗️ The identity and bytes a receipt is required to name. */
export type ComponentReceiptBindingV1 = Readonly<{ profile: ComponentProfileV1; pluginId: string; packageId: string; wasmSha256: string; semanticDescriptorSha256: string }>;

const RECEIPT_KEYS = Object.freeze([...schema.$defs.ComponentReceiptV1.required]) as readonly (keyof ComponentReceiptV1)[];
const SHA256 = new RegExp(schema.$defs.ComponentSha256.pattern, "u");
const PLUGIN_ID = new RegExp(schema.$defs.ComponentPluginId.pattern, "u");
const PACKAGE_ID = new RegExp(schema.$defs.ComponentPackageId.pattern, "u");
const TOOLCHAIN = new RegExp(schema.$defs.ComponentReceiptV1.properties.toolchain.pattern, "u");
const MINIMUM_WASM_BYTES = schema.$defs.ComponentReceiptV1.properties.wasmBytes.minimum;

/** 🦀️ Whether `identity` is a `rustc <release> (<commit-hash>)` toolchain identity (`ComponentReceiptV1.toolchain`). */
export function componentToolchainV1(identity: string): boolean {
  return TOOLCHAIN.test(identity);
}

/** 🚦️ Admits exactly one known component profile name. */
export function componentProfileV1(value: string | undefined): ComponentProfileV1 {
  if (value !== "release" && value !== "prototype") throw new Error(`component profile must be release or prototype, not ${JSON.stringify(value)}`);
  return value;
}

/** 🖋️ The one canonical encoding: schema key order, two-space indentation, one trailing newline. */
export function encodeComponentReceiptV1(receipt: ComponentReceiptV1): string {
  return `${JSON.stringify(Object.fromEntries(RECEIPT_KEYS.map((key) => [key, receipt[key]])), null, 2)}\n`;
}

/** 🔐️ Admits one decoded receipt value: exact schema members, each value admitted by its schema rule. */
export function componentReceiptValueV1(value: unknown): ComponentReceiptV1 {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("component receipt is not a JSON object");
  const record = value as Record<string, unknown>;
  const keys = Object.keys(record);
  if (keys.length !== RECEIPT_KEYS.length || keys.some((key) => !RECEIPT_KEYS.includes(key as keyof ComponentReceiptV1))) throw new Error(`component receipt members must be exactly ${RECEIPT_KEYS.join(", ")}`);
  if (record.schema !== COMPONENT_RECEIPT_SCHEMA_ID) throw new Error(`component receipt schema must be ${COMPONENT_RECEIPT_SCHEMA_ID}`);
  if (typeof record.pluginId !== "string" || !PLUGIN_ID.test(record.pluginId)) throw new Error("component receipt pluginId is not a plugin id");
  if (typeof record.packageId !== "string" || !PACKAGE_ID.test(record.packageId)) throw new Error("component receipt packageId is not a package id");
  const profile = componentProfileV1(typeof record.profile === "string" ? record.profile : undefined);
  if (record.target !== COMPONENT_TARGET) throw new Error(`component receipt target must be ${COMPONENT_TARGET}`);
  for (const key of ["wasmSha256", "semanticDescriptorSha256", "sourceTreeDigest", "provenanceSha256"] as const) if (typeof record[key] !== "string" || !SHA256.test(record[key] as string)) throw new Error(`component receipt ${key} is not a lowercase SHA-256`);
  if (typeof record.wasmBytes !== "number" || !Number.isSafeInteger(record.wasmBytes) || record.wasmBytes < MINIMUM_WASM_BYTES || record.wasmBytes > COMPONENT_PROFILE_POLICIES[profile].maximumBytes) throw new Error(`component receipt wasmBytes is outside the ${profile} bound`);
  if (typeof record.toolchain !== "string" || !componentToolchainV1(record.toolchain)) throw new Error("component receipt toolchain is not a rustc release and commit hash");
  return Object.freeze({ ...(record as ComponentReceiptV1) });
}

/** 🔐️ Strict-decodes a receipt file's text: a bounded, schema-valid value whose text is its own canonical encoding, so duplicate
 * members, reordering and foreign whitespace are refused without a second parser. */
export function parseComponentReceiptV1(text: string): ComponentReceiptV1 {
  if (text.length === 0 || text.length > COMPONENT_RECEIPT_MAX_BYTES) throw new Error("component receipt exceeds its bound");
  let value: unknown;
  try {
    value = JSON.parse(text);
  } catch (error) {
    throw new Error(`component receipt is not JSON: ${(error as Error).message}`);
  }
  const receipt = componentReceiptValueV1(value);
  if (encodeComponentReceiptV1(receipt) !== text) throw new Error("component receipt is not in its canonical encoding");
  return receipt;
}

/** 🔗️ Binds exactly the selected profile, package, component bytes and semantic descriptor. */
export function componentReceiptBindsV1(receipt: ComponentReceiptV1, expected: ComponentReceiptBindingV1): boolean {
  return receipt.profile === expected.profile && receipt.pluginId === expected.pluginId && receipt.packageId === expected.packageId && receipt.wasmSha256 === expected.wasmSha256 && receipt.semanticDescriptorSha256 === expected.semanticDescriptorSha256;
}
