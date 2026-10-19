import { posix, win32 } from "node:path";

/** 📚️ Prepends an admitted compiler directory using the native platform's loader environment. */
export function rustCompilerLoaderEnvironment(platform: "win32" | "linux" | "darwin", environment: Readonly<Record<string, string | undefined>>, directory: string): Record<string, string | undefined> {
  const windows = platform === "win32", delimiter = windows ? ";" : ":";
  if (!(windows ? win32 : posix).isAbsolute(directory) || /[\r\n\0]/u.test(directory) || directory.includes(delimiter)) throw Error("Compiler runtime library directory must be one absolute loader path");
  const result = { ...environment };
  const key = windows ? "PATH" : platform === "darwin" ? "DYLD_FALLBACK_LIBRARY_PATH" : "LD_LIBRARY_PATH";
  const keys = Object.keys(environment).filter(name => windows ? name.toLowerCase() === "path" : name === key).sort();
  const prior = keys.length ? environment[keys[0]!] : undefined;
  for (const name of keys) delete result[name];
  result[key] = directory + (prior ? delimiter + prior : "");
  return result;
}
