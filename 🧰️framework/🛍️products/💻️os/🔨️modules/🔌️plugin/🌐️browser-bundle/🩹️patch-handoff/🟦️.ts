/// <reference types="vitest/importMeta" />
/** @emoji 🩹️ Strict private browser-actor UI patch handoff. */
import type { UiNodeRecord, UiPatch, UiPatchOp } from "@semio-tech/framework";
import { actorInstanceLifetimeEquals, type ActorInstanceLifetime } from "../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts";
import { actorUiPatchReceiptEquals, decodeActorUiPatchReceipt, encodeActorUiPatchReceipt, validateActorUiPatchPairing, type ActorUiPatchReceipt } from "../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts";

export type BrowserActorUiPatchScopeV1 = { readonly spaceId: string; readonly documentId: string };

export type BrowserActorUiPatchOfferV1 = {
  readonly kind: "browser-actor-ui-patch";
  readonly scope: BrowserActorUiPatchScopeV1;
  readonly verifiedSurfaceId: string;
  readonly activationGeneration: string;
  readonly instanceId: number;
  readonly patch: UiPatch;
  readonly receipt: readonly number[];
};

export type BrowserActorUiPatchResultV1 = {
  readonly kind: "browser-actor-ui-patch-result";
  readonly scope: BrowserActorUiPatchScopeV1;
  readonly verifiedSurfaceId: string;
  readonly activationGeneration: string;
  readonly instanceId: number;
  readonly receipt: readonly number[];
  readonly outcome: "acknowledged" | "rejected";
  readonly revision: number;
  readonly reason?: string;
};

type WireVariant = { readonly tag?: unknown; readonly val?: unknown };
type WirePatch = { readonly surface?: unknown; readonly revision?: unknown; readonly baseRevision?: unknown; readonly ops?: unknown };

export type BrowserActorUiPatchDecodePort = {
  readonly decodePack: (bytes: Uint8Array, path: string) => unknown;
  readonly natural: (value: unknown, path: string) => number;
};

