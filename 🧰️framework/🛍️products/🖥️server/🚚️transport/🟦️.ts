/** 🛍️ The transport of the server product's TypeScript twin: error bodies, blob and app install receipts, the wire and call errors, and the owned transport interface with its fetch implementation.
 *
 * @see ../🟦️.ts — the TypeScript twin of the server product's wire contract
 */
//#region 🔖️Transport
/** 🚥️ The JSON body every gateway error renders as, so a client never parses a status line. A refusal that passes
 * (`throttled`, `overloaded`) names the wait in milliseconds after which the request is worth repeating. A `throttled`
 * refusal names an `allowance` when what is spent is not the address's allowance of requests but a named one the
 * instance counts a class of commands against — those that make it keep something, like a registration: the request
 * was not too fast, the address has made its share for now, and a client says so instead of only slowing down. */
export interface ErrorBody {
  readonly kind: string;
  readonly message: string;
  readonly retryAfterMs?: number;
  readonly allowance?: string;
}

/** 🧾️ What a successful blob upload answers with. */
export interface BlobReceipt {
  readonly hash: string;
  readonly size: number;
}

/** 🧩️ One installed entry discovered under an app root. */
export interface AppInstall {
  readonly id: string;
  readonly manifest: unknown;
}

/** 💥️ A wire value that does not match the contract. Thrown by every decoder in this module. */
export class WireError extends Error {
  readonly path: string;

  constructor(message: string, path: string) {
    super(path === "" ? message : `${path}: ${message}`);
    this.name = "WireError";
    this.path = path;
  }
}

/** 🚫️ A gateway answer that was not a success, carrying the tag a client branches on and, for a refusal that passes,
 * the wait the gateway named and the named allowance that is spent, when it is one. An answer that is no gateway error
 * body at all — a proxy's own page, an empty reply — carries the kind `unexpected`. */
export class ServerCallError extends Error {
  readonly status: number;
  readonly kind: string;
  readonly retryAfterMs: number | undefined;
  readonly allowance: string | undefined;

  constructor(status: number, body: ErrorBody) {
    super(`${body.kind}: ${body.message}`);
    this.name = "ServerCallError";
    this.status = status;
    this.kind = body.kind;
    this.retryAfterMs = body.retryAfterMs;
    this.allowance = body.allowance;
  }
}

/** 📨️ One request the client asks its transport to perform, in owned terms only. */
export interface HttpRequest {
  readonly method: "GET" | "HEAD" | "POST" | "PUT";
  readonly path: string;
  readonly query: Readonly<Record<string, string>>;
  readonly headers: Readonly<Record<string, string>>;
  readonly body?: string | Uint8Array;
}

/** 📬️ What a transport answers with. Bodies are pulled lazily so a `HEAD` never reads one. */
export interface HttpResponse {
  readonly status: number;
  text(): Promise<string>;
  bytes(): Promise<Uint8Array>;
}

/** 🚚️ The seam between this client and whatever performs the request.
 *
 * An interface rather than `fetch` directly, for the reason the repo states generally: nothing this
 * module exports may name a type it does not own. A host adapts its own `fetch`, its own test double
 * or its own instrumented pipeline with [`fetchTransport`] or by implementing this. */
export interface HttpTransport {
  send(request: HttpRequest): Promise<HttpResponse>;
}

/** 🌐️ Adapt the platform's `fetch` to [`HttpTransport`], rooted at one server's base URL. A request carries what the
 * client put into it and nothing the browser holds for the server's origin (`credentials: "omit"`): the gateway's
 * cross-origin grant never allows credentials. */
export function fetchTransport(baseUrl: string): HttpTransport {
  const root = baseUrl.replace(/\/+$/u, "");
  return {
    async send(request: HttpRequest): Promise<HttpResponse> {
      const query = new URLSearchParams(request.query as Record<string, string>).toString();
      const response = await fetch(`${root}${request.path}${query === "" ? "" : `?${query}`}`, {
        method: request.method,
        headers: { ...request.headers },
        credentials: "omit",
        body: request.body === undefined ? undefined : typeof request.body === "string" ? request.body : new Uint8Array(request.body),
      });
      return {
        status: response.status,
        text: () => response.text(),
        bytes: async () => new Uint8Array(await response.arrayBuffer()),
      };
    },
  };
}
//#endregion 🔖️Transport
