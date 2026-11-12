#!/usr/bin/env repo
/** 🖊️ `@teaching/staffage-dwg-rs` router: `repo ./📜️script.ts <wasm | test [cargo test args]>`.
 * `wasm` builds the DWG writer for `wasm32-unknown-unknown` through the repository Cargo ingress (`repo cargo`, shared store, the configuration of `SEMIO_BUILD_MODE`: `release`, otherwise `prototype`; one lane keeps one configuration) and puts the module next to `🖊️dwg/🟦️.ts` as `staffage_dwg.wasm` (git-ignored build output, never checked in); `test` runs the crate tests, which read every written file back with independent readers.
 * @see https://doc.rust-lang.org/cargo/reference/external-tools.html#json-messages */
import { spawnSync } from "node:child_process";
import { copyFileSync, existsSync, readFileSync, utimesSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const configuration: "prototype" | "release" = process.env.SEMIO_BUILD_MODE === "release" ? "release" : "prototype";
const crate = import.meta.dirname;
const manifest = join(crate, "Cargo.toml");
const target = resolve(crate, "../..", "staffage_dwg.wasm");

/** 🏗️ Runs `repo cargo` and returns its exit status; with `capture` its standard output is returned too (cargo's JSON messages), otherwise it passes through like standard error (progress, diagnostics). */
function cargo(args: readonly string[], capture = false): { readonly status: number; readonly stdout: string } {
  const result = spawnSync("repo", ["cargo", "--store=shared", `--configuration=${configuration}`, "--", ...args], { encoding: "utf8", maxBuffer: 1 << 28, stdio: ["inherit", capture ? "pipe" : "inherit", "inherit"], shell: process.platform === "win32" });
  if (result.error) throw new Error(`the repo launcher could not be started (${result.error.message}); install it with \`repo run repo-mcp:install\`; repo-Launcher nicht startbar`);
  return { status: result.status ?? 1, stdout: result.stdout ?? "" };
}

/** 🕸️ Builds the WebAssembly module and installs it as `staffage_dwg.wasm`; returns the installed path. */
function wasm(): string {
  const { status, stdout } = cargo(["build", "--manifest-path", manifest, "--lib", "--target", "wasm32-unknown-unknown", "--message-format=json-render-diagnostics"], true);
  if (status !== 0) throw new Error(`cargo build of staffage-dwg failed with status ${status}; Cargo-Build von staffage-dwg fehlgeschlagen (Status ${status})`);
  const artifacts = stdout.split("\n").flatMap((line) => {
    if (!line.startsWith("{")) return [];
    const message = JSON.parse(line) as { reason?: string; target?: { name?: string }; filenames?: string[] };
    return message.reason === "compiler-artifact" && message.target?.name === "staffage_dwg" ? (message.filenames ?? []).filter((file) => file.endsWith(".wasm")) : [];
  });
  const built = artifacts.at(-1);
  if (built === undefined || !existsSync(built)) throw new Error(`cargo reported no staffage_dwg.wasm; Cargo meldet kein staffage_dwg.wasm`);
  if (!existsSync(target) || !readFileSync(target).equals(readFileSync(built))) copyFileSync(built, target);
  else utimesSync(target, new Date(), new Date());
  return target;
}

const [command, ...rest] = process.argv.slice(2);
if (command === "wasm" && rest.length === 0) {
  process.stdout.write(`${wasm()}\n`);
} else if (command === "test") {
  process.exitCode = cargo(["test", "--manifest-path", manifest, ...rest]).status;
} else {
  process.stderr.write(`usage: repo ./📜️script.ts <wasm | test [cargo test args]>; Aufruf: repo ./📜️script.ts <wasm | test [Cargo-Testargumente]> (${dirname(manifest)})\n`);
  process.exitCode = 2;
}
