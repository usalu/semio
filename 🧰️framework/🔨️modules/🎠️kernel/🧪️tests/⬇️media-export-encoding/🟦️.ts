import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { mediaExportBytes, MediaExportEncodingError, MEDIA_EXPORT_BASE64_ENCODING, MEDIA_EXPORT_UTF8_ENCODING } from "../../🟦️.ts";
import { base64StandardDecode, base64StandardEncode } from "../../../🚪️io/🔤️base64/🟦️.ts";
import { wireEffectToFriendly, wireMediaExportEncoding } from "../../../🎭️actor/🖼️wire-turn/🟦️.ts";

/** ⬇️ TypeScript twin of `🧪️tests/⬇️media-export-encoding/🦀️.rs`, driven from the SAME fixture
 * (`🧫️fixtures/⬇️media-export-encoding/🔣️.json`): a `downloadMediaExport` envelope's
 * `(data, encoding)` pair becomes exactly the declared bytes, on both renderers.
 *
 * 🔍️ Independent where it counts: the oracle is not our codec. Every base64 row is cross-checked
 * against Node's own `Buffer` base64 decoder AND the platform `atob`, neither of which shares a line
 * with `base64StandardDecode` — and the strictness rows pin exactly where our codec is *stricter*
 * than `atob`, which is the reason we own one at all.
 *
 * 🧬️ It also drives the hop the defect actually lived on: a WIT `option<string>` reaches a renderer
 * decoder either as jco's bare value or as the actor boundary's `{tag:"some", val}` record, and the
 * React door read only the first — so EVERY binary export in the repo was saved as base64 text under
 * a binary file name (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️io-surface-2026-09-13.md` §7.3).
 */

interface EncodingCase {
  readonly id: string;
  readonly filename: string;
  readonly mimeType: string;
  readonly encoding: string | null;
  readonly data: string;
  readonly bytes: readonly number[];
}

interface RefusalCase {
  readonly id: string;
  readonly encoding: string | null;
  readonly data: string;
  readonly error: "unsupported" | "malformed";
}

interface EncodingFixture {
  readonly schema: string;
  readonly baseEncoding: string;
  readonly textEncoding: string;
  readonly cases: readonly EncodingCase[];
  readonly refusals: readonly RefusalCase[];
  readonly wireOptionShapes: { readonly tagged: { readonly tag: string; readonly val: string }; readonly taggedNone: { readonly tag: string }; readonly bare: string };
}

function loadFixture(): EncodingFixture {
  const here = dirname(fileURLToPath(import.meta.url));
  return JSON.parse(readFileSync(join(here, "../../🧫️fixtures/⬇️media-export-encoding/🔣️.json"), "utf8")) as EncodingFixture;
}

function friendlyDownload(encoding: unknown, data: string, filename: string, mimeType: string): { readonly filename: string; readonly mimeType: string; readonly data: string; readonly encoding?: string } {
  const effect = wireEffectToFriendly({ tag: "download-media-export", val: { filename, mimeType, data, encoding } }, (bytes) => [...bytes]);
  assert.ok(effect !== null && typeof effect === "object" && "downloadMediaExport" in effect, "the wire decoder must map download-media-export");
  return (effect as { readonly downloadMediaExport: { readonly filename: string; readonly mimeType: string; readonly data: string; readonly encoding?: string } }).downloadMediaExport;
}

