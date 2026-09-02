export * from '../../🟦️.ts';
import type { DwgApplicationHistory, DwgApplicationInfo, DwgAuxiliaryHeader, DwgClass, DwgDependency, DwgHeaderVariables, DwgIndexedPreview, DwgLogicalDrawing, DwgRevisionHistory, DwgSummaryInfo, DwgTemplate } from '../../🟦️.ts';

export interface DwgDiff {
  version?: string;
  maintenanceVersion?: number;
  codepage?: number;
  drawing?: DwgLogicalDrawing;
  header?: DwgHeaderVariables;
  classes?: DwgClass[];
  dependencies?: DwgDependency[];
  summary?: DwgSummaryInfo;
  application?: DwgApplicationInfo;
  template?: DwgTemplate;
  auxiliaryHeader?: DwgAuxiliaryHeader;
  revisionHistory?: DwgRevisionHistory;
  preview?: DwgIndexedPreview;
  applicationHistory?: DwgApplicationHistory;
}
