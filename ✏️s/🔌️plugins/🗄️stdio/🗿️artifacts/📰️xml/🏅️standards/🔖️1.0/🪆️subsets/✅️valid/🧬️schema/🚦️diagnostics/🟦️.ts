/** 🚦️ The diagnostic vocabulary of the XML valid subset: severities, the diagnostic type and the codes of the conformance gate. */

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

export const CODE_DOCUMENT_ELEMENT_MISSING = 'stdio.xml.valid.document-element-missing';

export const CODE_BOUNDARY_KIND = 'stdio.xml.valid.boundary-kind';

export const CODE_DOCTYPE_POSITION = 'stdio.xml.valid.doctype-position';

export const CODE_VERSION = 'stdio.xml.valid.version';

export const CODE_ENCODING_NAME = 'stdio.xml.valid.encoding-name';
