/** 🌐️ Bun's ambient `fetch` carries `preconnect`, so a bare handler is not a complete `typeof fetch`.
 * Suites that replace `globalThis.fetch` route their handler through [[stubFetch]], which keeps the
 * real `preconnect` instead of casting the difference away; the returned value is the same function
 * object the caller passed, so `vi.fn()` call assertions still hold.
 * @see https://bun.com/reference/globals/fetch */
type FetchStubHandler = (input: URL | RequestInfo, init?: RequestInit) => Promise<Response>;

/** 🩹️ Completes a fetch stub with the ambient `preconnect` so it satisfies `typeof fetch`. */
function stubFetch<H extends FetchStubHandler>(handler: H): typeof fetch {
  return Object.assign(handler, { preconnect: globalThis.fetch.preconnect });
}

export { type FetchStubHandler, stubFetch };
