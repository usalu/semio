type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { CODE_BOUNDARY_KIND, CODE_DOCTYPE_MISSING, CODE_DOCTYPE_POSITION, CODE_DOCUMENT_ELEMENT_MISSING, CODE_ENCODING_NAME, CODE_ROOT_NAME_MISMATCH, CODE_STANDALONE_EXTERNAL_SUBSET, CODE_VALIDITY_NOT_VERIFIED, CODE_VERSION, checkValidConformance, meta, snapshotWith } = dependencies;

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
      const without = { ...base, doc: { ...base.doc, root: undefined } };
      expect(checkValidConformance(without).some((d) => d.code === CODE_DOCUMENT_ELEMENT_MISSING && d.severity === 'error')).toBe(true);
      const boundary = { ...base, doc: { ...base.doc, epilog: [{ kind: 'text', text: 'x' }] } };
      expect(checkValidConformance(boundary).some((d) => d.code === CODE_BOUNDARY_KIND)).toBe(true);
      const position = { ...base, doc: { ...base.doc, doctype: { ...base.doc.doctype, prologPosition: 1n } } };
      expect(checkValidConformance(position).some((d) => d.code === CODE_DOCTYPE_POSITION)).toBe(true);
      const version = { ...base, doc: { ...base.doc, declaration: { version: '1.1' } } };
      expect(checkValidConformance(version).some((d) => d.code === CODE_VERSION)).toBe(true);
      const encoding = { ...base, doc: { ...base.doc, declaration: { version: '1.0', encoding: '9bad' } } };
      expect(checkValidConformance(encoding).some((d) => d.code === CODE_ENCODING_NAME)).toBe(true);
      const good = { ...base, doc: { ...base.doc, declaration: { version: '1.0', encoding: 'UTF-8' } } };
      expect(checkValidConformance(good).map((d) => d.code)).toEqual([CODE_VALIDITY_NOT_VERIFIED]);
    });
  });

}
