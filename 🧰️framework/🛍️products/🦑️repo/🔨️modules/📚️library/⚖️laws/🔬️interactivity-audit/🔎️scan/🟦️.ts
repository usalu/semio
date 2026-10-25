/** 🔎️ Interactivity audit: reachability predicates, comment masking and the file scan.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🏁️run/🟦️.ts
 */
import { policyMaskLiterals, policyTestModSpans, policyLineInTestMod, interactivityCfgTestItemSpans } from "../../../🔍️discovery/📋️policy/🔎️source/🟦️.ts";
import { policyReadFileSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyWalkRelFiles } from "../../../🔍️discovery/🚶️file-walk/🟦️.ts";
import { INTERACTIVITY_AUDIT_EXACT_BLOCKING_BRIDGE_FILES, INTERACTIVITY_AUDIT_PATTERNS, INTERACTIVITY_AUDIT_PRODUCTION_ROOTS, INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS, INTERACTIVITY_AUDIT_UI_ROOTS, type InteractivityAuditPatternDef, type InteractivityFinding } from "../🗂️files/🟦️.ts";
import type { SelfTestLanes } from "../../🧪️self-test-lanes/🟦️.ts";

/** ⏱️True when `relPath` sits under one of [[INTERACTIVITY_AUDIT_UI_ROOTS]]. */
function interactivityIsUiReachable(relPath: string): boolean {
  return INTERACTIVITY_AUDIT_UI_ROOTS.some((root) => relPath.startsWith(root));
}

function interactivityIsExactBlockingBridgeFile(relPath: string): boolean {
  return INTERACTIVITY_AUDIT_EXACT_BLOCKING_BRIDGE_FILES.some((file) => relPath === file);
}

/** 🛤️True when `relPath` sits under one of [[INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS]]. */
function interactivityIsRuntimeSanctioned(relPath: string): boolean {
  return INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS.some((root) => relPath.startsWith(root));
}

/** 📝️True for authored runtime sources; build scripts, fixtures, and test trees never execute in an interactive product process. */
export function interactivityIsRuntimeSource(relPath: string): boolean {
  return (
    INTERACTIVITY_AUDIT_PRODUCTION_ROOTS.some((root) => relPath === root || relPath.startsWith(`${root}/`)) &&
    !relPath.includes("/🧫️fixtures/") &&
    !relPath.includes("/🧪️tests/") &&
    !relPath.includes("/tests/") &&
    !relPath.includes("/benches/") &&
    !relPath.includes("/examples/") &&
    !relPath.endsWith("/build.rs")
  );
}



/** 🪆️Masks line and nested block comments after literals have been masked, preserving block depth across lines. */
function interactivityMaskComments(raw: string, initialDepth: number): { code: string; depth: number } {
  const masked = policyMaskLiterals(raw);
  let code = "";
  let depth = initialDepth;
  let index = 0;
  while (index < masked.length) {
    const pair = masked.slice(index, index + 2);
    if (depth > 0) {
      if (pair === "/*") {
        depth += 1;
        index += 2;
      } else if (pair === "*/") {
        depth -= 1;
        index += 2;
      } else {
        index += 1;
      }
      continue;
    }
    if (pair === "//") break;
    if (pair === "/*") {
      depth = 1;
      index += 2;
      continue;
    }
    code += masked[index];
    index += 1;
  }
  return { code, depth };
}





/** 🧪️Scans one file's lines for `patterns`, skipping `#[cfg(test)] mod … { … }` bodies — matches this repo's own R4 precedent that a test module is a sanctioned executor/blocking entry point, and keeps the audit signal free of unit-test noise unrelated to the UI-thread interactivity goal. */
function interactivityScanFile(repoRoot: string, relPath: string, patterns: readonly InteractivityAuditPatternDef[]): InteractivityFinding[] {
  const content = policyReadFileSafe(repoRoot, relPath);
  if (!content) return [];
  const findings: InteractivityFinding[] = [];
  const lines = content.split(/\r?\n/);
  const testSpans = [...policyTestModSpans(lines), ...interactivityCfgTestItemSpans(lines)];
  let blockCommentDepth = 0;
  lines.forEach((raw, i) => {
    const lineNo = i + 1;
    const masked = interactivityMaskComments(raw, blockCommentDepth);
    blockCommentDepth = masked.depth;
    if (policyLineInTestMod(testSpans, lineNo)) return;
    const codeOnly = masked.code;
    for (const pattern of patterns) {
      if (pattern.re.test(codeOnly)) findings.push({ category: pattern.category, file: relPath, line: lineNo, text: raw.trim().slice(0, 200) });
    }
  });
  return findings;
}

/** ⛔️Full forbidden-call scan: `blocking-bridge`/`sync-*` categories over [[INTERACTIVITY_AUDIT_UI_ROOTS]]; `thread-pool` over [[INTERACTIVITY_AUDIT_PRODUCTION_ROOTS]] minus [[INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS]]. */
export function interactivityAuditScan(repoRoot: string, lanes: SelfTestLanes): InteractivityFinding[] {
  lanes.interactivityRuntimeSourceSelfTests();
  const allRustFiles = policyWalkRelFiles(repoRoot, INTERACTIVITY_AUDIT_PRODUCTION_ROOTS, (_path, name) => name.endsWith(".rs")).filter(interactivityIsRuntimeSource);
  const uiPatterns = INTERACTIVITY_AUDIT_PATTERNS.filter((p) => p.scope === "ui");
  const runtimePatterns = INTERACTIVITY_AUDIT_PATTERNS.filter((p) => p.scope === "runtime-repo-wide");
  const findings: InteractivityFinding[] = [];
  for (const relPath of allRustFiles) {
    if (interactivityIsUiReachable(relPath) || interactivityIsExactBlockingBridgeFile(relPath)) findings.push(...interactivityScanFile(repoRoot, relPath, uiPatterns));
    if (!interactivityIsRuntimeSanctioned(relPath)) findings.push(...interactivityScanFile(repoRoot, relPath, runtimePatterns));
  }
  return findings;
}
