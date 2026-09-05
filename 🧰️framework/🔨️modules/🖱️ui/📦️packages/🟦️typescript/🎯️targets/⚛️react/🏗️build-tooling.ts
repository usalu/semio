//#region 🔌️Adapters
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vitest/config";
import type { IncomingMessage, ServerResponse } from "node:http";
//#endregion 🔌️Adapters

//#region 🔖️OwnedBuildContract
export type OwnedBuildNext = (error?: unknown) => void;
export type OwnedBuildMiddleware = (request: IncomingMessage, response: ServerResponse, next: OwnedBuildNext) => void | Promise<void>;

export type OwnedBuildServer = {
  readonly middlewares: { use(middleware: OwnedBuildMiddleware): void };
  readonly ws: { send(payload: Readonly<Record<string, unknown>>): void };
};

export type OwnedResolvedBuildConfig = {
  readonly root: string;
  readonly build: { readonly outDir: string; readonly write?: boolean };
};

export type OwnedBuildPlugin = {
  readonly name: string;
  readonly enforce?: "pre" | "post";
  readonly apply?: "build" | "serve";
  resolveId?(id: string, importer?: string): string | undefined | null | Promise<string | undefined | null>;
  load?(id: string): string | undefined | null | Promise<string | undefined | null>;
  configureServer?(server: OwnedBuildServer): void;
  configurePreviewServer?(server: OwnedBuildServer): void;
  configResolved?(config: OwnedResolvedBuildConfig): void;
  closeBundle?(): void | Promise<void>;
  transformIndexHtml?: unknown;
};

export type OwnedBuildOptions = {
  readonly target?: string;
  readonly outDir?: string;
  readonly emptyOutDir?: boolean;
  readonly sourcemap?: boolean;
  readonly minify?: string | boolean;
  readonly cssMinify?: boolean | string;
  readonly reportCompressedSize?: boolean;
  readonly esbuild?: {
    readonly drop?: readonly string[];
    readonly legalComments?: string;
    readonly [key: string]: unknown;
  };
  readonly [key: string]: unknown;
};

export type OwnedBuildConfig = {
  readonly root?: string;
  readonly base?: string;
  readonly publicDir?: string;
  readonly assetsInclude?: readonly string[];
  readonly plugins?: readonly (OwnedBuildPlugin | readonly OwnedBuildPlugin[])[];
  readonly worker?: Readonly<Record<string, unknown>>;
  readonly define?: Readonly<Record<string, unknown>>;
  readonly build?: OwnedBuildOptions;
  readonly server?: {
    readonly fs?: { readonly allow?: readonly string[] };
    readonly watch?: Readonly<Record<string, unknown>>;
    readonly [key: string]: unknown;
  };
  readonly resolve?: {
    readonly alias?: readonly { readonly find: string | RegExp; readonly replacement: string }[];
    readonly dedupe?: readonly string[];
    readonly [key: string]: unknown;
  };
  readonly optimizeDeps?: {
    readonly include?: readonly string[];
    readonly exclude?: readonly string[];
    readonly [key: string]: unknown;
  };
  readonly test?: Readonly<Record<string, unknown>>;
  readonly [key: string]: unknown;
};

export type OwnedTestProjectConfig = OwnedBuildConfig & { root?: string };
//#endregion 🔖️OwnedBuildContract

//#region 🏭️Factories
/** @emoji 🎨️ Tailwind's temporary build adapter behind the UI package that declares it. */
export function uiTailwindBuildPlugins(): OwnedBuildPlugin[] {
  return tailwindcss() as unknown as OwnedBuildPlugin[];
}

/** @emoji ⚛️ React's temporary build adapter behind the UI package that declares it. */
export function uiReactBuildPlugin(): OwnedBuildPlugin {
  return react() as unknown as OwnedBuildPlugin;
}

/** @emoji 🧪️ Defines a test/build config without exporting Vitest or Vite types. */
export function defineOwnedTestConfig<T extends OwnedBuildConfig>(config: T): T {
  return defineConfig(config as never) as T;
}

/** @emoji 🏗️ Identity helper for owned build configs that need no implementation runtime. */
export function defineOwnedBuildConfig<T extends OwnedBuildConfig>(config: T): T {
  return config;
}
//#endregion 🏭️Factories
