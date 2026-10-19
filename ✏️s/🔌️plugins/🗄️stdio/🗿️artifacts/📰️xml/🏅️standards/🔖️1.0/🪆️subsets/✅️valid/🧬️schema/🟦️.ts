/** 🧬️ XmlSnapshot schema (✳️valid subset) — reuses ✳️any types; mirrors Rust `check_valid_conformance`. */
import type { XmlSnapshot } from '../../🧱️base/🧬️schema/📸️snapshot/🟦️.ts';
import { CODE_DOCTYPE_MISSING, CODE_ROOT_NAME_MISMATCH, CODE_STANDALONE_EXTERNAL_SUBSET, CODE_VALIDITY_NOT_VERIFIED, CODE_DOCUMENT_ELEMENT_MISSING, CODE_BOUNDARY_KIND, CODE_DOCTYPE_POSITION, CODE_VERSION, CODE_ENCODING_NAME } from "./🚦️diagnostics/🟦️.ts";
import type { XmlValidDiagnostic } from "./🚦️diagnostics/🟦️.ts";

export const meta = {
  artifactKind: 's.stdio.xml',
  standard: '1.0',
  subset: 'valid',
} as const;

function rootElementName(snapshot: XmlSnapshot): string | undefined {
  const root = snapshot.doc.root;
  if (!root || root.kind !== 'element') return undefined;
  return root.name;
}

const hard = (code: string, message: string): XmlValidDiagnostic => ({ code, severity: 'error', message });
const soft = (code: string, message: string): XmlValidDiagnostic => ({ code, severity: 'warning', message });

/** 🛡️ Scope-limited XML 1.0 §5.1 validity gate — mirrors `derived_analysis::check_valid_conformance`. */
export function checkValidConformance(snapshot: XmlSnapshot): XmlValidDiagnostic[] {
  const out: XmlValidDiagnostic[] = [];
  const { doctype, declaration, prolog, epilog } = snapshot.doc;
  const actualRoot = rootElementName(snapshot);
  if (actualRoot === undefined) out.push(hard(CODE_DOCUMENT_ELEMENT_MISSING, 'XML validity requires a document element'));
  if ([...prolog, ...epilog].some((node) => node.kind !== 'comment' && node.kind !== 'processingInstruction')) {
    out.push(hard(CODE_BOUNDARY_KIND, 'XML validity permits only comments and processing instructions outside the document element'));
  }
  if (doctype !== undefined && (doctype.prologPosition ?? 0n) > BigInt(prolog.length)) {
    out.push(hard(CODE_DOCTYPE_POSITION, 'XML validity requires the doctype to occupy its declared prolog position'));
  }
  if (declaration !== undefined) {
    if (declaration.version !== '1.0') out.push(hard(CODE_VERSION, 'XML 1.0 validity requires declaration version 1.0'));
    if (declaration.encoding !== undefined && !/^[A-Za-z][A-Za-z0-9._-]*$/.test(declaration.encoding)) {
      out.push(hard(CODE_ENCODING_NAME, 'XML validity requires a syntactically valid declared encoding name'));
    }
  }
  if (doctype === undefined) {
    out.push(hard(CODE_DOCTYPE_MISSING, 'no <!DOCTYPE ...> declaration present -- XML 1.0 §5.1 validity requires one (a document without one can be well-formed at best)'));
  } else {
    if (actualRoot !== undefined && doctype.name !== actualRoot) {
      out.push(hard(CODE_ROOT_NAME_MISMATCH, `doctype declares root name '${doctype.name}' but the actual root element is '<${actualRoot}>' -- §2.8 requires the DOCTYPE Name to match the document element`));
    }
    if (doctype.externalId !== undefined && declaration?.standalone === true) {
      out.push(soft(CODE_STANDALONE_EXTERNAL_SUBSET, 'XML declaration says standalone="yes" but the doctype references an external subset (SYSTEM/PUBLIC) -- suspicious per §2.9'));
    }
  }
  out.push(soft(CODE_VALIDITY_NOT_VERIFIED, 'validity not fully verified: the typed doctype retains its name, external identifiers and entity declarations; DTD element and attribute content models (§3.2/§3.3) are absent from this owned schema'));
  return out;
}

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️stdio-xml-valid-conformance-mirror/🟦️.ts");
  await registerTests1(import.meta.vitest, { checkValidConformance }, { directory: import.meta.dir, url: import.meta.url });
}
