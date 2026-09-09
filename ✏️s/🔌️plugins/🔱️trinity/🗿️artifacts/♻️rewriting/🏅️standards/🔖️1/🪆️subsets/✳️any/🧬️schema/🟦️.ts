/** 🧬️ Rewriting's durable document contract. */
import { parseDslValue, type DslValue } from "../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
export type { DslValue as PropertyValue } from "../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";

export interface RewritingArtifact {
  /** @state artifact */
  beforeFixtureJson: string;
  /** @state artifact */
  lhsJson: string;
  /** @state artifact */
  rhsJson: string;
  /** @state artifact */
  parameterBindings: Record<string, DslValue>;
  /** @state artifact */
  ruleLayout: Record<string, LayoutPoint>;
}

/** 📐️ Position of a clause in the authored rewriting rule. */
export interface LayoutPoint { x: number; y: number; }

function record(value: DslValue, fields?: readonly string[]): Record<string, DslValue> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("rewriting value must be a record");
  if (fields && (Object.keys(value).length !== fields.length || fields.some((field) => !Object.hasOwn(value, field)))) throw new Error("rewriting record has incorrect fields");
  return value;
}

/** 📏️ Validates one durable layout point. */
export function parseLayoutPoint(value: unknown): LayoutPoint {
  const row = record(parseDslValue(value), ["x", "y"]);
  if (typeof row.x !== "number" || typeof row.y !== "number") throw new Error("layout coordinates must be finite numbers");
  return { x: row.x, y: row.y };
}

/** 🪪️ Validates the exact document and delegates dynamic values to the framework owner. */
export function parseRewritingArtifact(value: unknown): RewritingArtifact {
  const row = record(parseDslValue(value), ["beforeFixtureJson", "lhsJson", "rhsJson", "parameterBindings", "ruleLayout"]);
  if (typeof row.beforeFixtureJson !== "string" || typeof row.lhsJson !== "string" || typeof row.rhsJson !== "string") throw new Error("rewriting document sources must be strings");
  const parameterBindings = record(row.parameterBindings!);
  const ruleLayout = Object.fromEntries(Object.entries(record(row.ruleLayout!)).map(([key, point]) => [key, parseLayoutPoint(point)]));
  return { beforeFixtureJson: row.beforeFixtureJson, lhsJson: row.lhsJson, rhsJson: row.rhsJson, parameterBindings, ruleLayout };
}
