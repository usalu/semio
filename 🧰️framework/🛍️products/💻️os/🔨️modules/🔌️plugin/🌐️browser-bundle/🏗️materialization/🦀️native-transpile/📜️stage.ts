#!/usr/bin/env repo
import { execFileSync } from "node:child_process";
import { chmodSync, copyFileSync, mkdirSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { nativeTranspileBinaryNameV1 } from "./🟦️.ts";

/** 🏗️ Builds the native transpile through the repository cargo gate and stages the executable beside the other browser tools. */
const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(here, "..", "..", "..", "..", "..", "..", "..", "..");
const manifest = join(here, "Cargo.toml");
const output = execFileSync("repo", ["cargo", "--store=shared", "--configuration=release", "--", "build", "--manifest-path", manifest, "--message-format=json"], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 28, stdio: ["ignore", "pipe", "inherit"] });
const name = nativeTranspileBinaryNameV1();
let executable: string | undefined;
for (const line of output.split("\n")) {
  if (!line.startsWith("{")) continue;
  const message = JSON.parse(line) as { reason?: string; executable?: string | null; target?: { name?: string } };
  if (message.reason === "compiler-artifact" && message.target?.name === "semio-native-transpile" && message.executable) executable = message.executable;
}
if (!executable) throw new Error("cargo built no semio-native-transpile executable");
const destination = join(repoRoot, ".cache", "b", "p", "m", "browser", "release", "tools", "native-transpile", process.platform, name);
mkdirSync(dirname(destination), { recursive: true });
copyFileSync(executable, destination);
chmodSync(destination, 0o755);
console.log(destination);
