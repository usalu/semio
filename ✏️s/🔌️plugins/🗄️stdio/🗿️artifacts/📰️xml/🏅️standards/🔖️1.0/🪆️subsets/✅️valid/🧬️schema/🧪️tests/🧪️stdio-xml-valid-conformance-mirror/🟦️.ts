import type { XmlDocument, XmlDoctype, XmlNode, XmlSnapshot } from "../../../../🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import { CODE_BOUNDARY_KIND, CODE_DOCTYPE_MISSING, CODE_DOCTYPE_POSITION, CODE_DOCUMENT_ELEMENT_MISSING, CODE_ENCODING_NAME, CODE_ROOT_NAME_MISMATCH, CODE_STANDALONE_EXTERNAL_SUBSET, CODE_VALIDITY_NOT_VERIFIED, CODE_VERSION, type XmlValidDiagnostic } from "../../🚦️diagnostics/🟦️.ts";

type TestSource = { readonly directory: string; readonly url: string };

export type XmlValidTestDependencies = {
  readonly checkValidConformance: (snapshot: XmlSnapshot) => XmlValidDiagnostic[];
};

/** 🧪 Build a minimal snapshot around one structured doctype for the conformance mirror tests. */
function snapshotWith(
  doctype: Pick<XmlDoctype, 'name' | 'externalId'> | undefined,
  standalone: boolean | undefined,
  rootName: string,
): XmlSnapshot {
  const root: XmlNode = { kind: 'element', name: rootName, attrs: [], children: [] };
  const doc: XmlDocument = {
    declaration:
      standalone === undefined ? { version: '1.0' } : { version: '1.0', standalone },
    doctype: doctype === undefined ? undefined : { ...doctype, declarations: [] },
    root,
    prolog: [],
    epilog: [],
  };
  return { schema: 'stdio.xml', doc };
}

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: XmlValidTestDependencies, source: TestSource): Promise<void> {
  const { checkValidConformance } = dependencies;

  const { describe, expect, it } = vitest;

  describe('stdio.xml valid conformance mirror', () => {
    it('conforming doctype reports only the always-on advisory', () => {
      const diagnostics = checkValidConformance(snapshotWith({ name: 'html' }, undefined, 'html'));
      expect(diagnostics).toHaveLength(1);
      expect(diagnostics[0].code).toBe(CODE_VALIDITY_NOT_VERIFIED);
      expect(diagnostics[0].severity).toBe('warning');
    });

    it('missing doctype is hard', () => {
      const diagnostics = checkValidConformance(snapshotWith(undefined, undefined, 'html'));
      expect(diagnostics.some((d) => d.code === CODE_DOCTYPE_MISSING && d.severity === 'error')).toBe(true);
    });

    it('root name mismatch is hard', () => {
      const diagnostics = checkValidConformance(snapshotWith({ name: 'book' }, undefined, 'html'));
      expect(diagnostics.some((d) => d.code === CODE_ROOT_NAME_MISMATCH && d.severity === 'error')).toBe(true);
    });

    it('standalone yes with external subset is soft', () => {
      const diagnostics = checkValidConformance(
        snapshotWith({ name: 'html', externalId: { kind: 'system', systemId: 'http://example.com/html.dtd' } }, true, 'html'),
      );
      expect(diagnostics.some((d) => d.code === CODE_STANDALONE_EXTERNAL_SUBSET && d.severity === 'warning')).toBe(true);
    });

    it('standalone yes without an external subset stays quiet', () => {
      const diagnostics = checkValidConformance(snapshotWith({ name: 'html' }, true, 'html'));
      expect(diagnostics.map((d) => d.code)).toEqual([CODE_VALIDITY_NOT_VERIFIED]);
    });

    it('a document element, boundary kinds, doctype position, version and encoding are checked like the owned Rust validator', () => {
      const base = snapshotWith({ name: 'html' }, undefined, 'html');
      const without: typeof base = { ...base, doc: { ...base.doc, root: undefined } };
      expect(checkValidConformance(without).some((d) => d.code === CODE_DOCUMENT_ELEMENT_MISSING && d.severity === 'error')).toBe(true);
      const boundary: typeof base = { ...base, doc: { ...base.doc, epilog: [{ kind: 'text', text: 'x' }] } };
      expect(checkValidConformance(boundary).some((d) => d.code === CODE_BOUNDARY_KIND)).toBe(true);
      const position: typeof base = { ...base, doc: { ...base.doc, doctype: { name: 'html', declarations: [], prologPosition: 1n } } };
      expect(checkValidConformance(position).some((d) => d.code === CODE_DOCTYPE_POSITION)).toBe(true);
      const version: typeof base = { ...base, doc: { ...base.doc, declaration: { version: '1.1' } } };
      expect(checkValidConformance(version).some((d) => d.code === CODE_VERSION)).toBe(true);
      const encoding: typeof base = { ...base, doc: { ...base.doc, declaration: { version: '1.0', encoding: '9bad' } } };
      expect(checkValidConformance(encoding).some((d) => d.code === CODE_ENCODING_NAME)).toBe(true);
      const good: typeof base = { ...base, doc: { ...base.doc, declaration: { version: '1.0', encoding: 'UTF-8' } } };
      expect(checkValidConformance(good).map((d) => d.code)).toEqual([CODE_VALIDITY_NOT_VERIFIED]);
    });
  });

}
