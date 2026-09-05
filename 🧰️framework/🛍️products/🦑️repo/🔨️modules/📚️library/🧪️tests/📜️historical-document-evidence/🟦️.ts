import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import * as discovery from "../../🔍️discovery/🟦️.ts";
import * as normalization from "../../🧹️normalization/🟦️.ts";

const libraryRoot = resolve(import.meta.dir, "../.."), root = resolve(libraryRoot, "../../../../..");
const historicalDocumentEvidence = Reflect.get(normalization, "historicalDocumentEvidence") as (path: string, taxonomy: unknown, repoRoot: string) => boolean;

test("historicalDocumentEvidence and its schema are exported, registered and self-valid", () => {
  expect(typeof historicalDocumentEvidence).toBe("function");
  const schema = discovery.loadCatalogTaxonomy();
  expect(Object.keys(schema.historicalDocumentEvidencePopulations)).toEqual(["ticket-report", "ticket-workspace", "cursor-plan-snapshot", "dev-prompt-log"]);
  expect(schema.referenceClosure.historicalDocumentEvidence).toBe("ticket-report-workspace-cursor-plan-snapshot-and-dev-prompt-log-whole-document-excluded");
  expect(discovery.validateTaxonomy(schema)).toEqual([]);
});

test("validateHistoricalDocumentEvidencePopulations rejects every declared mutation and accepts the real registration", () => {
  const base = discovery.loadCatalogTaxonomy().historicalDocumentEvidencePopulations;
  expect(discovery.validateHistoricalDocumentEvidencePopulations(structuredClone(base))).toEqual([]);
  const mutations: ((value: Record<string, any>) => void)[] = [
    (value) => { delete value["cursor-plan-snapshot"]; },
    (value) => { delete value["ticket-workspace"]; },
    (value) => { delete value["dev-prompt-log"]; },
    (value) => { value["extra-population"] = structuredClone(value["cursor-plan-snapshot"]); },
    (value) => { value["dev-prompt-log"].directoryPattern = "**/wrong/**"; },
    (value) => { value["dev-prompt-log"].leafPattern = "^.+$"; },
    (value) => { value["ticket-report"].grammar = "wrong"; },
    (value) => { value["ticket-report"].directoryPattern = "**/wrong/**"; },
    (value) => { value["ticket-report"].leafPattern = "^📓️.md$"; },
    (value) => { value["ticket-workspace"].directoryPattern = "**/wrong/*"; },
    (value) => { value["ticket-workspace"].leafPattern = "^📓️.+\\.md$"; },
    (value) => { delete value["cursor-plan-snapshot"].reason; },
    (value) => { value["cursor-plan-snapshot"].extra = true; },
    (value) => { value["cursor-plan-snapshot"].leafPattern = "("; },
    (value) => { value["ticket-report"] = "not-an-object"; },
  ];
  for (const mutate of mutations) {
    const clone = structuredClone(base);
    mutate(clone);
    expect(discovery.validateHistoricalDocumentEvidencePopulations(clone).length, JSON.stringify(clone)).toBeGreaterThan(0);
  }
});

test("historicalDocumentEvidence matches the exact bounded population against the real repository, regardless of owning-ticket status, and never overrides a real machine-read contract", () => {
  const taxonomy = { discoverySchema: discovery.loadCatalogTaxonomy(), pathMatcher: discovery.createTaxonomyPathMatcher() };
  const cases: readonly (readonly [string, boolean, string])[] = [
    [".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w2-schema-api.md", true, "closed ticket, slugged 📓️ report"],
    [".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️goal-session-status.md", true, "the active (open) ticket's own report is exempt too — kind, not lifecycle, is the discriminator"],
    [".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/OS-EXCLUSIVE-STATE-AUTHORITY/🧪m3-plugin-component.pre-patch.rs", true, "a ticket-evidence Rust snapshot at ticket-root level"],
    [".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/terra-number-deasync.py", true, "a loose scratch script at ticket-root level"],
    [".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📌️important.md", true, "a ticket working note — excluded as a reference SOURCE only; ticket_close's emptiness check reads entry.size, never this file's content"],
    [".cursor/plans/2d_references_on_grid_ff798114.plan.md", true, "Cursor plan snapshot"],
    [".cursor/plans/not-a-plan.md", false, "a .cursor/plans file that is not a *.plan.md snapshot stays live"],
    ["🧰️framework/🔨️modules/📚️library/🔣️taxonomy.json", false, "production schema stays live"],
    [".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/🎫️ticket.json", false, "the ticket manifest itself matches a fixedFilenameContracts pattern (ticket-manifest) and is never exempted"],
    [".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS/Cargo.toml", false, "a ticket-embedded package's own manifest matches cargo-manifest and is never exempted"],
    [".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS/derive-dwg-fixture.c", false, "a loose C source sibling of a real ticket-root Cargo.toml stays live — the whole directory is package-owned"],
    [".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS/generate_w1_a_gltf_create_scene.mjs", false, "same package-owned directory — a build script sibling also stays live"],
    [".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS/📓️w1-gltf-create-scene-frozen-audit.md", true, "a real 📓️ narrative report sitting in that SAME package-owned directory, beside the very Cargo.toml that keeps its .c/.mjs siblings live, is still exempt — document kind, not proximity to a manifest, is the discriminator; prose cannot open() a file"],
    [".🧬semio/🦑️repo/💬️prompts/🐙️ueli.md", true, "a developer's own prompt-log transcript"],
    [".🧬semio/🦑️repo/💬️prompts/🐳️kinan.md", true, "a different developer's prompt-log transcript — the directory, not the filename, is the population"],
    [".🧬semio/🦑️repo/💬️prompts/notes.txt", false, "the population's leaf grammar requires .md — a non-markdown file under the same directory stays live"],
  ];
  for (const [path, expected, label] of cases) expect(historicalDocumentEvidence(path, taxonomy, root), label).toBe(expected);
});

