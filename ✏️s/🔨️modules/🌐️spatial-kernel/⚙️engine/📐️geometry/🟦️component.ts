// #region 🧲️Header
/** @emoji 🧭️ `@semio-tech/cad-js` — CAD domain module facet. See `cad/AGENTS.md`. */
import { ephemeralBox, ephemeralMap, ephemeralWeakMap } from "@semio-tech/framework";
import type { ArcPlaneFrame, EdgeCurve, EdgeGroup, EdgeInfo, FaceGroup, FaceInfo, MeshTransfer, Vec3 } from "@semio-tech/s-3d-js";
import { emptyMeshTransfer, kernelGeometry, solidRef } from "@semio-tech/s-3d-js";
// #endregion 🧲️Header



// #region 📦️📐️geometry
// #region 🧮️Vec
// #endregion 🧮️Vec

// #region 🧱️kernelGeometry

// #region 🎮️InteractionEvent
/** @emoji 🧭️ Interaction input envelope; `kind` selects `machine.states[*].on` keys. */
export type InteractionEvent = { readonly kind: string; readonly [k: string]: unknown };
// #endregion 🎮️InteractionEvent

// #region 🪪️Selection
const MODEL_ENTITY_KINDS = new Set<string>(["anchor", "vertex", "edge", "wire", "face", "shell", "solid", "object", "geometry", "attribute"]);

/** @emoji 🪪️ One picked geometry or derived view target for `selection.changed`. */
export interface SelectionTarget {
  readonly kind: ModelEntityKind;
  readonly id: string;
  readonly editable: boolean;
  readonly derivedFrom?: readonly { kind: EditableEntityKind; id: string }[];
}

/** @emoji 🪪️ Host selection payload; `targets` filtered by `SelectionSpec.accept`. */
export interface SelectionEvent extends InteractionEvent {
  readonly kind: "selection.changed";
  readonly targets: readonly SelectionTarget[];
  readonly point?: Vec3;
}

/** @emoji 🪪️ Per-state declarative filter for model vs extension-view picking. */
export interface SelectionSpec {
  readonly accept: readonly ModelEntityKind[];
  readonly multiple?: boolean;
  readonly prompt?: string;
}

/** @emoji 🧭️ Returns `targets` whose `kind` is listed in `spec.accept`. */
export function filterSelectionTargets(spec: SelectionSpec, targets: readonly SelectionTarget[]): SelectionTarget[] {
  return targets.filter((t) => spec.accept.includes(t.kind));
}

/** @emoji 🧭️ Maps object picks to wire/edge primitives when `spec.accept` lists curve geometry kinds. */
export function expandSelectionTargetsForAccept(model: Model, spec: SelectionSpec, targets: readonly SelectionTarget[]): SelectionTarget[] {
  const accept = new Set(spec.accept);
  const out: SelectionTarget[] = [];
  const seen = new Set<string>();
  const push = (kind: ModelEntityKind, id: string): void => {
    const key = `${kind}:${id}`;
    if (seen.has(key)) return;
    seen.add(key);
    out.push({ kind, id, editable: true });
  };
  for (const target of targets) {
    if (accept.has(target.kind)) {
      push(target.kind, target.id);
      continue;
    }
    if (target.kind !== "object") continue;
    const row = model.objects[target.id as ObjectRef];
    if (!row) continue;
    const curveRef = row.primitives.curve;
    if (!curveRef) continue;
    const topo = resolveKernelTopologyKind(model, String(curveRef));
    if (accept.has("wire") && (topo === "wire" || topo === null)) push("wire", String(curveRef));
    else if (accept.has("edge") && (topo === "edge" || topo === null)) push("edge", String(curveRef));
  }
  return out;
}

/** @emoji 🧭️ True when every target is accepted (and at least one target exists). */
export function selectionEventMatches(spec: SelectionSpec, ev: SelectionEvent): boolean {
  if (!ev.targets || ev.targets.length === 0) return false;
  const xs = filterSelectionTargets(spec, ev.targets);
  if (xs.length !== ev.targets.length) return false;
  if (!spec.multiple && xs.length > 1) return false;
  return true;
}

/** @emoji 🧭️ Active `selection` block for `state`, or `null` when unrestricted. */
export function getActiveSelectionSpec(
  spec: {
    readonly machine: {
      readonly states: readonly { readonly name: string; readonly selection?: SelectionSpec }[];
    };
  },
  state: string,
): SelectionSpec | null {
  const st = spec.machine.states.find((s) => s.name === state);
  const s = st?.selection;
  return s ?? null;
}

/** @emoji ✅️ Whether Enter/Space can fire a guarded `confirm` transition in `state`. */
export function interactionCanConfirmSelection(spec: InteractionSpec, state: string, ctx: Record<string, unknown>, preview: SpatialPreviewKernel): boolean {
  if (!getActiveSelectionSpec(spec, state)) return false;
  const handler = spec.machine.states.find((s) => s.name === state)?.on?.find((h) => h.event === "confirm");
  if (!handler) return false;
  for (const tr of handler.transitions) {
    if (typeof tr.key !== "string" || tr.key.length === 0) continue;
    if (tr.guard) {
      const g = lookupGuard(spec, tr.guard);
      if (!g || !evalGuard(g, { context: ctx, preview })) continue;
    }
    return true;
  }
  return false;
}
// #endregion 🪪️Selection

// #region 🗺️Paths
/** @emoji 🧭️ Root object for segmented path reads (`context`, `event`, or action `params`). */
export type PathRoot = "context" | "event" | "params";

/** @emoji 🧭️ One navigation step: object field or array index (no dynamic JSON keys). */
export type PathSegment = { readonly kind: "field"; readonly name: string } | { readonly kind: "index"; readonly index: number };

/** @emoji 🧭️ Absolute path into `context` or `event` payloads. */
export interface PathTarget {
  readonly root: PathRoot;
  readonly segments: readonly PathSegment[];
}

/** @emoji 🧭️ Reads `segments` from `root` (object/array chain). */
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

function pathRootRecord(root: PathRoot, env: ExprEnv): unknown {
  if (root === "context") return env.context;
  if (root === "params") return env.params ?? {};
  return env.event;
}

/** @emoji 🧭️ Resolves a `PathTarget` against `ExprEnv`. */
export function readPathTarget(t: PathTarget, env: ExprEnv): unknown {
  return readPathSegments(pathRootRecord(t.root, env), t.segments);
}

/** @emoji 🧭️ Writes `value` at `segments` under `root` (creates object/array shells). */
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

/** @emoji 🧭️ Writes into `env.context` using a context-rooted path. */
export function writePathTarget(t: PathTarget, env: ExprEnv, value: unknown): void {
  if (t.root !== "context") return;
  writePathSegments(env.context, t.segments, value);
}

/** @emoji 🧭️ Clears the value at `segments` (deletes final field or sets array slot to `undefined`). */
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

/** @emoji 🧭️ Clears `target` on `env.context`. */
export function clearPathTarget(t: PathTarget, env: ExprEnv): void {
  if (t.root !== "context") return;
  clearPathSegments(env.context, t.segments);
}
// #endregion 🗺️Paths

// #region 🏷️Metadata
/** @emoji 🏷️ Sidecar semantic fields keyed by geometry or derived entity id (`FaceRef`, `EdgeRef`, …); never stored on brepjs shapes. */
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

  /** @emoji 🧭️ Iterates `(entityId, fields)` pairs in stable id order. */
  entries(): Iterable<[string, Readonly<Record<string, unknown>>]> {
    return [...this.byId.entries()].sort(([a], [b]) => a.localeCompare(b));
  }

  /** @emoji 🧭️ Serializes sidecar attribute rows for STEP / JSON. */
  toJSON(): readonly { readonly id: string; readonly fields: Readonly<Record<string, unknown>> }[] {
    return [...this.entries()].map(([id, fields]) => ({ id, fields }));
  }

  /** @emoji 🧭️ Hydrates sidecar attributes from JSON rows. */
  static fromJSON(rows: readonly { readonly id: string; readonly fields: Readonly<Record<string, unknown>> }[]): AttributeTable {
    const store = new AttributeTable(() => {});
    for (const row of rows ?? []) store.byId.set(row.id, { ...row.fields });
    return store;
  }

  /** @emoji 🧭️ Replaces all attribute rows; bumps parent revision when `bumpRevision` is true. */
  loadSnapshot(rows: readonly { readonly id: string; readonly fields: Readonly<Record<string, unknown>> }[], bumpRevision = true): void {
    this.byId.clear();
    for (const row of rows ?? []) this.byId.set(row.id, { ...row.fields });
    if (bumpRevision && rows.length > 0) this.bumpRevision();
  }

  /** @emoji 👁️ Reads persisted hide/lock flags for any geometry or object entity id. */
  getEntityFlags(id: string): SpatialEntityFlags {
    return spatialEntityFlagsFromFields(this.get(id));
  }

  /** @emoji 👁️ Sets one persisted hide/lock flag; clears the field when set to false. */
  setEntityFlag(id: string, flag: SpatialEntityFlagKey, value: boolean): void {
    if (value) {
      this.setField(id, flag, true);
      return;
    }
    this.deleteField(id, flag);
  }
}

/** @emoji 👁️ Persisted per-entity hide/lock keys stored in `Model.metadata`. */
export type SpatialEntityFlagKey = "hidden" | "locked";

/** @emoji 👁️ Persisted per-entity hide/lock flags for CAD spatial entities. */
export interface SpatialEntityFlags {
  readonly hidden?: boolean;
  readonly locked?: boolean;
}

/** @emoji 👁️ Parses hide/lock flags from a metadata field row. */
export function spatialEntityFlagsFromFields(fields: Readonly<Record<string, unknown>> | undefined): SpatialEntityFlags {
  if (!fields) {
    return {};
  }
  return {
    ...(fields.hidden === true ? { hidden: true } : {}),
    ...(fields.locked === true ? { locked: true } : {}),
  };
}

/** @emoji 🪪️ `evalExpr` `field` target: a bound geometry row entity (`kind` + `id`). */
export interface ModelEntityRef {
  readonly kind: ModelEntityKind;
  readonly id: string;
}
// #endregion 🏷️Metadata

// #region 🗺️Expr
/** @emoji 🗺️ Tagged declarative expression evaluated by `evalExpr`. */
export type Expr = ExprPath | ExprConst | ExprVar | ExprField | ExprLet | ExprExists | ExprNotEmpty | ExprAll | ExprAny | ExprNot | ExprAbs | ExprDistance | ExprKernelCall | ExprBinop | ExprFold;

export interface ExprPath {
  readonly kind: "path";
  readonly root: PathRoot;
  readonly segments: readonly PathSegment[];
}
export interface ExprConst {
  readonly kind: "const";
  readonly value: unknown;
}
export interface ExprVar {
  readonly kind: "var";
  readonly name: string;
}
export interface ExprField {
  readonly kind: "field";
  readonly object: Expr;
  readonly name: string;
}
export interface ExprLet {
  readonly kind: "let";
  readonly bindings: readonly { readonly name: string; readonly value: Expr }[];
  readonly in: Expr;
}
export interface ExprExists {
  readonly kind: "exists";
  readonly target: PathTarget;
}
export interface ExprNotEmpty {
  readonly kind: "notEmpty";
  readonly target: PathTarget;
}
export interface ExprAll {
  readonly kind: "all";
  readonly args: readonly Expr[];
}
export interface ExprAny {
  readonly kind: "any";
  readonly args: readonly Expr[];
}
export interface ExprNot {
  readonly kind: "not";
  readonly arg: Expr;
}
export interface ExprAbs {
  readonly kind: "abs";
  readonly arg: Expr;
}
export interface ExprDistance {
  readonly kind: "distance";
  readonly a: Expr;
  readonly b: Expr;
}
export interface ExprKernelCall {
  readonly kind: "kernel.call";
  readonly function: string;
  readonly args?: Record<string, Expr>;
}
export interface ExprBinop {
  readonly kind: "binop";
  readonly operation: "==" | "!=" | ">" | "<" | ">=" | "<=" | "+" | "-" | "*" | "/";
  readonly left: Expr;
  readonly right: Expr;
}
export interface ExprFold {
  readonly kind: "fold";
  readonly operation: "min" | "max";
  readonly args: readonly [Expr, Expr];
}

export interface ExprEnv {
  readonly context: Record<string, unknown>;
  readonly event?: Record<string, unknown>;
  readonly params?: Record<string, unknown>;
  readonly vars?: Record<string, unknown>;
  readonly model?: Model;
  readonly metadata?: AttributeTable;
  readonly activeModelDefinitionId?: string | null;
  readonly kernel?: SpatialKernel;
  readonly actionId?: string;
  readonly preview: SpatialPreviewKernel;
}

function envWithVars(base: ExprEnv, vars: Record<string, unknown>): ExprEnv {
  return {
    context: base.context,
    event: base.event,
    params: base.params,
    vars: { ...base.vars, ...vars },
    model: base.model,
    activeModelDefinitionId: base.activeModelDefinitionId,
    kernel: base.kernel,
    actionId: base.actionId,
    metadata: base.metadata,
    preview: base.preview,
  };
}

function isVec3(v: unknown): v is Vec3 {
  return Array.isArray(v) && v.length === 3 && v.every((n) => typeof n === "number");
}

function isModelEntityRef(v: unknown): v is ModelEntityRef {
  if (!v || typeof v !== "object") return false;
  const o = v as Record<string, unknown>;
  return typeof o.kind === "string" && typeof o.id === "string";
}

/** @emoji 🧮️ Evaluates a tagged `Expr` against `ExprEnv` (guards + action values). */
export function evalExpr(expr: Expr, env: ExprEnv): unknown {
  switch (expr.kind) {
    case "const":
      return expr.value;
    case "path":
      return readPathTarget({ root: expr.root, segments: expr.segments }, env);
    case "var":
      return env.vars ? env.vars[expr.name] : undefined;
    case "field": {
      const o = evalExpr(expr.object, env);
      const model = env.model;
      if (model && isModelEntityRef(o)) {
        return readModelEntityProperty(model, env.metadata, o.kind, o.id, expr.name, {
          activeModelDefinitionId: env.activeModelDefinitionId,
          preview: env.preview,
        });
      }
      if (o !== null && o !== undefined && typeof o === "object" && !Array.isArray(o)) {
        return (o as Record<string, unknown>)[expr.name];
      }
      return undefined;
    }
    case "let": {
      const next: Record<string, unknown> = {};
      for (const b of expr.bindings) {
        next[b.name] = evalExpr(b.value, env);
      }
      return evalExpr(expr.in, envWithVars(env, next));
    }
    case "exists": {
      const v = readPathTarget(expr.target, env);
      return v !== undefined && v !== null;
    }
    case "notEmpty": {
      const v = readPathTarget(expr.target, env);
      if (v === undefined || v === null) return false;
      if (Array.isArray(v)) return v.length > 0;
      if (typeof v === "string") return v.length > 0;
      return true;
    }
    case "all":
      return expr.args.every((x) => Boolean(evalExpr(x, env)));
    case "any":
      return expr.args.some((x) => Boolean(evalExpr(x, env)));
    case "not":
      return !evalExpr(expr.arg, env);
    case "abs": {
      const v = evalExpr(expr.arg, env);
      return typeof v === "number" ? env.preview.abs(v) : undefined;
    }
    case "distance": {
      const va = evalExpr(expr.a, env);
      const vb = evalExpr(expr.b, env);
      if (!isVec3(va) || !isVec3(vb)) return undefined;
      return env.preview.vec3Distance(va, vb);
    }
    case "fold":
      return expr.operation === "min" ? env.preview.min2(Number(evalExpr(expr.args[0], env)), Number(evalExpr(expr.args[1], env))) : env.preview.max2(Number(evalExpr(expr.args[0], env)), Number(evalExpr(expr.args[1], env)));
    case "binop": {
      const left = evalExpr(expr.left, env);
      const right = evalExpr(expr.right, env);
      switch (expr.operation) {
        case "==":
          return left === right;
        case "!=":
          return left !== right;
        case ">":
          return Number(left) > Number(right);
        case "<":
          return Number(left) < Number(right);
        case ">=":
          return Number(left) >= Number(right);
        case "<=":
          return Number(left) <= Number(right);
        case "+":
          return Number(left) + Number(right);
        case "-":
          return Number(left) - Number(right);
        case "*":
          return Number(left) * Number(right);
        case "/":
          return Number(right) === 0 ? undefined : Number(left) / Number(right);
        default:
          return undefined;
      }
    }
    default:
      return undefined;
  }
}

/** @emoji 🧭️ Coerces `evalExpr` output to strict boolean guard result. */
export function evalGuard(expr: Expr, env: ExprEnv): boolean {
  return Boolean(evalExpr(expr, env));
}
// #endregion 🗺️Expr

// #region 📜️Spec
/** @emoji 📜️ Declared interaction-local context slots (`spatial.interaction/v1` `context`). */
export interface ContextFieldDecl {
  readonly name: string;
  readonly kind: "string" | "number" | "boolean" | "vec3" | "stringArray" | "unknown";
  readonly enumValues?: readonly string[];
}

/** @emoji 📜️ Named guard binding (`guards[]`). */
export interface NamedGuard {
  readonly name: string;
  readonly expr: Expr;
}

export interface TransitionSpec {
  readonly target?: string;
  readonly guard?: string;
  readonly transient?: boolean;
  readonly effects?: readonly EffectSpec[];
  readonly key?: string;
  readonly label?: string;
}

export type EffectSpec =
  | { readonly operation: "assign"; readonly target: PathTarget; readonly value: Expr }
  | { readonly operation: "clear"; readonly target: PathTarget }
  | { readonly operation: "append"; readonly target: PathTarget; readonly value: Expr }
  | { readonly operation: "emit"; readonly event: { readonly kind: string } }
  | { readonly operation: "raise"; readonly event: string }
  | { readonly operation: "openTransaction" }
  | { readonly operation: "commitTransaction" }
  | { readonly operation: "rollbackTransaction" }
  | { readonly operation: "requestPreview" }
  | { readonly operation: "kernel.query"; readonly query: string; readonly assignTo: PathTarget; readonly params?: Record<string, Expr> }
  | { readonly operation: "resolveEditable" }
  | { readonly operation: "setDiagnostic"; readonly severity: "info" | "warning" | "error"; readonly code: string; readonly message: string }
  | { readonly operation: "clearDiagnostic"; readonly code: string }
  | { readonly operation: "action"; readonly action: string; readonly params?: Record<string, Expr> }
  | {
      readonly operation: "interaction.call";
      readonly interaction: string;
      readonly inputs?: Record<string, Expr>;
      readonly outputs?: readonly InteractionOutputBinding[] | Record<string, unknown>;
    };

/** @emoji 📞️ Maps host context paths from expressions evaluated against the child session context. */
export interface InteractionOutputBinding {
  readonly target: PathTarget;
  readonly value: Expr;
}

function interactionOutputBindings(outputs: readonly InteractionOutputBinding[] | Record<string, unknown> | undefined): readonly InteractionOutputBinding[] | undefined {
  if (!outputs) return undefined;
  if (Array.isArray(outputs)) return outputs;
  return Object.entries(outputs).map(([hostKey, childKey]) => ({
    target: { root: "context", segments: [{ kind: "field", name: hostKey }] },
    value: { kind: "path", root: "context", segments: [{ kind: "field", name: String(childKey) }] },
  }));
}

export interface EventHandlerSpec {
  readonly event: string;
  readonly transitions: readonly TransitionSpec[];
}

export interface StateDefSpec {
  readonly name: string;
  readonly final?: boolean;
  readonly selection?: SelectionSpec;
  readonly on?: readonly EventHandlerSpec[];
}

export type DisplayItemSpec =
  | { readonly kind: "point"; readonly id: string; readonly role?: string; readonly position: Expr }
  | { readonly kind: "label"; readonly id: string; readonly role?: string; readonly text: string; readonly position: Expr }
  | { readonly kind: "segment"; readonly id: string; readonly role?: string; readonly from: Expr; readonly to: Expr }
  | { readonly kind: "linear-handle"; readonly id: string; readonly role?: string; readonly axis: Vec3; readonly origin: Expr }
  | { readonly kind: "box-preview"; readonly id: string; readonly role?: string; readonly cornerA: Expr; readonly cornerB: Expr; readonly height: Expr }
  | {
      readonly kind: "preview";
      readonly id: string;
      readonly role?: string;
      readonly previewKind: string;
      readonly params?: Record<string, Expr>;
    }
  | {
      readonly kind: "entity-highlight";
      readonly id: string;
      readonly role?: string;
      readonly geometryEntityKind: ModelEntityKind;
      readonly entityId: Expr;
    }
  | { readonly kind: "curve"; readonly id: string; readonly role?: string }
  | { readonly kind: "mesh"; readonly id: string; readonly role?: string };

export type CommitOperationSpec = {
  readonly kind: "action";
  readonly action: string;
  readonly params?: Record<string, Expr>;
};

