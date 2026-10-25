/** 🧪️ The Storybook dev server watches with native events on a host and polls only inside a devcontainer; polling a whole repository costs one stat watcher per file. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { storybookPollingDefaultV1 } from "../🏃️execution/🟦️.ts";

const fixture = JSON.parse(readFileSync(fileURLToPath(new URL("./🔣️.json", import.meta.url)), "utf8")) as { schemaVersion: number; cases: { id: string; environment: Record<string, string>; expected: string }[] };

test("polling defaults follow the container flag only", () => {
  expect(fixture.schemaVersion).toBe(1);
  for (const row of fixture.cases) expect(storybookPollingDefaultV1(row.environment), row.id).toBe(row.expected);
});
