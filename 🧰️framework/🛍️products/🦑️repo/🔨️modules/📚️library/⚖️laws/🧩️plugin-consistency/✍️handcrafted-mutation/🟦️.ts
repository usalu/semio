/** ✍️ Wave 4 plugin consistency policy: handcrafted spec scanners and direct mutation policies.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts
 */
import { canonicalPrimaryFilenameForKind, loadTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { type BreachRecord } from "../../../🚨️policy/🔖️breach/🟦️.ts";
import { loadCatalogTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { policyReadFileSafe, policyReaddirSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyWalkRelFiles } from "../../../🔍️discovery/🚶️file-walk/🟦️.ts";
import { policyListPluginArtifactDirs } from "../../../🔍️discovery/🗿️artifact/🏠️roots/🟦️.ts";
import { POLICY_MUTATIONS_FACET, POLICY_RS_COMPONENT_LEAF_NAME, POLICY_TS_COMPONENT_LEAF, policyArtifactRootOfMutationsDir, policyLeadingEmojiPrefix, policyStripEmoji } from "../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { policyFindAllMutationsDirs } from "../../../🧹️normalization/🧬️mutation/📸️captured-source/🟦️.ts";
import { policyKebabToPascal, policyMutationEnumVariantNames, policyMutationSemanticIdentity, policyMutationStructuralBreaches } from "../../../🧹️normalization/🧬️mutation/📐️structural-reachability/🟦️.ts";
import { policyListMutationDirs } from "../../../🧹️normalization/🧬️mutation/📇️direct-owner-index/🟦️.ts";
import { createHash } from "node:crypto";
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join, posix, resolve } from "node:path";
import { POLICY_DECLARED_USE_EXEMPTIONS, POLICY_EMPTY_EXAMPLE_EXEMPTIONS, POLICY_GENERIC_SPEC_EXEMPTIONS, POLICY_OP_FACET, POLICY_SPEC_DISTINCTNESS_EXEMPTIONS, POLICY_SPEC_WIRING_INCLUDE_EXEMPTIONS, POLICY_SPEC_WIRING_REGISTER_EXEMPTIONS } from "../📋️allowlists/🟦️.ts";
import type { PolicyCrateRef } from "../🏛️foundation/🟦️.ts";

//#region 🔧️PolicyRuleHandcraftedSpecP3
/** ⚖️P3/M4 handcrafted-grammar policy scanners (distinctness / generic / declared-use / wiring / empty examples). Exemptions shrink to empty by P6 — see ticket HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT. */

const POLICY_HANDCRAFTED_SPEC_ROOTS = ["✏️s", "🧰️framework"] as const;
const POLICY_HANDCRAFTED_FACETS = ["🗣️dsl", "🔧️op", "🔺️diff", "🎒️pack", "📡️spr", "🧬️mutations"] as const;
const POLICY_GRAMMAR_SPEC_LEAF = canonicalPrimaryFilenameForKind("grammar-semio", loadCatalogTaxonomy());
export const POLICY_PROTOCOL_SPEC_LEAF = canonicalPrimaryFilenameForKind("protocol-semio", loadCatalogTaxonomy());
export const POLICY_RS_COMPONENT_LEAF = canonicalPrimaryFilenameForKind(loadCatalogTaxonomy().componentFileKinds["🦀️rust"]!, loadCatalogTaxonomy());
const POLICY_FAMILY_ROOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/👪️family";

/** 🧹Normalizes grammar/protocol headers so distinctness compares body shape, not dialect names. */
function policyNormalizeSpecContent(content: string): string {
  return content
    .split(/\r?\n/)
    .map((line) => {
      if (/^(grammar|protocol|extension|schema)\s+\S+/.test(line)) {
        return line.replace(/^(grammar|protocol|extension|schema)\s+\S+/, "$1 __NAME__");
      }
      if (/^start\s+\S+/.test(line)) return "start __START__";
      return line;
    })
    .join("\n");
}

function policyHashSpecContent(content: string): string {
  return createHash("sha256").update(content).digest("hex");
}

/** 🔎️Walks `relRoots` for files matching `pred`, skipping `POLICY_SKIP_DIRS` / dotted dirs and directories matching `skipDir`. */
function policyDiscoverGrammarAndProtocolSpecs(repoRoot: string): string[] {
  return policyWalkRelFiles(repoRoot, POLICY_HANDCRAFTED_SPEC_ROOTS, (_p, name) => name.endsWith(".grammar.semio") || name.endsWith(".protocol.semio"));
}

/**
 * 🔏️Normalized-hash collision rule: two `.grammar.semio`/`.protocol.semio` under ✏️s/ or 🧰️framework/
 * must not share a body after dialect name/start normalization.
 */
function policySpecDistinctnessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const byHash = new Map<string, string[]>();
  for (const relPath of policyDiscoverGrammarAndProtocolSpecs(repoRoot)) {
    const raw = policyReadFileSafe(repoRoot, relPath);
    const hash = policyHashSpecContent(policyNormalizeSpecContent(raw));
    const group = byHash.get(hash);
    if (group) group.push(relPath);
    else byHash.set(hash, [relPath]);
  }
  for (const [hash, group] of byHash) {
    if (group.length < 2) continue;
    const sorted = [...group].sort();
    for (let i = 0; i < sorted.length; i++) {
      for (let j = i + 1; j < sorted.length; j++) {
        const a = sorted[i]!;
        const b = sorted[j]!;
        if (POLICY_SPEC_DISTINCTNESS_EXEMPTIONS.has(a) || POLICY_SPEC_DISTINCTNESS_EXEMPTIONS.has(b)) continue;
        breaches.push({
          id: `spec-distinctness-${hash.slice(0, 12)}-${a}-${b}`,
          summary: `Normalized spec collision between "${a}" and "${b}"`,
          kind: "handcrafted-grammar/spec-distinctness",
          scope: a,
          priority: "high",
          reason: "Each artifact facet must own a distinct grammar/protocol body; shared stubs after name/start normalization are forbidden.",
          solution: `Handcraft distinct specs for both paths, then remove them from POLICY_SPEC_DISTINCTNESS_EXEMPTIONS.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 🚩️Flags catch-all generic grammar shapes under ✏️s/ (prop IDENT=, untyped list/map/value with prop,
 * bare IDENT assign* shells, json/blob/base64/payload field names).
 */
function policyGenericSpecBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const grammars = policyWalkRelFiles(repoRoot, ["✏️s"], (_p, name) => name.endsWith(".grammar.semio"));
  for (const relPath of grammars) {
    if (POLICY_GENERIC_SPEC_EXEMPTIONS.has(relPath)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    const reasons: string[] = [];
    if (/prop\s*=\s*IDENT\s*"="\s*\(/.test(content) || /prop\s*=\s*IDENT\s*=/.test(content)) {
      reasons.push("catch-all prop = IDENT");
    }
    const untyped: string[] = [];
    for (const name of ["list", "map", "value"] as const) {
      if (new RegExp(`^\\s*${name}\\s*=`, "m").test(content)) untyped.push(name);
    }
    if (/\bprop\b/.test(content) && untyped.length > 0) {
      reasons.push(`untyped catch-all production(s) ${untyped.join(", ")} with prop`);
    }
    if (/=\s*IDENT\s+assign\*\s+block\?/.test(content) || /=\s*IDENT\s+assign\*/.test(content)) {
      reasons.push("bare IDENT assign* statement shell");
    }
    if (/-(json|blob|base64|payload)"/.test(content) || /-(json|blob|base64|payload)\b/.test(content)) {
      reasons.push("json/blob/base64/payload field name");
    }
    if (reasons.length === 0) continue;
    breaches.push({
      id: `generic-spec-${relPath}`,
      summary: `"${relPath}" still uses generic grammar shape: ${reasons.join("; ")}`,
      kind: "handcrafted-grammar/generic-spec",
      scope: relPath,
      priority: "high",
      reason: "Artifact grammars must be domain-true; catch-all prop/list/map/value and bare statement shells are migration leftovers.",
      solution: `Handcraft ${relPath}, then remove it from POLICY_GENERIC_SPEC_EXEMPTIONS.`,
    });
  }
  return breaches;
}

/** 📖️Loads `📖️family-*.grammar.semio` keyed by `family-*` id. */
function policyLoadFamilyFragments(repoRoot: string): Map<string, { relPath: string; productions: readonly string[] }> {
  const out = new Map<string, { relPath: string; productions: readonly string[] }>();
  const files = policyWalkRelFiles(repoRoot, [POLICY_FAMILY_ROOT], (_p, name) => name.startsWith("📖️family-") && name.endsWith(".grammar.semio"));
  for (const relPath of files) {
    const leaf = relPath.split("/").pop()!;
    const id = leaf.replace(/^📖️/, "").replace(/\.grammar\.semio$/, "");
    const content = policyReadFileSafe(repoRoot, relPath);
    const productions: string[] = [];
    for (const m of content.matchAll(/^([A-Za-z_][\w-]*)\s*=/gm)) {
      const name = m[1]!;
      if (name === "grammar" || name === "start" || name === "extension" || name === "dialect" || name === "use" || name === "schema" || name === "protocol") continue;
      productions.push(name);
    }
    out.set(id, { relPath, productions });
  }
  return out;
}

/**
 * 🔣️Every `use family-X` must reference at least one production symbol from that family fragment.
 */
function policyDeclaredUseBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const families = policyLoadFamilyFragments(repoRoot);
  const grammars = policyWalkRelFiles(repoRoot, ["✏️s"], (_p, name) => name.endsWith(".grammar.semio"));
  for (const relPath of grammars) {
    if (POLICY_DECLARED_USE_EXEMPTIONS.has(relPath)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    const uses = [...content.matchAll(/^use\s+(family-[\w-]+)/gm)].map((m) => m[1]!);
    if (uses.length === 0) continue;
    for (const fam of uses) {
      const frag = families.get(fam);
      if (!frag) {
        breaches.push({
          id: `declared-use-missing-${relPath}-${fam}`,
          summary: `"${relPath}" declares use ${fam} but no family fragment was found under ${POLICY_FAMILY_ROOT}`,
          kind: "handcrafted-grammar/declared-use",
          scope: relPath,
          priority: "high",
          reason: "Declared family imports must resolve to a 📖️family-*.grammar.semio fragment.",
          solution: `Add the family fragment or fix the use line, then remove ${relPath} from POLICY_DECLARED_USE_EXEMPTIONS if listed.`,
        });
        continue;
      }
      if (frag.productions.length === 0) continue;
      const referenced = frag.productions.some((prod) => new RegExp(`\b${prod.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}\b`).test(content));
      if (referenced) continue;
      breaches.push({
        id: `declared-use-${relPath}-${fam}`,
        summary: `"${relPath}" declares use ${fam} but never references any production from ${frag.relPath}`,
        kind: "handcrafted-grammar/declared-use",
        scope: relPath,
        priority: "high",
        reason: "A use family-X line that never mentions a family production is dead wiring.",
        solution: `Reference at least one production from ${fam} in the artifact grammar, then remove ${relPath} from POLICY_DECLARED_USE_EXEMPTIONS.`,
      });
    }
  }
  return breaches;
}



function policyArtifactHasRegisterLanguage(repoRoot: string, artRel: string): boolean {
  const rsFiles = policyWalkRelFiles(repoRoot, [artRel], (_p, name) => name.endsWith(".rs"));
  for (const rel of rsFiles) {
    if (policyReadFileSafe(repoRoot, rel).includes("register_language")) return true;
  }
  return false;
}

/**
 * 💠️Facet `🦀️.rs` must `include_str!` its sibling grammar/protocol spec; artifacts with any
 * facet specs must call `register_language` somewhere under the artifact (engine preferred).
 */
function policySpecWiringBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    let hasAnySpec = false;
    for (const facet of POLICY_HANDCRAFTED_FACETS) {
      const facetRel = `${artRel}/${facet}`;
      if (!existsSync(join(repoRoot, facetRel))) continue;
      const grammarRel = `${facetRel}/${POLICY_GRAMMAR_SPEC_LEAF}`;
      const protocolRel = `${facetRel}/${POLICY_PROTOCOL_SPEC_LEAF}`;
      const hasGrammar = existsSync(join(repoRoot, grammarRel));
      const hasProtocol = existsSync(join(repoRoot, protocolRel));
      if (!hasGrammar && !hasProtocol) continue;
      hasAnySpec = true;
      const rsRel = `${facetRel}/${POLICY_RS_COMPONENT_LEAF}`;
      if (!existsSync(join(repoRoot, rsRel))) continue;
      if (POLICY_SPEC_WIRING_INCLUDE_EXEMPTIONS.has(rsRel)) continue;
      const rsBody = policyReadFileSafe(repoRoot, rsRel);
      const specs: string[] = [];
      if (hasGrammar) specs.push(POLICY_GRAMMAR_SPEC_LEAF);
      if (hasProtocol) specs.push(POLICY_PROTOCOL_SPEC_LEAF);
      for (const specLeaf of specs) {
        const wired = rsBody.includes("include_str!") && rsBody.includes(specLeaf);
        if (wired) continue;
        breaches.push({
          id: `spec-wiring-include-${rsRel}-${specLeaf}`,
          summary: `"${rsRel}" does not include_str! sibling spec "${specLeaf}"`,
          kind: "handcrafted-grammar/spec-wiring-include",
          scope: rsRel,
          priority: "high",
          reason: "Facet Rust components must embed their normative .grammar.semio/.protocol.semio via include_str!.",
          solution: `Add include_str!("…/${specLeaf}") to ${rsRel}, then remove it from POLICY_SPEC_WIRING_INCLUDE_EXEMPTIONS.`,
        });
      }
    }
    if (!hasAnySpec) continue;
    if (POLICY_SPEC_WIRING_REGISTER_EXEMPTIONS.has(artRel)) continue;
    if (policyArtifactHasRegisterLanguage(repoRoot, artRel)) continue;
    breaches.push({
      id: `spec-wiring-register-${artRel}`,
      summary: `Artifact "${artRel}" has facet specs but never calls register_language`,
      kind: "handcrafted-grammar/spec-wiring-register",
      scope: artRel,
      priority: "high",
      reason: ("Artifacts that ship grammar/protocol specs must register the language with the shared DSL host (typically in ⚙️engine/" + POLICY_RS_COMPONENT_LEAF_NAME + ")."),
      solution: `Call register_language from the artifact engine (or another artifact .rs), then remove ${artRel} from POLICY_SPEC_WIRING_REGISTER_EXEMPTIONS.`,
    });
  }
  return breaches;
}

/**
 * 📏️ Any asset under `🖼️assets/` inside `📚️examples` must carry real payload (size > 64), covering every
 * kind suffix (`.dsl.semio`, `.op.semio`, `.spr.semio`, `.pack.semio`, `.diff.semio`, `.cmd.semio`) and media.
 */
/**
 * 🕳️Empty/stub `.semio` under `📚️examples/** /🖼️assets/` (all kinds) — size ≤ 64 is a breach.
 * `POLICY_EMPTY_EXAMPLE_EXEMPTIONS` stays empty.
 */
export function policyEmptyExampleBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const assetsDir = taxonomy.exampleAssetsDirName;
  const breaches: BreachRecord[] = [];
  const files = policyWalkRelFiles(repoRoot, [""], (relPath, name) => {
    if (!name.endsWith(".semio")) return false;
    const norm = relPath.replaceAll("\\", "/");
    if (!(norm.includes("/📚️examples/") || norm.startsWith("📚️examples/"))) return false;
    return norm.includes(`/${assetsDir}/`);
  });
  for (const relPath of files) {
    if (POLICY_EMPTY_EXAMPLE_EXEMPTIONS.has(relPath)) continue;
    let size = 0;
    try {
      size = statSync(join(repoRoot, relPath)).size;
    } catch {
      continue;
    }
    if (size > 64) continue;
    breaches.push({
      id: `empty-example-${relPath}`,
      summary: `"${relPath}" is an empty/envelope-only example asset (${size} bytes ≤ 64)`,
      kind: "handcrafted-grammar/empty-example",
      scope: relPath,
      priority: "high",
      reason: "Example `.semio` assets under 🖼️assets/ must include a real payload, not just an empty envelope or stub.",
      solution: `Seed a non-empty example at ${relPath}, then remove it from POLICY_EMPTY_EXAMPLE_EXEMPTIONS.`,
    });
  }
  return breaches;
}

function policyGenericCodecDeriveBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const files = policyWalkRelFiles(repoRoot, ["✏️s/🔌️plugins"], (relPath, name) => {
    return relPath.includes("/🗿️artifacts/") && name.endsWith(".rs");
  });
  const banned = [
    { re: /dsl::__rt::parse_document_record|dsl::__rt::print_document_record|dsl::__rt::parse_inline_record|dsl::__rt::print_inline_record/g, label: "__rt codec wrapper" },
    { re: /dsl::op_rt::|store::op_rt::/g, label: "op_rt generic OpBinary" },
  ];
  for (const relPath of files) {
    const content = policyReadFileSafe(repoRoot, relPath);
    for (const { re, label } of banned) {
      re.lastIndex = 0;
      let match: RegExpExecArray | null;
      while ((match = re.exec(content)) !== null) {
        const before = content.slice(0, match.index);
        const line = before.split(/\r?\n/).length;
        breaches.push({
          id: `generic-codec-runtime-${relPath}-${line}`,
          summary: `Residual generic codec path (${label}) in "${relPath}" (line ${line})`,
          kind: "handcrafted-grammar/generic-codec-derive",
          scope: relPath,
          line,
          priority: "high",
          reason: "P6 deleted derive-emitted DocumentDsl/OpText/DocumentPack/OpBinary and their __rt/op_rt entrypoints; artifacts must use handcrafted codecs.",
          solution: `Replace ${label} usage in ${relPath} with the artifact's handcrafted DocumentDsl/OpText/DocumentPack/OpBinary impl.`,
        });
      }
    }
  }
  return breaches;
}

/** 🔗 Cumulative `#[path]` targets declared by one `🦀️.rs` (absolute resolved paths). */
function policyCollectGluePathTargets(glueAbs: string): Set<string> {
  const declared = new Set<string>();
  if (!existsSync(glueAbs)) return declared;
  const libDir = dirname(glueAbs);
  const libText = readFileSync(glueAbs, "utf8");
  const baseStack: string[] = [libDir];
  let pendingPath: string | null = null;
  for (const rawLine of libText.split(/\r?\n/)) {
    const line = rawLine.trim();
    const pathMatch = line.match(/#\[path\s*=\s*"([^"]+)"\]/);
    if (pathMatch) {
      pendingPath = pathMatch[1] ?? null;
      continue;
    }
    const modMatch = line.match(/^(?:pub\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)/);
    if (modMatch) {
      const modName = modMatch[1]!;
      const base = baseStack[baseStack.length - 1] ?? libDir;
      let resolved: string;
      if (pendingPath === null) {
        resolved = join(base, modName);
      } else if (pendingPath === ".") {
        resolved = base;
      } else {
        resolved = join(base, pendingPath);
      }
      pendingPath = null;
      const asFile = resolved.endsWith(".rs") ? resolved : `${resolved}.rs`;
      const asModFile = join(resolved, "mod.rs");
      if (existsSync(asFile)) declared.add(resolve(asFile));
      else if (existsSync(asModFile)) declared.add(resolve(asModFile));
      else declared.add(resolve(asFile));
      if (line.includes("{")) baseStack.push(resolved.endsWith(".rs") ? dirname(resolved) : resolved);
      continue;
    }
    pendingPath = null;
    const opens = (line.match(/\{/g) ?? []).length;
    const closes = (line.match(/\}/g) ?? []).length;
    for (let i = 0; i < opens; i++) baseStack.push(baseStack[baseStack.length - 1] ?? libDir);
    for (let i = 0; i < closes; i++) {
      if (baseStack.length > 1) baseStack.pop();
    }
  }
  return declared;
}

/**
 * ☠️ Any `.rs` under `📚️examples` must be reachable via `#[path]` from a `🦀️.rs` — dead definition
 * or test shims are forbidden.
 */
export function policyDeadExampleLeafBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const reachable = new Set<string>();
  const owners = [...new Set(crates.map((crate) => crate.ownerRel).filter(Boolean))];
  const glueRoots = owners.length > 0 ? owners : ["✏️s/🔌️plugins"];
  for (const crate of crates) {
    for (const target of policyCollectGluePathTargets(join(repoRoot, crate.libRelPath))) reachable.add(target);
  }
  for (const glueRel of policyWalkRelFiles(repoRoot, glueRoots, (_p, name) => name === "🦀️.rs")) {
    for (const target of policyCollectGluePathTargets(join(repoRoot, glueRel))) reachable.add(target);
  }
  const fw = readdirSync(repoRoot).find((name) => name.endsWith("framework"));
  const exampleRoots = owners.length > 0 ? owners : fw ? ["✏️s/🔌️plugins", fw] : ["✏️s/🔌️plugins"];
  const exampleRs = policyWalkRelFiles(repoRoot, exampleRoots, (relPath, name) => {
    if (!name.endsWith(".rs")) return false;
    return relPath.replaceAll("\\", "/").includes("/📚️examples/");
  });
  for (const relPath of exampleRs) {
    const abs = resolve(join(repoRoot, relPath));
    if (reachable.has(abs)) continue;
    breaches.push({
      id: `dead-example-leaf-${relPath}`,
      summary: `"${relPath}" is not reachable via #[path] from any 🦀️.rs`,
      kind: "taxonomy/dead-example-leaf",
      scope: relPath,
      priority: "high",
      reason: "Every .rs under 📚️examples must be wired from the plugin 🦀️.rs (definition leaf or cfg(test) test leaf).",
      solution: `Add a #[path] mod declaration for ${relPath} under //#region 📚️Examples in the owning 🦀️.rs, or delete the dead file.`,
    });
  }
  return breaches;
}

/** 📕️Aggregates all P3/M4 handcrafted-grammar high-priority scanners for policy + verify gate. */
export function policyHandcraftedSpecP3Breaches(repoRoot: string): BreachRecord[] {
  return [
    ...policySpecDistinctnessBreaches(repoRoot),
    ...policyGenericSpecBreaches(repoRoot),
    ...policyDeclaredUseBreaches(repoRoot),
    ...policySpecWiringBreaches(repoRoot),
    ...policyEmptyExampleBreaches(repoRoot),
    ...policyGenericCodecDeriveBreaches(repoRoot),
  ];
}

/** 📂️Every concrete mutation directory owns one authoritative direct Rust component. */
function policyMutationDirectOwnerBreaches(repoRoot: string, mutationRoots: readonly string[] = policyFindAllMutationsDirs(repoRoot)): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const mutationsRel of mutationRoots) {
    const artRel = policyArtifactRootOfMutationsDir(mutationsRel);
    for (const mutName of policyListMutationDirs(repoRoot, mutationsRel)) {
      const mutRel = `${mutationsRel}/${mutName}`;
      const directRel = `${mutRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
      if (existsSync(join(repoRoot, directRel))) continue;
      breaches.push({
        id: `mutation-direct-owner-${mutRel}`,
        summary: `"${mutRel}" has no direct ${POLICY_RS_COMPONENT_LEAF_NAME}`,
        kind: "mutation/direct-owner",
        scope: artRel,
        priority: "high",
        reason: "Every concrete mutation is represented by exactly one direct semantic folder whose component is authoritative.",
        solution: `Move the concrete implementation to ${directRel}; optional diff, inverse, plan, text, binary, and tests remain child facets.`,
      });
    }
  }
  return breaches;
}

/**
 * 🧩️A direct mutation component owns its semantic descriptor and mutation implementation.
 */
function policyMutationImplPresenceBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const mutationsRel of policyFindAllMutationsDirs(repoRoot)) {
    const artRel = policyArtifactRootOfMutationsDir(mutationsRel);
    for (const mutName of policyListMutationDirs(repoRoot, mutationsRel)) {
      const mutRel = `${mutationsRel}/${mutName}`;
      const rsRel = `${mutRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
      const abs = join(repoRoot, rsRel);
      if (!existsSync(abs)) continue;
      const content = readFileSync(abs, "utf8");
      if (/\bimpl\b[^\n{]*\b(?:Mutation|MutationKind|CompositeMutationKind)\b/.test(content) && /\b(?:SEMANTICS|DESCRIPTOR)\b/.test(content)) continue;
      breaches.push({
        id: `mutation-impl-missing-${rsRel}`,
        summary: `"${rsRel}" does not own both mutation behavior and a semantic descriptor`,
        kind: "mutation/behavior-ownership",
        scope: artRel,
        priority: "high",
        reason: "The direct leaf is the visible source of semantic identity and apply/diff/inverse participation.",
        solution: `Define the leaf payload, descriptor, and MutationKind/Mutation implementation in ${rsRel}.`,
      });
    }
  }
  return breaches;
}

/** ⛓️Plugin roots that may declare runtime dependencies: `✏️s/🔌️plugins/<plugin>` and its `🧩️extensions/<ext>`. */
function policyDependencyOwnerRoots(repoRoot: string): readonly string[] {
  const roots: string[] = [];
  for (const plugin of policyReaddirSafe(repoRoot, "✏️s/🔌️plugins").filter((e) => e.isDirectory && !e.name.startsWith("."))) {
    const pluginRel = `✏️s/🔌️plugins/${plugin.name}`;
    roots.push(pluginRel);
    for (const ext of policyReaddirSafe(repoRoot, `${pluginRel}/🧩️extensions`).filter((e) => e.isDirectory && !e.name.startsWith("."))) {
      roots.push(`${pluginRel}/🧩️extensions/${ext.name}`);
    }
  }
  return roots;
}

/**
 * 🍃️Component leaves that belong to `ownerRel` ITSELF, excluding any nested `🧩️extensions/<ext>/`
 * subtree — an extension is its own dependency owner (it appears separately in
 * [[policyDependencyOwnerRoots]]), so folding its files into its parent plugin's walk made the parent
 * inherit the extension's `.depends_on(...)` and demand a Cargo dependency the parent never needs. A
 * plugin that hosts an extension declaring `.depends_on("cad")` is not itself a dependent of `cad`.
 */
export function policyOwnerOwnComponentFiles(repoRoot: string, ownerRel: string): string[] {
  return policyWalkRelFiles(
    repoRoot,
    [ownerRel],
    (_relPath, name) => name === POLICY_RS_COMPONENT_LEAF_NAME,
    (_relDir, name) => name === "🧩️extensions",
  );
}

/** 🔌️An owner's DECLARED runtime plugin dependencies — `[package.metadata.semio].depends-on` plus,
 * for an extension, its `extends` host (contract freeze §4 rule 1 makes the host `dependsOn[0]`).
 * Deliberately NOT the crate's `[dependencies]`: a `semio-s-plugin-<id>` Cargo line is a build-time
 * rlib link (codecs, schema types) that says nothing about needing `<id>`'s own actor loaded. */
function policySemioMetadataDependsOnIds(repoRoot: string, ownerRel: string): { readonly declared: Set<string>; readonly extendsHost?: string } {
  const manifest = policyReadFileSafe(repoRoot, `${ownerRel}/📦️packages/🦀️rust/Cargo.toml`);
  const semioBlock = manifest.split(/(?=\n\[)/).find((block) => block.trimStart().startsWith("[package.metadata.semio]")) ?? "";
  const extendsHost = semioBlock.match(/^extends\s*=\s*"([a-z0-9-]+)"/m)?.[1];
  const declared = new Set<string>();
  const list = semioBlock.match(/^depends-on\s*=\s*\[([^\]]*)\]/m)?.[1] ?? "";
  for (const match of list.matchAll(/"([a-z0-9-]+)"/g)) declared.add(match[1]!);
  return { declared, extendsHost };
}

/**
 * 🪞️The one declared runtime-dependency set of a plugin has two co-located mirrors that must agree:
 * `.depends_on("x", …)` on the owner's builder (`🦀️.rs`, the wire `PluginManifest.dependencies` the
 * host's load order, version checks and contribution gates read) and
 * `[package.metadata.semio].depends-on` in its Cargo manifest (what the plugin registry generator
 * reads, since it runs before any wasm build could emit a descriptor). Both directions are checked so
 * neither half can drift. An extension's `extends` host counts as declared on the metadata side.
 *
 * 🚫️ A Cargo `[dependencies]` link on a sibling plugin crate is NOT a runtime dependency and is not
 * examined here: deriving the runtime graph from library links pulled `stdio` (linked by ~every
 * plugin purely for in-process codecs) into every browser session's load set.
 */
export function policyPluginDependencyParityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const ownerRel of policyDependencyOwnerRoots(repoRoot)) {
    const declared = new Set<string>();
    for (const relPath of policyOwnerOwnComponentFiles(repoRoot, ownerRel)) {
      for (const match of policyReadFileSafe(repoRoot, relPath).matchAll(/\.depends_on\s*\(\s*"([a-z0-9-]+)"/g)) {
        declared.add(match[1]!);
      }
    }
    const metadata = policySemioMetadataDependsOnIds(repoRoot, ownerRel);
    const ownId = policyStripEmoji(ownerRel.split("/").pop() ?? "");
    for (const id of declared) {
      if (metadata.declared.has(id) || id === metadata.extendsHost) continue;
      breaches.push({
        id: `plugin-dependency-undeclared-metadata-${ownerRel}-${id}`,
        summary: `"${ownerRel}" declares .depends_on("${id}") with no matching [package.metadata.semio].depends-on entry`,
        kind: "plugin-dependency/parity",
        scope: ownerRel,
        priority: "high",
        reason: "The plugin registry is generated from Cargo metadata before any wasm build exists, so a runtime dependency only the builder knows about is missing from every load order and dev session closure.",
        solution: `Add "${id}" to depends-on in ${ownerRel}/📦️packages/🦀️rust/Cargo.toml's [package.metadata.semio].`,
      });
    }
    for (const id of metadata.declared) {
      if (id === ownId || declared.has(id)) continue;
      breaches.push({
        id: `plugin-dependency-undeclared-runtime-${ownerRel}-${id}`,
        summary: `"${ownerRel}" declares depends-on = ["${id}"] without declaring .depends_on("${id}")`,
        kind: "plugin-dependency/parity",
        scope: ownerRel,
        priority: "high",
        reason: "The host builds its load order, version checks and contribution gates from the runtime manifest — a dependency the manifest never mentions is invisible to all three.",
        solution: `Add .depends_on("${id}", …) to the plugin/extension builder in ${ownerRel}.`,
      });
    }
  }
  return breaches;
}

/**
 * 🎯️Every `ArtifactContribution::builder("s.<owner>.<artifact>")` must target an artifact whose
 * owning plugin the contributor declares as a direct dependency — contributing onto a plugin you do
 * not depend on is exactly the unowned-registration this ticket's contract forbids.
 */
export function policyContributionTargetBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const ownerRel of policyDependencyOwnerRoots(repoRoot)) {
    const declared = new Set<string>();
    const targets = new Map<string, string>();
    for (const relPath of policyOwnerOwnComponentFiles(repoRoot, ownerRel)) {
      const content = policyReadFileSafe(repoRoot, relPath);
      for (const match of content.matchAll(/\.depends_on\s*\(\s*"([a-z0-9-]+)"/g)) declared.add(match[1]!);
      for (const match of content.matchAll(/ArtifactContribution::builder\s*\(\s*"([^"]+)"/g)) targets.set(match[1]!, relPath);
    }
    for (const [kind, relPath] of targets) {
      const owner = kind.split(".")[1];
      if (!owner) {
        breaches.push({
          id: `contribution-target-ungrammatical-${relPath}-${kind}`,
          summary: `"${relPath}" contributes to "${kind}", which is not a canonical s.<plugin>.<artifact> kind`,
          kind: "plugin-dependency/contribution-target",
          scope: ownerRel,
          priority: "high",
          reason: "A contribution target must name its owning plugin so the dependency gate can resolve it.",
          solution: `Use the canonical kind id of the target artifact in ${relPath}.`,
        });
        continue;
      }
      if (declared.has(owner)) continue;
      breaches.push({
        id: `contribution-target-undeclared-${ownerRel}-${kind}`,
        summary: `"${ownerRel}" contributes to "${kind}" without declaring .depends_on("${owner}")`,
        kind: "plugin-dependency/contribution-target",
        scope: ownerRel,
        priority: "high",
        reason: "Mutations and inferences may only be registered onto artifacts of a directly declared dependency.",
        solution: `Add .depends_on("${owner}", …) to the plugin/extension builder in ${ownerRel}.`,
      });
    }
  }
  return breaches;
}

/**
 * ▶️Every `🔧️op/*.grammar.semio` must declare `start mutation` (production `mutation =` follows in fan-out).
 */
function policyOpGrammarStartMutationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const grammars = policyWalkRelFiles(repoRoot, ["✏️s"], (relPath, name) => {
    if (!name.endsWith(".grammar.semio")) return false;
    return relPath.replaceAll("\\", "/").includes(`/${POLICY_OP_FACET}/`);
  });
  for (const relPath of grammars) {
    const content = policyReadFileSafe(repoRoot, relPath);
    if (/^start\s+mutation\b/m.test(content)) continue;
    const legacy = /^start\s+operation\b/m.test(content);
    breaches.push({
      id: `op-grammar-start-mutation-${relPath}`,
      summary: legacy
        ? `"${relPath}" still declares start operation — must be start mutation`
        : `"${relPath}" is missing start mutation`,
      kind: "mutation-migration/op-grammar-start",
      scope: relPath,
      priority: "high",
      reason: "The 🔧️op grammar start symbol is the Mutation production (`start mutation`); OpText/OpBinary brand stays.",
      solution: `Change the start line in ${relPath} to "start mutation" (and rename production operation = to mutation =).`,
    });
  }
  return breaches;
}

/** 🏘️ Concrete mutation emojis are compared within their actual sibling owner, including declared domains. */
function policyMutationEmojiUniquenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const mutationsRel of policyFindAllMutationsDirs(repoRoot)) {
    const artRel = policyArtifactRootOfMutationsDir(mutationsRel);
    const seen = new Map<string, string>();
    for (const mutName of policyListMutationDirs(repoRoot, mutationsRel)) {
      const emoji = policyLeadingEmojiPrefix(posix.basename(mutName));
      if (!emoji) {
        breaches.push({
          id: `mutation-emoji-missing-${mutationsRel}/${mutName}`,
          summary: `"${mutationsRel}/${mutName}" has no leading emoji prefix`,
          kind: "mutation-migration/emoji-uniqueness",
          scope: artRel,
          priority: "medium",
          reason: "Each concrete mutation directory must pick a unique emoji among its siblings.",
          solution: `Rename ${mutName} to include a unique emoji prefix (e.g. ➕️objects-add).`,
        });
        continue;
      }
      const siblingKey = `${posix.dirname(mutName)}\0${emoji}`;
      const prev = seen.get(siblingKey);
      if (prev) {
        breaches.push({
          id: `mutation-emoji-dup-${artRel}-${emoji}-${mutName}`,
          summary: `"${mutationsRel}/${mutName}" reuses emoji "${emoji}" already used by "${prev}"`,
          kind: "mutation-migration/emoji-uniqueness",
          scope: artRel,
          priority: "medium",
          reason: "Specific mutation dir emojis must be unique among siblings.",
          solution: `Give ${mutName} a different emoji than ${prev}.`,
        });
        continue;
      }
      seen.set(siblingKey, mutName);
    }
  }
  return breaches;
}

