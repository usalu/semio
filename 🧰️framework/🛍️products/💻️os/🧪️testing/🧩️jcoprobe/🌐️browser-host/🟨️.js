/** 🧪️ terra-jco-spike host shim — implements `semio:jcoprobe/probe-host` for the jco-transpiled
 * `jcoprobe` component. `slowEcho` uses a REAL `setTimeout` delay (not a synchronous resolve) so S2
 * can prove the JS event loop keeps ticking while the guest awaits it. `fetchBody` hands back an
 * async generator of five single-byte chunks so S4 can prove chunk-by-chunk delivery. */
const CHUNKS = [0x10, 0x20, 0x30, 0x40, 0x50];

export async function slowEcho(ms, v) {
  const t0 = performance.now();
  console.log(`[host-shim] slowEcho(${ms}, ${v}) START t=${t0.toFixed(2)}`);
  await new Promise((resolve) => setTimeout(resolve, ms));
  const t1 = performance.now();
  console.log(`[host-shim] slowEcho(${ms}, ${v}) DONE  t=${t1.toFixed(2)} (elapsed=${(t1 - t0).toFixed(2)}ms)`);
  return v;
}

export async function fetchBody() {
  console.log(`[host-shim] fetchBody() called, handing back ${CHUNKS.length}-chunk async generator`);
  async function* gen() {
    for (const byte of CHUNKS) {
      console.log(`[host-shim] fetchBody stream yielding chunk 0x${byte.toString(16)}`);
      yield byte;
    }
  }
  return gen();
}
