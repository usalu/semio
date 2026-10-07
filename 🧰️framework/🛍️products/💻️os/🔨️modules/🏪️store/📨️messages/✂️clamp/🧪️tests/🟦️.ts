/** ✂️ The platform stable sort independently reproduces the bounded message ledger law. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { applyPatch } from "fast-json-patch";

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const law = read("../🧫️fixtures/🔣️.json");
const rank: Record<string, number> = { info: 0, warning: 1, error: 2, fatal: 3 };
const bytes = (text: string) => Buffer.byteLength(text);
type Message = { level: string; code: string; message: string; target: string[]; opIndex: number | null };
const size = (message: Message) => law.messageOwnerBytes + bytes(message.code) + bytes(message.message) + message.target.reduce((n, text) => n + bytes(text) + law.targetSegmentBytes, 0);

/** 🧮️ A stable-sort oracle chooses rows independently of the bounded cursor's severity passes. */
function oracle(row: any) {
  const messages: Message[] = Array.from({ length: row.messageRepeat ?? 1 }, () => row.messages).flat().map((message: any) => ({ ...message, message: message.text.repeat(message.repeat), target: Array.from({ length: message.targetRepeat ?? 1 }, () => message.target).flat() }));
  if (messages.reduce((n, message) => n + size(message), bytes(row.editId)) <= law.entryBytes)
    return { changed: false, kept: messages.map((_, index) => index), dropped: 0, summaryOp: null, truncatedUtf8Bytes: null };
  const budget = law.entryBytes - bytes(row.editId) - law.messageOwnerBytes - bytes("mutation.cascade") - bytes(`${messages.length} more messages`);
  const ranked = messages.map((message, index) => ({ index, message })).sort((a, b) => rank[b.message.level] - rank[a.message.level]);
  const kept = new Set<number>();
  let used = 0, truncatedUtf8Bytes: number | null = null;
  for (const { index, message } of ranked) {
    if (used + size(message) > budget) break;
    used += size(message);
    kept.add(index);
  }
  if (!kept.size && ranked.length) {
    const { index, message } = ranked[0];
    const target = Math.max(0, budget - law.messageOwnerBytes - bytes(message.code));
    let prefix = "";
    for (const character of message.message) {
      if (bytes(prefix) + bytes(character) > target) break;
      prefix += character;
    }
    kept.add(index);
    truncatedUtf8Bytes = bytes(prefix);
  }
  const dropped = messages.length - kept.size;
  const firstDropped = messages.findIndex((_, index) => !kept.has(index));
  return { changed: true, kept: [...kept].sort((a, b) => a - b), dropped, summaryOp: firstDropped < 0 ? null : messages[firstDropped].opIndex, truncatedUtf8Bytes };
}

test("✂️ bounded ledger fixture matches independent severity sorting and UTF-8 prefix selection", () => {
  for (const row of law.cases) expect(oracle(row), row.name).toEqual(row.expected);
});

test("🛑️ granted message retirement preserves the document baseline at every cancellation point", () => {
  for (const grant of law.workGrants) for (const cancelledAt of law.cancelAt) {
    let state = { baseline: { artifact: "unchanged", alternatives: ["original"] }, messages: Array.from({ length: cancelledAt }, (_, index) => index) };
    const baseline = structuredClone(state.baseline);
    while (state.messages.length) {
      const before = state.messages.length;
      for (let count = 0; count < grant && state.messages.length; count++)
        state = applyPatch(state, [{ op: "remove", path: "/messages/0" }], true, false).newDocument;
      expect(before - state.messages.length).toBeLessThanOrEqual(grant);
      expect(state.baseline).toEqual(baseline);
    }
    expect(state.messages).toEqual([]);
  }
});

/** 🚧️ Visible detail pruning preserves the original status of every fatal mutation. */
test("🚧️ summarized fatal messages preserve every repairable mutation status", () => {
  const outcomes = Array.from({ length: law.settlement.operations }, (_, opIndex) => ({ opIndex, worst: "fatal", messages: [{ code: "mutation.invariant", message: "n invariant violated" }] }));
  const selected = new Set(Array.from({ length: 90 }, (_, index) => index));
  const original = outcomes.map(outcome => outcome.worst);
  const bounded = outcomes.map(outcome => applyPatch(outcome, [{ op: "replace", path: "/messages", value: selected.has(outcome.opIndex) ? outcome.messages : [] }], true, false).newDocument);
  expect(bounded.map(outcome => outcome.worst)).toEqual(original);
  expect(bounded.every(outcome => outcome.worst === "fatal")).toBe(law.settlement.preserveFullWorst);
  expect(bounded.filter(outcome => outcome.messages.length === 0).length).toBeGreaterThan(0);
});

/** 📶️ An edit seal remains counted work after its last operation was prepared. */
test("📶️ unsettled diagnostic views keep replay progress below completion", () => {
  const { operations, edits, pendingWork, finishedWork } = law.settlement;
  const total = operations + edits;
  const pending = applyPatch({ done: 0, total }, [{ op: "replace", path: "/done", value: operations }], true, false).newDocument;
  expect(pending).toEqual(pendingWork);
  expect(pending.done).toBeLessThan(pending.total);
  const finished = applyPatch(pending, [{ op: "replace", path: "/done", value: operations + edits }], true, false).newDocument;
  expect(finished).toEqual(finishedWork);
  expect(finished.done).toBe(finished.total);
});
