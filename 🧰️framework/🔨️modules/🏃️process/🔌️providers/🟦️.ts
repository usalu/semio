import { createRequire } from "node:module";
import { isAbsolute } from "node:path";
import { pathToFileURL } from "node:url";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { validateJsonSchemaSubset } from "../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 🔌️ Environment variable in which the issuing parent designates provider modules; its name is fixed by the schema. */
export const SCRIPT_PROVIDERS_ENV: string = schema.definitions.Environment.const;
const PROVIDER_EXPORT: string = schema.definitions.Export.const;

/** 📨️ Designates the absolute module implementing each named framework-owned port. */
export type ScriptProviderMapV1 = Readonly<{ version: 1; providers: Readonly<Record<string, string>> }>;
declare const brand: unique symbol;
/** 🏷️ Names one framework-owned port and the exact functions its implementation must expose. */
export type ScriptProviderTokenV1<Provider extends object> = Readonly<{ name: string; methods: readonly (keyof Provider & string)[]; readonly [brand]?: Provider }>;

const loading = new Map<string, Promise<object>>();
const loaded = new Map<string, object>();
const requireModule = createRequire(import.meta.url);

function admitProvider<Provider extends object>(token: ScriptProviderTokenV1<Provider>, module: Readonly<Record<string, unknown>>): Provider {
  const provider = module[PROVIDER_EXPORT];
  if (provider === null || typeof provider !== "object") throw Error(`Script provider ${token.name} must export ${PROVIDER_EXPORT}`);
  for (const method of token.methods) if (typeof (provider as Readonly<Record<string, unknown>>)[method] !== "function") throw Error(`Script provider ${token.name} lacks ${method}`);
  return provider as Provider;
}

/** 🧩️ Declares a port; the implementation stays outside the framework and is designated at run time. */
export function defineScriptProviderV1<Provider extends object>(name: string, methods: readonly (keyof Provider & string)[]): ScriptProviderTokenV1<Provider> {
  if (validateJsonSchemaSubset(schema, { version: 1, providers: { [name]: "/" } }).length || !methods.length) throw Error("Closed script provider port declaration required");
  return Object.freeze({ name, methods: Object.freeze([...methods]) });
}

/** 🔐️ Admits the designation an issuing parent wrote into a child environment. */
export function readScriptProviderMapV1(environment: Readonly<Record<string, string | undefined>>): ScriptProviderMapV1 {
  const wire = environment[SCRIPT_PROVIDERS_ENV];
  if (wire === undefined) throw Error(`Script provider designation required in ${SCRIPT_PROVIDERS_ENV}`);
  let value: unknown;
  try { value = JSON.parse(wire); } catch { throw Error("Complete script provider designation required"); }
  if (validateJsonSchemaSubset(schema, value).length) throw Error("Complete script provider designation required");
  const map = value as ScriptProviderMapV1;
  if (Object.values(map.providers).some(path => !isAbsolute(path))) throw Error("Absolute script provider module paths required");
  return map;
}

/** 📤️ Authors the child environment value that designates provider modules. */
export function writeScriptProviderMapV1(providers: Readonly<Record<string, string>>): string {
  const map: ScriptProviderMapV1 = { version: 1, providers };
  if (validateJsonSchemaSubset(schema, map).length || Object.values(providers).some(path => !isAbsolute(path))) throw Error("Complete script provider designation required");
  return JSON.stringify(map);
}

/** 📥️ Loads and checks the designated implementation of one port exactly once per module. */
export async function loadScriptProviderV1<Provider extends object>(token: ScriptProviderTokenV1<Provider>, environment: Readonly<Record<string, string | undefined>> = process.env): Promise<Provider> {
  const path = readScriptProviderMapV1(environment).providers[token.name];
  if (path === undefined) throw Error(`Script provider ${token.name} is not designated`);
  const key = token.name + "\0" + path, ready = loaded.get(key);
  if (ready) return ready as Provider;
  let pending = loading.get(key);
  if (!pending) {
    pending = import(pathToFileURL(path).href).then((module: Readonly<Record<string, unknown>>) => {
      const provider = admitProvider(token, module);
      loaded.set(key, provider);
      return provider;
    });
    loading.set(key, pending);
    pending.catch(() => { loading.delete(key); });
  }
  return await pending as Provider;
}

/** ⚡️ Loads the same designated implementation synchronously for ports whose functions are synchronous; the module must not use top-level await. */
export function loadScriptProviderSyncV1<Provider extends object>(token: ScriptProviderTokenV1<Provider>, environment: Readonly<Record<string, string | undefined>> = process.env): Provider {
  const path = readScriptProviderMapV1(environment).providers[token.name];
  if (path === undefined) throw Error(`Script provider ${token.name} is not designated`);
  const key = token.name + "\0" + path, ready = loaded.get(key);
  if (ready) return ready as Provider;
  const provider = admitProvider(token, requireModule(path) as Readonly<Record<string, unknown>>);
  loaded.set(key, provider);
  return provider;
}
