#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.gif@87a/✳️any`.
//
// 🚫️ NOT `gif` 0.13, and that is the whole point. That crate cannot write a GIF87a at all: its encoder
// hardcodes `b"GIF89a"` (`src/encoder.rs:340`) and emits a Graphic Control Extension — an 89a-only
// block — for EVERY frame (`src/encoder.rs:178`, unconditional, despite its own doc comment saying
// "if necessary"). The generator this replaces patched the signature onto that output, which produced a
// file declaring `GIF87a` while carrying a GCE: a document that contradicts itself.
//
// Pillow writes conformant single-image GIF87a natively — correct signature, no GCE, background index
// settable — verified before this was built. Multi-image 87a is then ASSEMBLED from Pillow's own image
// blocks: the container header from one file, the image blocks from several, the trailer. Nothing is
// hand-encoded; the LZW data is Pillow's. The assembly asserts the source headers are byte-identical
// first, because differing global palettes would silently re-attribute every frame's colours.
//
// The JUDGE is `gif` 0.13, which reads 87a completely — see `../🔬️probes/🦀️reader`.
//
//   bun 📜️script.ts generate [--out <dir>]   # writes every fixture pair
//   bun 📜️script.ts manifests                 # prints the fixtureManifests entries
//
// @see ../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️gif-87a-conformance-and-writer-limits.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const HERE = import.meta.dir;
const FIXTURES_DIR = join(HERE, "..", "🧫️fixtures");
const ORACLE_ID = "gif-87a-any-mutate-reader";
const COMPARISON_PROFILE = "semantic-gif-87a-reader-v1";
/** 🧾️ The eleven kinds Pillow can express. Only `set-pixel-aspect-ratio` stays `-uncarried`: no
 *  surveyed writer emits that byte, in either GIF version.
 *
 *  🔍️`set-image-interlace` was ONCE recorded here as unwritable, on a measurement taken with a 4x3 and
 *  an 8x8 image. That was an artifact of the test, not of Pillow: `GifImagePlugin.get_interlace`
 *  defaults to 1 and only forces 0 when `min(im.size) < 16` (its "@PIL153 workaround"). At 16x16 and
 *  above Pillow writes the descriptor's interlace bit by default and clears it on `interlace=False`,
 *  both under a GIF87a signature — so the interlace pair uses a 16x16 canvas where every other kind
 *  uses 4x3. */
const KINDS: readonly string[] = ["no-mutation", "set-snapshot", "set-screen-size", "set-global-color-table", "set-background-color-index", "insert-image", "remove-image", "move-image", "set-image-geometry", "set-image-pixels", "set-image-interlace"];
const FIXTURE_DIRECTORIES: Readonly<Record<string, string>> = {
  "insert-image": "➕️insert-image",
  "move-image": "🔀️move-image",
  "no-mutation": "⏸️no-mutation",
  "remove-image": "➖️remove-image",
  "set-background-color-index": "🖌️set-background-color-index",
  "set-global-color-table": "🎨️set-global-color-table",
  "set-image-geometry": "📐️set-image-geometry",
  "set-image-interlace": "🪜️set-image-interlace",
  "set-image-pixels": "🎞️set-image-pixels",
  "set-pixel-aspect-ratio": "⚖️set-pixel-aspect-ratio",
  "set-screen-size": "🖥️set-screen-size",
  "set-snapshot": "📸️set-snapshot",
};
const BEFORE_FILE = "⬅️before.gif";
const AFTER_FILE = "➡️after.gif";
/** 🪞️ `no-mutation` declares the `no-op` outcome, so its pair must be IDENTICAL. Every other kind must
 *  move the projection, and the generator refuses to write one that does not. */
const NO_OP: ReadonlySet<string> = new Set(["no-mutation"]);
//#endregion 🧬️Contract

//#region 🐍️Writer
/** 🐍️ Pillow is the WRITER. Passed to `python3 -c` rather than living in its own file, because this
 *  node is allowed exactly one script. */
