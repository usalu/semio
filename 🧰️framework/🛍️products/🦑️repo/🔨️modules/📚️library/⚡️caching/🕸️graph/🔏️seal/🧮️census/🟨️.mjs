/** 🧮️ Pure census of persisted inference records: the header digest and the distinct observation rows of each, so a graph seal can be verified from one deduplicated fact table instead of by reading every record again. */
import { readFileSync } from "node:fs";
import { brotliDecompressSync } from "node:zlib";
import { createHash } from "node:crypto";
import { dirname, join } from "node:path";

const blocks = new Map();

/** 🧱️ The rows of one fact block, verified against its name and read once per worker. */
function blockRows(directory, digest) {
  let rows = blocks.get(digest);
  if (!rows) {
    const text = brotliDecompressSync(readFileSync(join(directory, digest.slice(0, 24) + ".json"))).toString("utf8");
    if (createHash("sha256").update(text).digest("hex") !== digest) throw new Error("Inference block content differs from its name");
    blocks.set(digest, rows = JSON.parse(text));
  }
  return rows;
}

/** 📊️ Reads the records in order; `rows` holds each distinct row once, as its canonical JSON text. */
export function censusInferenceRecordsV1(paths, context) {
  const distinct = new Set(), records = [];
  for (let index = 0; index < paths.length; index++) {
    const text = brotliDecompressSync(readFileSync(paths[index])).toString("utf8"), split = text.indexOf("\n");
    if (split < 0) throw new Error("Inference record header required: " + paths[index]);
    const header = text.slice(0, split), parsed = JSON.parse(header), observations = Array.isArray(parsed.observations) ? [...(parsed.blocks ?? []).flatMap((digest) => blockRows(dirname(paths[index]), digest)), ...parsed.observations] : undefined;
    if (!Array.isArray(observations)) throw new Error("Inference record observations required: " + paths[index]);
    for (const row of observations) distinct.add(JSON.stringify(row));
    records.push([paths[index], createHash("sha256").update(header).digest("hex"), observations.length]);
    context?.progress?.(index + 1, paths.length);
  }
  return { records, rows: [...distinct] };
}
