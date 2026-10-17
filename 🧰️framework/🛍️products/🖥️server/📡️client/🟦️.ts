/** 📡️ The typed client of the server product over an owned transport, and the websocket root of an http base URL.
 *
 * @see ../🟦️.ts — the TypeScript twin of the server product's wire contract
 */
import type { ActorKey, CommandEnvelope, CommandOutcome, EphemeralFrame, EventRecord, QueryEnvelope, QueryResult, Scope, ServerInstanceDefinition } from "../🧬️wire-types/🟦️.ts";
import { type AppInstall, type BlobReceipt, type HttpRequest, type HttpTransport, ServerCallError } from "../🚚️transport/🟦️.ts";
import { asArray, asNumber, asString, decodeAppInstall, decodeBlobReceipt, decodeCommandOutcome, decodeErrorBody, decodeEventRecord, decodeQueryResult, decodeServerInstanceDefinition, encodeCommandEnvelope, encodeEphemeralFrame, encodeQueryEnvelope } from "../🔤️codec/🟦️.ts";
import { CAPABILITY_HEADER, type DocumentJoin, encodePathSegment, presenceSocketUrl, type ServerCredential, socketRoot } from "../🛣️routes-and-sockets/🟦️.ts";

//#region 🔖️Transport
/** 🧯️ The error of one unsuccessful answer: the gateway's own error body when the text is one. */
export function serverCallError(status: number, text: string): ServerCallError {
  try {
    return new ServerCallError(status, decodeErrorBody(JSON.parse(text)));
  } catch {
    return new ServerCallError(status, { kind: "unexpected", message: text.slice(0, 256) });
  }
}

//#endregion 🔖️Transport
//#region 🔖️Client
/** 🖥️ A typed client over the gateway's own routes.
 *
 * Every method names the route it calls and returns an owned, decoded value — a caller never sees a
 * raw JSON object, and a wire-shape drift surfaces as a [`WireError`] naming the field rather than
 * as `undefined` three layers later. Failures arrive as [`ServerCallError`] carrying the gateway's
 * own `kind` tag, which is the thing to branch on: the status code is a transport detail. */
export class ServerClient {
  private readonly transport: HttpTransport;
  private readonly credential: ServerCredential;

  constructor(transport: HttpTransport, credential: ServerCredential = {}) {
    this.transport = transport;
    this.credential = credential;
  }

  /** 🏛️ `GET /instance` — what this deployment declares itself to be. */
  async instance(): Promise<ServerInstanceDefinition> {
    return decodeServerInstanceDefinition(await this.json("GET", "/instance", {}));
  }

  /** 📨️ `POST /commands` — submit one command and read the authority's outcome. */
  async submitCommand(envelope: CommandEnvelope): Promise<CommandOutcome> {
    return decodeCommandOutcome(await this.json("POST", "/commands", {}, JSON.stringify(encodeCommandEnvelope(envelope))));
  }

  /** ❓️ `POST /queries` — answer one query from the read models. */
  async query(envelope: QueryEnvelope): Promise<QueryResult> {
    return decodeQueryResult(await this.json("POST", "/queries", {}, JSON.stringify(encodeQueryEnvelope(envelope))));
  }

  /** 💨️ `POST /scopes/{scope}/ephemeral` — publish one lossy frame; answers how many sockets got it. */
  async publishEphemeral(frame: EphemeralFrame): Promise<number> {
    const body = await this.json("POST", `/scopes/${encodePathSegment(frame.scope)}/ephemeral`, {}, JSON.stringify(encodeEphemeralFrame(frame)));
    return asNumber(body, "delivered");
  }

  /** 📜️ `GET /actors/{tenant}/{kind}/{id}/events` — one page of durable history after `since`. */
  async events(actor: ActorKey, since = 0): Promise<EventRecord[]> {
    const body = await this.json("GET", this.actorPath(actor, "/events"), { since: String(since) });
    return asArray(body, "events").map((entry, index) => decodeEventRecord(entry, `events[${index}]`));
  }

