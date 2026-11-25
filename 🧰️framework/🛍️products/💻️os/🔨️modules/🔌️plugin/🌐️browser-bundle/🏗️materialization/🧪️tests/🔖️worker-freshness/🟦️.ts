/** 🔖️ A published shard worker older than the sources is stale, so a component bridge never meets a worker it was not generated with. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { createHash } from "node:crypto";
import { shardWorkerPublicationV1, shardWorkerSource } from "../../🧵️shard-worker-source/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const digest = (text: string): string => createHash("sha256").update(text).digest("hex");
const published = (row: (typeof fixture)[number]): string | undefined => row.published === "absent" ? undefined : row.published === "empty" ? "" : row.published === "generated" ? shardWorkerSource() : shardWorkerSource().replaceAll(row.removed!, "");

test("the fixture satisfies the publication schema", () => {
  const law = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(law(fixture), JSON.stringify(law.errors)).toBe(true);
  expect(law([{ name: "unknown", published: "generated", publication: "fresh" }])).toBe(false);
});

test("every published worker has the publication an independent digest gives it", () => {
  for (const row of fixture) {
    const text = published(row);
    expect(shardWorkerPublicationV1(text), row.name).toBe(row.publication as never);
    expect(text === undefined ? "missing" : digest(text) === digest(shardWorkerSource()) ? "current" : "stale", row.name).toBe(row.publication);
  }
});

test("the preparation and the playground launch both classify the published worker", async () => {
  const { readFileSync } = await import("node:fs");
  for (const owner of ["../../../../../🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts", "../../../../../../../🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🎮️playground/🟦️.ts"]) expect(readFileSync(new URL(owner, import.meta.url), "utf8"), owner).toContain("shardWorkerPublicationV1(");
});
