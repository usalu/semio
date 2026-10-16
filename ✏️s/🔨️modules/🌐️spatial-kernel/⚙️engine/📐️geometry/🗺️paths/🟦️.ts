/** 🗺️ Attribute paths and tables: path segments with their reads, writes and clears, the attribute table and the spatial entity flags. */
import type { ModelEntityKind } from "../🟦️.ts";

// #region 🗺️Paths
/** 🧭️ Root object for segmented path reads (`context`, `event`, or action `params`). */
export type PathRoot = "context" | "event" | "params";

/** 🧭️ One navigation step: object field or array index (no dynamic JSON keys). */
export type PathSegment = { readonly kind: "field"; readonly name: string } | { readonly kind: "index"; readonly index: number };

/** 🧭️ Absolute path into `context` or `event` payloads. */
export interface PathTarget {
  readonly root: PathRoot;
  readonly segments: readonly PathSegment[];
}

/** 🧭️ Reads `segments` from `root` (object/array chain). */
export function readPathSegments(root: unknown, segments: readonly PathSegment[]): unknown {
  let cur: unknown = root;
  for (const seg of segments) {
    if (cur === null || cur === undefined) return undefined;
    if (seg.kind === "field") {
      if (typeof cur !== "object" || Array.isArray(cur)) return undefined;
      cur = (cur as Record<string, unknown>)[seg.name];
    } else {
      if (!Array.isArray(cur)) return undefined;
      cur = cur[seg.index];
    }
  }
  return cur;
}

/** 🧭️ Writes `value` at `segments` under `root` (creates object/array shells). */
export function writePathSegments(root: Record<string, unknown>, segments: readonly PathSegment[], value: unknown): void {
  if (segments.length === 0) return;
  let cur: Record<string, unknown> | unknown[] = root;
  for (let i = 0; i < segments.length - 1; i++) {
    const seg = segments[i]!;
    const next = segments[i + 1]!;
    const nextIsIndex = next.kind === "index";
    if (seg.kind === "field") {
      const o = cur as Record<string, unknown>;
      let child = o[seg.name];
      if (child === undefined || child === null || typeof child !== "object") {
        o[seg.name] = nextIsIndex ? ([] as unknown[]) : {};
        child = o[seg.name];
      }
      cur = child as Record<string, unknown> | unknown[];
    } else {
      const arr = cur as unknown[];
      let child = arr[seg.index];
      if (child === undefined || child === null || typeof child !== "object") {
        arr[seg.index] = nextIsIndex ? ([] as unknown[]) : {};
        child = arr[seg.index];
      }
      cur = child as Record<string, unknown> | unknown[];
    }
  }
  const last = segments[segments.length - 1]!;
  if (last.kind === "field") {
    (cur as Record<string, unknown>)[last.name] = value;
  } else {
    (cur as unknown[])[last.index] = value;
  }
}

/** 🧭️ Clears the value at `segments` (deletes final field or sets array slot to `undefined`). */
export function clearPathSegments(root: Record<string, unknown>, segments: readonly PathSegment[]): void {
  if (segments.length === 0) return;
  if (segments.length === 1 && segments[0]!.kind === "field") {
    delete root[segments[0]!.name];
    return;
  }
  let cur: unknown = root;
  for (let i = 0; i < segments.length - 1; i++) {
    const seg = segments[i]!;
    if (cur === null || cur === undefined) return;
    if (seg.kind === "field") cur = (cur as Record<string, unknown>)[seg.name];
    else cur = (cur as unknown[])[seg.index];
  }
  const last = segments[segments.length - 1]!;
  const parent = cur as Record<string, unknown> | unknown[];
  if (last.kind === "field") delete (parent as Record<string, unknown>)[last.name];
  else (parent as unknown[])[last.index] = undefined;
}

// #region 🏷️Metadata
/** 🏷️ Sidecar semantic fields keyed by geometry or derived entity id (`FaceRef`, `EdgeRef`, …); never stored on brepjs shapes. */
export class AttributeTable {
  private readonly byId = new Map<string, Record<string, unknown>>();

  constructor(private readonly bumpRevision: () => void) {}

  get(id: string): Readonly<Record<string, unknown>> | undefined {
    const r = this.byId.get(id);
    return r ? r : undefined;
  }

  setField(id: string, key: string, value: unknown): void {
    let r = this.byId.get(id);
    if (!r) {
      r = {};
      this.byId.set(id, r);
    }
    r[key] = value;
    this.bumpRevision();
  }

  deleteField(id: string, key: string): void {
    const r = this.byId.get(id);
    if (!r || !Object.hasOwn(r, key)) return;
    delete r[key];
    if (Object.keys(r).length === 0) this.byId.delete(id);
    this.bumpRevision();
  }

  deleteEntity(id: string): void {
    if (this.byId.delete(id)) this.bumpRevision();
  }

  /** 🧭️ Iterates `(entityId, fields)` pairs in stable id order. */
  entries(): Iterable<[string, Readonly<Record<string, unknown>>]> {
    return [...this.byId.entries()].sort(([a], [b]) => a.localeCompare(b));
  }

  /** 🧭️ Serializes sidecar attribute rows for STEP / JSON. */
  toJSON(): readonly { readonly id: string; readonly fields: Readonly<Record<string, unknown>> }[] {
    return [...this.entries()].map(([id, fields]) => ({ id, fields }));
  }

  /** 🧭️ Hydrates sidecar attributes from JSON rows. */
  static fromJSON(rows: readonly { readonly id: string; readonly fields: Readonly<Record<string, unknown>> }[]): AttributeTable {
    const store = new AttributeTable(() => {});
    for (const row of rows ?? []) store.byId.set(row.id, { ...row.fields });
    return store;
  }

  /** 🧭️ Replaces all attribute rows; bumps parent revision when `bumpRevision` is true. */
  loadSnapshot(rows: readonly { readonly id: string; readonly fields: Readonly<Record<string, unknown>> }[], bumpRevision = true): void {
    this.byId.clear();
    for (const row of rows ?? []) this.byId.set(row.id, { ...row.fields });
    if (bumpRevision && rows.length > 0) this.bumpRevision();
  }

  /** 👁️ Reads persisted hide/lock flags for any geometry or object entity id. */
  getEntityFlags(id: string): SpatialEntityFlags {
    return spatialEntityFlagsFromFields(this.get(id));
  }

  /** 👁️ Sets one persisted hide/lock flag; clears the field when set to false. */
  setEntityFlag(id: string, flag: SpatialEntityFlagKey, value: boolean): void {
    if (value) {
      this.setField(id, flag, true);
      return;
    }
    this.deleteField(id, flag);
  }
}

/** 👁️ Persisted per-entity hide/lock keys stored in `Model.metadata`. */
export type SpatialEntityFlagKey = "hidden" | "locked";

/** 👁️ Persisted per-entity hide/lock flags for CAD spatial entities. */
export interface SpatialEntityFlags {
  readonly hidden?: boolean;
  readonly locked?: boolean;
}

/** 👁️ Parses hide/lock flags from a metadata field row. */
export function spatialEntityFlagsFromFields(fields: Readonly<Record<string, unknown>> | undefined): SpatialEntityFlags {
  if (!fields) {
    return {};
  }
  return {
    ...(fields.hidden === true ? { hidden: true } : {}),
    ...(fields.locked === true ? { locked: true } : {}),
  };
}

/** 🪪️ `evalExpr` `field` target: a bound geometry row entity (`kind` + `id`). */
export interface ModelEntityRef {
  readonly kind: ModelEntityKind;
  readonly id: string;
}
// #endregion 🏷️Metadata
//#endregion 🗺️Paths
