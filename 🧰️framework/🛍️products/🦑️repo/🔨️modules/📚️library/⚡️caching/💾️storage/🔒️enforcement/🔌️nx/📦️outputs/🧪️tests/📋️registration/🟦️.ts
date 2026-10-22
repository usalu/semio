import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import ts from "typescript";
import fixture from "./🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import owners from "./📇️owners.json";
import ownersSchema from "./🧬️schema/📇️owners.json";

test("toolkit output laws have a neutral permanent owner and bilingual dashboard control", () => {
  const root = process.cwd(), cache = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching"), project = JSON.parse(readFileSync(join(cache, "📋️project.json"), "utf8")), script = readFileSync(join(cache, "🧪️tests/🔬️script-commands/🟦️.ts"), "utf8"), db = new Database(":memory:");
  try {
    expect(new Ajv({ strict: true }).compile(schema.definitions.TestRegistration)(fixture)).toBe(true);
    expect(project.targets[fixture.target]?.options.command).toBe(fixture.command);
    expect(project.targets[fixture.target]?.cache).toBe(true);
    expect(project.targets[fixture.target]?.dependsOn).toEqual([]);
    expect(script).toContain('"toolkit-outputs":"' + fixture.source + '"');
    expect(script).toContain('selected==="toolkit-outputs"');
    const dashboard = JSON.parse(readFileSync(join(root, "📋️project.json"), "utf8")).metadata.semio.dashboard, control = dashboard.tools.find((row: any) => row.id === "build-task");
    expect(control.command).toEqual(["repo", "run", "{task}", "--output=stream"]);
    const labels = fixture.languages.map(language => ({ language, title: control.title[language.toLowerCase()] }));
    for (const label of labels) expect(label.title.length).toBeGreaterThan(0);
    expect(db.query("select count(*) count,count(distinct json_extract(value,'$.title')) names from json_each(?)").get(JSON.stringify(labels))).toEqual({ count: 2, names: 2 });
  } finally { db.close(); }
});

test("graph publication toolkit use and selected graph retain canonical owner targets", () => {
  const cache = join(process.cwd(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching"), file = join(cache, "📋️project.json"), text = readFileSync(file, "utf8"), project = JSON.parse(text), parsed = ts.parseConfigFileTextToJson(file, text), db = new Database(":memory:"), validate = new Ajv({ strict: true }).compile(ownersSchema);
  try {
    expect(validate(owners)).toBe(true);
    expect(validate({ ...owners, extra: true })).toBe(false);
    expect(parsed.error).toBeUndefined();
    db.exec("create table expected(target text primary key,command text);create table actual(target text primary key,command text)");
    for (const row of owners.targets) {
      expect(project.targets[row.target]?.options.command).toBe(row.command);
      expect(readFileSync(join(cache, row.source), "utf8").length).toBeGreaterThan(0);
      db.query("insert into expected values(?,?)").run(row.target, row.command);
      db.query("insert into actual values(?,?)").run(row.target, parsed.config.targets[row.target].options.command);
    }
    expect(db.query("select (select count(*) from actual) count,not exists(select * from expected except select * from actual) and not exists(select * from actual except select * from expected) matched").get()).toEqual({ count: 3, matched: 1 });
  } finally { db.close(); }
});
