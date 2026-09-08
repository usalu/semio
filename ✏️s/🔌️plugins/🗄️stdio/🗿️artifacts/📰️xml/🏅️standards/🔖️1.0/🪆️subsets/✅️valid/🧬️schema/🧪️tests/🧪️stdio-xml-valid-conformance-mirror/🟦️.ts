type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { CODE_DOCTYPE_MISSING, CODE_ROOT_NAME_MISMATCH, CODE_STANDALONE_EXTERNAL_SUBSET, CODE_VALIDITY_NOT_VERIFIED, checkValidConformance, meta, snapshotWith } = dependencies;

  const { describe, expect, it } = vitest;

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
