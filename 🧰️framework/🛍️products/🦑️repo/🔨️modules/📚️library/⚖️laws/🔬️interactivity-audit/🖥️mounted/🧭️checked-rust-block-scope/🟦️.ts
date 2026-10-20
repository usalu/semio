/** 🧭️ The checked Rust block scope: one uniquely declared Rust item extracted through the shared literal and comment aware block scanner. */
import { toolJobRustBlock } from "../../../🎯️tool-job-coverage/📋️inventory/🟦️.ts";
import { policyStripRustCommentsAndStrings } from "../../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";

/** 🔎️ Extracts one uniquely declared Rust item through the shared literal/comment-aware block scanner. */
export function interactivityCheckedRustBlockScope(source: string, marker: string, name: string, failures: string[]): string | undefined {
  const escaped = marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const declarations = [...source.matchAll(new RegExp(`^[\\t ]*${escaped}`, "gmu"))];
  const stale = `P5a policy boundary is stale: ${name}`;
  if (declarations.length !== 1) {
    if (!failures.includes(stale)) failures.push(stale);
    return undefined;
  }
  const start = source.indexOf(marker, declarations[0]!.index);
  const headerEnd = start + marker.length;
  const open = source.indexOf("{", headerEnd);
  const semicolon = source.indexOf(";", headerEnd);
  const block = open < 0 || (semicolon >= 0 && semicolon < open) ? undefined : toolJobRustBlock(source, open);
  if (!block) {
    if (!failures.includes(stale)) failures.push(stale);
    return undefined;
  }
  return policyStripRustCommentsAndStrings(source.slice(start, block.end));
}
