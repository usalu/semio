import { inputStreamCreate, ioErrorCreate, outputStreamCreate, pollableCreate } from "./io.js";
export { InMemoryHttpClient } from "./in-memory-http.js";
const symbolDispose = Symbol.dispose || Symbol.for("dispose");
const utf8Encoder = new TextEncoder();
const utf8Decoder = new TextDecoder();
const forbiddenHeaders = new Set(["connection", "keep-alive", "host"]);
const DEFAULT_HTTP_TIMEOUT_NS = 600000000000n;
// RFC 9110 compliant header validation
const TOKEN_RE = /^[!#$%&'*+\-.^_`|~0-9A-Za-z]+$/;
const FIELD_VALUE_RE = /^[\t\x20-\x7E\x80-\xFF]*$/;
const BRACKETED_IPV6_AUTHORITY_RE = /^\[([0-9A-Fa-f:.]+)\](?::([0-9]+))?$/;
const DNS_OR_IPV4_AUTHORITY_RE = /^([a-zA-Z0-9.-]+)(?::([0-9]+))?$/;
function validateHeaderName(name) {
    if (!TOKEN_RE.test(name)) {
        throw { tag: "invalid-syntax" };
    }
}
function validateHeaderValue(value) {
    const str = typeof value === "string" ? value : utf8Decoder.decode(value);
    if (!FIELD_VALUE_RE.test(str)) {
        throw { tag: "invalid-syntax" };
    }
}
class Fields {
    #immutable = false;
    #entries = [];
    #table = new Map();
    static fromList(entries) {
        const fields = new Fields();
        for (const [key, value] of entries) {
            fields.append(key, value);
        }
        return fields;
    }
    get(name) {
        const tableEntries = this.#table.get(name.toLowerCase());
        if (!tableEntries) {
            return [];
        }
        return tableEntries.map(([, v]) => v);
    }
    /**
     * WIT spec (https://github.com/WebAssembly/WASI/blob/91bb44c3c3a9b1e09187db23c85fa844d1fd6b15/proposals/http/wit/types.wit#L215-L223):
     *
     * > Set all of the values for a name. Clears any existing values for that
     * > name, if they have been set.
     * >
     * > Fails with `header-error.immutable` if the `fields` are immutable.
     * >
     * > Fails with `header-error.invalid-syntax` if the `field-name` or any of
     * > the `field-value`s are syntactically invalid.
     *
     * The existing-branch splice/reuse is an allocation optimization — values are cleared, not retained.
     */
    set(name, values) {
        if (this.#immutable) {
            throw { tag: "immutable" };
        }
        validateHeaderName(name);
        for (const value of values) {
            validateHeaderValue(value);
        }
        const lowercased = name.toLowerCase();
        if (forbiddenHeaders.has(lowercased)) {
            throw { tag: "forbidden" };
        }
        const tableEntries = this.#table.get(lowercased);
        if (tableEntries) {
            this.#entries = this.#entries.filter((entry) => !tableEntries.includes(entry));
            tableEntries.splice(0, tableEntries.length);
        }
        else {
            this.#table.set(lowercased, []);
        }
        const newTableEntries = this.#table.get(lowercased);
        for (const value of values) {
            const entry = [name, value];
            this.#entries.push(entry);
            newTableEntries.push(entry);
        }
    }
    has(name) {
        return this.#table.has(name.toLowerCase());
    }
    delete(name) {
        if (this.#immutable) {
            throw { tag: "immutable" };
        }
        const lowercased = name.toLowerCase();
        const tableEntries = this.#table.get(lowercased);
        if (tableEntries) {
            this.#entries = this.#entries.filter((entry) => !tableEntries.includes(entry));
            this.#table.delete(lowercased);
        }
    }
    append(name, value) {
        if (this.#immutable) {
            throw { tag: "immutable" };
        }
        validateHeaderName(name);
        validateHeaderValue(value);
        const lowercased = name.toLowerCase();
        if (forbiddenHeaders.has(lowercased)) {
            throw { tag: "forbidden" };
        }
        const entry = [name, value];
        this.#entries.push(entry);
        const tableEntries = this.#table.get(lowercased);
        if (tableEntries) {
            tableEntries.push(entry);
        }
        else {
            this.#table.set(lowercased, [entry]);
        }
    }
    entries() {
        return this.#entries;
    }
    clone() {
        return fieldsFromEntriesChecked(this.#entries);
    }
    static _lock(fields) {
        fields.#immutable = true;
        return fields;
    }
    static _fromEntriesChecked(entries) {
        const fields = new Fields();
        fields.#entries = entries;
        for (const entry of entries) {
            const lowercase = entry[0].toLowerCase();
            const existing = fields.#table.get(lowercase);
            if (existing) {
                existing.push(entry);
            }
            else {
                fields.#table.set(lowercase, [entry]);
            }
        }
        return fields;
    }
}
const fieldsLock = Fields._lock;
// @ts-expect-error - Deleting static method
delete Fields._lock;
const fieldsFromEntriesChecked = Fields._fromEntriesChecked;
// @ts-expect-error - Deleting static method
delete Fields._fromEntriesChecked;
class RequestOptions {
    #connectTimeout;
    #firstByteTimeout;
    #betweenBytesTimeout;
    connectTimeout() {
        return this.#connectTimeout;
    }
    setConnectTimeout(duration) {
        if (duration !== undefined && duration < 0n) {
            throw new Error("duration must not be negative");
        }
        this.#connectTimeout = duration;
    }
    firstByteTimeout() {
        return this.#firstByteTimeout;
    }
    setFirstByteTimeout(duration) {
        if (duration !== undefined && duration < 0n) {
            throw new Error("duration must not be negative");
        }
        this.#firstByteTimeout = duration;
    }
    betweenBytesTimeout() {
        return this.#betweenBytesTimeout;
    }
    setBetweenBytesTimeout(duration) {
        if (duration !== undefined && duration < 0n) {
            throw new Error("duration must not be negative");
        }
        this.#betweenBytesTimeout = duration;
    }
}
class OutgoingBody {
    #outputStream = null;
    #chunks = [];
    #finished = false;
    #resolveFinished;
    #finishedPromise = new Promise((resolve) => (this.#resolveFinished = resolve));
    #requestStream = null;
    #requestStreamController = null;
    #requestStreamCancelled = false;
    write() {
        const outputStream = this.#outputStream;
        if (outputStream === null) {
            throw undefined;
        }
        this.#outputStream = null;
        return outputStream;
    }
    static finish(body, trailers) {
        if (trailers) {
            throw { tag: "internal-error", val: "trailers unsupported" };
        }
        if (body.#finished) {
            throw { tag: "internal-error", val: "body already finished" };
        }
        body.#finished = true;
        if (!body.#requestStreamCancelled) {
            body.#requestStreamController?.close();
        }
        body.#resolveFinished();
    }
    #bodyData() {
        if (this.#chunks.length === 0) {
            return null;
        }
        let totalLen = 0;
        for (const chunk of this.#chunks) {
            totalLen += chunk.byteLength;
        }
        const result = new Uint8Array(totalLen);
        let offset = 0;
        for (const chunk of this.#chunks) {
            result.set(chunk, offset);
            offset += chunk.byteLength;
        }
        return result;
    }
    static async _finishedBodyData(outgoingBody) {
        await outgoingBody.#finishedPromise;
        return outgoingBody.#bodyData();
    }
    static _requestBodyStream(outgoingBody) {
        if (outgoingBody.#requestStream === null) {
            outgoingBody.#requestStream = new ReadableStream({
                start(controller) {
                    outgoingBody.#requestStreamController = controller;
                    for (const chunk of outgoingBody.#chunks) {
                        controller.enqueue(chunk);
                    }
                    outgoingBody.#chunks.length = 0;
                    if (outgoingBody.#finished) {
                        controller.close();
                    }
                },
                cancel() {
                    outgoingBody.#requestStreamCancelled = true;
                },
            });
        }
        return outgoingBody.#requestStream;
    }
    static _create() {
        const outgoingBody = new OutgoingBody();
        const chunks = outgoingBody.#chunks;
        outgoingBody.#outputStream = outputStreamCreate({
            write(buf) {
                if (outgoingBody.#finished || outgoingBody.#requestStreamCancelled) {
                    throw { tag: "closed" };
                }
                const chunk = new Uint8Array(buf);
                if (outgoingBody.#requestStreamController) {
                    outgoingBody.#requestStreamController.enqueue(chunk);
                }
                else {
                    chunks.push(chunk);
                }
            },
            blockingFlush() { },
            subscribe() {
                return pollableCreate();
            },
        });
        return outgoingBody;
    }
    [symbolDispose]() { }
}
const outgoingBodyCreate = OutgoingBody._create;
// @ts-expect-error - Deleting static method
delete OutgoingBody._create;
const outgoingBodyFinishedData = OutgoingBody._finishedBodyData;
// @ts-expect-error - Deleting static method
delete OutgoingBody._finishedBodyData;
const outgoingBodyRequestStream = OutgoingBody._requestBodyStream;
// @ts-expect-error - Deleting static method
delete OutgoingBody._requestBodyStream;
class OutgoingRequest {
    #method = { tag: "get" };
    #scheme = undefined;
    #pathWithQuery = undefined;
    #authority = undefined;
    #headers;
    #body;
    #bodyRequested = false;
    constructor(headers) {
        fieldsLock(headers);
        this.#headers = headers;
        this.#body = outgoingBodyCreate();
    }
    body() {
        if (this.#bodyRequested) {
            throw new Error("Body already requested");
        }
        this.#bodyRequested = true;
        return this.#body;
    }
    method() {
        return this.#method;
    }
    setMethod(method) {
        if (method.tag === "other" && method.val && !method.val.match(/^[a-zA-Z-]+$/)) {
            throw undefined;
        }
        this.#method = method;
    }
    pathWithQuery() {
        return this.#pathWithQuery;
    }
    setPathWithQuery(pathWithQuery) {
        if (pathWithQuery && !pathWithQuery.match(/^[a-zA-Z0-9.\-_~!$&'()*+,;=:@%?/]+$/)) {
            throw undefined;
        }
        this.#pathWithQuery = pathWithQuery;
    }
    scheme() {
        return this.#scheme;
    }
    setScheme(scheme) {
        if (scheme?.tag === "other" && scheme.val && !scheme.val.match(/^[a-zA-Z]+$/)) {
            throw undefined;
        }
        this.#scheme = scheme;
    }
    authority() {
        return this.#authority;
    }
    setAuthority(authority) {
        if (authority !== undefined) {
            const match = authority.startsWith("[")
                ? authority.match(BRACKETED_IPV6_AUTHORITY_RE)
                : authority.match(DNS_OR_IPV4_AUTHORITY_RE);
            if (!match || (match[2] !== undefined && Number(match[2]) > 65535)) {
                throw undefined;
            }
            try {
                const parsed = new URL(`http://${authority}/`);
                if (parsed.username || parsed.password || !parsed.hostname) {
                    throw undefined;
                }
            }
            catch {
                throw undefined;
            }
        }
        this.#authority = authority;
    }
    headers() {
        return this.#headers;
    }
    [symbolDispose]() { }
    static _handle(request, options, config = {}) {
        const scheme = schemeString(request.#scheme);
        const method = "val" in request.#method ? request.#method.val : request.#method.tag;
        if (!request.#pathWithQuery) {
            throw { tag: "HTTP-request-URI-invalid" };
        }
        const url = `${scheme}//${request.#authority || ""}${request.#pathWithQuery}`;
        const headers = new Headers();
        for (const [key, value] of request.#headers.entries()) {
            const lowerKey = key.toLowerCase();
            if (!forbiddenHeaders.has(lowerKey)) {
                headers.append(key, utf8Decoder.decode(value));
            }
        }
        // Request streams are opt-in because Firefox and Safari do not yet support them.
        // The portable default buffers until the guest explicitly finishes the body.
        const bodyData = request.#bodyRequested
            ? config.streamingRequestBodies
                ? outgoingBodyRequestStream(request.#body)
                : outgoingBodyFinishedData(request.#body)
            : null;
        let timeoutMs = Number(DEFAULT_HTTP_TIMEOUT_NS / 1000000n);
        if (options) {
            const ct = options.connectTimeout?.() ?? DEFAULT_HTTP_TIMEOUT_NS;
            const fbt = options.firstByteTimeout?.() ?? DEFAULT_HTTP_TIMEOUT_NS;
            const minTimeout = ct < fbt ? ct : fbt;
            timeoutMs = Number(minTimeout / 1000000n);
        }
        return futureIncomingResponseCreate(url, method.toUpperCase(), headers, bodyData, timeoutMs);
    }
}
const outgoingRequestHandle = OutgoingRequest._handle;
// @ts-expect-error - Deleting static method
delete OutgoingRequest._handle;
class IncomingBody {
    #finished = false;
    #stream = null;
    stream() {
        if (!this.#stream) {
            throw undefined;
        }
        const stream = this.#stream;
        this.#stream = null;
        return stream;
    }
    static finish(incomingBody) {
        if (incomingBody.#finished) {
            throw new Error("incoming body already finished");
        }
        incomingBody.#finished = true;
        return futureTrailersCreate();
    }
    [symbolDispose]() { }
    static _create(fetchResponse, bufferedBody) {
        const incomingBody = new IncomingBody();
        let buffer = bufferedBody ?? null;
        let bufferOffset = 0;
        let done = bufferedBody !== undefined;
        let reader = null;
        let readPromise = null;
        let readError = null;
        let disposed = false;
        function ready() {
            return done || (buffer !== null && bufferOffset < buffer.byteLength);
        }
        function startRead() {
            if (readPromise || ready()) {
                return;
            }
            if (!fetchResponse.body) {
                done = true;
                return;
            }
            reader ??= fetchResponse.body.getReader();
            const activeReader = reader;
            readPromise = (async () => {
                try {
                    // Empty Fetch chunks do not make a WASI input stream readable.
                    // Keep a single read in flight and buffer at most one nonempty chunk.
                    while (!done) {
                        const result = await activeReader.read();
                        if (disposed) {
                            return;
                        }
                        if (result.done) {
                            done = true;
                        }
                        else if (result.value.byteLength > 0) {
                            buffer = result.value;
                            bufferOffset = 0;
                            return;
                        }
                    }
                }
                catch (cause) {
                    done = true;
                    if (!disposed) {
                        readError = ioErrorCreate(cause instanceof Error ? cause.message : String(cause));
                    }
                }
                finally {
                    readPromise = null;
                    if (done) {
                        activeReader.releaseLock();
                    }
                }
            })();
        }
        async function waitForReadable() {
            while (!ready()) {
                startRead();
                await readPromise;
            }
        }
        function read(len) {
            if (readError) {
                const error = readError;
                readError = null;
                throw { tag: "last-operation-failed", val: error };
            }
            if (done && (buffer === null || bufferOffset >= buffer.byteLength)) {
                throw { tag: "closed" };
            }
            if (buffer === null || bufferOffset >= buffer.byteLength) {
                startRead();
                return new Uint8Array(0);
            }
            const toRead = Math.min(Number(len), buffer.byteLength - bufferOffset);
            const slice = buffer.slice(bufferOffset, bufferOffset + toRead);
            bufferOffset += toRead;
            if (bufferOffset >= buffer.byteLength) {
                buffer = null;
                bufferOffset = 0;
                startRead();
            }
            return slice;
        }
        function blockingRead(len) {
            if (len === 0n || ready()) {
                return read(len);
            }
            return waitForReadable().then(() => read(len));
        }
        function blockingSkip(len) {
            const result = blockingRead(len);
            return result instanceof Promise
                ? result.then((bytes) => BigInt(bytes.byteLength))
                : BigInt(result.byteLength);
        }
        incomingBody.#stream = inputStreamCreate({
            read,
            // WIT declares synchronous signatures; JSPI awaits these host results
            // for blocking imports. Preserve synchronous results for buffered bodies.
            blockingRead: blockingRead,
            blockingSkip: blockingSkip,
            subscribe() {
                return pollableCreate({ ready, wait: waitForReadable });
            },
            drop() {
                disposed = true;
                done = true;
                buffer = null;
                readError = null;
                if (reader) {
                    const activeReader = reader;
                    void activeReader
                        .cancel()
                        .catch(() => { })
                        .finally(() => {
                        activeReader.releaseLock();
                    });
                }
            },
        });
        if (bufferedBody === undefined) {
            startRead();
        }
        return incomingBody;
    }
}
const incomingBodyCreate = IncomingBody._create;
// @ts-expect-error - Deleting static method
delete IncomingBody._create;
class IncomingResponse {
    #headers;
    #status = 0;
    #body;
    status() {
        return this.#status;
    }
    headers() {
        return this.#headers;
    }
    consume() {
        if (this.#body === undefined) {
            throw undefined;
        }
        const body = this.#body;
        this.#body = undefined;
        return body;
    }
    [symbolDispose]() { }
    static _create(fetchResponse) {
        const res = new IncomingResponse();
        res.#status = fetchResponse.status;
        const headerEntries = [];
        const encoder = new TextEncoder();
        fetchResponse.headers.forEach((value, key) => {
            headerEntries.push([key, encoder.encode(value)]);
        });
        res.#headers = fieldsLock(fieldsFromEntriesChecked(headerEntries));
        res.#body = incomingBodyCreate(fetchResponse);
        return res;
    }
}
const incomingResponseCreate = IncomingResponse._create;
// @ts-expect-error - Deleting static method
delete IncomingResponse._create;
class IncomingRequest {
    #request;
    #headers;
    #body;
    method() {
        const method = this.#request.method.toLowerCase();
        return { tag: method };
    }
    pathWithQuery() {
        const url = new URL(this.#request.url);
        return `${url.pathname}${url.search}`;
    }
    scheme() {
        const protocol = new URL(this.#request.url).protocol;
        if (protocol === "http:") {
            return { tag: "HTTP" };
        }
        if (protocol === "https:") {
            return { tag: "HTTPS" };
        }
        return { tag: "other", val: protocol.slice(0, -1) };
    }
    authority() {
        return new URL(this.#request.url).host;
    }
    headers() {
        return this.#headers;
    }
    consume() {
        if (!this.#body) {
            throw new Error("incoming request body already consumed");
        }
        const body = this.#body;
        this.#body = undefined;
        return body;
    }
    static _create(request, bufferedBody) {
        const incoming = new IncomingRequest();
        incoming.#request = request.clone();
        const encoder = new TextEncoder();
        incoming.#headers = fieldsLock(fieldsFromEntriesChecked([...request.headers.entries()].map(([name, value]) => [
            name,
            encoder.encode(value),
        ])));
        incoming.#body = incomingBodyCreate(new Response(request.body), bufferedBody);
        return incoming;
    }
    static _toRequest(request) {
        return request.#request;
    }
}
const incomingRequestCreate = IncomingRequest._create;
// @ts-expect-error - Deleting static method
delete IncomingRequest._create;
const incomingRequestToRequest = IncomingRequest._toRequest;
// @ts-expect-error - Deleting static method
delete IncomingRequest._toRequest;
class OutgoingResponse {
    #headers;
    #status = 200;
    #body = outgoingBodyCreate();
    #bodyRequested = false;
    constructor(headers) {
        fieldsLock(headers);
        this.#headers = headers;
    }
    statusCode() {
        return this.#status;
    }
    setStatusCode(statusCode) {
        if (!Number.isInteger(statusCode) || statusCode < 100 || statusCode > 999) {
            throw new TypeError("invalid HTTP status code");
        }
        this.#status = statusCode;
    }
    headers() {
        return this.#headers;
    }
    body() {
        if (this.#bodyRequested) {
            throw new Error("outgoing response body already requested");
        }
        this.#bodyRequested = true;
        return this.#body;
    }
    static async _toResponse(response) {
        const headers = new Headers();
        for (const [name, value] of response.#headers.entries()) {
            headers.append(name, utf8Decoder.decode(value));
        }
        const body = response.#bodyRequested
            ? await outgoingBodyFinishedData(response.#body)
            : null;
        return new Response(body, {
            status: response.#status,
            headers,
        });
    }
}
const outgoingResponseToResponse = OutgoingResponse._toResponse;
// @ts-expect-error - Deleting static method
delete OutgoingResponse._toResponse;
class ResponseOutparam {
    #used = false;
    #resolve;
    #reject;
    static set(param, response) {
        if (param.#used) {
            throw new Error("response outparam already set");
        }
        param.#used = true;
        if (response.tag === "ok") {
            void outgoingResponseToResponse(response.val).then(param.#resolve, param.#reject);
        }
        else {
            param.#resolve(new Response(`WASI HTTP handler error: ${JSON.stringify(response.val)}`, {
                status: 500,
            }));
        }
    }
    static _isUsed(param) {
        return param.#used;
    }
    static _create() {
        const param = new ResponseOutparam();
        const response = new Promise((resolve, reject) => {
            param.#resolve = resolve;
            param.#reject = reject;
        });
        return [param, response];
    }
}
const responseOutparamCreate = ResponseOutparam._create;
// @ts-expect-error - Deleting static method
delete ResponseOutparam._create;
const responseOutparamIsUsed = ResponseOutparam._isUsed;
// @ts-expect-error - Deleting static method
delete ResponseOutparam._isUsed;
class FutureTrailers {
    #requested = false;
    subscribe() {
        return pollableCreate();
    }
    get() {
        if (this.#requested) {
            return { tag: "err", val: undefined };
        }
        this.#requested = true;
        return {
            tag: "ok",
            val: {
                tag: "ok",
                val: fieldsLock(fieldsFromEntriesChecked([])),
            },
        };
    }
    static _create() {
        return new FutureTrailers();
    }
}
const futureTrailersCreate = FutureTrailers._create;
// @ts-expect-error - Deleting static method
delete FutureTrailers._create;
function mapFetchError(err) {
    if (err.name === "AbortError") {
        return { tag: "connection-timeout" };
    }
    return { tag: "internal-error", val: err.message };
}
class FutureIncomingResponse {
    #result = undefined;
    #promise = null;
    #controller = null;
    #settled = false;
    subscribe() {
        return pollableCreate(this.#promise);
    }
    get() {
        if (this.#result === undefined) {
            return undefined;
        }
        const result = this.#result;
        this.#result = { tag: "err" };
        // The returned response now owns the body. Dropping this consumed future
        // must not abort a Fetch body that the guest is still streaming.
        if (result.tag === "ok" && result.val.tag === "ok") {
            this.#controller = null;
        }
        return result;
    }
    [symbolDispose]() {
        // Only abort if the request never settled. If fetch already
        // resolved (even with an ok response whose body is still being
        // streamed elsewhere), aborting here kills the underlying
        // connection and body stream out from under whoever is still
        // reading it, surfacing as a bogus "BodyStreamBuffer was aborted"
        // error on a request that actually succeeded.
        if (!this.#settled) {
            this.#controller?.abort();
        }
        this.#controller = null;
        this.#promise = null;
    }
    static _create(url, method, headers, bodyData, timeoutMs) {
        const future = new FutureIncomingResponse();
        const controller = new AbortController();
        future.#controller = controller;
        let timer;
        if (timeoutMs < Infinity) {
            timer = setTimeout(() => controller.abort(), timeoutMs);
        }
        future.#promise = Promise.resolve(bodyData)
            .then((bodyData) => {
            const init = {
                method,
                headers,
                signal: controller.signal,
            };
            if (bodyData && method !== "GET" && method !== "HEAD") {
                init.body = bodyData;
                if (bodyData instanceof ReadableStream) {
                    init.duplex = "half";
                }
            }
            return globalThis.fetch(url, init);
        })
            .then((response) => {
            if (timer) {
                clearTimeout(timer);
            }
            future.#settled = true;
            future.#result = {
                tag: "ok",
                val: {
                    tag: "ok",
                    val: incomingResponseCreate(response),
                },
            };
        }, (err) => {
            if (timer) {
                clearTimeout(timer);
            }
            future.#settled = true;
            future.#result = {
                tag: "ok",
                val: {
                    tag: "err",
                    val: mapFetchError(err),
                },
            };
        });
        return future;
    }
}
const futureIncomingResponseCreate = FutureIncomingResponse._create;
// @ts-expect-error - Deleting static method
delete FutureIncomingResponse._create;
function schemeString(scheme) {
    if (!scheme) {
        return "https:";
    }
    switch (scheme.tag) {
        case "HTTP":
            return "http:";
        case "HTTPS":
            return "https:";
        case "other":
            return scheme.val.toLowerCase() + ":";
    }
    return "https:";
}
function httpErrorCode(err) {
    if ("payload" in err) {
        return err.payload;
    }
    return {
        tag: "internal-error",
        val: "message" in err ? err.message : err.toDebugString(),
    };
}
let requestStreamingEnabled = false;
/** Enable or disable Fetch `ReadableStream` request bodies. Disabled by default. */
export function _setRequestStreaming(enabled) {
    if (typeof enabled !== "boolean") {
        throw new TypeError("request streaming setting must be a boolean");
    }
    requestStreamingEnabled = enabled;
}
export const outgoingHandler = {
    handle(request, options) {
        return outgoingRequestHandle(request, options, {
            streamingRequestBodies: requestStreamingEnabled,
        });
    },
};
export const incomingHandler = {
    handle() {
        throw "not-supported";
    },
};
/** Create a `wasi:http/incoming-handler` namespace backed by a Web Request handler. */
export function createIncomingHandler(handler) {
    return {
        async handle(request, responseOut) {
            try {
                const response = await handler(incomingRequestToRequest(request));
                if (!(response instanceof Response)) {
                    throw new TypeError("incoming HTTP handler must return a Response");
                }
                ResponseOutparam.set(responseOut, {
                    tag: "ok",
                    val: await responseToOutgoingResponse(response),
                });
            }
            catch (error) {
                ResponseOutparam.set(responseOut, {
                    tag: "err",
                    val: {
                        tag: "internal-error",
                        val: error instanceof Error ? error.message : String(error),
                    },
                });
            }
        },
    };
}
async function responseToOutgoingResponse(response) {
    const headers = [];
    response.headers.forEach((value, name) => headers.push([name, utf8Encoder.encode(value)]));
    const outgoing = new OutgoingResponse(fieldsFromEntriesChecked(headers));
    outgoing.setStatusCode(response.status);
    if (response.body) {
        const body = outgoing.body();
        const stream = body.write();
        const reader = response.body.getReader();
        for (let result = await reader.read(); !result.done; result = await reader.read()) {
            let offset = 0;
            while (offset < result.value.byteLength) {
                const permit = stream.checkWrite();
                if (permit === 0n) {
                    await stream.subscribe().block();
                    continue;
                }
                const length = Math.min(Number(permit), result.value.byteLength - offset);
                stream.write(result.value.subarray(offset, offset + length));
                offset += length;
            }
        }
        OutgoingBody.finish(body, undefined);
    }
    return outgoing;
}
/** Translate a browser Request through a WASI handler with a synchronously readable body. */
export async function handleIncomingRequest(request, handler) {
    const [responseOut, response] = responseOutparamCreate();
    const body = request.body
        ? new Uint8Array(await request.clone().arrayBuffer())
        : new Uint8Array();
    await handler(incomingRequestCreate(request, body), responseOut);
    if (!responseOutparamIsUsed(responseOut)) {
        throw new Error("WASI HTTP handler returned without setting its response outparam");
    }
    return response;
}
export const types = {
    Fields,
    FutureIncomingResponse,
    FutureTrailers,
    IncomingBody,
    IncomingRequest,
    IncomingResponse,
    OutgoingBody,
    OutgoingRequest,
    OutgoingResponse,
    ResponseOutparam,
    RequestOptions,
    httpErrorCode,
};
