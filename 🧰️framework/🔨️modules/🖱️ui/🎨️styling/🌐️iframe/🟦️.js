"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.playgroundIframeEmbedHeadersPlugin = playgroundIframeEmbedHeadersPlugin;
/** 🖼️ Permits local preview surfaces to embed playground frames. */
function playgroundIframeEmbedHeadersPlugin() {
    var useHeaders = function (_request, response, next) {
        response.setHeader("Content-Security-Policy", "frame-ancestors *");
        response.setHeader("Cross-Origin-Opener-Policy", "same-origin");
        response.setHeader("Cross-Origin-Embedder-Policy", "credentialless");
        next();
    };
    return {
        name: "playground-iframe-embed-headers",
        configureServer: function (server) {
            server.middlewares.use(useHeaders);
        },
        configurePreviewServer: function (server) {
            server.middlewares.use(useHeaders);
        },
    };
}