const WRITER = String.raw`
import io, os, sys
from PIL import Image

PALETTE_A = [255,0,0, 0,255,0, 0,0,255, 255,255,0]
PALETTE_B = [10,20,30, 200,180,160, 0,128,255, 255,0,128]

def single(w, h, data, palette=PALETTE_A, bg=0, interlace=None):
    im = Image.new('P', (w, h))
    im.putpalette(palette + [0] * (256 * 3 - len(palette)))
    im.putdata(data)
    im.info['background'] = bg
    buf = io.BytesIO()
    if interlace is None:
        im.save(buf, format='GIF')
    else:
        im.save(buf, format='GIF', interlace=interlace)
    return buf.getvalue()

def split(b):
    """Fixed-grammar walk over GIF blocks. Decodes nothing; only locates block boundaries."""
    assert b[:6] == b'GIF87a', b[:6]
    packed = b[10]
    i = 13
    if packed & 0x80:
        i += 3 * (2 ** ((packed & 0x07) + 1))
    header, blocks = b[:i], []
    while i < len(b):
        t = b[i]
        if t == 0x3B:
            break
        if t == 0x2C:
            start = i
            i += 10
            lp = b[start + 9]
            if lp & 0x80:
                i += 3 * (2 ** ((lp & 0x07) + 1))
            i += 1
            while b[i] != 0:
                i += 1 + b[i]
            i += 1
            blocks.append(b[start:i])
        elif t == 0x21:
            raise SystemExit('refusing to assemble from a file containing an extension block')
        else:
            raise SystemExit('unexpected block 0x%02x' % t)
    return header, blocks

def assemble(sources):
    """Container header from the first source, image blocks from all of them, then the trailer."""
    headers, blocks = [], []
    for s in sources:
        h, b = split(s)
        headers.append(h)
        blocks.extend(b)
    if len(set(headers)) != 1:
        raise SystemExit('source headers differ: assembling would re-attribute frame colours')
    return headers[0] + b''.join(blocks) + b'\x3B'

FA = [0,1,2,3, 0,1,2,3, 0,1,2,3]
FB = [3,2,1,0, 3,2,1,0, 3,2,1,0]
FC = [1,1,2,2, 3,3,0,0, 2,2,1,1]

def base():
    return assemble([single(4, 3, FA), single(4, 3, FB)])

def pair(kind):
    if kind == 'no-mutation':
        return base(), base()
    if kind == 'set-snapshot':
        return base(), assemble([single(2, 2, [0,1,2,3])])
    if kind == 'set-screen-size':
        return single(4, 3, FA), single(6, 2, [0,1,2,3,0,1, 2,3,0,1,2,3])
    if kind == 'set-global-color-table':
        return single(4, 3, FA), single(4, 3, FA, palette=PALETTE_B)
    if kind == 'set-background-color-index':
        return single(4, 3, FA, bg=0), single(4, 3, FA, bg=2)
    if kind == 'insert-image':
        return base(), assemble([single(4, 3, FA), single(4, 3, FC), single(4, 3, FB)])
    if kind == 'remove-image':
        return assemble([single(4, 3, FA), single(4, 3, FB), single(4, 3, FC)]), base()
    if kind == 'move-image':
        return base(), assemble([single(4, 3, FB), single(4, 3, FA)])
    if kind == 'set-image-geometry':
        return single(4, 3, FA), single(3, 4, [0,1,2, 3,0,1, 2,3,0, 1,2,3])
    if kind == 'set-image-pixels':
        return single(4, 3, FA), single(4, 3, FC)
    if kind == 'set-image-interlace':
        # 📐️16x16 is the smallest canvas Pillow will interlace: below it, get_interlace() forces 0.
        big = [(x + y) % 4 for y in range(16) for x in range(16)]
        return single(16, 16, big, interlace=False), single(16, 16, big, interlace=True)
    raise SystemExit('unknown kind ' + kind)

out_root, kind, directory_name, before_name, after_name = sys.argv[1:6]
before, after = pair(kind)
d = os.path.join(out_root, directory_name)
os.makedirs(d, exist_ok=True)
open(os.path.join(d, before_name), 'wb').write(before)
open(os.path.join(d, after_name), 'wb').write(after)
print('%s: before=%dB after=%dB' % (kind, len(before), len(after)))
`;
//#endregion 🐍️Writer

//#region 🔨️Build
const READER = join(HERE, "..", "🔬️probes", "🦀️reader");

function readerBinary(): string {
  const result = spawnSync("cargo", ["build", "--release", "--offline", "--manifest-path", join(READER, "Cargo.toml")], { stdio: "inherit" });
  if (result.status !== 0) throw new Error(`cargo build failed with status ${result.status}`);
  return join(READER, "target", "release", "reader");
}

function projectionOf(binary: string, path: string): string {
  const result = spawnSync(binary, ["project", path], { encoding: "utf8" });
  if (result.status !== 0) throw new Error(`reader refused ${path}: ${result.stdout}${result.stderr}`);
  return JSON.stringify(JSON.parse(result.stdout).measurements);
}

async function sha256(path: string): Promise<string> {
  const digest = await crypto.subtle.digest("SHA-256", readFileSync(path));
  return `sha256:${[...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("")}`;
}
//#endregion 🔨️Build

