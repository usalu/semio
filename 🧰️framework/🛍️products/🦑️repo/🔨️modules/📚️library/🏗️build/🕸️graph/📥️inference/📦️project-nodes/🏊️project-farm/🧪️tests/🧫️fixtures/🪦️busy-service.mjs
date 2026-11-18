/** 🪦️ A farm service whose session holds the event loop of the worker for two minutes with CPU bound work, the way the inference of a big project does. */
export async function open() { const until = Date.now() + 120000; while (Date.now() < until) Math.sqrt(Math.random()); return {}; }
export async function run() { return {}; }
export async function close() {}