/** 🔍️Every repository-owned `🧬️mutations` facet, with opaque/build/cache roots excluded before access. */
export function policyRepositoryOwnedRoots(): string[] {
  const configured = Object.entries(loadTaxonomy().areas ?? {})
    .filter(([, state]) => state === "clean")
    .map(([area]) => area);
  const candidates = [...new Set(["✏️s", "🧰️framework", "🌎️hub", "♻️mit-bestand", ...configured])]
    .filter((candidate) => candidate !== "compose" && !candidate.startsWith("compose/"));
  return candidates
    .filter((candidate) => !candidates.some((other) => other !== candidate && candidate.startsWith(`${other}/`)))
    .sort();
}

/**
 * 🗺️SEMANTIC-MUTATIONS-OVERHAUL rule 1: every repository-owned `.rs` file under a
 * `🧬️mutations/` facet or a `🎮️commands/` app-command dir must not reference the banned generic mutation
 * vocabulary the new `SemanticDescriptor`/`MutationKind`/`#[derive(dsl_derive::Mutations)]` mechanism
 * (see `🧰️framework/…/📡️spr/🎮️command/🦀️.rs`'s `🔖️Semantics` region) replaces: `SetSnapshot`
 * (the whole-document escape hatch), `NoMutation` (the sentinel sibling), and `CollectionMutation<`/`::`
 * (the generic collection wrapper, worst-offender count ~70 in one facet). `Set[A-Z]\w*` dispatch-enum
 * variant names are additionally flagged at `"medium"`; the generic escape hatches have no
 * suppression or exception mechanism.
 */
