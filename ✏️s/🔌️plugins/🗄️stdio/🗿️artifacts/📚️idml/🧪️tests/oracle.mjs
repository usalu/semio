/** 🧪️ Independent IDML package/reference oracle; XML admission is supplied through a test-only library port. */
import jsonPatch from "fast-json-patch";
import { zipSync, unzipSync, strFromU8 } from "fflate";

/** 📚️ Applies RFC 6902 witnesses and checks package ZIP/XML outputs with independent libraries. */
export function verifyIdmlMutations(corpus, xml) {
  let cases = 0;
  let xmlParts = 0;
  let archives = 0;
  for (const vector of corpus.cases) {
    const saved = JSON.stringify(vector.before);
    const admitted = (document) => {
      const paths = document.parts.map((part) => part.path);
      if (new Set(paths).size !== paths.length || paths.some((path) => !path || path.startsWith("/") || /[\\\\:]/.test(path) || path.replace(/\/$/, "").split("/").some((part) => !part || part === "." || part === ".."))) return false;
      if (paths[0] !== "mimetype" || strFromU8(Uint8Array.from(document.parts[0].bytes)) !== "application/vnd.adobe.indesign-idml-package") return false;
      if (document.parts.reduce((size, part) => size + part.bytes.length, 0) > (8 << 20) || paths.length > 16384) return false;
      for (const part of document.parts.filter((part) => part.path.endsWith(".xml"))) {
        const text = strFromU8(Uint8Array.from(part.bytes));
        if (/<!DOCTYPE/i.test(text) || xml.validate(text) !== true) return false;
      }
      const designmap = document.parts.find((part) => part.path === "designmap.xml");
      if (!designmap) return false;
      const version = xml.parse(strFromU8(Uint8Array.from(designmap.bytes))).Document?.["@_DOMVersion"];
      return version === document.domVersion && /^[0-9]+\.[0-9]+$/.test(version);
    };
    let candidate = structuredClone(vector.before);
    if (vector.mutation.mutation === "replacePackage") candidate = structuredClone(vector.mutation.document);
    else {
      const part = candidate.parts.find((part) => part.path === vector.mutation.path);
      if (!part) candidate = null;
      else {
        part.bytes = vector.mutation.bytes;
        if (part.path === "designmap.xml" && xml.validate(strFromU8(Uint8Array.from(part.bytes))) === true) candidate.domVersion = xml.parse(strFromU8(Uint8Array.from(part.bytes))).Document?.["@_DOMVersion"];
      }
    }
    const accepted = !!candidate && admitted(candidate);
    if (accepted !== vector.accepted) throw new Error(vector.name + ": explicit admission witness mismatch");
    const semantic = accepted ? candidate : vector.before;
    if (JSON.stringify(semantic) !== JSON.stringify(vector.after)) throw new Error(vector.name + ": independent mutation outcome mismatch");
    const after = jsonPatch.applyPatch(structuredClone(vector.before), vector.patch, true, false).newDocument;
    if (JSON.stringify(after) !== JSON.stringify(vector.after)) throw new Error(vector.name + ": JSON Patch witness mismatch");
    if (JSON.stringify(vector.before) !== saved) throw new Error(vector.name + ": modified base");
    const files = {};
    for (const part of after.parts) {
      const bytes = Uint8Array.from(part.bytes);
      files[part.path] = [bytes, { level: 0 }];
      if (part.path.endsWith(".xml")) {
        const text = strFromU8(bytes);
        if (xml.validate(text) !== true) throw new Error(vector.name + ": invalid XML output");
        xmlParts++;
      }
    }
    const archive = zipSync(files);
    const restored = unzipSync(archive);
    if (Object.keys(restored).join("\n") !== after.parts.map((part) => part.path).join("\n")) throw new Error(vector.name + ": archive order mismatch");
    if (archive[8] !== 0 || archive[9] !== 0) throw new Error(vector.name + ": mimetype must be stored");
    if (strFromU8(restored.mimetype) !== "application/vnd.adobe.indesign-idml-package") throw new Error(vector.name + ": MIME identity mismatch");
    for (const part of after.parts) if ([...restored[part.path]].join(",") !== part.bytes.join(",")) throw new Error(vector.name + ": archive payload mismatch");
    const design = xml.parse(strFromU8(restored["designmap.xml"]));
    if (design.Document?.["@_DOMVersion"] !== after.domVersion) throw new Error(vector.name + ": DOM version mismatch");
    archives++;
    cases++;
  }
  return { cases, archives, xmlParts };
}
