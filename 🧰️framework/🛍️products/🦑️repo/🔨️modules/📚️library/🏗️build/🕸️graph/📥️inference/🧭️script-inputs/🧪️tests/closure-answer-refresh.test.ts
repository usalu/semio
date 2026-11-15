/** 🔁️ A query over a command closure whose shared provider entries change while it is answered (a peer adds a file to a hot tool directory between the check that admits an entry and the check that expands it) is answered again from refreshed entries instead of failing the project with `closure.provider-entry-stale`, so a farm worker no longer hands the project back to the parent for it. */
import { expect,test } from "bun:test";
import { readFileSync } from "node:fs";
import { INFERENCE_LIMITS_V1 } from "../../🧬️schema/🟨️.mjs";
import schema from "../../🧬️schema/🔣️.json";
import { answeredAgainstFreshProvidersV1,withClosurePassV1 } from "../🟨️.mjs";

test("the attempts are declared with an English and a German text", () => {
 expect(INFERENCE_LIMITS_V1.closureAnswerAttempts).toBe(3);
 for (const language of ["en", "de"] as const) expect(schema.definitions.LimitsDescriptions.const[language].closureAnswerAttempts.length).toBeGreaterThan(40);
 expect(schema.definitions.LimitsDescriptions.const.en.closureAnswerAttempts).not.toBe(schema.definitions.LimitsDescriptions.const.de.closureAnswerAttempts);
});

test("an answer that holds is returned at once and nothing is refreshed", async () => {
 let refreshed = 0;
 expect(await answeredAgainstFreshProvidersV1(() => 7, async () => { refreshed++; })).toBe(7);
 expect(await answeredAgainstFreshProvidersV1(() => false, async () => { refreshed++; })).toBe(false);
 expect(refreshed).toBe(0);
});

test("an answer that is undefined until the providers were refreshed twice is returned after the second refresh", async () => {
 let providers = 0, refreshed = 0;
 const answer = () => providers < 2 ? undefined : "fresh";
 expect(await answeredAgainstFreshProvidersV1(answer, async () => { refreshed++; providers++; })).toBe("fresh");
 expect(refreshed).toBe(2);
});

test("an answer that stays undefined is refreshed for exactly the declared attempts and then stays undefined, so the caller refuses the query as stale", async () => {
 let asked = 0, refreshed = 0;
 expect(await answeredAgainstFreshProvidersV1(() => { asked++; return undefined; }, async () => { refreshed++; })).toBeUndefined();
 expect(refreshed).toBe(INFERENCE_LIMITS_V1.closureAnswerAttempts);
 expect(asked).toBe(INFERENCE_LIMITS_V1.closureAnswerAttempts + 1);
 asked = 0;
 expect(await answeredAgainstFreshProvidersV1(() => { asked++; return undefined; }, async () => undefined, 0)).toBeUndefined();
 expect(asked).toBe(1);
});

test("a refresh that fails fails the query with its own error", async () => {
 await expect(answeredAgainstFreshProvidersV1(() => undefined, async () => { throw Error("refresh lost"); })).rejects.toThrow("refresh lost");
});

test("a project that is inferred in a worker runs inside one closure pass that nests the pass of its parent", async () => {
 expect(readFileSync(new URL("../../📦️project-nodes/🟨️.mjs", import.meta.url), "utf8")).toMatch(/await withClosurePassV1\(\(\) => observeInferenceSnapshotV1\(\(\) => observeProjectInferenceV1\(\{ cachePath, key: projectRecordKeyV1/u);
 let inner = 0;
 expect(await withClosurePassV1(async () => withClosurePassV1(async () => { inner++; return "nested"; }))).toBe("nested");
 expect(inner).toBe(1);
});
