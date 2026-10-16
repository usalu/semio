/** 🦀️ Rust source evidence readers: production/test source pairs, store roots and comment stripping.
 * @see 📜️script.ts
 */
import { memberStoreProducerSourceDeclarationV1, type ToolJobMemberStoreProducerSources, type MemberProducerSourceExtension } from "../../../../../../💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🏭️member/📥️source/🟦️.ts";
import { inspectRustModuleGraphFacts, inspectRustStructure } from "../../🟦️.ts";
import { POLICY_SPLIT_SOURCE_ROOTS, policyModuleTreeFiles, policyReadFileSafe, policyReadModuleTreeSource } from "../🟦️.ts";
import { isAbsolute, posix, sep } from "node:path";
import { INTERACTIVITY_AUDIT_UI_RECONCILE_CURSOR_FILE, INTERACTIVITY_AUDIT_UI_RECONCILE_FILE, INTERACTIVITY_AUDIT_UI_RECONCILE_PRIMITIVES_FILE } from "../../../⚖️laws/🔬️interactivity-audit/🗂️files/🟦️.ts";

type PolicyRustSourceEntry = Readonly<{ path: string; source: string }>;
export type PolicyRustSourceEvidence = Readonly<{ production: PolicyRustSourceEntry; tests: readonly PolicyRustSourceEntry[] }>;

export function policyRustProductionSource(evidence: string | PolicyRustSourceEvidence): string {
  return typeof evidence === "string" ? evidence : evidence.production.source;
}

export function policyRustTestSource(evidence: string | PolicyRustSourceEvidence): string {
  return typeof evidence === "string" ? evidence : evidence.tests.map((entry) => entry.source).join("\n");
}

/**
 * 🏪️ The os Store's production source: its root plus the vocabulary, ledger, codec and composition modules wave E1 decomposed
 * that root into (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/📓️2026-10-13-wave-e1-os-kernel.md`).
 * Store-wide policies read them as one text, so they scan exactly the production source the single root file held.
 */
export const OS_STORE_ROOT_SOURCES = [
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/♻️retirement/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️command/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/📜️space-history/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/📝️text/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🦀️.rs",
] as const;

/** 🏭️ Binds exact domain-owned member producer bodies and this workspace’s explicit semio extension. */
export function policyReadMemberStoreProducerSources(repoRoot:string):ToolJobMemberStoreProducerSources{
  const read=(reference:Readonly<{path:string;symbols:readonly string[]}>)=>policyReadRustSourceEvidence(repoRoot,reference.path).production;
  const extensionPath="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/📥️source/🔣️.json";
  const extension=JSON.parse(policyReadFileSafe(repoRoot,extensionPath)) as MemberProducerSourceExtension["declaration"];
  const roles=Object.fromEntries(Object.entries(memberStoreProducerSourceDeclarationV1.roles).map(([role,reference])=>[role,read(reference)])) as Omit<ToolJobMemberStoreProducerSources,"extensions">;
  const store=policyReadRustSourceEvidence(repoRoot,memberStoreProducerSourceDeclarationV1.roles.store.path);
  if(!store.tests.some(test=>test.path===memberStoreProducerSourceDeclarationV1.roles.storeLaws.path))throw new Error("[member producer evidence] missing declared canonical store laws");
  const lawSource=roles.storeLaws as {path:string;source:string};
  const catalogPath=memberStoreProducerSourceDeclarationV1.roles.catalogLaws.path;
  if(!inspectRustStructure(lawSource.source).includes.some(item=>item.macro==="include"&&!item.usesOutDir&&(()=>{try{return posix.normalize(posix.join(posix.dirname(lawSource.path),JSON.parse(item.expression)))===catalogPath;}catch{return false;}})()))throw new Error("[member producer evidence] missing declared canonical catalog laws");
  return {...roles,extensions:[{declaration:extension,producer:read(extension.roles.producer),members:read(extension.roles.members)}]};
}

export function policyReadOsStoreSource(repoRoot: string): string {
  return OS_STORE_ROOT_SOURCES.map((path) => policyReadFileSafe(repoRoot, path)).join("\n");
}

