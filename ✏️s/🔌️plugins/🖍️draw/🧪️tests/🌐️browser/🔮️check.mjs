/** 🧪️ Exercises neutral PDF oracle refusal and content-change laws through independent rasterization. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { assertPdfDifference, assertPdfPainting } from "./🔮️oracle.mjs";
const fixture = JSON.parse(readFileSync(new URL("./🔮️cases.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("./📐️oracle.json", import.meta.url), "utf8"));
const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
assert(validate(fixture), JSON.stringify(validate.errors));
const document = page => {
  const stream = page.rectangles.map(rectangle => rectangle.join(" ") + " re f").join("\n");
  const objects = ["<< /Type /Catalog /Pages 2 0 R >>", "<< /Type /Pages /Kids [3 0 R] /Count 1 >>", "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 100 100] /Resources << >> /Contents 4 0 R >>", "<< /Length " + Buffer.byteLength(stream) + " >>\nstream\n" + stream + "\nendstream", "<< /Title (" + page.title + ") >>"];
  let pdf = "%PDF-1.4\n";
  const offsets = [0];
  for (const [index, object] of objects.entries()) {
    offsets.push(Buffer.byteLength(pdf));
    pdf += (index + 1) + " 0 obj\n" + object + "\nendobj\n";
  }
  const xref = Buffer.byteLength(pdf);
  pdf += "xref\n0 6\n0000000000 65535 f \n" + offsets.slice(1).map(offset => String(offset).padStart(10, "0") + " 00000 n \n").join("") + "trailer\n<< /Size 6 /Root 1 0 R /Info 5 0 R >>\nstartxref\n" + xref + "\n%%EOF\n";
  return Buffer.from(pdf);
};
let passed = 0;
for (const row of fixture.cases) {
  const left = row.malformed ? Buffer.from("invalid PDF") : document(row.left);
  if (row.relation === "refuse") await assert.rejects(() => assertPdfPainting(left, 1));
  else {
    const right = document(row.right);
    if (row.name === "same PDF") assert.deepEqual(left, right);
    else assert.notDeepEqual(left, right, "Metadata and geometry cases must differ in source bytes");
    const before = await assertPdfPainting(left, 1);
    const after = await assertPdfPainting(right, 1);
    if (row.relation === "same") {
      assert.equal(before.digest, after.digest);
      assert.throws(() => assertPdfDifference(before, after));
    } else assert(assertPdfDifference(before, after).changedPixels > 0);
  }
  console.log("[draw-pdf-oracle] passed: " + row.name);
  passed++;
}
assert.equal(passed, 5);
console.log("[draw-pdf-oracle] 5 passed");
