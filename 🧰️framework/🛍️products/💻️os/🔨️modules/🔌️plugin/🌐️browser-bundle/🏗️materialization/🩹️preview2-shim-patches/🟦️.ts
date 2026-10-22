/** 🩹️ Preview2 shim patches of the plugin web materialization: the vendored shim copy, the guest log classification patch and the whole line release patch. */
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { classifyGuestLogLine } from "../../🌐️wasi/🟦️.ts";

export function ensurePreview2ShimVendorAt(preview2VendorDir: string, repoRoot: string): void {
  const distDir = join(repoRoot, "node_modules/@bytecodealliance/preview2-shim/dist/browser");
  const libDir = join(repoRoot, "node_modules/@bytecodealliance/preview2-shim/lib/browser");
  const sourceDir = existsSync(distDir) ? distDir : libDir;
  if (!existsSync(sourceDir)) throw new Error("missing @bytecodealliance/preview2-shim browser shims; run bun install");
  mkdirSync(preview2VendorDir, { recursive: true });
  for (const entry of readdirSync(sourceDir, { withFileTypes: true })) {
    if (!entry.isFile() || !entry.name.endsWith(".js")) continue;
    copyFileSync(join(sourceDir, entry.name), join(preview2VendorDir, entry.name));
  }
  patchPreview2ShimGuestLogClassification(join(preview2VendorDir, "cli.js"));
  patchPreview2ShimGuestLogLineRelease(join(preview2VendorDir, "cli.js"));
}

/** 🗣️ Preserves Preview2's streaming decoder and classifies complete guest debug lines. */
export function patchPreview2ShimGuestLogClassification(cliPath: string): void {
  const source = readFileSync(cliPath, "utf8");
  const original = "consoleStream((line) => console.error(line))";
  const classified = `consoleStream((line) => (${classifyGuestLogLine.toString()})("stderr", line) === "debug" ? console.debug(line) : console.error(line))`;
  const originalCount = source.split(original).length - 1, classifiedCount = source.split(classified).length - 1;
  if (originalCount === 0 && classifiedCount === 2) return;
  if (originalCount !== 2 || classifiedCount !== 0) throw new Error(`preview2 cli.js guest-log patch did not match: ${cliPath}`);
  writeFileSync(cliPath, source.replaceAll(original, classified));
}

/** 🗣️ Upstream's own release of a buffered guest line — `@bytecodealliance/preview2-shim@0.30.0`'s
 * `dist/browser/cli.js` `consoleStream`, where `flush()` publishes whatever is buffered, complete
 * line or not. Anchored verbatim so a shim upgrade fails closed instead of silently reinstating the
 * fragment storm below. */
const PREVIEW2_PARTIAL_LINE_RELEASE = `    const flush = () => {
        pending += decoder.decode();
        if (pending) {
            writeLine(pending);
        }
        pending = "";
    };
    return {
        write(contents) {
            pending += decoder.decode(contents, { stream: true });
            emitCompleteLines();
        },
        flush,
        blockingFlush() {
            this.flush?.();
        },
        // OutputStream marks itself closed before calling the handler's drop.
        drop: flush,
    };
`;

/** 🧵 One logical guest line is ONE host emit — the law `🌐️wasi/🟦️.ts`'s `createGuestLogLineSink`
 * already states for the semio-owned browser WASI profile, transplanted onto the vendored Preview2
 * stream. A flush releases COMPLETE lines only; an unterminated tail is released on drop, written
 * out directly rather than through `this.flush`, because `OutputStream[Symbol.dispose]` clears
 * `open` before it calls `drop` and `this.flush` then throws `{ tag: "closed" }`. */
const SEMIO_COMPLETE_LINE_RELEASE = `    const flush = () => {
        pending += decoder.decode();
        emitCompleteLines();
    };
    const drop = () => {
        pending += decoder.decode();
        emitCompleteLines();
        if (pending) {
            writeLine(pending);
            pending = "";
        }
    };
    return {
        write(contents) {
            pending += decoder.decode(contents, { stream: true });
            emitCompleteLines();
        },
        flush,
        blockingFlush() {
            this.flush?.();
        },
        drop,
    };
`;

/** 🧵 Releases guest log lines whole, so {@link patchPreview2ShimGuestLogClassification}'s severity
 * classifies a MESSAGE instead of a `core::fmt` fragment.
 *
 * 🐛️ Rust's stderr is unbuffered, so `core::fmt::write` turns one `eprintln!("[TRACE] … {a} … {b}")`
 * into one `blocking-write-and-flush` per literal piece and per argument, and `io.js`'s
 * `blockingWriteAndFlush` is `handler.write()` + `handler.blockingFlush()`. Under upstream's partial
 * release every fragment became its own console call: fragment 1 carried the `[TRACE] ` prefix and
 * reached `console.debug`, fragments 2..n did not and reached `console.error` — 16 bogus `[error]`
 * lines on every canvas play pane and 172 on puzzle3d, which failed the strict acceptance suite on
 * 44 of 60 panes (ticket 26/09/19, `📓️console-spam.md`). */
export function patchPreview2ShimGuestLogLineRelease(cliPath: string): void {
  const source = readFileSync(cliPath, "utf8");
  const occurrences = (needle: string) => source.split(needle).length - 1;
  const partialCount = occurrences(PREVIEW2_PARTIAL_LINE_RELEASE), wholeCount = occurrences(SEMIO_COMPLETE_LINE_RELEASE);
  if (partialCount === 0 && wholeCount === 1) return;
  if (partialCount !== 1 || wholeCount !== 0) throw new Error(`preview2 cli.js guest-log line release patch did not match: ${cliPath}`);
  writeFileSync(cliPath, source.replace(PREVIEW2_PARTIAL_LINE_RELEASE, SEMIO_COMPLETE_LINE_RELEASE));
}