/** 🏪️ Rust evidence of the os Store: the decomposed production source of {@link OS_STORE_ROOT_SOURCES} plus the root's declared canonical tests. */
export function policyReadOsStoreEvidence(repoRoot: string): PolicyRustSourceEvidence {
  const evidence = policyReadRustSourceEvidence(repoRoot, OS_STORE_ROOT_SOURCES[0]);
  return { production: { path: evidence.production.path, source: policyReadOsStoreSource(repoRoot) }, tests: evidence.tests };
}

export function policyReadRustSourceEvidence(repoRoot: string, sourcePath: string): PolicyRustSourceEvidence {
  const normalizedSourcePath = sourcePath.split(sep).join("/");
  if (isAbsolute(sourcePath) || normalizedSourcePath.split("/").includes("..")) throw new Error(`[policy rust evidence] invalid production path: ${sourcePath}`);
  const source = policyReadFileSafe(repoRoot, normalizedSourcePath);
  if (!source) throw new Error(`[policy rust evidence] production source missing: ${normalizedSourcePath}`);
  const tests: PolicyRustSourceEntry[] = [];
  const split = POLICY_SPLIT_SOURCE_ROOTS.includes(normalizedSourcePath);
  for (const file of split ? policyModuleTreeFiles(repoRoot, normalizedSourcePath) : [normalizedSourcePath]) {
    const owner = posix.dirname(file);
    for (const module of inspectRustModuleGraphFacts(file === normalizedSourcePath ? source : policyReadFileSafe(repoRoot, file)).modules) {
      if (module.inline || !module.conditional || module.pathTarget === null) continue;
      const target = posix.normalize(posix.join(owner, module.pathTarget));
      const segments = target.split("/");
      const testsAt = segments.lastIndexOf("🧪️tests");
      if (testsAt < 0) continue;
      if (testsAt !== segments.length - 3 || segments.at(-1) !== "🦀️.rs" || !segments.at(-2)) {
        throw new Error(`[policy rust evidence] conditional external module ${module.modulePath.join("::")} is not canonical: ${target}`);
      }
      if (tests.some((entry) => entry.path === target)) continue;
      const targetSource = policyReadFileSafe(repoRoot, target);
      if (!targetSource) throw new Error(`[policy rust evidence] declared canonical test source missing: ${target}`);
      tests.push({ path: target, source: POLICY_SPLIT_SOURCE_ROOTS.includes(target) ? policyReadModuleTreeSource(repoRoot, target) : targetSource });
    }
  }
  return { production: { path: normalizedSourcePath, source: split ? policyReadModuleTreeSource(repoRoot, normalizedSourcePath) : source }, tests };
}

export function policyMutateRustSourceEvidence(evidence: PolicyRustSourceEvidence, from: string, to: string, last = false): PolicyRustSourceEvidence {
  const entries = [evidence.production, ...evidence.tests];
  const matches = entries.flatMap((entry, entryIndex) => {
    const index = last ? entry.source.lastIndexOf(from) : entry.source.indexOf(from);
    return index < 0 ? [] : [{ entryIndex, index }];
  });
  const match = matches[0];
  if (matches.length !== 1 || match === undefined) throw new Error(`[policy rust evidence] mutation source ${matches.length === 0 ? "missing" : "ambiguous"}: ${from}`);
  const { entryIndex, index } = match;
  const changed = (entry: PolicyRustSourceEvidence["production"], candidateIndex: number) => candidateIndex === entryIndex ? { ...entry, source: entry.source.slice(0, index) + to + entry.source.slice(index + from.length) } : entry;
  return { production: changed(evidence.production, 0), tests: evidence.tests.map((entry, candidateIndex) => changed(entry, candidateIndex + 1)) };
}

export const POLICY_RUST_TEST_EVIDENCE_BOUNDARY = "\n//#region 🧪️DeclaredCanonicalTestEvidence:";

/** ♻️ The live reconcile as the one text the unsplit module held: the authority, cursor and primitives crate roots with their mounted children. */
export function policyReadUiReconcileSource(repoRoot: string): string {
  return [INTERACTIVITY_AUDIT_UI_RECONCILE_FILE, INTERACTIVITY_AUDIT_UI_RECONCILE_CURSOR_FILE, INTERACTIVITY_AUDIT_UI_RECONCILE_PRIMITIVES_FILE].map((file) => policyReadRustPolicySource(repoRoot, file)).join("\n");
}