/** @emoji 📜️ Parsed static interaction document (`spatial.interaction/v1`). */
export interface InteractionSpec {
  readonly schema: "spatial.interaction";
  readonly id: string;
  readonly version: string;
  readonly label?: string;
  readonly context?: { readonly fields: readonly ContextFieldDecl[] };
  readonly requires?: Record<string, unknown>;
  readonly guards?: readonly NamedGuard[];
  readonly machine: {
    readonly initial: string;
    readonly states: readonly StateDefSpec[];
  };
  readonly display?: {
    readonly states?: readonly { readonly state: string; readonly items: readonly DisplayItemSpec[] }[];
  };
  readonly interaction?: InteractionSpatialConfig;
  readonly commit: {
    readonly when?: string;
    readonly fromStates?: readonly string[];
    readonly outputDataPath?: PathTarget;
    readonly operation: CommitOperationSpec;
  };
  /** @emoji 📞️ `standalone` (default) for hosts; `callable` only via `interaction.call`. */
  readonly invocation?: InteractionInvocation;
  /** @emoji 🏷️ Typology object created when this interaction commits geometry. */
  readonly produces?: { readonly typology: string };
}

/** @emoji 📞️ How hosts may start an interaction (`standalone` vs nested-only `callable`). */
export type InteractionInvocation = "standalone" | "callable";

/** @emoji 🎛️ Engagement control kind declared on {@link InteractionLengthEntrySpec} / {@link InteractionScalarEntrySpec}. */
export type InteractionEngagementControlKind = "slider" | "stepper" | "ring";

/** @emoji 🎛️ Optional engagement control parameters on numeric interaction entries. */
export interface InteractionEngagementEntryControl {
  readonly control?: InteractionEngagementControlKind;
  readonly min?: number;
  readonly max?: number;
  readonly step?: number;
  readonly unit?: string;
  readonly default?: number;
}

/** @emoji 📏️ One rubber-band state where REPL digits clamp distance along the cursor ray. */
export interface InteractionLengthEntrySpec extends InteractionEngagementEntryControl {
  readonly state: string;
  readonly anchor: string;
  readonly field: string;
  /** @emoji ✅️ Host commit on Enter/Space (`pointer.down` default, `confirm` for scalar-like steps). */
  readonly commit?: "pointer.down" | "confirm";
}

/** @emoji 🔢️ One state where REPL digits set a scalar context field live (`set.height`, `set.radius`, …). */
export interface InteractionScalarEntrySpec extends InteractionEngagementEntryControl {
  readonly state: string;
  readonly event: string;
  readonly field: string;
  /** @emoji ✅️ Host commit on Enter/Space (defaults to `confirm`). */
  readonly commit?: "pointer.down" | "confirm";
  /** @emoji 📍️ Context path to Vec3 for axis XY (Z from `axisFloor` when set). */
  readonly axisAnchor?: string;
  /** @emoji 📍️ Context path to Vec3 whose Z is the axis floor (defaults to `axisAnchor`). */
  readonly axisFloor?: string;
  readonly axis?: readonly [number, number, number];
}

/** @emoji 🎚️ Resolved numeric engagement control for one interaction state. */
export interface ResolvedInteractionEngagementNumericControl {
  readonly kind: "slider" | "stepper";
  readonly label: string;
  readonly value: number;
  readonly min: number;
  readonly max?: number;
  readonly step: number;
  readonly unit?: string;
}

/** @emoji 🧫️ Resolved ring engagement control for one interaction state. */
export interface ResolvedInteractionEngagementRingControl {
  readonly kind: "ring";
  readonly label: string;
  readonly value?: string;
  readonly options: readonly { readonly id: string; readonly label: string }[];
  readonly min: number;
  readonly max: number;
  readonly step: number;
}

/** @emoji 🎛️ Resolved engagement control descriptor for {@link interactionControlForState}. */
export type ResolvedInteractionEngagementControl = ResolvedInteractionEngagementNumericControl | ResolvedInteractionEngagementRingControl;

/** @emoji 🎮️ Host + viewport hints for spatial picking (declared per interaction). */
export interface InteractionSpatialConfig {
  readonly spatialGroundPick?: boolean;
  readonly pickDisabledStates?: readonly string[];
  readonly groundPointerMoveStates?: readonly string[];
  readonly heightDragStates?: readonly string[];
  readonly verticalRodStates?: readonly string[];
  readonly heightConfirmState?: string | null;
  readonly lengthEntry?: readonly InteractionLengthEntrySpec[];
  readonly scalarEntry?: readonly InteractionScalarEntrySpec[];
}

/** @emoji 📞️ Resolved invocation for an interaction document. */
export function interactionInvocation(spec: InteractionSpec): InteractionInvocation {
  if (spec.invocation === "callable" || spec.invocation === "standalone") return spec.invocation;
  return "standalone";
}

/** @emoji 📞️ True when an interaction must not be started standalone by hosts. */
export function isCallableOnlyInteraction(spec: InteractionSpec): boolean {
  return interactionInvocation(spec) === "callable";
}

function guardNames(spec: InteractionSpec): Set<string> {
  return new Set((spec.guards ?? []).map((g) => g.name));
}

function findState(spec: InteractionSpec, name: string): StateDefSpec | undefined {
  return spec.machine.states.find((s) => s.name === name);
}

/** @emoji 🏁️ True when `state` is marked `final` on the interaction machine. */
export function isFinalInteractionState(spec: InteractionSpec, state: string): boolean {
  return Boolean(findState(spec, state)?.final);
}

function listFinalInteractionStates(spec: InteractionSpec): string[] {
  return spec.machine.states.filter((s) => s.final).map((s) => s.name);
}

function normalizeInteractionCallEffectRaw(fx: Record<string, unknown>): boolean {
  if (fx.operation !== "interaction.call" || typeof fx.interaction !== "string") return false;
  const outputs = fx.outputs;
  if (outputs === undefined) return true;
  if (Array.isArray(outputs)) return outputs.every((row) => row && typeof row === "object" && "target" in (row as object) && "value" in (row as object));
  if (outputs && typeof outputs === "object") {
    fx.outputs = Object.entries(outputs as Record<string, unknown>).map(([hostKey, childKey]) => ({
      target: { root: "context", segments: [{ kind: "field", name: hostKey }] },
      value: { kind: "path", root: "context", segments: [{ kind: "field", name: String(childKey) }] },
    }));
    return true;
  }
  return false;
}

function normalizeInteractionDocumentRaw(r: Record<string, unknown>): void {
  const spatial = r.interaction;
  if (spatial && typeof spatial === "object" && (spatial as Record<string, unknown>).callableOnly === true) {
    r.invocation = "callable";
    delete (spatial as Record<string, unknown>).callableOnly;
  }
  const machine = r.machine;
  if (!machine || typeof machine !== "object") return;
  const states = (machine as Record<string, unknown>).states;
  if (!Array.isArray(states)) return;
  for (const st of states) {
    if (!st || typeof st !== "object") continue;
    const on = (st as Record<string, unknown>).on;
    if (!Array.isArray(on)) continue;
    for (const h of on) {
      if (!h || typeof h !== "object") continue;
      const transitions = (h as Record<string, unknown>).transitions;
      if (!Array.isArray(transitions)) continue;
      for (const tr of transitions) {
        if (!tr || typeof tr !== "object") continue;
        const effects = (tr as Record<string, unknown>).effects;
        if (!Array.isArray(effects)) continue;
        for (const eff of effects) {
          if (!eff || typeof eff !== "object") continue;
          if (!normalizeInteractionCallEffectRaw(eff as Record<string, unknown>)) return;
        }
      }
    }
  }
}

/** @emoji 📞️ Writes child session values onto host context using declarative output bindings. */
export function mergeInteractionCallOutputs(hostContext: Record<string, unknown>, childContext: Record<string, unknown>, outputs: readonly InteractionOutputBinding[] | Record<string, unknown> | undefined): void {
  const bindings = interactionOutputBindings(outputs);
  if (!bindings?.length) return;
  const childEnv: ExprEnv = { context: childContext, event: { kind: "interaction.return" } };
  const hostEnv: ExprEnv = { ...childEnv, context: hostContext };
  for (const row of bindings) {
    writePathTarget(row.target, hostEnv, evalExpr(row.value, childEnv));
  }
}

/** @emoji 🧾️ Validates and returns an `InteractionSpec` or `null` when malformed. */
export function parseInteractionSpec(raw: unknown): InteractionSpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = structuredClone(raw) as Record<string, unknown>;
  if (r.schema !== "spatial.interaction") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string") return null;
  const machine = r.machine;
  if (!machine || typeof machine !== "object") return null;
  const m = machine as Record<string, unknown>;
  if (typeof m.initial !== "string" || !Array.isArray(m.states)) return null;
  const states = m.states as unknown[];
  const names = new Set<string>();
  for (const st of states) {
    if (!st || typeof st !== "object") return null;
    const s = st as Record<string, unknown>;
    if (typeof s.name !== "string" || s.name.length === 0) return null;
    if (names.has(s.name)) return null;
    names.add(s.name);
    const sel = s.selection;
    if (sel) {
      if (typeof sel !== "object") return null;
      const acc = (sel as Record<string, unknown>).accept;
      if (!Array.isArray(acc) || acc.length === 0) return null;
      for (const k of acc) {
        if (typeof k !== "string" || !MODEL_ENTITY_KINDS.has(k)) return null;
      }
    }
    const on = s.on;
    if (on !== undefined) {
      if (!Array.isArray(on)) return null;
      for (const h of on) {
        if (!h || typeof h !== "object") return null;
        const he = h as Record<string, unknown>;
        if (typeof he.event !== "string" || !Array.isArray(he.transitions)) return null;
        for (const tr of he.transitions as unknown[]) {
          if (!tr || typeof tr !== "object") return null;
          const t = tr as Record<string, unknown>;
          if (t.effects !== undefined && !Array.isArray(t.effects)) return null;
          if (t.actions !== undefined) return null;
        }
      }
    }
  }
  if (!names.has(m.initial as string)) return null;
  const commit = r.commit;
  if (!commit || typeof commit !== "object") return null;
  const c = commit as Record<string, unknown>;
  const operation = c.operation;
  if (!operation || typeof operation !== "object") return null;
  const o = operation as Record<string, unknown>;
  if (o.kind !== "action" || typeof o.action !== "string") return null;
  if (r.invocation !== undefined && r.invocation !== "standalone" && r.invocation !== "callable") return null;
  normalizeInteractionDocumentRaw(r);
  const spec = r as unknown as InteractionSpec;
  const gn = guardNames(spec);
  if (c.when !== undefined && typeof c.when === "string" && !gn.has(c.when)) return null;
  for (const st of spec.machine.states) {
    for (const h of st.on ?? []) {
      for (const tr of h.transitions) {
        if (tr.guard !== undefined && typeof tr.guard === "string" && !gn.has(tr.guard)) return null;
      }
    }
  }
  return spec;
}

const COMPILED_INITIAL_CONTEXTS = ephemeralWeakMap<InteractionSpec, Record<string, unknown>>("s.plugins.cad.modules.core.component.ts.COMPILED_INITIAL_CONTEXTS");

function initialStartTransition(spec: InteractionSpec): TransitionSpec | null {
  const initial = findState(spec, spec.machine.initial);
  const handler = initial?.on?.find((h) => h.event === "start");
  const transition = handler?.transitions[0];
  return transition?.target ? transition : null;
}

function staticInitialContext(spec: InteractionSpec, transition: TransitionSpec | null): Record<string, unknown> {
  const context: Record<string, unknown> = {};
  if (!transition?.effects) return context;
  const env: ExprEnv = { context, event: { kind: "start" } };
  for (const effect of transition.effects) {
    if (effect.operation === "interaction.call") continue;
    if (effect.operation === "assign") writePathTarget(effect.target, env, evalExpr(effect.value, env));
    else if (effect.operation === "clear") clearPathTarget(effect.target, env);
    else if (effect.operation === "append") {
      const cur = readPathTarget(effect.target, env);
      const next = Array.isArray(cur) ? [...cur, evalExpr(effect.value, env)] : [evalExpr(effect.value, env)];
      writePathTarget(effect.target, env, next);
    }
  }
  return context;
}

function compileInitialState(spec: InteractionSpec, transition: TransitionSpec | null): InteractionSpec {
  if (!transition?.target) return spec;
  return {
    ...spec,
    machine: {
      ...spec.machine,
      initial: transition.target,
    },
  };
}

/** @emoji 📜️ Scripted commands that end in `committed` should commit from that state, not missing `ready`. */
function normalizeCommitFromStates(spec: InteractionSpec): InteractionSpec {
  const finals = listFinalInteractionStates(spec);
  const hasReady = spec.machine.states.some((s) => s.name === "ready");
  if (hasReady || finals.length === 0) return spec;
  const from = spec.commit.fromStates;
  const onlyReady = !from || (from.length === 1 && from[0] === "ready");
  if (!onlyReady) return spec;
  const fromStates = finals.includes("committed") ? ["committed"] : finals;
  return { ...spec, commit: { ...spec.commit, fromStates } };
}

export function initialContextForSpec(spec: InteractionSpec): Record<string, unknown> {
  return structuredClone(COMPILED_INITIAL_CONTEXTS.get(spec) ?? {});
}

/** @emoji 🧭️ Normalizes a parsed interaction so runtime sessions begin in the first active state. */
export function compileInteraction(spec: InteractionSpec): InteractionSpec {
  const start = initialStartTransition(spec);
  if (!start) {
    const normalized = normalizeCommitFromStates(spec);
    if (!COMPILED_INITIAL_CONTEXTS.has(normalized)) COMPILED_INITIAL_CONTEXTS.set(normalized, {});
    return normalized;
  }
  const compiled = normalizeCommitFromStates(compileInitialState(spec, start));
  COMPILED_INITIAL_CONTEXTS.set(compiled, staticInitialContext(spec, start));
  return compiled;
}
// #endregion 📜️Spec

// #region 🧱️kernelGeometry

type AnchorRef = kernelGeometry.AnchorRef;
export type VertexRef = kernelGeometry.VertexRef;
export type EdgeRef = kernelGeometry.EdgeRef;
export type WireRef = kernelGeometry.WireRef;
export type FaceRef = kernelGeometry.FaceRef;
export type ShellRef = kernelGeometry.ShellRef;
export type SolidRef = kernelGeometry.SolidRef;
type GeometryEntityKind = kernelGeometry.GeometryEntityKind;
type EditableEntityKind = GeometryEntityKind;

/** @emoji 🧭️ Framework + brepjs sub-element selection kinds. */
export type ModelEntityKind = EditableEntityKind | "object" | "geometry" | "attribute";
// #endregion 🧱️kernelGeometry

type VertexRecord = kernelGeometry.VertexRecord;
type AnchorAttachment = kernelGeometry.AnchorAttachment;
type AnchorRecord = kernelGeometry.AnchorRecord;
type EdgeRecord = kernelGeometry.EdgeRecord;
type WireRecord = kernelGeometry.WireRecord;
type FaceSurface = kernelGeometry.FaceSurface;
type FaceRecord = kernelGeometry.FaceRecord;
type ShellRecord = kernelGeometry.ShellRecord;
type SolidPrimitive = kernelGeometry.SolidPrimitive;
type SolidRecord = kernelGeometry.SolidRecord;
type KernelGeometryJson = kernelGeometry.KernelGeometryJson;

/** @emoji 🪪️ Opaque object id in a model. */
export type ObjectRef = string & { readonly __brand: "ObjectRef" };

/** @emoji 🪪️ Typology id referenced by objects and extension assets. */
export type TypologyRef = string & { readonly __brand: "TypologyRef" };

/** @emoji 📦️ Primitive refs owned by one object row. */
export type SpatialObjectPrimitives = Readonly<Record<string, string>>;

/** @emoji 📦️ Object instance row in a model (`typology` + kernel `primitives`). */
export interface SpatialObjectRecord {
  readonly id: ObjectRef;
  readonly typology: TypologyRef;
  readonly primitives: SpatialObjectPrimitives;
  readonly attributes?: Readonly<Record<string, unknown>>;
}

/** @emoji 🗺️ Serializable model (`spatial.model/v1`). */
export interface ModelJson {
  readonly schema: "spatial.model";
  readonly revision: number;
  readonly objects: readonly SpatialObjectRecord[];
  readonly geometry: KernelGeometryJson;
  readonly metadata?: readonly { readonly id: string; readonly fields: Readonly<Record<string, unknown>> }[];
}

function recordsById<T extends { id: string }>(xs: readonly T[]): Record<string, T> {
  const o: Record<string, T> = {};
  for (const x of xs) o[x.id] = x;
  return o;
}

function sortedRecordValues<T extends { id: string }>(bucket: Record<string, T>): T[] {
  return Object.keys(bucket)
    .sort()
    .map((k) => bucket[k]!);
}

function sortedPrimitiveEntries(primitives: Record<string, unknown>): readonly (readonly [string, string])[] {
  return Object.entries(primitives)
    .filter((entry): entry is [string, string] => typeof entry[0] === "string" && typeof entry[1] === "string" && entry[0].length > 0)
    .sort(([left], [right]) => left.localeCompare(right));
}

function normalizeSpatialObjectPrimitives(primitives: unknown): SpatialObjectPrimitives {
  return primitives && typeof primitives === "object" ? Object.fromEntries(sortedPrimitiveEntries(primitives as Record<string, unknown>)) : {};
}

function defaultBaseObjectTypology(): TypologyRef {
  const manifest = listModelDefinitionManifests().find((row) => row.id === defaultModelDefinitionId());
  return (manifest?.baseObjectTypology ?? "") as TypologyRef;
}

function normalizeSpatialObjectRecord(row: SpatialObjectRecord | Record<string, unknown>): SpatialObjectRecord {
  return {
    id: String(row.id) as ObjectRef,
    typology: String(row.typology ?? defaultBaseObjectTypology()) as TypologyRef,
    primitives: Array.isArray(row.primitives) ? {} : normalizeSpatialObjectPrimitives(row.primitives),
    ...(row.attributes && typeof row.attributes === "object" ? { attributes: row.attributes as Readonly<Record<string, unknown>> } : {}),
  };
}

function readVec3FromUnknown(value: unknown): Vec3 | null {
  if (!Array.isArray(value) || value.length < 3) return null;
  const x = Number(value[0]);
  const y = Number(value[1]);
  const z = Number(value[2]);
  if (![x, y, z].every((n) => Number.isFinite(n))) return null;
  return [x, y, z];
}

/** @emoji 🧱️ Promotes inline `objects[].primitives[]` rows into kernel geometry tables and slot refs. */
export function materializeInlineObjectPrimitives(model: Model, rawObjects: readonly unknown[] = []): void {
  let changed = false;
  const rawById = new Map<string, Record<string, unknown>>();
  for (const entry of rawObjects) {
    if (!entry || typeof entry !== "object") continue;
    const row = entry as Record<string, unknown>;
    if (typeof row.id === "string") rawById.set(row.id, row);
  }
  for (const [objectId, object] of Object.entries(model.objects)) {
    const raw = rawById.get(objectId)?.primitives;
    if (!Array.isArray(raw)) continue;
    const slots: Record<string, string> = {};
    for (const entry of raw) {
      if (!entry || typeof entry !== "object") continue;
      const row = entry as Record<string, unknown>;
      const kind = typeof row.kind === "string" ? row.kind : "";
      const id = typeof row.id === "string" ? row.id : "";
      if (!kind || !id) continue;
      const slot = typeof row.slot === "string" && row.slot.length > 0 ? row.slot : kind;
      slots[slot] = id;
      if (kind === "vertex") {
        const position = readVec3FromUnknown(row.position);
        if (position) {
          model.vertices[id as VertexRef] = { id: id as VertexRef, position };
          changed = true;
        }
        continue;
      }
      if ((kind === "curve" || kind === "wire") && Array.isArray(row.edgeIds)) {
        model.wires[id as WireRef] = { id: id as WireRef, edgeIds: row.edgeIds.map(String) as EdgeRef[] };
        changed = true;
        continue;
      }
      if ((kind === "curve" || kind === "edge") && Array.isArray(row.vertexIds) && row.vertexIds.length >= 2) {
        model.edges[id as EdgeRef] = {
          id: id as EdgeRef,
          vertexIds: [String(row.vertexIds[0]), String(row.vertexIds[1])] as [VertexRef, VertexRef],
          ...(row.curve && typeof row.curve === "object" ? { curve: row.curve as EdgeCurve } : {}),
        };
        changed = true;
        continue;
      }
      if ((kind === "surface" || kind === "face") && Array.isArray(row.wireIds)) {
        model.faces[id as FaceRef] = {
          id: id as FaceRef,
          wireIds: row.wireIds.map(String) as WireRef[],
          ...(row.surface && typeof row.surface === "object" ? { surface: row.surface as FaceSurface } : {}),
        };
        changed = true;
        continue;
      }
      if (kind === "shell" && Array.isArray(row.faceIds)) {
        model.shells[id as ShellRef] = { id: id as ShellRef, faceIds: row.faceIds.map(String) as FaceRef[] };
        changed = true;
        continue;
      }
      if (kind === "solid") {
        model.solids[id as SolidRef] = {
          id: id as SolidRef,
          shellIds: Array.isArray(row.shellIds) ? (row.shellIds.map(String) as ShellRef[]) : [],
          ...(row.solid && typeof row.solid === "object" ? { solid: row.solid as SolidPrimitive } : {}),
        };
        changed = true;
      }
    }
    model.objects[objectId] = { ...object, primitives: normalizeSpatialObjectPrimitives(slots) };
    changed = true;
  }
  if (changed) model.bump();
}

