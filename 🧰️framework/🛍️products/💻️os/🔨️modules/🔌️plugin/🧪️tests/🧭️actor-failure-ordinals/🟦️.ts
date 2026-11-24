/** 🧭️ The plugin WIT owns the wire ordinals of the neutral actor failure frontier and its budgetless job exports. */
import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import corpus from "../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/⚠️failure/🧪️tests/🔣️.json" with { type: "json" };

const schema = (name: string) => readFileSync(new URL(`../../🧬️schema/${name}`, import.meta.url), "utf8");

test("WIT selector ordinals match the neutral semantic frontier through SQLite", () => {
  const reactor = schema("📜️reactor.wit"), jobs = schema("📜️jobs.wit");
  const entries = reactor.match(/enum actor-failure-call-kind\s*\{([^}]+)\}/)![1]!.split(",").map((value) => value.trim()).filter(Boolean);
  const db = new Database(":memory:");
  try {
    db.exec("create table kinds(tag integer primary key,name text unique)");
    for (const [index, name] of entries.entries()) db.query("insert into kinds values(?,?)").run(index + 1, name);
    for (const kind of corpus.kinds) expect(db.query("select name from kinds where tag=?").get(kind.tag)).toEqual({ name: kind.wire });
  } finally { db.close(); }
  expect(reactor).toContain("stage-actor-context-budget: func(budget: actor-context-budget);");
  expect(reactor).toContain("stage-cold-pair-page: func(page: cold-document-pair-page)");
  expect(reactor).toContain("stage-command-page: async func(");
  for (const verb of ["publish", "read", "acknowledge"]) expect(jobs).toContain(verb + "-actor-failure: async func(reference: actor-failure-call-reference)");
  expect(jobs).toContain("stage-job-budget: func(budget: job-budget);");
});
