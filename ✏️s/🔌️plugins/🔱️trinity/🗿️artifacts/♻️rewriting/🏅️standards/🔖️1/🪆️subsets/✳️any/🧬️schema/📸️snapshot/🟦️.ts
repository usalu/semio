/** 📸️ Rewriting's durable document snapshot. */
import { parseRewritingArtifact, type LayoutPoint, type PropertyValue } from "../🟦️.ts";
export type { LayoutPoint, PropertyValue } from "../🟦️.ts";

export interface RewritingSnapshot {
  /** @state artifact */
  beforeFixtureJson: string;
  /** @state artifact */
  lhsJson: string;
  /** @state artifact */
  rhsJson: string;
  /** @state artifact */
  parameterBindings: Record<string, PropertyValue>;
  /** @state artifact */
  ruleLayout: Record<string, LayoutPoint>;
}

/** 📷️ Snapshot projection uses the durable document contract. */
export function parseRewritingSnapshot(value: unknown): RewritingSnapshot {
  return parseRewritingArtifact(value);
}
