import { handleIncomingRequest } from "./http.js";
/** A fetch-like browser client for invoking an in-memory WASI HTTP handler. */
export class InMemoryHttpClient {
    #handler;
    constructor(handler) {
        this.#handler = typeof handler === "function" ? handler : handler.handle.bind(handler);
    }
    fetch(request) {
        return handleIncomingRequest(request, this.#handler);
    }
}
