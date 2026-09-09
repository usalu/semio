/// <reference types="vitest/importMeta" />
/** @emoji 🎯️ Strict private Shell-to-browser-actor action handoff. */

export const BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES = 256 * 1024;
export const BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM = 4_096;
export const BROWSER_ACTOR_ACTION_HOST_EFFECT_MAXIMUM = 1;
export const BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION = 15;

export type BrowserActorActionScopeV1 = { readonly spaceId: string; readonly documentId: string };

export type BrowserActorActionOwnerV1 = {
  readonly scope: BrowserActorActionScopeV1;
  readonly verifiedSurfaceId: string;
  readonly appChannelVersion: typeof BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION;
  readonly activationGeneration: string;
  readonly instanceId: number;
  readonly surfaceRevision: number;
  readonly actionSequence: number;
};

export type BrowserActorActionPayloadV1 =
  | { readonly kind: "ui-intent"; readonly bytes: readonly number[] }
  | { readonly kind: "app-command"; readonly bytes: readonly number[] };

export type BrowserActorActionRequestV1 = BrowserActorActionOwnerV1 & {
  readonly kind: "browser-actor-action";
  readonly payload: BrowserActorActionPayloadV1;
};

export type BrowserActorActionResultV1 = BrowserActorActionOwnerV1 & {
  readonly kind: "browser-actor-action-result";
  readonly outcome: "guest-applied" | "rejected";
  readonly mutationCount: number;
  readonly hostEffects: readonly (readonly number[])[];
  readonly reason?: string;
};

function object(value: unknown, path: string, required: readonly string[], optional: readonly string[] = []): Readonly<Record<string, unknown>> {
  if (value === null || typeof value !== "object" || Array.isArray(value) || Object.getPrototypeOf(value) !== Object.prototype) throw new Error(`${path}: expected an object`);
  const record = value as Readonly<Record<string, unknown>>;
  const allowed = new Set([...required, ...optional]);
  if (Object.keys(record).some((key) => !allowed.has(key)) || required.some((key) => !(key in record))) throw new Error(`${path}: invalid exact fields`);
  return record;
}

function text(value: unknown, path: string): string {
  if (typeof value !== "string" || value.length === 0 || new TextEncoder().encode(value).length > 256 || /[\u0000-\u001f\u007f]/u.test(value)) throw new Error(`${path}: invalid text`);
  return value;
}

function natural(value: unknown, path: string, minimum: number, maximum = Number.MAX_SAFE_INTEGER): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < minimum || value > maximum) throw new Error(`${path}: invalid natural`);
  return value;
}

function generation(value: unknown): string {
  if (typeof value !== "string" || !/^[1-9][0-9]{0,19}$/u.test(value) || BigInt(value) > 0xffffffffffffffffn) throw new Error("browserActorAction.activationGeneration: invalid");
  return value;
}

function appChannelVersion(value: unknown): typeof BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION {
  if (value !== BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION) throw new Error("browserActorAction.appChannelVersion: unsupported");
  return BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION;
}

function scope(value: unknown): BrowserActorActionScopeV1 {
  const record = object(value, "browserActorAction.scope", ["documentId", "spaceId"]);
  return { spaceId: text(record.spaceId, "browserActorAction.scope.spaceId"), documentId: text(record.documentId, "browserActorAction.scope.documentId") };
}

function bytes(value: unknown, path: string): readonly number[] {
  if (!Array.isArray(value) || value.length === 0 || value.length > BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES || value.some((byte) => typeof byte !== "number" || !Number.isInteger(byte) || byte < 0 || byte > 255)) throw new Error(`${path}: invalid bounded bytes`);
  return [...value];
}

function payload(value: unknown): BrowserActorActionPayloadV1 {
  const record = object(value, "browserActorAction.payload", ["bytes", "kind"]);
  if (record.kind !== "ui-intent" && record.kind !== "app-command") throw new Error("browserActorAction.payload.kind: invalid");
  return { kind: record.kind, bytes: bytes(record.bytes, "browserActorAction.payload.bytes") };
}

/** 📦️ Copies a bounded publication batch before it crosses a lifetime boundary. */
export function parseBrowserActorHostEffectBytesV1(value: unknown): readonly (readonly number[])[] {
  if (!Array.isArray(value) || value.length > BROWSER_ACTOR_ACTION_HOST_EFFECT_MAXIMUM) throw new Error("browserActorActionResult.hostEffects: invalid bounded batch");
  let size = 0;
  return value.map((item) => {
    const copy = bytes(item, "browserActorActionResult.hostEffects");
    size += copy.length;
    if (size > BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES) throw new Error("browserActorActionResult.hostEffects: invalid bounded bytes");
    return copy;
  });
}

