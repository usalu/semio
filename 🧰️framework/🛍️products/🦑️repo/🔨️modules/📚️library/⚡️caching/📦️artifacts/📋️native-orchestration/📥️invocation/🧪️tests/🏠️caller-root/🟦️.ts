import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { callerRepositoryRootV1 } from "../../🟦️.ts";
import fixture from "./🔣️.json";

test("every admitted caller principal names its repository root and every other shape is refused, as an independent SQLite projection says", () => {
  const database = new Database(":memory:");
  try {
    for (const row of fixture.cases) {
      const wire = JSON.stringify(row.value), oracle = database.query("select case when ?1 is null or json_type(?1,'$.version') is null and json_type(?1,'$.native') is null then null when json_type(?1,'$.native')='object' then json_extract(?1,'$.workspaceRoot') else json_extract(?1,'$.repositoryRoot') end root").get(wire) as { root: string | null };
      if (row.accepted) {
        expect(callerRepositoryRootV1(row.value), row.name).toBe(oracle.root!);
        expect(oracle.root, row.name).not.toBeNull();
      } else expect(() => callerRepositoryRootV1(row.value), row.name).toThrow("Original");
    }
  } finally { database.close(); }
});
