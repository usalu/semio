const key = Symbol.for("semio.output.contract.v1"), emit = (): void => { console.log("[noise] console.log"); process.stdout.write("[noise] process.stdout.write\n"); };
const timer = setInterval(() => { if ((globalThis as Record<symbol, unknown>)[key] || !process.argv.includes("--json")) emit(); }, 2);
timer.unref();
process.once("beforeExit", emit);
