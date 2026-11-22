// #region 🔌️Adapters
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { deflateSync } from "node:zlib";
import { dirname, resolve } from "node:path";
import type { OwnedBuildPlugin } from "../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🛠️build-tooling/🟦️.ts";
// #endregion 🔌️Adapters
import type { Issue } from "../../../🔨️modules/🗂️catalog/🟦️.ts";
import type { Asset, Text } from "../../../🧬️schema/🟦️.ts";
import { CHECK_THREW, StaffageFindings, type FindingsOptions } from "./🩺️findings/🟦️.ts";

/** 🔖️ The import query under which a catalog asset file is served as its shipped payload; `import.meta.glob` needs it as a literal, so the loader spells it out. */
export const STAFFAGE_ASSET_QUERY = "?staffage-asset";

/** 🛣️ The path under the site's base at which the development server streams the findings of a served revision: `<base>@staffage-findings/<sha256 hex>`. */
export const STAFFAGE_FINDINGS_ROUTE = "@staffage-findings/";

/** 📦️ One authored asset as it ships: its revision and its compact text, with the place of every scale volume in it. */
export type StaffageAssetPayload = {
  readonly revision: string;
  readonly json: string;
  readonly volumeIndex?: { readonly headerEnd: number; readonly ranges: Readonly<Record<string, readonly [number, number]>> };
};

/** 🗜️ The virtual module stores compressed geometry until the reader selects the asset. A build and a test carry the findings of the check as `issues`; the development server carries the address its findings arrive at as `findings`, so serving the module never waits for the check. */
export type StaffageServedPayload = Omit<StaffageAssetPayload, "json"> & { readonly header: string; readonly scales?: Readonly<Record<string, string>>; readonly drawings?: Readonly<Record<string, string>>; readonly issues?: readonly Issue[]; readonly findings?: string };

/** 📨️ One line of the findings stream (newline-delimited JSON): how far the check is, then once either the findings or `stale` when the file no longer has the asked revision. */
export type StaffageFindingsLine = { readonly progress: readonly [done: number, total: number, label: Text] } | { readonly issues: readonly Issue[] } | { readonly stale: true };

/** 👀️ The part of a Vite or Rollup plugin context that registers a file whose change invalidates the module. */
export type WatchContext = { readonly addWatchFile: (file: string) => void };

/** ♻️ What the development server appends to an asset module: the module takes its own updates, so a change to an authored file reloads only the pages that loaded that asset instead of every open page. */
export const STAFFAGE_ASSET_REFRESH = "\nif (import.meta.hot) import.meta.hot.accept(() => globalThis.location?.reload());";

const PREFIX = "\0staffage-asset:";
const SUFFIX = ".mjs";

function compact(value: unknown): string {
  const negativeZero = (JSON as JSON & { rawJSON(value: string): unknown }).rawJSON("-0");
  return JSON.stringify(value, (_key, field) => Object.is(field, -0) ? negativeZero : field);
}

const pack = (text: string): string => deflateSync(text, { level: 3 }).toString("base64");

/** 🏷️ The revision of an authored asset text: `sha256:` of the text with line breaks normalised to `\n`, exactly as `revisionOfText` states it.
 * @see ../../../🔨️modules/🏷️revision/🟦️.ts — `revisionOfText` */
export const staffageRevision = (text: string): string => `sha256:${createHash("sha256").update(text.replace(/\r\n/gu, "\n"), "utf8").digest("hex")}`;

function payloadOfText(text: string): StaffageAssetPayload {
  const revision = staffageRevision(text);
  const asset = (JSON.parse(text) as { asset: Asset }).asset;
  if (!(asset?.volumes && typeof asset.volumes === "object" && !Array.isArray(asset.volumes))) return { revision, json: compact(asset) };
  const { volumes, ...header } = asset;
  let json = compact(header).slice(0, -1);
  const headerEnd = json.length;
  json += (headerEnd > 1 ? "," : "") + '"volumes":{';
  const ranges: Record<string, [number, number]> = {};
  for (const [index, [key, volume]] of Object.entries(volumes).entries()) {
    json += (index ? "," : "") + JSON.stringify(key) + ":";
    const start = json.length;
    json += compact(volume);
    ranges[key] = [start, json.length];
  }
  return { revision, json: json + "}}", volumeIndex: { headerEnd, ranges } };
}

/** 🗜️ Reads one authored asset file: its revision, and the asset stripped of its indentation and envelope so the browser parses it without hashing anything.
 * @see ../🔨️modules/📚️catalog/🟦️.ts — the loader
 * @see {@link staffageRevision} */
export const staffageAssetPayload = (file: string): StaffageAssetPayload => payloadOfText(readFileSync(file, "utf8"));

