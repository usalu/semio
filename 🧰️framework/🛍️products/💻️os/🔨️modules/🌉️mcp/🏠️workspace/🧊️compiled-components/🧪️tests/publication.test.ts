import { expect, test } from "bun:test";
import Ajv from "ajv";
import actor from "../../../../🔌️plugin/🧠️interpreter/🏃️runtime/🏗️startup/🧪️tests/🧫️fixtures/prepared.json";
import law from "./🧫️fixtures/publication.json";
import schema from "./🧬️schema/publication.json";

test("only a live owning runtime publishes the prepared Wasm module", async () => {
  expect(new Ajv({ strict: true }).compile(schema)(law)).toBe(true);
  const bytes = Buffer.from(actor.coreHex, "hex");
  const digest = new Bun.CryptoHasher("sha256").update(bytes).digest("hex");
  const authority = new Map<string, WebAssembly.Module>();

  for (const row of law.steps) {
    const caller = new AbortController();
    let preparedActor = false;
    if (row.cancel === "at-cache-check") {
      expect(authority.has(digest)).toBe(true);
      caller.abort();
    }
    let published: WebAssembly.Module | undefined;
    if (!caller.signal.aborted) {
      published = row.runtime === "shared" ? authority.get(digest) : undefined;
      if (!published) {
        const prepared = await WebAssembly.compile(bytes);
        const wasm = new WebAssembly.Instance(prepared).exports;
        expect(WebAssembly.Module.imports(prepared)).toEqual([]);
        expect((wasm.semio_owned_prepare_actor_v1 as () => number)()).toBe(1);
        expect(new Uint8Array((wasm.memory as WebAssembly.Memory).buffer)[actor.marker.offset]).toBe(actor.marker.prepared);
        preparedActor = true;
        if (row.cancel === "before-publication") caller.abort();
        if (!caller.signal.aborted) {
          published = prepared;
          if (row.runtime === "shared") authority.set(digest, prepared);
        }
      }
    }
    const result = !published ? "cancelled" : published === authority.get(digest) ? "owner" : "local";
    expect(result).toBe(row.result);
    expect(authority.has(digest)).toBe(row.shared);
    expect(preparedActor).toBe(row.prepared);
  }
});