/** ⬇️ The whole law, callable without vitest so the nx lane runs it as a plain oracle. */
export function testMediaExportEncodingContract(): void {
  const fixture = loadFixture();
  assert.equal(fixture.baseEncoding, MEDIA_EXPORT_BASE64_ENCODING, "the fixture names the encoding the contract knows");
  assert.equal(fixture.textEncoding, MEDIA_EXPORT_UTF8_ENCODING, "the fixture names the textual spelling a producer may state");
  assert.ok(fixture.cases.length >= 8, "the fixture must keep driving both lanes");

  // 1️⃣ Every fixture export decodes to exactly its declared bytes.
  for (const row of fixture.cases) {
    const bytes = mediaExportBytes(row.data, row.encoding ?? undefined);
    assert.deepEqual([...bytes], [...row.bytes], `${row.id}: decoded bytes`);
  }

  // 2️⃣ A base64 export is NEVER its own base64 text — the assertion every broken shell passed.
  const binary = fixture.cases.filter((row) => row.encoding === MEDIA_EXPORT_BASE64_ENCODING && row.bytes.length > 0);
  assert.ok(binary.length > 0, "the fixture must keep a binary lane");
  for (const row of binary) {
    const bytes = mediaExportBytes(row.data, row.encoding ?? undefined);
    assert.notDeepEqual([...bytes], [...new TextEncoder().encode(row.data)], `${row.id}: was saved as its own base64 text`);
  }

  // 3️⃣ A textual export is its own UTF-8 bytes under BOTH spellings a live producer uses.
  const textual = fixture.cases.filter((row) => row.encoding !== MEDIA_EXPORT_BASE64_ENCODING);
  assert.ok(textual.length >= 3, "the fixture must keep both textual spellings");
  assert.ok(textual.some((row) => row.encoding === null) && textual.some((row) => row.encoding === MEDIA_EXPORT_UTF8_ENCODING), "both spellings must be driven");
  for (const row of textual) {
    assert.deepEqual([...mediaExportBytes(row.data, undefined)], [...new TextEncoder().encode(row.data)], `${row.id}: textual bytes`);
    assert.deepEqual([...mediaExportBytes(row.data, MEDIA_EXPORT_UTF8_ENCODING)], [...new TextEncoder().encode(row.data)], `${row.id}: declared textual bytes`);
  }

  // 4️⃣ Refusals are loud and typed — never a silent text save.
  for (const row of fixture.refusals) {
    let thrown: unknown;
    try {
      mediaExportBytes(row.data, row.encoding ?? undefined);
    } catch (error) {
      thrown = error;
    }
    assert.ok(thrown instanceof MediaExportEncodingError, `${row.id}: must refuse with the typed error`);
    assert.equal((thrown as MediaExportEncodingError).reason, row.error, `${row.id}: refusal reason`);
  }

  // 5️⃣ Third-party oracles: Node's Buffer and the platform `atob`, neither of them ours.
  for (const row of binary) {
    assert.deepEqual([...base64StandardDecode(row.data)], [...Buffer.from(row.data, "base64")], `${row.id}: Buffer oracle`);
    const viaAtob = Uint8Array.from(atob(row.data), (character) => character.charCodeAt(0));
    assert.deepEqual([...base64StandardDecode(row.data)], [...viaAtob], `${row.id}: atob oracle`);
    assert.equal(base64StandardEncode(Uint8Array.from(row.bytes)), Buffer.from(Uint8Array.from(row.bytes)).toString("base64"), `${row.id}: Buffer encode oracle`);
  }

  // 6️⃣ Where we are deliberately STRICTER than `atob` — the reason this codec exists rather than a
  // call into the platform. Each of these is a silent, wrong-bytes save if the platform answers.
  for (const lenient of ["QQ", "Zh==", "Zm=v"]) {
    let atobAccepted = false;
    try {
      atob(lenient);
      atobAccepted = true;
    } catch {
      atobAccepted = false;
    }
    let oursRefused = false;
    try {
      mediaExportBytes(lenient, MEDIA_EXPORT_BASE64_ENCODING);
    } catch (error) {
      oursRefused = error instanceof MediaExportEncodingError;
    }
    assert.ok(oursRefused, `${lenient}: the contract must refuse non-canonical base64 (atob accepted=${atobAccepted})`);
  }

  // 7️⃣ The hop the defect lived on: BOTH WIT `option<string>` shapes must reach the same bytes.
  for (const row of fixture.cases) {
    const shapes: readonly unknown[] = row.encoding === null ? [undefined, fixture.wireOptionShapes.taggedNone] : [row.encoding, { tag: "some", val: row.encoding }];
    for (const shape of shapes) {
      const decoded = friendlyDownload(shape, row.data, row.filename, row.mimeType);
      assert.equal(decoded.filename, row.filename, `${row.id}: filename survives ${JSON.stringify(shape)}`);
      assert.equal(decoded.mimeType, row.mimeType, `${row.id}: mimeType survives ${JSON.stringify(shape)}`);
      assert.equal(decoded.encoding, row.encoding ?? undefined, `${row.id}: encoding survives ${JSON.stringify(shape)}`);
      assert.deepEqual([...mediaExportBytes(decoded.data, decoded.encoding)], [...row.bytes], `${row.id}: bytes after the wire hop ${JSON.stringify(shape)}`);
    }
  }

  // 8️⃣ The option reader itself, used by both renderer doors.
  assert.equal(wireMediaExportEncoding(fixture.wireOptionShapes.tagged), MEDIA_EXPORT_BASE64_ENCODING);
  assert.equal(wireMediaExportEncoding(fixture.wireOptionShapes.bare), MEDIA_EXPORT_BASE64_ENCODING);
  assert.equal(wireMediaExportEncoding(fixture.wireOptionShapes.taggedNone), undefined);
  assert.equal(wireMediaExportEncoding(undefined), undefined);

  console.log(`media-export-encoding cases=${fixture.cases.length} refusals=${fixture.refusals.length} binary=${binary.length} textual=${textual.length} oracles=Buffer,atob`);
}
