/** 🧬️ XmlSnapshot schema (✳️valid subset) — reuses ✳️any types; mirrors Rust `check_valid_conformance`. */
import type { XmlDocument, XmlNode, XmlSnapshot } from '../../✳️any/🧬️schema/📸️snapshot/🟦️component.ts';

export const meta = {
  artifactKind: 's.stdio.xml',
  standard: '1.0',
  subset: 'valid',
} as const;

export type XmlValidDiagnosticSeverity = 'error' | 'warning' | 'fatal';

export interface XmlValidDiagnostic {
  code: string;
  severity: XmlValidDiagnosticSeverity;
  message: string;
}

export const CODE_DOCTYPE_MISSING = 'stdio.xml.valid.doctype-missing';
export const CODE_ROOT_NAME_MISMATCH = 'stdio.xml.valid.root-name-mismatch';
export const CODE_STANDALONE_EXTERNAL_SUBSET = 'stdio.xml.valid.standalone-external-subset';
export const CODE_VALIDITY_NOT_VERIFIED = 'stdio.xml.valid.validity-not-fully-verified';

function parseDoctypeRootName(doctype: string): string | undefined {
  const lower = doctype.toLowerCase();
  if (!lower.startsWith('<!doctype')) return undefined;
  const rest = doctype.slice('<!doctype'.length).trimStart();
  const end = rest.search(/\s|\[|>/);
  const name = end === -1 ? rest : rest.slice(0, end);
  return name.length > 0 ? name : undefined;
}

function rootElementName(snapshot: XmlSnapshot): string | undefined {
  const root = snapshot.doc.root;
  if (!root || root.kind !== 'element') return undefined;
  return root.name;
}

function doctypeReferencesExternalSubset(doctype: string): boolean {
  return doctype.includes('SYSTEM') || doctype.includes('PUBLIC');
}

/** 🛡️ Scope-limited XML 1.0 §5.1 validity gate — mirrors `derived_analysis::check_valid_conformance`. */
export function checkValidConformance(snapshot: XmlSnapshot): XmlValidDiagnostic[] {
  const out: XmlValidDiagnostic[] = [];
  const doctype = snapshot.doc.doctype;
  if (doctype === undefined) {
    out.push({
      code: CODE_DOCTYPE_MISSING,
      severity: 'error',
      message:
        'no <!DOCTYPE ...> declaration present -- XML 1.0 §5.1 validity requires one (a document without one can be well-formed at best)',
    });
  } else {
    const declaredRoot = parseDoctypeRootName(doctype);
    const actualRoot = rootElementName(snapshot);
    if (declaredRoot !== undefined && actualRoot !== undefined && declaredRoot !== actualRoot) {
      out.push({
        code: CODE_ROOT_NAME_MISMATCH,
        severity: 'error',
        message: `doctype declares root name '${declaredRoot}' but the actual root element is '<${actualRoot}>' -- §2.8 requires the DOCTYPE Name to match the document element`,
      });
    }
    if (doctypeReferencesExternalSubset(doctype) && snapshot.doc.declaration?.standalone === true) {
      out.push({
        code: CODE_STANDALONE_EXTERNAL_SUBSET,
        severity: 'warning',
        message:
          'XML declaration says standalone="yes" but the doctype references an external subset (SYSTEM/PUBLIC) -- suspicious per §2.9',
      });
    }
  }
  out.push({
    code: CODE_VALIDITY_NOT_VERIFIED,
    severity: 'warning',
    message:
      'validity not fully verified: doctype is retained as raw unparsed String without internal/external subset markup declarations parsed, so full DTD content-model validation is out of scope -- only doctype presence and declared-root/actual-root agreement are checked',
  });
  return out;
}

export function snapshotWith(
  doctype: string | undefined,
  standalone: boolean | undefined,
  rootName: string,
): XmlSnapshot {
  const root: XmlNode = { kind: 'element', name: rootName, attrs: [], children: [] };
  const doc: XmlDocument = {
    declaration:
      standalone === undefined ? { version: '1.0' } : { version: '1.0', standalone },
    doctype,
    root,
  };
  return { schema: 'stdio.xml', doc };
}

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe('stdio.xml valid conformance mirror', () => {
    it('conforming doctype reports only the always-on advisory', () => {
      const diagnostics = checkValidConformance(snapshotWith('<!DOCTYPE html>', undefined, 'html'));
      expect(diagnostics).toHaveLength(1);
      expect(diagnostics[0].code).toBe(CODE_VALIDITY_NOT_VERIFIED);
      expect(diagnostics[0].severity).toBe('warning');
    });

    it('missing doctype is hard', () => {
      const diagnostics = checkValidConformance(snapshotWith(undefined, undefined, 'html'));
      expect(diagnostics.some((d) => d.code === CODE_DOCTYPE_MISSING && d.severity === 'error')).toBe(true);
    });

    it('root name mismatch is hard', () => {
      const diagnostics = checkValidConformance(snapshotWith('<!DOCTYPE book>', undefined, 'html'));
      expect(diagnostics.some((d) => d.code === CODE_ROOT_NAME_MISMATCH && d.severity === 'error')).toBe(true);
    });

    it('standalone yes with external subset is soft', () => {
      const diagnostics = checkValidConformance(
        snapshotWith('<!DOCTYPE html SYSTEM "http://example.com/html.dtd">', true, 'html'),
      );
      expect(diagnostics.some((d) => d.code === CODE_STANDALONE_EXTERNAL_SUBSET)).toBe(true);
    });
  });
}
