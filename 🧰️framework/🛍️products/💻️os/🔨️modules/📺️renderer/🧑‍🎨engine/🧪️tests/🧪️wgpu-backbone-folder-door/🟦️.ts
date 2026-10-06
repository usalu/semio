import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import { backboneFolderHop, createWgpuPageHostIo } from "../../🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts";

/** 🗃️ The PAGE half of the browser wgpu shell's folder door, over the language-neutral corpus
 * `🧫️fixtures/🧫️wgpu-backbone-folder-door` that the Rust half (`🐚️Shell/🧪️tests/🧪️wgpu-local-folders`) reads too: every
 * request the shell sends becomes exactly the corpus's one fetch against the dev host's backbone route, and every HTTP
 * outcome is answered exactly as the corpus says — the stored archive as base64, nothing-written-yet, a status, or the
 * transport refusal. The corpus itself is checked by Ajv against its schema. */
type DoorRequest = { readonly name: string; readonly json: Record<string, string>; readonly fetch: { readonly method: string; readonly url: string; readonly body: boolean } };
type DoorAnswer = { readonly name: string; readonly verb: "read" | "write"; readonly response: { readonly status?: number; readonly body?: string; readonly error?: string }; readonly page: Record<string, unknown> };

const engine = join(dirname(fileURLToPath(import.meta.url)), "../..");
const readJson = (path: string): unknown => JSON.parse(readFileSync(path, "utf8"));
const corpus = readJson(join(engine, "🧫️fixtures", "🧫️wgpu-backbone-folder-door", "🔣️.json")) as { readonly requests: readonly DoorRequest[]; readonly answers: readonly DoorAnswer[] };
const archive = new Uint8Array([1, 2, 3]);

const base64Bytes = (value: string): Uint8Array => Uint8Array.from(atob(value), (character) => character.charCodeAt(0));

describe("wgpu backbone folder door", () => {
  it("the shared corpus is valid against its schema", () => {
    const validate = new Ajv({ allErrors: true, strict: false }).compile(readJson(join(engine, "🧬️schema", "🔣️wgpu-backbone-folder-door", "🔣️.json")) as object);
    expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
  });

  it("turns every request of the corpus into exactly its one fetch on the backbone route", async () => {
    for (const request of corpus.requests) {
      const calls: { readonly url: string; readonly method: string | undefined; readonly body: unknown }[] = [];
      const fetchImpl = (async (url: string, init?: RequestInit) => {
        calls.push({ url, method: init?.method, body: init?.body });
        return new Response(null, { status: 204 });
      }) as unknown as typeof fetch;
      await backboneFolderHop(request.json as never, request.fetch.body ? archive : null, fetchImpl);
      expect(calls.map((call) => ({ url: call.url, method: call.method })), request.name).toEqual([{ url: request.fetch.url, method: request.fetch.method }]);
      if (request.fetch.body) expect(new Uint8Array(calls[0]!.body as ArrayBuffer), request.name).toEqual(archive);
      else expect(calls[0]!.body, request.name).toBeUndefined();
    }
  });

  it("answers every HTTP outcome of the corpus exactly as the corpus says", async () => {
    for (const answer of corpus.answers) {
      const fetchImpl = (async () => {
        if (answer.response.error !== undefined) throw new TypeError(answer.response.error);
        return new Response(answer.response.body === undefined ? null : (base64Bytes(answer.response.body).slice().buffer as ArrayBuffer), { status: answer.response.status });
      }) as unknown as typeof fetch;
      const request = { op: "backbone-folder", verb: answer.verb, uri: "folder:///Users/ada/drawings", documentId: "board.ports.directed.v1", ...(answer.verb === "write" ? { schema: "s.puzzle.2d" } : {}) } as const;
      expect(await backboneFolderHop(request, answer.verb === "write" ? archive : null, fetchImpl), answer.name).toEqual(answer.page);
    }
  });

  it("refuses a write that carries no archive without fetching, and is reachable through the page door", async () => {
    let fetched = false;
    const fetchImpl = (async () => {
      fetched = true;
      return new Response(null, { status: 200 });
    }) as unknown as typeof fetch;
    expect(await backboneFolderHop({ op: "backbone-folder", verb: "write", uri: "folder:///x", documentId: "d" }, null, fetchImpl)).toEqual({ error: "backbone-folder: a write carries no archive" });
    expect(fetched).toBe(false);
    const door = createWgpuPageHostIo();
    const answer = JSON.parse(await door(JSON.stringify({ op: "backbone-folder", verb: "write", uri: "folder:///x", documentId: "d" }), null)) as { readonly error?: string };
    expect(answer.error).toBe("backbone-folder: a write carries no archive");
  });
});