function owner(record: Readonly<Record<string, unknown>>): BrowserActorActionOwnerV1 {
  return {
    scope: scope(record.scope),
    verifiedSurfaceId: text(record.verifiedSurfaceId, "browserActorAction.verifiedSurfaceId"),
    appChannelVersion: appChannelVersion(record.appChannelVersion),
    activationGeneration: generation(record.activationGeneration),
    instanceId: natural(record.instanceId, "browserActorAction.instanceId", 0, 0xffffffff),
    surfaceRevision: natural(record.surfaceRevision, "browserActorAction.surfaceRevision", 1),
    actionSequence: natural(record.actionSequence, "browserActorAction.actionSequence", 1),
  };
}

/** 📥️ Decodes one exact, bounded action request without accepting actor or document authority bytes. */
export function parseBrowserActorActionRequestV1(value: unknown): BrowserActorActionRequestV1 {
  const record = object(value, "browserActorAction", ["actionSequence", "activationGeneration", "appChannelVersion", "instanceId", "kind", "payload", "scope", "surfaceRevision", "verifiedSurfaceId"]);
  if (record.kind !== "browser-actor-action") throw new Error("browserActorAction.kind: invalid");
  return { kind: "browser-actor-action", ...owner(record), payload: payload(record.payload) };
}

/** 📤️ Decodes the worker's exact action disposition; mutation bodies remain on the ordinary Commands lane. */
export function parseBrowserActorActionResultV1(value: unknown): BrowserActorActionResultV1 {
  const source = object(value, "browserActorActionResult", ["actionSequence", "activationGeneration", "appChannelVersion", "hostEffects", "instanceId", "kind", "mutationCount", "outcome", "scope", "surfaceRevision", "verifiedSurfaceId"], ["reason"]);
  if (source.kind !== "browser-actor-action-result" || (source.outcome !== "guest-applied" && source.outcome !== "rejected")) throw new Error("browserActorActionResult.outcome: invalid");
  if ((source.outcome === "rejected") !== (source.reason !== undefined)) throw new Error("browserActorActionResult.reason: invalid pairing");
  const hostEffects = parseBrowserActorHostEffectBytesV1(source.hostEffects);
  if (source.outcome === "rejected" && hostEffects.length !== 0) throw new Error("browserActorActionResult.hostEffects: rejected publication");
  return {
    kind: "browser-actor-action-result",
    ...owner(source),
    outcome: source.outcome,
    mutationCount: natural(source.mutationCount, "browserActorActionResult.mutationCount", 0, BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM),
    hostEffects,
    ...(source.reason === undefined ? {} : { reason: text(source.reason, "browserActorActionResult.reason") }),
  };
}

/** 🪪️ Exact private owner equality; a matching action sequence alone never settles another lifetime. */
export function browserActorActionOwnerMatchesV1(left: BrowserActorActionOwnerV1, right: BrowserActorActionOwnerV1): boolean {
  return (
    left.scope.spaceId === right.scope.spaceId &&
    left.scope.documentId === right.scope.documentId &&
    left.verifiedSurfaceId === right.verifiedSurfaceId &&
    left.appChannelVersion === right.appChannelVersion &&
    left.activationGeneration === right.activationGeneration &&
    left.instanceId === right.instanceId &&
    left.surfaceRevision === right.surfaceRevision &&
    left.actionSequence === right.actionSequence
  );
}

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️browser-actor-action-handoff-validates-the-neutral-schema-and-exact-owne/🟦️.ts");
  const { decodeAppCommand, decodePackValue } = await import("../../../../🟦️.ts");
  const { createBrowserActorAppCommandRequestV1 } = await import("./🎛️command/🟦️.ts");
  const { createBrowserActorUiIntentRequestV1 } = await import("./🧭️intent/🟦️.ts");
  await registerTests1(import.meta.vitest, { BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES, browserActorActionOwnerMatchesV1, createBrowserActorAppCommandRequestV1, createBrowserActorUiIntentRequestV1, decodeAppCommand, decodePackValue, parseBrowserActorActionRequestV1, parseBrowserActorActionResultV1 }, { directory: (await import("node:url")).fileURLToPath(new URL(".", import.meta.url)), url: import.meta.url });
}
