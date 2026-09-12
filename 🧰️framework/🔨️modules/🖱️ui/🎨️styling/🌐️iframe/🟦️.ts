type HeaderResponse = Readonly<{
  setHeader(name: string, value: string): void;
}>;

type HeaderMiddleware = (request: unknown, response: HeaderResponse, next: () => void) => void;

type HeaderServer = Readonly<{
  middlewares: Readonly<{
    use(middleware: HeaderMiddleware): void;
  }>;
}>;

/** 🖼️ Permits local preview surfaces to embed playground frames. */
export function playgroundIframeEmbedHeadersPlugin() {
  const useHeaders: HeaderMiddleware = (_request, response, next) => {
    response.setHeader("Content-Security-Policy", "frame-ancestors *");
    response.setHeader("Cross-Origin-Opener-Policy", "same-origin");
    response.setHeader("Cross-Origin-Embedder-Policy", "credentialless");
    next();
  };
  return {
    name: "playground-iframe-embed-headers",
    configureServer(server: HeaderServer) {
      server.middlewares.use(useHeaders);
    },
    configurePreviewServer(server: HeaderServer) {
      server.middlewares.use(useHeaders);
    },
  };
}
