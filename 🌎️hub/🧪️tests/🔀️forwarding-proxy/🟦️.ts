/** 🔀️ Laws of the hub image drill's language-agnostic contract (`🧫️fixtures/🐳️docker-image-v1`): every Docker argv the
 * harness runs, and the loopback forwarding proxy that stands in for a production hub's TLS-terminating reverse proxy —
 * checked live against an upstream that records what reached it, with Node's own `http` client and the third-party `ws`
 * client as independent oracles for the HTTP and WebSocket paths. */
import { request as httpRequest } from "node:http";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { hubForwardedHeaders, hubForwardingProxy } from "../../🤝️integration-harness/🟦️.ts";
import { hubImageBuildArguments, hubImageCredentialArguments, hubImageRunArguments, hubImageRunNames, hubImageSeedArguments } from "../🐳️docker-image/🟦️.ts";

const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/🐳️docker-image-v1/🔣️.json"), "utf8"));

type WsOracle = { protocol: string; binaryType: string; on(event: string, listener: (...args: any[]) => void): void; send(data: Uint8Array | string): void; close(): void };
type WsOracleConstructor = new (url: string, protocols: string[]) => WsOracle;

describe("hub docker image fixture", () => {
  it("derives every Docker argv the drill runs from its inputs", () => {
    expect(fixture.schema).toBe("semio.hub.docker-image-fixture/v1");
    const names = hubImageRunNames(fixture.runNames.tag, fixture.runNames.runId);
    expect(names).toEqual(fixture.runNames.expected);
    for (const row of fixture.build) expect(hubImageBuildArguments({ tag: row.tag, jobs: row.jobs })).toEqual(row.argv);
    expect(hubImageSeedArguments(names)).toEqual(fixture.seed);
    expect(hubImageCredentialArguments(names, fixture.credential.email, fixture.credential.displayName)).toEqual(fixture.credential.argv);
    expect(hubImageRunArguments(names, fixture.run)).toEqual(fixture.run.argv);
  });

  it("forwards the client's headers minus hop-by-hop ones plus its own TLS statement", () => {
    const forwarded = hubForwardedHeaders(new Headers(fixture.forwarding.incoming), fixture.forwarding.forwardedHost);
    for (const [name, value] of Object.entries(fixture.forwarding.expected)) expect(forwarded.get(name)).toBe(value);
    for (const name of fixture.forwarding.dropped) expect(forwarded.has(name)).toBe(false);
  });
});

describe.skipIf(typeof Bun === "undefined")("hub forwarding proxy", () => {
  it("carries HTTP and WebSocket traffic to the upstream stamped as TLS-terminated", async () => {
    const seen: { headers: Record<string, string>; subprotocols: string }[] = [];
    const upstream = Bun.serve<{ proto: string }>({
      hostname: "127.0.0.1",
      port: 0,
      fetch(request, server) {
        const headers = Object.fromEntries([...request.headers.entries()]);
        seen.push({ headers, subprotocols: request.headers.get("sec-websocket-protocol") ?? "" });
        if (request.headers.get("upgrade")?.toLowerCase() === "websocket") return server.upgrade(request, { data: { proto: headers["x-forwarded-proto"] ?? "" }, headers: { "Sec-WebSocket-Protocol": fixture.forwarding.subprotocols[0] } }) ? undefined : new Response(null, { status: 400 });
        return Response.json(headers, { status: 207 });
      },
      websocket: {
        message(socket, message) {
          if (message === "close") return socket.close(fixture.forwarding.closeCode, "fixture");
          socket.send(message);
        },
      },
    });
    const proxy = hubForwardingProxy(`http://127.0.0.1:${upstream.port}`, fixture.forwarding.forwardedHost);
    try {
      const proxied = new URL(proxy.origin);
      const echoed = await new Promise<{ status: number; body: Record<string, string> }>((resolveEcho, rejectEcho) => {
        const outgoing = httpRequest({ host: proxied.hostname, port: Number(proxied.port), path: "/readyz?probe=1", method: "POST", headers: { ...fixture.forwarding.incoming, host: proxied.host, "content-type": "application/json" } }, (response) => {
          let text = "";
          response.setEncoding("utf8");
          response.on("data", (chunk: string) => (text += chunk));
          response.on("end", () => resolveEcho({ status: response.statusCode ?? 0, body: JSON.parse(text) }));
        });
        outgoing.on("error", rejectEcho);
        outgoing.end(JSON.stringify({ probe: true }));
      });
      expect(echoed.status).toBe(207);
      for (const [name, value] of Object.entries(fixture.forwarding.expected)) expect(echoed.body[name]).toBe(value);
      expect(echoed.body["content-length"]).toBe(String(JSON.stringify({ probe: true }).length));

      const { WebSocket: WsClient } = (await import("ws" as string)) as { WebSocket: WsOracleConstructor };
      const client = new WsClient(`${proxy.origin.replace(/^http/u, "ws")}/scopes/fixture/document/ws?surface=s`, fixture.forwarding.subprotocols);
      client.binaryType = "nodebuffer";
      await new Promise<void>((resolveOpen, rejectOpen) => {
        client.on("open", () => resolveOpen());
        client.on("error", rejectOpen);
      });
      expect(client.protocol).toBe(fixture.forwarding.subprotocols[0]);
      const payload = new Uint8Array([0, 1, 2, 250, 251, 252, 253, 254, 255]);
      const echo = await new Promise<Uint8Array>((resolveFrame) => {
        client.on("message", (data: Buffer) => resolveFrame(new Uint8Array(data)));
        client.send(payload);
      });
      expect([...echo]).toEqual([...payload]);
      const closed = await new Promise<number>((resolveClose) => {
        client.on("close", (code: number) => resolveClose(code));
        client.send("close");
      });
      expect(closed).toBe(fixture.forwarding.closeCode);
      const upgrade = seen.find((row) => row.subprotocols !== "");
      expect(upgrade?.subprotocols).toBe(fixture.forwarding.subprotocols.join(", "));
      expect(upgrade?.headers["x-forwarded-proto"]).toBe("https");
      expect(upgrade?.headers["x-forwarded-host"]).toBe(fixture.forwarding.forwardedHost);
      expect(proxy.requests()).toBe(2);
    } finally {
      proxy.stop();
      upstream.stop(true);
    }
  });
});
