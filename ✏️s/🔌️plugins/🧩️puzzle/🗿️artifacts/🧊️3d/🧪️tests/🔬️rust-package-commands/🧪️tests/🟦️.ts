import { expect, test } from "bun:test";
import cases from "./🔣️.json";
import { runtimeLifecycleRoute } from "../🟦️.ts";

test("runtime lifecycle routes only the owned acceptance test", () => {
  for (const row of cases) expect(runtimeLifecycleRoute(row.args) ?? null).toEqual(row.expected);
});
