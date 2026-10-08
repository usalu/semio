import { existsSync, readFileSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import { playPageHost } from "../../📦️site/📄pages/🟦️.ts";

/** 📂 Resolves only an authored static file; missing assets never fall back to HTML. */
export function releaseAssetPath(root: string, request: string): string | undefined {
  const path = request.split(/[?#]/, 1)[0]!;
  let parts: string[];
  try { parts = path.split("/").slice(1).map(decodeURIComponent); } catch { return undefined; }
  if (parts.some(part => part === "." || part === ".." || /[/\\\0]/.test(part))) return undefined;
  return resolve(root, ...(parts.every(part => part.length === 0) ? ["index.html"] : parts));
}

/** 🌐 Maps exact CDN origins onto independent static listeners for browser verification. */
export function releaseRequestUrl(request: string, origins: Readonly<Record<string, string>>): string | undefined {
  const url = new URL(request), origin = origins[url.origin];
  return origin === undefined ? undefined : new URL(url.pathname + url.search, origin).href;
}

/** 📦 Serves the four emitted deployment pages, preserving file responses, cache and CORS metadata. */
export function servePublishedPlay(pages: string): { readonly baseURL: string; readonly origins: Readonly<Record<string, string>>; stop(): Promise<void> } {
  const names = ["play", "map", "media", "modules"];
  for (const name of names) {
    const root = join(pages, name);
    if (!existsSync(join(root, "CNAME")) || (name === "play" && !existsSync(join(root, "index.html")))) throw new Error(`Missing published Play page ${root}; run the fresh build first`);
  }
  const servers: Bun.Server<undefined>[] = [], origins: Record<string, string> = {};
  try {
    for (const name of names) {
      const root = join(pages, name), metadata = join(root, "_headers");
      const declared = existsSync(metadata) ? readFileSync(metadata, "utf8") : "";
      const cors = declared.includes("Access-Control-Allow-Origin: *"), cacheControl = declared.match(/^\s*Cache-Control:\s*([^\r\n]+)/im)?.[1]?.trim() ?? "no-store";
      const server = Bun.serve({
        hostname: "127.0.0.1", port: 0,
        async fetch(request) {
          const headers: Record<string, string> = { "Cache-Control": cacheControl };
          if (cors) headers["Access-Control-Allow-Origin"] = "*";
          if (!["GET", "HEAD"].includes(request.method)) return new Response(null, { status: 405, headers });
          const path = releaseAssetPath(root, new URL(request.url).pathname);
          if (path === undefined || !existsSync(path) || !statSync(path).isFile()) return new Response("Published file not found", { status: 404, headers });
          const file = Bun.file(path);
          headers["Content-Type"] = file.type;
          headers["Content-Length"] = String(file.size);
          return new Response(request.method === "HEAD" ? null : file, { headers });
        },
      });
      servers.push(server);
      origins[`https://${playPageHost(name, "play.semio-tech.com")}`] = server.url.origin;
    }
    return { baseURL: servers[0]!.url.href, origins, async stop() { await Promise.all(servers.map(server => server.stop(true))); } };
  } catch (error) { for (const server of servers) server.stop(true); throw error; }
}
