/** 🧭️ Wave 4 plugin consistency policy: schema overhaul phase 2 pilot-closer rules.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts
 */
import { type BreachRecord } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { policyReadFileSafe, policyReaddirSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME, policyStripEmoji } from "../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { POLICY_STDIO_ARTIFACTS_REL } from "../🚪️artifact-io/🟦️.ts";
import { policyAllRustFiles, policyExtractFnBody, policyNormalizeRelPath } from "../🏛️foundation/🟦️.ts";
import { POLICY_PROTOCOL_SPEC_LEAF } from "../✍️handcrafted-mutation/🟦️.ts";
import { policyListStdioSchemaOwningEntries, policyStdioStandardKey } from "../🧬️schema-overhaul-s2/🟦️.ts";

//#region 🔧️PolicyRuleSchemaOverhaulPC
/**
 * 🎫️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, Phase 2 PC (pilot
 * closer wave, after M1/M2/M3's mechanism spine + the P1(json/csv)/P2(zip/png)/P3(txt/binary) pilot
 * ladder). Four new shrink-only rules tracking the Phase 2 program's own "real dialect" migration —
 * same allowlist semantics as the S2 rules above (`policyGrammarHonestyBreaches` et al.): a "new
 * breach" fires when a file/standard fails the check and isn't allowlisted (an unexpected regression
 * or an undiscovered gap); a "stale" low-priority breach fires when an allowlisted entry has ALREADY
 * been fixed, so cleanup is never silently forgotten. Every allowlist below was seeded by running the
 * exact detection logic against the real tree at PC time (not hand-guessed) — see `p2-pc-report.md`
 * in the ticket folder for the exact commands/counts. These are heuristic, textual checks (matching
 * every other rule in this file, e.g. `policyGrammarHonestyBreaches`'s own placeholder-marker scan) —
 * the REAL enforcement for grammar/protocol conformance is each artifact's own
 * `committed_facet_files_parse`/`grammar_conformance_law`/`protocol_walk_law` tests plus the
 * framework's `STDIO_CONFORMANCE_GRADUATED` graduation list in `🧪️fixture/🦀️-sweep.rs`.
 */

/**
 * 📏️`POLICY_GRAMMAR_PARSEABILITY`: every stdio `📖️.grammar.semio` file should look like the
 * REAL M1/M2-era dialect (`dialect grammar` on its own header line, `grammar <id>`, `start <prod>`)
 * and not still carry an old-dialect/ABNF tell (a `;`-prefixed comment line, `%xHH` character-class
 * syntax, or a bare `*hexdig`/`*OCTET` prefix-repetition placeholder — none of which this dialect's
 * lexer/parser accepts). Textual heuristic only (`looksLikeRealGrammarOrProtocolDialect` below), not a
 * real parse — matching the house style documented above. Seeded with the current census: json/csv/
 * zip/png/txt/binary (the P1-P3 pilot ladder) are OUT; every other stdio standard's grammar leaves are
 * still IN (old fossil headers or unconverted ABNF bodies) — including a few standards this heuristic
 * ALSO already finds header-conformant by accident (`stdio/pdf#1.7` has a contract-correct header
 * but an untouched ABNF body, correctly caught as still-fossil by the ABNF-tell check; `stdio/semio#v1`'s
 * `✳️object` subset — a live, unrelated concurrent ticket's WIP artifact, not part of this program's
 * roster — happens to already pass both checks, so it is honestly NOT in this seed; see the PC report).
 */

/** 🔎️Shared textual heuristic for `POLICY_GRAMMAR_PARSEABILITY`/`POLICY_PROTOCOL_PARSEABILITY`: does this
 * `.grammar.semio`/`.protocol.semio` file look like the real M1/M2 dialect (own-line `dialect grammar`/
 * `dialect protocol` header directive, an `<kind> <id>` line, a `start <production>` line) with no
 * leftover old-dialect/ABNF tell (`#`-comment lines are stripped first, since a real file's own doc
 * comments legitimately quote the old ABNF spec in prose — e.g. csv's grammar cites RFC 4180's own
 * `%x20-21` char-class range inside a `#` comment, which must not trip this check). Not a real parse —
 * see this region's own doc comment for why a heuristic is sufficient here.
 */