export function objectPrimitiveEntries(object: SpatialObjectRecord): readonly (readonly [string, string])[] {
  return sortedPrimitiveEntries(object.primitives);
}

export function objectPrimitiveRefs(object: SpatialObjectRecord): readonly string[] {
  return objectPrimitiveEntries(object).map(([, primitiveRef]) => primitiveRef);
}

export function objectPrimaryPrimitiveRef(object: SpatialObjectRecord): string | null {
  return objectPrimitiveEntries(object)[0]?.[1] ?? null;
}

/** @emoji 🧱️ Mutable in-memory model: objects + kernel-private geometry + attribute store. */
export class Model {
  revision = 0;
  objects: Record<string, SpatialObjectRecord> = {};
  anchors: Record<string, AnchorRecord> = {};
  vertices: Record<string, VertexRecord> = {};
  edges: Record<string, EdgeRecord> = {};
  wires: Record<string, WireRecord> = {};
  faces: Record<string, FaceRecord> = {};
  shells: Record<string, ShellRecord> = {};
  solids: Record<string, SolidRecord> = {};
  readonly metadata: AttributeTable = new AttributeTable(() => this.bump());

  /** @emoji 🧭️ Serializes to `ModelJson` (stable id-sorted arrays). */
  toJSON(): ModelJson {
    const meta = this.metadata.toJSON();
    return {
      schema: "spatial.model",
      revision: this.revision,
      objects: sortedRecordValues(this.objects),
      geometry: {
        anchors: sortedRecordValues(this.anchors),
        vertices: sortedRecordValues(this.vertices),
        edges: sortedRecordValues(this.edges),
        wires: sortedRecordValues(this.wires),
        faces: sortedRecordValues(this.faces),
        shells: sortedRecordValues(this.shells),
        solids: sortedRecordValues(this.solids),
      },
      ...(meta.length > 0 ? { metadata: meta } : {}),
    };
  }

  /** @emoji 🧭️ Hydrates from `ModelJson`. */
  static fromJSON(j: ModelJson): Model {
    const g = new Model();
    g.revision = j.revision;
    const rawObjects = j.objects ?? [];
    g.objects = recordsById(rawObjects.map((row) => normalizeSpatialObjectRecord(row as SpatialObjectRecord | Record<string, unknown>)));
    const geo = j.geometry ?? (j as unknown as KernelGeometryJson);
    g.anchors = recordsById(geo.anchors ?? []);
    g.vertices = recordsById(geo.vertices ?? []);
    g.edges = recordsById(geo.edges ?? []);
    g.wires = recordsById(geo.wires ?? []);
    g.faces = recordsById(geo.faces ?? []);
    g.shells = recordsById(geo.shells ?? []);
    g.solids = recordsById(geo.solids ?? []);
    if (j.metadata?.length) g.metadata.loadSnapshot(j.metadata, false);
    materializeInlineObjectPrimitives(g, rawObjects);
    return g;
  }

  bump(): void {
    this.revision += 1;
  }

  /** @emoji 👁️ Reads persisted hide/lock flags for an entity id from metadata. */
  getEntityFlags(id: string): SpatialEntityFlags {
    return this.metadata.getEntityFlags(id);
  }

  /** @emoji 👁️ Sets one persisted hide/lock flag on an entity id. */
  setEntityFlag(id: string, flag: SpatialEntityFlagKey, value: boolean): void {
    this.metadata.setEntityFlag(id, flag, value);
  }
}

/** @emoji 🗑️ Removes object rows by id; geometry primitives stay intact so linked topology remains valid. */
export function deleteObjectsFromModel(model: Model, objectIds: readonly string[]): readonly string[] {
  const removed: string[] = [];
  for (const id of objectIds) {
    if (!model.objects[id]) continue;
    delete model.objects[id];
    removed.push(id);
  }
  if (removed.length > 0) model.bump();
  return removed;
}

/** @emoji 🪪️ Object ids from selection eligible for deletion (excludes geometry primitives). */
export function deletableObjectIdsFromSelection(selection: readonly SelectionTarget[]): readonly string[] {
  const ids: string[] = [];
  const seen = new Set<string>();
  for (const target of selection) {
    if (target.kind !== "object" || seen.has(target.id)) continue;
    seen.add(target.id);
    ids.push(target.id);
  }
  return ids;
}

/** @emoji #⃣ Stable FNV-1a digest for canonical geometry fingerprints. */
export function fnv1aHex(input: string): string {
  let h = 0x811c9dc5;
  for (let i = 0; i < input.length; i++) {
    h ^= input.charCodeAt(i);
    h = Math.imul(h, 0x01000193);
  }
  return (h >>> 0).toString(16).padStart(8, "0");
}

/** @emoji #⃣ Opaque content hash for a hashed primitive (vertex position fingerprint). */
export type GeometryPrimitiveHash = string & { readonly __brand: "GeometryPrimitiveHash" };

/** @emoji #⃣ Quantizes a coordinate for stable hashing. */
export function quantizeCoord(value: number, decimals = 9): number {
  const factor = 10 ** decimals;
  return Math.round(value * factor) / factor;
}

/** @emoji #⃣ Hashes canonical primitive payload bytes. */
export function hashPrimitivePayload(kind: string, payload: string): GeometryPrimitiveHash {
  return `${kind[0]}:${fnv1aHex(payload)}` as GeometryPrimitiveHash;
}

/** @emoji #⃣ Hashes a vertex position (`cad/AGENTS.md` primitive hashing). */
export function hashVertexPosition(position: Vec3): GeometryPrimitiveHash {
  const q = position.map((c) => quantizeCoord(c)) as Vec3;
  return hashPrimitivePayload("vertex", `${q[0]},${q[1]},${q[2]}`);
}

/** @emoji #⃣ Hashes an edge by vertex ids and optional curve. */
export function hashEdgeRecord(edge: EdgeRecord, vertices: Record<string, VertexRecord>): GeometryPrimitiveHash {
  const positions = edge.vertexIds.map((vid) => {
    const p = vertices[vid]?.position ?? ([0, 0, 0] as Vec3);
    return p.map((c) => quantizeCoord(c)).join(",");
  });
  const curveKey = edge.curve ? JSON.stringify(edge.curve) : "line";
  return hashPrimitivePayload("edge", `${edge.vertexIds.join(",")}|${curveKey}|${positions.join(";")}`);
}

/** @emoji #⃣ Hashes a wire by sorted edge ids. */
export function hashWireRecord(wire: WireRecord): GeometryPrimitiveHash {
  return hashPrimitivePayload("wire", [...wire.edgeIds].sort().join(","));
}

/** @emoji #⃣ Hashes a face by sorted wire ids and surface kind. */
export function hashFaceRecord(face: FaceRecord): GeometryPrimitiveHash {
  const surfaceKey = face.surface ? JSON.stringify(face.surface) : "none";
  return hashPrimitivePayload("face", `${[...face.wireIds].sort().join(",")}|${surfaceKey}`);
}

/** @emoji #⃣ Hashes a shell by sorted face ids. */
export function hashShellRecord(shell: ShellRecord): GeometryPrimitiveHash {
  return hashPrimitivePayload("shell", [...shell.faceIds].sort().join(","));
}

/** @emoji #⃣ Hashes a solid by sorted shell ids and solid primitive. */
export function hashSolidRecord(solid: SolidRecord): GeometryPrimitiveHash {
  const primitiveKey = solid.solid ? JSON.stringify(solid.solid) : "none";
  return hashPrimitivePayload("solid", `${[...solid.shellIds].sort().join(",")}|${primitiveKey}`);
}

/** @emoji #⃣ Hashes an anchor position and attachment. */
export function hashAnchorRecord(anchor: AnchorRecord): GeometryPrimitiveHash {
  return hashPrimitivePayload("anchor", `${anchor.position.map((c) => quantizeCoord(c)).join(",")}|${JSON.stringify(anchor.attachment)}`);
}

/** @emoji #⃣ Per-primitive hashes for one model (`ModelSpace` geometry fingerprint). */
export type ModelPrimitiveHashes = Readonly<Partial<Record<KernelTopologyKind, Readonly<Record<string, GeometryPrimitiveHash>>>>>;

/** @emoji #⃣ Maps primitive tables on `model` to content hashes (every vertex and primitive). */
export function hashModelPrimitives(model: Model): ModelPrimitiveHashes {
  const out: Partial<Record<KernelTopologyKind, Record<string, GeometryPrimitiveHash>>> = {};
  const put = (kind: KernelTopologyKind, id: string, hash: GeometryPrimitiveHash): void => {
    out[kind] ??= {};
    out[kind]![id] = hash;
  };
  for (const [id, row] of Object.entries(model.anchors)) put("anchor", id, hashAnchorRecord(row));
  for (const [id, row] of Object.entries(model.vertices)) put("vertex", id, hashVertexPosition(row.position));
  for (const [id, row] of Object.entries(model.edges)) put("edge", id, hashEdgeRecord(row, model.vertices));
  for (const [id, row] of Object.entries(model.wires)) put("wire", id, hashWireRecord(row));
  for (const [id, row] of Object.entries(model.faces)) put("face", id, hashFaceRecord(row));
  for (const [id, row] of Object.entries(model.shells)) put("shell", id, hashShellRecord(row));
  for (const [id, row] of Object.entries(model.solids)) put("solid", id, hashSolidRecord(row));
  return out;
}

/** @emoji #⃣ Maps every model vertex id to its position hash. */
export function hashModelVertices(model: Model): Readonly<Record<string, GeometryPrimitiveHash>> {
  return hashModelPrimitives(model).vertex ?? {};
}

/** @emoji 🗺️ Serializable model space (`spatial.modelspace/v1`). */
export interface ModelSpaceJson {
  readonly schema: "spatial.modelspace";
  readonly revision: number;
  readonly models: readonly { readonly id: string; readonly model: ModelJson }[];
}

/** @emoji 🌌️ Container for linked models; geometry vertices are hashed per model. */
export class ModelSpace {
  revision = 0;
  models: Record<string, Model> = {};

  /** @emoji 🔗️ Registers or replaces a linked model. */
  link(modelId: string, model: Model): void {
    this.models[modelId] = model;
    this.bump();
  }

  /** @emoji ✂️ Removes a linked model. */
  unlink(modelId: string): void {
    if (!(modelId in this.models)) return;
    delete this.models[modelId];
    this.bump();
  }

  /** @emoji 🔍️ Returns a linked model or `null`. */
  get(modelId: string): Model | null {
    return this.models[modelId] ?? null;
  }

  /** @emoji #⃣ Vertex position hashes keyed by linked model id. */
  vertexHashesByModel(): Readonly<Record<string, Readonly<Record<string, GeometryPrimitiveHash>>>> {
    const out: Record<string, Readonly<Record<string, GeometryPrimitiveHash>>> = {};
    for (const [modelId, model] of Object.entries(this.models)) out[modelId] = hashModelVertices(model);
    return out;
  }

  /** @emoji #⃣ Full primitive hashes keyed by linked model id. */
  geometryHashesByModel(): Readonly<Record<string, ModelPrimitiveHashes>> {
    const out: Record<string, ModelPrimitiveHashes> = {};
    for (const [modelId, model] of Object.entries(this.models)) out[modelId] = hashModelPrimitives(model);
    return out;
  }

  /** @emoji 🔄️ Transfers a transformation from a linked source model into a new linked target model. */
  transfer(linkedSourceId: string, linkedTargetId: string, spec: TransformationSpec, preview: SpatialPreviewKernel): Model {
    const source = this.models[linkedSourceId];
    if (!source) throw new Error(`ModelSpace: unknown source model ${linkedSourceId}`);
    const target = applyTransformation(spec, source, preview);
    this.link(linkedTargetId, target);
    return target;
  }

  /** @emoji 🧭️ Serializes linked models (stable id order). */
  toJSON(): ModelSpaceJson {
    const models = Object.keys(this.models)
      .sort()
      .map((id) => ({ id, model: this.models[id]!.toJSON() }));
    return { schema: "spatial.modelspace", revision: this.revision, models };
  }

  /** @emoji 🧭️ Hydrates from `ModelSpaceJson`. */
  static fromJSON(json: ModelSpaceJson): ModelSpace {
    const space = new ModelSpace();
    space.revision = json.revision;
    for (const row of json.models ?? []) space.models[row.id] = Model.fromJSON(row.model);
    return space;
  }

  bump(): void {
    this.revision += 1;
  }
}


/** @emoji 🧭️ Reads `name` from metadata, geometry records, or model objects. */
export function readModelEntityProperty(
  model: Model,
  meta: AttributeTable | undefined,
  kind: ModelEntityKind,
  id: string,
  name: string,
  opts?: {
    readonly activeModelDefinitionId?: string | null;
    readonly preview?: SpatialPreviewKernel;
  },
): unknown {
  const bag = meta?.get(id);
  if (bag && name in bag) return (bag as Record<string, unknown>)[name];
  switch (kind) {
    case "anchor": {
      const anchor = model.anchors[id];
      if (!anchor) return undefined;
      if (name === "position") return opts?.preview?.evaluateAnchorPosition(model, anchor) ?? anchor.position;
      return (anchor as unknown as Record<string, unknown>)[name];
    }
    case "vertex":
      return (model.vertices[id] as unknown as Record<string, unknown> | undefined)?.[name];
    case "edge":
      return (model.edges[id] as unknown as Record<string, unknown> | undefined)?.[name];
    case "wire":
      return (model.wires[id] as unknown as Record<string, unknown> | undefined)?.[name];
    case "face":
      return (model.faces[id] as unknown as Record<string, unknown> | undefined)?.[name];
    case "shell":
      return (model.shells[id] as unknown as Record<string, unknown> | undefined)?.[name];
    case "solid":
      return (model.solids[id] as unknown as Record<string, unknown> | undefined)?.[name];
    case "object": {
      const hit = model.objects[id];
      if (!hit) return undefined;
      if (name === "id") return id;
      if (name === "typology") return hit.typology;
      if (name === "primitives") return hit.primitives;
      return (hit.attributes as Record<string, unknown> | undefined)?.[name];
    }
    case "geometry":
      return model.solids[id] ? id : undefined;
    case "attribute":
      return meta?.get(id)?.[name];
    default:
      return undefined;
  }
}

/** @emoji 🧾️ Parses `spatial.model/v1` JSON into a model or returns `null`. */
export function parseModelJson(raw: unknown): Model | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.model") return null;
  const geoKeys = ["anchors", "vertices", "edges", "wires", "faces", "shells", "solids"] as const;
  const geometry: Record<string, unknown> = r.geometry && typeof r.geometry === "object" ? { ...(r.geometry as Record<string, unknown>) } : {};
  if (!Array.isArray(geometry.solids) && Array.isArray((geometry as { cells?: unknown }).cells)) geometry.solids = (geometry as { cells: unknown[] }).cells;
  for (const k of geoKeys) {
    if (!Array.isArray(geometry[k]) && Array.isArray(r[k])) geometry[k] = r[k];
    if (!Array.isArray(geometry[k])) geometry[k] = [];
  }
  const json: ModelJson = {
    schema: "spatial.model",
    revision: typeof r.revision === "number" ? r.revision : 0,
    objects: Array.isArray(r.objects) ? (r.objects as SpatialObjectRecord[]) : [],
    geometry: geometry as KernelGeometryJson,
  };
  return Model.fromJSON(json);
}

/** @emoji 🧱️ Standalone primitive kinds allowed on typology objects (`cad/AGENTS.md`). */
export type TypologyPrimitiveKind = "anchor" | "solid" | "surface" | "curve";

/** @emoji 🧬️ Kernel-private topology entity kinds (faces, wires, … are not standalone primitives). */
export type KernelTopologyKind = "anchor" | "vertex" | "edge" | "wire" | "face" | "shell" | "solid";

/** @emoji 🏷️ Parsed model-definition manifest (`spatial.modelDefinition/v1` on disk). */
export interface ModelDefinitionManifest {
  readonly schema: "spatial.modelDefinition";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly kinds: readonly string[];
  readonly default?: boolean;
  readonly baseObjectTypology?: string;
  readonly kernelTypologies?: Readonly<Partial<Record<KernelTopologyKind, string>>>;
}

/** @emoji 🧭️ Default geometry-edit model definition id (manifest `default: true`). */
export function defaultModelDefinitionId(): string {
  if (defaultModelDefinitionIdCache.current) return defaultModelDefinitionIdCache.current;
  const manifests = listModelDefinitionManifests();
  const row = manifests.find((m) => m.default) ?? manifests[0];
  defaultModelDefinitionIdCache.current = row?.id ?? "";
  return defaultModelDefinitionIdCache.current;
}

/** @emoji 🧭️ True when the active definition is geometry edit (`ModelDefinition`) rather than typology objects. */
export function isShapeModelDefinition(modelDefinitionId: string | null | undefined): boolean {
  if (modelDefinitionId == null) return true;
  return kernelTypologyIds(modelDefinitionId) !== null;
}

/** @emoji 🪪️ Kernel typology ids per primitive kind on a model-definition manifest. */
export function kernelTypologyIds(modelDefinitionId: string): Readonly<Partial<Record<KernelTopologyKind, string>>> | null {
  const manifest = listModelDefinitionManifests().find((row) => row.id === modelDefinitionId);
  const map = manifest?.kernelTypologies;
  if (!map || Object.keys(map).length === 0) return null;
  return map;
}

/** @emoji 🧾️ Parses a model-definition manifest JSON or returns `null`. */
export function parseModelDefinitionManifest(raw: unknown): ModelDefinitionManifest | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.modelDefinition" && r.schema !== "spatial.extension") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  if (!Array.isArray(r.kinds) || r.kinds.length === 0) return null;
  const kernelTypologies = parseKernelTypologies(r.kernelTypologies);
  return {
    schema: "spatial.modelDefinition",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    kinds: r.kinds as string[],
    ...(r.default === true ? { default: true } : {}),
    ...(typeof r.baseObjectTypology === "string" && r.baseObjectTypology.length > 0 ? { baseObjectTypology: r.baseObjectTypology } : {}),
    ...(kernelTypologies ? { kernelTypologies } : {}),
  };
}

function parseKernelTypologies(raw: unknown): Readonly<Partial<Record<KernelTopologyKind, string>>> | undefined {
  if (!raw || typeof raw !== "object" || Array.isArray(raw)) return undefined;
  const allowed = new Set<KernelTopologyKind>(["anchor", "vertex", "edge", "wire", "face", "shell", "solid"]);
  const out: Partial<Record<KernelTopologyKind, string>> = {};
  for (const [key, value] of Object.entries(raw as Record<string, unknown>)) {
    if (!allowed.has(key as KernelTopologyKind) || typeof value !== "string" || value.length === 0) continue;
    out[key as KernelTopologyKind] = value;
  }
  return Object.keys(out).length > 0 ? out : undefined;
}

/** @emoji 📚️ Lists model-definition manifests under spatial/asset/modelDefinition. */
export function listModelDefinitionManifests(): readonly ModelDefinitionManifest[] {
  return modelDefinitionManifestCatalog()
    .map((raw) => parseModelDefinitionManifest(raw))
    .filter((m): m is ModelDefinitionManifest => m !== null);
}

/** @emoji 🎨️ Surface pattern kind for typology display styling. */
export type TypologyStylePatternKind = "none" | "hatch" | "crosshatch" | "dots";

/** @emoji 🎨️ Authored surface pattern on a typology. */
export interface TypologyStylePatternSpec {
  readonly kind: TypologyStylePatternKind;
  readonly direction?: number;
  readonly spacing?: number;
  readonly lineWidth?: number;
  readonly color?: string;
}

/** @emoji 🎨️ Optional authored display style on a typology asset. */
export interface TypologyStyleSpec {
  readonly color?: string;
  readonly edgeColor?: string;
  readonly opacity?: number;
  readonly pattern?: TypologyStylePatternSpec;
}

/** @emoji 🎨️ Fully resolved display style for one typology (auto fallback + authored overrides). */
export interface ResolvedTypologyStyle {
  readonly color: string;
  readonly edgeColor: string;
  readonly opacity: number;
  readonly pattern: Required<Pick<TypologyStylePatternSpec, "kind" | "direction" | "spacing" | "lineWidth" | "color">>;
}

/** @emoji 🏷️ Parsed typology asset (`spatial.typology/v1`). */
export interface TypologySpec {
  readonly schema: "spatial.typology";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly primitiveKinds: readonly TypologyPrimitiveKind[];
  readonly actions: readonly string[];
  readonly interactions: readonly string[];
  readonly properties?: readonly string[];
  readonly attributes?: readonly string[];
  readonly style?: TypologyStyleSpec;
}

