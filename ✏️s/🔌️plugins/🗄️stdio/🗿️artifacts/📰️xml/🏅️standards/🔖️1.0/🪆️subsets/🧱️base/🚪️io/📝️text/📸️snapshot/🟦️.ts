/** 📝️ Text representation for `stdio.xml` (snapshot) -- the well-formed XML document text itself
 * (`xml_document_to_text`/`xml_document_from_text`'s wire form: optional `<?xml ...?>`
 * declaration, optional `<!DOCTYPE ...>`, then the root element), wrapped in the semio envelope
 * header for the `.xml` DSL surface. */
export type XmlSnapshotText = string;