//#region 🚪️Commands
function generate(outRoot: string): number {
  const binary = readerBinary();
  const failures: string[] = [];
  for (const kind of KINDS) {
    const directory = FIXTURE_DIRECTORIES[kind]!;
    const written = spawnSync("python3", ["-c", WRITER, outRoot, kind, directory, BEFORE_FILE, AFTER_FILE], { stdio: "inherit" });
    if (written.status !== 0) {
      failures.push(`${kind}: writer failed`);
      continue;
    }
    // 🔍️Observability, checked through the READER before the pair is allowed to stand: a mutation whose
    // projection does not move is not evidence, and a `no-op` kind whose projection DOES move is a
    // different bug. Both are refused rather than committed.
    const before = projectionOf(binary, join(outRoot, directory, BEFORE_FILE));
    const after = projectionOf(binary, join(outRoot, directory, AFTER_FILE));
    const moved = before !== after;
    if (NO_OP.has(kind) && moved) failures.push(`${kind}: declares no-op but the projection moved`);
    else if (!NO_OP.has(kind) && !moved) failures.push(`${kind}: not observable in the reader's projection`);
  }
  console.error(`[generator] ${KINDS.length - failures.length}/${KINDS.length} fixture pair(s) into ${outRoot}`);
  for (const failure of failures) console.error(`[generator] ${failure}`);
  return failures.length > 0 ? 1 : 0;
}

async function manifests(): Promise<void> {
  const entries = [];
  for (const kind of KINDS) {
    const directory = FIXTURE_DIRECTORIES[kind]!;
    const dir = join(FIXTURES_DIR, directory);
    if (!existsSync(dir)) throw new Error(`missing fixture directory for ${kind} — run generate first`);
    const files = [];
    for (const [role, name] of [["expected-before-gif", BEFORE_FILE], ["expected-after-gif", AFTER_FILE]] as const) {
      const path = join(dir, name);
      files.push({ role, path: `../🧫️fixtures/${directory}/${name}`, mediaType: "image/gif", sha256: await sha256(path), bytes: readFileSync(path).length });
    }
    entries.push({
      schema: "semio.repository-test.fixture/v2",
      id: `${kind}-${NO_OP.has(kind) ? "no-op" : "applied"}`,
      class: "third-party-generated",
      target: { artifact: "s.stdio.gif", standard: "87a", subset: "any" },
      mutation: kind,
      outcome: NO_OP.has(kind) ? "no-op" : "applied",
      units: { length: "unitless", angle: "degree" },
      files,
      provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
      generator: { oracle: ORACLE_ID, packageVersion: "11.3.0", engineFamily: "pillow", engineVersion: "11.3.0", command: "bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🏭️generator/📜️script.ts generate", platform: process.platform },
      comparisonProfile: COMPARISON_PROFILE,
      reproducible: true,
      family: "mechanical",
      notes: `A conformant GIF87a written by Pillow 11.3.0 — correct signature, no Graphic Control Extension — with the ${kind} mutation expressed as the difference between the pair. Multi-image cases are assembled from Pillow's own image blocks, with the source headers asserted byte-identical first so no frame's colours are re-attributed. The judge is gif 0.13, a different third-party implementation, which reports the declared VERSION as part of its projection.`,
    });
  }
  console.log(JSON.stringify(entries, null, 2));
}
//#endregion 🚪️Commands

