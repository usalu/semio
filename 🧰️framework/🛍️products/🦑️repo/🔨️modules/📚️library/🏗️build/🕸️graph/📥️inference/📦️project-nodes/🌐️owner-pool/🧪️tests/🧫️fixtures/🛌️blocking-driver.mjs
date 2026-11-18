/** 🛌️ A driver for the laws of children that must end: every claim holds the event loop of the child with CPU bound work for two minutes, the way a real owner does. */
export async function openOwnerPoolInferenceV1() {
  return { claim() { const until = Date.now() + 120000; while (Date.now() < until) Math.sqrt(Math.random()); return { state: "held" }; }, run() { return true; }, close() {} };
}
