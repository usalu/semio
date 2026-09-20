/** 🌿️ Ambient declaration for the ONE Vite virtual module this product's entry points import.
 *
 * It lives in its own directory on purpose. TypeScript's `include` globbing keeps only the
 * highest-priority extension per path (`.ts` > `.tsx` > `.d.ts`), so this file used to sit beside
 * `♻️activation/🟦️.ts` as `♻️activation/🟦️.d.ts` and was silently dropped from every program —
 * which is why `🧑‍💻dev/🟦️.ts` reported `TS2307: Cannot find module 'virtual:semio-playground-session'`
 * while the declaration was right there. The runtime module is emitted by
 * `semioPlaygroundSessionVitePlugin` under {@link PLAYGROUND_SESSION_VITE_SPECIFIER}. */
declare module "virtual:semio-playground-session" {
  export const PLAYGROUND_SESSION: import("@semio-tech/framework").PlaygroundBootSession;
}
