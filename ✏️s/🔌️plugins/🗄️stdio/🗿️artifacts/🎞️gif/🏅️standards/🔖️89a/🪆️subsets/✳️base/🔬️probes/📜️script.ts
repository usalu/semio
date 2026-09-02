#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ The LOGICAL SCREEN DESCRIPTOR reader, shared by both GIF versions.
//
// `set-pixel-aspect-ratio` was the last kind in this artifact, and it was recorded uncarried after
// every available LIBRARY was checked — correctly, as far as that went: `gif` 0.13.3 (`encoder.rs:345`)
// and 0.14.2 (`encoder.rs:401`) each write a hardcoded `0u8` for the aspect byte and NEITHER has any
// parse path for it, and Pillow surfaces only `background` and `version` after a round trip.
//
// What that survey never covered is that its INVENTORY was scoped to libraries — cargo, npm and PyPI —
// and never to installed command-line tools, even though Protocol v2 lists `third-party-cli` as a
// qualifying oracle kind. giflib's `giftext` prints the descriptor including `Aspect = N`, and its
// `gifbuild` writes a GIF from a text description carrying `pixel aspect byte N`. Reader and writer
// both third-party, both from giflib, and `gifbuild` is byte-deterministic across runs.
//
//   bun 📜️script.ts gif-screen-project --input <a.gif>
//   bun 📜️script.ts gif-screen-compare --input <a.gif> --input <b.gif>

//#endregion 🧲️Header

//#region 🔌️Adapters
import { spawnSync } from "node:child_process";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const ENGINE = { family: "giflib", implementation: "giftext logical screen descriptor dump (giflib 6.1)", version: "6.1" } as const;
const PROBE_VERSION = "giflib@6.1";
//#endregion 🧬️Contract

//#region 🔬️Probe
/** 🖥️ The logical screen descriptor as giflib reports it.
 *
 *  `giftext` echoes the INPUT PATH in its output; that line is dropped, because a projection carrying
 *  the fixture's own filename would differ for reasons that have nothing to do with the bytes. */
function project(absPath: string): Record<string, unknown> {
  const result = spawnSync("giftext", [absPath], { encoding: "utf8" });
  if (result.status !== 0) throw new Error(`giftext refused ${absPath}: ${result.stderr}`);
  const lines = String(result.stdout)
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.length > 0 && !line.includes(absPath));
  const screen = lines.find((line) => line.startsWith("Screen Size")) ?? null;
  const descriptor = lines.find((line) => line.includes("Aspect =")) ?? null;
  const field = (source: string | null, name: string): number | null => {
    if (!source) return null;
    const match = source.match(new RegExp(`${name}\\s*=\\s*(\\d+)`));
    return match ? Number(match[1]) : null;
  };
  return {
    screenWidth: field(screen, "Width"),
    screenHeight: field(screen, "Height"),
    colorResolution: field(descriptor, "ColorResolution"),
    bitsPerPixel: field(descriptor, "BitsPerPixel"),
    background: field(descriptor, "BackGround"),
    aspect: field(descriptor, "Aspect"),
    imageCount: lines.filter((line) => line.startsWith("Image #")).length,
  };
}

function emit(probe: string, status: string, measurements: Record<string, unknown>, message?: string): number {
  const report = {
    schema: "semio.repository-test.probe-report/v2",
    probe,
    probeVersion: PROBE_VERSION,
    engine: ENGINE,
    status,
    durationMs: 0,
    measurements,
    ...(message ? { diagnostics: [{ severity: "error", message }] } : {}),
  };
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  return status === "failed" ? 1 : 0;
}
//#endregion 🔬️Probe

//#region 🚀️Entry
const [probe = "", ...rest] = process.argv.slice(2);
const inputs: string[] = [];
for (let index = 0; index < rest.length; index += 1) if (rest[index] === "--input") inputs.push(rest[index + 1]!);
try {
  if (probe === "gif-screen-project") process.exit(emit(probe, "ok", project(inputs[0]!)));
  else if (probe === "gif-screen-compare") {
    const expected = project(inputs[0]!);
    const actual = project(inputs[1]!);
    process.exit(emit(probe, "ok", { equal: JSON.stringify(expected) === JSON.stringify(actual), expected, actual }));
  } else {
    console.error("usage: bun 📜️script.ts <gif-screen-project|gif-screen-compare> --input <path> [--input <path>]");
    process.exit(2);
  }
} catch (error) {
  process.exit(emit(probe || "(none)", "failed", {}, String((error as Error).message ?? error)));
}
//#endregion 🚀️Entry
