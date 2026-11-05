/** 📃️ Neutral page admission laws, checked against SQLite uniqueness and independent JSON serialization. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "./🧫️fixtures/📋️schema.json";

type Payload = { kind: string; encoding: string; value: unknown };
type Grant = { items: number; copyBytes: number; capacityBytes: number; releaseBytes: number; depth: number };
type Event = { type: string; source: string; grant?: Grant };
type Row = { id: string; payload: Payload; cancelPayload?: Payload; events: Event[]; expected: unknown };
type State = { phase: string; cancelled: boolean; ownerRetained: boolean; outboxAccepts: number; queuedPages: number; nextSequence: number; queuedSequence: number | null; allocatedBytes: number; releasedBytes: number; frame: string | null; retainedInput: string | null };
const encode = (payload: Payload): string => payload.encoding === "raw" ? String(payload.value) : JSON.stringify(payload.value);

function exercise(row: Row) {
  const database = new Database(":memory:");
  database.exec("CREATE TABLE outbox(owner TEXT PRIMARY KEY, payload TEXT NOT NULL); CREATE TABLE pages(owner TEXT PRIMARY KEY, sequence INTEGER NOT NULL, frame TEXT NOT NULL);");
  const state: State = { phase: "pending", cancelled: false, ownerRetained: true, outboxAccepts: 0, queuedPages: 0, nextSequence: fixture.initialSequence, queuedSequence: null, allocatedBytes: 0, releasedBytes: 0, frame: null, retainedInput: JSON.stringify(row.payload) };
  const results: string[] = [];
  try {
    for (const event of row.events) {
      const before = structuredClone(state);
      let result: string;
      if (event.source !== "original") result = "wrong-source";
      else if (event.type === "cancel") {
        if (state.phase === "pending") state.cancelled = true;
        result = "cancelled";
      } else if (event.type === "admit") {
        if (state.phase !== "pending") result = "already-queued";
        else {
          const denied = (Object.keys(fixture.quote) as (keyof Grant)[]).find(axis => event.grant![axis] < fixture.quote[axis]);
          if (denied) result = "denied:" + denied;
          else {
            const selected = state.cancelled ? row.cancelPayload! : row.payload;
            const frame = encode(selected);
            database.transaction(() => {
              database.prepare("INSERT INTO pages(owner,sequence,frame) VALUES(?,?,?)").run(row.id, state.nextSequence, frame);
              if (!state.cancelled) database.prepare("INSERT INTO outbox(owner,payload) VALUES(?,?)").run(row.id, JSON.stringify(row.payload));
            })();
            state.phase = "queued";
            state.queuedPages = 1;
            state.queuedSequence = state.nextSequence;
            state.allocatedBytes += fixture.quote.capacityBytes;
            state.releasedBytes += fixture.quote.releaseBytes;
            state.outboxAccepts = state.cancelled ? 0 : 1;
            state.frame = frame;
            result = "queued";
          }
        }
      } else if (event.type === "present") {
        if (state.phase === "queued") { state.phase = "presented"; result = "presented"; }
        else result = "present-refused";
      } else if (event.type === "ack") {
        if (state.phase === "presented") { state.phase = "acknowledged"; state.nextSequence++; result = "acknowledged"; }
        else result = "ack-refused";
      } else if (event.type === "retire" && state.phase === "acknowledged") {
        state.phase = "retired";
        state.ownerRetained = false;
        state.retainedInput = null;
        state.releasedBytes += fixture.quote.capacityBytes;
        result = "retired";
      } else result = "retire-refused";
      results.push(result);
      if (result.startsWith("denied:") || result === "wrong-source" || result === "ack-refused" || result === "already-queued") expect(state).toEqual(before);
      if (state.ownerRetained) expect(state.retainedInput).toBe(JSON.stringify(row.payload));
      expect(database.query("SELECT count(*) AS count FROM pages").get()).toEqual({ count: state.queuedPages });
      expect(database.query("SELECT count(*) AS count FROM outbox").get()).toEqual({ count: state.outboxAccepts });
      if (state.frame !== null) {
        const page = database.query("SELECT sequence,frame FROM pages WHERE owner=?").get(row.id);
        expect(page).toEqual({ sequence: state.queuedSequence, frame: state.frame });
      }
    }
    return { results, state };
  } finally { database.close(); }
}

test("independent Ajv validates the language-neutral admission corpus", () => {
  const validate = new Ajv({ strict: false }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(fixture.cases.map(row => row.id)).size).toBe(fixture.cases.length);
  expect(new Set(fixture.cases.map(row => row.payload.kind))).toEqual(new Set(["laneReceipt", "accepted", "ui", "download", "fault", "terminal"]));
});

for (const row of fixture.cases) test(row.id, () => {
  const actual = exercise(row as Row);
  expect(actual).toEqual(row.expected);
  const selected = actual.state.cancelled ? (row as Row).cancelPayload! : row.payload;
  if (actual.state.frame !== null) expect(actual.state.frame).toBe(encode(selected));
  if (selected.encoding === "json" && actual.state.frame !== null) expect(JSON.parse(actual.state.frame)).toEqual(selected.value);
  if (actual.state.phase === "retired") expect(actual.state.releasedBytes).toBe(fixture.initialOwnedBytes + actual.state.allocatedBytes);
});

test("schema rejects payload-family shape substitutions", () => {
  const validate = new Ajv({ strict: false }).compile(schema);
  for (const payload of [
    { kind: "laneReceipt", encoding: "json", value: ["accepted", 3] },
    { kind: "accepted", encoding: "json", value: { generationBefore: 7, generationAfter: 8 } },
    { kind: "ui", encoding: "json", value: { kind: "partial", utilities: "yes" } },
    { kind: "download", encoding: "json", value: ["name", "text/plain", null] },
    { kind: "fault", encoding: "json", value: { message: "Cancelled" } },
    { kind: "terminal", encoding: "raw", value: 7 },
  ]) {
    const wrong = structuredClone(fixture);
    wrong.cases[0].payload = payload as typeof wrong.cases[0]["payload"];
    expect(validate(wrong)).toBe(false);
  }
});
