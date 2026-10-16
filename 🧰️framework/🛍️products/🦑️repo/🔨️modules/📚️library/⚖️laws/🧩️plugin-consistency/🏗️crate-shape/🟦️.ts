/** 🪚️ Wave 4 plugin consistency policy: region format, manifest region, struct naming, mod layout and SDK mechanism rules.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts
 */
import { policyTestModSpans } from "../../../🔍️discovery/📋️policy/🔎️source/🟦️.ts";
import { type BreachRecord } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { policyLineOfIndex } from "../../../🔍️discovery/📍️source-coordinate/🟦️.ts";
import { POLICY_LABELS_TWO_AXIS_ALLOWLIST, POLICY_TREE_ITEM_REDEFINITION_ALLOWLIST } from "../📋️allowlists/🟦️.ts";
import { policyAppIdFromCrateDir, type PolicyCrateRef, policyExtractFnBody, policyLabelName, policyModAtLine, policyPairRegionSpans, policyParseModSpans, policyParseRegionEvents, policyPascalAppStructName } from "../🏛️foundation/🟦️.ts";

//#region 🔧️PolicyRuleRegionFormat
/** 📏️V2 rule: `//#region 🔖️Name` / `//#endregion 🔖️Name` (no space after `//`), tests region must be `🧪️Tests`. */
export function policyRegionFormatBreaches(scope: string, lines: readonly string[]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const events = policyParseRegionEvents(lines);
  for (const ev of events) {
    const marker = ev.kind === "open" ? "region" : "endregion";
    if (ev.spaceAfterSlashes) {
      breaches.push({
        id: `region-format-space-${scope}-${ev.line}`,
        summary: `Region marker has a space after "//" (must be "//#${marker}", not "// #${marker}")`,
        kind: "app-plugin/region-marker-format",
        scope,
        line: ev.line,
        // 🎫️ W0 (Crate Consolidation & Plugin Taxonomy Restructure, root-script-policy-revival ticket):
        // downgraded from "high" the moment the discoverer revival first ran this rule against real
        // crates — ~1850 pre-existing "// #region" (space) markers across the plugin tree had never
        // been checked before (the old discoverer matched nothing), so this was never a hard gate in
        // practice. Fixing that many markers spans hundreds of files outside this ticket's script.ts-only
        // ownership; WARN-only until a dedicated formatting sweep ticket cleans it up, then flip back to "high".
        priority: "medium",
        reason: "Wave 4 V2 structure audit: region markers must be //#region 🔖️Name / //#endregion 🔖️Name, no space after //.",
        solution: `Remove the space between "//" and "#${marker}" on line ${ev.line}.`,
      });
    }
    if (!ev.label) {
      breaches.push({
        id: `region-format-missing-label-${scope}-${ev.line}`,
        summary: `//#${marker} marker is missing its label`,
        kind: "app-plugin/region-marker-format",
        scope,
        line: ev.line,
        priority: "medium",
        reason: "Every region open/close marker must carry its emoji+name label so pairs stay self-documenting.",
        solution: `Add the matching label after //#${marker} on line ${ev.line}.`,
      });
    }
  }
  for (const span of policyPairRegionSpans(events)) {
    const bare = policyLabelName(span.label);
    if (/^tests?$/i.test(bare) && span.label !== "🧪️Tests") {
      breaches.push({
        id: `region-tests-label-${scope}-${span.startLine}`,
        summary: `Tests region labeled "${span.label}" must be exactly "🧪️Tests"`,
        kind: "app-plugin/region-tests-label",
        scope,
        line: span.startLine,
        priority: "medium",
        reason: "Wave 4 V2 structure audit: the tests region sigil is reserved as 🧪️Tests.",
        solution: `Rename the region label at line ${span.startLine} (and its matching //#endregion) to "🧪️Tests".`,
      });
    }
    if (span.closeLabel && span.closeLabel !== span.label) {
      breaches.push({
        id: `region-label-mismatch-${scope}-${span.startLine}`,
        summary: `//#endregion label "${span.closeLabel}" does not match its //#region label "${span.label}"`,
        kind: "app-plugin/region-marker-format",
        scope,
        line: span.endLine,
        priority: "medium",
        reason: "Region open/close labels must match so the pairing stays unambiguous.",
        solution: `Fix the //#endregion label at line ${span.endLine} to read "${span.label}".`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleRegionFormat

//#region 🔧️PolicyRuleManifestRegion
/** 📏️V2 rule: every `App::builder(...)` call must be enclosed in a region labeled exactly `🔖️Manifest`. */
export function policyManifestRegionBreaches(scope: string, lines: readonly string[]): BreachRecord[] {
  const spans = policyPairRegionSpans(policyParseRegionEvents(lines));
  const testSpans = policyTestModSpans(lines);
  const breaches: BreachRecord[] = [];
  lines.forEach((line, i) => {
    if (!/App::builder\(/.test(line)) return;
    const lineNo = i + 1;
    if (testSpans.some((s) => s.startLine <= lineNo && lineNo <= s.endLine)) return; // synthetic test fixture, not a real app registration
    const enclosed = spans.some((s) => s.label === "🔖️Manifest" && s.startLine <= lineNo && lineNo <= s.endLine);
    if (!enclosed) {
      breaches.push({
        id: `manifest-region-${scope}-${lineNo}`,
        summary: `App::builder(...) call is not enclosed in a "🔖️Manifest" region`,
        kind: "app-plugin/manifest-region",
        scope,
        line: lineNo,
        priority: "medium",
        reason: "Wave 4 V2 structure audit: each app's App::builder(...) registration must live inside its own //#region 🔖️Manifest.",
        solution: `Wrap the App::builder(...) call at line ${lineNo} in a dedicated //#region 🔖️Manifest / //#endregion 🔖️Manifest.`,
      });
    }
  });
  return breaches;
}
//#endregion 🔧️PolicyRuleManifestRegion

//#region 🔧️PolicyRuleStructNaming
/** 📏️V2 rule: the `DocumentApp` struct backing `<PREFIX>_APP_ID` must be named `PascalCase(id) + "App"`. */
export function policyStructNamingBreaches(scope: string, content: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const idsByConst = new Map<string, string>();
  const constRe = /const\s+([A-Z][A-Z0-9_]*_APP_ID)\s*:\s*&str\s*=\s*"([^"]+)"/g;
  let cm: RegExpExecArray | null;
  while ((cm = constRe.exec(content))) idsByConst.set(cm[1]!, cm[2]!);

  const implRe = /impl\s+DocumentApp\s+for\s+(\w+)\s*\{/g;
  let m: RegExpExecArray | null;
  while ((m = implRe.exec(content))) {
    const structName = m[1]!;
    const window = content.slice(m.index + m[0].length, m.index + m[0].length + 2000);
    const constMatch = window.match(/\b([A-Z][A-Z0-9_]*_APP_ID)\b/);
    const id = constMatch ? idsByConst.get(constMatch[1]!) : undefined;
    if (!id) continue;
    const expected = policyPascalAppStructName(id);
    if (structName !== expected) {
      breaches.push({
        id: `struct-naming-${scope}-${structName}`,
        summary: `App struct "${structName}" for id "${id}" should be named "${expected}"`,
        kind: "app-plugin/struct-naming",
        scope,
        line: policyLineOfIndex(content, m.index),
        priority: "medium",
        reason: 'Wave 4 V2 structure audit: app struct name must be PascalCase(app id) + "App".',
        solution: `Rename struct ${structName} to ${expected} (and all its references) in ${scope}.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleStructNaming

//#region 🔧️PolicyRuleModLayout
/** 📏️V2 rule: crates declaring 2+ apps must isolate each `impl DocumentApp for` inside its own `pub mod`. */
export function policyModLayoutBreaches(scope: string, lines: readonly string[]): BreachRecord[] {
  const implLines: number[] = [];
  const implRe = /impl\s+DocumentApp\s+for\s+\w+\s*\{/;
  lines.forEach((line, i) => {
    if (implRe.test(line)) implLines.push(i + 1);
  });
  if (implLines.length < 2) return [];
  const modSpans = policyParseModSpans(lines);
  const breaches: BreachRecord[] = [];
  for (const lineNo of implLines) {
    if (!modSpans.some((s) => s.startLine <= lineNo && lineNo <= s.endLine)) {
      breaches.push({
        id: `mod-layout-${scope}-${lineNo}`,
        summary: `Multi-app crate declares an app (DocumentApp impl at line ${lineNo}) outside any "pub mod app_<name>" wrapper`,
        kind: "app-plugin/mod-layout",
        scope,
        line: lineNo,
        priority: "high",
        reason: "Wave 4 V2 structure audit: crates with 2+ apps must isolate each app inside its own pub mod (app_2d/app_3d, d2/d3/d5, ...).",
        solution: `Wrap the app declared at line ${lineNo} in its own pub mod block, matching the pattern used by gis/procedural/trinity/puzzle/s.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleModLayout

//#region 🔧️PolicyRuleSdkMechanisms
/** 🔎️Resolves a `use ... importedName as alias` rename so delegation checks accept the aliased call form too. */
function policyResolveImportAlias(content: string, importedName: string): string | undefined {
  return content.match(new RegExp(`\\b${importedName}\\s+as\\s+(\\w+)\\b`))?.[1];
}

/** 📏️V1 rule: local `fn selection_ids` must delegate to `semio_framework_plugin::selection_ids` before adding a fallback key. */
export function policySelectionIdsBreaches(scope: string, content: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const alias = policyResolveImportAlias(content, "selection_ids");
  const re = /fn\s+selection_ids\s*\(/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(content))) {
    const body = policyExtractFnBody(content, m.index);
    if (body.includes("semio_framework_plugin::selection_ids(") || (alias && new RegExp(`\\b${alias}\\(`).test(body))) continue;
    const lineNo = policyLineOfIndex(content, m.index);
    breaches.push({
      id: `sdk-selection-ids-${scope}-${lineNo}`,
      summary: `Local "fn selection_ids" does not delegate to semio_framework_plugin::selection_ids`,
      kind: "app-plugin/sdk-selection-ids",
      scope,
      line: lineNo,
      priority: "medium",
      reason: "Wave 4 V1 duplication audit: apps needing an extra fallback key must still call the SDK's selection_ids core first (see procedural::app_3d, reasoning/mindmap for the reference shape).",
      solution: `Rewrite selection_ids at line ${lineNo} to call semio_framework_plugin::selection_ids(args) first and only add the extra fallback key on top.`,
    });
  }
  return breaches;
}

/** 📏️V1 rule: local `fn new_app`/`fn new_app_with_registry`/`fn meta` must stay thin typed delegates to the SDK artifact app laws (allowing a `::<Turbofish>` generic before the call's `(`). */
export function policyArtifactAppLawDelegateBreaches(scope: string, content: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const specs: readonly { re: RegExp; mustRe: RegExp; label: string }[] = [
    { re: /fn\s+new_app_with_registry\s*[<(]/g, mustRe: /artifact_app_laws::new_app_with_registry(?:::<[^>]*>)?\s*\(/, label: "artifact_app_laws::new_app_with_registry" },
    { re: /fn\s+new_app\s*[<(]/g, mustRe: /artifact_app_laws::new_app(?:::<[^>]*>)?\s*\(/, label: "artifact_app_laws::new_app" },
    { re: /fn\s+meta\s*\(/g, mustRe: /artifact_app_laws::meta\s*\(/, label: "artifact_app_laws::meta" },
  ];
  for (const { re, mustRe, label } of specs) {
    let m: RegExpExecArray | null;
    while ((m = re.exec(content))) {
      const body = policyExtractFnBody(content, m.index);
      if (mustRe.test(body)) continue;
      const lineNo = policyLineOfIndex(content, m.index);
      breaches.push({
        id: `sdk-artifact-app-laws-delegate-${scope}-${lineNo}`,
        summary: `Local "${m[0]!.trim()}" does not delegate to semio_framework_plugin::${label}`,
        kind: "app-plugin/sdk-artifact-app-laws-delegate",
        scope,
        line: lineNo,
        priority: "medium",
        reason: "Wave 4 V1 duplication audit: per-type new_app/new_app_with_registry/meta helpers must stay thin typed delegates to the SDK artifact app laws.",
        solution: `Make the helper at line ${lineNo} call ${label}(...) instead of reimplementing it.`,
      });
    }
  }
  return breaches;
}

/** 📏️V1 rule: local `tree_item_with_action` redefinitions need an allowlisted SDK gap; other `tree_item_*` wrappers must delegate to it. */
export function policyTreeItemBreaches(crate: PolicyCrateRef, content: string, lines: readonly string[]): BreachRecord[] {
  const scope = crate.dir;
  const breaches: BreachRecord[] = [];
  const modSpans = policyParseModSpans(lines);

  const redefRe = /fn\s+tree_item_with_action\s*\(/g;
  let m: RegExpExecArray | null;
  while ((m = redefRe.exec(content))) {
    const lineNo = policyLineOfIndex(content, m.index);
    // 🌱️ The original allowlist keyed off an enclosing `pub mod d3 { … }` wrapper (one shared crate,
    // multiple apps namespaced by mod); today (and in the future taxonomy) each app is its own crate,
    // so fall back to the app id from the crate's own path when there's no such wrapper.
    const mod = policyModAtLine(modSpans, lineNo) || policyAppIdFromCrateDir(scope);
    if (POLICY_TREE_ITEM_REDEFINITION_ALLOWLIST.has(`${crate.pluginId}#${mod}`)) continue;
    breaches.push({
      id: `sdk-tree-item-redefinition-${scope}-${lineNo}`,
      summary: `Local "fn tree_item_with_action" shadows the SDK primitive of the same name`,
      kind: "app-plugin/sdk-tree-item",
      scope,
      line: lineNo,
      priority: "medium",
      reason: "Wave 4 V1 duplication audit: redefining tree_item_with_action locally is only accepted for a documented SDK gap (e.g. icon rendering).",
      solution: `Delete this and call semio_framework_plugin::tree_item_with_action directly, or if it exists for a genuine SDK gap, add it to POLICY_TREE_ITEM_REDEFINITION_ALLOWLIST citing the ticket.`,
    });
  }

  const wrapRe = /fn\s+(tree_item_\w+)\s*\(/g;
  while ((m = wrapRe.exec(content))) {
    const fnName = m[1]!;
    if (fnName === "tree_item_with_action") continue;
    const signatureEnd = content.indexOf(")", m.index);
    const signature = signatureEnd === -1 ? "" : content.slice(m.index, content.indexOf("{", signatureEnd));
    if (!signature.includes("ActionDescriptor")) continue; // structurally different helper (e.g. no action param) — not a delegation candidate
    const body = policyExtractFnBody(content, m.index);
    if (body.includes("tree_item_with_action(")) continue;
    const lineNo = policyLineOfIndex(content, m.index);
    breaches.push({
      id: `sdk-tree-item-wrapper-${scope}-${lineNo}`,
      summary: `Local "fn ${fnName}" does not delegate to tree_item_with_action`,
      kind: "app-plugin/sdk-tree-item",
      scope,
      line: lineNo,
      priority: "medium",
      reason: "Wave 4 V1 duplication audit: tree_item_* wrapper helpers must build on tree_item_with_action via struct-update syntax.",
      solution: `Rewrite ${fnName} at line ${lineNo} to construct its result via tree_item_with_action(...) plus a struct-update override.`,
    });
  }
  return breaches;
}

/** 📏️V1 rule: `struct XLabels` must be defined inside `semio_framework_plugin::app_labels! { ... }`, unless allowlisted as a documented SDK gap. */
export function policyLabelsStructBreaches(crate: PolicyCrateRef, content: string): BreachRecord[] {
  const scope = crate.dir;
  const breaches: BreachRecord[] = [];
  const re = /struct\s+(\w*Labels)\s*\{/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(content))) {
    const structName = m[1]!;
    // 🌱️ Line-based lookback (not a fixed char window): the `app_labels!` invocation can sit several
    // lines above the struct decl behind a multi-line /// doc comment (e.g. remodel, procedural).
    const lineNo = policyLineOfIndex(content, m.index);
    const precedingLines = content.split("\n").slice(Math.max(0, lineNo - 8), lineNo - 1);
    if (precedingLines.some((l) => l.includes("app_labels!"))) continue;
    const allowed = POLICY_LABELS_TWO_AXIS_ALLOWLIST.has(`${crate.pluginId}#${structName}`);
    breaches.push({
      id: `sdk-labels-struct-${scope}-${structName}`,
      summary: allowed ? `"struct ${structName}" is a tracked SDK-primitive gap (terminology axis) — Wave 4 decision pending` : `"struct ${structName}" hand-rolls its label set instead of semio_framework_plugin::app_labels!/LocaleLabels`,
      kind: "app-plugin/sdk-labels-struct",
      scope,
      line: lineNo,
      priority: allowed ? "low" : "medium",
      reason: allowed
        ? "Wave 4 V1 duplication audit flagged this for a Wave-4 design decision (extend LocaleLabels/app_labels! to a two-axis resolver, or formally accept the gap) — tracked, not a lint failure."
        : "Wave 4 V1 duplication audit: hand-rolled Labels structs (NATIVE/REUSE-style consts + resolver fn) should route through semio_framework_plugin::app_labels!/LocaleLabels unless there's a documented SDK-primitive gap.",
      solution: allowed ? `See .🧬semio/🦑️repo/🎫️tickets/26/07/18/WAVE-4-V1-DUPLICATION-HUNTER-AUDIT for the pending decision; if formally accepted, keep this allowlisted with that citation.` : `Route ${structName} through semio_framework_plugin::app_labels! { ... }, or if it needs a second axis, add it to POLICY_LABELS_TWO_AXIS_ALLOWLIST citing a ticket.`,
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleSdkMechanisms
