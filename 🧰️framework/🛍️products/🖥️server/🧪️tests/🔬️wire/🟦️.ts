import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { HttpRequest, HttpTransport } from "../../🟦️.ts";

type TestSource = { readonly directory: string; readonly url: string };

/** 🔌️ The TypeScript half of the server wire gate: every shared vector round-trips byte-for-byte, the route table mirrors the
 * gateway router, and the typed client speaks the same wire shape. */
export async function registerServerWireTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "SERVER_ROUTES" | "ServerClient" | "WireError" | "decodeActorKey" | "decodeCommandEnvelope" | "decodeCommandOutcome" | "decodeCommandReceipt" | "decodeDocumentFrame" | "decodeEphemeralFrame" | "decodeEventRecord" | "decodeEventStreamFrame" | "decodeFrontierSummary" | "decodeHybridLogicalClock" | "decodePrincipal" | "decodeQueryConsistency" | "decodeQueryEnvelope" | "decodeQueryResult" | "decodeRejection" | "decodeServerInstanceDefinition" | "decodeTraceContext" | "documentLane" | "encodeActorKey" | "encodeCommandEnvelope" | "encodeCommandOutcome" | "encodeCommandReceipt" | "encodeEphemeralFrame" | "encodeEventRecord" | "encodeFrontierSummary" | "encodeHybridLogicalClock" | "encodePrincipal" | "encodeQueryConsistency" | "encodeQueryEnvelope" | "encodeQueryResult" | "encodeRejection" | "encodeServerInstanceDefinition" | "encodeTraceContext" | "ephemeralLane" | "fetchTransport" | "socketRoot" | "streamLane">, source: TestSource): Promise<void> {
  const { SERVER_ROUTES, ServerClient, WireError, decodeActorKey, decodeCommandEnvelope, decodeCommandOutcome, decodeCommandReceipt, decodeDocumentFrame, decodeEphemeralFrame, decodeEventRecord, decodeEventStreamFrame, decodeFrontierSummary, decodeHybridLogicalClock, decodePrincipal, decodeQueryConsistency, decodeQueryEnvelope, decodeQueryResult, decodeRejection, decodeServerInstanceDefinition, decodeTraceContext, documentLane, encodeActorKey, encodeCommandEnvelope, encodeCommandOutcome, encodeCommandReceipt, encodeEphemeralFrame, encodeEventRecord, encodeFrontierSummary, encodeHybridLogicalClock, encodePrincipal, encodeQueryConsistency, encodeQueryEnvelope, encodeQueryResult, encodeRejection, encodeServerInstanceDefinition, encodeTraceContext, ephemeralLane, fetchTransport, socketRoot, streamLane } = dependencies;
  const { describe, expect, it } = vitest;

  const productRoot = dirname(fileURLToPath(source.url));
  const fixture = JSON.parse(readFileSync(resolve(productRoot, "🧫️fixtures/🔌️wire/🔣️.json"), "utf8")) as {
    schema: string;
    routes: { method: string; path: string }[];
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
  });

  describe("🛣️routes", () => {
    const routerSource = readFileSync(resolve(productRoot, "🔨️modules/📡️gateway/🦀️.rs"), "utf8");
    const baseRouter = routerSource.slice(routerSource.indexOf("fn base_router<I: ServerInstance>"));
    const block = baseRouter.slice(0, baseRouter.indexOf("\n}"));
    const mounted = block
      .split("\n")
      .filter((line) => line.includes('.route("'))
      .flatMap((line) => {
        const path = /\.route\("([^"]+)"/u.exec(line)![1];
        const verbs = line.slice(line.indexOf(path) + path.length);
        return [...verbs.matchAll(/\b(get|post|put|head)\(/gu)].map(([, verb]) => ({ method: verb.toUpperCase(), path }));
      });

    it("mirrors the router the gateway actually mounts", () => {
      expect(mounted).toEqual(fixture.routes);
      expect(SERVER_ROUTES).toEqual(fixture.routes);
    });

    it("keys the same lanes the gateway does", () => {
      expect(streamLane({ tenant: "t1", kind: "counter", id: "c1" })).toBe("stream:t1/counter/c1");
      expect(documentLane("space-1")).toBe("document:space-1");
      expect(ephemeralLane("space-1")).toBe("ephemeral:space-1");
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

    it("answers a blob negotiation from the status alone", async () => {
      const { transport } = recording((request) => ({ status: request.method === "HEAD" ? 404 : 200, body: "" }));
      expect(await new ServerClient(transport).hasBlob("00".repeat(32))).toBe(false);
    });

    it("builds both socket urls from the same base", () => {
      const client = new ServerClient(fetchTransport("http://127.0.0.1:6081"));
      expect(client.eventStreamUrl("http://127.0.0.1:6081", { tenant: "t1", kind: "counter", id: "c1" }, 9)).toBe("ws://127.0.0.1:6081/actors/t1/counter/c1/events/ws?since=9");
      expect(client.documentSocketUrl("http://127.0.0.1:6081", "space-1", { surface: "editor" })).toBe("ws://127.0.0.1:6081/scopes/space-1/document/ws?surface=editor");
      expect(client.documentSocketUrl("http://127.0.0.1:6081", "space-1")).toBe("ws://127.0.0.1:6081/scopes/space-1/document/ws");
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
