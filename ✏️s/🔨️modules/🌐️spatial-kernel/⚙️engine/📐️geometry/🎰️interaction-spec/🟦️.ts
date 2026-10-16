/** 🎰️ Interaction and action specs: states, transitions, effects, display items, engagement controls, and their parsers and compilers. */
import { ephemeralWeakMap } from "@semio-tech/framework";
import type { Vec3 } from "@semio-tech/framework-3d-js";
import type { SpatialPreviewKernel } from "../../🗺️spatial/🟦️.ts";
import { MODEL_ENTITY_KINDS, type SelectionSpec } from "../🪪️selection/🟦️.ts";
import type { PathTarget } from "../🗺️paths/🟦️.ts";
import { readPathTarget, writePathTarget, clearPathTarget, type Expr, type ExprEnv, evalExpr, evalGuard } from "../🧮️expressions/🟦️.ts";
import type { ModelEntityKind } from "../🟦️.ts";

//#region 🪪️Selection
/** 🧭️ Active `selection` block for `state`, or `null` when unrestricted. */
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

/** ✅️ Whether Enter/Space can fire a guarded `confirm` transition in `state`. */
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

// #region 📜️Spec
/** 📜️ Declared interaction-local context slots (`spatial.interaction/v1` `context`). */
export interface ContextFieldDecl {
  readonly name: string;
  readonly kind: "string" | "number" | "boolean" | "vec3" | "stringArray" | "unknown";
  readonly enumValues?: readonly string[];
}

/** 📜️ Named guard binding (`guards[]`). */
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
  | { readonly mutation: "assign"; readonly target: PathTarget; readonly value: Expr }
  | { readonly mutation: "clear"; readonly target: PathTarget }
  | { readonly mutation: "append"; readonly target: PathTarget; readonly value: Expr }
  | { readonly mutation: "emit"; readonly event: { readonly kind: string } }
  | { readonly mutation: "raise"; readonly event: string }
  | { readonly mutation: "openTransaction" }
  | { readonly mutation: "commitTransaction" }
  | { readonly mutation: "rollbackTransaction" }
  | { readonly mutation: "requestPreview" }
  | { readonly mutation: "kernel.query"; readonly query: string; readonly assignTo: PathTarget; readonly params?: Record<string, Expr> }
  | { readonly mutation: "resolveEditable" }
  | { readonly mutation: "setDiagnostic"; readonly severity: "info" | "warning" | "error"; readonly code: string; readonly message: string }
  | { readonly mutation: "clearDiagnostic"; readonly code: string }
  | { readonly mutation: "action"; readonly action: string; readonly params?: Record<string, Expr> }
  | {
      readonly mutation: "interaction.call";
      readonly interaction: string;
      readonly inputs?: Record<string, Expr>;
      readonly outputs?: readonly InteractionOutputBinding[] | Record<string, unknown>;
    };

/** 📞️ Maps host context paths from expressions evaluated against the child session context. */
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

/** 📜️ Parsed static interaction document (`spatial.interaction/v1`). */
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
  /** 📞️ `standalone` (default) for hosts; `callable` only via `interaction.call`. */
  readonly invocation?: InteractionInvocation;
  /** 🏷️ Typology object created when this interaction commits geometry. */
  readonly produces?: { readonly typology: string };
}

/** 📞️ How hosts may start an interaction (`standalone` vs nested-only `callable`). */
export type InteractionInvocation = "standalone" | "callable";

/** 🎛️ Engagement control kind declared on {@link InteractionLengthEntrySpec} / {@link InteractionScalarEntrySpec}. */
export type InteractionEngagementControlKind = "slider" | "stepper" | "ring";

/** 🎛️ Optional engagement control parameters on numeric interaction entries. */
export interface InteractionEngagementEntryControl {
  readonly control?: InteractionEngagementControlKind;
  readonly min?: number;
  readonly max?: number;
  readonly step?: number;
  readonly unit?: string;
  readonly default?: number;
}

