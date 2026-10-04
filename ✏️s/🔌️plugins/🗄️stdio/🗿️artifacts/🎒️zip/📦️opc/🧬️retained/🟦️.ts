import type {OpcPart, OpcRelationship} from '../🟦️.ts';
/** 🏷️ One retained metadata owner keeps both named literal fields. */
export interface RetainedOpcMetadataEntry { name: string; contentType: string }
/** 🧬️ Logical retained OPC fields preserve ordered duplicate metadata records. */
export interface RetainedOpcPackage {
  parts: OpcPart[];
  contentTypes: {defaults: RetainedOpcMetadataEntry[]; overrides: RetainedOpcMetadataEntry[]};
  relationships: Record<string, OpcRelationship[]>;
  comment: string;
}
