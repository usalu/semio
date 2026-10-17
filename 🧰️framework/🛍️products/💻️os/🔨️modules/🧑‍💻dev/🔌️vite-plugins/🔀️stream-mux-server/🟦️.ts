/** 🔀️ The dev server's stream multiplexer: the upgrade-server shapes, the runtime websocket module and the per-server stream mux. */
import { STREAM_MUX_PATH } from "@semio-tech/framework-os";
import { STREAM_MUX_BOUNDS_V1, StreamMuxServerV1 } from "@semio-tech/framework/io/stream-mux";

//#region BackboneVitePlugin
//#region 🔀️StreamMuxServer
/** 🌐️ The part of a dev server's HTTP server the stream channel needs. */
export type DevUpgradeServer = {
  on(event: "upgrade", listener: (req: { readonly url?: string; readonly headers: Record<string, string | string[] | undefined> }, socket: { destroy(): void }, head: unknown) => void): unknown;
  once(event: "close", listener: () => void): unknown;
};

/** 🔌️ The runtime's own WebSocket (Bun ships `ws` built in) as far as the stream channel uses it. */
type RuntimeWebSocket = {
  readonly bufferedAmount?: number;
  send(frame: string): void;
  close(code: number, reason: string): void;
  on(event: "message", listener: (data: unknown, isBinary: boolean) => void): unknown;
  on(event: "close" | "error", listener: () => void): unknown;
};
type RuntimeWebSocketServer = { handleUpgrade(req: unknown, socket: unknown, head: unknown, accepted: (socket: RuntimeWebSocket) => void): void };
type RuntimeWebSocketModule = { readonly WebSocketServer: new (options: { readonly noServer: true; readonly maxPayload: number; readonly handleProtocols: (protocols: Set<string>) => string | false }) => RuntimeWebSocketServer };

const RUNTIME_WEBSOCKET_MODULE = "ws";
const devStreamMuxServers = new WeakMap<object, StreamMuxServerV1>();

/** 🔀️ The ONE stream channel (`semio.io.stream-mux/v1` at `STREAM_MUX_PATH`) of a dev server, created on first use per
 * HTTP server: it accepts same-origin WebSocket upgrades (subprotocol `semio.stream-mux.v1`) through the runtime's own `ws` —
 * Bun's `node:http` upgrade socket does not transmit raw writes (measured, ticket 26/09/23 F2) — and every dev plugin that
 * serves a long-lived stream registers its route here instead of holding an HTTP/1.1 response open. Without an HTTP server
 * (a middleware-only harness) it answers a private server nothing can connect to. */
export function devStreamMuxServer(httpServer: DevUpgradeServer | null | undefined): StreamMuxServerV1 {
  if (httpServer === null || httpServer === undefined) return new StreamMuxServerV1();
  const known = devStreamMuxServers.get(httpServer);
  if (known !== undefined) return known;
  const mux = new StreamMuxServerV1();
  devStreamMuxServers.set(httpServer, mux);
  const acceptor = (import(RUNTIME_WEBSOCKET_MODULE) as Promise<RuntimeWebSocketModule>).then(
    ({ WebSocketServer }) =>
      new WebSocketServer({
        noServer: true,
        maxPayload: STREAM_MUX_BOUNDS_V1.maxFrameBytes,
        handleProtocols: (protocols) => (protocols.has(STREAM_MUX_BOUNDS_V1.subprotocol) ? STREAM_MUX_BOUNDS_V1.subprotocol : false),
      }),
    (error: unknown) => {
      console.error("[stream-mux] the runtime WebSocket server is unavailable", error);
      return null;
    },
  );
  httpServer.on("upgrade", (req, socket, head) => {
    let path: string;
    try {
      path = new URL(req.url ?? "", "http://127.0.0.1").pathname;
    } catch {
      return;
    }
    if (path !== STREAM_MUX_PATH) return;
    const origin = req.headers.origin;
    const host = req.headers.host;
    if (Array.isArray(origin) || Array.isArray(host) || (origin !== undefined && origin !== `http://${host}` && origin !== `https://${host}`)) {
      socket.destroy();
      return;
    }
    void acceptor.then((acceptorServer) => {
      if (acceptorServer === null) return socket.destroy();
      acceptorServer.handleUpgrade(req, socket, head, (webSocket) => {
        const connection = mux.connect({ send: (frame) => webSocket.send(frame), bufferedAmount: () => webSocket.bufferedAmount ?? 0, close: (code, reason) => webSocket.close(code, reason) });
        webSocket.on("message", (data, isBinary) => {
          if (isBinary) webSocket.close(1003, "stream-mux: text frames only");
          else connection.receive(String(data));
        });
        webSocket.on("close", () => connection.closed());
        webSocket.on("error", () => connection.closed());
      });
    });
  });
  httpServer.once("close", () => mux.close());
  return mux;
}
//#endregion 🔀️StreamMuxServer
//#endregion BackboneVitePlugin