async function aspectMode(command: string): Promise<number> {
  const ASPECT_OUT = process.env.SEMIO_FIXTURE_OUT ?? join(import.meta.dir, "..", "🧫️fixtures");
  // 🖥️ASPECT MODE — the artifact's last kind, closed by a third-party CLI.
  //
  // `gif` 0.13.3 (`encoder.rs:345`) and 0.14.2 (`encoder.rs:401`) each write a hardcoded `0u8` for the
  // aspect byte and NEITHER has a parse path for it; Pillow surfaces only `background` and `version`.
  // Every LIBRARY was checked and the negative was right — but the inventory behind it was scoped to
  // libraries and never to installed CLIs, and Protocol v2 lists `third-party-cli` as qualifying.
  //
  // giflib does both halves: `gifbuild -d` dumps a text description carrying `pixel aspect byte N`,
  // `gifbuild` writes a GIF back from it (byte-deterministic across runs, checked), and `giftext`
  // reports `Aspect = N`. The only authored step is one line of that text description — fixture
  // authoring, which the goal statement admits, with giflib doing every byte of the encoding.
  if (command === "aspect" || command === "aspect-manifests") {
    const seedDir = join(import.meta.dir, "..", "🧫️fixtures");
    const seedKind = readdirSync(seedDir).find((name) => existsSync(join(seedDir, name, BEFORE_FILE)));
    if (!seedKind) throw new Error(`no committed ${BEFORE_FILE} to seed the aspect pair from`);
    const seed = join(seedDir, seedKind, BEFORE_FILE);
    const kind = "set-pixel-aspect-ratio";
    const directory = FIXTURE_DIRECTORIES[kind]!;
    const dir = join(ASPECT_OUT, directory);
    const probes = join(import.meta.dir, "..", "..", "..", "..", "9️⃣89a", "🪆️subsets", "🧱️base", "🔬️probes", "📜️script.ts");
    if (command === "aspect") {
      mkdirSync(dir, { recursive: true });
      const dump = spawnSync("gifbuild", ["-d", seed], { encoding: "utf8" });
      if (dump.status !== 0) throw new Error(`gifbuild -d failed: ${dump.stderr}`);
      const before = dump.stdout;
      if (!before.includes("pixel aspect byte 0")) throw new Error("the seed does not carry a zero aspect byte to change");
      const after = before.replace("pixel aspect byte 0", "pixel aspect byte 49");
      for (const [name, text] of [[BEFORE_FILE, before], [AFTER_FILE, after]] as const) {
        const built = spawnSync("gifbuild", [], { input: text, maxBuffer: 64 * 1024 * 1024 });
        if (built.status !== 0) throw new Error(`gifbuild failed for ${name}`);
        writeFileSync(join(dir, name), built.stdout);
      }
      const cmp = spawnSync("bun", [probes, "gif-screen-compare", "--input", join(dir, BEFORE_FILE), "--input", join(dir, AFTER_FILE)], { encoding: "utf8" });
      if (cmp.status !== 0) throw new Error(`reader refused the pair: ${cmp.stdout}${cmp.stderr}`);
      const m = JSON.parse(cmp.stdout).measurements;
      if (m.equal === true) { console.error(`[generator] ${kind}: not observable in the screen projection`); return 1; }
      // 🔍️The pair must move the ASPECT and nothing else in the descriptor.
      for (const field of ["screenWidth", "screenHeight", "colorResolution", "bitsPerPixel", "background", "imageCount"]) {
        if (m.expected[field] !== m.actual[field]) { console.error(`[generator] ${kind}: the pair also changed ${field}`); return 1; }
      }
      console.error(`[generator] ${kind}: observable, aspect ${m.expected.aspect} -> ${m.actual.aspect}`);
      return 0;
    }
    const files = [];
    for (const [role, name] of [["expected-before-gif", BEFORE_FILE], ["expected-after-gif", AFTER_FILE]] as const) {
      const bytes = readFileSync(join(dir, name));
      const digest = await crypto.subtle.digest("SHA-256", bytes);
      files.push({ role, path: `../🧫️fixtures/${directory}/${name}`, mediaType: "image/gif", sha256: `sha256:${[...new Uint8Array(digest)].map((b) => b.toString(16).padStart(2, "0")).join("")}`, bytes: bytes.length });
    }
    process.stdout.write(`${JSON.stringify([{
      schema: "semio.repository-test.fixture/v2",
      id: `aspect-${kind}`,
      class: "third-party-generated",
      target: { artifact: "s.stdio.gif", standard: "87a", subset: "any" },
      mutation: kind,
      outcome: "applied",
      units: { length: "unitless", angle: "degree" },
      files,
      provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
      generator: { oracle: "giflib-gif-screen-cli", packageVersion: "6.1", engineFamily: "giflib", engineVersion: "6.1", command: "bun 📜️script.ts aspect", platform: process.platform },
      comparisonProfile: "semantic-gif-screen-v1",
      reproducible: true,
      family: "mechanical",
      notes: "This subset's own committed seed, dumped by gifbuild -d and rebuilt twice by gifbuild — once with the pixel aspect byte at 0 and once at 49. Every other descriptor field is asserted identical at generation time. No library reader reaches this byte: both vendored gif crate versions write a hardcoded zero and neither parses it, and Pillow surfaces only background and version. giftext reports Aspect = N.",
    }], null, 2)}\n`);
    return 0;
  }
  return 2;
}

//#region 🚀️Entry
const [command, ...rest] = process.argv.slice(2);
const outFlagIndex = rest.indexOf("--out");
const outRoot = outFlagIndex >= 0 ? rest[outFlagIndex + 1]! : (process.env.SEMIO_FIXTURE_OUT ?? FIXTURES_DIR);
if (command === "aspect" || command === "aspect-manifests") process.exit(await aspectMode(command));
else if (command === "generate") process.exit(generate(outRoot));
else if (command === "manifests") await manifests();
else {
  console.error("usage: bun 📜️script.ts <generate [--out <dir>]|manifests|aspect|aspect-manifests>");
  process.exit(2);
}
//#endregion 🚀️Entry
