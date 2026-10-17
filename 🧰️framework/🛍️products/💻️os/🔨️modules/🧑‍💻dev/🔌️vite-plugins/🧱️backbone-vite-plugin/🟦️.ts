/** 🧱️ The backbone Vite plugin of the dev server: the request and response shapes, the canonical bootstrap mirror header helpers and the plugin that serves backbone document IO. */
import { BACKBONE_ENDPOINT_PATH, DEV_STREAM_ROUTES, STREAM_MUX_ANNOUNCEMENT, STREAM_MUX_PATH, backboneKindFromUri } from "@semio-tech/framework-os";
import { CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BODY_BYTES, CANONICAL_BOOTSTRAP_FOLDER_MIRROR_PATH, type CanonicalBootstrapFolderMirrorControlV1, type CanonicalBootstrapFolderMirrorReserveV1, publishCanonicalBootstrapFolderMirror, reserveCanonicalBootstrapFolderMirror, retireCanonicalBootstrapFolderMirror, stageCanonicalBootstrapFolderMirror, validateCanonicalBootstrapControl } from "../📇️bootstrap-folder-mirror/🟦️.ts";
import { canonicalBootstrapMirrorError, readBackbonePayload, watchFolderChanges, writeBackbonePayload } from "../🗄️backbone-storage/🟦️.ts";
import { devStreamMuxServer, type DevUpgradeServer } from "../🔀️stream-mux-server/🟦️.ts";

//#region BackboneVitePlugin
export type BackboneServerRequest = { method?: string; url?: string; headers?: Record<string, string | string[] | undefined>; on: (event: string, handler: (chunk?: unknown) => void) => void; off: (event: string, handler: (chunk?: unknown) => void) => void };
export type BackboneServerResponse = { statusCode: number; setHeader: (name: string, value: string) => void; write: (chunk: string) => void; end: (body?: string | Uint8Array) => void };

function canonicalBootstrapMirrorControlFromHeaders(headers: BackboneServerRequest["headers"]): CanonicalBootstrapFolderMirrorControlV1 {
  const epochSource = headers?.["x-semio-canonical-bootstrap-epoch"];
  const authorization = headers?.authorization;
  if (Array.isArray(epochSource) || Array.isArray(authorization) || !/^[1-9][0-9]*$/u.test(epochSource ?? "") || !authorization?.startsWith("SemioFolderBootstrap ")) throw canonicalBootstrapMirrorError("invalid");
  const epoch = Number(epochSource);
  const control = { epoch, capability: authorization.slice("SemioFolderBootstrap ".length) };
  validateCanonicalBootstrapControl(control);
  return control;
}

function canonicalBootstrapMirrorContentType(headers: BackboneServerRequest["headers"]): string | null {
  const source = headers?.["content-type"];
  return typeof source === "string" ? source.trim().toLowerCase() : null;
}

function canonicalBootstrapMirrorStatus(error: unknown): number {
  const code = typeof error === "object" && error !== null && "code" in error ? (error as { code?: unknown }).code : undefined;
  return code === "conflict" ? 409 : code === "too-large" ? 413 : 400;
}

function collectBackboneRequestBody(req: BackboneServerRequest, limit: number, complete: (body: Uint8Array | null) => void): void {
  const chunks: Buffer[] = [];
  let length = 0;
  let exceeded = false;
  req.on("data", (chunk) => {
    if (exceeded) return;
    const bytes = Buffer.isBuffer(chunk) ? chunk : Buffer.from(String(chunk));
    length += bytes.byteLength;
    if (length > limit) {
      exceeded = true;
      chunks.length = 0;
      return;
    }
    chunks.push(bytes);
  });
  req.on("end", () => complete(exceeded ? null : new Uint8Array(Buffer.concat(chunks))));
}

/** 💾️ Vite middleware for browser file/folder backbone IO: `GET|PUT ${BACKBONE_ENDPOINT_PATH}?uri=&documentId=&schema=`
 * for read/write — a document nothing has written yet reads as `204 No Content`, the ordinary first-boot answer, never a
 * `404` in the console of a fresh data root (ticket 26/09/23 U5) — plus the `backbone.folder` stream route (key = the `folder://`
 * uri) on the dev stream channel for external-edit notices; `🏪️store/👷️worker/🟦️.ts`'s folder transport falls back to its slow
 * sanity poll while no stream is open. While serving it announces the channel to the page (`STREAM_MUX_ANNOUNCEMENT`), so a
 * page dials it exactly where it exists. */
