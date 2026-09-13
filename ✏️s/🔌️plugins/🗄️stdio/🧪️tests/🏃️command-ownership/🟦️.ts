import { describe, test } from "bun:test";
import { verifyStdioCommandOwnership } from "../📦️artifact-package-graph/🟦️.ts";

describe("stdio command ownership", () => {
  test("binds semantic owners, direct routers, source data and registered inputs", async () => {
    await verifyStdioCommandOwnership();
  }, 30_000);
});
