/** 🚪️ IO mirror for stdio.xml 1.0/✳️valid — reuses ✳️any wire kinds; derived gate lives in schema TS. */
export const meta = {
  artifactKind: 's.stdio.xml',
  standard: '1.0',
  subset: 'valid',
  archetype: 'derived' as const,
  derivesFrom: '*',
} as const;

export const importKinds = ['txt/utf-8/*'] as const;
export const exportKinds = ['txt/utf-8/*'] as const;

export const hardCodes = [
  'stdio.xml.valid.doctype-missing',
  'stdio.xml.valid.root-name-mismatch',
] as const;

export const negativeExamples = ['no-doctype'] as const;