export function semioBackboneVitePlugin() {
  return {
    name: "semio-backbone",
    config(_config: unknown, env: { readonly command: string }) {
      return env.command === "serve" ? { define: { [`import.meta.env.${STREAM_MUX_ANNOUNCEMENT}`]: JSON.stringify(STREAM_MUX_PATH) } } : undefined;
    },
    configureServer(server: { middlewares: { use: (handler: (req: BackboneServerRequest, res: BackboneServerResponse, next: () => void) => void) => void }; httpServer?: DevUpgradeServer | null }) {
      devStreamMuxServer(server.httpServer).route(DEV_STREAM_ROUTES.backboneFolder, {
        admit: (uri) => backboneKindFromUri(uri) === "folder",
        coalesce: () => "changed",
        source: (uri, emit) => watchFolderChanges(uri, () => emit("changed")),
      });
      server.middlewares.use((req, res, next) => {
        if (!req.url?.startsWith(BACKBONE_ENDPOINT_PATH)) return next();
        const requestUrl = new URL(req.url, "http://127.0.0.1");
        const uri = requestUrl.searchParams.get("uri");
        if (!uri) {
          res.statusCode = 400;
          res.end("missing uri");
          return;
        }
        const documentId = requestUrl.searchParams.get("documentId");
        const schema = requestUrl.searchParams.get("schema");
        if (requestUrl.pathname.startsWith(`${CANONICAL_BOOTSTRAP_FOLDER_MIRROR_PATH}/`)) {
          const action = requestUrl.pathname.slice(CANONICAL_BOOTSTRAP_FOLDER_MIRROR_PATH.length + 1);
          if (documentId === null || backboneKindFromUri(uri) !== "folder" || requestUrl.searchParams.size !== 2 || !["reserve", "stage", "publish", "retire"].includes(action)) {
            res.statusCode = 400;
            res.end("invalid canonical bootstrap folder mirror request");
            return;
          }
          if ((action === "stage" && req.method !== "PUT") || (action !== "stage" && req.method !== "POST")) {
            res.statusCode = 405;
            res.end("method not allowed");
            return;
          }
          const expectedContentType = action === "reserve" ? "application/json" : action === "stage" ? "application/octet-stream" : null;
          if (canonicalBootstrapMirrorContentType(req.headers) !== expectedContentType) {
            res.statusCode = 415;
            res.end("unsupported media type");
            return;
          }
          const limit = action === "reserve" ? 8_192 : action === "stage" ? CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BODY_BYTES : 0;
          collectBackboneRequestBody(req, limit, (body) => {
            void (async () => {
              if (body === null) throw canonicalBootstrapMirrorError("too-large");
              if (action === "reserve") {
                const source = new TextDecoder("utf-8", { fatal: true }).decode(body);
                const parsed = JSON.parse(source) as CanonicalBootstrapFolderMirrorReserveV1;
                if (JSON.stringify(parsed) !== source) throw canonicalBootstrapMirrorError("invalid");
                const owner = await reserveCanonicalBootstrapFolderMirror(uri, documentId, parsed);
                res.statusCode = 201;
                res.setHeader("content-type", "application/json");
                res.setHeader("cache-control", "no-store");
                res.end(`${JSON.stringify(owner)}\n`);
                return;
              }
              const control = canonicalBootstrapMirrorControlFromHeaders(req.headers);
              if (action === "stage") await stageCanonicalBootstrapFolderMirror(uri, documentId, control, body);
              else if (action === "publish") await publishCanonicalBootstrapFolderMirror(uri, documentId, control);
              else await retireCanonicalBootstrapFolderMirror(uri, documentId, control);
              res.statusCode = 204;
              res.end();
            })().catch((error) => {
              res.statusCode = canonicalBootstrapMirrorStatus(error);
              res.setHeader("content-type", "application/json");
              res.end(`${JSON.stringify({ error: error instanceof Error ? error.message : "canonical bootstrap folder mirror invalid" })}\n`);
            });
          });
          return;
        }
        if (req.method === "GET") {
          readBackbonePayload(uri, documentId)
            .then((payload) => {
              if (payload == null) {
                res.statusCode = 204;
                res.setHeader("cache-control", "no-store");
                res.end();
                return;
              }
              res.statusCode = 200;
              res.setHeader("content-type", "application/octet-stream");
              res.end(Buffer.from(payload));
            })
            .catch((error) => {
              res.statusCode = 500;
              res.end(String(error));
            });
          return;
        }
        if (req.method === "PUT") {
          collectBackboneRequestBody(req, CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BODY_BYTES, (body) => {
            if (body === null) {
              res.statusCode = 413;
              res.end("payload too large");
              return;
            }
            writeBackbonePayload(uri, documentId, schema, body)
              .then(() => {
                res.statusCode = 200;
                res.setHeader("content-type", "application/octet-stream");
                res.end(new Uint8Array());
              })
              .catch((error) => {
                res.statusCode = 500;
                res.end(String(error));
              });
          });
          return;
        }
        res.statusCode = 405;
        res.end("method not allowed");
      });
    },
  };
}
//#endregion BackboneVitePlugin