function servedModule(payload: StaffageAssetPayload, verdict: { readonly issues: readonly Issue[] } | { readonly findings: string }): string {
  const { json, ...metadata } = payload;
  const index = payload.volumeIndex;
  let text = index ? json.slice(0, index.headerEnd) + "}" : json;
  const scales = index && Object.fromEntries(Object.entries(index.ranges).map(([scale, [start, end]]) => [scale, pack(json.slice(start, end))]));
  const header = JSON.parse(text);
  const drawings: Record<string, string> = {};
  for (const [scale, representation] of Object.entries(header?.representations ?? {})) {
    if (representation && typeof representation === "object" && "geometry" in representation) {
      drawings[scale] = pack(compact(representation.geometry));
      delete representation.geometry;
    }
  }
  if (Object.keys(drawings).length) text = compact(header);
  const served: StaffageServedPayload = { drawings, ...metadata, ...verdict, header: pack(text), ...(scales && { scales }) };
  return `export default ${JSON.stringify(served)};`;
}

/** 🧍️ Serves a revisioned asset module and never checks an asset on the thread that serves: the findings come from {@link StaffageFindings}.
 *
 * - The development server answers at once with the address of the findings; `<base>@staffage-findings/<revision>` then streams the progress of the check and its findings as {@link StaffageFindingsLine}s, and a reader who closes the stream withdraws their question.
 *   The module accepts its own updates ({@link STAFFAGE_ASSET_REFRESH}). The check is not part of the server's own program, so a change to a rule no longer restarts the server: a changed TypeScript source ends the checking threads and forgets the kept findings, and the next question is answered by the new rules.
 * - A build and a test wait for the findings and carry them in the module; several modules are checked at once.
 * - A release rejects a module with findings, naming the authored file and every finding in English and German.
 * @see {@link staffageAssetPayload} */
export function staffageAssetVitePlugin(release = false, options: FindingsOptions = {}): OwnedBuildPlugin {
  let root = process.cwd();
  let route: string | null = null;
  let service: StaffageFindings | undefined;
  const findings = (): StaffageFindings => (service ??= new StaffageFindings(options));
  const close = async (): Promise<void> => {
    const closing = service;
    service = undefined;
    await closing?.close();
  };
  const files = new Map<string, string>();
  const revisions = new Map<string, string>();
  return {
    name: "staffage-asset-payload",
    enforce: "pre",
    configResolved(config) {
      root = config.root;
    },
    configureServer(server) {
      if (server.httpServer === null) return;
      route = `${server.config.base}${STAFFAGE_FINDINGS_ROUTE}`;
      const prefix = route;
      server.httpServer.once("close", () => void close());
      server.watcher.on("change", (file) => void (file.endsWith(".ts") && close()));
      server.middlewares.use((request, response, next) => {
        const url = request.url ?? "";
        if (!url.startsWith(prefix)) return next();
        const revision = `sha256:${url.slice(prefix.length).split("?")[0]}`;
        const file = files.get(revision);
        const line = (value: StaffageFindingsLine): string => `${JSON.stringify(value)}\n`;
        response.setHeader("content-type", "application/x-ndjson; charset=utf-8");
        response.setHeader("cache-control", "no-store");
        let text: string | undefined;
        try {
          text = file === undefined ? undefined : readFileSync(file, "utf8");
        } catch {
          text = undefined;
        }
        if (text === undefined || staffageRevision(text) !== revision) return void response.end(line({ stale: true }));
        const ticket = findings().ask(revision, text, (done, total, label) => void response.write(line({ progress: [done, total, label] })));
        let settled = false;
        response.on("close", () => settled || ticket.cancel());
        ticket.findings.then(
          (issues) => {
            settled = true;
            response.end(line({ issues }));
          },
          (error: unknown) => {
            settled = true;
            if (!response.writableEnded) response.end(line(error instanceof DOMException && error.name === "AbortError" ? { stale: true } : { issues: [CHECK_THREW] }));
          },
        );
      });
    },
    resolveId(id, importer) {
      if (!id.endsWith(STAFFAGE_ASSET_QUERY)) return null;
      const path = id.slice(0, -STAFFAGE_ASSET_QUERY.length);
      const from = importer === undefined ? root : dirname(importer.split("?")[0]!);
      const file = path.startsWith(".") ? resolve(from, path) : existsSync(path) ? path : resolve(root, `.${path}`);
      return `${PREFIX}${file}${SUFFIX}`;
    },
    load(this: WatchContext | void, id: string) {
      if (!id.startsWith(PREFIX)) return null;
      const file = id.slice(PREFIX.length, -SUFFIX.length);
      this?.addWatchFile(file);
      const text = readFileSync(file, "utf8");
      const payload = payloadOfText(text);
      if (route !== null) {
        const previous = revisions.get(file);
        if (previous !== undefined) files.delete(previous);
        revisions.set(file, payload.revision);
        files.set(payload.revision, file);
        return servedModule(payload, { findings: `${route}${payload.revision.slice("sha256:".length)}` }) + STAFFAGE_ASSET_REFRESH;
      }
      return findings().ask(payload.revision, text).findings.catch((): readonly Issue[] => [CHECK_THREW]).then((issues) => {
        if (release && issues.length) throw new Error([file, ...issues.map(({ code, path, message }) => `${path} [${code}] ${message.en} / ${message.de}`)].join("\n"));
        return servedModule(payload, { issues });
      });
    },
    closeBundle: close,
  };
}
