/** 🔺️ Rewriting's document delta uses shared map algebra for both keyed fields. */
import { parseMapDelta, type MapDelta } from "../../../../../../../../../../../🧰️framework/🔨️modules/📡️replication/🎮️mutation/🗂️map/🧬️schema/🟦️.ts";
import { parseDslValue } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
import { parseLayoutPoint, type LayoutPoint, type PropertyValue } from "../🟦️.ts";
export type { MapDelta } from "../../../../../../../../../../../🧰️framework/🔨️modules/📡️replication/🎮️mutation/🗂️map/🧬️schema/🟦️.ts";
export type { LayoutPoint, PropertyValue } from "../🟦️.ts";

export interface RewritingDiff {
  /** @state artifact */
  beforeFixtureJson?: string | null;
  /** @state artifact */
  lhsJson?: string | null;
  /** @state artifact */
  rhsJson?: string | null;
  /** @state artifact */
  parameterBindings?: MapDelta<PropertyValue> | null;
  /** @state artifact */
  ruleLayout?: MapDelta<LayoutPoint> | null;
}

/** 🪪️ Validates shared map structure and each field's domain payload. */
export function parseRewritingDiff(value: unknown): RewritingDiff {
  const row = parseDslValue(value);
  const keys = ["beforeFixtureJson", "lhsJson", "rhsJson", "parameterBindings", "ruleLayout"];
  if (row === null || typeof row !== "object" || Array.isArray(row) || Object.keys(row).some((key) => !keys.includes(key))) throw new Error("rewriting diff has incorrect fields");
  for (const key of keys.slice(0, 3)) if (row[key] != null && typeof row[key] !== "string") throw new Error("rewriting source change must be text");
  return {
    beforeFixtureJson: row.beforeFixtureJson as string | null ?? null,
    lhsJson: row.lhsJson as string | null ?? null,
    rhsJson: row.rhsJson as string | null ?? null,
    parameterBindings: row.parameterBindings == null ? null : parseMapDelta(row.parameterBindings),
    ruleLayout: row.ruleLayout == null ? null : parseMapDelta(row.ruleLayout, parseLayoutPoint),
  };
}