/**
 * 🚫️High-priority banned generic-vocabulary tokens: the whole-document escape hatch, the sentinel, and the
 * generic collection wrapper. Deliberately substring (no `\b` word boundary) matching the
 * seeding grep exactly, so derived type names like `SetSnapshotDiff`/`SetSnapshotMutation` still count —
 * they're just as much banned vocabulary as the bare identifier.
 */
const POLICY_SEMANTIC_VOCABULARY_HIGH_TOKENS: readonly { label: string; re: RegExp }[] = [
  { label: "SetSnapshot", re: /SetSnapshot/ },
  { label: "NoMutation", re: /NoMutation/ },
  { label: "CollectionMutation<...>", re: /CollectionMutation(<|::)/ },
];

/** 🏷️Every `.rs` file under a `🧬️mutations/` facet or a `🎮️commands/` app-command dir, repo-wide. */
export function policyListSemanticVocabularyScanFiles(repoRoot: string): string[] {
  return policyWalkRelFiles(repoRoot, policyRepositoryOwnedRoots(), (relPath, name) => {
    if (!name.endsWith(".rs")) return false;
    const norm = relPath.replaceAll("\\", "/");
    return norm.includes(`/${POLICY_MUTATIONS_FACET}/`) || norm.includes("/🎮️commands/");
  });
}

