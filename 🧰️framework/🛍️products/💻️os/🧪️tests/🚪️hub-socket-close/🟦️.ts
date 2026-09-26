/** 🚪️ Every close the browser initiates on a hub socket must actually close it (`📇️directory/🔌️client/🚪️socket-close`). A browser
 * `WebSocket.close(code)` throws `InvalidAccessError` unless `code` is 1000 or in 3000..4999
 * (https://websockets.spec.whatwg.org/#dom-websocket-close); the store worker closed with RFC 6455's 1008/1002, so every refused
 * activation, refused checkpoint pair, actor mismatch or protocol mismatch threw instead of closing and left the hub socket open
 * (ticket 26/09/23 S15, catalog B2 gis sweep: `Failed to execute 'close' on 'WebSocket' … 1008 is neither`). jsdom's WebSocket,
 * which implements the WHATWG close() steps, is the independent oracle for every declared code and for the admissibility rule. */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { BROWSER_HUB_SOCKET_CLOSE_V1, browserSocketCloseCodeAdmissibleV1, closeHubSocketV1, type BrowserHubSocketCloseReasonV1 } from "../../🔨️modules/📇️directory/🔌️client/🚪️socket-close/🟦️.ts";

type OracleSocket = { close(code?: number, reason?: string): void; onerror: (() => void) | null };
type OracleWindow = { WebSocket: new (url: string) => OracleSocket; close(): void };
const oracleWindow = async (): Promise<OracleWindow> => {
  const { JSDOM } = (await import("jsdom" as string)) as { JSDOM: new (html: string, options: { url: string }) => { window: OracleWindow } };
  return new JSDOM("", { url: "http://127.0.0.1/" }).window;
};
const oracleCloses = (window: OracleWindow, code: number, reason: string): boolean => {
  const socket = new window.WebSocket("ws://127.0.0.1:9/");
  socket.onerror = () => undefined;
  try {
    socket.close(code, reason);
    return true;
  } catch (error) {
    expect((error as { name?: string }).name).toBe("InvalidAccessError");
    return false;
  }
};
const source = (path: string) => readFileSync(fileURLToPath(new URL(path, import.meta.url)), "utf8");
const BROWSER_SOCKET_SOURCES = ["../../🔨️modules/🏪️store/👷️worker/🟦️.ts", "../../🟦️.ts"];

describe("🚪️ the browser's own hub socket closes", () => {
  it("closes with every declared code and reason in a WHATWG WebSocket", async () => {
    const window = await oracleWindow();
    try {
      for (const [reason, row] of Object.entries(BROWSER_HUB_SOCKET_CLOSE_V1.codes)) {
        expect(oracleCloses(window, row.code, row.reason), reason).toBe(true);
        expect(new TextEncoder().encode(row.reason).byteLength, reason).toBeLessThanOrEqual(BROWSER_HUB_SOCKET_CLOSE_V1.admissible.reasonMaxBytes);
        expect(row.code, reason).toBe(4000 + (row.rfc6455 % 1000));
        expect(oracleCloses(window, row.rfc6455, row.reason), `${reason} under its RFC 6455 code`).toBe(false);
      }
    } finally {
      window.close();
    }
  });

  it("decides admissibility exactly as the WHATWG oracle does", async () => {
    const window = await oracleWindow();
    try {
      for (const code of [999, 1000, 1001, 1002, 1008, 1011, 1015, 2999, 3000, 3999, 4000, 4002, 4008, 4401, 4999, 5000, 65535]) {
        expect(browserSocketCloseCodeAdmissibleV1(code), String(code)).toBe(oracleCloses(window, code, ""));
      }
    } finally {
      window.close();
    }
  });

  it("hands the socket the declared code and reason", () => {
    const calls: [number | undefined, string | undefined][] = [];
    for (const reason of Object.keys(BROWSER_HUB_SOCKET_CLOSE_V1.codes) as BrowserHubSocketCloseReasonV1[]) closeHubSocketV1({ close: (code, text) => calls.push([code, text]) }, reason);
    expect(calls).toEqual(Object.values(BROWSER_HUB_SOCKET_CLOSE_V1.codes).map((row) => [row.code, row.reason]));
  });

  it("leaves no browser hub socket closed with a literal code a browser refuses", () => {
    for (const path of BROWSER_SOCKET_SOURCES) {
      const refused = [...source(path).matchAll(/\.close\(\s*(\d+)/gu)].map((match) => Number(match[1])).filter((code) => !browserSocketCloseCodeAdmissibleV1(code));
      expect(refused, path).toEqual([]);
    }
  });
});