/** 📏️ One rubber-band state where REPL digits clamp distance along the cursor ray. */
export interface InteractionLengthEntrySpec extends InteractionEngagementEntryControl {
  readonly state: string;
  readonly anchor: string;
  readonly field: string;
  /** ✅️ Host commit on Enter/Space (`pointer.down` default, `confirm` for scalar-like steps). */
  readonly commit?: "pointer.down" | "confirm";
}

/** 🔢️ One state where REPL digits set a scalar context field live (`set.height`, `set.radius`, …). */
export interface InteractionScalarEntrySpec extends InteractionEngagementEntryControl {
  readonly state: string;
  readonly event: string;
  readonly field: string;
  /** ✅️ Host commit on Enter/Space (defaults to `confirm`). */
  readonly commit?: "pointer.down" | "confirm";
  /** 📍️ Context path to Vec3 for axis XY (Z from `axisFloor` when set). */
  readonly axisAnchor?: string;
  /** 📍️ Context path to Vec3 whose Z is the axis floor (defaults to `axisAnchor`). */
  readonly axisFloor?: string;
  readonly axis?: readonly [number, number, number];
}

/** 🎚️ Resolved numeric engagement control for one interaction state. */
export interface ResolvedInteractionEngagementNumericControl {
  readonly kind: "slider" | "stepper";
  readonly label: string;
  readonly value: number;
  readonly min: number;
  readonly max?: number;
  readonly step: number;
  readonly unit?: string;
}

/** 🧫️ Resolved ring engagement control for one interaction state. */
export interface ResolvedInteractionEngagementRingControl {
  readonly kind: "ring";
  readonly label: string;
  readonly value?: string;
  readonly options: readonly { readonly id: string; readonly label: string }[];
  readonly min: number;
  readonly max: number;
  readonly step: number;
}

/** 🎛️ Resolved engagement control descriptor for {@link interactionControlForState}. */
export type ResolvedInteractionEngagementControl = ResolvedInteractionEngagementNumericControl | ResolvedInteractionEngagementRingControl;

/** 🎮️ Host + viewport hints for spatial picking (declared per interaction). */
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

/** 📞️ Resolved invocation for an interaction document. */
export function interactionInvocation(spec: InteractionSpec): InteractionInvocation {
  if (spec.invocation === "callable" || spec.invocation === "standalone") return spec.invocation;
  return "standalone";
}

/** 📞️ True when an interaction must not be started standalone by hosts. */
export function isCallableOnlyInteraction(spec: InteractionSpec): boolean {
  return interactionInvocation(spec) === "callable";
}

function guardNames(spec: InteractionSpec): Set<string> {
  return new Set((spec.guards ?? []).map((g) => g.name));
}

/** 📜️ Resolves a named guard's `Expr` from `spec.guards`. */
function lookupGuard(spec: InteractionSpec, name: string): Expr | null {
  return spec.guards?.find((g) => g.name === name)?.expr ?? null;
}

function findState(spec: InteractionSpec, name: string): StateDefSpec | undefined {
  return spec.machine.states.find((s) => s.name === name);
}

/** 🏁️ True when `state` is marked `final` on the interaction machine. */
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

/** 📞️ Writes child session values onto host context using declarative output bindings. */
export function mergeInteractionCallOutputs(hostContext: Record<string, unknown>, childContext: Record<string, unknown>, outputs: readonly InteractionOutputBinding[] | Record<string, unknown> | undefined): void {
  const bindings = interactionOutputBindings(outputs);
  if (!bindings?.length) return;
  const childEnv: ExprEnv = { context: childContext, event: { kind: "interaction.return" } };
  const hostEnv: ExprEnv = { ...childEnv, context: hostContext };
  for (const row of bindings) {
    writePathTarget(row.target, hostEnv, evalExpr(row.value, childEnv));
  }
}

/** 🧾️ Validates and returns an `InteractionSpec` or `null` when malformed. */
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

