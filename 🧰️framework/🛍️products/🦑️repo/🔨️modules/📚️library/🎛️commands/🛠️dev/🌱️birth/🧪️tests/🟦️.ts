import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { resolve } from "node:path";
import { rootDevMcpBuildRequestsV1, rootDevMcpFreshCapabilitiesV1 } from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import cases from "../🧫️fixtures/🔣️.json" with { type: "json" };

test("canonical dev MCP birth agrees with independent SQLite selection and AJV target validation", () => {
  const oracle = new Ajv({ strict: false, allErrors: true }), database = new Database(":memory:");
  expect(oracle.validate(schema, cases), JSON.stringify(oracle.errors)).toBe(true);
  const select = database.query("SELECT json_array_length($args)>=4 AND json_extract($args,'$[0]')='dev' AND json_extract($args,'$[1]')='mcp' AND json_extract($args,'$[2]')='stdio' AND json_extract($args,'$[3]')='os' AND NOT EXISTS(SELECT 1 FROM json_each($args) WHERE value IN ('-h','--help')) AS selected, (SELECT count(*) FROM json_each($args) WHERE key>=4 AND value='--catalog-target') AS flags, (SELECT json_extract($args,'$['||(key+1)||']') FROM json_each($args) WHERE key>=4 AND value='--catalog-target' LIMIT 1) AS target");
  try {
    for (const row of cases) {
      const result = select.get({ $args: JSON.stringify(row.argv) }) as { selected: number; flags: number; target: unknown };
      const refused = Boolean(result.selected && (result.flags > 1 || result.flags === 1 && !oracle.validate(schema.$defs.Target, result.target)));
      expect(refused, row.id).toBe(row.rejected);
      if (refused) { expect(() => rootDevMcpBuildRequestsV1(row.argv), row.id).toThrow(); continue; }
      const targets = result.selected ? [...new Set([schema.$defs.Protocol.const.buildTarget, ...(result.flags ? [result.target as string] : [])])] : null;
      expect(targets, row.id).toEqual(row.targets);
      expect(rootDevMcpBuildRequestsV1(row.argv)?.map(request => request[1]) ?? null, row.id).toEqual(targets);
    }
  } finally { database.close(); }
});

test("fresh dev MCP receives original providers and both declared native children", async () => {
  const environment: Record<string, string | undefined> = {}, root = resolve(import.meta.dirname, "../../../../../../../../..");
  const args = ["dev", "mcp", "stdio", "os", "--catalog-target", "@semio-tech/s-dev-composition:generate"];
  const capabilities = await rootDevMcpFreshCapabilitiesV1(root, args, environment) as { workspaceRoot: string; buildMode: string; native: unknown; launches: string[][] };
  expect(capabilities.workspaceRoot).toBe(root);
  expect(capabilities.buildMode).toBe("prototype");
  expect(capabilities.native).toBeDefined();
  expect(environment.SEMIO_SCRIPT_PROVIDERS).toBeDefined();
  for (const request of rootDevMcpBuildRequestsV1(args)!) expect(capabilities.launches).toContainEqual(["repo", ...request]);
  expect(await rootDevMcpFreshCapabilitiesV1(root, ["dev", "s"], {})).toBeNull();
});