/** @emoji 🧭️ Infers default `primitiveKinds` from a shipped typology id when the asset omits the field. */
export function inferTypologyPrimitiveKinds(typology: string): readonly TypologyPrimitiveKind[] {
  const id = typology.toLowerCase();
  if (id.includes(".selection.") || id.includes(".command.")) return [];
  if (id.includes(".measure.") && id.includes("volume")) return [];
  if (id.includes(".entity.") || id.includes("create-anchor")) return ["anchor"];
  if (id.includes(".measure.")) return ["anchor"];
  if (id.includes(".curve.") || id.includes(".linefem") || id.includes("lineelement")) return ["curve"];
  if (id.includes(".surface.") || id.includes("surfacefem") || id.includes("surfaceelement")) return ["surface"];
  if (id.includes(".primitive.") || id.includes(".solid.") || id.includes("solidelement")) return ["solid"];
  if (id.includes(".feature.extrude")) return ["solid"];
  if (id.includes(".feature.offset")) return ["surface", "solid"];
  if (id.includes(".transform.") || id.includes(".edit.")) return ["solid", "surface", "curve"];
  if (id.includes(".beam") || id.includes(".railing")) return ["curve"];
  if (id.includes(".column") || id.includes(".stair") || id.includes(".hull")) return ["solid"];
  if (id.includes("wall") || id.includes("slab") || id.includes(".roof") || id.includes("baseplate") || id.includes("ceiling") || id.includes("foundation") || id.includes(".door") || id.includes("window")) {
    return ["surface"];
  }
  return ["solid"];
}

function parseTypologyPrimitiveKinds(raw: unknown, typology: string): readonly TypologyPrimitiveKind[] {
  if (!Array.isArray(raw) || raw.length === 0) return inferTypologyPrimitiveKinds(typology);
  const allowed = new Set<TypologyPrimitiveKind>(["anchor", "solid", "surface", "curve"]);
  const kinds: TypologyPrimitiveKind[] = [];
  for (const entry of raw) {
    if (typeof entry !== "string") continue;
    const k = entry as TypologyPrimitiveKind;
    if (allowed.has(k)) kinds.push(k);
  }
  return kinds.length ? kinds : inferTypologyPrimitiveKinds(typology);
}

/** @emoji 🧭️ Maps a standalone typology primitive kind to the kernel entity kind used for picking. */
export function typologyPrimitiveToEntityKind(kind: TypologyPrimitiveKind): ModelEntityKind {
  if (kind === "surface") return "face";
  if (kind === "curve") return "wire";
  return kind;
}

const TYPOLOGY_STYLE_PATTERN_KINDS = new Set<TypologyStylePatternKind>(["none", "hatch", "crosshatch", "dots"]);

function parseTypologyStylePatternSpec(raw: unknown): TypologyStylePatternSpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  const kind = r.kind;
  if (typeof kind !== "string" || !TYPOLOGY_STYLE_PATTERN_KINDS.has(kind as TypologyStylePatternKind)) return null;
  return {
    kind: kind as TypologyStylePatternKind,
    direction: typeof r.direction === "number" ? r.direction : undefined,
    spacing: typeof r.spacing === "number" ? r.spacing : undefined,
    lineWidth: typeof r.lineWidth === "number" ? r.lineWidth : undefined,
    color: typeof r.color === "string" ? r.color : undefined,
  };
}

function parseTypologyStyleSpec(raw: unknown): TypologyStyleSpec | undefined {
  if (!raw || typeof raw !== "object") return undefined;
  const r = raw as Record<string, unknown>;
  const pattern = r.pattern !== undefined ? parseTypologyStylePatternSpec(r.pattern) : undefined;
  if (r.pattern !== undefined && !pattern) return undefined;
  return {
    color: typeof r.color === "string" ? r.color : undefined,
    edgeColor: typeof r.edgeColor === "string" ? r.edgeColor : undefined,
    opacity: typeof r.opacity === "number" ? r.opacity : undefined,
    pattern: pattern ?? undefined,
  };
}

/** @emoji 🧾️ Parses `spatial.typology/v1` JSON or returns `null`. */
export function parseTypologySpec(raw: unknown): TypologySpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.typology") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  const actions = Array.isArray(r.actions) ? r.actions.filter((row): row is string => typeof row === "string") : [];
  const interactions = Array.isArray(r.interactions) ? r.interactions.filter((row): row is string => typeof row === "string") : [];
  return {
    schema: "spatial.typology",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    primitiveKinds: parseTypologyPrimitiveKinds(r.primitiveKinds, r.id),
    actions,
    interactions,
    properties: Array.isArray(r.properties) ? (r.properties as string[]) : undefined,
    attributes: Array.isArray(r.attributes) ? (r.attributes as string[]) : undefined,
    style: parseTypologyStyleSpec(r.style),
  };
}

// #region 🎨️TypologyStyle
function hashTypologyId(typology: string): number {
  let h = 2166136261;
  for (let i = 0; i < typology.length; i++) {
    h ^= typology.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  return h >>> 0;
}

function hslToHex(h: number, s: number, l: number): string {
  const hue = ((h % 360) + 360) % 360;
  const sat = Math.min(1, Math.max(0, s));
  const lit = Math.min(1, Math.max(0, l));
  const c = (1 - Math.abs(2 * lit - 1)) * sat;
  const x = c * (1 - Math.abs(((hue / 60) % 2) - 1));
  const m = lit - c / 2;
  let r = 0;
  let g = 0;
  let b = 0;
  if (hue < 60) {
    r = c;
    g = x;
  } else if (hue < 120) {
    r = x;
    g = c;
  } else if (hue < 180) {
    g = c;
    b = x;
  } else if (hue < 240) {
    g = x;
    b = c;
  } else if (hue < 300) {
    r = x;
    b = c;
  } else {
    r = c;
    b = x;
  }
  const toByte = (v: number) =>
    Math.round((v + m) * 255)
      .toString(16)
      .padStart(2, "0");
  return `#${toByte(r)}${toByte(g)}${toByte(b)}`;
}

function darkenHexColor(hex: string, amount = 0.28): string {
  const raw = hex.replace("#", "");
  if (raw.length !== 6) return hex;
  const r = Number.parseInt(raw.slice(0, 2), 16);
  const g = Number.parseInt(raw.slice(2, 4), 16);
  const b = Number.parseInt(raw.slice(4, 6), 16);
  const scale = 1 - amount;
  const toByte = (v: number) =>
    Math.round(v * scale)
      .toString(16)
      .padStart(2, "0");
  return `#${toByte(r)}${toByte(g)}${toByte(b)}`;
}

function autoTypologyStyle(typology: string): ResolvedTypologyStyle {
  const hash = hashTypologyId(typology);
  const hue = (hash * 137.508) % 360;
  const color = hslToHex(hue, 0.58, 0.52);
  return {
    color,
    edgeColor: darkenHexColor(color, 0.32),
    opacity: 0.72,
    pattern: { kind: "none", direction: 0, spacing: 0.35, lineWidth: 0.03, color: darkenHexColor(color, 0.18) },
  };
}

function mergeTypologyStyle(typology: string, authored?: TypologyStyleSpec): ResolvedTypologyStyle {
  const base = autoTypologyStyle(typology);
  const pattern = authored?.pattern;
  const fill = authored?.color ?? base.color;
  return {
    color: fill,
    edgeColor: authored?.edgeColor ?? darkenHexColor(fill, 0.32),
    opacity: authored?.opacity ?? base.opacity,
    pattern: {
      kind: pattern?.kind ?? base.pattern.kind,
      direction: pattern?.direction ?? base.pattern.direction,
      spacing: pattern?.spacing ?? base.pattern.spacing,
      lineWidth: pattern?.lineWidth ?? base.pattern.lineWidth,
      color: pattern?.color ?? darkenHexColor(fill, 0.18),
    },
  };
}

/** @emoji 🎨️ Stable cache key for renderer material/pattern reuse. */
export function typologyStyleCacheKey(style: ResolvedTypologyStyle): string {
  const p = style.pattern;
  return `${style.color}|${style.edgeColor}|${style.opacity}|${p.kind}|${p.direction}|${p.spacing}|${p.lineWidth}|${p.color}`;
}

/** @emoji 🎨️ Resolves display style for a typology (deterministic auto fallback + optional asset override). */
export function resolveTypologyStyle(typology: string): ResolvedTypologyStyle {
  if (typologyStyleCache.current?.has(typology)) return typologyStyleCache.current.get(typology)!;
  const authored = loadTypology(typology)?.style;
  const resolved = mergeTypologyStyle(typology, authored);
  if (!typologyStyleCache.current) typologyStyleCache.current = new Map();
  typologyStyleCache.current.set(typology, resolved);
  return resolved;
}
// #endregion 🎨️TypologyStyle

function shippedTypologyCatalog(): readonly TypologySpec[] {
  return dedupeDefinitionCatalog(
    modelDefinitionTypologyCatalog()
      .map((raw) => parseTypologySpec(raw))
      .filter((spec): spec is TypologySpec => spec !== null),
  );
}

/** @emoji 📚️ Lists typologies from shipped spatial/asset/modelDefinition assets. */
export function listModelDefinitionTypologies(): readonly TypologySpec[] {
  return shippedTypologyCatalog();
}

/** @emoji 📚️ Loads a model-definition typology by stable `id`. */
export function loadTypology(typology: string): TypologySpec | null {
  return shippedTypologyCatalog().find((t) => t.id === typology) ?? null;
}

/** @emoji 📚️ Resolves the typology whose `interactions` list includes `interactionId`. */
export function typologyForInteraction(interactionId: string): TypologySpec | null {
  return shippedTypologyCatalog().find((t) => t.interactions.some((id) => id === interactionId)) ?? null;
}

/** @emoji 🏷️ Parsed attribute definition (`spatial.attribute/v1`). */
export interface AttributeDefinitionSpec {
  readonly schema: "spatial.attribute";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly field: string;
  readonly targets: readonly string[];
  readonly value: unknown;
  readonly geometrySelector?: { readonly kinds: readonly string[] };
}

/** @emoji 🧾️ Parses `spatial.attribute/v1` JSON or returns `null`. */
export function parseAttributeDefinitionSpec(raw: unknown): AttributeDefinitionSpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.attribute") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  if (typeof r.field !== "string" || !Array.isArray(r.targets) || r.targets.length === 0) return null;
  if (!("value" in r)) return null;
  const selector = r.geometrySelector;
  const geometrySelector = selector && typeof selector === "object" && Array.isArray((selector as { kinds?: unknown }).kinds) ? { kinds: (selector as { kinds: string[] }).kinds } : undefined;
  return {
    schema: "spatial.attribute",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    field: r.field,
    targets: r.targets as string[],
    value: r.value,
    geometrySelector,
  };
}

/** @emoji 🏷️ Parsed property definition (`spatial.property/v1`). */
export interface PropertyDefinitionSpec {
  readonly schema: "spatial.property";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly unit?: string;
  readonly sources?: Readonly<Record<string, unknown>>;
  readonly output?: Readonly<Record<string, unknown>>;
}

/** @emoji 🧾️ Parses `spatial.property/v1` JSON or returns `null`. */
export function parsePropertyDefinitionSpec(raw: unknown): PropertyDefinitionSpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.property") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  return {
    schema: "spatial.property",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    unit: typeof r.unit === "string" ? r.unit : undefined,
    sources: r.sources && typeof r.sources === "object" ? (r.sources as Record<string, unknown>) : undefined,
    output: r.output && typeof r.output === "object" ? (r.output as Record<string, unknown>) : undefined,
  };
}

function dedupeDefinitionCatalog<T extends { readonly id: string }>(rows: readonly T[]): readonly T[] {
  const seen = new Set<string>();
  const out: T[] = [];
  for (const row of rows) {
    if (seen.has(row.id)) continue;
    seen.add(row.id);
    out.push(row);
  }
  return out;
}

function shippedAttributeDefinitionCatalog(): readonly AttributeDefinitionSpec[] {
  return dedupeDefinitionCatalog(
    modelDefinitionAttributeCatalog()
      .map((raw) => parseAttributeDefinitionSpec(raw))
      .filter((spec): spec is AttributeDefinitionSpec => spec !== null),
  );
}

function shippedPropertyDefinitionCatalog(): readonly PropertyDefinitionSpec[] {
  return dedupeDefinitionCatalog(
    modelDefinitionPropertyCatalog()
      .map((raw) => parsePropertyDefinitionSpec(raw))
      .filter((spec): spec is PropertyDefinitionSpec => spec !== null),
  );
}

/** @emoji 📚️ Lists attribute definitions from model-definition assets. */
export function listModelDefinitionAttributeDefinitions(): readonly AttributeDefinitionSpec[] {
  return shippedAttributeDefinitionCatalog();
}

/** @emoji 📚️ Lists property definitions from model-definition assets. */
export function listModelDefinitionPropertyDefinitions(): readonly PropertyDefinitionSpec[] {
  return shippedPropertyDefinitionCatalog();
}

/** @emoji 📚️ Loads an attribute definition by stable `id`. */
export function loadAttributeDefinition(attributeId: string): AttributeDefinitionSpec | null {
  return shippedAttributeDefinitionCatalog().find((row) => row.id === attributeId) ?? null;
}

/** @emoji 📚️ Loads a property definition by stable `id`. */
export function loadPropertyDefinition(propertyId: string): PropertyDefinitionSpec | null {
  return shippedPropertyDefinitionCatalog().find((row) => row.id === propertyId) ?? null;
}

/** @emoji 🧭️ Resolves the kernel topology kind referenced by one primitive ref. */
export function resolveKernelTopologyKind(model: Model, primitiveRef: string): KernelTopologyKind | null {
  if (model.anchors[primitiveRef]) return "anchor";
  if (model.vertices[primitiveRef]) return "vertex";
  if (model.edges[primitiveRef]) return "edge";
  if (model.wires[primitiveRef]) return "wire";
  if (model.faces[primitiveRef]) return "face";
  if (model.shells[primitiveRef]) return "shell";
  if (model.solids[primitiveRef]) return "solid";
  return null;
}

/** @emoji 🧭️ Resolves the standalone primitive kind referenced by one primitive ref. */
export function resolveStandalonePrimitiveKind(model: Model, primitiveRef: string): TypologyPrimitiveKind | null {
  if (model.anchors[primitiveRef]) return "anchor";
  if (model.solids[primitiveRef]) return "solid";
  if (model.faces[primitiveRef]) return "surface";
  if (model.wires[primitiveRef] || model.edges[primitiveRef]) return "curve";
  return null;
}

/** @emoji 🧭️ Resolves the kernel topology kind referenced by one primitive ref. */
export const resolvePrimitiveRefKind = resolveKernelTopologyKind;

/** @emoji 🌳️ Nested primitive node under an object primitive (`solid` → `shell` → `face` → `wire` → `edge` → `vertex`). */
export interface ModelPrimitiveDocumentNode {
  readonly kind: KernelTopologyKind;
  readonly id: string;
  readonly children: readonly ModelPrimitiveDocumentNode[];
}

function sortedPrimitiveChildIds(ids: readonly string[]): string[] {
  return [...ids].sort((a, b) => a.localeCompare(b));
}

function buildModelPrimitiveDocumentNode(model: Model, kind: KernelTopologyKind, id: string): ModelPrimitiveDocumentNode | null {
  const children: ModelPrimitiveDocumentNode[] = [];
  switch (kind) {
    case "solid": {
      const solid = model.solids[id];
      if (!solid) return null;
      for (const shellId of sortedPrimitiveChildIds(solid.shellIds)) {
        const child = buildModelPrimitiveDocumentNode(model, "shell", shellId);
        if (child) children.push(child);
      }
      break;
    }
    case "shell": {
      const shell = model.shells[id];
      if (!shell) return null;
      for (const faceId of sortedPrimitiveChildIds(shell.faceIds)) {
        const child = buildModelPrimitiveDocumentNode(model, "face", faceId);
        if (child) children.push(child);
      }
      break;
    }
    case "face": {
      const face = model.faces[id];
      if (!face) return null;
      for (const wireId of sortedPrimitiveChildIds(face.wireIds)) {
        const child = buildModelPrimitiveDocumentNode(model, "wire", wireId);
        if (child) children.push(child);
      }
      break;
    }
    case "wire": {
      const wire = model.wires[id];
      if (!wire) return null;
      for (const edgeId of sortedPrimitiveChildIds(wire.edgeIds)) {
        const child = buildModelPrimitiveDocumentNode(model, "edge", edgeId);
        if (child) children.push(child);
      }
      break;
    }
    case "edge": {
      const edge = model.edges[id];
      if (!edge) return null;
      for (const vertexId of sortedPrimitiveChildIds(edge.vertexIds)) {
        const child = buildModelPrimitiveDocumentNode(model, "vertex", vertexId);
        if (child) children.push(child);
      }
      break;
    }
    case "vertex":
      if (!model.vertices[id]) return null;
      break;
    case "anchor":
      if (!model.anchors[id]) return null;
      break;
  }
  return { kind, id, children };
}

/** @emoji 🌳️ Builds nested primitive document under one object primitive ref in `model`. */
export function buildModelPrimitiveDocument(model: Model, primitiveRef: string): ModelPrimitiveDocumentNode | null {
  const kind = resolvePrimitiveRefKind(model, primitiveRef);
  if (!kind) return null;
  return buildModelPrimitiveDocumentNode(model, kind, primitiveRef);
}

/** @emoji ✅️ Whether `typology` allows objects whose geometry resolves to `primitiveKind`. */
export function typologyAllowsPrimitiveKind(typology: TypologySpec, primitiveKind: TypologyPrimitiveKind): boolean {
  return typology.primitiveKinds.includes(primitiveKind);
}

/** @emoji ✅️ Whether `object` on `model` satisfies its typology `primitiveKinds`. */
export function objectMatchesTypologyPrimitives(model: Model, object: SpatialObjectRecord): boolean {
  const typology = loadTypology(object.typology);
  if (!typology || typology.primitiveKinds.length === 0) return false;
  const primitiveKinds = objectPrimitiveRefs(object)
    .map((primitiveRef) => resolveStandalonePrimitiveKind(model, primitiveRef))
    .filter((kind): kind is TypologyPrimitiveKind => kind !== null);
  return primitiveKinds.length > 0 && primitiveKinds.every((kind) => typologyAllowsPrimitiveKind(typology, kind));
}

/** @emoji 🧭️ Typology → entity kind map for one model definition (`ModelDefinition` includes kernel typology ids; AEC typologies map to `object`). */
export function buildTypologyToEntityKindMapForModelDefinition(modelDefinitionId: string): Readonly<Record<string, ModelEntityKind>> {
  const out: Record<string, ModelEntityKind> = {};
  const kernelTypologies = kernelTypologyIds(modelDefinitionId);
  if (kernelTypologies) {
    for (const [kind, id] of Object.entries(kernelTypologies)) {
      if (typeof id === "string" && id.length > 0) out[id] = kind as ModelEntityKind;
    }
    for (const spec of listTypologiesForModelDefinition(modelDefinitionId)) {
      if (spec.primitiveKinds.length !== 1) continue;
      const kind = typologyPrimitiveToEntityKind(spec.primitiveKinds[0]!);
      if (kind === "anchor" && !spec.id.includes("entity") && !spec.id.includes("measure")) continue;
      out[spec.id] = kind;
    }
    return out;
  }
  for (const spec of listTypologiesForModelDefinition(modelDefinitionId)) out[spec.id] = "object";
  return out;
}

/** @emoji ✅️ Whether a property definition applies to `object` on `model`. */
export function propertyDefinitionAppliesToObject(defn: PropertyDefinitionSpec, object: SpatialObjectRecord): boolean {
  const typologies = defn.sources?.typologies;
  if (Array.isArray(typologies) && typologies.length > 0) return typologies.includes(object.typology);
  return true;
}

/** @emoji 📐️ Context passed to registered property computers. */
export interface PropertyComputeContext {
  readonly model: Model;
  readonly kernel: SpatialKernel;
  readonly object: SpatialObjectRecord;
  readonly defn: PropertyDefinitionSpec;
}

/** @emoji 📐️ Registered property computer for one property definition id. */
export type PropertyComputer = (ctx: PropertyComputeContext) => Promise<Record<string, unknown>>;

const propertyComputers = ephemeralMap<string, PropertyComputer>("s.plugins.cad.modules.core.component.ts.propertyComputers");

/** @emoji 📐️ Registers a TypeScript computer for one property definition id. */
export function registerPropertyComputer(propertyId: string, computer: PropertyComputer): void {
  propertyComputers.set(propertyId, computer);
}

/** @emoji 📐️ Derives property output for one model object from a property definition. */
export async function derivePropertyValue(defn: PropertyDefinitionSpec, ctx: { readonly model: Model; readonly kernel: SpatialKernel; readonly object: SpatialObjectRecord }): Promise<Record<string, unknown>> {
  if (!propertyDefinitionAppliesToObject(defn, ctx.object)) return {};
  const computer = propertyComputers.get(defn.id);
  if (computer) return computer({ ...ctx, defn });
  const output = defn.output ?? {};
  return { ...output };
}

