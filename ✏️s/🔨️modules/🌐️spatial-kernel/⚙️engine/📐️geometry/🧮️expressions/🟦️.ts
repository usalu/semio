/** 🧮️ The interaction expression language: `Expr` nodes, environments, evaluation, guards, path targets on an environment and entity property reads. */
import type { Vec3 } from "@semio-tech/framework-3d-js";
import type { SpatialKernel, SpatialPreviewKernel } from "../../🗺️spatial/🟦️.ts";
import { type PathRoot, type PathSegment, type PathTarget, readPathSegments, writePathSegments, clearPathSegments, AttributeTable, type ModelEntityRef } from "../🗺️paths/🟦️.ts";
import { Model } from "../🧊️model/🟦️.ts";
import type { ModelEntityKind } from "../🟦️.ts";

//#region 🗺️Paths
function pathRootRecord(root: PathRoot, env: ExprEnv): unknown {
  if (root === "context") return env.context;
  if (root === "params") return env.params ?? {};
  return env.event;
}

/** 🧭️ Resolves a `PathTarget` against `ExprEnv`. */
export function readPathTarget(t: PathTarget, env: ExprEnv): unknown {
  return readPathSegments(pathRootRecord(t.root, env), t.segments);
}

/** 🧭️ Writes into `env.context` using a context-rooted path. */
export function writePathTarget(t: PathTarget, env: ExprEnv, value: unknown): void {
  if (t.root !== "context") return;
  writePathSegments(env.context, t.segments, value);
}

/** 🧭️ Clears `target` on `env.context`. */
export function clearPathTarget(t: PathTarget, env: ExprEnv): void {
  if (t.root !== "context") return;
  clearPathSegments(env.context, t.segments);
}
// #endregion 🗺️Paths

// #region 🗺️Expr
/** 🗺️ Tagged declarative expression evaluated by `evalExpr`. */
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
  readonly preview?: SpatialPreviewKernel;
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

/** 🧭️ Reads `name` from metadata, geometry records, or model objects. */
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

/** 🧮️ Evaluates a tagged `Expr` against `ExprEnv` (guards + action values). */
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
      return typeof v === "number" ? env.preview?.abs(v) : undefined;
    }
    case "distance": {
      const va = evalExpr(expr.a, env);
      const vb = evalExpr(expr.b, env);
      if (!isVec3(va) || !isVec3(vb)) return undefined;
      return env.preview?.vec3Distance(va, vb);
    }
    case "fold":
      return expr.operation === "min" ? env.preview?.min2(Number(evalExpr(expr.args[0], env)), Number(evalExpr(expr.args[1], env))) : env.preview?.max2(Number(evalExpr(expr.args[0], env)), Number(evalExpr(expr.args[1], env)));
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

/** 🧭️ Coerces `evalExpr` output to strict boolean guard result. */
export function evalGuard(expr: Expr, env: ExprEnv): boolean {
  return Boolean(evalExpr(expr, env));
}
// #endregion 🗺️Expr
