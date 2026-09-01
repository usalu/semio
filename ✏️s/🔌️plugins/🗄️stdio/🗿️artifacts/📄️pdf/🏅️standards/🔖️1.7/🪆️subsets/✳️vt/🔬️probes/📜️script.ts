#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔐️ The ENCRYPTION reader for every `pdf@1.7` conformance subset. Shared deliberately: the question
// — does this document carry a standard security handler — is identical across `vt`, `a`, `e` and `x`,
// and the framework lets a probe's `command` point at any script (brep's manifold probe already points
// at step's).
//
// Why a second reader at all: `lopdf` 0.44, which judges every other kind in these subsets, DECRYPTS
// transparently on load with the empty user password and then reports `is_encrypted() == false` on a
// genuinely encrypted document — measured. It also cannot WRITE an encryption dictionary: its writer
// demands the encryption state a real decryption would have recorded, so a synthetic `/Encrypt` can be
// neither written nor read back. Both halves of the encryption kinds were therefore invisible to it.
//
// `pypdf` 6.14 both encrypts (`PdfWriter.encrypt`) and reports it (`PdfReader.is_encrypted`), and its
// output is byte-deterministic across runs — checked three times before this was relied on.
//
//   bun 📜️script.ts pdf-encryption-project --input <a.pdf>
//   bun 📜️script.ts pdf-encryption-compare --input <a.pdf> --input <b.pdf>

//#endregion 🧲️Header

//#region 🔌️Adapters
import { spawnSync } from "node:child_process";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const ENGINE = { family: "pypdf", implementation: "pypdf 6.14 standard security handler reader", version: "6.14.2" } as const;
const PROBE_VERSION = "pypdf@6.14.2";

const READER = String.raw`
import json, sys
from pypdf import PdfReader

def project(path):
    reader = PdfReader(path)
    encrypted = reader.is_encrypted
    out = {"encrypted": bool(encrypted), "pageCount": None, "encryption": None}
    if encrypted:
        # 🔓️An empty user password is what the generator encrypts with; decrypting is how the page
        # count stays comparable across the pair rather than becoming "unknown" on one side only.
        try:
            reader.decrypt("")
        except Exception:
            pass
        enc = getattr(reader, "encryption", None)
        if enc is not None:
            out["encryption"] = {"algorithm": str(getattr(enc, "algorithm", None))}
    try:
        out["pageCount"] = len(reader.pages)
    except Exception:
        out["pageCount"] = None
    return out

paths = sys.argv[1:]
if len(paths) == 1:
    print(json.dumps(project(paths[0]), sort_keys=True))
else:
    a, b = project(paths[0]), project(paths[1])
    print(json.dumps({"equal": a == b, "expected": a, "actual": b}, sort_keys=True))
`;
//#endregion 🧬️Contract

//#region 🔬️Probe
function run(paths: readonly string[]): Record<string, unknown> {
  const result = spawnSync("python3", ["-c", READER, ...paths], { encoding: "utf8" });
  if (result.status !== 0) throw new Error(`pypdf reader failed: ${result.stderr}`);
  return JSON.parse(result.stdout) as Record<string, unknown>;
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
  if (probe === "pdf-encryption-project") process.exit(emit(probe, "ok", run([inputs[0]!])));
  else if (probe === "pdf-encryption-compare") process.exit(emit(probe, "ok", run([inputs[0]!, inputs[1]!])));
  else {
    console.error("usage: bun 📜️script.ts <pdf-encryption-project|pdf-encryption-compare> --input <path> [--input <path>]");
    process.exit(2);
  }
} catch (error) {
  process.exit(emit(probe || "(none)", "failed", {}, String((error as Error).message ?? error)));
}
//#endregion 🚀️Entry
