/** 🩺️ The findings of an asset come from threads of their own: the serving thread stays free, a revision is checked once, a check nobody waits for is stopped, and the development server answers with the asset first and its findings later.
 * @see ./🥒️.feature — the scenarios
 * @see ../../🌐️vite/🩺️findings/🟦️.ts — the service
 * @see ../../🌐️vite/🟦️.ts — the plugin */
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import { validateAsset, type Asset, type Issue } from "@teaching/staffage";
import type { OwnedBuildServer } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🛠️build-tooling/🟦️.ts";
import { StaffageFindings, type FindingsThread } from "../../🌐️vite/🩺️findings/🟦️.ts";
import type { FindingsAnswer, FindingsQuestion } from "../../🌐️vite/🩺️findings/🧵️worker/🟦️.ts";
import { STAFFAGE_ASSET_QUERY, STAFFAGE_ASSET_REFRESH, STAFFAGE_FINDINGS_ROUTE, staffageAssetVitePlugin, staffageRevision, type StaffageFindingsLine, type StaffageServedPayload } from "../../🌐️vite/🟦️.ts";

const fixture = resolve(import.meta.dirname, "../../../../🧫️fixtures/👤️sample-figure/🔣️.json");
const text = readFileSync(fixture, "utf8");
const document = JSON.parse(text) as { asset: Asset };
const finding: Issue = { path: "$", code: "broken", message: { en: "Broken", de: "Defekt" } };

type Fake = { readonly thread: FindingsThread; readonly questions: FindingsQuestion[]; ended: boolean; answer: (answer: FindingsAnswer) => void };

function fakes(): { readonly made: Fake[]; readonly thread: () => FindingsThread } {
  const made: Fake[] = [];
  return {
    made,
    thread: () => {
      const fake: Fake = { questions: [], ended: false, answer: () => undefined, thread: { post: (question) => void fake.questions.push(question), listen: (answer) => void (fake.answer = answer), end: () => void (fake.ended = true), hold: () => undefined } };
      made.push(fake);
      return fake.thread;
    },
  };
}

const services: StaffageFindings[] = [];
const service = (options: ConstructorParameters<typeof StaffageFindings>[0]): StaffageFindings => {
  const made = new StaffageFindings(options);
  services.push(made);
  return made;
};
afterEach(async () => {
  for (const made of services.splice(0)) await made.close();
});

