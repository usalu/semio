/** 📥️ Folder read-back law (design §22.22, live fault F4): Ajv validates the shared corpus
 * (`🛂️admission/📄️document/🧫️fixtures/🧫️folder-read-back/🔣️.json`); every case is replayed through
 * {@link FolderReadBackRouteV1} over a real {@link FolderArchivePersistenceV1} with scripted program answers and through this
 * file's own reducer of the rule; an answer the route asks for that the case does not script fails the case. A load the
 * person cancels is never told as a failure (live fault F13), and a FIRST archive that is not taken detaches the folder
 * instead of leaving the program to write over it. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020";
import { FolderArchivePersistenceV1, FolderReadBackRouteV1, type FolderReadBackMergeV1, type FolderReadBackOutcomeV1 } from "../../🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";

type Merge = "unmergeable" | { readonly merged: number; readonly ahead: number };
type Answers = { readonly merge?: Merge; readonly load?: "loaded" | "stale" | "cancelled" | "failed" };
type Outcome = "adopted" | "reattached" | "failed" | FolderReadBackOutcomeV1;
type ReadBackEvent = { readonly absent: null } | { readonly reconnect: null } | { readonly archive: Answers };
type Case = { readonly id: string; readonly events: readonly ReadBackEvent[]; readonly calls: readonly string[]; readonly outcomes: readonly Outcome[] };
type Run = { readonly calls: string[]; readonly outcomes: Outcome[] };

const cancel = (): DOMException => new DOMException("the person cancelled the load", "AbortError");
const cancelled = (error: unknown): boolean => error instanceof DOMException && error.name === "AbortError";

const fixtures = new URL("../../🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🧫️fixtures/🧫️folder-read-back/", import.meta.url);
const corpus = JSON.parse(readFileSync(new URL("🔣️.json", fixtures), "utf8")) as { readonly cases: readonly Case[] };

/** ⏯️ Drives the route with one case's events and answers the calls it made and how each event ended. */
async function replay(testCase: Case): Promise<Run> {
  const calls: string[] = [];
  const outcomes: Outcome[] = [];
  const attach = (): FolderReadBackRouteV1<Answers> => new FolderReadBackRouteV1<Answers>(new FolderArchivePersistenceV1(async () => { calls.push("write"); }));
  let route = attach();
  let detached = false;
  for (const event of testCase.events) {
    if ("reconnect" in event) {
      route = attach();
      detached = false;
      outcomes.push("reattached");
      continue;
    }
    if ("absent" in event) {
      await route.absent();
      outcomes.push(detached ? "superseded" : "adopted");
      continue;
    }
    const answers = event.archive;
    const outcome = await route.archive(answers, () => true, {
      merge: async (archive): Promise<FolderReadBackMergeV1> => {
        calls.push("merge");
        if (archive.merge === undefined) throw new Error(`${testCase.id}: the route merged an archive the case scripts no merge for`);
        return archive.merge === "unmergeable" ? { kind: "unmergeable" } : { kind: "merged", ...archive.merge };
      },
      load: async (archive) => {
        calls.push("load");
        if (archive.load === undefined) throw new Error(`${testCase.id}: the route loaded an archive the case scripts no load for`);
        if (archive.load === "cancelled") throw cancel();
        if (archive.load === "failed") throw new Error("plugin.document-load.history-invalid");
        return archive.load === "loaded";
      },
      cancelled,
      detach: async (failure) => {
        calls.push("detach");
        expect([testCase.id, failure === null ? "cancelled" : failure.error instanceof Error ? failure.error.message : "unknown"]).toEqual([testCase.id, answers.load === "cancelled" ? "cancelled" : "plugin.document-load.history-invalid"]);
      },
      history: async () => { calls.push("history"); },
      refresh: async () => { calls.push("refresh"); },
    }).then((ended): Outcome => ended, (error: unknown): Outcome => {
      if (!(error instanceof Error) || error.message !== "plugin.document-load.history-invalid") throw error;
      return "failed";
    });
    detached ||= outcome === "detached";
    outcomes.push(outcome);
  }
  return { calls, outcomes };
}