/** @emoji 📚️ Property definitions for one model definition that apply to `object` on `model`. */
export function listApplicablePropertyDefinitionsForModelDefinition(modelDefinitionId: string, model: Model, object: SpatialObjectRecord): readonly PropertyDefinitionSpec[] {
  const scoped = new Set(listPropertyDefinitionsForModelDefinition(modelDefinitionId).map((row) => row.id));
  return shippedPropertyDefinitionCatalog().filter((defn) => scoped.has(defn.id) && propertyDefinitionAppliesToObject(defn, object));
}

// #region 📊️StatDefinitions
/** @emoji 📊️ Live stat output descriptor shipped with a model definition. */
export interface StatOutputSpec {
  readonly key: string;
  readonly label: string;
  readonly unit?: string;
  readonly format?: "integer" | "decimal" | "percent";
}

/** @emoji 📊️ Model-definition live stat declaration (`spatial.stat/v1`). */
export interface StatDefinitionSpec {
  readonly schema: "spatial.stat";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly scopes: readonly ("model" | "selection")[];
  readonly sources?: Readonly<{ readonly typologies?: readonly string[] }>;
  readonly outputs: readonly StatOutputSpec[];
}

/** @emoji 🧾️ Parses `spatial.stat/v1` JSON or returns `null`. */
export function parseStatDefinitionSpec(raw: unknown): StatDefinitionSpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.stat") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  if (!Array.isArray(r.outputs) || r.outputs.length === 0) return null;
  const outputs: StatOutputSpec[] = [];
  for (const row of r.outputs) {
    if (!row || typeof row !== "object") return null;
    const o = row as Record<string, unknown>;
    if (typeof o.key !== "string" || typeof o.label !== "string") return null;
    outputs.push({
      key: o.key,
      label: o.label,
      unit: typeof o.unit === "string" ? o.unit : undefined,
      format: o.format === "integer" || o.format === "decimal" || o.format === "percent" ? o.format : undefined,
    });
  }
  const scopesRaw = r.scopes;
  const scopes: ("model" | "selection")[] = Array.isArray(scopesRaw) && scopesRaw.length > 0 ? scopesRaw.filter((scope): scope is "model" | "selection" => scope === "model" || scope === "selection") : ["model", "selection"];
  if (scopes.length === 0) return null;
  const sourcesRaw = r.sources;
  const sources =
    sourcesRaw && typeof sourcesRaw === "object"
      ? {
          typologies: Array.isArray((sourcesRaw as { typologies?: unknown }).typologies) ? (sourcesRaw as { typologies: string[] }).typologies.filter((id): id is string => typeof id === "string") : undefined,
        }
      : undefined;
  return {
    schema: "spatial.stat",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    scopes,
    sources: sources?.typologies?.length ? sources : undefined,
    outputs,
  };
}

function shippedStatDefinitionCatalog(): readonly StatDefinitionSpec[] {
  return dedupeDefinitionCatalog(
    modelDefinitionStatCatalog()
      .map((raw) => parseStatDefinitionSpec(raw))
      .filter((spec): spec is StatDefinitionSpec => spec !== null),
  );
}

/** @emoji 📚️ Lists stat definitions from model-definition assets. */
export function listModelDefinitionStatDefinitions(): readonly StatDefinitionSpec[] {
  return shippedStatDefinitionCatalog();
}

/** @emoji 📚️ Loads a stat definition by stable `id`. */
export function loadStatDefinition(statId: string): StatDefinitionSpec | null {
  return shippedStatDefinitionCatalog().find((row) => row.id === statId) ?? null;
}

function statOwnerById(): ReadonlyMap<string, string> {
  if (statOwnerByIdCache.current) return statOwnerByIdCache.current;
  const map = new Map<string, string>();
  for (const [path, raw] of Object.entries(modelDefinitionAssetModules.current.statDefinitions)) {
    const owner = modelDefinitionIdFromAssetPath(path);
    const spec = parseStatDefinitionSpec(raw);
    if (!owner || !spec) continue;
    map.set(spec.id, owner);
  }
  statOwnerByIdCache.current = map;
  return map;
}

/** @emoji 🧭️ Stat definitions owned by a model definition. */
export function listStatDefinitionsForModelDefinition(modelDefinitionId: string): readonly StatDefinitionSpec[] {
  return shippedStatDefinitionCatalog().filter((row) => statOwnerById().get(row.id) === modelDefinitionId);
}

/** @emoji ✅️ Whether a stat definition supports one compute scope. */
export function statDefinitionAppliesToScope(defn: StatDefinitionSpec, scope: "model" | "selection"): boolean {
  return defn.scopes.includes(scope);
}

/** @emoji 🧾️ Formats one stat output value for display. */
export function formatStatOutputValue(value: number, format?: StatOutputSpec["format"]): string {
  if (!Number.isFinite(value)) return "—";
  if (format === "integer") return String(Math.round(value));
  if (format === "percent") return `${(value * 100).toFixed(1)}%`;
  if (format === "decimal") return value.toLocaleString(undefined, { maximumFractionDigits: 3 });
  return String(value);
}

/** @emoji ✅️ Whether `object` is included in stat sources for one definition. */
export function statDefinitionAppliesToObject(defn: StatDefinitionSpec, object: SpatialObjectRecord): boolean {
  const typologies = defn.sources?.typologies;
  if (Array.isArray(typologies) && typologies.length > 0) return typologies.includes(object.typology);
  return true;
}

/** @emoji 📊️ Context passed to registered stat computers. */
export interface StatComputeContext {
  readonly model: Model;
  readonly kernel: SpatialKernel;
  readonly modelDefinitionId: string;
  readonly scope: "model" | "selection";
  readonly objects: readonly SpatialObjectRecord[];
}

/** @emoji 📊️ Registered stat computer for one stat definition id. */
export type StatComputer = (ctx: StatComputeContext) => Promise<Record<string, number>>;

const statComputers = ephemeralMap<string, StatComputer>("s.plugins.cad.modules.core.component.ts.statComputers");

/** @emoji 📊️ Registers a TypeScript computer for one stat definition id. */
export function registerStatComputer(statId: string, computer: StatComputer): void {
  statComputers.set(statId, computer);
}

function zeroStatOutputs(defn: StatDefinitionSpec): Record<string, number> {
  const out: Record<string, number> = {};
  for (const row of defn.outputs) out[row.key] = 0;
  return out;
}

export function collectSolidRefsForObjects(model: Model, objects: readonly SpatialObjectRecord[]): string[] {
  const ids = new Set<string>();
  for (const object of objects) {
    for (const primitiveRef of objectPrimitiveRefs(object)) {
      if (resolvePrimitiveRefKind(model, primitiveRef) === "solid") ids.add(primitiveRef);
    }
  }
  return [...ids];
}

export function collectFaceRefsForObjects(model: Model, objects: readonly SpatialObjectRecord[]): string[] {
  const ids = new Set<string>();
  for (const object of objects) {
    for (const primitiveRef of objectPrimitiveRefs(object)) {
      const kind = resolveStandalonePrimitiveKind(model, primitiveRef);
      if (kind === "surface") {
        ids.add(primitiveRef);
        continue;
      }
      if (kind === "solid") {
        const solid = model.solids[primitiveRef];
        if (!solid) continue;
        for (const shellId of solid.shellIds) {
          const shell = model.shells[shellId];
          if (!shell) continue;
          for (const faceId of shell.faceIds) ids.add(faceId);
        }
      }
    }
  }
  return [...ids];
}

export function collectVertexPositionsForObjects(model: Model, objects: readonly SpatialObjectRecord[]): Vec3[] {
  const positions: Vec3[] = [];
  const seen = new Set<string>();
  const visitPrimitive = (primitiveRef: string): void => {
    if (seen.has(primitiveRef)) return;
    seen.add(primitiveRef);
    const kind = resolvePrimitiveRefKind(model, primitiveRef);
    if (kind === "vertex") {
      const vertex = model.vertices[primitiveRef];
      if (vertex) positions.push(vertex.position);
      return;
    }
    if (kind === "anchor") {
      const anchor = model.anchors[primitiveRef];
      if (anchor) positions.push(anchor.position);
      return;
    }
    const tree = buildModelPrimitiveDocument(model, primitiveRef);
    if (!tree) return;
    const walk = (node: ModelPrimitiveDocumentNode): void => {
      if (node.kind === "vertex") {
        const vertex = model.vertices[node.id];
        if (vertex) positions.push(vertex.position);
        return;
      }
      if (node.kind === "anchor") {
        const anchor = model.anchors[node.id];
        if (anchor) positions.push(anchor.position);
        return;
      }
      for (const child of node.children) walk(child);
    };
    walk(tree);
  };
  for (const object of objects) {
    for (const primitiveRef of objectPrimitiveRefs(object)) visitPrimitive(primitiveRef);
  }
  return positions;
}

export function bboxSizesFromPositions(positions: readonly Vec3[]): { readonly sizeX: number; readonly sizeY: number; readonly sizeZ: number } {
  if (positions.length === 0) return { sizeX: 0, sizeY: 0, sizeZ: 0 };
  let minX = positions[0]![0];
  let minY = positions[0]![1];
  let minZ = positions[0]![2];
  let maxX = minX;
  let maxY = minY;
  let maxZ = minZ;
  for (const [x, y, z] of positions) {
    minX = Math.min(minX, x);
    minY = Math.min(minY, y);
    minZ = Math.min(minZ, z);
    maxX = Math.max(maxX, x);
    maxY = Math.max(maxY, y);
    maxZ = Math.max(maxZ, z);
  }
  return { sizeX: maxX - minX, sizeY: maxY - minY, sizeZ: maxZ - minZ };
}

/** @emoji 📊️ Derives live stat output for one definition and scope. */
export async function computeStat(defn: StatDefinitionSpec, ctx: StatComputeContext): Promise<Record<string, number>> {
  if (!statDefinitionAppliesToScope(defn, ctx.scope)) return zeroStatOutputs(defn);
  const objects = ctx.objects.filter((object) => statDefinitionAppliesToObject(defn, object));
  if (objects.length === 0) return zeroStatOutputs(defn);
  const computer = statComputers.get(defn.id);
  if (!computer) return zeroStatOutputs(defn);
  const values = await computer({ ...ctx, objects });
  const out = zeroStatOutputs(defn);
  for (const row of defn.outputs) {
    const value = values[row.key];
    out[row.key] = typeof value === "number" && Number.isFinite(value) ? value : 0;
  }
  return out;
}

/** @emoji 📊️ Objects included when computing stats for one model definition scope. */
export function objectsForStatCompute(model: Model, modelDefinitionId: string, defn: StatDefinitionSpec, scope: "model" | "selection", selectionObjects: readonly SpatialObjectRecord[]): readonly SpatialObjectRecord[] {
  const typologyFilter = defn.sources?.typologies;
  let base: readonly SpatialObjectRecord[];
  if (scope === "selection") {
    base = selectionObjects.filter((object) => model.objects[object.id]);
  } else if (typologyFilter && typologyFilter.length > 0) {
    base = Object.values(model.objects).filter((object) => typologyFilter.includes(object.typology));
  } else {
    base = listModelObjectsForModelDefinition(model, modelDefinitionId);
  }
  return base.filter((object) => statDefinitionAppliesToObject(defn, object));
}

// #endregion 📊️StatDefinitions

/** @emoji 🧭️ Throws when `actionId` is outside the active model definition catalog. */
export function assertActionAvailableInModelDefinition(actionId: string, activeModelDefinitionId?: string | null): void {
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  if (!actionAvailableInModelDefinition(actionId, mdId)) {
    throw new Error(`action ${actionId} is not available in model definition ${mdId}`);
  }
}

/** @emoji 🧱️ Primitive entity kinds selectable on factory geometry (excludes typology `object` rows). */
export const PRIMITIVE_MODEL_ENTITY_KINDS: readonly ModelEntityKind[] = ["anchor", "vertex", "edge", "wire", "face", "shell", "solid"];

/** @emoji ✅️ True when `defn` applies to a model entity kind under the active model definition. */
export function attributeDefinitionAppliesToEntity(defn: AttributeDefinitionSpec, entityKind: ModelEntityKind): boolean {
  if (!defn.targets.includes(entityKind)) return false;
  const selector = defn.geometrySelector?.kinds;
  if (selector && selector.length > 0 && !selector.includes(entityKind)) return false;
  return true;
}

/** @emoji 📚️ Attribute definitions for one model definition and entity kind. */
export function listAttributeDefinitionsForModelDefinitionEntity(modelDefinitionId: string, entityKind: ModelEntityKind): readonly AttributeDefinitionSpec[] {
  return listAttributeDefinitionsForModelDefinition(modelDefinitionId).filter((defn) => attributeDefinitionAppliesToEntity(defn, entityKind));
}

/** @emoji 🧲️ True when the active model definition exposes factory-geometry pick targets (all definitions). */
export function modelDefinitionUsesGeometryPicking(_modelDefinitionId: string): boolean {
  return true;
}

/** @emoji 📋️ String/number/boolean/record options from an attribute value schema. */
export function attributeDefinitionValueOptions(defn: AttributeDefinitionSpec): readonly string[] | null {
  const schema = defn.value;
  if (!schema || typeof schema !== "object") return null;
  const row = schema as Record<string, unknown>;
  if (row.kind === "string" && Array.isArray(row.options)) {
    return row.options.filter((option): option is string => typeof option === "string");
  }
  if (row.kind === "oneOf" && Array.isArray(row.variants)) {
    for (const variant of row.variants) {
      if (!variant || typeof variant !== "object") continue;
      const v = variant as Record<string, unknown>;
      if (v.kind === "string" && Array.isArray(v.options)) {
        return v.options.filter((option): option is string => typeof option === "string");
      }
    }
  }
  return null;
}

/** @emoji 🧾️ Value editor kind inferred from an attribute definition schema. */
export function attributeDefinitionEditorKind(defn: AttributeDefinitionSpec): "string" | "enum" | "number" | "boolean" | "text" {
  if (attributeDefinitionValueOptions(defn)) return "enum";
  const schema = defn.value;
  if (!schema || typeof schema !== "object") return "text";
  const row = schema as Record<string, unknown>;
  if (row.kind === "number") return "number";
  if (row.kind === "boolean") return "boolean";
  if (row.kind === "string") return "string";
  if (row.kind === "oneOf") return "text";
  return "text";
}

/** @emoji ✅️ Validates a value against an attribute definition schema. */
export function validateAttributeValue(defn: AttributeDefinitionSpec, value: unknown): boolean {
  const schema = defn.value;
  if (!schema || typeof schema !== "object") return false;
  const row = schema as Record<string, unknown>;
  const kind = row.kind;
  if (kind === "string") {
    if (typeof value !== "string") return false;
    const options = row.options;
    if (Array.isArray(options) && options.length > 0) return options.includes(value);
    return true;
  }
  if (kind === "number") return typeof value === "number" && Number.isFinite(value);
  if (kind === "boolean") return typeof value === "boolean";
  if (kind === "record") {
    if (!value || typeof value !== "object" || Array.isArray(value)) return false;
    return true;
  }
  if (kind === "oneOf" && Array.isArray(row.variants)) {
    return row.variants.some((variant) => validateAttributeValue({ ...defn, value: variant }, value));
  }
  return false;
}

/** @emoji 🪪️ Qualified transformation id (`modelDefinitionId.transformationId`). */
export function qualifiedTransformationId(modelDefinitionId: string, transformationId: string): string {
  return `${modelDefinitionId}.${transformationId}`;
}

/** @emoji 🔄️ Z-band selector for surface classification rules. */
export type TransformationDeriveZBand = "min" | "max" | "mid";

/** @emoji 🔄️ One surface-classification rule in a transformation `derive` block. */
export interface TransformationDeriveClassifyRule {
  readonly role: string;
  readonly typology: string;
  readonly dominantAxis?: "x" | "y" | "z";
  readonly minDominantNormal?: number;
  readonly minAxisNormal?: number;
  readonly zBand?: TransformationDeriveZBand;
  readonly fallback?: boolean;
}

/** @emoji 🔄️ Opening metadata → typology mapping in a transformation `derive` block. */
export interface TransformationDeriveOpening {
  readonly fields: readonly string[];
  readonly values: readonly (string | boolean)[];
  readonly typology: string;
  readonly role: string;
}

/** @emoji 🔄️ Solid-fuse options for a transformation `derive` block. */
export interface TransformationDeriveFuse {
  readonly hullSolidId?: string;
  readonly contactPairs?: readonly (readonly [string, string])[];
  readonly maxSeparation?: number;
}

/** @emoji 🔄️ Source primitive collection for a transformation `derive` block. */
export interface TransformationDeriveCollect {
  readonly sourceModelDefinition: string;
  readonly primitiveKind: TypologyPrimitiveKind;
}

/** @emoji 🔄️ Hull object row for a transformation `derive` block. */
export interface TransformationDeriveHull {
  readonly typology: string;
  readonly primitiveKind: string;
}

/** @emoji 🔄️ Ensures typology rows exist after derive. */
export interface TransformationDeriveEnsure {
  readonly typology: string;
  readonly empty?: boolean;
}

/** @emoji 🔄️ Declarative surface-classification derive spec on `spatial.transformation/v1`. */
export interface TransformationDeriveSpec {
  readonly collect: TransformationDeriveCollect;
  readonly fuse?: TransformationDeriveFuse;
  readonly hull: TransformationDeriveHull;
  readonly classify: {
    readonly zTolRatio?: number;
    readonly zTolMin?: number;
    readonly rules: readonly TransformationDeriveClassifyRule[];
    readonly opening?: TransformationDeriveOpening;
    readonly mergeRoles?: readonly string[];
    readonly mergeByPlane?: boolean;
  };
  readonly ensure?: readonly TransformationDeriveEnsure[];
}

/** @emoji 🔄️ Parsed transformation (`spatial.transformation/v1`). */
export interface TransformationSpec {
  readonly schema: "spatial.transformation";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly modelDefinitionId: string;
  readonly source: { readonly modelDefinition: string };
  readonly target: { readonly modelDefinition: string };
  readonly typologies: readonly string[];
  readonly derive?: TransformationDeriveSpec;
}

function parseTransformationDeriveClassifyRule(raw: unknown): TransformationDeriveClassifyRule | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (typeof r.role !== "string" || typeof r.typology !== "string") return null;
  const dominantAxis = r.dominantAxis === "x" || r.dominantAxis === "y" || r.dominantAxis === "z" ? r.dominantAxis : undefined;
  const zBand = r.zBand === "min" || r.zBand === "max" || r.zBand === "mid" ? r.zBand : undefined;
  return {
    role: r.role,
    typology: r.typology,
    ...(dominantAxis ? { dominantAxis } : {}),
    ...(typeof r.minDominantNormal === "number" ? { minDominantNormal: r.minDominantNormal } : {}),
    ...(typeof r.minAxisNormal === "number" ? { minAxisNormal: r.minAxisNormal } : {}),
    ...(zBand ? { zBand } : {}),
    ...(r.fallback === true ? { fallback: true } : {}),
  };
}

function parseTransformationDeriveSpec(raw: unknown): TransformationDeriveSpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  const collect = r.collect as Record<string, unknown> | undefined;
  const hull = r.hull as Record<string, unknown> | undefined;
  const classify = r.classify as Record<string, unknown> | undefined;
  if (typeof collect?.sourceModelDefinition !== "string") return null;
  const primitiveKind = collect.primitiveKind;
  const allowed = new Set<TypologyPrimitiveKind>(["anchor", "solid", "surface", "curve"]);
  if (typeof primitiveKind !== "string" || !allowed.has(primitiveKind as TypologyPrimitiveKind)) return null;
  if (typeof hull?.typology !== "string" || typeof hull?.primitiveKind !== "string") return null;
  if (!classify || !Array.isArray(classify.rules) || classify.rules.length === 0) return null;
  const rules = classify.rules.map(parseTransformationDeriveClassifyRule).filter((row): row is TransformationDeriveClassifyRule => row !== null);
  if (rules.length !== classify.rules.length) return null;
  const fuseRaw = r.fuse as Record<string, unknown> | undefined;
  const fuse: TransformationDeriveFuse | undefined =
    fuseRaw && typeof fuseRaw === "object"
      ? {
          ...(typeof fuseRaw.hullSolidId === "string" ? { hullSolidId: fuseRaw.hullSolidId } : {}),
          ...(Array.isArray(fuseRaw.contactPairs)
            ? {
                contactPairs: fuseRaw.contactPairs.filter((pair): pair is [string, string] => Array.isArray(pair) && pair.length === 2 && typeof pair[0] === "string" && typeof pair[1] === "string").map((pair) => [pair[0], pair[1]] as const),
              }
            : {}),
          ...(typeof fuseRaw.maxSeparation === "number" ? { maxSeparation: fuseRaw.maxSeparation } : {}),
        }
      : undefined;
  const openingRaw = classify.opening as Record<string, unknown> | undefined;
  const opening: TransformationDeriveOpening | undefined =
    openingRaw && Array.isArray(openingRaw.fields) && openingRaw.fields.every((f) => typeof f === "string") && Array.isArray(openingRaw.values) && typeof openingRaw.typology === "string" && typeof openingRaw.role === "string"
      ? {
          fields: openingRaw.fields as string[],
          values: openingRaw.values as (string | boolean)[],
          typology: openingRaw.typology,
          role: openingRaw.role,
        }
      : undefined;
  const ensure: TransformationDeriveEnsure[] | undefined = Array.isArray(r.ensure)
    ? r.ensure
        .map((row) => {
          if (!row || typeof row !== "object") return null;
          const e = row as Record<string, unknown>;
          if (typeof e.typology !== "string") return null;
          return { typology: e.typology, ...(e.empty === true ? { empty: true } : {}) };
        })
        .filter((row): row is TransformationDeriveEnsure => row !== null)
    : undefined;
  return {
    collect: { sourceModelDefinition: collect.sourceModelDefinition, primitiveKind: primitiveKind as TypologyPrimitiveKind },
    ...(fuse ? { fuse } : {}),
    hull: { typology: hull.typology, primitiveKind: hull.primitiveKind },
    classify: {
      rules,
      ...(typeof classify.zTolRatio === "number" ? { zTolRatio: classify.zTolRatio } : {}),
      ...(typeof classify.zTolMin === "number" ? { zTolMin: classify.zTolMin } : {}),
      ...(opening ? { opening } : {}),
      ...(Array.isArray(classify.mergeRoles) ? { mergeRoles: classify.mergeRoles.filter((x): x is string => typeof x === "string") } : {}),
      ...(classify.mergeByPlane === true ? { mergeByPlane: true } : {}),
    },
    ...(ensure?.length ? { ensure } : {}),
  };
}

