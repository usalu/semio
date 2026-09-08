/// <reference types="vitest/importMeta" />
/** @emoji 🎯️ Strict private Shell-to-browser-actor action handoff. */

export const BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES = 256 * 1024;
export const BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM = 4_096;
export const BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION = 14;

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

export type BrowserActorActionRequestV1 = BrowserActorActionOwnerV1 & {
  readonly kind: "browser-actor-action";
  readonly command: readonly number[];
  readonly viewState: readonly number[];
};

export type BrowserActorActionResultV1 = BrowserActorActionOwnerV1 & {
  readonly kind: "browser-actor-action-result";
  readonly outcome: "acknowledged" | "rejected";
  readonly mutationCount: number;
  readonly reason?: string;
};

export type BrowserActorActionInvocationV1 = {
  readonly address: {
    readonly pluginId: string;
    readonly appId: string;
    readonly modeId: string;
    readonly windowKindId: string;
    readonly windowInstanceId: string;
    readonly actionId: string;
  };
  readonly arguments: Readonly<Record<string, unknown>>;
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
  const record = object(value, "browserActorAction", ["actionSequence", "activationGeneration", "appChannelVersion", "command", "instanceId", "kind", "scope", "surfaceRevision", "verifiedSurfaceId", "viewState"]);
  if (record.kind !== "browser-actor-action") throw new Error("browserActorAction.kind: invalid");
  return { kind: "browser-actor-action", ...owner(record), command: bytes(record.command, "browserActorAction.command"), viewState: bytes(record.viewState, "browserActorAction.viewState") };
}

/** 📤️ Decodes the worker's exact action disposition; mutation bodies remain on the ordinary Commands lane. */
export function parseBrowserActorActionResultV1(value: unknown): BrowserActorActionResultV1 {
  const source = object(value, "browserActorActionResult", ["actionSequence", "activationGeneration", "appChannelVersion", "instanceId", "kind", "mutationCount", "outcome", "scope", "surfaceRevision", "verifiedSurfaceId"], ["reason"]);
  if (source.kind !== "browser-actor-action-result" || (source.outcome !== "acknowledged" && source.outcome !== "rejected")) throw new Error("browserActorActionResult.outcome: invalid");
  if ((source.outcome === "rejected") !== (source.reason !== undefined)) throw new Error("browserActorActionResult.reason: invalid pairing");
  return {
    kind: "browser-actor-action-result",
    ...owner(source),
    outcome: source.outcome,
    mutationCount: natural(source.mutationCount, "browserActorActionResult.mutationCount", 0, BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM),
    ...(source.reason === undefined ? {} : { reason: text(source.reason, "browserActorActionResult.reason") }),
  };
}

/** 🧭 Decodes the Pack-projected action address before any guest turn is admitted. */
export function parseBrowserActorActionInvocationV1(value: unknown): BrowserActorActionInvocationV1 {
  const record = object(value, "browserActorAction.invocation", ["address", "arguments"]);
  const address = object(record.address, "browserActorAction.invocation.address", ["actionId", "appId", "modeId", "pluginId", "windowInstanceId", "windowKindId"]);
  const argumentsValue = object(record.arguments, "browserActorAction.invocation.arguments", Object.keys(record.arguments as Readonly<Record<string, unknown>>));
  return {
    address: {
      pluginId: text(address.pluginId, "browserActorAction.invocation.address.pluginId"),
      appId: text(address.appId, "browserActorAction.invocation.address.appId"),
      modeId: text(address.modeId, "browserActorAction.invocation.address.modeId"),
      windowKindId: text(address.windowKindId, "browserActorAction.invocation.address.windowKindId"),
      windowInstanceId: text(address.windowInstanceId, "browserActorAction.invocation.address.windowInstanceId"),
      actionId: text(address.actionId, "browserActorAction.invocation.address.actionId"),
    },
    arguments: argumentsValue,
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
  const { expect, it } = import.meta.vitest;
  it("browser actor action handoff validates the neutral schema and exact owner with an independent oracle", async () => {
    const { readFileSync } = await import("node:fs");
    const Ajv = (await import("ajv")).default;
    const equal = (await import("fast-deep-equal")).default;
    const fixture = JSON.parse(readFileSync(new URL("./🧫️fixture/🔣️.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8"));
    expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
    const request = parseBrowserActorActionRequestV1(fixture.request);
    const acknowledged = parseBrowserActorActionResultV1(fixture.acknowledged);
    expect(browserActorActionOwnerMatchesV1(request, acknowledged)).toBe(true);
    expect(equal(parseBrowserActorActionInvocationV1(fixture.invocation), fixture.invocation)).toBe(true);
    expect(parseBrowserActorActionResultV1(fixture.rejected).reason).toBe("action-refused");
    for (const hostile of fixture.hostileResults) expect(browserActorActionOwnerMatchesV1(request, parseBrowserActorActionResultV1(hostile))).toBe(false);
    expect(() => parseBrowserActorActionRequestV1({ ...fixture.request, command: new Array(BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES + 1).fill(0) })).toThrow(/bounded bytes/u);
    expect(() => parseBrowserActorActionInvocationV1({ ...fixture.invocation, address: { ...fixture.invocation.address, extra: true } })).toThrow(/exact fields/u);
  });
}
