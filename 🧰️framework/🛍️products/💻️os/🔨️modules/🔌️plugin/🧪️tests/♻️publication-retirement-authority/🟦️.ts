import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

/** ♻️ Third-party twin of the publication-retirement fixture: every lane declares the same retirement law, and only window-transient forgives a rejected authority. */
export function testPublicationRetirementAuthorityOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/♻️publication-retirement-authority/🔣️.json", import.meta.url), "utf8"));
  assert.equal(fixture.schema, "framework.plugin.publication-retirement-authority.v1");
  assert.equal(fixture.law.incompleteRetirementTurnIsOk, true);
  assert.equal(fixture.law.rejectionSurfacesOnceAtTheTerminalTurn, true);
  assert.equal(fixture.law.terminalTurnRequiresTerminalEmptiness, true);
  assert.ok(fixture.law.minimumIncompleteTurnsWhileRejected >= 1);

  type Lane = { id: string; lane: string; storeLabel: string; rejectionIsFatal: boolean; terminalOutcome: string; rejectedFault: string | null; falseTerminalFault: string };
  const lanes = fixture.lanes as Lane[];
  assert.equal(new Set(lanes.map((row) => row.id)).size, lanes.length, "every lane id is distinct");
  assert.equal(new Set(lanes.map((row) => row.storeLabel)).size, lanes.length, "every store label is distinct");

  for (const row of lanes) {
    assert.equal(row.falseTerminalFault, `${row.storeLabel} publication closed without terminal emptiness`, `${row.id} false-terminal wording`);
    assert.equal(row.rejectionIsFatal, row.terminalOutcome === "rejected", `${row.id} fatality matches its terminal outcome`);
    assert.equal(row.rejectedFault, row.rejectionIsFatal ? `${row.storeLabel} publication rejected stale or cancelled authority` : null, `${row.id} rejection wording`);
    assert.ok(!row.falseTerminalFault.includes("is retiring a rejected authority"), `${row.id} never re-raises the per-turn fault`);
  }
  const lenient = lanes.filter((row) => !row.rejectionIsFatal);
  assert.deepEqual(lenient.map((row) => row.lane), ["WindowTransient"], "window-transient is the one lenient lane");

  const reBegin = fixture.windowTransientReBegin;
  assert.equal(reBegin.staleAuthorityBeginAccepted, false, "the authority captured at admission is stale once the superseding write lands");
  assert.equal(reBegin.refreshedAuthorityBeginAccepted, true, "refreshing the authority re-admits the same window");
  assert.equal(reBegin.refreshedGenerationExceedsCaptured, true);
  assert.ok(reBegin.capturedRevision < reBegin.supersedingRevision && reBegin.supersedingRevision < reBegin.rejectedRevision && reBegin.rejectedRevision < reBegin.reBeginRevision, "the re-begin fixture revisions are ordered");
}

/** 🧮️ Checks publication progress against the independent SQL case expression and parses its Rust owners. */
export async function testPublicationProgressOracle(): Promise<void> {
  const { Database } = await import("bun:sqlite");
  const { default: Parser } = await import("web-tree-sitter");
  const { dirname, join } = await import("node:path");
  const { fileURLToPath } = await import("node:url");
  const cases = JSON.parse(readFileSync(new URL("./progress.json", import.meta.url), "utf8"));
  const db = new Database(":memory:");
  try {
    const increment = db.query("SELECT ? + CASE WHEN ? = 'publishing' AND (? > 0 OR ? > 0) THEN 1 ELSE 0 END AS progress");
    for (const row of cases) assert.deepEqual(increment.get(row.before, row.stage, row.items, row.released), { progress: row.after }, row.id);
  } finally {
    db.close();
  }

  await Parser.init({ locateFile: () => join(dirname(fileURLToPath(import.meta.resolve("web-tree-sitter"))), "tree-sitter.wasm") });
  const parser = new Parser();
  parser.setLanguage(await Parser.Language.load(join(dirname(fileURLToPath(import.meta.resolve("tree-sitter-wasms/package.json"))), "out/tree-sitter-rust.wasm")));
  try {
    const sources = [
      "../../🧵️retained-command/🔄️full-operation/🦀️.rs",
      "../../🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs",
      "../../🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🏃️mounted-run-and-pump/🦀️.rs",
      "../../🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs",
      "../../🧵️instance/🏪️stores/📥️staging/🧾️typed-operations/🦀️.rs",
      "./🦀️.rs",
    ];
    for (const path of sources) {
      const tree = parser.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
      try {
        assert.deepEqual(tree.rootNode.descendantsOfType("ERROR").map(node => node.startPosition), [], path);
      } finally {
        tree.delete();
      }
    }
  } finally {
    parser.delete();
  }
}

/** ♻️ Checks that the mounted publisher pays for Store closure before another Store advance. */
export async function testPublicationCloseOracle(): Promise<void> {
  const { Database } = await import("bun:sqlite");
  const fixture = JSON.parse(readFileSync(new URL("./publication-close.json", import.meta.url), "utf8"));
  const source = readFileSync(new URL("../../🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs", import.meta.url), "utf8");
  const database = new Database(":memory:");
  try {
    const route = database.query("SELECT CASE WHEN ? THEN 'page' WHEN ? THEN CASE WHEN ? THEN 'close' ELSE 'wait' END ELSE 'publish' END AS next");
    for (const row of fixture.routes) {
      assert.deepEqual(route.get(Number(row.page), Number(row.closing), Number(row.funded)), { next: row.next });
    }
    const page = source.indexOf("mounted.advance_pending_result_page(cx)?");
    const ready = source.indexOf("mounted.granted_retirement_ready()");
    const close = source.indexOf("mounted.granted_retirement_step(same, grant)");
    const publish = source.indexOf("advance_apply_batch_with_turn_ceiling");
    assert.ok(page >= 0 && page < ready && ready < close && close < publish);
    assert.match(source.slice(ready, close), /PendingArtifactStorePublication::is_closing[\s\S]*original_lifecycle_step\(cx, retained, demand/);
  } finally {
    database.close();
  }
}

/** 🪟️ Checks when an admitted window snapshot can carry its accepted receipt to the result page. */
export async function testWindowConfigRefreshOracle(): Promise<void> {
  const { Database } = await import("bun:sqlite");
  const fixture = JSON.parse(readFileSync(new URL("./window-config-refresh.json", import.meta.url), "utf8"));
  const source = readFileSync(new URL("../../🧵️instance/🏪️stores/📥️staging/🧾️typed-operations/🦀️.rs", import.meta.url), "utf8");
  const database = new Database(":memory:");
  try {
    const ready = database.query("SELECT (? >= ?) AND NOT ? AS ready");
    for (const row of fixture.cases) {
      assert.deepEqual(ready.get(row.generation, row.accepted, Number(row.pending)), { ready: Number(row.ready) }, row.id);
    }
    const capture = source.indexOf("self.window_config_store.refresh(authority, effect)?");
    const witness = source.indexOf("authority.pending_snapshot.is_none() && authority.generation >= receipt.generation_after");
    assert.ok(witness >= 0 && witness < capture, "an already transferred authority reaches its page without capturing another snapshot");
    assert.match(source.slice(witness, capture), /return Ok\(true\)/);
  } finally {
    database.close();
  }
}

if (import.meta.main) {
  testPublicationRetirementAuthorityOracle();
  await testPublicationProgressOracle();
  await testPublicationCloseOracle();
  await testWindowConfigRefreshOracle();
}