/** 🧮️ This law's own reducer of the rule. */
function model(testCase: Case): Run {
  const calls: string[] = [];
  const outcomes: Outcome[] = [];
  let adopted = false;
  let detached = false;
  for (const event of testCase.events) {
    if ("reconnect" in event) {
      adopted = false;
      detached = false;
      outcomes.push("reattached");
      continue;
    }
    if (detached) {
      outcomes.push("superseded");
      continue;
    }
    if ("absent" in event) {
      adopted = true;
      calls.push("write");
      outcomes.push("adopted");
      continue;
    }
    const { merge, load } = event.archive;
    if (adopted) {
      calls.push("merge");
      if (merge !== "unmergeable" && merge !== undefined) {
        if (merge.merged > 0) calls.push("history", "refresh");
        if (merge.ahead > 0) calls.push("write");
        outcomes.push("held");
        continue;
      }
    }
    calls.push("load");
    if (load === "loaded") {
      calls.push("history", "refresh");
      adopted = true;
      outcomes.push("held");
    } else if (load === "cancelled" || load === "failed") {
      if (adopted) outcomes.push(load);
      else {
        calls.push("detach");
        detached = true;
        outcomes.push("detached");
      }
    } else outcomes.push("superseded");
  }
  return { calls, outcomes };
}

test("the corpus is valid and hostile rows are refused", () => {
  const first = corpus.cases[0]!;
});

test("every corpus case drives exactly its calls, in the route and in the reducer", async () => {
  for (const testCase of corpus.cases) {
    const scripted = { calls: [...testCase.calls], outcomes: [...testCase.outcomes] };
    expect(model(testCase), `${testCase.id} (reducer)`).toEqual(scripted);
    expect(await replay(testCase), `${testCase.id} (route)`).toEqual(scripted);
  }
});

test("a read-back that is no longer current re-reads and writes nothing", async () => {
  const calls: string[] = [];
  const route = new FolderReadBackRouteV1<string>(new FolderArchivePersistenceV1(async () => { calls.push("write"); }));
  await route.absent();
  const ports = { merge: async (): Promise<FolderReadBackMergeV1> => ({ kind: "merged", merged: 3, ahead: 2 }), load: async () => true, cancelled, detach: async () => { calls.push("detach"); }, history: async () => { calls.push("history"); }, refresh: async () => { calls.push("refresh"); } };
  expect(await route.archive("archive", () => false, ports)).toBe("superseded");
  expect(calls).toEqual(["write"]);
});

test("one read-back is driven at a time, the newest waiting one replaces the one before it, and a superseded merge still refreshes what it took", async () => {
  const calls: string[] = [];
  const route = new FolderReadBackRouteV1<string>(new FolderArchivePersistenceV1(async () => { calls.push("write"); }));
  await route.absent();
  const gates = new Map<string, () => void>();
  let driving = 0;
  const ports = {
    merge: async (archive: string): Promise<FolderReadBackMergeV1> => {
      driving += 1;
      expect(driving).toBe(1);
      calls.push(`merge ${archive}`);
      await new Promise<void>((resolve) => gates.set(archive, resolve));
      driving -= 1;
      return { kind: "merged", merged: 1, ahead: 0 };
    },
    load: async () => { throw new Error("an adopted folder's read-back is never loaded"); },
    cancelled,
    detach: async () => { calls.push("detach"); },
    history: async () => { calls.push("history"); },
    refresh: async () => { calls.push("refresh"); },
  };
  const first = route.archive("first", () => true, ports);
  await Promise.resolve();
  await Promise.resolve();
  const second = route.archive("second", () => true, ports);
  const third = route.archive("third", () => true, ports);
  expect(await second).toBe("superseded");
  gates.get("first")!();
  expect(await first).toBe("held");
  for (let turn = 0; turn < 8 && !gates.has("third"); turn += 1) await Promise.resolve();
  gates.get("third")!();
  expect(await third).toBe("held");
  expect(calls).toEqual(["write", "merge first", "history", "refresh", "merge third", "history", "refresh"]);
});

test("only the load's own rejection is read as a cancel: a cancel-shaped failure of the re-read still fails the read-back", async () => {
  const route = new FolderReadBackRouteV1<string>(new FolderArchivePersistenceV1(async () => {}));
  const ports = { merge: async (): Promise<FolderReadBackMergeV1> => ({ kind: "unmergeable" }), load: async () => true, cancelled, detach: async () => { throw new Error("a re-read that fails detaches nothing"); }, history: async () => { throw cancel(); }, refresh: async () => {} };
  await expect(route.archive("archive", () => true, ports)).rejects.toThrow("the person cancelled the load");
});