function object(value: unknown, path: string, fields?: readonly string[]): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${path}: expected object`);
  const record = value as Record<string, unknown>;
  if (fields !== undefined && Object.keys(record).sort().join(",") !== [...fields].sort().join(",")) throw new Error(`${path}: invalid fields`);
  return record;
}

function shape(value: unknown, path: string, required: readonly string[], optional: readonly string[] = []): Record<string, unknown> {
  const record = object(value, path);
  const allowed = new Set([...required, ...optional]);
  if (Object.keys(record).some((field) => !allowed.has(field)) || required.some((field) => !(field in record))) throw new Error(`${path}: invalid fields`);
  return record;
}

function text(value: unknown, path: string): string {
  if (typeof value !== "string" || value.length === 0 || new TextEncoder().encode(value).length > 256 || /[\u0000-\u001f\u007f]/u.test(value)) throw new Error(`${path}: invalid text`);
  return value;
}

function bytes(value: unknown, path: string): Uint8Array {
  if (value instanceof Uint8Array) return value;
  if (!Array.isArray(value) || value.some((byte) => !Number.isInteger(byte) || byte < 0 || byte > 255)) throw new Error(`${path}: invalid bytes`);
  return Uint8Array.from(value);
}

function naturalNumber(value: unknown, path: string, maximum = Number.MAX_SAFE_INTEGER): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0 || value > maximum) throw new Error(`${path}: invalid natural`);
  return value;
}

function normalizeNode(raw: unknown, port: BrowserActorUiPatchDecodePort): UiNodeRecord {
  const record = shape(raw, "uiPatch.upsert.node", ["accessibility", "activity", "component", "id", "key", "layout", "style"], ["bindings", "children", "disabled", "menu", "transition"]) as Partial<UiNodeRecord>;
  if (record.disabled !== undefined && typeof record.disabled !== "boolean") throw new Error("uiPatch.upsert.node.disabled: invalid boolean");
  if (record.bindings !== undefined && !Array.isArray(record.bindings)) throw new Error("uiPatch.upsert.node.bindings: invalid list");
  if (record.children !== undefined && !Array.isArray(record.children)) throw new Error("uiPatch.upsert.node.children: invalid list");
  return {
    ...(record as UiNodeRecord),
    id: port.natural(record.id, "uiPatch.upsert.node.id"),
    disabled: record.disabled ?? false,
    transition: record.transition ?? null,
    bindings: Array.isArray(record.bindings) ? record.bindings : [],
    menu: record.menu ?? null,
    children: Array.isArray(record.children) ? record.children.map((child) => port.natural(child, "uiPatch.upsert.node.children[]")) : [],
  };
}

function decodeOps(raw: unknown, port: BrowserActorUiPatchDecodePort): UiPatchOp[] {
  if (!Array.isArray(raw) || raw.length > 4_096) throw new Error("uiPatch.ops: invalid list");
  return raw.map((candidate, index): UiPatchOp => {
    const variant = object(candidate, `uiPatch.ops[${index}]`, ["tag", "val"]) as WireVariant;
    const tag = text(variant.tag, `uiPatch.ops[${index}].tag`);
    if (tag === "remove" || tag === "set-root") return { type: tag === "remove" ? "remove" : "setRoot", id: port.natural(variant.val, `uiPatch.ops[${index}].val`) };
    const path = `uiPatch.ops[${index}].val`;
    const value = object(variant.val, path);
    const node = () => port.natural(value.node, `uiPatch.ops[${index}].node`);
    switch (tag) {
      case "upsert":
        object(value, path, ["node"]);
        return { type: "upsert", ...normalizeNode(port.decodePack(bytes(value.node, `uiPatch.ops[${index}].node`), `uiPatch.ops[${index}].node`), port) };
      case "set-component":
        object(value, path, ["component", "node"]);
        return { type: "setComponent", id: node(), component: port.decodePack(bytes(value.component, `uiPatch.ops[${index}].component`), `uiPatch.ops[${index}].component`) as Extract<UiPatchOp, { type: "setComponent" }>["component"] };
      case "set-layout":
        object(value, path, ["layout", "node"]);
        return { type: "setLayout", id: node(), layout: port.decodePack(bytes(value.layout, `uiPatch.ops[${index}].layout`), `uiPatch.ops[${index}].layout`) as Extract<UiPatchOp, { type: "setLayout" }>["layout"] };
      case "set-activity": {
        object(value, path, ["activity", "node"]);
        const activity = port.decodePack(bytes(value.activity, `uiPatch.ops[${index}].activity`), `uiPatch.ops[${index}].activity`) as Pick<Extract<UiPatchOp, { type: "setActivity" }>, "activity" | "disabled">;
        return { type: "setActivity", id: node(), activity: activity.activity, disabled: activity.disabled };
      }
      case "set-children":
        object(value, path, ["children", "node"]);
        if (!Array.isArray(value.children)) throw new Error(`uiPatch.ops[${index}].children: invalid list`);
        return { type: "setChildren", id: node(), children: value.children.map((child) => port.natural(child, `uiPatch.ops[${index}].children[]`)) };
      case "set-style":
        object(value, path, ["node", "style"]);
        return { type: "setStyle", id: node(), style: port.decodePack(bytes(value.style, `uiPatch.ops[${index}].style`), `uiPatch.ops[${index}].style`) as Extract<UiPatchOp, { type: "setStyle" }>["style"] };
      case "set-accessibility":
        object(value, path, ["accessibility", "node"]);
        return {
          type: "setAccessibility",
          id: node(),
          accessibility: port.decodePack(bytes(value.accessibility, `uiPatch.ops[${index}].accessibility`), `uiPatch.ops[${index}].accessibility`) as Extract<UiPatchOp, { type: "setAccessibility" }>["accessibility"],
        };
      case "set-bindings":
        object(value, path, ["bindings", "node"]);
        return { type: "setBindings", id: node(), bindings: port.decodePack(bytes(value.bindings, `uiPatch.ops[${index}].bindings`), `uiPatch.ops[${index}].bindings`) as Extract<UiPatchOp, { type: "setBindings" }>["bindings"] };
      case "set-menu":
        object(value, path, ["menu", "node"]);
        return { type: "setMenu", id: node(), menu: port.decodePack(bytes(value.menu, `uiPatch.ops[${index}].menu`), `uiPatch.ops[${index}].menu`) as Extract<UiPatchOp, { type: "setMenu" }>["menu"] };
      default:
        throw new Error(`uiPatch.ops[${index}]: unknown tag ${tag}`);
    }
  });
}

function parseCanonicalOps(value: unknown): UiPatchOp[] {
  if (!Array.isArray(value) || value.length > 4_096) throw new Error("browserActorUiPatch.patch.ops: invalid list");
  return value.map((candidate, index): UiPatchOp => {
    const path = `browserActorUiPatch.patch.ops[${index}]`;
    const record = object(candidate, path);
    switch (record.type) {
      case "upsert": {
        const node = object(record, path, ["accessibility", "activity", "bindings", "children", "component", "disabled", "id", "key", "layout", "menu", "style", "transition", "type"]);
        if (typeof node.disabled !== "boolean" || !Array.isArray(node.bindings) || !Array.isArray(node.children)) throw new Error(`${path}: invalid node`);
        return { ...(node as UiNodeRecord), type: "upsert", id: naturalNumber(node.id, `${path}.id`), children: node.children.map((child) => naturalNumber(child, `${path}.children[]`)) };
      }
      case "setComponent":
        object(record, path, ["component", "id", "type"]);
        return { type: "setComponent", id: naturalNumber(record.id, `${path}.id`), component: record.component as Extract<UiPatchOp, { type: "setComponent" }>["component"] };
      case "setLayout":
        object(record, path, ["id", "layout", "type"]);
        return { type: "setLayout", id: naturalNumber(record.id, `${path}.id`), layout: record.layout as Extract<UiPatchOp, { type: "setLayout" }>["layout"] };
      case "setActivity":
        object(record, path, ["activity", "disabled", "id", "type"]);
        if (typeof record.disabled !== "boolean") throw new Error(`${path}.disabled: invalid boolean`);
        return { type: "setActivity", id: naturalNumber(record.id, `${path}.id`), activity: record.activity as Extract<UiPatchOp, { type: "setActivity" }>["activity"], disabled: record.disabled };
      case "setChildren":
        object(record, path, ["children", "id", "type"]);
        if (!Array.isArray(record.children)) throw new Error(`${path}.children: invalid list`);
        return { type: "setChildren", id: naturalNumber(record.id, `${path}.id`), children: record.children.map((child) => naturalNumber(child, `${path}.children[]`)) };
      case "setStyle":
        object(record, path, ["id", "style", "type"]);
        return { type: "setStyle", id: naturalNumber(record.id, `${path}.id`), style: record.style as Extract<UiPatchOp, { type: "setStyle" }>["style"] };
      case "setAccessibility":
        object(record, path, ["accessibility", "id", "type"]);
        return { type: "setAccessibility", id: naturalNumber(record.id, `${path}.id`), accessibility: record.accessibility as Extract<UiPatchOp, { type: "setAccessibility" }>["accessibility"] };
      case "setBindings":
        object(record, path, ["bindings", "id", "type"]);
        if (!Array.isArray(record.bindings)) throw new Error(`${path}.bindings: invalid list`);
        return { type: "setBindings", id: naturalNumber(record.id, `${path}.id`), bindings: record.bindings as Extract<UiPatchOp, { type: "setBindings" }>["bindings"] };
      case "setMenu":
        object(record, path, ["id", "menu", "type"]);
        return { type: "setMenu", id: naturalNumber(record.id, `${path}.id`), menu: record.menu as Extract<UiPatchOp, { type: "setMenu" }>["menu"] };
      case "remove":
      case "setRoot":
        object(record, path, ["id", "type"]);
        return { type: record.type, id: naturalNumber(record.id, `${path}.id`) };
      default:
        throw new Error(`${path}.type: invalid`);
    }
  });
}

/** 🪪️ Preserves the exact guest-issued receipt while decoding at most one canonical UI patch. */
export function captureBrowserActorUiPatchV1(
  patches: unknown,
  receiptValue: unknown,
  expectedLifetime: ActorInstanceLifetime,
  expectedSurface: string,
  port: BrowserActorUiPatchDecodePort,
): { readonly instanceId: number; readonly patch: UiPatch; readonly receipt: ActorUiPatchReceipt } | null {
  if (!Array.isArray(patches)) throw new Error("uiPatch: invalid list");
  const receiptBytes = receiptValue === undefined || receiptValue === null ? null : bytes(receiptValue, "uiPatch.receipt");
  const receipt = receiptBytes === null ? null : decodeActorUiPatchReceipt(receiptBytes);
  validateActorUiPatchPairing(patches.length, receipt);
  if (patches.length === 0) return null;
  if (receipt === null || !actorInstanceLifetimeEquals(receipt.lifetime, expectedLifetime)) throw new Error("uiPatch: lifetime mismatch");
  const patch = object(patches[0], "uiPatch", ["baseRevision", "ops", "revision", "surface"]) as WirePatch;
  const surface = object(patch.surface, "uiPatch.surface", ["instance", "surface"]);
  const instanceId = port.natural(surface.instance, "uiPatch.surface.instance");
  const surfaceId = text(surface.surface, "uiPatch.surface.surface");
  if (instanceId !== expectedLifetime.instanceId || surfaceId !== expectedSurface) throw new Error("uiPatch: surface mismatch");
  return {
    instanceId,
    patch: { surface: surfaceId, baseRevision: port.natural(patch.baseRevision, "uiPatch.baseRevision"), revision: port.natural(patch.revision, "uiPatch.revision"), ops: decodeOps(patch.ops, port) },
    receipt,
  };
}

function scope(value: unknown): BrowserActorUiPatchScopeV1 {
  const record = object(value, "browserActorUiPatch.scope", ["documentId", "spaceId"]);
  return { spaceId: text(record.spaceId, "browserActorUiPatch.scope.spaceId"), documentId: text(record.documentId, "browserActorUiPatch.scope.documentId") };
}

function generation(value: unknown): string {
  if (typeof value !== "string" || !/^[1-9][0-9]{0,19}$/u.test(value) || BigInt(value) > 0xffffffffffffffffn) throw new Error("browserActorUiPatch.activationGeneration: invalid");
  return value;
}

function wireReceipt(value: unknown): number[] {
  const encoded = bytes(value, "browserActorUiPatch.receipt");
  encodeActorUiPatchReceipt(decodeActorUiPatchReceipt(encoded));
  return Array.from(encoded);
}

/** 📥️ Strictly decodes the worker-to-shell patch offer carried by the private Pack wire. */
export function parseBrowserActorUiPatchOfferV1(value: unknown): BrowserActorUiPatchOfferV1 {
  const record = object(value, "browserActorUiPatch", ["activationGeneration", "instanceId", "kind", "patch", "receipt", "scope", "verifiedSurfaceId"]);
  if (record.kind !== "browser-actor-ui-patch") throw new Error("browserActorUiPatch.kind: invalid");
  const patch = object(record.patch, "browserActorUiPatch.patch", ["baseRevision", "ops", "revision", "surface"]);
  const instanceId = naturalNumber(record.instanceId, "browserActorUiPatch.instanceId", 0xffffffff);
  return {
    kind: "browser-actor-ui-patch",
    scope: scope(record.scope),
    verifiedSurfaceId: text(record.verifiedSurfaceId, "browserActorUiPatch.verifiedSurfaceId"),
    activationGeneration: generation(record.activationGeneration),
    instanceId,
    patch: {
      surface: text(patch.surface, "browserActorUiPatch.patch.surface"),
      baseRevision: naturalNumber(patch.baseRevision, "browserActorUiPatch.patch.baseRevision"),
      revision: naturalNumber(patch.revision, "browserActorUiPatch.patch.revision"),
      ops: parseCanonicalOps(patch.ops),
    },
    receipt: wireReceipt(record.receipt),
  };
}

/** 📤️ Strictly decodes the shell's exact transaction verdict. */
export function parseBrowserActorUiPatchResultV1(value: unknown): BrowserActorUiPatchResultV1 {
  const source = object(value, "browserActorUiPatchResult");
  const outcome = source.outcome;
  const expected =
    outcome === "rejected"
      ? ["activationGeneration", "instanceId", "kind", "outcome", "reason", "receipt", "revision", "scope", "verifiedSurfaceId"]
      : ["activationGeneration", "instanceId", "kind", "outcome", "receipt", "revision", "scope", "verifiedSurfaceId"];
  object(value, "browserActorUiPatchResult", expected);
  if (source.kind !== "browser-actor-ui-patch-result" || (outcome !== "acknowledged" && outcome !== "rejected")) throw new Error("browserActorUiPatchResult: invalid outcome");
  const instanceId = naturalNumber(source.instanceId, "browserActorUiPatchResult.instanceId", 0xffffffff),
    revision = naturalNumber(source.revision, "browserActorUiPatchResult.revision");
  const reason = outcome === "rejected" ? text(source.reason, "browserActorUiPatchResult.reason") : undefined;
  return {
    kind: "browser-actor-ui-patch-result",
    scope: scope(source.scope),
    verifiedSurfaceId: text(source.verifiedSurfaceId, "browserActorUiPatchResult.verifiedSurfaceId"),
    activationGeneration: generation(source.activationGeneration),
    instanceId,
    receipt: wireReceipt(source.receipt),
    outcome,
    revision,
    ...(reason === undefined ? {} : { reason }),
  };
}

/** 🧬️ Exact private owner equality; revisions and surfaces never substitute for the issued receipt. */
export function browserActorUiPatchOwnerMatchesV1(offer: BrowserActorUiPatchOfferV1, result: BrowserActorUiPatchResultV1): boolean {
  return (
    offer.scope.spaceId === result.scope.spaceId &&
    offer.scope.documentId === result.scope.documentId &&
    offer.verifiedSurfaceId === result.verifiedSurfaceId &&
    offer.activationGeneration === result.activationGeneration &&
    offer.instanceId === result.instanceId &&
    actorUiPatchReceiptEquals(decodeActorUiPatchReceipt(Uint8Array.from(offer.receipt)), decodeActorUiPatchReceipt(Uint8Array.from(result.receipt)))
  );
}

if (import.meta.vitest) {
  const { expect, it } = import.meta.vitest;
  it("browser actor patch handoff validates the neutral schema and exact owner with an independent oracle", async () => {
    const { readFileSync } = await import("node:fs");
    const Ajv = (await import("ajv")).default;
    const equal = (await import("fast-deep-equal")).default;
    const fixture = JSON.parse(readFileSync(new URL("./🧫️fixture/🔣️.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8"));
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(fixture)).toBe(true);
    const offer = parseBrowserActorUiPatchOfferV1(fixture.offer);
    const result = parseBrowserActorUiPatchResultV1(fixture.acknowledged);
    expect(browserActorUiPatchOwnerMatchesV1(offer, result)).toBe(true);
    expect(equal(offer.receipt, result.receipt)).toBe(true);
    expect(browserActorUiPatchOwnerMatchesV1(offer, parseBrowserActorUiPatchResultV1(fixture.rejected))).toBe(true);
    for (const hostile of fixture.hostileResults) expect(browserActorUiPatchOwnerMatchesV1(offer, parseBrowserActorUiPatchResultV1(hostile))).toBe(false);
    expect(() => parseBrowserActorUiPatchOfferV1({ ...fixture.offer, patch: { ...fixture.offer.patch, baseRevision: "0" } })).toThrow(/baseRevision/u);
    expect(() => parseBrowserActorUiPatchOfferV1({ ...fixture.offer, patch: { ...fixture.offer.patch, ops: [{ type: "remove", id: 1, extra: true }] } })).toThrow(/invalid fields/u);
    expect(() => parseBrowserActorUiPatchOfferV1({ ...fixture.offer, patch: { ...fixture.offer.patch, ops: [{ type: "unknown", id: 1 }] } })).toThrow(/type: invalid/u);
  });
}
