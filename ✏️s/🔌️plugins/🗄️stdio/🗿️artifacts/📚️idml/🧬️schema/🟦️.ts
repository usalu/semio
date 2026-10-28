/** 📚️ IDML intrinsic values preserve ordered package members independently of ZIP serialization. */
export interface IdmlPart { path: string; bytes: number[] }
export interface IdmlPackage { domVersion: string; parts: IdmlPart[] }

/** 🧬️ Parses the structural package contract; the canonical codec admits XML and package relationships. */
export function parseIdmlPackageValue(value: unknown): IdmlPackage {
  if (typeof value !== 'object' || value === null || Array.isArray(value)) throw new Error('stdio.idml.value');
  const record = value as Record<string, unknown>;
  if (Object.keys(record).some(key => key !== 'domVersion' && key !== 'parts') || typeof record.domVersion !== 'string' || !/^[0-9]+\.[0-9]+$/.test(record.domVersion) || !Array.isArray(record.parts) || record.parts.length > 16384) throw new Error('stdio.idml.value');
  let size = 0;
  const names = new Set<string>();
  const parts = record.parts.map((row: unknown): IdmlPart => {
    if (typeof row !== 'object' || row === null || Array.isArray(row)) throw new Error('stdio.idml.part');
    const part = row as Record<string, unknown>;
    if (Object.keys(part).some(key => key !== 'path' && key !== 'bytes') || typeof part.path !== 'string' || !part.path || part.path.startsWith('/') || /[\\:]/.test(part.path) || part.path.replace(/\/$/, '').split('/').some(segment => !segment || segment === '.' || segment === '..') || names.has(part.path) || !Array.isArray(part.bytes)) throw new Error('stdio.idml.part');
    names.add(part.path);
    size += part.bytes.length;
    if (size > 8 * 1024 * 1024 || part.bytes.some(byte => !Number.isInteger(byte) || byte < 0 || byte > 255) || part.path.endsWith('/') && part.bytes.length) throw new Error('stdio.idml.bytes');
    return {path: part.path, bytes: [...part.bytes]};
  });
  const mime = 'application/vnd.adobe.indesign-idml-package';
  if (parts[0]?.path !== 'mimetype' || parts[0].bytes.length !== mime.length || parts[0].bytes.some((byte, index) => byte !== mime.charCodeAt(index)) || !names.has('designmap.xml')) throw new Error('stdio.idml.package');
  return {domVersion: record.domVersion, parts};
}