/** @emoji 🧾️ Parses `spatial.transformation/v1` JSON; `modelDefinitionId` comes from the asset folder. */
export function parseTransformationSpec(raw: unknown, modelDefinitionId: string): TransformationSpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.transformation") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  const source = r.source as { modelDefinition?: unknown } | undefined;
  const target = r.target as { modelDefinition?: unknown } | undefined;
  if (typeof source?.modelDefinition !== "string" || typeof target?.modelDefinition !== "string") return null;
  if (!Array.isArray(r.typologies) || r.typologies.length === 0) return null;
  const typologies = r.typologies.filter((row): row is string => typeof row === "string");
  if (typologies.length !== r.typologies.length) return null;
  const derive = r.derive !== undefined ? parseTransformationDeriveSpec(r.derive) : undefined;
  if (r.derive !== undefined && !derive) return null;
  return {
    schema: "spatial.transformation",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    modelDefinitionId,
    source: { modelDefinition: source.modelDefinition },
    target: { modelDefinition: target.modelDefinition },
    typologies,
    ...(derive ? { derive } : {}),
  };
}

function modelDefinitionIdFromTransformationAssetPath(assetPath: string): string | null {
  return modelDefinitionIdFromAssetPath(assetPath);
}

function shippedTransformationCatalog(): readonly TransformationSpec[] {
  const seen = new Set<string>();
  const out: TransformationSpec[] = [];
  for (const [path, raw] of Object.entries(modelDefinitionTransformationModules())) {
    const modelDefinitionId = modelDefinitionIdFromTransformationAssetPath(path);
    if (!modelDefinitionId) continue;
    const spec = parseTransformationSpec(raw, modelDefinitionId);
    if (!spec) continue;
    const qid = qualifiedTransformationId(spec.modelDefinitionId, spec.id);
    if (seen.has(qid)) continue;
    seen.add(qid);
    out.push(spec);
  }
  return out;
}

/** @emoji 📚️ Lists transformation assets under spatial/asset/modelDefinition. */
export function listModelDefinitionTransformations(): readonly TransformationSpec[] {
  return shippedTransformationCatalog();
}

