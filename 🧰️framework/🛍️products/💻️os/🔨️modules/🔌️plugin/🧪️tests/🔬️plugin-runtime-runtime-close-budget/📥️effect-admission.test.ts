/** 🚧️ Document readiness and pending effect ownership. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { readFileSync } from "node:fs";

test("pending effects wait through archive and replacement retirement", async () => {
  const fixture = (name: string) => Bun.file(new URL(`🧫️fixtures/📥️effect-admission/${name}.json`, import.meta.url)).json();
  const [schema, turns] = await Promise.all([fixture("🧬️schema"), fixture("🔣️")]);
  expect(new Ajv({ strict: true }).compile(schema)(turns)).toBe(true);

  const db = new Database(":memory:");
  try {
    const oracle = db.query(`
      WITH blocked AS (
        SELECT NOT $merge AND $archive IN ('staged', 'pending', 'running', 'retiring')
          OR $replacement IN ('candidate', 'retiring') AS document
      )
      SELECT $armed AND NOT document AND $poll = 'paused' AS runnable,
        $armed AND (document OR $poll = 'paused') AS retained
      FROM blocked
    `);
    for (const turn of turns) {
      expect(oracle.get({
        $archive: turn.archive, $merge: Number(turn.merge), $replacement: turn.replacement,
        $armed: Number(turn.armed), $poll: turn.poll,
      })).toEqual({ runnable: Number(turn.runnable), retained: Number(turn.retained) });
    }
  } finally {
    db.close();
  }
});

test("the reactor retains paused effects and the render path gates snapshot preparation", () => {
  const read = (path: string) => readFileSync(new URL(`../../${path}/🦀️.rs`, import.meta.url), "utf8");
  const docs = read("🧵️instance/📄️docs/📥️staging/⏪️time-travel");
  const predicate = docs.slice(docs.indexOf("pub fn document_blocks_pending_effects"));
  expect(predicate).toContain("!load.merge && !load.terminal()");
  expect(predicate).toContain("replacement.state != ActiveArtifactStoreReplacementState::Complete");
  const render = read("🧵️instance/🖼️render/📥️staging/🪟️projections");
  const poll = render.slice(render.indexOf("pub async fn pending_effects("));
  expect(poll).toMatch(/document_blocks_pending_effects\(\).*return Ok\(None\);[\s\S]*mounted_job_prepare_snapshot_read[\s\S]*\.snapshot_read\(\)/);
  const ops = read("⚛️reactor/🏃️runtime/🧰️ops/🧾️typed-operations");
  const drive = ops.slice(ops.indexOf("pub fn plugin_drive_pending_effects"));
  expect(drive).toContain("if !document_ingress_pending(id) { armed.push(cell.clone()); }");
  expect(drive).toMatch(/pending_effects_waits_for_document\(\) \{ continue; \}[\s\S]*\.app\.pending_effects\([\s\S]*pending_effects\.publish\(\)[\s\S]*None => runnable = true/);
  const exchange = read("⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🏁️exchange-finish");
  expect(exchange).toMatch(/pending_effects_waits_for_document\(\) \{\s*guard\.cell\.pending_effects\.retain\(\);\s*return Ok\(Some\(Vec::new\(\)\)\);/);
  const turn = read("⚛️reactor/🔄️turn/🌀️poll-kernel-turn");
  expect(turn).toMatch(/drive_cold_pair_ingress\([\s\S]*step_restored_document_loads\([\s\S]*let document_ingress_pending[\s\S]*COLD_PAIR_DOCUMENT_LOAD[\s\S]*RESTORED_DOCUMENT_LOADS[\s\S]*plugin_drive_pending_effects\(runtime, &mut effects, document_ingress_pending/);
  const admissionPaths = [
    "⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/📦️document-archive-commands",
    "⚛️reactor/🏃️runtime/🧰️ops/📄️document",
    "⚛️reactor/🏃️runtime/🧰️ops/🎞️media",
  ];
  for (const path of admissionPaths) {
    expect(read(path)).toMatch(/let admitted = instance\.app\.begin_document_archive_load\([^;]+;\s*if admitted \{ (instance\.cell|cell)\.pending_effects\.retain\(\); \}/);
  }
});

test("a late cancellation can only reach the matching mounted shell occupant", async () => {
  const fixture = (name: string) => Bun.file(new URL(`🧫️fixtures/📥️effect-admission/🚫️cancel${name}.json`, import.meta.url)).json();
  const [schema, cases] = await Promise.all([fixture(".schema"), fixture("")]);
  expect(new Ajv({ strict: true, allowUnionTypes: true }).compile(schema)(cases)).toBe(true);
  const db = new Database(":memory:");
  try {
    const match = db.query("SELECT EXISTS(SELECT 1 WHERE ? = ?) AS cancelled");
    for (const row of cases) expect(match.get(row.job, row.occupant)).toEqual({ cancelled: Number(row.cancelled) });
  } finally {
    db.close();
  }
  const relative = "../../../../../../../✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/⚙️mounted-job/🦀️.rs";
  const mounted = readFileSync(new URL(relative, import.meta.url), "utf8");
  const cancel = mounted.slice(mounted.indexOf("fn cancel("), mounted.indexOf("fn checkpoint("));
  expect(cancel).toMatch(/shell\.as_ref\(\)\.filter\(\|state\| state\.identity == self\.identity\)[\s\S]*state\.cancel\.cancel_now\(\)/);
});