export function policySemanticVocabularyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyListSemanticVocabularyScanFiles(repoRoot)) {
    const content = policyReadFileSafe(repoRoot, relPath);
    const hitTokens = POLICY_SEMANTIC_VOCABULARY_HIGH_TOKENS.filter((t) => t.re.test(content)).map((t) => t.label);
    if (hitTokens.length > 0) {
      breaches.push({
        id: `semantic-vocabulary-banned-${relPath}`,
        summary: `"${relPath}" references banned generic mutation vocabulary: ${hitTokens.join(", ")}`,
        kind: "mutation-migration/semantic-vocabulary",
        scope: relPath,
        priority: "high",
        reason: "SetSnapshot/NoMutation/CollectionMutation are generic escape hatches replaced by explicit direct semantic mutations.",
        solution: `Replace ${hitTokens.join(", ")} in ${relPath} with explicit semantic MutationKind payloads.`,
      });
    }
    const bareSetVariants = [...new Set([...content.matchAll(/\bSet[A-Z]\w*/g)].map((m) => m[0]))].filter((name) => name !== "SetSnapshot");
    if (bareSetVariants.length > 0) {
      breaches.push({
        id: `semantic-vocabulary-bare-set-${relPath}`,
        summary: `"${relPath}" declares ${bareSetVariants.length} bare Set* identifier(s): ${bareSetVariants.slice(0, 6).join(", ")}${bareSetVariants.length > 6 ? ", …" : ""}`,
        kind: "mutation-migration/semantic-vocabulary",
        scope: relPath,
        priority: "medium",
        reason: "Set* is the generic verb this overhaul retires in favor of the closed APPROVED_VERBS vocabulary (rename/change/move/flatten/connect/group/…) — advisory (unallowlisted) until the fan-out wave rewrites dispatch enums.",
        solution: `Rename the Set* variant(s) in ${relPath} to a semantic verb-entity name backed by a MutationKind payload.`,
      });
    }
  }
  return breaches;
}

function policyMutationDispatchCoverageBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const mutationsRel of policyFindAllMutationsDirs(repoRoot)) {
    const dispatchRel = `${mutationsRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
    if (!existsSync(join(repoRoot, dispatchRel))) continue;
    const variantNames = new Set(policyMutationEnumVariantNames(policyReadFileSafe(repoRoot, dispatchRel)));
    if (variantNames.size === 0) continue;
    const triadPascalNames = new Set(policyListMutationDirs(repoRoot, mutationsRel).map((dir) => policyKebabToPascal(policyMutationSemanticIdentity(mutationsRel, dir))));
    const uncoveredVariants = [...variantNames].filter((v) => !triadPascalNames.has(v));
    const orphanTriads = [...triadPascalNames].filter((t) => !variantNames.has(t));
    if (uncoveredVariants.length === 0 && orphanTriads.length === 0) continue;
    breaches.push({
      id: `mutation-dispatch-coverage-${dispatchRel}`,
      summary: `"${dispatchRel}" dispatch variants and ${mutationsRel} triad dirs disagree: ${uncoveredVariants.length} variant(s) with no triad dir, ${orphanTriads.length} triad dir(s) with no variant`,
      kind: "mutation-migration/dispatch-coverage",
      scope: mutationsRel,
      priority: "medium",
      reason: "Once the fan-out wave adopts #[derive(dsl_derive::Mutations)], every dispatch variant must be a single-field tuple wrapping exactly one triad dir's MutationKind payload — advisory until then.",
      solution: `Reconcile ${dispatchRel}'s enum variants with the triad dirs under ${mutationsRel} (rename to match, or add/remove triad dirs).`,
    });
  }
  return breaches;
}