const COMPILED_INITIAL_CONTEXTS = ephemeralWeakMap<InteractionSpec, Record<string, unknown>>("s.spatial-kernel.geometry.COMPILED_INITIAL_CONTEXTS");

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
    if (effect.mutation === "interaction.call") continue;
    if (effect.mutation === "assign") writePathTarget(effect.target, env, evalExpr(effect.value, env));
    else if (effect.mutation === "clear") clearPathTarget(effect.target, env);
    else if (effect.mutation === "append") {
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

/** 📜️ Scripted commands that end in `committed` should commit from that state, not missing `ready`. */
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

/** 🧭️ Normalizes a parsed interaction so runtime sessions begin in the first active state. */
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

/** 🧾️ Declared shape of one `spatial.action/v1` step-spec parameter. */
export interface ActionParameterSpec {
  readonly kind: "string" | "number" | "boolean" | "vec3" | "stringArray" | "unknown";
}

/** 🧾️ One headless step of a parsed `spatial.action/v1` document. */
export type ActionStepSpec =
  | { readonly operation: "let"; readonly name: string; readonly value: Expr }
  | { readonly operation: "setContext"; readonly values: Record<string, Expr> }
  | { readonly operation: "deleteContext"; readonly keys: readonly string[] }
  | { readonly operation: "kernel.call"; readonly function: string; readonly args?: Record<string, Expr>; readonly assignTo?: string }
  | { readonly operation: "guard"; readonly condition: Expr; readonly message?: string }
  | { readonly operation: "return"; readonly diff?: Expr; readonly data?: Expr; readonly patch?: Expr; readonly result?: Expr };

/** 🧾️ Parsed data-only `spatial.action/v1` document. */
export interface ActionSpec {
  readonly schema: "spatial.action";
  readonly id: string;
  readonly version: string;
  readonly label?: string;
  readonly args?: Record<string, unknown>;
  readonly parameters?: Record<string, ActionParameterSpec>;
  readonly variables?: readonly { readonly name: string; readonly value: Expr }[];
  readonly steps: readonly ActionStepSpec[];
}

function hasExecutableActionField(raw: Record<string, unknown>): boolean {
  for (const key of ["run", "code", "function", "handler", "script"]) {
    if (key in raw) return true;
  }
  return false;
}

function isActionStepSpec(raw: unknown): raw is ActionStepSpec {
  if (!raw || typeof raw !== "object") return false;
  const r = raw as Record<string, unknown>;
  if (typeof r.operation !== "string") return false;
  if (r.operation === "let") return typeof r.name === "string" && Boolean(r.value);
  if (r.operation === "setContext") return Boolean(r.values) && typeof r.values === "object" && !Array.isArray(r.values);
  if (r.operation === "deleteContext") return Array.isArray(r.keys) && r.keys.every((k) => typeof k === "string");
  if (r.operation === "kernel.call") return typeof r.function === "string";
  if (r.operation === "guard") return Boolean(r.condition);
  if (r.operation === "return") return true;
  return false;
}

/** 🧾️ Parses a data-only `spatial.action/v1` document. */
export function parseActionSpec(raw: unknown): ActionSpec | null {
  if (!raw || typeof raw !== "object" || Array.isArray(raw)) return null;
  const r = structuredClone(raw) as Record<string, unknown>;
  if (hasExecutableActionField(r)) return null;
  if (r.schema !== "spatial.action") return null;
  if (typeof r.id !== "string" || r.id.length === 0 || typeof r.version !== "string") return null;
  if (r.args !== undefined && (!r.args || typeof r.args !== "object" || Array.isArray(r.args))) return null;
  if (!Array.isArray(r.steps) || r.steps.length === 0 || !r.steps.every(isActionStepSpec)) return null;
  const variables = r.variables;
  if (variables !== undefined) {
    if (!Array.isArray(variables)) return null;
    if (!variables.every((v) => v && typeof v === "object" && typeof (v as Record<string, unknown>).name === "string" && Boolean((v as Record<string, unknown>).value))) return null;
  }
  return r as unknown as ActionSpec;
}
// #endregion 📜️Spec