  /** 📦️ `GET /blobs/{hash}` — content-addressed bytes. */
  async blob(hash: string): Promise<Uint8Array> {
    const response = await this.transport.send({ method: "GET", path: `/blobs/${encodePathSegment(hash)}`, query: {}, headers: this.headers() });
    if (response.status >= 400) throw serverCallError(response.status, await response.text());
    return response.bytes();
  }

  /** ❓️ `HEAD /blobs/{hash}` — the cheap half of an upload negotiation. */
  async hasBlob(hash: string): Promise<boolean> {
    const response = await this.transport.send({ method: "HEAD", path: `/blobs/${encodePathSegment(hash)}`, query: {}, headers: this.headers() });
    if (response.status === 200) return true;
    if (response.status === 404) return false;
    throw new ServerCallError(response.status, { kind: "unexpected", message: `HEAD /blobs answered ${response.status}` });
  }

  /** 💾️ `PUT /blobs/{hash}` — store bytes at a content address the server re-derives and verifies. */
  async putBlob(hash: string, bytes: Uint8Array): Promise<BlobReceipt> {
    const response = await this.transport.send({
      method: "PUT",
      path: `/blobs/${encodePathSegment(hash)}`,
      query: {},
      headers: { ...this.headers(), "content-type": "application/octet-stream" },
      body: bytes,
    });
    const text = await response.text();
    if (response.status >= 400) throw serverCallError(response.status, text);
    return decodeBlobReceipt(JSON.parse(text));
  }

  /** 📋️ `GET /apps` — the names of every static app this instance hosts. */
  async apps(): Promise<string[]> {
    return asArray(await this.json("GET", "/apps", {}), "apps").map((entry, index) => asString(entry, `apps[${index}]`));
  }

  /** 🧩️ `GET /apps/{app}/installs` — the `install.json` entries under one app's root. */
  async appInstalls(app: string): Promise<AppInstall[]> {
    const body = await this.json("GET", `/apps/${encodePathSegment(app)}/installs`, {});
    return asArray(body, "installs").map((entry, index) => decodeAppInstall(entry, `installs[${index}]`));
  }

  /** 📡️ The websocket URL of one actor's durable lane, resuming after `since`. */
  eventStreamUrl(baseUrl: string, actor: ActorKey, since = 0): string {
    return `${socketRoot(baseUrl)}${this.actorPath(actor, "/events/ws")}?since=${encodeURIComponent(String(since))}`;
  }

  /** 📄️ The websocket URL of one scope's document lane. Actor identity is never a query parameter. */
  documentSocketUrl(baseUrl: string, scope: Scope, join: DocumentJoin = {}): string {
    const query = new URLSearchParams();
    if (join.surface !== undefined) query.set("surface", join.surface);
    if (join.resume !== undefined) query.set("resume", join.resume);
    const encoded = query.toString();
    const suffix = encoded.length > 0 ? `?${encoded}` : "";
    return `${socketRoot(baseUrl)}/scopes/${encodePathSegment(scope)}/document/ws${suffix}`;
  }

  /** 👥️ The websocket URL of one scope's presence room — see [`presenceSocketUrl`]. */
  presenceSocketUrl(baseUrl: string, scope: Scope, surface: string): string {
    return presenceSocketUrl(baseUrl, scope, surface);
  }

  private actorPath(actor: ActorKey, suffix: string): string {
    return `/actors/${encodePathSegment(actor.tenant)}/${encodePathSegment(actor.kind)}/${encodePathSegment(actor.id)}${suffix}`;
  }

  private headers(): Record<string, string> {
    const headers: Record<string, string> = {};
    if (this.credential.bearer !== undefined) headers.authorization = `Bearer ${this.credential.bearer}`;
    if (this.credential.capability !== undefined) headers[CAPABILITY_HEADER] = this.credential.capability;
    return headers;
  }

  private async json(method: HttpRequest["method"], path: string, query: Record<string, string>, body?: string): Promise<unknown> {
    const headers = body === undefined ? this.headers() : { ...this.headers(), "content-type": "application/json" };
    const response = await this.transport.send({ method, path, query, headers, body });
    const text = await response.text();
    if (response.status >= 400) throw serverCallError(response.status, text);
    return text === "" ? null : JSON.parse(text);
  }
}
//#endregion 🔖️Client
