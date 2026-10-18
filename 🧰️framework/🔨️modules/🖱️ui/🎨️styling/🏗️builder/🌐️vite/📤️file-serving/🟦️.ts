import type { IncomingMessage, ServerResponse } from "node:http";
import { statSync, createReadStream } from "node:fs";

export function contentTypeForStaticDirAsset(filePath: string): string | undefined {
  if (filePath.endsWith(".js") || filePath.endsWith(".mjs")) {
    return "text/javascript";
  }
  if (filePath.endsWith(".wasm")) {
    return "application/wasm";
  }
  if (filePath.endsWith(".json")) {
    return "application/json";
  }
  if (filePath.endsWith(".png")) {
    return "image/png";
  }
  if (filePath.endsWith(".jpg") || filePath.endsWith(".jpeg")) {
    return "image/jpeg";
  }
  if (filePath.endsWith(".pdf")) {
    return "application/pdf";
  }
  if (filePath.endsWith(".svg")) {
    return "image/svg+xml";
  }
  return undefined;
}

/** @emoji 🏷️ Serves one file with HTTP validators: a weak `ETag` from its size and nanosecond mtime, `Last-Modified`,
 * `Content-Length` and `Cache-Control: no-cache` (always revalidate — a restage rewrites files in place). A request whose
 * `If-None-Match` names the current tag (weak comparison, RFC 9110 §13.1.2), or — without one — whose `If-Modified-Since`
 * is not older than the file, is answered `304` with no body; `HEAD` gets the headers only.
 *
 * 🐛️ Without validators every reload of `s` re-downloaded every staged descriptor, the 8 MB guest font pack and every
 * component core module in full (measured: 15.7 MB of descriptors + 8.4 MB of fonts per warm reload, ticket 26/09/23 F2);
 * with them an unchanged file costs one `304`. */
export function serveFileWithValidatorsV1(req: IncomingMessage, res: ServerResponse, filePath: string, contentType: string | undefined): void {
  const stats = statSync(filePath, { bigint: true });
  const tag = `W/"${stats.size.toString(16)}-${stats.mtimeNs.toString(16)}"`;
  const modifiedSeconds = Number(stats.mtimeMs / 1000n);
  res.setHeader("ETag", tag);
  res.setHeader("Last-Modified", new Date(modifiedSeconds * 1000).toUTCString());
  res.setHeader("Cache-Control", "no-cache");
  if (contentType) res.setHeader("Content-Type", contentType);
  const matches = req.headers["if-none-match"];
  const since = req.headers["if-modified-since"];
  const opaque = (value: string): string => value.trim().replace(/^W\//u, "");
  const notModified =
    typeof matches === "string"
      ? matches.split(",").some((candidate) => candidate.trim() === "*" || opaque(candidate) === opaque(tag))
      : typeof since === "string" && Number.isFinite(Date.parse(since)) && Math.floor(Date.parse(since) / 1000) >= modifiedSeconds;
  if (notModified) {
    res.statusCode = 304;
    res.end();
    return;
  }
  res.setHeader("Content-Length", stats.size.toString());
  if (req.method === "HEAD") {
    res.end();
    return;
  }
  createReadStream(filePath).pipe(res);
}
