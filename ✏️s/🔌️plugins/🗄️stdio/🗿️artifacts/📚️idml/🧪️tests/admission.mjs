/** 🔬️ Supplies the package-owned admission port through an independent test-only XML library. */
export function xmlAdmission(xml) {
  const parser = new xml.XMLParser({ignoreAttributes: false, parseAttributeValue: false});
  return {admit(parts) {
    let version;
    for (const part of parts) if (part.path.endsWith('.xml')) {
      const source = new TextDecoder('utf-8', {fatal: true}).decode(Uint8Array.from(part.bytes));
      if (/<!DOCTYPE/i.test(source) || xml.XMLValidator.validate(source) !== true) throw new Error('oracle.idml.xml');
      if (part.path === 'designmap.xml') version = parser.parse(source).Document?.['@_DOMVersion'];
    }
    if (typeof version !== 'string' || !/^[0-9]+\.[0-9]+$/.test(version)) throw new Error('oracle.idml.designmap');
    return version;
  }};
}
