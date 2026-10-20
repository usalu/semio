import { createServer, type IncomingMessage, type Server, type ServerResponse } from "node:http";
import { isDeepStrictEqual } from "node:util";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 📨️ Carries one open owner declaration; its provider admits all provider-specific fields. */
export interface AssetDeliveryDeclarationV1 {
  readonly kind: string;
  readonly route: string;
  readonly [key: string]: unknown;
}

/** 🎚️ Selects fetching or prepared-cache delivery. */
export type AssetDeliveryModeV1 = "fetch" | "bundle";

/** 📍️ Supplies the delivery root and mode explicitly at the composition boundary. */
export interface AssetDeliveryContextV1 {
  readonly root: string;
  readonly mode: AssetDeliveryModeV1;
}

/** ⏭️ Hands control to the next middleware of the delivery chain. */
export type AssetBuildNextV1 = (error?: unknown) => void;

/** 🌉️ A delivery middleware exactly as the dispatcher chains it. */
export type AssetBuildMiddlewareV1 = (request: IncomingMessage, response: ServerResponse, next: AssetBuildNextV1) => void | Promise<void>;

/** 🏷️ The only property of a build plugin the dispatcher reads: the builder that supplies providers owns the rest of the plugin shape. */
export interface AssetBuildPluginV1 {
  readonly name: string;
}

/** 🔌️ Contributes an admitted asset kind without extending the general dispatcher. */
export interface AssetDeliveryProviderV1<Plugin extends AssetBuildPluginV1 = AssetBuildPluginV1> {
  readonly kind: string;
  readonly middleware: (context: AssetDeliveryContextV1, declaration: AssetDeliveryDeclarationV1) => AssetBuildMiddlewareV1;
  readonly plugins: (context: AssetDeliveryContextV1, declaration: AssetDeliveryDeclarationV1) => readonly Plugin[];
}

/** 🛂️ Admits the open wire envelope using the language-neutral schema's exact bounds. */
export function parseAssetDeliveryDeclarationV1(value: unknown): AssetDeliveryDeclarationV1 {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw Error("Asset declaration must be a record");
  const record = value as Record<string, unknown>;
  for (const [key, rule] of Object.entries(schema.properties)) {
    const field = record[key];
    if (typeof field !== "string" || Array.from(field).length < rule.minLength || Array.from(field).length > rule.maxLength || !new RegExp(rule.pattern, "u").test(field)) throw Error("Invalid asset declaration " + key);
  }
  return record as AssetDeliveryDeclarationV1;
}

/** 🧭️ Resolves only the two authored delivery modes. */
export function resolveAssetDeliveryModeV1(value?: string): AssetDeliveryModeV1 {
  if (value === undefined || value === "" || value === "fetch") return "fetch";
  if (value === "bundle") return "bundle";
  throw Error("Asset delivery mode must be fetch or bundle");
}

function selectDeliveries<Plugin extends AssetBuildPluginV1>(declarations: readonly AssetDeliveryDeclarationV1[], providers: readonly AssetDeliveryProviderV1<Plugin>[]): readonly (readonly [AssetDeliveryProviderV1<Plugin>, AssetDeliveryDeclarationV1])[] {
  const registry = new Map<string, AssetDeliveryProviderV1<Plugin>>();
  for (const provider of providers) {
    parseAssetDeliveryDeclarationV1({ kind: provider.kind, route: "/" });
    if (registry.has(provider.kind)) throw Error("Duplicate asset provider " + provider.kind);
    if (typeof provider.middleware !== "function" || typeof provider.plugins !== "function") throw Error("Incomplete asset provider " + provider.kind);
    registry.set(provider.kind, provider);
  }
  const seen = new Map<string, AssetDeliveryDeclarationV1>();
  const deliveries: (readonly [AssetDeliveryProviderV1<Plugin>, AssetDeliveryDeclarationV1])[] = [];
  for (const value of declarations) {
    const declaration = parseAssetDeliveryDeclarationV1(value);
    const provider = registry.get(declaration.kind);
    if (!provider) throw Error("Missing asset provider " + declaration.kind);
    const key = declaration.kind + ":" + declaration.route;
    const prior = seen.get(key);
    if (prior) {
      if (!isDeepStrictEqual(prior, declaration)) throw Error("Conflicting asset declarations " + key);
      continue;
    }
    seen.set(key, declaration);
    deliveries.push([provider, declaration]);
  }
  return deliveries;
}

/** 🏗️ Composes only explicitly supplied build providers in declaration order. */
export function createAssetBuildPluginsV1<Plugin extends AssetBuildPluginV1>(root: string, declarations: readonly AssetDeliveryDeclarationV1[], providers: readonly AssetDeliveryProviderV1<Plugin>[], mode: AssetDeliveryModeV1 = "fetch"): Plugin[] {
  const context = { root, mode: resolveAssetDeliveryModeV1(mode) };
  return selectDeliveries(declarations, providers).flatMap(([provider, declaration]) => [...provider.plugins(context, declaration)]);
}

/** 🌐️ Starts a caller-owned HTTP pipeline after every declaration and provider is admitted. */
export function createAssetHttpServerV1(root: string, port: number, declarations: readonly AssetDeliveryDeclarationV1[], providers: readonly AssetDeliveryProviderV1[], mode: AssetDeliveryModeV1 = "fetch", host = "127.0.0.1"): Server {
  const context = { root, mode: resolveAssetDeliveryModeV1(mode) };
  const middlewares = selectDeliveries(declarations, providers).map(([provider, declaration]) => provider.middleware(context, declaration));
  const server = createServer((request, response) => {
    const run = (index: number): void => {
      const middleware = middlewares[index];
      if (middleware) middleware(request, response, () => run(index + 1));
      else { response.statusCode = 404; response.end(); }
    };
    run(0);
  });
  server.listen(port, host);
  return server;
}