test("the ticket-report leaf pattern requires a non-empty slug; the ticket-workspace directory pattern admits a ticket-root descendant at any nesting depth", () => {
  const populations = discovery.loadCatalogTaxonomy().historicalDocumentEvidencePopulations;
  const reportLeaf = new RegExp(populations["ticket-report"]!.leafPattern, "u");
  expect(reportLeaf.test("📓️.md")).toBe(false);
  expect(reportLeaf.test("📓️report.md")).toBe(true);
  const taxonomy = { discoverySchema: discovery.loadCatalogTaxonomy(), pathMatcher: discovery.createTaxonomyPathMatcher() };
  const matches = (p: string) => taxonomy.pathMatcher.matches(p, populations["ticket-workspace"]!.directoryPattern);
  expect(matches(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/terra-number-deasync.py")).toBe(true);
  expect(matches(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/nested/terra-number-deasync.py")).toBe(true);
  expect(matches(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/nested/deeper/still/terra-number-deasync.py")).toBe(true);
  expect(matches(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME")).toBe(false);
});

/** 🧫️ Builds one isolated, real Git fixture repository under this ticket's own 🗑️temp/ so the proof runs the actual `planTaxonomy` engine, not a mock. */
function fixture() {
  const parent = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🗑️temp/📜️historical-document-evidence-runs");
  mkdirSync(parent, { recursive: true });
  const repoRoot = mkdtempSync(join(parent, "fixture-"));
  const put = (path: string, bytes: string) => { const target = join(repoRoot, path); mkdirSync(dirname(target), { recursive: true }); writeFileSync(target, bytes); };
  const git = (...args: string[]) => { const result = Bun.spawnSync(["git", ...args], { cwd: repoRoot, stdout: "pipe", stderr: "pipe" }); expect(result.exitCode, result.stderr.toString()).toBe(0); return result.stdout.toString().trim(); };
  return { repoRoot, put, git };
}

test("ticket narrative, evidence and scratch never block a move, a production file still does, and a ticket-embedded package boundary is never swept in", () => {
  const { repoRoot, put, git } = fixture();
  const scope = "🧪️tests/🧪️fixture", source = `${scope}/🦀️.rs`, final = `${scope}/🦀️.rs`;
  const prose = (owner: string) => `Earlier the component lived at ${source} before ${owner} moved it.\n`;

  const closedTicketDir = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/HISTREF-CLOSED-FIXTURE";
  const openTicketDir = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/HISTREF-OPEN-FIXTURE";
  const noManifestTicketDir = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/HISTREF-NO-MANIFEST-FIXTURE";
  const closedReport = `${closedTicketDir}/📓️report.md`, openReport = `${openTicketDir}/📓️report.md`, noManifestReport = `${noManifestTicketDir}/📓️report.md`;
  const evidenceSnapshot = `${closedTicketDir}/🧪scratch.pre-patch.rs`;
  const looseScratch = `${closedTicketDir}/loose-scratch.txt`;
  const workingNote = `${closedTicketDir}/📌️note.md`;
  const cursorPlan = ".cursor/plans/histref_fixture_test_deadbeef.plan.md";
  const productionDoc = "🧪️tests/📖️histref-production-neighbornotes.md";
  const embeddedManifest = `${closedTicketDir}/embedded-pkg/Cargo.toml`;
  const embeddedSource = `${closedTicketDir}/embedded-pkg/lib.rs`;
  const embeddedReport = `${closedTicketDir}/embedded-pkg/📓️note.md`;
  const schemaPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";

  const schema = structuredClone(discovery.loadCatalogTaxonomy()) as Record<string, any>;
  delete schema.generatorContracts["plugin-registry"].inputDiscovery;
  put(schemaPath, `${JSON.stringify(schema, null, 2)}\n`);
  put(source, "pub fn value() -> u32 { 7 }\n");
  put(`${closedTicketDir}/🎫️ticket.json`, `${JSON.stringify({ title: "Histref Closed Fixture", status: "closed", description: "fixture" }, null, 2)}\n`);
  put(closedReport, prose(closedTicketDir));
  put(evidenceSnapshot, prose(closedTicketDir));
  put(looseScratch, prose(closedTicketDir));
  put(workingNote, prose(closedTicketDir));
  put(`${openTicketDir}/🎫️ticket.json`, `${JSON.stringify({ title: "Histref Open Fixture", status: "open", description: "fixture" }, null, 2)}\n`);
  put(openReport, prose(openTicketDir));
  put(noManifestReport, prose(noManifestTicketDir));
  put(cursorPlan, prose(".cursor/plans"));
  put(productionDoc, prose(scope));
  put(embeddedManifest, '[package]\nname = "histref-embedded"\nversion = "0.1.0"\n');
  put(embeddedSource, prose(`${closedTicketDir}/embedded-pkg`));
  put(embeddedReport, prose(`${closedTicketDir}/embedded-pkg`));

  git("init", "-q");
  put(".git/info/exclude", `${schemaPath}\n`);
  git("add", "--all");
  git("-c", "user.name=Histref Fixture", "-c", "user.email=fixture@invalid.example", "-c", "commit.gpgsign=false", "commit", "-qm", "Histref fixture");
  const baselineCommit = git("rev-parse", "HEAD");

  const plan = normalization.planTaxonomy(normalization.inventoryTaxonomy({ repoRoot, scope, workers: 1 }), { baselineCommit, excludedTreeDigests: [] });

  expect(plan.moves.map((move) => [move.sourcePath, move.destinationPath])).toEqual([[source, final]]);
  expect(plan.regenerations).toEqual([]);
  expect(plan.edits).toEqual([]);

  const blocked = new Set(plan.unresolved.map((row) => row.path));
  expect(blocked.has(closedReport), "closed-ticket report must not block the move").toBe(false);
  expect(blocked.has(openReport), "open-ticket report must not block the move either — document kind is the discriminator, not lifecycle status").toBe(false);
  expect(blocked.has(noManifestReport), "a report whose ticket has no manifest at all must not block the move").toBe(false);
  expect(blocked.has(evidenceSnapshot), "a ticket evidence snapshot must not block the move").toBe(false);
  expect(blocked.has(looseScratch), "a loose ticket-root scratch file must not block the move").toBe(false);
  expect(blocked.has(workingNote), "a ticket working note must not block the move").toBe(false);
  expect(blocked.has(cursorPlan), "Cursor plan snapshot must not block the move").toBe(false);
  expect(blocked.has(productionDoc), "a production file must still block the move").toBe(true);
  expect(blocked.has(embeddedSource), "a source file inside a ticket-embedded package boundary must still block the move — never swept in as scratch").toBe(true);
  expect(blocked.has(embeddedReport), "a 📓️ narrative report sitting INSIDE the same ticket-embedded package boundary — beside embeddedSource, under the very manifest that keeps embeddedSource live — must still not block the move: document kind, not proximity to a manifest, is the discriminator").toBe(false);
  for (const path of [productionDoc, embeddedSource]) {
    const rows = plan.unresolved.filter((row) => row.path === path);
    expect(rows.length, path).toBeGreaterThan(0);
    for (const row of rows) {
      expect(row.code).toBe("reference-syntax-unsupported");
      expect(row.message).toContain("unsupported-path-syntax");
      expect(row.message).toContain(source);
    }
  }
  console.log("[DEBUG] Historical document evidence fixture", JSON.stringify({ repoRoot, unresolved: plan.unresolved.length, moves: plan.moves.length, blocked: [...blocked] }));
  rmSync(repoRoot, { recursive: true, force: true });
}, 20_000);

test("the exemption is driven entirely by the schema population, not hardcoded: an empty population map reproduces the pre-change blocked state for the exact same paths", () => {
  const taxonomy = { discoverySchema: { historicalDocumentEvidencePopulations: {}, fixedFilenameContracts: {} }, pathMatcher: discovery.createTaxonomyPathMatcher() };
  const openReportPath = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️goal-session-status.md";
  const cursorPlanPath = ".cursor/plans/2d_references_on_grid_ff798114.plan.md";
  const promptLogPath = ".🧬semio/🦑️repo/💬️prompts/🐙️ueli.md";
  expect(historicalDocumentEvidence(openReportPath, taxonomy, root), "before: no population means even a ticket report is a live reference").toBe(false);
  expect(historicalDocumentEvidence(cursorPlanPath, taxonomy, root), "before: no population means a Cursor plan snapshot is a live reference too").toBe(false);
  expect(historicalDocumentEvidence(promptLogPath, taxonomy, root), "before: no population means a prompt-log transcript is a live reference too").toBe(false);
  const real = { discoverySchema: discovery.loadCatalogTaxonomy(), pathMatcher: discovery.createTaxonomyPathMatcher() };
  expect(historicalDocumentEvidence(openReportPath, real, root), "after: the real registration exempts it").toBe(true);
  expect(historicalDocumentEvidence(cursorPlanPath, real, root), "after: the real registration exempts it").toBe(true);
  expect(historicalDocumentEvidence(promptLogPath, real, root), "after: the real registration exempts it").toBe(true);
});

/** 🧫️ Builds an isolated, git-free directory tree — `historicalDocumentEvidence`'s package-boundary check only calls `readdirSync`, so no repository is needed. */
function looseFixture() {
  const parent = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🗑️temp/📜️historical-document-evidence-runs");
  mkdirSync(parent, { recursive: true });
  const repoRoot = mkdtempSync(join(parent, "prompt-fixture-"));
  const put = (path: string, bytes: string) => { const target = join(repoRoot, path); mkdirSync(dirname(target), { recursive: true }); writeFileSync(target, bytes); };
  return { repoRoot, put };
}

test("dev-prompt-log honors both existing negatives: a fixedFilenameContracts match under 💬️prompts/ is never exempted, and neither is a path inside a hypothetical package embedded directly under it", () => {
  const real = discovery.loadCatalogTaxonomy();
  const pathMatcher = discovery.createTaxonomyPathMatcher();

  // Negative 1: a synthetic fixedFilenameContracts rule matching an .md leaf under 💬️prompts/ must still block — the guard reads the schema generically, so this proves the mechanism without depending on any real contract happening to be markdown-shaped today.
  const withMdContract = { discoverySchema: { ...real, fixedFilenameContracts: { ...real.fixedFilenameContracts, "fixture-prompt-manifest": { pathPattern: "**/.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md", scope: { kind: "single-file" }, grammar: "fixed-filename-v1" } } }, pathMatcher };
  expect(historicalDocumentEvidence(".🧬semio/🦑️repo/💬️prompts/🐙️ueli.md", withMdContract, root), "a fixedFilenameContracts match is never exempted, even under 💬️prompts/").toBe(false);
  expect(historicalDocumentEvidence(".🧬semio/🦑️repo/💬️prompts/🐳️kinan.md", withMdContract, root), "an unrelated sibling under the same directory is unaffected").toBe(true);

  // Negative 2: a real Cargo.toml embedded directly under a fixture 💬️prompts/ directory must disqualify its sibling .md from the exemption — mirrors the ticket-embedded-package proof, generalized to the prompt-log boundary root.
  const { repoRoot, put } = looseFixture();
  put(".🧬semio/🦑️repo/💬️prompts/embedded-pkg/Cargo.toml", '[package]\nname = "prompt-fixture-embedded"\nversion = "0.1.0"\n');
  put(".🧬semio/🦑️repo/💬️prompts/embedded-pkg/notes.md", "narrative that happens to sit inside a real package boundary\n");
  put(".🧬semio/🦑️repo/💬️prompts/🐙️ueli.md", "ordinary prompt-log transcript, no package boundary above it\n");
  const taxonomy = { discoverySchema: real, pathMatcher };
  expect(historicalDocumentEvidence(".🧬semio/🦑️repo/💬️prompts/embedded-pkg/notes.md", taxonomy, repoRoot), "a doc inside a package boundary embedded under 💬️prompts/ must not be exempted").toBe(false);
  expect(historicalDocumentEvidence(".🧬semio/🦑️repo/💬️prompts/🐙️ueli.md", taxonomy, repoRoot), "an ordinary prompt log with no package boundary above it stays exempted").toBe(true);
  rmSync(repoRoot, { recursive: true, force: true });
});
