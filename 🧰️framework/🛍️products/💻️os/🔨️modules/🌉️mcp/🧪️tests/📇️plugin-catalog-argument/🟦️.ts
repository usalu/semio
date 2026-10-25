import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { takeMcpPluginCatalog } from "../../🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/📇️plugin-catalog-argument.json", import.meta.url), "utf8")) as { repoRoot: string; rows: { name: string; args: string[]; argsOut?: string[]; env?: Record<string, string>; error?: string }[] };

for (const row of fixture.rows) {
  test(row.name, () => {
    if (row.error) return expect(() => takeMcpPluginCatalog(row.args, fixture.repoRoot)).toThrow(row.error);
    expect(takeMcpPluginCatalog(row.args, fixture.repoRoot)).toEqual({ args: row.argsOut!, env: row.env! });
  });
}
