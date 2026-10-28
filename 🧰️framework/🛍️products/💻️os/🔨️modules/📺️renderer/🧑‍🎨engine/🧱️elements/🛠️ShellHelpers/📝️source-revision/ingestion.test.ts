import { expect, test } from "bun:test";
import { LocalDocumentOwnerRegistryV1 } from "../../🗣️Interpreter/🧭️local-document-owner/🟦️.ts";
import { LocalSourceRevisionRegistryV1 } from "./registry.ts";
import { beginDocumentSourceWorkV1 } from "./document.ts";
import { DocumentSourceIngestionV1 } from "./ingestion.ts";

function fixture() {
  const owners = new LocalDocumentOwnerRegistryV1(), sources = new LocalSourceRevisionRegistryV1();
  const identity = { pluginId: "ifc", appId: "model", sessionInstanceId: 7, runtimeKey: "document", clientInstanceId: "client" };
  const plugin = { subscribeOperationCompletions: () => () => {} };
  const host = { owners, sources, refresh: () => {} };
  const entry = {};
  const begin = () => beginDocumentSourceWorkV1(host, identity, plugin);
  const owner = owners.acquire(identity), source = sources.acquire(owner, plugin)!;
  return { entry, begin, owner, source, plugin, retire: () => owners.retire(identity) };
}

test("queued custody transfers to both original receiving holds before queue retirement", async () => {
  const f = fixture(), ingestion = new DocumentSourceIngestionV1<object>();
  let starts = 0;
  const begin = () => { starts++; return f.begin(); };
  ingestion.queued(f.entry, begin);
  ingestion.queued(f.entry, begin);
  expect(starts).toBe(1);
  let finishOne!: () => void, finishTwo!: () => void;
  const one = ingestion.receive(begin, () => new Promise<void>(resolve => { finishOne = resolve; }));
  const two = ingestion.receive(begin, () => new Promise<void>(resolve => { finishTwo = resolve; }));
  expect(starts).toBe(3);
  ingestion.drained(f.entry);
  expect(f.source.begin(f.owner, f.plugin)).toBeNull();
  finishOne(); await one;
  expect(f.source.begin(f.owner, f.plugin)).toBeNull();
  finishTwo(); await two;
  expect(f.source.begin(f.owner, f.plugin)).not.toBeNull();
  ingestion.drained(f.entry);
});

test("both synchronous and asynchronous original receiving failures remain unresolved", async () => {
  for (const synchronous of [true, false]) {
    const f = fixture(), ingestion = new DocumentSourceIngestionV1<object>(), error = new Error("original ingestion failed");
    await expect(ingestion.receive(f.begin, () => { if (synchronous) throw error; return Promise.reject(error); })).rejects.toBe(error);
    expect(f.source.begin(f.owner, f.plugin)).toBeNull();
    f.retire();
    expect(f.source.isBound(f.owner, f.plugin)).toBe(false);
  }
});

test("original receiving hold exists before the producer callback", async () => {
  const f = fixture(), ingestion = new DocumentSourceIngestionV1<object>();
  expect(await ingestion.receive(f.begin, async () => {
    expect(f.source.begin(f.owner, f.plugin)).toBeNull();
    return "original result";
  })).toBe("original result");
  expect(f.source.begin(f.owner, f.plugin)).not.toBeNull();
});

test("a complete archive attempt includes its asynchronous original lane close", async () => {
  const f = fixture(), ingestion = new DocumentSourceIngestionV1<object>();
  let loaded!: () => void, closed!: () => void;
  const load = new Promise<void>(resolve => { loaded = resolve; });
  const close = new Promise<void>(resolve => { closed = resolve; });
  const completion = ingestion.receive(f.begin, async () => {
    try { await load; }
    finally { await close; }
    return true;
  });
  loaded();
  await Promise.resolve();
  expect(f.source.begin(f.owner, f.plugin)).toBeNull();
  closed();
  expect(await completion).toBe(true);
  expect(f.source.begin(f.owner, f.plugin)).not.toBeNull();
});