describe("the findings service", () => {
  it("checks on a thread of its own, reports the six parts in order and finds what validateAsset finds", async () => {
    const broken = JSON.stringify({ ...document, asset: { ...document.asset, dimensions: { ...document.asset.dimensions, heightM: 0 } } });
    const findings = service({ threads: 1 });
    let ticks = 0;
    const beat = setInterval(() => (ticks += 1), 1);
    const heard: number[] = [];
    const issues = await findings.ask(staffageRevision(broken), broken, (done, total) => heard.push(done / total)).findings;
    const ticksWhileChecking = ticks;
    clearInterval(beat);
    expect(issues).toEqual(validateAsset((JSON.parse(broken) as { asset: Asset }).asset));
    expect(issues.map((issue) => issue.code)).toContain("dimension-invalid");
    expect(heard).toEqual([1 / 6, 2 / 6, 3 / 6, 4 / 6, 5 / 6, 1]);
    expect(ticksWhileChecking).toBeGreaterThan(0);
    expect(await findings.ask(staffageRevision(text), text).findings).toEqual(validateAsset(document.asset));
  });

  it("runs one check per revision, lets every reader hear it and keeps the findings", async () => {
    const { made, thread } = fakes();
    const findings = service({ threads: 2, thread });
    const first: number[] = [];
    const second: number[] = [];
    const one = findings.ask("sha256:a", "A", (done) => first.push(done));
    made[0]!.answer({ id: made[0]!.questions[0]!.id, progress: [1, 6, { en: "Rig", de: "Rig" }] });
    const two = findings.ask("sha256:a", "A", (done) => second.push(done));
    made[0]!.answer({ id: made[0]!.questions[0]!.id, progress: [2, 6, { en: "Drawings", de: "Zeichnungen" }] });
    made[0]!.answer({ id: made[0]!.questions[0]!.id, issues: [finding] });
    expect(await Promise.all([one.findings, two.findings])).toEqual([[finding], [finding]]);
    expect([first, second]).toEqual([[1, 2], [1, 2]]);
    expect(await findings.ask("sha256:a", "A").findings).toEqual([finding]);
    expect(made).toHaveLength(1);
    expect(made[0]!.questions.map((question) => question.text)).toEqual(["A"]);
  });

  it("stops a check nobody waits for and keeps one somebody still waits for", async () => {
    const { made, thread } = fakes();
    const findings = service({ threads: 1, thread });
    const running = findings.ask("sha256:a", "A");
    const waiting = findings.ask("sha256:b", "B");
    const shared = findings.ask("sha256:c", "C");
    const sharedToo = findings.ask("sha256:c", "C");
    expect(made[0]!.questions.map((question) => question.text)).toEqual(["A"]);
    waiting.cancel();
    await expect(waiting.findings).rejects.toMatchObject({ name: "AbortError" });
    shared.cancel();
    running.cancel();
    await expect(running.findings).rejects.toMatchObject({ name: "AbortError" });
    expect(made[0]!.ended).toBe(true);
    expect(made).toHaveLength(2);
    expect(made[1]!.questions.map((question) => question.text)).toEqual(["C"]);
    expect(findings.pending).toEqual(["sha256:c"]);
    made[1]!.answer({ id: made[1]!.questions[0]!.id, issues: [] });
    expect(await sharedToo.findings).toEqual([]);
    expect(findings.pending).toEqual([]);
  });
});

type Sent = { readonly lines: StaffageFindingsLine[]; ended: boolean; close: () => void };

function listeningServer(): { readonly server: OwnedBuildServer; readonly read: (url: string) => { readonly sent: Sent; readonly passed: boolean }; readonly changed: (file: string) => void } {
  let middleware: Parameters<OwnedBuildServer["middlewares"]["use"]>[0] = () => undefined;
  let change: (file: string) => void = () => undefined;
  const server = { middlewares: { use: (given: typeof middleware) => void (middleware = given) }, ws: { send: () => undefined }, config: { root: "/", cacheDir: "/", base: "/", server: {} }, watcher: { emit: () => true, on: (_event: "change", listener: (file: string) => void) => void (change = listener) }, httpServer: { listening: true, on: () => undefined, once: () => undefined } } as unknown as OwnedBuildServer;
  return {
    server,
    changed: (file) => change(file),
    read: (url) => {
      const sent: Sent = { lines: [], ended: false, close: () => undefined };
      const take = (chunk?: string): void => void (chunk === undefined ? undefined : sent.lines.push(...chunk.split("\n").filter((line) => line !== "").map((line) => JSON.parse(line) as StaffageFindingsLine)));
      let passed = false;
      const response = { setHeader: () => undefined, write: (chunk: string) => (take(chunk), true), end: (chunk?: string) => (take(chunk), (sent.ended = true)), on: (_event: "close", listener: () => void) => void (sent.close = listener), get writableEnded() { return sent.ended; } };
      void middleware({ url } as never, response as never, () => void (passed = true));
      return { sent, get passed() { return passed; } };
    },
  };
}

async function module(plugin: ReturnType<typeof staffageAssetVitePlugin>, file: string): Promise<StaffageServedPayload> {
  const id = (await plugin.resolveId!(file + STAFFAGE_ASSET_QUERY))!;
  const load = plugin.load!;
  const source = (await load(id))!;
  const end = source.indexOf(`;${STAFFAGE_ASSET_REFRESH}`);
  return JSON.parse(source.slice("export default ".length, end < 0 ? -1 : end)) as StaffageServedPayload;
}