/** @emoji 📚️ Loads a transformation by qualified id (`aec.building.energy.from_geometry`). */
export function loadTransformation(qualifiedId: string): TransformationSpec | null {
  return shippedTransformationCatalog().find((row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qualifiedId) ?? null;
}

/** @emoji 🔄️ Lists transformations whose target is `modelDefinitionId` (derive current definition from source). */
export function listTransformationsIntoModelDefinition(modelDefinitionId: string): readonly TransformationSpec[] {
  return listModelDefinitionTransformations().filter((row) => row.target.modelDefinition === modelDefinitionId);
}

/** @emoji 🔄️ Lists transformations whose source is `modelDefinitionId` (derive another definition from current). */
export function listTransformationsFromModelDefinition(modelDefinitionId: string): readonly TransformationSpec[] {
  return listModelDefinitionTransformations().filter((row) => row.source.modelDefinition === modelDefinitionId);
}

// #region 🧭️ModelDefinitionScope
function modelDefinitionAssetPathRest(assetPath: string): string | null {
  const normalized = assetPath.replace(/\\/g, "/");
  for (const marker of ["/asset/modelDefinition/", "assets/modelDefinition/", "assets/modelDefinitions/", "🖼️assets/🏗️modelDefinitions/"]) {
    const idx = normalized.indexOf(marker);
    if (idx >= 0) return normalized.slice(idx + marker.length);
  }
  return null;
}

function modelDefinitionFolderFromAssetPath(assetPath: string): string | null {
  const rest = modelDefinitionAssetPathRest(assetPath);
  if (!rest) return null;
  const folder = rest.split("/")[0];
  return folder || null;
}

function modelDefinitionFolderIdMap(): ReadonlyMap<string, string> {
  if (modelDefinitionFolderIdMapCache.current) return modelDefinitionFolderIdMapCache.current;
  const map = new Map<string, string>();
  const modules = {
    ...modelDefinitionAssetModules.current.manifests,
    ...modelDefinitionAssetModules.current.extensions,
  };
  for (const [path, raw] of Object.entries(modules)) {
    const folder = modelDefinitionFolderFromAssetPath(path);
    const manifest = parseModelDefinitionManifest(raw);
    if (!folder || !manifest) continue;
    map.set(folder, manifest.id);
  }
  modelDefinitionFolderIdMapCache.current = map;
  return map;
}

/** @emoji 🧭️ Resolves manifest `id` from an asset path under `spatial/asset/modelDefinition`. */
export function modelDefinitionIdFromAssetPath(assetPath: string): string | null {
  const folder = modelDefinitionFolderFromAssetPath(assetPath);
  if (!folder) return null;
  return modelDefinitionFolderIdMap().get(folder) ?? null;
}

function typologyOwnerById(): ReadonlyMap<string, string> {
  if (typologyOwnerByIdCache.current) return typologyOwnerByIdCache.current;
  const map = new Map<string, string>();
  for (const [path, raw] of Object.entries(modelDefinitionAssetModules.current.typologies)) {
    const owner = modelDefinitionIdFromAssetPath(path);
    const spec = parseTypologySpec(raw);
    if (!owner || !spec) continue;
    map.set(spec.id, owner);
  }
  typologyOwnerByIdCache.current = map;
  return map;
}

/** @emoji 🧭️ Typologies owned by a model-definition folder manifest. */
export function listTypologiesForModelDefinition(modelDefinitionId: string): readonly TypologySpec[] {
  const owners = typologyOwnerById();
  return shippedTypologyCatalog().filter((row) => owners.get(row.id) === modelDefinitionId);
}

function actionOwnerById(): ReadonlyMap<string, string> {
  if (actionOwnerByIdCache.current) return actionOwnerByIdCache.current;
  const map = new Map<string, string>();
  for (const [path, raw] of Object.entries(modelDefinitionAssetModules.current.actions)) {
    const owner = modelDefinitionIdFromAssetPath(path);
    const spec = parseActionSpec(raw);
    if (!owner || !spec) continue;
    map.set(spec.id, owner);
  }
  actionOwnerByIdCache.current = map;
  return map;
}

/** @emoji 🧭️ True when an action asset file lives under `modelDefinitionId`. */
export function actionOwnedByModelDefinition(actionId: string, modelDefinitionId: string): boolean {
  return actionOwnerById().get(actionId) === modelDefinitionId;
}

/** @emoji 🧭️ Model definition that owns a typology asset. */
export function modelDefinitionIdForTypology(typologyId: string): string | null {
  return typologyOwnerById().get(typologyId) ?? null;
}

/** @emoji 📚️ Host-facing interaction row from model-definition interaction JSON. */
export interface SpatialInteraction {
  readonly id: string;
  readonly label: string;
  /** @emoji ⌨️ Host interaction key; must stay unique and appear in `label`. */
  readonly key: string;
}

function interactionOwnerById(): ReadonlyMap<string, string> {
  if (interactionOwnerByIdCache.current) return interactionOwnerByIdCache.current;
  const map = new Map<string, string>();
  for (const [path, raw] of Object.entries(modelDefinitionAssetModules.current.interactions)) {
    const owner = modelDefinitionIdFromAssetPath(path);
    const spec = parseInteractionSpec(raw);
    if (!owner || !spec) continue;
    map.set(spec.id, owner);
  }
  interactionOwnerByIdCache.current = map;
  return map;
}

/** @emoji 🧭️ Model definition that owns an interaction asset. */
export function modelDefinitionIdForInteraction(interactionId: string): string | null {
  return interactionOwnerById().get(interactionId) ?? null;
}

/** @emoji 🧭️ Interactions shipped for a model definition (folder assets + typology references). */
export function listSpatialInteractionsForModelDefinition(modelDefinitionId: string, options?: { readonly includeCallable?: boolean }): readonly SpatialInteraction[] {
  const ids = new Set<string>();
  for (const [id, owner] of interactionOwnerById()) {
    if (owner === modelDefinitionId) ids.add(id);
  }
  for (const typology of listTypologiesForModelDefinition(modelDefinitionId)) {
    for (const interactionId of typology.interactions) ids.add(interactionId);
  }
  const catalog = new Map(shippedSpatialInteractionCatalog().map((row) => [row.id, row] as const));
  return [...ids]
    .map((id) => catalog.get(id))
    .filter((row): row is SpatialInteraction => row !== undefined)
    .filter((row) => {
      if (options?.includeCallable) return true;
      const spec = loadSpatialInteraction(row.id);
      return spec ? !isCallableOnlyInteraction(spec) : true;
    })
    .sort((a, b) => a.id.localeCompare(b.id));
}

function attributeOwnerById(): ReadonlyMap<string, string> {
  if (attributeOwnerByIdCache.current) return attributeOwnerByIdCache.current;
  const map = new Map<string, string>();
  for (const [path, raw] of Object.entries(modelDefinitionAssetModules.current.attributes)) {
    const owner = modelDefinitionIdFromAssetPath(path);
    const spec = parseAttributeDefinitionSpec(raw);
    if (!owner || !spec) continue;
    map.set(spec.id, owner);
  }
  attributeOwnerByIdCache.current = map;
  return map;
}

/** @emoji 🧭️ Attribute definitions owned by a model definition. */
export function listAttributeDefinitionsForModelDefinition(modelDefinitionId: string): readonly AttributeDefinitionSpec[] {
  const owners = attributeOwnerById();
  return shippedAttributeDefinitionCatalog().filter((row) => owners.get(row.id) === modelDefinitionId);
}

function propertyOwnerById(): ReadonlyMap<string, string> {
  if (propertyOwnerByIdCache.current) return propertyOwnerByIdCache.current;
  const map = new Map<string, string>();
  for (const [path, raw] of Object.entries({
    ...modelDefinitionAssetModules.current.propertyDefinitions,
    ...modelDefinitionAssetModules.current.properties,
  })) {
    const owner = modelDefinitionIdFromAssetPath(path);
    const spec = parsePropertyDefinitionSpec(raw);
    if (!owner || !spec) continue;
    map.set(spec.id, owner);
  }
  propertyOwnerByIdCache.current = map;
  return map;
}

/** @emoji 🧭️ Property definitions referenced by typologies in a model definition. */
export function listPropertyDefinitionsForModelDefinition(modelDefinitionId: string): readonly PropertyDefinitionSpec[] {
  const ids = new Set<string>();
  for (const row of shippedPropertyDefinitionCatalog()) {
    if (propertyOwnerById().get(row.id) === modelDefinitionId) ids.add(row.id);
  }
  for (const typology of listTypologiesForModelDefinition(modelDefinitionId)) {
    for (const propertyId of typology.properties ?? []) ids.add(propertyId);
  }
  return [...ids].map((id) => loadPropertyDefinition(id)).filter((row): row is PropertyDefinitionSpec => row !== null);
}

/** @emoji 🧭️ Interaction ids invoked via `interaction.call` in one spec. */
export function interactionIdsReferencedByInteractionSpec(spec: InteractionSpec): readonly string[] {
  const ids = new Set<string>();
  for (const st of spec.machine.states) {
    for (const h of st.on ?? []) {
      for (const tr of h.transitions) {
        for (const fx of tr.effects ?? []) {
          if (fx.operation === "interaction.call") ids.add(fx.interaction);
        }
      }
    }
  }
  return [...ids];
}

/** @emoji 🧭️ Action ids referenced by one interaction spec (transition effects + commit + nested interactions). */
export function actionIdsReferencedByInteractionSpec(spec: InteractionSpec): readonly string[] {
  const ids = new Set<string>();
  if (spec.commit.operation.kind === "action") {
    const kit = typologyConstructKitByInteraction().get(spec.id);
    if (kit) {
      ids.add(kit.constructFrom2PointsAndHeight);
      ids.add(kit.constructFromCurveAndHeight);
      ids.add(kit.constructFromSurface);
    } else {
      ids.add(spec.commit.operation.action);
    }
  }
  for (const st of spec.machine.states) {
    for (const h of st.on ?? []) {
      for (const tr of h.transitions) {
        for (const fx of tr.effects ?? []) {
          if (fx.operation === "action") ids.add(fx.action);
        }
      }
    }
  }
  for (const nestedId of interactionIdsReferencedByInteractionSpec(spec)) {
    const nested = loadSpatialInteraction(nestedId);
    if (nested) {
      for (const actionId of actionIdsReferencedByInteractionSpec(nested)) ids.add(actionId);
    }
  }
  return [...ids];
}

/** @emoji 🧭️ Action ids declared on typologies, action assets, or owned interactions. */
export function listActionsForModelDefinition(modelDefinitionId: string): readonly string[] {
  const ids = new Set<string>();
  for (const typology of listTypologiesForModelDefinition(modelDefinitionId)) {
    for (const actionId of typology.actions) {
      if (actionOwnedByModelDefinition(actionId, modelDefinitionId)) ids.add(actionId);
    }
  }
  for (const [path, raw] of Object.entries(modelDefinitionAssetModules.current.actions)) {
    if (modelDefinitionIdFromAssetPath(path) !== modelDefinitionId) continue;
    const spec = parseActionSpec(raw);
    if (spec) ids.add(spec.id);
  }
  for (const row of listSpatialInteractionsForModelDefinition(modelDefinitionId)) {
    const interaction = loadSpatialInteraction(row.id);
    if (interaction) {
      for (const actionId of actionIdsReferencedByInteractionSpec(interaction)) ids.add(actionId);
    }
  }
  return [...ids].sort((a, b) => a.localeCompare(b));
}

/** @emoji 🧭️ True when `actionId` is declared in the active model definition (or is `selection.apply`). */
export function actionAvailableInModelDefinition(actionId: string, modelDefinitionId: string): boolean {
  if (actionId === "selection.apply") return true;
  if (actionId.startsWith("command.")) return true;
  const transformation = loadTransformation(actionId);
  if (transformation) {
    return transformation.source.modelDefinition === modelDefinitionId || transformation.target.modelDefinition === modelDefinitionId;
  }
  return listActionsForModelDefinition(modelDefinitionId).includes(actionId);
}

/** @emoji 🧭️ Selection command fixtures whose action assets belong to a model definition. */
export function listSelectionOperationsForModelDefinition(modelDefinitionId: string): readonly SelectionOperationInteractionDef[] {
  return selectionOperationsForModelDefinitionFromActions(modelDefinitionId);
}

/** @emoji 🧭️ Selection entity kinds available while a model definition is active (factory primitives + objects). */
export function modelDefinitionSelectionEntityKinds(modelDefinitionId: string): readonly ModelEntityKind[] {
  const entityKindIds = new Set<string>([...PRIMITIVE_MODEL_ENTITY_KINDS, "object", "geometry", "attribute"]);
  const kinds = new Set<ModelEntityKind>([...PRIMITIVE_MODEL_ENTITY_KINDS, "object"]);
  for (const defn of listAttributeDefinitionsForModelDefinition(modelDefinitionId)) {
    for (const kind of defn.targets) {
      if (entityKindIds.has(kind)) kinds.add(kind as ModelEntityKind);
    }
    for (const kind of defn.geometrySelector?.kinds ?? []) {
      if (entityKindIds.has(kind)) kinds.add(kind as ModelEntityKind);
    }
  }
  const ordered: ModelEntityKind[] = [];
  for (const kind of PRIMITIVE_MODEL_ENTITY_KINDS) {
    if (kinds.has(kind)) ordered.push(kind);
  }
  for (const kind of kinds) {
    if (!(PRIMITIVE_MODEL_ENTITY_KINDS as readonly string[]).includes(kind)) ordered.push(kind);
  }
  return ordered;
}

/** @emoji 🧭️ Object rows owned by typologies declared under a model definition. */
export function listModelObjectsForModelDefinition(model: Model, modelDefinitionId: string): readonly SpatialObjectRecord[] {
  const typologyIds = new Set(listTypologiesForModelDefinition(modelDefinitionId).map((row) => row.id));
  const kernelTypologyMap = kernelTypologyIds(modelDefinitionId);
  if (kernelTypologyMap) {
    for (const typologyId of Object.values(kernelTypologyMap)) {
      if (typeof typologyId === "string" && typologyId.length > 0) typologyIds.add(typologyId);
    }
  }
  return Object.values(model.objects).filter((row) => {
    if (typologyIds.has(row.typology)) return true;
    return modelDefinitionIdForTypology(row.typology) === modelDefinitionId;
  });
}

/** @emoji 🧭️ Counts in-view typology objects for a model definition (renderer scope). */
export function countViewObjectsForModelDefinition(model: Model, modelDefinitionId: string): number {
  return listModelObjectsForModelDefinition(model, modelDefinitionId).length;
}

/** @emoji 🧭️ Summarizes scoped catalogs for the active model definition (hosts + REPL). */
export interface ModelDefinitionScope {
  readonly modelDefinitionId: string;
  readonly typologies: readonly TypologySpec[];
  readonly interactions: readonly SpatialInteraction[];
  readonly selectionOperations: readonly SelectionOperationInteractionDef[];
  readonly attributeDefinitions: readonly AttributeDefinitionSpec[];
  readonly propertyDefinitions: readonly PropertyDefinitionSpec[];
  readonly statDefinitions: readonly StatDefinitionSpec[];
  readonly actions: readonly string[];
  readonly selectionEntityKinds: readonly ModelEntityKind[];
}

/** @emoji 🧭️ Resolves everything available under one model definition manifest id. */
export function resolveModelDefinitionScope(modelDefinitionId: string): ModelDefinitionScope {
  return {
    modelDefinitionId,
    typologies: listTypologiesForModelDefinition(modelDefinitionId),
    interactions: listSpatialInteractionsForModelDefinition(modelDefinitionId),
    selectionOperations: listSelectionOperationsForModelDefinition(modelDefinitionId),
    attributeDefinitions: listAttributeDefinitionsForModelDefinition(modelDefinitionId),
    propertyDefinitions: listPropertyDefinitionsForModelDefinition(modelDefinitionId),
    statDefinitions: listStatDefinitionsForModelDefinition(modelDefinitionId),
    actions: listActionsForModelDefinition(modelDefinitionId),
    selectionEntityKinds: modelDefinitionSelectionEntityKinds(modelDefinitionId),
  };
}
// #endregion 🧭️ModelDefinitionScope

// #region 🔄️TransformationGeometry
/** @emoji 🔄️ Registered transformation applier for one qualified transformation id. */
export type TransformationApplier = (spec: TransformationSpec, source: Model) => Model;

const transformationAppliers = ephemeralMap<string, TransformationApplier>("s.plugins.cad.modules.core.component.ts.transformationAppliers");

/** @emoji 🔄️ Registers a model-definition-specific transformation implementation. */
export function registerTransformationApplier(qualifiedTransformationId: string, applier: TransformationApplier): void {
  transformationAppliers.set(qualifiedTransformationId, applier);
}

function collectTransformationPrimitiveRefs(model: Model, sourceModelDefinition: string, primitiveKind: TypologyPrimitiveKind): readonly SolidRef[] {
  const refs = new Set<string>();
  for (const obj of listModelObjectsForModelDefinition(model, sourceModelDefinition)) {
    for (const [kind, primitiveRef] of objectPrimitiveEntries(obj)) {
      if (kind === primitiveKind && model.solids[primitiveRef]) refs.add(primitiveRef);
    }
  }
  if (!refs.size) {
    for (const id of Object.keys(model.solids)) refs.add(id);
  }
  return [...refs].sort().map((id) => id as SolidRef);
}

export function transformationObjectId(typology: string, index: number): ObjectRef {
  return (index === 0 ? typology : `${typology}#${index}`) as ObjectRef;
}

function deriveOpeningMatch(model: Model, faceId: FaceRef, opening: TransformationDeriveOpening): boolean {
  const fields = model.metadata.get(String(faceId));
  if (!fields) return false;
  for (const field of opening.fields) {
    const value = fields[field];
    if (opening.values.some((candidate) => candidate === value)) return true;
  }
  return false;
}

function deriveClassifyRuleMatches(rule: TransformationDeriveClassifyRule, normal: Vec3, centroid: Vec3, zMin: number, zMax: number, zTol: number): boolean {
  if (rule.fallback) return true;
  const ax = Math.abs(normal[0]);
  const ay = Math.abs(normal[1]);
  const az = Math.abs(normal[2]);
  if (rule.dominantAxis === "z" && rule.minDominantNormal != null) {
    if (!(az >= ax && az >= ay && az >= rule.minDominantNormal)) return false;
    if (rule.zBand === "max") return centroid[2] >= zMax - zTol;
    if (rule.zBand === "min") return centroid[2] <= zMin + zTol;
    return true;
  }
  if (rule.minAxisNormal != null) return ax >= rule.minAxisNormal || ay >= rule.minAxisNormal;
  return false;
}

function classifyFaceFromDeriveRules(derive: TransformationDeriveSpec, normal: Vec3, centroid: Vec3, zMin: number, zMax: number, zTol: number): TransformationDeriveClassifyRule {
  for (const rule of derive.classify.rules) {
    if (deriveClassifyRuleMatches(rule, normal, centroid, zMin, zMax, zTol)) return rule;
  }
  return derive.classify.rules[derive.classify.rules.length - 1]!;
}

export function cloneModelGeometryShell(source: Model): Model {
  const target = new Model();
  target.revision = source.revision;
  target.anchors = source.anchors;
  target.vertices = source.vertices;
  target.edges = source.edges;
  target.wires = source.wires;
  target.faces = source.faces;
  target.shells = { ...source.shells };
  target.solids = { ...source.solids };
  return target;
}

/** @emoji 🔄️ Copies geometry and keeps only object rows whose typology is listed on the transformation spec. */
function applyTransformationFallback(spec: TransformationSpec, source: Model): Model {
  const target = cloneModelGeometryShell(source);
  const allowedTypologies = new Set(spec.typologies);
  const objects: Model["objects"] = {};
  for (const row of Object.values(source.objects)) {
    if (!allowedTypologies.has(row.typology)) continue;
    objects[row.id] = row;
  }
  target.objects = objects;
  target.bump();
  return target;
}

function runDeriveTransformation(spec: TransformationSpec, source: Model, preview: SpatialPreviewKernel): Model {
  const derive = spec.derive;
  if (!derive) throw new Error(`transformation ${qualifiedTransformationId(spec.modelDefinitionId, spec.id)} is missing derive`);
  const target = cloneModelGeometryShell(source);
  const solidRefs = collectTransformationPrimitiveRefs(source, derive.collect.sourceModelDefinition, derive.collect.primitiveKind);
  if (!solidRefs.length) {
    target.bump();
    return target;
  }
  const sourceObjectIds = sortedRecordValues(source.objects).map((row) => String(row.id));
  const { hullSolid, externalFaces } = preview.fuseSolidsToExternalFaces(target, solidRefs, {
    hullSolidId: derive.fuse?.hullSolidId,
    contactPairs: derive.fuse?.contactPairs,
    maxSeparation: derive.fuse?.maxSeparation,
  });
  if (solidRefs.length > 1) {
    target.metadata.setField(String(hullSolid), "fuseSourceSolidIds", solidRefs.map(String));
    target.solids[hullSolid] = { id: hullSolid, shellIds: [] };
  }
  const centroids: Vec3[] = [];
  const faceMeta: { readonly face: FaceRef; readonly normal: Vec3; readonly centroid: Vec3 }[] = [];
  for (const faceId of externalFaces) {
    const face = target.faces[faceId];
    if (!face) continue;
    const normal = preview.faceNormal(target, face);
    const centroid = preview.faceCentroid(target, face);
    if (!normal || !centroid) continue;
    faceMeta.push({ face: faceId, normal, centroid });
    centroids.push(centroid);
  }
  let zMin = Infinity;
  let zMax = -Infinity;
  for (const c of centroids) {
    zMin = Math.min(zMin, c[2]);
    zMax = Math.max(zMax, c[2]);
  }
  if (!Number.isFinite(zMin)) {
    zMin = 0;
    zMax = 0;
  }
  const zTolRatio = derive.classify.zTolRatio ?? 0.02;
  const zTolMin = derive.classify.zTolMin ?? 1e-3;
  const zTol = Math.max((zMax - zMin) * zTolRatio, zTolMin);
  const roleCounts = new Map<string, number>();
  const hullTypology = derive.hull.typology as TypologyRef;
  target.objects[transformationObjectId(hullTypology, 0)] = {
    id: transformationObjectId(hullTypology, 0),
    typology: hullTypology,
    primitives: { [derive.hull.primitiveKind]: String(hullSolid) },
    attributes: { sourceObjectIds, fusedSolidIds: solidRefs.map(String) },
  };
  const mergeRoles = new Set(derive.classify.mergeRoles ?? []);
  const mergeByPlane = derive.classify.mergeByPlane === true;
  const groupedFaces = new Map<string, { readonly role: string; readonly typology: TypologyRef; readonly faces: FaceRef[] }>();
  for (const row of faceMeta) {
    const opening = derive.classify.opening;
    const classified =
      opening && deriveOpeningMatch(source, row.face, opening)
        ? { role: opening.role, typology: opening.typology as TypologyRef }
        : (() => {
            const rule = classifyFaceFromDeriveRules(derive, row.normal, row.centroid, zMin, zMax, zTol);
            return { role: rule.role, typology: rule.typology as TypologyRef };
          })();
    const mergeGroup = mergeRoles.has(classified.role);
    const groupKey = mergeGroup && mergeByPlane ? `${classified.typology}:${preview.facePlaneGroupKey(row.normal, row.centroid)}` : `${classified.typology}:${String(row.face)}`;
    const existing = groupedFaces.get(groupKey);
    if (existing) existing.faces.push(row.face);
    else groupedFaces.set(groupKey, { role: classified.role, typology: classified.typology, faces: [row.face] });
  }
  for (const group of groupedFaces.values()) {
    const index = roleCounts.get(group.typology) ?? 0;
    roleCounts.set(group.typology, index + 1);
    const objectId = transformationObjectId(group.typology, index);
    target.objects[objectId] = {
      id: objectId,
      typology: group.typology,
      primitives: { surface: String(group.faces[0]!) },
      attributes: { sourceObjectIds, surfaceRole: group.role, faceIds: group.faces.map(String) },
    };
  }
  for (const ensure of derive.ensure ?? []) {
    if (!roleCounts.has(ensure.typology)) {
      const objectId = transformationObjectId(ensure.typology, 0);
      target.objects[objectId] = {
        id: objectId,
        typology: ensure.typology as TypologyRef,
        primitives: ensure.empty ? {} : { surface: String(externalFaces[0] ?? "") },
        attributes: { sourceObjectIds },
      };
    }
  }
  target.bump();
  return target;
}

// #endregion 🔄️TransformationGeometry

/** @emoji 🔄️ Derives a target-definition model from a source model (shared geometry, new object rows). */
export function applyTransformation(spec: TransformationSpec, source: Model, preview: SpatialPreviewKernel): Model {
  const qualified = qualifiedTransformationId(spec.modelDefinitionId, spec.id);
  const applier = transformationAppliers.get(qualified);
  if (applier) return applier(spec, source);
  if (spec.derive) return runDeriveTransformation(spec, source, preview);
  return applyTransformationFallback(spec, source);
}

// #endregion 🧱️Model

// #endregion 📦️📐️geometry

// #region 🧪️Tests
import { SpatialKernel, SpatialPreviewKernel, applyModelDiff } from "../🗺️spatial/🟦️component.ts";
import { CAD_GUMBALL_HIDDEN, cadGumballConfigVisible, collectGeometrySelectionTargets, modelDefinitionActionRegistry, runRegisteredAction } from "../../../../🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️actions/🟦️component.ts";

const __geometryTestRuntime = import.meta.vitest ? await import("../../../../🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️component.ts") : null;
const __geometryTestKernel = import.meta.vitest ? await import("../🧱️brepjs/🟦️component.ts") : null;
const CAD_E2E_ROUTES_MODEL_SPACE_JSON =
  '{"schema":"spatial.modelspace","revision":1,"models":[{"id":"spatial.shape","model":{"schema":"spatial.model","revision":1,"objects":[{"id":"object-wire-orbit-a","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"r10","position":[6,0,0.8]},{"kind":"vertex","id":"r11","position":[7.4,0.6,0.8]},{"kind":"vertex","id":"r12","position":[8.8,0.2,0.8]},{"kind":"vertex","id":"r13","position":[9.9,1.1,0.8]},{"kind":"vertex","id":"r14","position":[10.2,2.6,0.8]},{"kind":"vertex","id":"r15","position":[9.4,3.9,0.8]},{"kind":"vertex","id":"r16","position":[7.8,4.4,0.8]},{"kind":"vertex","id":"r17","position":[6.2,4.1,0.8]},{"kind":"curve","id":"re10","vertexIds":["r10","r11"]},{"kind":"curve","id":"re11","vertexIds":["r11","r12"]},{"kind":"curve","id":"re12","vertexIds":["r12","r13"]},{"kind":"curve","id":"re13","vertexIds":["r13","r14"]},{"kind":"curve","id":"re14","vertexIds":["r14","r15"]},{"kind":"curve","id":"re15","vertexIds":["r15","r16"]},{"kind":"curve","id":"re16","vertexIds":["r16","r17"]},{"kind":"curve","id":"re17","vertexIds":["r17","r10"]},{"kind":"curve","slot":"wire","id":"orbit-a","edgeIds":["re10","re11","re12","re13","re14","re15","re16","re17"]}]},{"id":"object-wire-spine-b","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"r18","position":[2,6,1.6]},{"kind":"vertex","id":"r19","position":[3.5,6.8,1.6]},{"kind":"vertex","id":"r20","position":[5.2,6.5,1.6]},{"kind":"vertex","id":"r21","position":[6.8,7.2,1.6]},{"kind":"vertex","id":"r22","position":[7.5,8.4,1.6]},{"kind":"vertex","id":"r23","position":[6.1,9.1,1.6]},{"kind":"curve","id":"re18","vertexIds":["r18","r19"]},{"kind":"curve","id":"re19","vertexIds":["r19","r20"]},{"kind":"curve","id":"re20","vertexIds":["r20","r21"]},{"kind":"curve","id":"re21","vertexIds":["r21","r22"]},{"kind":"curve","id":"re22","vertexIds":["r22","r23"]},{"kind":"curve","slot":"wire","id":"spine-b","edgeIds":["re18","re19","re20","re21","re22"]}]},{"id":"object-wire-stub-wire","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"r0","position":[0,0,0]},{"kind":"vertex","id":"r1","position":[1.2,0.4,0]},{"kind":"vertex","id":"r2","position":[2.6,0.1,0]},{"kind":"vertex","id":"r3","position":[3.8,0.9,0]},{"kind":"vertex","id":"r4","position":[4.5,2.1,0]},{"kind":"vertex","id":"r5","position":[4.2,3.5,0]},{"kind":"vertex","id":"r6","position":[3.1,4.2,0]},{"kind":"vertex","id":"r7","position":[1.5,4.5,0]},{"kind":"vertex","id":"r8","position":[0.2,3.8,0]},{"kind":"vertex","id":"r9","position":[-0.4,2.2,0]},{"kind":"curve","id":"re0","vertexIds":["r0","r1"]},{"kind":"curve","id":"re1","vertexIds":["r1","r2"]},{"kind":"curve","id":"re2","vertexIds":["r2","r3"]},{"kind":"curve","id":"re3","vertexIds":["r3","r4"]},{"kind":"curve","id":"re4","vertexIds":["r4","r5"]},{"kind":"curve","id":"re5","vertexIds":["r5","r6"]},{"kind":"curve","id":"re6","vertexIds":["r6","r7"]},{"kind":"curve","id":"re7","vertexIds":["r7","r8"]},{"kind":"curve","id":"re8","vertexIds":["r8","r9"]},{"kind":"curve","id":"re9","vertexIds":["r9","r0"]},{"kind":"curve","slot":"wire","id":"stub-wire","edgeIds":["re0","re1","re2","re3","re4","re5","re6","re7","re8","re9"]}]}]}}]}';

if (import.meta.vitest) {
  __geometryTestRuntime!.bootstrapCadModules();
  const { BrepjsKernel, preciseSpatialKernelMath } = __geometryTestKernel!;
  const geometryRoutesFixtureJson = JSON.parse(CAD_E2E_ROUTES_MODEL_SPACE_JSON) as ModelSpaceJson;
  const M = preciseSpatialKernelMath;
  const { describe, expect, it } = import.meta.vitest;

  describe("@semio-tech/cad-js/core vec", () => {
    it("adds and distances", () => {
      expect(M.vec3Distance([0, 0, 0], [3, 4, 0])).toBe(5);
    });
  });
  describe("@semio-tech/cad-js/core model definition catalogs", () => {
    it("loads attribute and property definition assets", () => {
      const attributes = listModelDefinitionAttributeDefinitions();
      const properties = listModelDefinitionPropertyDefinitions();
      expect(attributes.length).toBeGreaterThanOrEqual(6);
      expect(properties.some((row) => row.id === "spatial.shape.volume")).toBe(true);
      expect(loadAttributeDefinition("spatial.shape.material")?.field).toBe("material");
      expect(loadPropertyDefinition("spatial.shape.volume")?.unit).toBe("volume");
    });
    it("loads stat definition assets", () => {
      const stats = listModelDefinitionStatDefinitions();
      expect(stats.some((row) => row.id === "spatial.shape.geometry")).toBe(true);
      expect(stats.some((row) => row.id === "energy.demand")).toBe(true);
      expect(stats.some((row) => row.id === "structure.stability")).toBe(true);
      expect(loadStatDefinition("spatial.shape.geometry")?.outputs.some((row) => row.key === "totalVolume")).toBe(true);
      expect(listStatDefinitionsForModelDefinition("aec.building.energy").map((row) => row.id)).toContain("energy.demand");
      expect(resolveModelDefinitionScope(defaultModelDefinitionId()).statDefinitions.map((row) => row.id)).toContain("spatial.shape.geometry");
      expect(formatStatOutputValue(0.456, "percent")).toBe("45.6%");
    });
    it("computes spatial.shape.geometry stats for solid-backed objects", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const solidId = Object.keys(model.solids)[0]!;
      model.objects["obj-a"] = {
        id: "obj-a" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: solidId },
      };
      const defn = loadStatDefinition("spatial.shape.geometry")!;
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const out = await computeStat(defn, {
        model,
        kernel,
        modelDefinitionId: defaultModelDefinitionId(),
        scope: "model",
        objects: objectsForStatCompute(model, defaultModelDefinitionId(), defn, "model", []),
      });
      expect(out.objectCount).toBe(1);
      expect(out.solidCount).toBe(1);
      expect(out.totalVolume).toBeCloseTo(1, 3);
      expect(out.sizeX).toBeCloseTo(1, 3);
      expect(out.sizeY).toBeCloseTo(1, 3);
      expect(out.sizeZ).toBeCloseTo(1, 3);
    });
    it("computes energy and structure stats with finite outputs", async () => {
      const energyModel = new Model();
      applyModelDiff(energyModel, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("hull-solid")));
      energyModel.objects["hull"] = {
        id: "hull" as ObjectRef,
        typology: "energy.energy.hull" as TypologyRef,
        primitives: { solid: "hull-solid" },
      };
      const energyDefn = loadStatDefinition("energy.demand")!;
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const energyOut = await computeStat(energyDefn, {
        model: energyModel,
        kernel,
        modelDefinitionId: "aec.building.energy",
        scope: "model",
        objects: objectsForStatCompute(energyModel, "aec.building.energy", energyDefn, "model", []),
      });
      expect(Number.isFinite(energyOut.heatedVolume)).toBe(true);
      expect(Number.isFinite(energyOut.annualHeatingDemand)).toBe(true);
      expect(energyOut.heatedVolume).toBeGreaterThan(0);

      const structureModel = new Model();
      applyModelDiff(structureModel, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [0.4, 0.4, 0], height: 3 }, solidRef("column-solid")));
      structureModel.objects["column"] = {
        id: "column" as ObjectRef,
        typology: "structure.structure.reinforcedconcretecolumn" as TypologyRef,
        primitives: { solid: "column-solid" },
      };
      const structureDefn = loadStatDefinition("structure.stability")!;
      const structureOut = await computeStat(structureDefn, {
        model: structureModel,
        kernel,
        modelDefinitionId: "aec.building.structure",
        scope: "model",
        objects: objectsForStatCompute(structureModel, "aec.building.structure", structureDefn, "model", []),
      });
      expect(structureOut.elementCount).toBe(1);
      expect(Number.isFinite(structureOut.estimatedMass)).toBe(true);
      expect(structureOut.stabilityIndex).toBeGreaterThan(0);
      expect(structureOut.stabilityIndex).toBeLessThanOrEqual(1);
    });
    it("loads geometry and AEC typology assets", () => {
      const typologies = listModelDefinitionTypologies();
      expect(typologies.length).toBeGreaterThanOrEqual(27);
      expect(loadTypology("energy.energy.hull")?.properties).toContain("energy.heatedvolume");
      expect(loadTypology("energy.energy.hull")?.properties).toContain("spatial.shape.volume");
    });
    it("assigns primitiveKinds to geometry typologies", () => {
      const box = loadTypology("spatial.shape.primitive.box");
      const line = loadTypology("spatial.shape.curve.line");
      const plane = loadTypology("spatial.shape.surface.plane");
      expect(box?.primitiveKinds).toEqual(["solid"]);
      expect(line?.primitiveKinds).toEqual(["curve"]);
      expect(plane?.primitiveKinds).toEqual(["surface"]);
    });
    it("assigns primitiveKinds to AEC typologies", () => {
      expect(loadTypology("building.building.slab")?.primitiveKinds).toEqual(["surface"]);
      expect(loadTypology("building.building.wall")?.primitiveKinds).toEqual(["surface"]);
      expect(loadTypology("building.building.beam")?.primitiveKinds).toEqual(["curve"]);
      expect(loadTypology("building.building.column")?.primitiveKinds).toEqual(["solid"]);
      expect(loadTypology("energy.energy.externalwall")?.primitiveKinds).toEqual(["surface"]);
      expect(loadTypology("energy.energy.hull")?.primitiveKinds).toEqual(["solid"]);
      expect(loadTypology("structure.structure.onewayreinforcedconcreteslab")?.primitiveKinds).toEqual(["surface"]);
      expect(loadTypology("structure.linefem.lineelement")?.primitiveKinds).toEqual(["curve"]);
      expect(loadTypology("structure.surfacefem.surfaceelement")?.primitiveKinds).toEqual(["surface"]);
      expect(loadTypology("structure.solidfem.solidelement")?.primitiveKinds).toEqual(["solid"]);
    });
    it("resolveTypologyStyle is deterministic and distinct per typology id", () => {
      const a = resolveTypologyStyle("spatial.shape.primitive.box");
      const b = resolveTypologyStyle("spatial.shape.primitive.box");
      const c = resolveTypologyStyle("spatial.shape.curve.line");
      expect(a).toEqual(b);
      expect(a.color).toMatch(/^#[0-9a-f]{6}$/i);
      expect(c.color).not.toBe(a.color);
      expect(a.pattern.kind).toBe("none");
    });
    it("resolveTypologyStyle merges authored typology style overrides", () => {
      const slab = resolveTypologyStyle("structure.structure.onewayreinforcedconcreteslab");
      expect(slab.color).toBe("#8B7355");
      expect(slab.pattern.kind).toBe("hatch");
      expect(slab.pattern.direction).toBe(0);
      const wall = resolveTypologyStyle("structure.structure.reinforcedconcreteexternalwall");
      expect(wall.pattern.kind).toBe("crosshatch");
      expect(wall.edgeColor).toBe("#3D5560");
      const roof = resolveTypologyStyle("energy.energy.roof");
      expect(roof.pattern.kind).toBe("hatch");
      expect(roof.pattern.direction).toBe(90);
    });
    it("derives spatial.shape.volume for solid-backed objects", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const solidId = Object.keys(model.solids)[0]!;
      const object = {
        id: "obj-a" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: solidId },
      };
      const defn = loadPropertyDefinition("spatial.shape.volume")!;
      const kernel = {
        syncSolidsFromModel: async () => {},
        solidVolume: async () => 42,
      } as unknown as SpatialKernel;
      const out = await derivePropertyValue(defn, { model, kernel, object });
      expect(out.volume).toBe(42);
      expect(listApplicablePropertyDefinitionsForModelDefinition(defaultModelDefinitionId(), model, object).map((row) => row.id)).toContain("spatial.shape.volume");
    });
    it("validates object geometry against typology primitiveKinds", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const solidId = Object.keys(model.solids)[0]!;
      model.objects["obj-a"] = {
        id: "obj-a" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: solidId },
      };
      expect(objectMatchesTypologyPrimitives(model, model.objects["obj-a"]!)).toBe(true);
      model.objects["obj-b"] = {
        id: "obj-b" as ObjectRef,
        typology: "spatial.shape.selection.select-all" as TypologyRef,
        primitives: { solid: solidId },
      };
      expect(objectMatchesTypologyPrimitives(model, model.objects["obj-b"]!)).toBe(false);
      model.objects["obj-c"] = {
        id: "obj-c" as ObjectRef,
        typology: "building.building.slab" as TypologyRef,
        primitives: { solid: solidId },
      };
      expect(objectMatchesTypologyPrimitives(model, model.objects["obj-c"]!)).toBe(false);
      const faceId = Object.keys(model.faces)[0]!;
      model.objects["obj-d"] = {
        id: "obj-d" as ObjectRef,
        typology: "building.building.slab" as TypologyRef,
        primitives: { surface: faceId },
      };
      expect(objectMatchesTypologyPrimitives(model, model.objects["obj-d"]!)).toBe(true);
    });
  });
  describe("@semio-tech/cad-js/core model space and hashing", () => {
    it("hashes vertex positions stably", () => {
      const a = hashVertexPosition([1, 2, 3]);
      const b = hashVertexPosition([1.0000000004, 2, 3]);
      expect(a).toBe(b);
      expect(hashVertexPosition([1, 2, 4])).not.toBe(a);
    });
    it("links models in a model space", () => {
      const space = new ModelSpace();
      const m = new Model();
      applyModelDiff(m, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      space.link("primary", m);
      expect(space.get("primary")).toBe(m);
      const hashes = space.vertexHashesByModel();
      expect(Object.keys(hashes.primary ?? {}).length).toBe(8);
      const geo = space.geometryHashesByModel().primary;
      expect(Object.keys(geo?.solid ?? {}).length).toBeGreaterThanOrEqual(1);
      const roundTrip = ModelSpace.fromJSON(space.toJSON());
      expect(roundTrip.get("primary")?.revision).toBe(m.revision);
      expect(hashModelVertices(roundTrip.get("primary")!)).toEqual(hashes.primary);
    });
    it("hashes edges and solids deterministically", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const hashes = hashModelPrimitives(model);
      const solidId = Object.keys(model.solids)[0]!;
      const edgeId = Object.keys(model.edges)[0]!;
      expect(hashes.solid?.[solidId]).toBeTruthy();
      expect(hashes.edge?.[edgeId]).toBeTruthy();
      expect(hashes.solid?.[solidId]).toBe(hashSolidRecord(model.solids[solidId]!));
    });
  });
  describe("@semio-tech/cad-js/core transformations", () => {
    it("detects visible CAD gumball configs", () => {
      expect(cadGumballConfigVisible(CAD_GUMBALL_HIDDEN)).toBe(false);
      expect(cadGumballConfigVisible({ ...CAD_GUMBALL_HIDDEN, rotate: true })).toBe(true);
      expect(cadGumballConfigVisible(null)).toBe(false);
    });

    it("lists model definition manifests and transformation directions", () => {
      const manifests = listModelDefinitionManifests();
      expect(manifests.some((row) => row.id === "spatial.shape")).toBe(true);
      expect(manifests.some((row) => row.id === "aec.building.energy")).toBe(true);
      expect(listTransformationsIntoModelDefinition("aec.building.energy").some((row) => row.id === "from_geometry")).toBe(true);
      expect(listTransformationsFromModelDefinition("spatial.shape").some((row) => row.target.modelDefinition === "aec.building.energy")).toBe(true);
    });
    it("scopes catalogs to active model definition", () => {
      const shape = resolveModelDefinitionScope(defaultModelDefinitionId());
      const energy = resolveModelDefinitionScope("aec.building.energy");
      expect(shape.interactions.some((row) => row.id === "primitive.box")).toBe(true);
      expect(energy.interactions.length).toBe(5);
      expect(energy.interactions.every((row) => row.id.startsWith("energy.energy.construct"))).toBe(true);
      expect(energy.interactions.some((row) => row.id === "primitive.box")).toBe(false);
      expect(energy.typologies.some((row) => row.id === "energy.energy.hull")).toBe(true);
      expect(energy.selectionEntityKinds).toContain("object");
      expect(energy.selectionEntityKinds).toContain("solid");
      expect(shape.selectionEntityKinds).toContain("vertex");
      expect(shape.selectionEntityKinds).toContain("face");
      const structure = resolveModelDefinitionScope("aec.building.structure");
      expect(structure.selectionEntityKinds).toContain("face");
      expect(structure.selectionEntityKinds).toContain("object");
      expect(listAttributeDefinitionsForModelDefinitionEntity("aec.building.structure", "face").some((row) => row.field === "exposure")).toBe(true);
      expect(listPropertyDefinitionsForModelDefinition("aec.building.energy").some((row) => row.id === "energy.heatedvolume")).toBe(true);
      expect(actionAvailableInModelDefinition("primitive.createBoxFromCorners", defaultModelDefinitionId())).toBe(true);
      expect(actionAvailableInModelDefinition("primitive.createBoxFromCorners", "aec.building.energy")).toBe(false);
      expect(listSelectionOperationsForModelDefinition(defaultModelDefinitionId()).some((row) => row.id === "selection.selectVertices")).toBe(true);
      expect(listSelectionOperationsForModelDefinition("aec.building.energy").length).toBe(0);
    });
    it("buildModelPrimitiveDocument nests shell through vertex under solid", () => {
      const model = new Model();
      const solid = solidRef("box-solid");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solid));
      const tree = buildModelPrimitiveDocument(model, String(solid));
      expect(tree?.kind).toBe("solid");
      expect(tree?.children).toHaveLength(1);
      const shell = tree!.children[0]!;
      expect(shell.kind).toBe("shell");
      expect(shell.children.length).toBeGreaterThan(0);
      const face = shell.children[0]!;
      expect(face.kind).toBe("face");
      expect(face.children.some((row) => row.kind === "wire")).toBe(true);
      const wire = face.children.find((row) => row.kind === "wire")!;
      expect(wire.children.some((row) => row.kind === "edge")).toBe(true);
      const edge = wire.children.find((row) => row.kind === "edge")!;
      expect(edge.children.every((row) => row.kind === "vertex")).toBe(true);
      expect(edge.children.length).toBe(2);
    });

    it("listModelObjectsForModelDefinition filters objects by typology ownership", () => {
      const model = new Model();
      const cell = solidRef("c0");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      model.objects["hull"] = { id: "hull" as ObjectRef, typology: "energy.energy.hull", primitives: { solid: String(cell) } };
      model.objects["other"] = { id: "other" as ObjectRef, typology: "spatial.shape.primitive.box", primitives: { solid: String(cell) } };
      expect(listModelObjectsForModelDefinition(model, "aec.building.energy").map((row) => String(row.id))).toEqual(["hull"]);
      expect(countViewObjectsForModelDefinition(model, "aec.building.energy")).toBe(1);
    });

    it("listModelObjectsForModelDefinition includes kernel typology objects on shape model definition", () => {
      const model = new Model();
      const cell = solidRef("imported");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      model.objects["imported"] = {
        id: "imported" as ObjectRef,
        typology: "spatial.shape.kernel.solid",
        primitives: { solid: String(cell) },
      };
      expect(listModelObjectsForModelDefinition(model, defaultModelDefinitionId()).map((row) => String(row.id))).toEqual(["imported"]);
    });
    it("listModelObjectsForModelDefinition lists BIM class objects for aec.building", async () => {
      const { readFile } = await import("node:fs/promises");
      const { resolve } = await import("node:path");
      const fixturePath = resolve(import.meta.dirname, "../../📚️examples/🖼️assets/🎮️play/🔣️hexagonal-cut-concrete-forest-left.model.json");
      const fixtureJson = JSON.parse(await readFile(fixturePath, "utf8")) as ModelSpaceJson;
      const space = ModelSpace.fromJSON(fixtureJson);
      const building = space.models["aec.building"]!;
      expect(listTypologiesForModelDefinition("aec.building").some((row) => row.id === "building.building.column")).toBe(true);
      expect(listModelObjectsForModelDefinition(building, "aec.building")).toHaveLength(12);
    });
    it("collectGeometrySelectionTargets scopes object rows to model definition typologies", () => {
      const model = new Model();
      const cell = solidRef("c0");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      model.objects["hull"] = { id: "hull" as ObjectRef, typology: "energy.energy.hull", primitives: { solid: String(cell) } };
      model.objects["other"] = { id: "other" as ObjectRef, typology: "spatial.shape.primitive.box", primitives: { solid: String(cell) } };
      const all = collectGeometrySelectionTargets(model, ["object"], "aec.building.energy");
      expect(all.map((row) => row.id)).toEqual(["hull"]);
    });
    it("ActionRegistry rejects out-of-scope actions for active model definition", async () => {
      const actions = modelDefinitionActionRegistry();
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      await expect(
        runRegisteredAction(
          actions,
          "primitive.createBoxFromCorners",
          { cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1, __context: {}, __event: { kind: "test" } },
          { kernel, preview: kernel as unknown as SpatialPreviewKernel, model, activeModelDefinitionId: "aec.building.energy" },
        ),
      ).rejects.toThrow(/not available in model definition aec\.building\.energy/);
    });
    it("loads and applies from_geometry transformation", () => {
      const spec = loadTransformation("aec.building.energy.from_geometry");
      expect(spec?.source.modelDefinition).toBe("spatial.shape");
      expect(spec?.target.modelDefinition).toBe("aec.building.energy");
      const source = new Model();
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      source.objects["geom"] = {
        id: "geom" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "box" },
      };
      const target = applyTransformation(spec!, source, M);
      expect(target.objects["energy.energy.hull"]?.typology).toBe("energy.energy.hull");
      expect(target.objects["energy.energy.hull"]?.primitives).toEqual({ solid: "box" });
      expect(target.solids.box).toBe(source.solids.box);
      expect(listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.roof")).toHaveLength(1);
      expect(listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.baseplate")).toHaveLength(1);
      expect(listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.externalwall")).toHaveLength(4);
      expect(target.objects["energy.energy.roof"]?.primitives.surface).toBe("box-box-face-top");
      expect(target.objects["energy.energy.baseplate"]?.primitives.surface).toBe("box-box-face-bottom");
      const space = new ModelSpace();
      space.link("geometry", source);
      space.transfer("geometry", "energy", spec!, M);
      expect(space.get("energy")?.objects["energy.energy.windows"]).toBeTruthy();
    });
    it("loads and applies from_building transformation with shared solid geometry", () => {
      const spec = loadTransformation("aec.building.structure.from_building");
      expect(spec?.source.modelDefinition).toBe("aec.building");
      expect(spec?.target.modelDefinition).toBe("aec.building.structure");
      const source = new Model();
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [0.4, 0.4, 3], height: 3 }, solidRef("column-solid")));
      source.objects["col"] = {
        id: "col" as ObjectRef,
        typology: "building.building.column" as TypologyRef,
        primitives: { solid: "column-solid" },
      };
      const target = applyTransformation(spec!, source, M);
      expect(listModelObjectsForModelDefinition(target, "aec.building.structure.classic")).toHaveLength(1);
      expect(target.objects["structure.structure.reinforcedconcretecolumn"]?.typology).toBe("structure.structure.reinforcedconcretecolumn");
      expect(target.objects["structure.structure.reinforcedconcretecolumn"]?.primitives.solid).toBe("column-solid");
      expect(target.solids["column-solid"]).toBe(source.solids["column-solid"]);
    });
    it("from_geometry fuses touching shape solids and drops internal faces", () => {
      const spec = loadTransformation("aec.building.energy.from_geometry")!;
      const source = new Model();
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("lower")));
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 1], cornerB: [1, 1, 1], height: 1 }, solidRef("upper")));
      source.objects["lower"] = {
        id: "lower" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "lower" },
      };
      source.objects["upper"] = {
        id: "upper" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "upper" },
      };
      const target = applyTransformation(spec, source, M);
      expect(target.solids["from_geometry-hull"]?.shellIds).toEqual([]);
      expect(target.metadata.get("from_geometry-hull")?.fuseSourceSolidIds).toEqual(["lower", "upper"]);
      expect(target.objects["energy.energy.hull"]?.primitives.solid).toBe("from_geometry-hull");
      expect(target.objects["energy.energy.roof"]?.primitives.surface).toBe("box-upper-face-top");
      expect(target.objects["energy.energy.baseplate"]?.primitives.surface).toBe("box-lower-face-bottom");
      expect(listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.externalwall")).toHaveLength(4);
      expect(listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.hull" && row.id !== "energy.energy.hull")).toHaveLength(0);
      expect(target.objects["box-lower-face-top"]).toBeUndefined();
      expect(target.objects["box-upper-face-bottom"]).toBeUndefined();
    });
    it("from_geometry hull heated volume is boolean union not vertex AABB", async () => {
      const spec = loadTransformation("aec.building.energy.from_geometry")!;
      const source = new Model();
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("west")));
      applyModelDiff(source, M.boxModelDiff({ cornerA: [3, 0, 0], cornerB: [4, 1, 0], height: 1 }, solidRef("east")));
      source.objects["west"] = {
        id: "west" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "west" },
      };
      source.objects["east"] = {
        id: "east" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "east" },
      };
      const target = applyTransformation(spec, source, M);
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const hull = target.objects["energy.energy.hull"]!;
      const heated = await derivePropertyValue(loadPropertyDefinition("energy.heatedvolume")!, { model: target, kernel, object: hull });
      expect(heated.heatedvolume).toBeCloseTo(2, 3);
    });
  });
  describe("@semio-tech/cad-js/core attribute validation", () => {
    it("validates opening attribute options", () => {
      const defn = loadAttributeDefinition("spatial.shape.opening")!;
      expect(validateAttributeValue(defn, "window")).toBe(true);
      expect(validateAttributeValue(defn, "invalid")).toBe(false);
    });
  });
  describe("@semio-tech/cad-js/core edge and solid geometry", () => {
    it("arcEndOnCircle projects off-circle pick onto arc", () => {
      const end = M.arcEndOnCircle([0, 0, 0], [2, 0, 0], [0, 3, 0]);
      expect(end[0]).toBeCloseTo(0, 5);
      expect(end[1]).toBeCloseTo(2, 5);
      expect(M.vec3Distance([0, 0, 0], end)).toBeCloseTo(2, 5);
    });
    it("arcSamplePoints quarter arc from center start end", () => {
      const pts = M.arcSamplePoints([0, 0, 0], [2, 0, 0], [0, 2, 0], 4);
      expect(pts[0]).toEqual([2, 0, 0]);
      expect(pts[pts.length - 1]![0]).toBeCloseTo(0, 5);
      expect(pts[pts.length - 1]![1]).toBeCloseTo(2, 5);
    });
    it("circleSamplePoints and edgeCurveLength for circle curves", () => {
      const pts = M.circleSamplePoints([0, 0, 0], [0, 0, 1], 2, 64);
      expect(pts.length).toBeGreaterThan(8);
      expect(
        M.edgeCurveLength({ kind: "circle", center: [0, 0, 0], normal: [0, 0, 1], radius: 2 }, [
          [2, 0, 0],
          [2, 0, 0],
        ]),
      ).toBeCloseTo(M.cos(0) * 0 + M.sin(0) * 0 + 4 * 3.141592653589793, 3);
    });
    it("nurbs edge samples through control poles", () => {
      const curve = M.nurbsCurveFromPoles([
        [0, 0, 0],
        [1, 2, 0],
        [3, 0, 0],
      ] as Vec3[])!;
      const v0 = "v0" as VertexRef;
      const v1 = "v1" as VertexRef;
      const verts = {
        [v0]: { id: v0, position: [0, 0, 0] as Vec3 },
        [v1]: { id: v1, position: [3, 0, 0] as Vec3 },
      };
      const edge: EdgeRecord = { id: "e" as EdgeRef, vertexIds: [v0, v1], curve };
      const pts = M.edgeSamplePoints(verts, edge, 24);
      expect(pts.length).toBeGreaterThan(4);
    });
    it("solidPrimitiveAabb sphere bounds", () => {
      const b = M.solidPrimitiveAabb({ kind: "sphere", center: [1, 2, 3], radius: 5 });
      expect(b.min).toEqual([-4, -3, -2]);
      expect(b.max).toEqual([6, 7, 8]);
    });
  });
  describe("@semio-tech/cad-js/core expr", () => {
    it("evaluates numeric fold min expr", () => {
      const e: Expr = {
        kind: "fold",
        operation: "min",
        args: [
          { kind: "const", value: 3 },
          { kind: "const", value: 7 },
        ],
      };
      expect(evalExpr(e, { context: {}, preview: M })).toBe(3);
    });
    it("evaluates guards used by box factory", () => {
      const g: Expr = {
        kind: "all",
        args: [
          { kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "origin" }] } },
          {
            kind: "binop",
            operation: ">",
            left: { kind: "path", root: "context", segments: [{ kind: "field", name: "height" }] },
            right: { kind: "const", value: 0 },
          },
        ],
      };
      expect(evalGuard(g, { context: { origin: [0, 0, 0], height: 2 } })).toBe(true);
    });
  });
  describe("@semio-tech/cad-js/core model json", () => {
    it("materializeInlineObjectPrimitives promotes play fixture wires into model geometry", () => {
      const space = ModelSpace.fromJSON(geometryRoutesFixtureJson as ModelSpaceJson);
      const model = space.models["spatial.shape"]!;
      expect(model.wires["stub-wire"]?.edgeIds.length).toBeGreaterThan(0);
      expect(model.objects["object-wire-stub-wire"]?.primitives.wire).toBe("stub-wire");
    });

    it("parseModelJson fills missing entity arrays with empty lists", () => {
      const model = parseModelJson({
        schema: "spatial.model",
        revision: 1,
        objects: [],
        geometry: {
          vertices: [{ id: "v0", position: [0, 0, 0] }],
          edges: [{ id: "e0", vertexIds: ["v0", "v0"] }],
        },
      });
      expect(model).not.toBeNull();
      expect(Object.keys(model!.anchors).length).toBe(0);
      expect(Object.keys(model!.vertices).length).toBe(1);
      expect(Object.keys(model!.edges).length).toBe(1);
    });
  });
  describe("@semio-tech/cad-js/core metadata", () => {
    it("AttributeTable setField bumps model revision", () => {
      const g = new Model();
      const r0 = g.revision;
      g.metadata.setField("e1", "exposure", "external");
      expect(g.revision).toBeGreaterThan(r0);
      expect(g.metadata.get("e1")?.exposure).toBe("external");
    });

    it("AttributeTable entries and JSON roundtrip", () => {
      const store = new AttributeTable(() => {});
      store.setField("face-1", "exposure", "external");
      store.setField("face-2", "uValue", 0.25);
      const json = store.toJSON();
      expect(json).toHaveLength(2);
      const restored = AttributeTable.fromJSON(json);
      expect(restored.get("face-1")?.exposure).toBe("external");
      expect(restored.get("face-2")?.uValue).toBe(0.25);
    });

    it("ModelJson metadata roundtrip", () => {
      const g = new Model();
      g.metadata.setField("solid-a", "tag", "roof");
      const back = Model.fromJSON(g.toJSON());
      expect(back.metadata.get("solid-a")?.tag).toBe("roof");
    });

    it("AttributeTable getEntityFlags and setEntityFlag roundtrip", () => {
      const g = new Model();
      expect(g.getEntityFlags("obj-1")).toEqual({});
      g.setEntityFlag("obj-1", "hidden", true);
      g.setEntityFlag("face-2", "locked", true);
      expect(g.getEntityFlags("obj-1")).toEqual({ hidden: true });
      expect(g.getEntityFlags("face-2")).toEqual({ locked: true });
      g.setEntityFlag("obj-1", "hidden", false);
      expect(g.getEntityFlags("obj-1")).toEqual({});
      const back = Model.fromJSON(g.toJSON());
      expect(back.getEntityFlags("face-2")).toEqual({ locked: true });
    });
  });
  describe("@semio-tech/cad-js/core selection filter", () => {
    it("selectionEventMatches rejects kinds outside accept", () => {
      const spec: SelectionSpec = { accept: ["face"], multiple: false };
      const ok: SelectionEvent = {
        kind: "selection.changed",
        targets: [{ kind: "face", id: "f1", editable: true }],
      };
      const bad: SelectionEvent = {
        kind: "selection.changed",
        targets: [{ kind: "surface", id: "s1", editable: false }],
      };
      expect(selectionEventMatches(spec, ok)).toBe(true);
      expect(selectionEventMatches(spec, bad)).toBe(false);
    });

    it("expandSelectionTargetsForAccept maps object picks to wire primitives", () => {
      const model = new Model();
      const typology = "spatial.shape.curve.interpolate-curve";
      model.objects[typology as ObjectRef] = {
        id: typology as ObjectRef,
        typology: typology as TypologyRef,
        primitives: { curve: "w-interp" },
      };
      const spec: SelectionSpec = { accept: ["wire", "edge"], multiple: true };
      const expanded = expandSelectionTargetsForAccept(model, spec, [{ kind: "object", id: typology, editable: true }]);
      expect(expanded).toEqual([{ kind: "wire", id: "w-interp", editable: true }]);
    });
  });
}
// #endregion 🧪️Tests
