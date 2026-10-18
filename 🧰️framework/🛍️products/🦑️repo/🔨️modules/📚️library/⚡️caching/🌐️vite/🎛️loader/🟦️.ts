/** 🎛️ How Vite loads a configuration module: `runner` evaluates it in Vite's module runner, which is closed again before an async factory resumes, so only `native` (the runtime's own `import`) admits a factory that awaits opaque dynamic imports, as the os dev builder's does.
 * @see https://vite.dev/config/#configloader */
export type ViteConfigLoaderV1 = "runner" | "native";

/** 🎛️ Admits one loader name before it reaches a Vite command line; absent means `runner`. */
export function readViteConfigLoaderV1(value: unknown): ViteConfigLoaderV1 {
  if (value === undefined) return "runner";
  if (value !== "runner" && value !== "native") throw Error("Vite config loader must be runner or native");
  return value;
}
