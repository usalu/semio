import assert from "node:assert/strict";
import { join, isAbsolute } from "node:path";
import { writeFile } from "node:fs/promises";
import { createHash } from "node:crypto";
import { printTemplatePdfNames, withPublishedPrintDocumentV1 } from "../../../🔨️modules/🖨️tectonic-template-compilation/🟦️.ts";
import { readPrintMaterialFileV1 } from "../../../📦️artifacts/📥️material/📤️producer/🟦️.ts";
import { inspectPrintPdfV1, rasterPrintPdfV1 } from "../../../📦️artifacts/📖️pdf/🟦️.ts";
import { withPrintScratchV1, type PrintExecutionV1 } from "../../../📦️artifacts/🎛️operation/🟦️.ts";
import fixture from "../🔣️.json";
import contrast from "../../../🔨️modules/🎨print-design-token-paints/🧪️tests/🔣️.json";
import { loadPrintDesignTokens } from "../../../🔨️modules/🎨print-design-token-paints/🧮️resolution/🟦️.ts";
import { thesisPdfPaintFactsV1, thesisFilledHeadingNumberPaintV1 } from "./🎨️paint/🟦️.ts";
import { contrastRatio } from "@semio-tech/ui-styling";
import { colord, extend } from "colord";
import a11yPlugin from "colord/plugins/a11y";
extend([a11yPlugin]);
import { captureOwnedCommandResultV1, OwnedCommandFailure } from "@semio-tech/process/owned-execution";
import { readNativeCallerAuthorityV1 } from "@semio-tech/repo-lib/artifacts/native-orchestration/invocation";
import { scriptInvocationEnvironment, withRequestedIssuedResourceControlV1, type ScriptInvocation } from "@semio-tech/process/routing";
import { checkResourceLeaseControl, type ResourceLeaseControl } from "@semio-tech/process/leases";

const normalize = (value: string): string => value.normalize("NFC").replace(/[‘’]/g, "'").replace(/\s+/g, "");

/** 🚪️ Executes the unchanged physical compilation script outside every parent Print store operation. */
export async function runThesisPublicationBuilds(checkout: string, workspace: string, invocation: ScriptInvocation, phaseControl: ResourceLeaseControl): Promise<void> {
  const script = join(checkout, "🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📜️script.ts");
  for (const row of fixture.publications) await withRequestedIssuedResourceControlV1(phaseControl, 0, async control => {
    const transport = readNativeCallerAuthorityV1(invocation.capabilities).transport;
    const captured = await captureOwnedCommandResultV1(process.execPath, [script, "build", row.id], workspace, {
      control, env: scriptInvocationEnvironment(invocation, process.env),
      output: { maximumBytes: Math.min(transport.maximumBytes, 16777216), maximumLines: Math.min(transport.maximumLines, 65536) },
    });
    const primary = captured.status !== 0 || captured.signal !== null ? new OwnedCommandFailure("Actual Print compilation entry", { version: 1, status: captured.status, signal: captured.signal }) : undefined;
    try {
      const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
      assert.ok(artifacts && isAbsolute(artifacts), "Absolute compilation diagnostic directory");
      const target = join(artifacts, "actual-" + row.id + "-entry.log");
      assert.ok(target.length <= fixture.privateWorkspace.maximumPathLength);
      checkResourceLeaseControl(control);
      await writeFile(target, "status=" + captured.status + " signal=" + captured.signal + "\n[stdout]\n" + captured.stdout + "\n[stderr]\n" + captured.stderr, { signal: control.signal });
    } catch (error) {
      checkResourceLeaseControl(control);
      throw primary ?? error;
    }
    checkResourceLeaseControl(control);
    if (primary) throw primary;
    console.log("[print] Actual compilation entry built " + row.id);
  });
}

