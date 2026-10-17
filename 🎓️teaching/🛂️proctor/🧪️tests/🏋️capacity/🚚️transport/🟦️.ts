/** 🚚️ The transport of the capacity gate: the HTTP and WebSocket client on Bun's own `fetch`, `WebSocket` and sockets, the quiz client's real envelopes and the raw wire of the abusive script. */
import { ProctorUnavailable, RETRY_TIMING, retryTransient } from "../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🛂️proctor/🟦️.ts";
import { PRESENCE_PROTOCOL } from "../../../../../🧰️framework/🛍️products/🖥️server/🧬️wire-types/🟦️.ts";
import { type HttpRequest, type HttpResponse, type HttpTransport } from "../../../../../🧰️framework/🛍️products/🖥️server/🚚️transport/🟦️.ts";
import { presenceSocketUrl } from "../../../../../🧰️framework/🛍️products/🖥️server/🛣️routes-and-sockets/🟦️.ts";
import { fail, GIVE_UP_MS, type Measurements, NO_ANSWER, record, SITE, WATCH_INTERVAL_MS } from "../📊️settings-and-measurements/🟦️.ts";

//#region 🚚️Transport
export const pause = (milliseconds: number): Promise<void> => new Promise((done) => setTimeout(done, milliseconds));
export const between = (low: number, high: number): number => low + Math.random() * (high - low);

/** 📨️ The headers the terminating proxy forwards a request of the site with, for the client at `address`. */
export function forwarded(address: string): Record<string, string> {
  return { "x-forwarded-proto": "https", "x-forwarded-for": address, origin: SITE };
}

/** 🚚️ `fetch` against the proctor as the proxy would forward the client at `address`. A request that got no answer —
 * no connection, a connection cut — is {@link ProctorUnavailable}, as it is to the quiz client's own transport. */
export function transport(base: string, address: string, seen?: (status: number) => void): HttpTransport {
  return {
    async send(request: HttpRequest): Promise<HttpResponse> {
      const query = new URLSearchParams(request.query as Record<string, string>).toString();
      let status: number;
      let text: string;
      try {
        const response = await fetch(`${base}${request.path}${query === "" ? "" : `?${query}`}`, { method: request.method, headers: { ...request.headers, ...forwarded(address) }, body: request.body === undefined ? undefined : typeof request.body === "string" ? request.body : new Uint8Array(request.body) });
        status = response.status;
        text = await response.text();
      } catch (error) {
        throw new ProctorUnavailable(error instanceof Error ? `${(error as { code?: string }).code ?? error.name}: ${error.message}` : String(error));
      }
      seen?.(status);
      return { status, text: async () => text, bytes: async () => new TextEncoder().encode(text) };
    },
  };
}

/** 🚪️ One presence socket of a learner: joins `scope`, counts what arrives, resends its latest state after a reconnect. */
export class Room {
  private socket: WebSocket | undefined;
  private latest: unknown;
  private watching: readonly string[] | undefined;
  private gone = false;

  constructor(
    private readonly base: string,
    private readonly address: string,
    private readonly scope: string,
    private readonly surface: string,
    private readonly into: () => Measurements,
  ) {}

  /** 🔌️ Join the room as the quiz client's room does: a join that got no answer is made again after the client's own
   * backoff, and the time until the `welcome` — the waits included — is what the learner waited. */
  async join(): Promise<void> {
    const started = performance.now();
    let attempts = 0;
    try {
      await retryTransient(
        () => {
          attempts += 1;
          return this.gone ? Promise.resolve() : this.open(started);
        },
        RETRY_TIMING,
        AbortSignal.timeout(GIVE_UP_MS),
      );
    } catch (error) {
      fail(this.into(), `presence ${this.scope}: ${error instanceof Error ? `${error.name}: ${error.message}` : String(error)}`);
    } finally {
      this.into().sockets.rejoined += Math.max(0, attempts - 1);
    }
    if (this.latest !== undefined) this.send({ type: "state", state: this.latest });
    if (this.watching !== undefined) this.send({ type: "watch", scopes: this.watching, intervalMs: WATCH_INTERVAL_MS });
  }

  /** 🚪️ Open one socket and wait for its `welcome`. A socket that never opened because there was no connection or the
   * connection was cut ({@link NO_ANSWER}) is {@link ProctorUnavailable}, as a request without an answer is; a join the
   * proctor answered with a refusal, and a socket that closes after its welcome, are errors of the hall. */
  private open(started: number): Promise<void> {
    const socket = new WebSocket(presenceSocketUrl(this.base, this.scope, this.surface), { headers: forwarded(this.address), protocols: [PRESENCE_PROTOCOL] } as unknown as string[]);
    this.socket = socket;
    const into = this.into();
    return new Promise<void>((welcomed, unanswered) => {
      let greeted = false;
      socket.addEventListener("message", (event) => {
        const text = typeof event.data === "string" ? event.data : "";
        const now = this.into();
        now.sockets.frames += 1;
        now.sockets.bytes += text.length;
        if (!greeted) {
          greeted = true;
          record(into, "socket join", performance.now() - started);
          into.sockets.opened += 1;
          welcomed();
        } else if (text.startsWith('{"type":"refused"')) {
          now.sockets.refused += 1;
          fail(now, `presence ${this.scope} refused a frame: ${text.slice(0, 160)}`);
        }
      });
      socket.addEventListener("close", (event) => {
        if (this.socket === socket) {
          this.socket = undefined;
          if (!greeted && event.code === NO_ANSWER) {
            unanswered(new ProctorUnavailable(`no welcome from ${this.scope} (${event.reason || event.code})`));
            return;
          }
          const now = this.into();
          now.sockets.dropped += 1;
          fail(now, `presence ${this.scope} closed (${event.code}${greeted ? "" : ", before its welcome"})`);
        }
        welcomed();
      });
      socket.addEventListener("error", () => undefined);
    });
  }

  /** 📣️ Share `state` (the latest wins; it is sent again after a reconnect). */
  share(state: unknown): void {
    this.latest = state;
    this.send({ type: "state", state });
  }

  /** 👀️ Watch `scopes` read-only. */
  watch(scopes: readonly string[]): void {
    this.watching = scopes;
    this.send({ type: "watch", scopes, intervalMs: WATCH_INTERVAL_MS });
  }

  /** 📶️ Drop the socket as a network would and join again after `afterMs`, unless the learner left meanwhile. */
  async reconnect(afterMs: number): Promise<void> {
    this.drop();
    await pause(afterMs);
    if (!this.gone) await this.join();
  }

  /** 🚶️ Leave the room for good. */
  leave(): void {
    this.gone = true;
    this.drop();
  }

  private drop(): void {
    const socket = this.socket;
    this.socket = undefined;
    socket?.close();
  }

  private send(frame: unknown): void {
    if (this.socket?.readyState === WebSocket.OPEN) this.socket.send(JSON.stringify(frame));
  }
}
//#endregion 🚚️Transport
