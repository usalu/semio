import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { generatorPreviewExecution } from "../../../🔍️discovery/✅️owned-file-approvals/🟦️.ts";

type PreviewInput = Parameters<typeof generatorPreviewExecution>;
type Fixture = {
  refusals: { name: string; contract: PreviewInput[0]; target: PreviewInput[1]; reason: string }[];
  accepted: { name: string; contract: PreviewInput[0]; target: PreviewInput[1]; args: string[]; cwd: string };
};

const fixture: Fixture = JSON.parse(readFileSync(fileURLToPath(new URL("./🔣️refusals.json", import.meta.url)), "utf8"));

describe("🧾 owned preview refusal reasons", () => {
  for (const refusal of fixture.refusals) test(refusal.name, () => {
    let thrown: unknown;
    try {
      generatorPreviewExecution(refusal.contract, refusal.target);
    } catch (error) {
      thrown = error;
    }
    expect(thrown).toBeInstanceOf(Error);
    expect((thrown as Error).message).toContain(refusal.reason);
    expect(((thrown as Error).cause as Error).message).toContain(refusal.reason);
  });

  test(fixture.accepted.name, () => {
    const execution = generatorPreviewExecution(fixture.accepted.contract, fixture.accepted.target);
    expect(execution.command).toBe(process.versions.bun ? process.execPath : "bun");
    expect(execution.args).toEqual(fixture.accepted.args);
    expect(execution.cwd).toBe(fixture.accepted.cwd);
  });
});