/** 📤️ Publishes actual private theme PDFs and exports independent text and cover/body raster evidence. */
export async function verifyThesisPublication(execution: PrintExecutionV1): Promise<void> {
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  assert.ok(artifacts && isAbsolute(artifacts), "Explicit absolute private test proof directory");
  const proofs: unknown[] = [], report = join(artifacts, "publication-proof.json");
  const exportFile = async (name: string, bytes: Uint8Array | string): Promise<void> => {
    const target = join(artifacts, name);
    assert.ok(target.length <= fixture.privateWorkspace.maximumPathLength);
    await writeFile(target, bytes);
  };
  const summary = (complete: boolean): string => JSON.stringify({ kind: "private-test-publication", assertionVersion: fixture.publicationProof.version, buildIngress: "actual-original-root-entry", canonicalWorkspaceProduct: false, complete, files: proofs }, null, 2) + "\n";
  assert.ok(report.length <= fixture.privateWorkspace.maximumPathLength);
  await writeFile(report, summary(false));
  for (const row of fixture.publications) {
    await withPrintScratchV1(execution, "thesis-publication", async scratch => {
      await withPublishedPrintDocumentV1(scratch.execution, "print", row.id, "@semio-tech/print:build-" + row.id, async artifact => {
        const names = Object.values(printTemplatePdfNames(row.id + ".tex"));
        assert.deepEqual([...artifact.files].sort(), [...names].sort());
        for (const name of names) {
          const bytes = await readPrintMaterialFileV1(join(artifact.path, name), 67108864, artifact.control);
          assert.equal(new TextDecoder().decode(bytes.subarray(0, 5)), "%PDF-");
          await exportFile("private-" + name, bytes);
          const actual = { operation: scratch.execution.operation, phaseControl: artifact.control };
          const pdf = await inspectPrintPdfV1(actual, scratch.destination, bytes);
          assert.ok(pdf.pages.length >= 4, name + " has academic content");
          const pages = pdf.pages.map((page, index) => ({ page: index + 1, text: page.items.flatMap(item => "str" in item ? [item.str] : []).join(" ") }));
          const text = pages.map(page => page.text).join(" "), normalized = normalize(text);
          const textFile = "private-" + name.replace(/\.pdf$/, "") + "-text.json", textProof = JSON.stringify({ canonicalWorkspaceProduct: false, assertionVersion: fixture.publicationProof.version, pages });
          assert.ok(Buffer.byteLength(textProof) <= fixture.publicationProof.maximumTextBytes);
          await exportFile(textFile, textProof + "\n");
          assert.doesNotMatch(text, /[\uFFFD\uFFFF]|\?{2}|\[\s*\?\s*\]/u);
          for (const expected of row.expected) assert.ok(normalized.includes(normalize(expected)), name + ": " + expected);
          for (const expected of row.bibliography) assert.ok(normalized.toLowerCase().includes(normalize(expected).toLowerCase()), name + " bibliography: " + expected);
          for (const key of row.citationKeys) assert.ok(!normalized.includes(key), name + " resolves citation " + key);
          const body = pdf.pages.findIndex((page, index) => normalize(pages[index]!.text).includes(normalize(row.firstBodyMarker)) && page.items.some(item => "str" in item && item.str.trim() === "1" && item.transform[5] < page.height / 4));
          assert.ok(body > 0, name + " has its first Arabic body page");
          const theme = name.includes("-dark") ? "dark" : "light", inks = contrast.actualThemes.find(row => row.theme === theme), surface = loadPrintDesignTokens().colors.secondary;
          assert.ok(inks && surface);
          const paint = thesisPdfPaintFactsV1(pdf.pages[body]!, artifact.control), paintProof = JSON.stringify({ canonicalWorkspaceProduct: false, page: body + 1, ...paint });
          assert.ok(Buffer.byteLength(paintProof) <= fixture.publicationProof.maximumTextBytes);
          await exportFile("private-" + name.replace(/\.pdf$/, "-paint.json"), paintProof + "\n");
          assert.ok(paint.fills.includes(surface.toLowerCase()), name + " actual filled section surface");
          const filledHeadings = row.filledHeadings.map(heading => {
            const runs = thesisFilledHeadingNumberPaintV1(paint.runs, heading, "#" + inks.filled);
            const numerals = runs.map(run => ({ ...run, ratio: contrastRatio(run.color, surface), oracleRatio: colord(run.color).contrast(surface) }));
            for (const numeral of numerals) {
              assert.ok(numeral.ratio >= contrast.minimum, name + " filled numeral contrast: " + heading.title + " / " + numeral.text);
              assert.ok(Math.abs(numeral.ratio - numeral.oracleRatio) < 0.011);
            }
            return { ...heading, numerals };
          });
          const rasters = await rasterPrintPdfV1(actual, scratch.destination, bytes, [1, body + 1], fixture.publicationProof.previewScale), previews: unknown[] = [];
          for (const image of rasters) {
            const file = "private-" + name.replace(/\.pdf$/, "") + (image.page === 1 ? "-cover.png" : "-body.png");
            await exportFile(file, image.png);
            previews.push({ file, page: image.page, width: image.width, height: image.height, sha256: image.sha256 });
          }
          proofs.push({ file: "private-" + name, pages: pdf.pages.length, sha256: createHash("sha256").update(bytes).digest("hex"), firstArabicBodyPage: body + 1, textFile, filledHeadings, previews });
          console.log("[print] Privately published, consumed and exported " + name + " PASS");
        }
        await artifact.recheck();
      });
    });
  }
  await writeFile(report, summary(true));
}
