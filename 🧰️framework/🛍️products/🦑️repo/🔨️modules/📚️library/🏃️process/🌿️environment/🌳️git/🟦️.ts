import { existsSync, readFileSync } from "node:fs";
import { devNull, homedir } from "node:os";
import { join } from "node:path";

/** 🧯️ Copies the process environment and replaces unreadable global Git configuration. */
export function safeGitEnv(extraEnv?: Record<string, string>): Record<string, string> {
  const env: Record<string, string> = {};
  for (const [key, value] of Object.entries(process.env)) {
    if (value !== undefined) env[key] = value;
  }
  Object.assign(env, extraEnv);
  const configuredGlobal = env.GIT_CONFIG_GLOBAL?.trim();
  const globalConfig = configuredGlobal || join(homedir(), ".gitconfig");
  try {
    if (configuredGlobal && !existsSync(globalConfig)) throw new Error("missing configured global Git config");
    if (existsSync(globalConfig)) readFileSync(globalConfig, "utf8");
  } catch {
    env.GIT_CONFIG_GLOBAL = devNull;
  }
  return env;
}

/** 🌳️Git subprocess env with explicit `cwd` — ignores inherited `GIT_DIR` / `GIT_WORK_TREE`. */
export function gitSpawnEnv(): Record<string, string> {
  const env = safeGitEnv();
  delete env.GIT_DIR;
  delete env.GIT_WORK_TREE;
  return env;
}

