/** 🧭️ The checked Rust block scope: one uniquely declared Rust item extracted through the shared literal and comment aware block scanner. */
import { toolJobRustBlock } from "../../../🎯️tool-job-coverage/📋️inventory/🟦️.ts";
import { policyStripRustCommentsAndStrings } from "../../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";

/** 🔎️ Extracts one uniquely declared Rust item through the shared literal/comment-aware block scanner. */
export function interactivityCheckedRustBlockScope(source: string, marker: string, name: string, failures: string[]): string | undefined {
  const bare = marker.replace(/^(?:pub(?:\([^)]*\))?\s+)/, "");
  const escaped = bare.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const tail = bare.endsWith("(") ? "" : "(?![A-Za-z0-9_])";
  const declarations = [...source.matchAll(new RegExp(`^[\\t ]*(?:pub(?:\\([^\\n)]*\\))?\\s+)?${escaped}${tail}`, "gmu"))];
  const stale = `P5a policy boundary is stale: ${name}`;
  if (declarations.length !== 1) {
    if (!failures.includes(stale)) failures.push(stale);
    return undefined;
  }
  const found = declarations[0]!;
  const start = found.index ?? 0;
  const headerEnd = start + found[0].length;
  const open = source.indexOf("{", headerEnd);
  const semicolon = source.indexOf(";", headerEnd);
  const block = open < 0 || (semicolon >= 0 && semicolon < open) ? undefined : toolJobRustBlock(source, open);
  if (!block) {
    if (!failures.includes(stale)) failures.push(stale);
    return undefined;
  }
  return policyStripRustCommentsAndStrings(source.slice(start, block.end));
}
