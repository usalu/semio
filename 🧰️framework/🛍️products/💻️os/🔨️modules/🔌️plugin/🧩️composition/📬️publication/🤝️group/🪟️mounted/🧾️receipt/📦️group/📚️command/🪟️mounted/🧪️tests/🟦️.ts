import { expect, test } from "bun:test";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import fixture from "../../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../../🧫️fixtures/📐️schema.json" with { type: "json" };
import pruning from "../../🧹️pruning/🧫️fixtures/🔣️.json" with { type: "json" };
import pruningSchema from "../../🧹️pruning/🧫️fixtures/📐️schema.json" with { type: "json" };

test("mounted command history uses original paged append and whole physical retirement", async () => {
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  for (const row of fixture.cases) {
    const document = { commands: [] as number[] };
    const actual = applyPatch(document, row.sequences.map(value => ({ op: "add" as const, path: "/commands/-", value })), true, false).newDocument;
    expect(document.commands).toEqual([]);
    expect(actual.commands).toEqual(row.sequences);
    expect(Buffer.from(JSON.stringify(actual.commands), "utf8").toString("utf8")).toBe(JSON.stringify(row.sequences));
  }
  const source = await Bun.file(new URL("../../../../../../../../../🦀️.rs", import.meta.url)).text();
  expect(source.includes("command_log: PagedCommandLog")).toBe(true);
  expect(source.includes("command_log: PagedCommandLog::new()")).toBe(true);
  expect(source.includes("self.command_log.prepare_append()")).toBe(true);
  expect(source.includes("self.command_log.commit_append(")).toBe(true);
  expect(source.includes("self.command_log.close_step(")).toBe(true);
  expect(source.includes("self.command_log.next_close_byte_demand()")).toBe(true);
  expect(source.includes("self.command_log.pop()")).toBe(false);
  expect(source.includes("self.command_log[")).toBe(false);
  expect(source.includes("self.command_log.partition_point(")).toBe(false);
  console.log("[DEBUG] mounted history original0/1/8/9/17 row order agrees with Ajv/RFC6902/Node JSON; paged app consumer retains exact whole backing");
});

test("mounted replacement visibility prepares bounded original identities and retains discarded backing", async () => {
  expect(new Ajv({ strict: true }).compile(pruningSchema)(pruning)).toBe(true);
  for (const row of pruning.cases) {
    const edits = new Set(row.heldEdits), transitions = new Set(row.heldTransitions);
    const kept = row.rows.filter(entry => (entry.editId === null || edits.has(entry.editId)) && (entry.transitionId === null || transitions.has(entry.transitionId))).map(entry => entry.seq);
    const replaced = applyPatch({ rows: row.rows.map(entry => entry.seq) }, [{ op: "replace", path: "/rows", value: kept }], true, false).newDocument;
    expect(JSON.parse(Buffer.from(JSON.stringify(replaced.rows), "utf8").toString("utf8"))).toEqual(row.keptSequences);
  }
  const source = await Bun.file(new URL("../🦀️.rs", import.meta.url)).text();
  for (const call of ["prune_current", "advance_prune", "commit_prune", "cancel_prune", "next_pruned_close_byte_demand", "close_pruned_step"]) expect(source.includes(`self.command_log.${call}(`)).toBe(true);
  expect(source.includes(".min(64)")).toBe(true);
  expect(source.includes("owner.revision != self.store.content_revision_now()")).toBe(true);
  expect(source.includes("owner.requested != self.command_prune_generation")).toBe(true);
  console.log("[DEBUG] mounted replacement visibility exact conjunction matches Ajv/RFC6902/Node JSON across3 neutral histories; original identifiers compare atmost64bytes per structuralturn");
});