/**
 * 🦜️SEMANTIC-MUTATIONS-OVERHAUL rule 3 (`policyMutationTsMirrorBreaches`): flags any `.ts` leaf anywhere
 * under a `🧬️mutations/` facet whose content is trivially `export {};` (or empty) — near-universal today
 * (triad `.ts` leaves were scaffolded as stubs, see `policyIsConstitutionalTsFacadePath`'s explicit tolerance for this exact shape),
 * so this stays `"low"` advisory with no allowlist rather than seeding ~1000+ file paths for a finding that
 * blocks nothing; it exists so a real TS mirror rollout has a tracked signal to burn down file-by-file.
 */
function policyMutationTsMirrorBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const tsFiles = policyWalkRelFiles(repoRoot, ["✏️s"], (relPath, name) => name === POLICY_TS_COMPONENT_LEAF && relPath.replaceAll("\\", "/").includes(`/${POLICY_MUTATIONS_FACET}/`));
  for (const relPath of tsFiles) {
    const stripped = policyReadFileSafe(repoRoot, relPath)
      .replace(/\/\*[\s\S]*?\*\//g, "")
      .replace(/\/\/.*$/gm, "")
      .trim();
    if (stripped !== "" && stripped !== "export {};") continue;
    breaches.push({
      id: `mutation-ts-mirror-stub-${relPath}`,
      summary: `"${relPath}" is a trivial "export {};" stub with no real TS mirror`,
      kind: "mutation-migration/ts-mirror",
      scope: relPath,
      priority: "low",
      reason: "A triad leaf's TS mirror should eventually re-export the same MutationKind payload shape as its Rust sibling, not stay an empty stub.",
      solution: `Give ${relPath} real content mirroring its ${POLICY_RS_COMPONENT_LEAF_NAME} sibling once the DSL TS codegen for this triad lands.`,
    });
  }
  for (const rsRel of policyWalkRelFiles(repoRoot, ["✏️s"], (relPath, name) => name === POLICY_RS_COMPONENT_LEAF_NAME && relPath.replaceAll("\\", "/").includes(`/${POLICY_MUTATIONS_FACET}/`))) {
    const tsRel = `${rsRel.slice(0, rsRel.lastIndexOf("/"))}/${POLICY_TS_COMPONENT_LEAF}`;
    if (existsSync(join(repoRoot, tsRel))) continue;
    breaches.push({
      id: `mutation-ts-mirror-absent-${tsRel}`,
      summary: `"${rsRel}" has no ${POLICY_TS_COMPONENT_LEAF} mirror beside it at all`,
      kind: "mutation-migration/ts-mirror",
      scope: tsRel,
      priority: "low",
      reason: "An ABSENT mirror is invisible to the stub scan above, so a facet that never scaffolded its .ts leaves looked cleaner than one that did — this half closes that blind spot.",
      solution: `Create ${tsRel} mirroring its ${POLICY_RS_COMPONENT_LEAF_NAME} sibling's MutationKind payload shape.`,
    });
  }
  return breaches;
}
//#endregion 🧬️DirectMutationPolicies

