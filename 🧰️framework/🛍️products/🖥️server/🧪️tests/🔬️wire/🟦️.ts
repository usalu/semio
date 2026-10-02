import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { HttpRequest, HttpTransport } from "../../🟦️.ts";

type TestSource = { readonly directory: string; readonly url: string };

/** 🔌️ The TypeScript half of the server wire gate: every shared vector round-trips byte-for-byte, the route table mirrors the
 * gateway router, and the typed client speaks the same wire shape. */
export async function registerServerWireTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "PRESENCE_PROTOCOL" | "SERVER_ERRORS" | "SERVER_ROUTES" | "ServerCallError" | "ServerClient" | "WireError" | "decodeErrorBody" | "encodeErrorBody" | "serverCallError" | "decodeActorKey" | "decodeCommandEnvelope" | "decodeCommandOutcome" | "decodeCommandReceipt" | "decodeDocumentFrame" | "decodeEphemeralFrame" | "decodeEventRecord" | "decodeEventStreamFrame" | "decodeFrontierSummary" | "decodeHybridLogicalClock" | "decodePresenceFrame" | "decodePrincipal" | "decodeQueryConsistency" | "decodeQueryEnvelope" | "decodeQueryResult" | "decodeRejection" | "decodeServerInstanceDefinition" | "decodeTraceContext" | "documentLane" | "encodeActorKey" | "encodeCommandEnvelope" | "encodeCommandOutcome" | "encodeCommandReceipt" | "encodeEphemeralFrame" | "encodeEventRecord" | "encodeFrontierSummary" | "encodeHybridLogicalClock" | "encodePresenceFrame" | "encodePrincipal" | "encodeQueryConsistency" | "encodeQueryEnvelope" | "encodeQueryResult" | "encodeRejection" | "encodeServerInstanceDefinition" | "encodeTraceContext" | "ephemeralLane" | "fetchTransport" | "presenceLane" | "presenceSocketUrl" | "socketRoot" | "streamLane">, source: TestSource): Promise<void> {
  const { PRESENCE_PROTOCOL, SERVER_ERRORS, SERVER_ROUTES, ServerCallError, ServerClient, WireError, decodeErrorBody, encodeErrorBody, serverCallError, decodeActorKey, decodeCommandEnvelope, decodeCommandOutcome, decodeCommandReceipt, decodeDocumentFrame, decodeEphemeralFrame, decodeEventRecord, decodeEventStreamFrame, decodeFrontierSummary, decodeHybridLogicalClock, decodePresenceFrame, decodePrincipal, decodeQueryConsistency, decodeQueryEnvelope, decodeQueryResult, decodeRejection, decodeServerInstanceDefinition, decodeTraceContext, documentLane, encodeActorKey, encodeCommandEnvelope, encodeCommandOutcome, encodeCommandReceipt, encodeEphemeralFrame, encodeEventRecord, encodeFrontierSummary, encodeHybridLogicalClock, encodePresenceFrame, encodePrincipal, encodeQueryConsistency, encodeQueryEnvelope, encodeQueryResult, encodeRejection, encodeServerInstanceDefinition, encodeTraceContext, ephemeralLane, fetchTransport, presenceLane, presenceSocketUrl, socketRoot, streamLane } = dependencies;
  const { describe, expect, it, vi } = vitest;

  const productRoot = dirname(fileURLToPath(source.url));
  const fixture = JSON.parse(readFileSync(resolve(productRoot, "🧫️fixtures/🔌️wire/🔣️.json"), "utf8")) as {
    schema: string;
    routes: { method: string; path: string; group: string }[];
    errors: { status: number; kind: string }[];
    vectors: { name: string; type: string; json: unknown }[];
  };

  const roundTrips: Record<string, (value: unknown) => unknown> = {
    actorKey: (value) => encodeActorKey(decodeActorKey(value)),
    principal: (value) => encodePrincipal(decodePrincipal(value)),
    hybridLogicalClock: (value) => encodeHybridLogicalClock(decodeHybridLogicalClock(value)),
    traceContext: (value) => encodeTraceContext(decodeTraceContext(value)),
    frontierSummary: (value) => encodeFrontierSummary(decodeFrontierSummary(value)),
    eventRecord: (value) => encodeEventRecord(decodeEventRecord(value)),
    ephemeralFrame: (value) => encodeEphemeralFrame(decodeEphemeralFrame(value)),
    commandEnvelope: (value) => encodeCommandEnvelope(decodeCommandEnvelope(value)),
    commandReceipt: (value) => encodeCommandReceipt(decodeCommandReceipt(value)),
    rejection: (value) => encodeRejection(decodeRejection(value)),
    commandOutcome: (value) => encodeCommandOutcome(decodeCommandOutcome(value)),
    queryConsistency: (value) => encodeQueryConsistency(decodeQueryConsistency(value)),
    queryEnvelope: (value) => encodeQueryEnvelope(decodeQueryEnvelope(value)),
    queryResult: (value) => encodeQueryResult(decodeQueryResult(value)),
    serverInstanceDefinition: (value) => encodeServerInstanceDefinition(decodeServerInstanceDefinition(value)),
    presenceFrame: (value) => encodePresenceFrame(decodePresenceFrame(value)),
    errorBody: (value) => encodeErrorBody(decodeErrorBody(value)),
  };

  describe("🔌️wire", () => {
    it("declares the schema every vector belongs to", () => {
      expect(fixture.schema).toBe("semio.framework.server.wire/v1");
      expect(fixture.vectors.length).toBeGreaterThan(0);
      expect(new Set(fixture.vectors.map((vector) => vector.name)).size).toBe(fixture.vectors.length);
    });

    it("round-trips every shared vector byte-for-byte", () => {
      for (const vector of fixture.vectors) {
        const roundTrip = roundTrips[vector.type];
        expect(roundTrip, `no TypeScript twin for vector type ${vector.type}`).toBeTypeOf("function");
        expect(roundTrip(vector.json), vector.name).toEqual(vector.json);
      }
    });

    it("covers every wire type the fixture names", () => {
      expect(new Set(fixture.vectors.map((vector) => vector.type))).toEqual(new Set(Object.keys(roundTrips)));
    });

    it("refuses a payload byte outside 0..=255", () => {
      expect(() => decodeEventRecord({ stream: { tenant: "t", kind: "k", id: "i" }, seq: 1, hlc: { millis: 0, counter: 0 }, kind: "e", payload: [256] })).toThrow(WireError);
    });

    it("names the field a malformed value was found at", () => {
      expect(() => decodeCommandReceipt({ commandId: "c", actor: { tenant: "t", kind: "k", id: "i" }, revision: "nope", acceptedAt: { millis: 0, counter: 0 } })).toThrow("receipt.revision");
    });

    it("carries a vector for every presence frame and refuses a malformed one", () => {
      expect([...new Set(fixture.vectors.filter((vector) => vector.type === "presenceFrame").map((vector) => (vector.json as { type: string }).type))].sort()).toEqual(["batch", "refused", "state", "watch", "watched", "welcome"]);
      expect(encodePresenceFrame({ type: "watch", scopes: ["space-1", "space-1/home"], intervalMs: 250 })).toEqual({ type: "watch", scopes: ["space-1", "space-1/home"], intervalMs: 250 });
      expect(decodePresenceFrame({ type: "watched", scope: "space-1", entries: [], left: ["s"], snapshot: false })).toEqual({ type: "watched", scope: "space-1", entries: [], left: ["s"] });
      expect(encodePresenceFrame({ type: "watched", scope: "space-1", entries: [], left: [], snapshot: true })).toEqual({ type: "watched", scope: "space-1", entries: [], left: [], snapshot: true });
      expect(() => decodePresenceFrame({ type: "watch", scopes: ["a"], intervalMs: 2.5 })).toThrow("presenceFrame.intervalMs");
      expect(() => decodePresenceFrame({ type: "watched", scope: "a", entries: [], left: [], snapshot: "yes" })).toThrow("presenceFrame.snapshot");
      expect(() => decodePresenceFrame({ type: "welcome", session: "s", colour: 256, roster: [] })).toThrow("presenceFrame.colour");
      expect(() => decodePresenceFrame({ type: "state" })).toThrow("presenceFrame.state");
      expect(() => decodePresenceFrame({ type: "cursor" })).toThrow(WireError);
      expect(decodePresenceFrame({ type: "state", state: null })).toEqual({ type: "state", state: null });
    });
  });

  describe("🛣️routes", () => {
    const routerSource = readFileSync(resolve(productRoot, "🔨️modules/📡️gateway/🦀️.rs"), "utf8");
    const baseRouter = routerSource.slice(routerSource.indexOf("fn base_router<I: ServerInstance>"));
    const block = baseRouter.slice(0, baseRouter.indexOf("\n}"));
    let group = "core";
    const mounted = block.split("\n").flatMap((line) => {
      const condition = /^\s*if (?:groups\.(\w+)|\w+) \{$/u.exec(line.trimEnd());
      if (condition) group = condition[1] ?? "documents";
      if (!line.includes('.route("')) return [];
      const path = /\.route\("([^"]+)"/u.exec(line)![1];
      const verbs = line.slice(line.indexOf(path) + path.length);
      return [...verbs.matchAll(/\b(get|post|put|head)\(/gu)].map(([, verb]) => ({ method: verb.toUpperCase(), path, group }));
    });

    it("mirrors the router the gateway actually mounts, group by group", () => {
      expect(mounted).toEqual(fixture.routes);
      expect(SERVER_ROUTES).toEqual(fixture.routes);
      expect(SERVER_ROUTES.filter((route) => route.group === "core").map((route) => route.path)).toEqual(["/instance", "/commands", "/queries", "/scopes/{scope}/presence/ws", "/actors/{tenant}/{kind}/{id}/events", "/actors/{tenant}/{kind}/{id}/events/ws"]);
    });

    it("names every error the gateway answers with", () => {
      expect(SERVER_ERRORS).toEqual(fixture.errors);
      const gateway = routerSource.slice(routerSource.indexOf("pub fn kind(&self) -> &'static str"));
      const kinds = [...gateway.slice(0, gateway.indexOf("\n    }")).matchAll(/=> "(\w+)"/gu)].map(([, kind]) => kind);
      expect(kinds).toEqual(fixture.errors.map((error) => error.kind));
    });

    it("keys the same lanes the gateway does", () => {
      expect(streamLane({ tenant: "t1", kind: "counter", id: "c1" })).toBe("stream:t1/counter/c1");
      expect(documentLane("space-1")).toBe("document:space-1");
      expect(ephemeralLane("space-1")).toBe("ephemeral:space-1");
      expect(presenceLane("space-1")).toBe("presence:space-1");
      expect(socketRoot("http://127.0.0.1:6081/")).toBe("ws://127.0.0.1:6081");
    });
  });

  describe("🖥️client", () => {
    function recording(answer: (request: HttpRequest) => { status: number; body: string }): { transport: HttpTransport; seen: HttpRequest[] } {
      const seen: HttpRequest[] = [];
      return {
        seen,
        transport: {
          async send(request) {
            seen.push(request);
            const { status, body } = answer(request);
            return { status, text: async () => body, bytes: async () => new TextEncoder().encode(body) };
          },
        },
      };
    }

    const receipt = { commandId: "cmd-1", actor: { tenant: "t1", kind: "counter", id: "c1" }, revision: 1, acceptedAt: { millis: 7, counter: 0 } };

    it("submits a command as the wire shape and decodes the outcome", async () => {
      const { transport, seen } = recording(() => ({ status: 200, body: JSON.stringify({ status: "accepted", receipt, events: [], frontier: null }) }));
      const outcome = await new ServerClient(transport, { bearer: "token-1" }).submitCommand(decodeCommandEnvelope(fixture.vectors.find((vector) => vector.type === "commandEnvelope")!.json));
      expect(outcome.status).toBe("accepted");
      expect(seen[0].method).toBe("POST");
      expect(seen[0].path).toBe("/commands");
      expect(seen[0].headers.authorization).toBe("Bearer token-1");
      expect(JSON.parse(seen[0].body as string).trace).toEqual({ trace_id: "trace-1", span_id: "span-1" });
    });

    it("addresses an actor's history with its three path segments", async () => {
      const { transport, seen } = recording(() => ({ status: 200, body: "[]" }));
      await new ServerClient(transport).events({ tenant: "t1", kind: "counter", id: "c1" }, 4);
      expect(seen[0].path).toBe("/actors/t1/counter/c1/events");
      expect(seen[0].query).toEqual({ since: "4" });
    });

    it("turns a gateway refusal into an error carrying its own tag", async () => {
      const { transport } = recording(() => ({ status: 403, body: JSON.stringify({ kind: "forbidden", message: "no" }) }));
      await expect(new ServerClient(transport).apps()).rejects.toMatchObject({ name: "ServerCallError", kind: "forbidden", status: 403 });
    });

    it("carries the wait of a refusal that passes and survives an answer that is no gateway error", async () => {
      const throttled = recording(() => ({ status: 429, body: JSON.stringify({ kind: "throttled", message: "too many requests", retryAfterMs: 1500 }) }));
      await expect(new ServerClient(throttled.transport).instance()).rejects.toMatchObject({ name: "ServerCallError", kind: "throttled", status: 429, retryAfterMs: 1500, allowance: undefined });
      const spent = recording(() => ({ status: 429, body: JSON.stringify({ kind: "throttled", message: "the sign-up allowance of this address is spent", retryAfterMs: 36000, allowance: "sign-up" }) }));
      await expect(new ServerClient(spent.transport).instance()).rejects.toMatchObject({ name: "ServerCallError", kind: "throttled", status: 429, retryAfterMs: 36000, allowance: "sign-up" });
      expect(() => decodeErrorBody({ kind: "throttled", message: "m", allowance: 7 })).toThrow("error.allowance");
      for (const body of ["", "<html>502 Bad Gateway</html>", "null", JSON.stringify({ kind: 7 })]) {
        const proxy = recording(() => ({ status: 502, body }));
        await expect(new ServerClient(proxy.transport).instance(), body).rejects.toMatchObject({ name: "ServerCallError", kind: "unexpected", status: 502, retryAfterMs: undefined });
      }
      expect(serverCallError(403, JSON.stringify({ kind: "forbidden", message: "forbidden" }))).toBeInstanceOf(ServerCallError);
      expect(() => decodeErrorBody({ kind: "throttled", message: "m", retryAfterMs: -1 })).toThrow("error.retryAfterMs");
    });

    it("sends no credentials with a request", async () => {
      const seen: (RequestInit | undefined)[] = [];
      vi.stubGlobal("fetch", async (_url: string, init?: RequestInit) => {
        seen.push(init);
        return new Response("[]", { status: 200 });
      });
      try {
        await new ServerClient(fetchTransport("https://proctor.example"), { bearer: "token-1" }).events({ tenant: "t1", kind: "counter", id: "c1" });
      } finally {
        vi.unstubAllGlobals();
      }
      expect(seen.map((init) => init?.credentials)).toEqual(["omit"]);
      expect((seen[0]?.headers as Record<string, string>).authorization).toBe("Bearer token-1");
    });

    it("answers a blob negotiation from the status alone", async () => {
      const { transport } = recording((request) => ({ status: request.method === "HEAD" ? 404 : 200, body: "" }));
      expect(await new ServerClient(transport).hasBlob("00".repeat(32))).toBe(false);
    });

    it("builds both socket urls from the same base", () => {
      const client = new ServerClient(fetchTransport("http://127.0.0.1:6081"));
      expect(client.eventStreamUrl("http://127.0.0.1:6081", { tenant: "t1", kind: "counter", id: "c1" }, 9)).toBe("ws://127.0.0.1:6081/actors/t1/counter/c1/events/ws?since=9");
      expect(client.documentSocketUrl("http://127.0.0.1:6081", "space-1", { surface: "editor" })).toBe("ws://127.0.0.1:6081/scopes/space-1/document/ws?surface=editor");
      expect(client.documentSocketUrl("http://127.0.0.1:6081", "space-1")).toBe("ws://127.0.0.1:6081/scopes/space-1/document/ws");
      expect(client.presenceSocketUrl("https://proctor.example/", "architecture/quiz/physics", "run")).toBe("wss://proctor.example/scopes/architecture%2Fquiz%2Fphysics/presence/ws?surface=run");
      expect(presenceSocketUrl("http://127.0.0.1:6081", "architecture", "home page")).toBe("ws://127.0.0.1:6081/scopes/architecture/presence/ws?surface=home+page");
      expect(PRESENCE_PROTOCOL).toBe("semio.presence.v1");
    });

    it("classifies both document frame shapes", () => {
      expect(decodeDocumentFrame(new Uint8Array([1, 2]))).toEqual({ kind: "engine", bytes: new Uint8Array([1, 2]) });
      expect(decodeDocumentFrame("storage entry not found")).toEqual({ kind: "error", message: "storage entry not found" });
    });

    it("reads an event-stream frame the gateway sent as text", () => {
      const vector = fixture.vectors.find((entry) => entry.type === "eventRecord")!;
      expect(encodeEventRecord(decodeEventStreamFrame(JSON.stringify(vector.json)))).toEqual(vector.json);
    });
  });
}
