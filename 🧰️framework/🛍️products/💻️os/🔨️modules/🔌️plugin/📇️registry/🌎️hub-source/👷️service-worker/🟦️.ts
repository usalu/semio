/**
 * 👷️ The plugin module store's service worker. Registered by the shell at scope `/` before its first plugin worker exists,
 * it controls the page and every plugin worker, and answers exactly one route — `/_semio/plugin-modules/…` — from the
 * persisted store, re-verifying every file on every load (`serveStoredPluginModuleFileV1`). Every other request goes to
 * the network: through a static route where the platform has them, so the page's own traffic never wakes this worker.
 * @see ../🗄️store/🟦️.ts
 */
import { PLUGIN_MODULE_STORE_V1 } from "../🧬️schema/🟦️.ts";
import { serveStoredPluginModuleFileV1 } from "../🗄️store/🟦️.ts";

type ExtendableEventLike = Event & { waitUntil(promise: Promise<unknown>): void };
type InstallEventLike = ExtendableEventLike & { addRoutes?: (rules: readonly unknown[]) => Promise<void> };
type FetchEventLike = Event & { readonly request: Request; respondWith(response: Promise<Response>): void };
type ServiceWorkerScopeLike = typeof globalThis & {
  readonly location: Location;
  skipWaiting(): Promise<void>;
  readonly clients: { claim(): Promise<void> };
  readonly caches: CacheStorage;
  URLPattern?: new (init: { pathname: string }) => unknown;
};

const scope = globalThis as ServiceWorkerScopeLike;

scope.addEventListener("install", (event) => {
  const install = event as InstallEventLike;
  const routes = install.addRoutes && scope.URLPattern
    ? install.addRoutes([
        { condition: { urlPattern: new scope.URLPattern({ pathname: `${PLUGIN_MODULE_STORE_V1.serveRoute}/*` }) }, source: "fetch-event" },
        { condition: { urlPattern: new scope.URLPattern({ pathname: "/*" }) }, source: "network" },
      ]).catch(() => undefined)
    : Promise.resolve();
  install.waitUntil(routes.then(() => scope.skipWaiting()));
});

scope.addEventListener("activate", (event) => {
  (event as ExtendableEventLike).waitUntil(scope.clients.claim());
});

scope.addEventListener("fetch", (event) => {
  const fetchEvent = event as FetchEventLike;
  const url = new URL(fetchEvent.request.url);
  if (url.origin !== scope.location.origin || !url.pathname.startsWith(`${PLUGIN_MODULE_STORE_V1.serveRoute}/`) || fetchEvent.request.method !== "GET") return;
  fetchEvent.respondWith(
    (async () => {
      const answer = await serveStoredPluginModuleFileV1(await scope.caches.open(PLUGIN_MODULE_STORE_V1.name), scope.location.origin, url.pathname);
      if (answer.status === 200) return new Response(answer.body as Uint8Array<ArrayBuffer>, { status: 200, headers: { "content-type": answer.contentType, "content-length": String(answer.body.byteLength), "cache-control": "no-store" } });
      return new Response(JSON.stringify({ error: answer.problem }), { status: 404, headers: { "content-type": "application/json", "cache-control": "no-store" } });
    })(),
  );
});