/** 🧩️ Source of a crate family: the production source of every crate root of one carved db module first, then the test evidence of all of them. */
export function policyReadRustPolicyFamily(repoRoot: string, roots: readonly string[]): string {
  const evidence = roots.map((root) => policyReadRustSourceEvidence(repoRoot, root));
  return evidence.map((entry) => entry.production.source).join("\n") + evidence.flatMap((entry) => entry.tests).map((entry) => `${POLICY_RUST_TEST_EVIDENCE_BOUNDARY}${entry.path}\n${entry.source}`).join("");
}

export function policyReadRustPolicySource(repoRoot: string, sourcePath: string): string {
  const source = policyReadFileSafe(repoRoot, sourcePath);
  if (!source) return source;
  if (posix.extname(sourcePath) !== ".rs") return POLICY_SPLIT_SOURCE_ROOTS.includes(sourcePath) ? policyReadModuleTreeSource(repoRoot, sourcePath) : source;
  const evidence = policyReadRustSourceEvidence(repoRoot, sourcePath);
  return evidence.production.source + evidence.tests.map((entry) => `${POLICY_RUST_TEST_EVIDENCE_BOUNDARY}${entry.path}\n${entry.source}`).join("");
}

/**
 * 🎫 Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 1: strips
 * `//` line comments, `/* *​/` block comments (nesting-aware -- Rust block comments nest), and
 * double-quoted string literal bodies (raw strings `r#"..."#` are NOT unescaped -- a false-negative
 * there is far cheaper than the false-positives block/line comments would otherwise cause) so
 * `impl <Trait> for` detection only ever sees real code, never a trait name mentioned in a doc
 * comment or a string. Replaces the old `body.includes(traitName)` substring check, which a doc
 * comment merely NAMING the trait (e.g. "still needs ArtifactBuilder") would satisfy without a
 * single real `impl` block existing.
 */
export function policyStripRustCommentsAndStrings(content: string): string {
  let out = "";
  let i = 0;
  const n = content.length;
  while (i < n) {
    const two = content.slice(i, i + 2);
    if (two === "//") {
      while (i < n && content[i] !== "\n") i++;
      continue;
    }
    if (two === "/*") {
      i += 2;
      let depth = 1;
      while (i < n && depth > 0) {
        const pair = content.slice(i, i + 2);
        if (pair === "/*") { depth++; i += 2; continue; }
        if (pair === "*/") { depth--; i += 2; continue; }
        i++;
      }
      continue;
    }
    if (content[i] === '"') {
      out += '"';
      i++;
      while (i < n && content[i] !== '"') {
        if (content[i] === "\\") { i += 2; continue; }
        i++;
      }
      if (i < n) i++;
      out += '"';
      continue;
    }
    if (content[i] === "'") {
      // 🩹 Rust char literal (`'"'`, `'{'`, `'\n'`, `'\''`, `'\u{1F600}'`, …) vs a lifetime/generic
      // tick (`'a`, `'static`) — a char literal always closes with another `'` within a few chars
      // (accounting for escapes); a lifetime tick never does. Without this, a char literal whose
      // content is itself `'"'` gets misread as the START of a double-quoted string by the block
      // above, silently swallowing everything up to the next literal `"` in the file — which can
      // hide real code (e.g. a later `impl Trait for ...`) from every regex-based policy check
      // that runs against the stripped output.
      let j = i + 1;
      if (content[j] === "\\") {
        j += 1;
        if (content[j] === "u" && content[j + 1] === "{") {
          j += 2;
          while (j < n && content[j] !== "}") j++;
          j += 1;
        } else {
          j += content[j] === "x" ? 3 : 1;
        }
      } else if (j < n) {
        j += 1;
      }
      if (content[j] === "'") {
        out += content.slice(i, j + 1);
        i = j + 1;
        continue;
      }
      // Not a char literal -- a lifetime/generic tick. Copy just the tick through; the identifier
      // that follows contains no quote characters, so falling through the loop handles it safely.
      out += content[i];
      i++;
      continue;
    }
    out += content[i];
    i++;
  }
  return out;
}