function policyLooksLikeRealGrammarOrProtocolDialect(content: string, kind: "grammar" | "protocol"): boolean {
  const lines = content.split(/\r?\n/);
  const codeLines = lines.filter((l) => !l.trim().startsWith("#"));
  const headerWord = kind === "grammar" ? "dialect grammar" : "dialect protocol";
  const idWord = kind === "grammar" ? "grammar" : "protocol";
  const idRe = new RegExp(`^${idWord}\\s+\\S+`);
  const hasDialectLine = lines.some((l) => l.trim() === headerWord);
  const hasIdLine = lines.some((l) => idRe.test(l.trim()));
  const hasStartLine = lines.some((l) => /^start\s+\S+/.test(l.trim()));
  const code = codeLines.join("\n");
  const hasAbnfTell = codeLines.some((l) => l.trim().startsWith(";")) || /%x[0-9A-Fa-f]/.test(code) || code.includes("*hexdig") || code.includes("*OCTET");
  return hasDialectLine && hasIdLine && hasStartLine && !hasAbnfTell;
}

const POLICY_GRAMMAR_PARSEABILITY_FACETS = ["📸️snapshot", "🔺️diff", "🧬️mutations"] as const;

const POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST = new Set<string>([
  "stdio/avi/standards#1.0-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/avi/standards#1.0-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/avi/standards#1.0-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/epw/standards#energyplus-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/epw/standards#energyplus-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/epw/standards#energyplus-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/html/standards#5-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/html/standards#5-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/html/standards#5-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/mp3/standards#mpeg1-layer3-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/mp3/standards#mpeg1-layer3-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/mp3/standards#mpeg1-layer3-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/mp4/standards#isobmff-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/mp4/standards#isobmff-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/mp4/standards#isobmff-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-animation-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-animation-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-animation-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-audio-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-audio-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-audio-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-brep-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-brep-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-brep-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-cad-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-cad-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-cad-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-document-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-document-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-document-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-drawing-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-drawing-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-drawing-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-image-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-image-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-image-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-mesh-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-mesh-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-mesh-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-model-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-model-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-model-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-presentation-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-presentation-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-presentation-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-video-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-video-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-video-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-workflow-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-workflow-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-workflow-schema-snapshot-text-component.grammar.semio",
  "stdio/tsv/standards#iana-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/tsv/standards#iana-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/tsv/standards#iana-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/wav/standards#riff-pcm-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/wav/standards#riff-pcm-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/wav/standards#riff-pcm-subsets-any-schema-snapshot-text-component.grammar.semio",
]);

function policyGrammarParseabilityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const entry of policyListStdioSchemaOwningEntries(repoRoot)) {
    for (const facet of POLICY_GRAMMAR_PARSEABILITY_FACETS) {
      const rel = `${entry.subsetRel}/🧬️schema/${facet}/📝️text/📖️.grammar.semio`;
      if (!existsSync(join(repoRoot, rel))) continue;
      const content = policyReadFileSafe(repoRoot, rel);
      const looksReal = policyLooksLikeRealGrammarOrProtocolDialect(content, "grammar");
      const normalized = policyNormalizeRelPath(rel);
      const allowlisted = POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST.has(normalized);
      if (!looksReal) {
        if (allowlisted) continue;
        breaches.push({
          id: `grammar-parseability-${rel}`,
          summary: `"${rel}" does not look like the real grammar dialect (dialect grammar / grammar <id> / start <prod>, no leftover ABNF tell)`,
          kind: "stdio-artifacts/grammar-parseability",
          scope: entry.artRel,
          priority: "medium",
          reason: "Phase 2's own mandate: every stdio grammar leaf must be handcrafted in the REAL parse_grammar dialect the repo's Recognizer actually compiles, not an unparseable fossil header or an unconverted ABNF body (see 📖️phase2-design.md and p2-w0-recon-report.md's parseability census).",
          solution: `Rewrite ${rel} in the real dialect per 📖️grammar-recipe.md, or if this standard hasn't reached its FG-wave yet, add "${normalized}" to POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST citing this ticket.`,
        });
      } else if (allowlisted) {
        breaches.push({
          id: `grammar-parseability-stale-${rel}`,
          summary: `"${rel}" is allowlisted in POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST but already looks like the real dialect`,
          kind: "stdio-artifacts/grammar-parseability",
          scope: entry.artRel,
          priority: "low",
          reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
          solution: `Remove "${normalized}" from POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 📏️`POLICY_PROTOCOL_PARSEABILITY`: same heuristic, for `📡️.protocol.semio` files. Seeded
 * with the current census: json/csv/zip/png/txt/binary OUT; every other stdio standard IN (same
 * `stdio/semio#v1/subsets#object` accidental-pass caveat as the grammar rule above applies here too).
 */
const POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST = new Set<string>([
  "stdio/avi/standards#1.0-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/avi/standards#1.0-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/avi/standards#1.0-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/epw/standards#energyplus-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/epw/standards#energyplus-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/epw/standards#energyplus-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/html/standards#5-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/html/standards#5-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/html/standards#5-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/mp3/standards#mpeg1-layer3-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/mp3/standards#mpeg1-layer3-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/mp3/standards#mpeg1-layer3-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/mp4/standards#isobmff-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/mp4/standards#isobmff-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/mp4/standards#isobmff-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-animation-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-animation-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-animation-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-audio-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-audio-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-audio-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-brep-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-brep-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-brep-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-cad-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-cad-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-cad-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-document-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-document-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-document-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-drawing-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-drawing-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-drawing-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-image-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-image-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-image-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-mesh-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-mesh-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-mesh-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-model-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-model-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-model-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-presentation-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-presentation-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-presentation-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-video-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-video-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-video-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-workflow-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-workflow-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-workflow-schema-snapshot-binary-component.protocol.semio",
  "stdio/tsv/standards#iana-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/tsv/standards#iana-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/tsv/standards#iana-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/wav/standards#riff-pcm-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/wav/standards#riff-pcm-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/wav/standards#riff-pcm-subsets-any-schema-snapshot-binary-component.protocol.semio",
]);

function policyProtocolParseabilityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const entry of policyListStdioSchemaOwningEntries(repoRoot)) {
    for (const facet of POLICY_GRAMMAR_PARSEABILITY_FACETS) {
      const rel = `${entry.subsetRel}/🧬️schema/${facet}/💾️binary/${POLICY_PROTOCOL_SPEC_LEAF}`;
      if (!existsSync(join(repoRoot, rel))) continue;
      const content = policyReadFileSafe(repoRoot, rel);
      const looksReal = policyLooksLikeRealGrammarOrProtocolDialect(content, "protocol");
      const normalized = policyNormalizeRelPath(rel);
      const allowlisted = POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST.has(normalized);
      if (!looksReal) {
        if (allowlisted) continue;
        breaches.push({
          id: `protocol-parseability-${rel}`,
          summary: `"${rel}" does not look like the real protocol dialect (dialect protocol / protocol <id> / start <block>, no leftover ABNF tell)`,
          kind: "stdio-artifacts/protocol-parseability",
          scope: entry.artRel,
          priority: "medium",
          reason: "Phase 2's own mandate: every stdio protocol leaf must be handcrafted in the REAL parse_protocol dialect walk_protocol actually walks, not an unparseable fossil header or an unconverted ABNF body (see 📖️phase2-design.md and p2-w0-recon-report.md's parseability census).",
          solution: `Rewrite ${rel} in the real dialect per 📖️grammar-recipe.md, or if this standard hasn't reached its FG-wave yet, add "${normalized}" to POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST citing this ticket.`,
        });
      } else if (allowlisted) {
        breaches.push({
          id: `protocol-parseability-stale-${rel}`,
          summary: `"${rel}" is allowlisted in POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST but already looks like the real dialect`,
          kind: "stdio-artifacts/protocol-parseability",
          scope: entry.artRel,
          priority: "low",
          reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
          solution: `Remove "${normalized}" from POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 📏️`POLICY_FIXTURE_HONESTY`: every stdio ARTIFACT's (not per-standard — the demo fixture pair lives
 * once per artifact dir, shared across a multi-standard artifact like gif 87a/89a) `🗣️.dsl.semio`
 * must start with a genuine `semio stdio.<artifact>`-prefixed preamble line (not a Phase-1-era fake like
 * `{"hello":"stdio.xml","n":1}` with no preamble at all), AND a sibling `🎒️.pack.semio` must
 * exist on disk. Seeded with the current census: the 6 piloted artifacts (binary/csv/json/png/txt/zip)
 * are OUT; every other stdio artifact is still IN (including a stray, content-less `🧬️schema` dir
 * directly under `🗿️artifacts/` — not a real artifact, harmless to seed, will self-resolve as a stale
 * entry if that debris is ever cleaned up).
 */
const POLICY_FIXTURE_HONESTY_ALLOWLIST = new Set<string>([
  "stdio/avi",
  "stdio/epw",
  "stdio/html",
  "stdio/mp3",
  "stdio/mp4",
  "stdio/schema",
  "stdio/semio",
  "stdio/tsv",
  "stdio/wav",
]);

function policyStdioArtifactKey(artifactId: string): string {
  return `stdio/${artifactId}`;
}

function policyFixtureHonestyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const art of policyReaddirSafe(repoRoot, POLICY_STDIO_ARTIFACTS_REL)) {
    if (!art.isDirectory) continue;
    const artRel = `${POLICY_STDIO_ARTIFACTS_REL}/${art.name}`;
    const artifactId = policyStripEmoji(art.name);
    const dslRel = `${artRel}/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`;
    const packRel = `${artRel}/📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio`;
    const dslAbs = join(repoRoot, dslRel);
    const dslOk = existsSync(dslAbs) && readFileSync(dslAbs, "utf8").split(/\r?\n/, 1)[0]!.trim().startsWith(`semio stdio.${artifactId}`);
    const packOk = existsSync(join(repoRoot, packRel));
    const key = policyStdioArtifactKey(artifactId);
    const allowlisted = POLICY_FIXTURE_HONESTY_ALLOWLIST.has(key);
    const honest = dslOk && packOk;
    if (!honest) {
      if (allowlisted) continue;
      breaches.push({
        id: `fixture-honesty-${artRel}`,
        summary: `"${artRel}" fixtures are not genuine (dsl.semio preamble ok=${dslOk}, pack.semio present=${packOk})`,
        kind: "stdio-artifacts/fixture-honesty",
        scope: artRel,
        priority: "medium",
        reason: "Phase 2's own mandate: 🗣️.dsl.semio must be the genuine print_dsl output (with its mandatory `semio stdio.<artifact>...` preamble line, not a Phase-1-era fake) and a genuine 🎒️.pack.semio (real encode_pack bytes) must exist alongside it (see 📖️phase2-design.md's per-standard deliverable list).",
        solution: `Regenerate ${dslRel}/${packRel} from the real print_dsl/encode_pack output, or if this artifact hasn't reached its FG-wave yet, add "${key}" to POLICY_FIXTURE_HONESTY_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `fixture-honesty-stale-${artRel}`,
        summary: `"${artRel}" is allowlisted in POLICY_FIXTURE_HONESTY_ALLOWLIST but its fixtures are already genuine`,
        kind: "stdio-artifacts/fixture-honesty",
        scope: artRel,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${key}" from POLICY_FIXTURE_HONESTY_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️`POLICY_LANGUAGE_REGISTRATION`: every stdio (artifact, standard)'s `⚙️engine::register()` must call
 * `dsl::register_language` at least 5 times — the full 5-role `LanguageSpec` registration (Document/
 * Ops/Diff/Pack/Spr) every P1-P3 pilot landed, per note's exemplar pattern (see 📖️grammar-recipe.md).
 * Plain grep count on the standard's own `⚙️engine/🦀️.rs`, matching this file's other
 * grep-count-based rules. Seeded with the current census: the 6 piloted standards (5 calls each) are
 * OUT; every other stdio standard is still IN (0-1 calls — the pre-Phase-2 single-role registration,
 * or none at all).
 */
const POLICY_LANGUAGE_REGISTRATION_MIN_CALLS = 5;
const POLICY_LANGUAGE_REGISTRATION_ALLOWLIST = new Set<string>([
  "stdio/avi/standards#1.0",
  "stdio/epw/standards#energyplus",
  "stdio/html/standards#5",
  // 🎓️ P2-PW: jpg/jfif-1.01's real gap (0 of 5 register_language calls, flagged by P2-FG2's closer)
  // was fixed by the dedicated FG2-fix wave (p2-fg2-fix-jpg-report.md) — jpg now genuinely registers
  // 5/5 languages, confirmed by direct grep. Removed below along with every other now-real FG-wave
  // standard; kept here only as the historical pointer for why this allowlist once carried jpg.
  "stdio/mp3/standards#mpeg1-layer3",
  "stdio/mp4/standards#isobmff",
  "stdio/semio/standards#v1",
  "stdio/tsv/standards#iana",
  "stdio/wav/standards#riff-pcm",
]);

function policyLanguageRegistrationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const seenStandards = new Set<string>();
  for (const entry of policyListStdioSchemaOwningEntries(repoRoot)) {
    const standardRel = entry.subsetRel.split("/🪆️subsets/")[0]!;
    const key = policyStdioStandardKey(entry.artifactId, entry.standardSlug);
    if (seenStandards.has(key)) continue; // 🔒 one check per (artifact, standard), not per schema-owning subset
    seenStandards.add(key);
    const engineRel = `${standardRel}/⚙️engine/${POLICY_RS_COMPONENT_LEAF_NAME}`;
    const count = existsSync(join(repoRoot, engineRel)) ? (policyReadFileSafe(repoRoot, engineRel).match(/register_language/g) ?? []).length : 0;
    const allowlisted = POLICY_LANGUAGE_REGISTRATION_ALLOWLIST.has(key);
    const ok = count >= POLICY_LANGUAGE_REGISTRATION_MIN_CALLS;
    if (!ok) {
      if (allowlisted) continue;
      breaches.push({
        id: `language-registration-${key}`,
        summary: `"${engineRel}" calls register_language ${count} time(s), fewer than the required ${POLICY_LANGUAGE_REGISTRATION_MIN_CALLS} (5-role LanguageSpec set)`,
        kind: "stdio-artifacts/language-registration",
        scope: entry.artRel,
        priority: "medium",
        reason: "Phase 2's own mandate: every standard registers the full 5-role LanguageSpec (Document/Ops/Diff/Pack/Spr), per note's exemplar and every P1-P3 pilot's own register_pilot_languages() (see 📖️grammar-recipe.md's registration checklist).",
        solution: `Add the missing dsl::register_language roles to ${engineRel}'s register_pilot_languages(), or if this standard hasn't reached its FG-wave yet, add "${key}" to POLICY_LANGUAGE_REGISTRATION_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `language-registration-stale-${key}`,
        summary: `"${engineRel}" is allowlisted in POLICY_LANGUAGE_REGISTRATION_ALLOWLIST but already registers ${count} >= ${POLICY_LANGUAGE_REGISTRATION_MIN_CALLS} languages`,
        kind: "stdio-artifacts/language-registration",
        scope: entry.artRel,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${key}" from POLICY_LANGUAGE_REGISTRATION_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️`POLICY_STDIO_JSON_TRANSFER_BAN`: no stdio artifact's `ArtifactPack`/`OpBinary`/`DiffCodec` impl
 * block may use `serde_json::to_vec(`/`serde_json::from_slice(` as its transfer mechanism (brace-
 * matched scan of each `impl (...::)?(ArtifactPack|OpBinary|DiffCodec) for ... { ... }` block's own
 * body — deliberately narrower than a whole-file grep, so an artifact's legitimate NATIVE json
 * parsing elsewhere in the same file, e.g. gltf's own `⚙️engine`, is never a false positive). A second,
 * narrower check covers the one real cross-artifact bridge surface W0's census found that isn't
 * literally one of those three impls: any `.rs` file under a `🚪️io/` (import/export bridge) dir using
 * a `serde_json::{to_vec,from_slice,to_string,from_str}(` call (gltf's json-snapshot deserializer
 * bridge). Per-standard `#diff`/op/pack facets legitimately implementing these traits with REAL binary
 * (`dsl::ByteWriter`/`pack_rt`/`dsl::variants_binary`) are unaffected — only the literal serde_json
 * call inside the impl body trips this. Seeded with the current census (run fresh at PC time, per
 * this rule's own mandate to confirm rather than assume): W0's originally-named 4 (ifc/2x3's
 * mutations OpBinary, svg's and xml's snapshot ArtifactPack, gltf's io bridge) are all still real
 * violations, unfixed by the pilot ladder (none of those 4 standards were in the P1-P3 roster) —
 * plus a real, larger set of additional violations from a separate, currently-live concurrent ticket
 * scaffolding new stdio artifact types (avi/mp3/mp4/wav's mutations OpBinary, and most of 🧿️semio
 * v1's many subsets' snapshot ArtifactPack / mutations OpBinary) — included honestly since this
 * policy's detection logic was run for real against the current tree, not limited to the original 4.
 */
const POLICY_STDIO_JSON_TRANSFER_BAN_TRAIT_RE = /impl(?:<[^>]*>)?\s+(?:[\w:]+::)?(ArtifactPack|OpBinary|DiffCodec)[^{]*for/g;

/** 🔎️First of `ArtifactPack`/`OpBinary`/`DiffCodec` whose own `impl ... for ... { ... }` block body contains a literal `serde_json::to_vec(`/`serde_json::from_slice(` call, or `undefined` if none. */
function policyStdioJsonTransferTraitHit(content: string): string | undefined {
  POLICY_STDIO_JSON_TRANSFER_BAN_TRAIT_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_STDIO_JSON_TRANSFER_BAN_TRAIT_RE.exec(content))) {
    const block = policyExtractFnBody(content, m.index);
    if (block.includes("serde_json::to_vec(") || block.includes("serde_json::from_slice(")) return m[1];
  }
  return undefined;
}

const POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST = new Set<string>([
  "stdio/avi/standards#1.0-subsets-any-schema-mutations-component",
  // 🎓️ P2-PW: the live `serde_json::to_vec(&from.value)` transfer call this entry was seeded for was
  // genuinely replaced by FG3 (reusing json's own real text codec, p2-fg3-closer-report.md) — direct
  // read confirms zero live serde_json calls remain in this file. Stays allowlisted only because the
  // checker's substring scan (by design, not stripping comments — see this rule's own doc comment)
  // still matches the SAME string appearing inside this file's own historical doc comment describing
  // what the old code used to do ("was a literal `serde_json::to_vec(&from.value)` JSON..."). A known
  // checker limitation, not a real violation and not a rule-logic change; left in place rather than
  // removed to avoid a false "medium" breach at the next policy run.
  "stdio/gltf/standards#2.0-subsets-any-io-import-deserializers-artifacts-json-rfc8259-any-component",
  "stdio/mp3/standards#mpeg1-layer3-subsets-any-schema-mutations-component",
  "stdio/mp4/standards#isobmff-subsets-any-schema-mutations-component",
  "stdio/semio/standards#v1-subsets-animation-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-any-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-audio-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-brep-schema-mutations-component",
  "stdio/semio/standards#v1-subsets-brep-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-cad-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-document-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-drawing-schema-mutations-component",
  "stdio/semio/standards#v1-subsets-drawing-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-image-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-mesh-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-model-schema-mutations-component",
  "stdio/semio/standards#v1-subsets-model-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-object-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-presentation-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-video-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-workflow-schema-snapshot-component",
  "stdio/wav/standards#riff-pcm-subsets-any-schema-mutations-component",
]);

function policyStdioJsonTransferBanBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const prefix = `${POLICY_STDIO_ARTIFACTS_REL}/`;
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.startsWith(prefix)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    const traitHit = policyStdioJsonTransferTraitHit(content);
    const ioHit =
      relPath.includes("🚪️io") &&
      (content.includes("serde_json::to_vec(") || content.includes("serde_json::from_slice(") || content.includes("serde_json::to_string(") || content.includes("serde_json::from_str("));
    const hit = traitHit ?? (ioHit ? "io-bridge" : undefined);
    const normalized = policyNormalizeRelPath(relPath);
    const allowlisted = POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST.has(normalized);
    if (hit) {
      if (allowlisted) continue;
      breaches.push({
        id: `stdio-json-transfer-ban-${relPath}`,
        summary: `"${relPath}" uses serde_json (${hit}) as an ArtifactPack/OpBinary/DiffCodec transfer mechanism`,
        kind: "stdio-artifacts/json-transfer-ban",
        scope: relPath,
        priority: "medium",
        reason: "Phase 2 decision 4 (no JSON / no serde on any transfer path): stdio artifact transfer paths must be real binary (dsl::ByteWriter/pack_rt/dsl::variants_binary) or DSL text, never a literal serde_json round trip disguised as a binary pack.",
        solution: `Replace the serde_json call in ${relPath}'s transfer impl with a real binary/DSL-text encoding, or if this standard hasn't reached its FG-wave yet, add "${normalized}" to POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `stdio-json-transfer-ban-stale-${relPath}`,
        summary: `"${relPath}" is allowlisted in POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST but no longer uses serde_json as its transfer mechanism`,
        kind: "stdio-artifacts/json-transfer-ban",
        scope: relPath,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${normalized}" from POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/** ⚖️Aggregates PC's four new parseability/fixture-honesty/registration/json-transfer-ban rules. */
export function policySchemaOverhaulPCBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyGrammarParseabilityBreaches(repoRoot),
    ...policyProtocolParseabilityBreaches(repoRoot),
    ...policyFixtureHonestyBreaches(repoRoot),
    ...policyLanguageRegistrationBreaches(repoRoot),
    ...policyStdioJsonTransferBanBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleSchemaOverhaulPC
