/** ⏳️ `@semio-tech/framework-async` package entry point — re-exports the module's TS surface at
 * `../../🟦️.ts` (the owned-schema mirror plus the documented `WebAsyncScope` seam). No behavior
 * lives in this package yet: the concrete `HostAsyncRuntime` implementation is a Rust-only sibling
 * crate (design ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, packet R2), and its future
 * web-host counterpart (`WebAsyncScope`) is a documented seam, not implemented anywhere yet.
 */
export * from "../../🟦️";