describe("the asset plugin", () => {
  const until = async (done: () => boolean): Promise<void> => {
    for (let waited = 0; !done() && waited < 60_000; waited += 5) await new Promise((resolve) => setTimeout(resolve, 5));
  };

  it("serves the asset at once on a development server and streams progress and findings at its address", async () => {
    const directory = mkdtempSync(join(tmpdir(), "staffage-findings-"));
    const plugin = staffageAssetVitePlugin(false, { threads: 1 });
    const { server, read } = listeningServer();
    try {
      const file = join(directory, "invalid.json");
      const broken = JSON.stringify({ ...document, asset: { ...document.asset, dimensions: { ...document.asset.dimensions, heightM: 0 } } });
      writeFileSync(file, broken);
      await plugin.configureServer!(server);
      const id = (await plugin.resolveId!(file + STAFFAGE_ASSET_QUERY))!;
      const load = plugin.load!;
      expect(load(id)).toMatch(/^export default \{.*\};\nif \(import\.meta\.hot\) import\.meta\.hot\.accept\(/su);
      const payload = await module(plugin, file);
      expect(payload.issues).toBeUndefined();
      expect(payload.findings).toBe(`/${STAFFAGE_FINDINGS_ROUTE}${staffageRevision(broken).slice("sha256:".length)}`);
      expect(read("/elsewhere").passed).toBe(true);
      const { sent } = read(payload.findings!);
      await until(() => sent.ended);
      expect(sent.lines.slice(0, -1).map((line) => ("progress" in line ? line.progress[0] : null))).toEqual([1, 2, 3, 4, 5, 6]);
      expect(sent.lines.at(-1)).toEqual({ issues: JSON.parse(JSON.stringify(validateAsset((JSON.parse(broken) as { asset: Asset }).asset))) });
      expect(read(`/${STAFFAGE_FINDINGS_ROUTE}${"0".repeat(64)}`).sent.lines).toEqual([{ stale: true }]);
      writeFileSync(file, text);
      expect(read(payload.findings!).sent.lines).toEqual([{ stale: true }]);
    } finally {
      await plugin.closeBundle!();
      rmSync(directory, { recursive: true, force: true });
    }
  });

  it("withdraws the check of a reader who closes the stream, and forgets kept findings when a rule source changes", async () => {
    const { made, thread } = fakes();
    const plugin = staffageAssetVitePlugin(false, { threads: 1, thread });
    const { server, read, changed } = listeningServer();
    await plugin.configureServer!(server);
    const payload = await module(plugin, fixture);
    const early = read(payload.findings!);
    expect(made[0]!.questions).toHaveLength(1);
    early.sent.close();
    expect(made[0]!.ended).toBe(true);
    const kept = read(payload.findings!);
    made[1]!.answer({ id: made[1]!.questions[0]!.id, issues: [finding] });
    await until(() => kept.sent.ended);
    expect(kept.sent.lines).toEqual([{ issues: [finding] }]);
    const again = read(payload.findings!);
    await until(() => again.sent.ended);
    expect(again.sent.lines).toEqual([{ issues: [finding] }]);
    expect(made).toHaveLength(2);
    changed("/somewhere/rule.ts");
    const fresh = read(payload.findings!);
    expect(made).toHaveLength(3);
    made[2]!.answer({ id: made[2]!.questions[0]!.id, issues: [] });
    await until(() => fresh.sent.ended);
    expect(fresh.sent.lines).toEqual([{ issues: [] }]);
    await plugin.closeBundle!();
  });

  it("waits for the findings without a listening server and carries them in the module", async () => {
    const { made, thread } = fakes();
    const plugin = staffageAssetVitePlugin(false, { threads: 1, thread });
    const loading = module(plugin, fixture);
    await until(() => made.length === 1 && made[0]!.questions.length === 1);
    made[0]!.answer({ id: made[0]!.questions[0]!.id, issues: [finding] });
    const payload = await loading;
    expect(payload.issues).toEqual([finding]);
    expect(payload.findings).toBeUndefined();
    await plugin.closeBundle!();
  });
});