/**
 * 🎮️App command folders must be one semantically named command (verb-noun kebab), matching
 * `🧬️mutations/<emoji><verb>-<noun>/`. Noun buckets (`✍️text`, `🗂️selection`, `🗣️locale`) are
 * architecture violations. Held at `"medium"` while ticket `26/08/13/SEMANTIC-COMMAND-NAMES`
 * fans out; graduate to `"high"` once every plugin leaf is 1:1.
 */
const POLICY_COMMAND_VERBS = new Set<string>([
  "create", "delete", "insert", "remove", "add", "rename", "change", "update", "move",
  "drag", "resize", "rotate", "scale", "reorder", "edit", "replace", "duplicate",
  "connect", "disconnect", "bind", "unbind", "group", "ungroup", "flatten", "unflatten",
  "split", "merge", "extract", "inline", "clear", "fix", "toggle", "apply", "set",
  "format", "open", "load", "save", "import", "export", "lint", "request", "select",
  "hover", "evaluate", "run", "retry", "calibrate", "place", "commit", "input",
  "submit", "abort", "patch", "paint", "nudge", "reset", "focus", "step", "query",
  "fill", "snap", "ingest", "evaluate",
]);

function policyCommandFolderSemanticBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const walk = (relDir: string): void => {
    for (const ent of policyReaddirSafe(repoRoot, relDir)) {
      if (!ent.isDirectory || ent.name.startsWith(".")) continue;
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.name === "🎮️commands") {
        for (const cmd of policyReaddirSafe(repoRoot, childRel)) {
          if (!cmd.isDirectory) continue;
          const folder = cmd.name;
          const emoji = policyLeadingEmojiPrefix(folder);
          const slug = emoji ? folder.slice(emoji.length) : folder;
          const verb = slug.split("-")[0] ?? slug;
          if (POLICY_COMMAND_VERBS.has(verb)) continue;
          breaches.push({
            id: `command-folder-semantic-${childRel}/${folder}`,
            summary: `"${childRel}/${folder}" is a noun bucket, not a semantic command`,
            kind: "taxonomy/command-folder-semantic",
            scope: childRel,
            priority: "medium",
            reason: "Each 🎮️commands/ leaf must be one command named like a mutation (verb-noun kebab, e.g. 📝️text-edit). Grouping under domain nouns (text, selection, locale, camera) is banned.",
            solution: `Split ${childRel}/${folder} into one folder per command using an approved verb prefix (${[...POLICY_COMMAND_VERBS].slice(0, 8).join(", ")}, …).`,
          });
        }
        continue;
      }
      if (ent.name === "target" || ent.name === "node_modules") continue;
      walk(childRel);
    }
  };
  walk("✏️s");
  return breaches;
}

/** ⚙️Aggregates Wave 2b mutation / ArtifactEngine / op-grammar scanners. */
export function policyMutationArtifactEngineBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyMutationStructuralBreaches(repoRoot),
    ...policyOpGrammarStartMutationBreaches(repoRoot),
    ...policyMutationEmojiUniquenessBreaches(repoRoot),
    ...policySemanticVocabularyBreaches(repoRoot),
    ...policyCommandFolderSemanticBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleMutationArtifactEngines
