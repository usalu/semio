import { refuseProcessV1 } from "../../📖️texts/🟦️.ts";

/** 🧩️ Evaluates one transpiled production function against explicit ports; every other free name it reaches resolves to the global scope or to a pass-through stub (it calls the last function argument it receives), so an unrelated new dependency of the function cannot break the law that the test states. */
export function evaluateExtractedFunctionV1(code: string, name: string, ports: Record<string, unknown>): any {
  ports = { __importMeta: { url: "file:///extracted-function/", dir: "/extracted-function", dirname: "/extracted-function" }, ...ports };
  const pass = (...values: unknown[]): unknown => { const body = [...values].reverse().find(value => typeof value === "function") as (() => unknown) | undefined; return body ? body() : undefined; };
  const scope = new Proxy(ports, {
    has: (_target, key) => typeof key === "string",
    get: (target, key) => { if (typeof key !== "string") return undefined; if (key in target) return target[key]; if (key in globalThis) return (globalThis as Record<string, unknown>)[key]; return pass; },
    set: (target, key, value) => { target[key as string] = value; return true; },
  });
  const found = new Function("scope", "with(scope){" + code.replaceAll("import.meta", "__importMeta") + ";return " + name + ";}")(scope);
  if (found === pass) throw refuseProcessV1("missing", "testing.extracted-function-required", { name });
  return found;
}
