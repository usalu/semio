import type { DwgSnapshot } from '../📸️snapshot/🟦️.ts';
export type DwgMutation =
  | { mutation: 'setVersionInfo'; version: string; maintenanceVersion: number; codepage: number }
  | { mutation: 'setDrawing'; drawing: DwgSnapshot['drawing'] }
  | { mutation: 'setHeader'; header: DwgSnapshot['header'] }
  | { mutation: 'setClasses'; classes: DwgSnapshot['classes'] }
  | { mutation: 'setDependencies'; dependencies: DwgSnapshot['dependencies'] }
  | { mutation: 'setSummary'; summary: DwgSnapshot['summary'] }
  | { mutation: 'setApplication'; application: DwgSnapshot['application'] }
  | { mutation: 'setTemplate'; template: DwgSnapshot['template'] }
  | { mutation: 'setAuxiliaryHeader'; auxiliaryHeader: DwgSnapshot['auxiliaryHeader'] }
  | { mutation: 'setRevisionHistory'; revisionHistory: DwgSnapshot['revisionHistory'] }
  | { mutation: 'setPreview'; preview: DwgSnapshot['preview'] }
  | { mutation: 'setApplicationHistory'; applicationHistory: DwgSnapshot['applicationHistory'] };
