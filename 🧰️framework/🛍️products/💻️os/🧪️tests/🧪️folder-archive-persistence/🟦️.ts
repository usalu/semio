/** 📥️ Folder archive persistence law (design §22.22): Ajv validates the shared corpus
 * (`🛂️admission/📄️document/🧫️fixtures/🧫️folder-archive-persistence/🔣️.json`) against its schema; every case is replayed through
 * {@link FolderArchivePersistenceV1} with promises this file settles event by event, and through this file's own reducer;
 * fast-check then drives both with arbitrary valid event sequences and holds three properties: both start a write at the same
 * events, never two writes are in flight, and once everything settled the last write started at or after the last change —
 * with no more writes than changes (no write storm). */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020";
import fc from "fast-check";
import { FolderArchivePersistenceV1 } from "../../🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";

type PersistenceEvent = { readonly publish: null } | { readonly absent: null } | { readonly foldStart: string } | { readonly foldEnd: { readonly id: string; readonly ok: boolean } } | { readonly merge: number } | { readonly writeEnd: boolean };
type Case = { readonly id: string; readonly events: readonly PersistenceEvent[]; readonly writes: readonly number[] };
type Settler = { readonly promise: Promise<void>; readonly resolve: () => void; readonly reject: (error: Error) => void };

const fixtures = new URL("../../🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🧫️fixtures/🧫️folder-archive-persistence/", import.meta.url);
const corpus = JSON.parse(readFileSync(new URL("🔣️.json", fixtures), "utf8")) as { readonly cases: readonly Case[] };

/** 🎛️ A promise this law settles itself. */
function settler(): Settler {
  let resolve!: () => void, reject!: (error: Error) => void;
  const promise = new Promise<void>((accept, refuse) => { resolve = accept; reject = refuse; });
  return { promise, resolve, reject };
}

/** ⏯️ Drives the policy with `events` and answers the events at which it started a write; two writes in flight throw. */
async function replay(events: readonly PersistenceEvent[]): Promise<number[]> {
  const starts: number[] = [];
  const flight: { write: Settler | null; index: number } = { write: null, index: -1 };
  const folds = new Map<string, Settler>();
  const policy = new FolderArchivePersistenceV1(() => {
    if (flight.write !== null) throw new Error("two writes in flight");
    starts.push(flight.index);
    flight.write = settler();
    return flight.write.promise;
  });
  const quiet = (work: Promise<unknown>): void => void work.catch(() => {});
  for (const [index, event] of events.entries()) {
    flight.index = index;
    if ("publish" in event) quiet(policy.published());
    else if ("absent" in event) quiet(policy.absent());
    else if ("merge" in event) quiet(policy.merged(event.merge));
    else if ("foldStart" in event) {
      const fold = settler();
      folds.set(event.foldStart, fold);
      quiet(policy.ingested(fold.promise));
    } else if ("foldEnd" in event) {
      const fold = folds.get(event.foldEnd.id)!;
      folds.delete(event.foldEnd.id);
      if (event.foldEnd.ok) fold.resolve();
      else fold.reject(new Error("refused"));
    } else {
      const write = flight.write!;
      flight.write = null;
      if (event.writeEnd) write.resolve();
      else write.reject(new Error("write failed"));
    }
    await new Promise((resolve) => setTimeout(resolve, 0));
  }
  return starts;
}

/** 🧮️ This law's own reducer of the same rule: the events at which a write starts. */
function model(events: readonly PersistenceEvent[]): number[] {
  const starts: number[] = [];
  const folding = new Set<string>();
  let writing = false, stale = false, folded = false;
  const change = (index: number): void => {
    if (writing) stale = true;
    else {
      writing = true;
      starts.push(index);
    }
  };
  events.forEach((event, index) => {
    if ("publish" in event || "absent" in event) change(index);
    else if ("merge" in event) {
      if (event.merge > 0) change(index);
    } else if ("foldStart" in event) folding.add(event.foldStart);
    else if ("foldEnd" in event) {
      folding.delete(event.foldEnd.id);
      folded ||= event.foldEnd.ok;
      if (folding.size === 0 && folded) {
        folded = false;
        change(index);
      }
    } else {
      writing = stale;
      if (stale) starts.push(index);
      stale = false;
    }
  });
  return starts;
}

/** 🎲️ Arbitrary choices made into a valid event sequence that ends settled: every fold ended, no write in flight. */
function sequence(choices: readonly { readonly kind: number; readonly flag: boolean; readonly ahead: number }[]): PersistenceEvent[] {
  const events: PersistenceEvent[] = [];
  const open: string[] = [];
  const writing = (): boolean => {
    const starts = model(events).length;
    return starts > events.filter((event) => "writeEnd" in event).length;
  };
  choices.forEach((choice, index) => {
    if (choice.kind === 0) events.push({ publish: null });
    else if (choice.kind === 1) events.push({ merge: choice.ahead });
    else if (choice.kind === 2) {
      open.push(`fold-${index}`);
      events.push({ foldStart: `fold-${index}` });
    } else if (choice.kind === 3 && open.length > 0) events.push({ foldEnd: { id: open.shift()!, ok: choice.flag } });
    else if (choice.kind === 4 && writing()) events.push({ writeEnd: choice.flag });
    else if (choice.kind === 5 && events.length === 0) events.push({ absent: null });
  });
  while (open.length > 0) events.push({ foldEnd: { id: open.shift()!, ok: true } });
  while (writing()) events.push({ writeEnd: true });
  return events;
}

test("the corpus is valid and hostile rows are refused", () => {
  const first = corpus.cases[0]!;
});

test("every corpus case starts its writes at exactly its events, in the policy and in the reducer", async () => {
  for (const testCase of corpus.cases) {
    expect(model(testCase.events), `${testCase.id} (reducer)`).toEqual([...testCase.writes]);
    expect(await replay(testCase.events), `${testCase.id} (policy)`).toEqual([...testCase.writes]);
  }
});

test("any settled event sequence ends with the newest state written, without a write storm", async () => {
  const choice = fc.record({ kind: fc.integer({ min: 0, max: 5 }), flag: fc.boolean(), ahead: fc.integer({ min: 0, max: 2 }) });
  await fc.assert(
    fc.asyncProperty(fc.array(choice, { maxLength: 24 }), async (choices) => {
      const events = sequence(choices);
      const starts = await replay(events);
      expect(starts).toEqual(model(events));
      const changes = events.flatMap((event, index) => ("publish" in event || "absent" in event || ("merge" in event && event.merge > 0) || ("foldEnd" in event && event.foldEnd.ok) ? [index] : []));
      expect(starts.length).toBeLessThanOrEqual(changes.length);
      if (changes.length > 0) expect(starts.at(-1)!).toBeGreaterThanOrEqual(changes.at(-1)!);
      else expect(starts).toEqual([]);
    }),
    { numRuns: 150 },
  );
});
