
//#region 🔖️StatusSurfaceHtml
/** 🎨️ Light/dark background+foreground hex pair mirrored from {@link PLAYGROUND_PLAY_BOOT_INLINE_STYLE}
 * / {@link PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT} — this file has no `../🎨️styling/🔣️.json` import, so these are the
 * canonical values already baked into every other boot surface here, not new ones. */
const SEMIO_STATUS_SURFACE_COLORS = { lightBg: "#f7f3e3", lightFg: "#001117", darkBg: "#001117", darkFg: "#f7f3e3" } as const;

const SEMIO_STATUS_SURFACE_GLYPH: Record<"empty" | "error" | "loading", string> = { empty: "◌️", error: "⚠️", loading: "…" };

function semioStatusSurfaceInlineStyle(): string {
  const c = SEMIO_STATUS_SURFACE_COLORS;
  return `html{color-scheme:light dark}html,body{height:100dvh;margin:0}body{background-color:${c.lightBg};color:${c.lightFg};display:flex;align-items:center;justify-content:center;font-family:system-ui,sans-serif}@media (prefers-color-scheme: dark){body{background-color:${c.darkBg};color:${c.darkFg}}}`;
}

/** 🚦️ Minimal, standalone status document (empty/error/loading) for host-agnostic contexts that
 * can't run React — e.g. a WebView2 navigation-failure page — fully inline-styled so it renders with zero
 * external CSS/JS dependency, reusing the same light/dark hex values every other boot surface in this
 * file uses. */
export function statusSurfaceHtml(spec: { readonly kind: "empty" | "error" | "loading"; readonly title: string; readonly description?: string }): string {
  const description = spec.description ? `<p style="margin:8px 0 0;font-size:14px;opacity:0.72">${spec.description}</p>` : "";
  return `<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>${spec.title}</title>
    <style>${semioStatusSurfaceInlineStyle()}</style>
  </head>
  <body data-status-kind="${spec.kind}">
    <div style="text-align:center;max-width:28rem;padding:0 24px">
      <p style="margin:0 0 8px;font-size:28px" aria-hidden="true">${SEMIO_STATUS_SURFACE_GLYPH[spec.kind]}</p>
      <p style="margin:0;font-size:16px;font-weight:600">${spec.title}</p>
      ${description}
    </div>
  </body>
</html>
`;
}
//#endregion 🔖️StatusSurfaceHtml

//#region 🔖️ServeClose
/** 🧷️ The part of a dev serve's HTTP server that closing it needs; an HTTP/2 server has no `closeAllConnections`. */
export type ServeCloseHttpServer = {
  close(callback?: (error?: Error) => void): unknown;
  closeAllConnections(): void;
  readonly listening: boolean;
  emit(event: "close"): boolean;
};

/** 🔎️ Whether a serve's HTTP server can close every connection it holds. */
export function isServeCloseHttpServer(value: object | null): value is ServeCloseHttpServer {
  return value !== null && typeof (value as Partial<ServeCloseHttpServer>).closeAllConnections === "function" && typeof (value as Partial<ServeCloseHttpServer>).close === "function";
}

/** 🚪️ Makes closing a dev serve mean what Vite asks of it under every runtime: every open connection gone, `close`
 * emitted once, the callback called. Vite destroys the sockets it tracked and then waits for `server.close`; Bun 1.3's
 * `node:http` ignores `socket.destroy()` on a served connection, so its graceful `close` never completes while a browser holds
 * a keep-alive connection. `server.restart()` (every edit of the config entry) then stopped listening and never listened
 * again, shutdown hung, and every resource released on the server's `close` event (source watchers, the stream channel)
 * stayed live — measured: bun 1.3.14 unreachable 10 s after a restart under a connected page, node 24 restarted in 4 ms
 * (ticket 26/09/23 F2, `📓️wp-f2.md`). `closeAllConnections` is the runtime's own "close every connection": on Node it
 * destroys them and `close` proceeds as usual; on Bun it stops the server outright, so the plugin completes the close.
 *
 * @see https://nodejs.org/api/http.html#servercloseallconnections
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/♨️hot-update/🟦️.ts */
export function semioServeCloseVitePlugin() {
  return {
    name: "semio-serve-close",
    apply: "serve" as const,
    configureServer(server: { readonly httpServer: object | null }) {
      const httpServer = server.httpServer;
      if (!isServeCloseHttpServer(httpServer)) return;
      const close = httpServer.close.bind(httpServer);
      httpServer.close = (callback) => {
        const listening = httpServer.listening;
        httpServer.closeAllConnections();
        if (!listening || httpServer.listening) return close(callback);
        process.nextTick(() => {
          httpServer.emit("close");
          callback?.();
        });
        return httpServer;
      };
    },
  };
}
//#endregion 🔖️ServeClose

/** 🧦️ The part of the socket of an HTTP upgrade that ending it softly needs. */
export type ServeUpgradeSocket = {
  destroySoon?: () => void;
  readonly writable: boolean;
  readonly writableFinished: boolean;
  end(): unknown;
  destroy(): unknown;
  once(event: "finish", listener: () => void): unknown;
};

/** 🧦️ The part of a dev serve's HTTP server that sees every upgrade first. */
export type ServeUpgradeHttpServer = {
  prependListener(event: "upgrade", listener: (request: unknown, socket: ServeUpgradeSocket) => void): unknown;
};

/** 🔎️ Whether a serve's HTTP server lets a listener see an upgrade before every other one. */
export function isServeUpgradeHttpServer(value: object | null): value is ServeUpgradeHttpServer {
  return value !== null && typeof (value as Partial<ServeUpgradeHttpServer>).prependListener === "function";
}

/** 🕊️ Ends `socket` once what was written to it is flushed, as Node's `Socket#destroySoon` does: end it while it is
 * writable, then destroy it at once when it is finished, else as soon as it finishes.
 * @see https://github.com/nodejs/node/blob/main/lib/net.js — `Socket.prototype.destroySoon` */
export function destroySoon(socket: ServeUpgradeSocket): void {
  if (socket.writable) socket.end();
  if (socket.writableFinished) socket.destroy();
  else socket.once("finish", () => socket.destroy());
}

/** 🧦️ Lets every socket an upgrade arrives on be ended softly under every runtime. The dev server's proxy of a
 * WebSocket route ends the browser's socket with `destroySoon` once its upstream answers without upgrading or ends — a
 * backend that restarts or goes away does exactly that — and Bun 1.3's `node:http` hands upgrades a socket without
 * `destroySoon`: the `TypeError` killed the whole dev server whenever its backend went away while a socket was proxied.
 * The listener runs before the proxy's and gives such a socket {@link destroySoon}. */
export function semioServeUpgradeVitePlugin() {
  return {
    name: "semio-serve-upgrade",
    apply: "serve" as const,
    configureServer(server: { readonly httpServer: object | null }) {
      const httpServer = server.httpServer;
      if (!isServeUpgradeHttpServer(httpServer)) return;
      httpServer.prependListener("upgrade", (_request, socket) => {
        if (typeof socket.destroySoon !== "function") socket.destroySoon = () => destroySoon(socket);
      });
    },
  };
}
