import { test, expect } from "bun:test";
import { fileURLToPath } from "node:url";
import { types } from "@bytecodealliance/jco";
import ts from "typescript";
import { PLUGIN_STAGED_BUDGET_GROUPS, pluginStagedGuestSource, stagedGuest } from "../🟦️.ts";

const EXPECTED = [
  { stage: ["reactor", "stageActorContextBudget"], consumers: { reactor: ["admitActorContext", "admitActorInput", "closeActorContext", "closeActorOutput", "poll"], checkpoint: ["checkpoint", "restore"], describe: ["describe"], codec: ["sqliteSchema", "sqliteExport", "sqliteImport", "sqliteTakeFile", "sqliteTakePayload", "sqliteRetirement", "sqliteClose", "packSchemaHash", "genesis", "printMirror", "applyOps", "replayEnvelopes"] } },
  { stage: ["jobs", "stageJobBudget"], consumers: { jobs: ["startJob", "stepJob", "readJobOutput", "acknowledgeJobOutput", "cancelJob", "closeJobs", "publishActorFailure", "readActorFailure", "acknowledgeActorFailure"] } },
];

test("staged budget consumers derive from the owned WIT and the independent JCO declarations", async () => {
  expect(JSON.parse(JSON.stringify(PLUGIN_STAGED_BUDGET_GROUPS))).toEqual(EXPECTED);
  const generated = await types(fileURLToPath(new URL("../../../🧬️schema", import.meta.url)), { name: "actor-contract", worldName: "actor", instantiation: "async", asyncMode: "jspi" });
  const asynchronous = (owner: string) => {
    const source = ts.createSourceFile(owner + ".ts", new TextDecoder().decode(generated["interfaces/semio-framework-" + owner + ".d.ts"]!), ts.ScriptTarget.Latest, true);
    return source.statements.filter(ts.isFunctionDeclaration).filter((node) => node.type?.getText(source).startsWith("Promise")).map((node) => node.name!.text);
  };
  const exempt: Record<string, string[]> = { reactor: ["stageCommandPage", "channelVersion"], jobs: ["takeSegmentedDownloadChunk"] };
  for (const { consumers } of PLUGIN_STAGED_BUDGET_GROUPS) for (const [owner, methods] of Object.entries(consumers)) expect(methods).toEqual(asynchronous(owner).filter((name) => !exempt[owner]?.includes(name)));
  for (const { stage: [owner, name] } of PLUGIN_STAGED_BUDGET_GROUPS) expect(asynchronous(owner)).not.toContain(name);
});

test("a staged guest stages the last argument immediately before every consuming call and leaves the rest untouched", async () => {
  const log: unknown[][] = [];
  const raw = { reactor: { stageActorContextBudget: (budget: unknown) => log.push(["stage", budget]), stageCommandPage: async (page: unknown) => log.push(["page", page]), poll: async (...args: unknown[]) => { log.push(["poll", ...args]); return "polled"; } }, jobs: { stageJobBudget: (budget: unknown) => log.push(["stageJob", budget]), startJob: async (...args: unknown[]) => { log.push(["start", ...args]); return "started"; } }, describe: { describe: async (...args: unknown[]) => { log.push(["describe", ...args]); return "described"; } } };
  for (const guest of [stagedGuest(raw, PLUGIN_STAGED_BUDGET_GROUPS), new Function(pluginStagedGuestSource() + ";return pluginStagedGuest")()(raw)]) {
    log.length = 0;
    expect(await guest.reactor.poll("budget-1")).toBe("polled");
    await guest.reactor.stageCommandPage("page");
    expect(await guest.jobs.startJob(7n, "budget-2")).toBe("started");
    expect(await guest.describe.describe("budget-3")).toBe("described");
    expect(log).toEqual([["stage", "budget-1"], ["poll"], ["page", "page"], ["stageJob", "budget-2"], ["start", 7n], ["stage", "budget-3"], ["describe"]]);
  }
});
