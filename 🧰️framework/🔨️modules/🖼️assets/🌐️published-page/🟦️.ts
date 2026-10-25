/** 🛰️ Published page delivery: root-relative asset paths are prefixed with the CDN page that publishes them, and same-origin asset fetches are
 * relocated the same way. The route map is injected by a production build as `SEMIO_PLAY_PAGE_ORIGINS`; without it every function is the identity. */
/// <reference types="vite/client" />

/** 📖️ CDN page origins keyed by root route. Empty unless a production build injects them. */
export function readPublishedPageOrigins(): Readonly<Record<string, string>> {
  let raw: string | undefined;
  try { raw = import.meta.env.SEMIO_PLAY_PAGE_ORIGINS as string | undefined; } catch { return {}; }
  if (!raw) return {};
  const parsed = JSON.parse(raw) as unknown;
  if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) throw new Error("SEMIO_PLAY_PAGE_ORIGINS must be a route map");
  return Object.freeze(parsed as Record<string, string>);
}

/** 🌐️ Prefix a root-relative asset path with the CDN page that publishes it. Absolute URLs stay put. */
export function publishedPageUrl(path: string, origins: Readonly<Record<string, string>> = readPublishedPageOrigins()): string {
  if (!path.startsWith("/") || path.startsWith("//")) return path;
  const encoded = path.split(/[?#]/, 1)[0]!;
  if (/%(?:2f|5c)/iu.test(encoded)) return path;
  let pathname: string;
  try { pathname = decodeURIComponent(encoded); } catch { return path; }
  const key = Object.keys(origins).sort((a, b) => b.length - a.length).find((prefix) => pathname === prefix || pathname.startsWith(`${prefix}/`));
  const origin = key === undefined ? undefined : origins[key];
  return origin === undefined ? path : `${origin.replace(/\/$/, "")}${path}`;
}

/** 📍️ Points a same-origin asset request at the CDN page that publishes that route. */
export function relocatePublishedRequestUrl(url: string, pageOrigin: string, origins: Readonly<Record<string, string>> = readPublishedPageOrigins()): string {
  let path = url;
  if (/^[a-z][a-z0-9+.-]*:/i.test(url)) {
    let parsed: URL;
    try { parsed = new URL(url); } catch { return url; }
    if (parsed.origin !== pageOrigin) return url;
    path = `${parsed.pathname}${parsed.search}${parsed.hash}`;
  }
  const next = publishedPageUrl(path, origins);
  return next === path ? url : next;
}

const publishedFetchInstalled = Symbol.for("semio.publishedPageFetch");

/** 📮️ Sends root-relative and same-origin asset fetches to the page that publishes them. */
export function installPublishedPageFetch(): void {
  const origins = readPublishedPageOrigins();
  const scope = globalThis as typeof globalThis & { fetch?: typeof fetch; location?: { origin: string } };
  if (Object.keys(origins).length === 0 || typeof scope.fetch !== "function" || typeof scope.location?.origin !== "string") return;
  const holder = scope as typeof scope & { [publishedFetchInstalled]?: true };
  if (holder[publishedFetchInstalled]) return;
  holder[publishedFetchInstalled] = true;
  const native = scope.fetch.bind(scope);
  const pageOrigin = scope.location.origin;
  scope.fetch = ((input: RequestInfo | URL, init?: RequestInit) => {
    if (typeof Request !== "undefined" && input instanceof Request) {
      if (input.method !== "GET" && input.method !== "HEAD") return native(input, init);
      const next = relocatePublishedRequestUrl(input.url, pageOrigin, origins);
      return next === input.url ? native(input, init) : native(new Request(next, input), init);
    }
    const raw = typeof input === "string" ? input : input instanceof URL ? input.href : "";
    const next = raw ? relocatePublishedRequestUrl(raw, pageOrigin, origins) : "";
    return native((next || input) as RequestInfo | URL, init);
  }) as typeof fetch;
}

installPublishedPageFetch();
