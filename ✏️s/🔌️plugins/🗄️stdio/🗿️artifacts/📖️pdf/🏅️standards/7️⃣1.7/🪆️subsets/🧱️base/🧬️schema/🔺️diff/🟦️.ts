import type { PdfAdmittedStreamRole } from "../🪪️stream-roles/🟦️.ts";
/** 🧬️ PdfDiff — TypeScript facet of `s.stdio.pdf.1.7` (diff), generated from the Rust model
 *  by 🐍️generate-schema-facets.py (ticket 26/09/18/PDF-ARTIFACT-SPEC-COMPLETE). */
import type { ObjRef, PdfAcroForm, PdfAction, PdfActionKind, PdfAnnotation, PdfAnnotationKind, PdfAppearance, PdfAppearanceEntry, PdfAppearanceState, PdfBorderStyle, PdfCcittParameters, PdfColorSpace, PdfDate, PdfDecimal, PdfDestination, PdfDestinationFit, PdfDictEntry, PdfEncryption, PdfEncryptionAlgorithm, PdfFileSpecification, PdfFormField, PdfFormFieldKind, PdfFunction, PdfInlineImage, PdfLineCap, PdfLineJoin, PdfMarkInfo, PdfMarkupAnnotation, PdfObject, PdfOp, PdfOpenAction, PdfOptionalContent, PdfOptionalContentGroup, PdfPage, PdfPageLayout, PdfPageMode, PdfPredictor, PdfPropertyList, PdfStreamFilter, PdfTextArrayItem, PdfTextString, PdfTransparencyGroup, PdfViewerPreferences } from '../📸️snapshot/🟦️.ts';
import { registerSchemaDocument, validateAgainst, SchemaRefusal } from '../📸️snapshot/🟦️.ts';

export interface PdfDiff {
  declaredVersion?: string | null;
  pages?: PdfPagesDiff | null;
  fonts?: PdfKeyedDiffPdfFont | null;
  images?: PdfKeyedDiffPdfImage | null;
  forms?: PdfKeyedDiffPdfFormXObject | null;
  extGStates?: PdfKeyedDiffPdfExtGState | null;
  shadings?: PdfKeyedDiffPdfShading | null;
  patterns?: PdfKeyedDiffPdfPattern | null;
  colorSpaces?: PdfKeyedDiffPdfNamedColorSpace | null;
  properties?: PdfKeyedDiffPdfNamedProperties | null;
  outlines?: PdfIndexedDiffPdfOutlineItem | null;
  namedDestinations?: PdfIndexedDiffPdfNamedDestination | null;
  pageLabels?: PdfIndexedDiffPdfPageLabelRange | null;
  embeddedFiles?: PdfKeyedDiffPdfEmbeddedFile | null;
  outputIntents?: PdfIndexedDiffPdfOutputIntent | null;
  acroForm?: PdfSetPdfAcroForm | null;
  optionalContent?: PdfSetPdfOptionalContent | null;
  pageLayout?: PdfSetPdfPageLayout | null;
  pageMode?: PdfSetPdfPageMode | null;
  viewerPreferences?: PdfSetPdfViewerPreferences | null;
  openAction?: PdfSetPdfOpenAction | null;
  language?: PdfSetString | null;
  markInfo?: PdfSetPdfMarkInfo | null;
  metadata?: PdfSetString | null;
  documentId?: PdfSetArrVec_u8x2 | null;
  encryption?: PdfSetPdfEncryption | null;
  info?: PdfInfoDiff | null;
  catalogExtra?: PdfDictDiff | null;
  objects?: PdfObjectsDiff | null;
  trailer?: PdfDictDiff | null;
  graphEdit?: boolean;
  admittedStreamRoles?: { removed?: number[]; modified?: { index: number; value: PdfAdmittedStreamRole }[]; added?: { index: number; value: PdfAdmittedStreamRole }[] } | null;
}

export interface PdfInfoDiff {
  title?: PdfSetString | null;
  author?: PdfSetString | null;
  subject?: PdfSetString | null;
  keywords?: PdfSetString | null;
  creator?: PdfSetString | null;
  producer?: PdfSetString | null;
  creationDate?: PdfSetPdfDate | null;
  modificationDate?: PdfSetPdfDate | null;
  trapped?: PdfSetString | null;
  extra?: PdfDictEntry[] | null;
}

export type PdfSetPdfDate =
  | { kind: "clear" }
  | { kind: "set"; value: PdfDate };

export interface PdfDictDiff {
  removed?: string[];
  modified?: PdfDictModified[];
  added?: PdfDictAdded[];
}

export interface PdfDictAdded {
  index: number;
  key: string;
  item: PdfObject;
}

export interface PdfDictModified {
  key: string;
  diff: PdfValueDiff;
}

export type PdfValueDiff =
  | { kind: "replace"; value: PdfObject }
  | { kind: "bool"; value: boolean }
  | { kind: "int"; value: number }
  | { kind: "real"; value: PdfDecimal }
  | { kind: "str"; value: number[] }
  | { kind: "name"; value: string }
  | { kind: "ref"; value: ObjRef }
  | { kind: "array"; diff: PdfArrayDiff }
  | { kind: "dict"; diff: PdfDictDiff }
  | { kind: "stream"; dict?: PdfDictDiff | null; data?: number[] | null; filters?: PdfStreamFilter[] | null };

export interface PdfArrayDiff {
  removed?: number[];
  modified?: PdfArrayModified[];
  added?: PdfArrayAdded[];
}

export interface PdfArrayAdded {
  index: number;
  item: PdfObject;
}

export interface PdfArrayModified {
  index: number;
  diff: PdfValueDiff;
}

export interface PdfObjectsDiff {
  removed?: ObjRef[];
  modified?: PdfObjectModified[];
  added?: PdfObjectAdded[];
}

export interface PdfObjectAdded {
  index: number;
  id: ObjRef;
  value: PdfObject;
}

export interface PdfObjectModified {
  id: ObjRef;
  diff: PdfValueDiff;
}

export type PdfSetPdfEncryption =
  | { kind: "clear" }
  | { kind: "set"; value: PdfEncryption };

export type PdfSetArrVec_u8x2 =
  | { kind: "clear" }
  | { kind: "set"; value: [number[], number[]] };

export type PdfSetString =
  | { kind: "clear" }
  | { kind: "set"; value: string };

export type PdfSetPdfMarkInfo =
  | { kind: "clear" }
  | { kind: "set"; value: PdfMarkInfo };

export type PdfSetPdfOpenAction =
  | { kind: "clear" }
  | { kind: "set"; value: PdfOpenAction };

export type PdfSetPdfViewerPreferences =
  | { kind: "clear" }
  | { kind: "set"; value: PdfViewerPreferences };

export type PdfSetPdfPageMode =
  | { kind: "clear" }
  | { kind: "set"; value: PdfPageMode };

export type PdfSetPdfPageLayout =
  | { kind: "clear" }
  | { kind: "set"; value: PdfPageLayout };

export type PdfSetPdfOptionalContent =
  | { kind: "clear" }
  | { kind: "set"; value: PdfOptionalContent };

export type PdfSetPdfAcroForm =
  | { kind: "clear" }
  | { kind: "set"; value: PdfAcroForm };

export interface PdfIndexedDiffPdfOutputIntent {
  removed?: number[];
  modified?: PdfIndexedItemT[];
  added?: PdfIndexedItemT[];
}

export interface PdfIndexedItemT {
  index: number;
  value: T;
}

export interface PdfKeyedDiffPdfEmbeddedFile {
  removed?: string[];
  modified?: PdfKeyedItemT[];
  added?: PdfIndexedItemT[];
}

export interface PdfKeyedItemT {
  key: string;
  value: T;
}

export interface PdfIndexedDiffPdfPageLabelRange {
  removed?: number[];
  modified?: PdfIndexedItemT[];
  added?: PdfIndexedItemT[];
}

export interface PdfIndexedDiffPdfNamedDestination {
  removed?: number[];
  modified?: PdfIndexedItemT[];
  added?: PdfIndexedItemT[];
}

export interface PdfIndexedDiffPdfOutlineItem {
  removed?: number[];
  modified?: PdfIndexedItemT[];
  added?: PdfIndexedItemT[];
}

export interface PdfKeyedDiffPdfNamedProperties {
  removed?: string[];
  modified?: PdfKeyedItemT[];
  added?: PdfIndexedItemT[];
}

export interface PdfKeyedDiffPdfNamedColorSpace {
  removed?: string[];
  modified?: PdfKeyedItemT[];
  added?: PdfIndexedItemT[];
}

export interface PdfKeyedDiffPdfPattern {
  removed?: string[];
  modified?: PdfKeyedItemT[];
  added?: PdfIndexedItemT[];
}

export interface PdfKeyedDiffPdfShading {
  removed?: string[];
  modified?: PdfKeyedItemT[];
  added?: PdfIndexedItemT[];
}

export interface PdfKeyedDiffPdfExtGState {
  removed?: string[];
  modified?: PdfKeyedItemT[];
  added?: PdfIndexedItemT[];
}

export interface PdfKeyedDiffPdfFormXObject {
  removed?: string[];
  modified?: PdfKeyedItemT[];
  added?: PdfIndexedItemT[];
}

export interface PdfKeyedDiffPdfImage {
  removed?: string[];
  modified?: PdfKeyedItemT[];
  added?: PdfIndexedItemT[];
}

export interface PdfKeyedDiffPdfFont {
  removed?: string[];
  modified?: PdfKeyedItemT[];
  added?: PdfIndexedItemT[];
}

export interface PdfPagesDiff {
  removed?: number[];
  modified?: PdfPageModified[];
  added?: PdfPageAdded[];
}

export interface PdfPageAdded {
  index: number;
  page: PdfPage;
}

export interface PdfPageModified {
  index: number;
  diff: PdfPageDiff;
}

export interface PdfPageDiff {
  mediaBox?: [number, number, number, number] | null;
  cropBox?: PdfSetPdfRect | null;
  bleedBox?: PdfSetPdfRect | null;
  trimBox?: PdfSetPdfRect | null;
  artBox?: PdfSetPdfRect | null;
  rotate?: number | null;
  userUnit?: PdfSetf64 | null;
  content?: PdfIndexedDiffPdfOp | null;
  annotations?: PdfIndexedDiffPdfAnnotation | null;
  group?: PdfSetPdfTransparencyGroup | null;
  thumbnail?: PdfSetString | null;
  structParents?: PdfSetu32 | null;
  transition?: PdfSetVec_PdfDictEntry | null;
  duration?: PdfSetf64 | null;
  metadata?: PdfSetString | null;
  additionalActions?: PdfDictEntry[] | null;
  extra?: PdfDictDiff | null;
}

export type PdfSetf64 =
  | { kind: "clear" }
  | { kind: "set"; value: number };

export type PdfSetVec_PdfDictEntry =
  | { kind: "clear" }
  | { kind: "set"; value: PdfDictEntry[] };

export type PdfSetu32 =
  | { kind: "clear" }
  | { kind: "set"; value: number };

export type PdfSetPdfTransparencyGroup =
  | { kind: "clear" }
  | { kind: "set"; value: PdfTransparencyGroup };

export interface PdfIndexedDiffPdfAnnotation {
  removed?: number[];
  modified?: PdfIndexedItemT[];
  added?: PdfIndexedItemT[];
}

export interface PdfIndexedDiffPdfOp {
  removed?: number[];
  modified?: PdfIndexedItemT[];
  added?: PdfIndexedItemT[];
}

export type PdfSetPdfRect =
  | { kind: "clear" }
  | { kind: "set"; value: [number, number, number, number] };

/** 🧭️ One step into one object's `PdfObject` tree — the `path` of `SetDictEntry`/`RemoveDictEntry`. */
export type PdfPathSegment =
  | { kind: "arrayIndex"; index: number }
  | { kind: "dictKey"; key: string };

/** 📐️ Which optional page box `SetPageBox` addresses. */
export type PdfPageBox = "crop" | "bleed" | "trim" | "art";

/** 🔣️ The JSON Schema document this facet is validated against. */
export const schema = {
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/diff.json",
  "title": "PdfDiff",
  "type": "object",
  "properties": {
    "declaredVersion": {
      "anyOf": [
        {
          "type": "string"
        },
        {
          "type": "null"
        }
      ]
    },
    "pages": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfPagesDiff"
        },
        {
          "type": "null"
        }
      ]
    },
    "fonts": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfKeyedDiffPdfFont"
        },
        {
          "type": "null"
        }
      ]
    },
    "images": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfKeyedDiffPdfImage"
        },
        {
          "type": "null"
        }
      ]
    },
    "forms": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfKeyedDiffPdfFormXObject"
        },
        {
          "type": "null"
        }
      ]
    },
    "extGStates": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfKeyedDiffPdfExtGState"
        },
        {
          "type": "null"
        }
      ]
    },
    "shadings": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfKeyedDiffPdfShading"
        },
        {
          "type": "null"
        }
      ]
    },
    "patterns": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfKeyedDiffPdfPattern"
        },
        {
          "type": "null"
        }
      ]
    },
    "colorSpaces": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfKeyedDiffPdfNamedColorSpace"
        },
        {
          "type": "null"
        }
      ]
    },
    "properties": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfKeyedDiffPdfNamedProperties"
        },
        {
          "type": "null"
        }
      ]
    },
    "outlines": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfIndexedDiffPdfOutlineItem"
        },
        {
          "type": "null"
        }
      ]
    },
    "namedDestinations": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfIndexedDiffPdfNamedDestination"
        },
        {
          "type": "null"
        }
      ]
    },
    "pageLabels": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfIndexedDiffPdfPageLabelRange"
        },
        {
          "type": "null"
        }
      ]
    },
    "embeddedFiles": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfKeyedDiffPdfEmbeddedFile"
        },
        {
          "type": "null"
        }
      ]
    },
    "outputIntents": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfIndexedDiffPdfOutputIntent"
        },
        {
          "type": "null"
        }
      ]
    },
    "acroForm": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfSetPdfAcroForm"
        },
        {
          "type": "null"
        }
      ]
    },
    "optionalContent": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfSetPdfOptionalContent"
        },
        {
          "type": "null"
        }
      ]
    },
    "pageLayout": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfSetPdfPageLayout"
        },
        {
          "type": "null"
        }
      ]
    },
    "pageMode": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfSetPdfPageMode"
        },
        {
          "type": "null"
        }
      ]
    },
    "viewerPreferences": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfSetPdfViewerPreferences"
        },
        {
          "type": "null"
        }
      ]
    },
    "openAction": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfSetPdfOpenAction"
        },
        {
          "type": "null"
        }
      ]
    },
    "language": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfSetString"
        },
        {
          "type": "null"
        }
      ]
    },
    "markInfo": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfSetPdfMarkInfo"
        },
        {
          "type": "null"
        }
      ]
    },
    "metadata": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfSetString"
        },
        {
          "type": "null"
        }
      ]
    },
    "documentId": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfSetArrVec_u8x2"
        },
        {
          "type": "null"
        }
      ]
    },
    "encryption": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfSetPdfEncryption"
        },
        {
          "type": "null"
        }
      ]
    },
    "info": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfInfoDiff"
        },
        {
          "type": "null"
        }
      ]
    },
    "catalogExtra": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfDictDiff"
        },
        {
          "type": "null"
        }
      ]
    },
    "objects": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfObjectsDiff"
        },
        {
          "type": "null"
        }
      ]
    },
    "trailer": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfDictDiff"
        },
        {
          "type": "null"
        }
      ]
    },
    "graphEdit": {
      "type": "boolean"
    },
    "admittedStreamRoles": {
      "anyOf": [
        {
          "type": "object",
          "properties": {
            "removed": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            },
            "modified": {
              "type": "array",
              "items": {
                "type": "object",
                "properties": {
                  "index": {
                    "type": "integer",
                    "minimum": 0
                  },
                  "value": {
                    "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfAdmittedStreamRole"
                  }
                },
                "required": [
                  "index",
                  "value"
                ],
                "additionalProperties": false
              }
            },
            "added": {
              "type": "array",
              "items": {
                "type": "object",
                "properties": {
                  "index": {
                    "type": "integer",
                    "minimum": 0
                  },
                  "value": {
                    "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfAdmittedStreamRole"
                  }
                },
                "required": [
                  "index",
                  "value"
                ],
                "additionalProperties": false
              }
            }
          },
          "additionalProperties": false
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "$defs": {
    "PdfArrayAdded": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "item": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfObject"
        }
      },
      "required": [
        "index",
        "item"
      ]
    },
    "PdfArrayDiff": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "type": "integer",
            "minimum": 0
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfArrayModified"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfArrayAdded"
          }
        }
      }
    },
    "PdfArrayModified": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "diff": {
          "$ref": "#/$defs/PdfValueDiff"
        }
      },
      "required": [
        "index",
        "diff"
      ]
    },
    "PdfDictAdded": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "key": {
          "type": "string"
        },
        "item": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfObject"
        }
      },
      "required": [
        "index",
        "key",
        "item"
      ]
    },
    "PdfDictDiff": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictModified"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictAdded"
          }
        }
      }
    },
    "PdfDictModified": {
      "type": "object",
      "properties": {
        "key": {
          "type": "string"
        },
        "diff": {
          "$ref": "#/$defs/PdfValueDiff"
        }
      },
      "required": [
        "key",
        "diff"
      ]
    },
    "PdfIndexedDiffPdfAnnotation": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "type": "integer",
            "minimum": 0
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfAnnotation"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfAnnotation"
          }
        }
      }
    },
    "PdfIndexedDiffPdfNamedDestination": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "type": "integer",
            "minimum": 0
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfNamedDestination"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfNamedDestination"
          }
        }
      }
    },
    "PdfIndexedDiffPdfOp": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "type": "integer",
            "minimum": 0
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfOp"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfOp"
          }
        }
      }
    },
    "PdfIndexedDiffPdfOutlineItem": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "type": "integer",
            "minimum": 0
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfOutlineItem"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfOutlineItem"
          }
        }
      }
    },
    "PdfIndexedDiffPdfOutputIntent": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "type": "integer",
            "minimum": 0
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfOutputIntent"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfOutputIntent"
          }
        }
      }
    },
    "PdfIndexedDiffPdfPageLabelRange": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "type": "integer",
            "minimum": 0
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfPageLabelRange"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfPageLabelRange"
          }
        }
      }
    },
    "PdfIndexedItemPdfAnnotation": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfAnnotation"
        }
      },
      "required": [
        "index",
        "value"
      ]
    },
    "PdfIndexedItemPdfEmbeddedFile": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfEmbeddedFile"
        }
      },
      "required": [
        "index",
        "value"
      ]
    },
    "PdfIndexedItemPdfExtGState": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfExtGState"
        }
      },
      "required": [
        "index",
        "value"
      ]
    },
    "PdfIndexedItemPdfFont": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfFont"
        }
      },
      "required": [
        "index",
        "value"
      ]
    },
    "PdfIndexedItemPdfFormXObject": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfFormXObject"
        }
      },
      "required": [
        "index",
        "value"
      ]
    },
    "PdfIndexedItemPdfImage": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfImage"
        }
      },
      "required": [
        "index",
        "value"
      ]
    },
    "PdfIndexedItemPdfNamedColorSpace": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfNamedColorSpace"
        }
      },
      "required": [
        "index",
        "value"
      ]
    },
    "PdfIndexedItemPdfNamedDestination": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfNamedDestination"
        }
      },
      "required": [
        "index",
        "value"
      ]
    },
    "PdfIndexedItemPdfNamedProperties": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfNamedProperties"
        }
      },
      "required": [
        "index",
        "value"
      ]
    },
    "PdfIndexedItemPdfOp": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfOp"
        }
      },
      "required": [
        "index",
        "value"
      ]
    },
    "PdfIndexedItemPdfOutlineItem": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfOutlineItem"
        }
      },
      "required": [
        "index",
        "value"
      ]
    },
    "PdfIndexedItemPdfOutputIntent": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfOutputIntent"
        }
      },
      "required": [
        "index",
        "value"
      ]
    },
    "PdfIndexedItemPdfPageLabelRange": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfPageLabelRange"
        }
      },
      "required": [
        "index",
        "value"
      ]
    },
    "PdfIndexedItemPdfPattern": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfPattern"
        }
      },
      "required": [
        "index",
        "value"
      ]
    },
    "PdfIndexedItemPdfShading": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfShading"
        }
      },
      "required": [
        "index",
        "value"
      ]
    },
    "PdfInfoDiff": {
      "type": "object",
      "properties": {
        "title": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetString"
            },
            {
              "type": "null"
            }
          ]
        },
        "author": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetString"
            },
            {
              "type": "null"
            }
          ]
        },
        "subject": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetString"
            },
            {
              "type": "null"
            }
          ]
        },
        "keywords": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetString"
            },
            {
              "type": "null"
            }
          ]
        },
        "creator": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetString"
            },
            {
              "type": "null"
            }
          ]
        },
        "producer": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetString"
            },
            {
              "type": "null"
            }
          ]
        },
        "creationDate": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetPdfDate"
            },
            {
              "type": "null"
            }
          ]
        },
        "modificationDate": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetPdfDate"
            },
            {
              "type": "null"
            }
          ]
        },
        "trapped": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetString"
            },
            {
              "type": "null"
            }
          ]
        },
        "extra": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfDictEntry"
              }
            },
            {
              "type": "null"
            }
          ]
        }
      }
    },
    "PdfKeyedDiffPdfEmbeddedFile": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfKeyedItemPdfEmbeddedFile"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfEmbeddedFile"
          }
        }
      }
    },
    "PdfKeyedDiffPdfExtGState": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfKeyedItemPdfExtGState"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfExtGState"
          }
        }
      }
    },
    "PdfKeyedDiffPdfFont": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfKeyedItemPdfFont"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfFont"
          }
        }
      }
    },
    "PdfKeyedDiffPdfFormXObject": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfKeyedItemPdfFormXObject"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfFormXObject"
          }
        }
      }
    },
    "PdfKeyedDiffPdfImage": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfKeyedItemPdfImage"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfImage"
          }
        }
      }
    },
    "PdfKeyedDiffPdfNamedColorSpace": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfKeyedItemPdfNamedColorSpace"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfNamedColorSpace"
          }
        }
      }
    },
    "PdfKeyedDiffPdfNamedProperties": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfKeyedItemPdfNamedProperties"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfNamedProperties"
          }
        }
      }
    },
    "PdfKeyedDiffPdfPattern": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfKeyedItemPdfPattern"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfPattern"
          }
        }
      }
    },
    "PdfKeyedDiffPdfShading": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfKeyedItemPdfShading"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfIndexedItemPdfShading"
          }
        }
      }
    },
    "PdfKeyedItemPdfEmbeddedFile": {
      "type": "object",
      "properties": {
        "key": {
          "type": "string"
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfEmbeddedFile"
        }
      },
      "required": [
        "key",
        "value"
      ]
    },
    "PdfKeyedItemPdfExtGState": {
      "type": "object",
      "properties": {
        "key": {
          "type": "string"
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfExtGState"
        }
      },
      "required": [
        "key",
        "value"
      ]
    },
    "PdfKeyedItemPdfFont": {
      "type": "object",
      "properties": {
        "key": {
          "type": "string"
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfFont"
        }
      },
      "required": [
        "key",
        "value"
      ]
    },
    "PdfKeyedItemPdfFormXObject": {
      "type": "object",
      "properties": {
        "key": {
          "type": "string"
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfFormXObject"
        }
      },
      "required": [
        "key",
        "value"
      ]
    },
    "PdfKeyedItemPdfImage": {
      "type": "object",
      "properties": {
        "key": {
          "type": "string"
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfImage"
        }
      },
      "required": [
        "key",
        "value"
      ]
    },
    "PdfKeyedItemPdfNamedColorSpace": {
      "type": "object",
      "properties": {
        "key": {
          "type": "string"
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfNamedColorSpace"
        }
      },
      "required": [
        "key",
        "value"
      ]
    },
    "PdfKeyedItemPdfNamedProperties": {
      "type": "object",
      "properties": {
        "key": {
          "type": "string"
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfNamedProperties"
        }
      },
      "required": [
        "key",
        "value"
      ]
    },
    "PdfKeyedItemPdfPattern": {
      "type": "object",
      "properties": {
        "key": {
          "type": "string"
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfPattern"
        }
      },
      "required": [
        "key",
        "value"
      ]
    },
    "PdfKeyedItemPdfShading": {
      "type": "object",
      "properties": {
        "key": {
          "type": "string"
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfShading"
        }
      },
      "required": [
        "key",
        "value"
      ]
    },
    "PdfObjectAdded": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "id": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/ObjRef"
        },
        "value": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfObject"
        }
      },
      "required": [
        "index",
        "id",
        "value"
      ]
    },
    "PdfObjectModified": {
      "type": "object",
      "properties": {
        "id": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/ObjRef"
        },
        "diff": {
          "$ref": "#/$defs/PdfValueDiff"
        }
      },
      "required": [
        "id",
        "diff"
      ]
    },
    "PdfObjectsDiff": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/ObjRef"
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfObjectModified"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfObjectAdded"
          }
        }
      }
    },
    "PdfPageAdded": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "page": {
          "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfPage"
        }
      },
      "required": [
        "index",
        "page"
      ]
    },
    "PdfPageBox": {
      "type": "string",
      "enum": [
        "crop",
        "bleed",
        "trim",
        "art"
      ]
    },
    "PdfPageDiff": {
      "type": "object",
      "properties": {
        "mediaBox": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "$ref": "https://json.schemas.assets.semio-tech.com/framework/value/schema.json#/$defs/Binary64Transport"
              },
              "minItems": 4,
              "maxItems": 4
            },
            {
              "type": "null"
            }
          ]
        },
        "cropBox": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetPdfRect"
            },
            {
              "type": "null"
            }
          ]
        },
        "bleedBox": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetPdfRect"
            },
            {
              "type": "null"
            }
          ]
        },
        "trimBox": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetPdfRect"
            },
            {
              "type": "null"
            }
          ]
        },
        "artBox": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetPdfRect"
            },
            {
              "type": "null"
            }
          ]
        },
        "rotate": {
          "anyOf": [
            {
              "type": "integer"
            },
            {
              "type": "null"
            }
          ]
        },
        "userUnit": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetf64"
            },
            {
              "type": "null"
            }
          ]
        },
        "content": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfIndexedDiffPdfOp"
            },
            {
              "type": "null"
            }
          ]
        },
        "annotations": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfIndexedDiffPdfAnnotation"
            },
            {
              "type": "null"
            }
          ]
        },
        "group": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetPdfTransparencyGroup"
            },
            {
              "type": "null"
            }
          ]
        },
        "thumbnail": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetString"
            },
            {
              "type": "null"
            }
          ]
        },
        "structParents": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetu32"
            },
            {
              "type": "null"
            }
          ]
        },
        "transition": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetVec_PdfDictEntry"
            },
            {
              "type": "null"
            }
          ]
        },
        "duration": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetf64"
            },
            {
              "type": "null"
            }
          ]
        },
        "metadata": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSetString"
            },
            {
              "type": "null"
            }
          ]
        },
        "additionalActions": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfDictEntry"
              }
            },
            {
              "type": "null"
            }
          ]
        },
        "extra": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfDictDiff"
            },
            {
              "type": "null"
            }
          ]
        }
      }
    },
    "PdfPageModified": {
      "type": "object",
      "properties": {
        "index": {
          "type": "integer",
          "minimum": 0
        },
        "diff": {
          "$ref": "#/$defs/PdfPageDiff"
        }
      },
      "required": [
        "index",
        "diff"
      ]
    },
    "PdfPagesDiff": {
      "type": "object",
      "properties": {
        "removed": {
          "type": "array",
          "items": {
            "type": "integer",
            "minimum": 0
          }
        },
        "modified": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfPageModified"
          }
        },
        "added": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfPageAdded"
          }
        }
      }
    },
    "PdfPathSegment": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "arrayIndex"
            },
            "index": {
              "type": "integer",
              "minimum": 0
            }
          },
          "required": [
            "kind",
            "index"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "dictKey"
            },
            "key": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "key"
          ]
        }
      ]
    },
    "PdfSetArrVec_u8x2": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "clear"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "set"
            },
            "value": {
              "type": "array",
              "items": {
                "type": "array",
                "items": {
                  "type": "integer",
                  "minimum": 0
                }
              },
              "minItems": 2,
              "maxItems": 2
            }
          },
          "required": [
            "kind",
            "value"
          ]
        }
      ]
    },
    "PdfSetPdfAcroForm": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "clear"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "set"
            },
            "value": {
              "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfAcroForm"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        }
      ]
    },
    "PdfSetPdfDate": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "clear"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "set"
            },
            "value": {
              "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfDate"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        }
      ]
    },
    "PdfSetPdfEncryption": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "clear"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "set"
            },
            "value": {
              "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfEncryption"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        }
      ]
    },
    "PdfSetPdfMarkInfo": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "clear"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "set"
            },
            "value": {
              "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfMarkInfo"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        }
      ]
    },
    "PdfSetPdfOpenAction": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "clear"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "set"
            },
            "value": {
              "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfOpenAction"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        }
      ]
    },
    "PdfSetPdfOptionalContent": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "clear"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "set"
            },
            "value": {
              "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfOptionalContent"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        }
      ]
    },
    "PdfSetPdfPageLayout": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "clear"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "set"
            },
            "value": {
              "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfPageLayout"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        }
      ]
    },
    "PdfSetPdfPageMode": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "clear"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "set"
            },
            "value": {
              "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfPageMode"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        }
      ]
    },
    "PdfSetPdfRect": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "clear"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "set"
            },
            "value": {
              "type": "array",
              "items": {
                "$ref": "https://json.schemas.assets.semio-tech.com/framework/value/schema.json#/$defs/Binary64Transport"
              },
              "minItems": 4,
              "maxItems": 4
            }
          },
          "required": [
            "kind",
            "value"
          ]
        }
      ]
    },
    "PdfSetPdfTransparencyGroup": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "clear"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "set"
            },
            "value": {
              "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfTransparencyGroup"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        }
      ]
    },
    "PdfSetPdfViewerPreferences": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "clear"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "set"
            },
            "value": {
              "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfViewerPreferences"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        }
      ]
    },
    "PdfSetString": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "clear"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "set"
            },
            "value": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        }
      ]
    },
    "PdfSetVec_PdfDictEntry": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "clear"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "set"
            },
            "value": {
              "type": "array",
              "items": {
                "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfDictEntry"
              }
            }
          },
          "required": [
            "kind",
            "value"
          ]
        }
      ]
    },
    "PdfSetf64": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "clear"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "set"
            },
            "value": {
              "$ref": "https://json.schemas.assets.semio-tech.com/framework/value/schema.json#/$defs/Binary64Transport"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        }
      ]
    },
    "PdfSetu32": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "clear"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "set"
            },
            "value": {
              "type": "integer",
              "minimum": 0
            }
          },
          "required": [
            "kind",
            "value"
          ]
        }
      ]
    },
    "PdfValueDiff": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "replace"
            },
            "value": {
              "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfObject"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "bool"
            },
            "value": {
              "type": "boolean"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "int"
            },
            "value": {
              "type": "integer"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "real"
            },
            "value": {
              "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfDecimal"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "str"
            },
            "value": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            }
          },
          "required": [
            "kind",
            "value"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "name"
            },
            "value": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "ref"
            },
            "value": {
              "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/ObjRef"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "array"
            },
            "diff": {
              "$ref": "#/$defs/PdfArrayDiff"
            }
          },
          "required": [
            "kind",
            "diff"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "dict"
            },
            "diff": {
              "$ref": "#/$defs/PdfDictDiff"
            }
          },
          "required": [
            "kind",
            "diff"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "stream"
            },
            "dict": {
              "anyOf": [
                {
                  "$ref": "#/$defs/PdfDictDiff"
                },
                {
                  "type": "null"
                }
              ]
            },
            "data": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "integer",
                    "minimum": 0
                  }
                },
                {
                  "type": "null"
                }
              ]
            },
            "filters": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "$ref": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json#/$defs/PdfStreamFilter"
                  }
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        }
      ]
    }
  }
} as const;

export { registerSchemaDocument, validateAgainst, SchemaRefusal };
registerSchemaDocument(schema);
export const parsePdfDiff = (value: unknown): PdfDiff => validateAgainst<PdfDiff>(schema, "", value);
export const parsePdfDictDiff = (value: unknown): PdfDictDiff => validateAgainst<PdfDictDiff>(schema, "/$defs/PdfDictDiff", value);
export const parsePdfDictAdded = (value: unknown): PdfDictAdded => validateAgainst<PdfDictAdded>(schema, "/$defs/PdfDictAdded", value);
export const parsePdfDictModified = (value: unknown): PdfDictModified => validateAgainst<PdfDictModified>(schema, "/$defs/PdfDictModified", value);
export const parsePdfValueDiff = (value: unknown): PdfValueDiff => validateAgainst<PdfValueDiff>(schema, "/$defs/PdfValueDiff", value);
export const parsePdfArrayDiff = (value: unknown): PdfArrayDiff => validateAgainst<PdfArrayDiff>(schema, "/$defs/PdfArrayDiff", value);
export const parsePdfArrayAdded = (value: unknown): PdfArrayAdded => validateAgainst<PdfArrayAdded>(schema, "/$defs/PdfArrayAdded", value);
export const parsePdfArrayModified = (value: unknown): PdfArrayModified => validateAgainst<PdfArrayModified>(schema, "/$defs/PdfArrayModified", value);
export const parsePdfObjectsDiff = (value: unknown): PdfObjectsDiff => validateAgainst<PdfObjectsDiff>(schema, "/$defs/PdfObjectsDiff", value);
export const parsePdfObjectAdded = (value: unknown): PdfObjectAdded => validateAgainst<PdfObjectAdded>(schema, "/$defs/PdfObjectAdded", value);
export const parsePdfObjectModified = (value: unknown): PdfObjectModified => validateAgainst<PdfObjectModified>(schema, "/$defs/PdfObjectModified", value);
export const parsePdfSetPdfEncryption = (value: unknown): PdfSetPdfEncryption => validateAgainst<PdfSetPdfEncryption>(schema, "/$defs/PdfSetPdfEncryption", value);
export const parsePdfSetArrVec_u8x2 = (value: unknown): PdfSetArrVec_u8x2 => validateAgainst<PdfSetArrVec_u8x2>(schema, "/$defs/PdfSetArrVec_u8x2", value);
export const parsePdfSetString = (value: unknown): PdfSetString => validateAgainst<PdfSetString>(schema, "/$defs/PdfSetString", value);
export const parsePdfSetPdfMarkInfo = (value: unknown): PdfSetPdfMarkInfo => validateAgainst<PdfSetPdfMarkInfo>(schema, "/$defs/PdfSetPdfMarkInfo", value);
export const parsePdfSetPdfOpenAction = (value: unknown): PdfSetPdfOpenAction => validateAgainst<PdfSetPdfOpenAction>(schema, "/$defs/PdfSetPdfOpenAction", value);
export const parsePdfSetPdfViewerPreferences = (value: unknown): PdfSetPdfViewerPreferences => validateAgainst<PdfSetPdfViewerPreferences>(schema, "/$defs/PdfSetPdfViewerPreferences", value);
export const parsePdfSetPdfPageMode = (value: unknown): PdfSetPdfPageMode => validateAgainst<PdfSetPdfPageMode>(schema, "/$defs/PdfSetPdfPageMode", value);
export const parsePdfSetPdfPageLayout = (value: unknown): PdfSetPdfPageLayout => validateAgainst<PdfSetPdfPageLayout>(schema, "/$defs/PdfSetPdfPageLayout", value);
export const parsePdfSetPdfOptionalContent = (value: unknown): PdfSetPdfOptionalContent => validateAgainst<PdfSetPdfOptionalContent>(schema, "/$defs/PdfSetPdfOptionalContent", value);
export const parsePdfSetPdfAcroForm = (value: unknown): PdfSetPdfAcroForm => validateAgainst<PdfSetPdfAcroForm>(schema, "/$defs/PdfSetPdfAcroForm", value);
export const parsePdfIndexedDiffPdfOutputIntent = (value: unknown): PdfIndexedDiffPdfOutputIntent => validateAgainst<PdfIndexedDiffPdfOutputIntent>(schema, "/$defs/PdfIndexedDiffPdfOutputIntent", value);
export const parsePdfIndexedItemT = (value: unknown): PdfIndexedItemT => validateAgainst<PdfIndexedItemT>(schema, "/$defs/PdfIndexedItemT", value);
export const parsePdfKeyedDiffPdfEmbeddedFile = (value: unknown): PdfKeyedDiffPdfEmbeddedFile => validateAgainst<PdfKeyedDiffPdfEmbeddedFile>(schema, "/$defs/PdfKeyedDiffPdfEmbeddedFile", value);
export const parsePdfKeyedItemT = (value: unknown): PdfKeyedItemT => validateAgainst<PdfKeyedItemT>(schema, "/$defs/PdfKeyedItemT", value);
export const parsePdfIndexedDiffPdfPageLabelRange = (value: unknown): PdfIndexedDiffPdfPageLabelRange => validateAgainst<PdfIndexedDiffPdfPageLabelRange>(schema, "/$defs/PdfIndexedDiffPdfPageLabelRange", value);
export const parsePdfIndexedDiffPdfNamedDestination = (value: unknown): PdfIndexedDiffPdfNamedDestination => validateAgainst<PdfIndexedDiffPdfNamedDestination>(schema, "/$defs/PdfIndexedDiffPdfNamedDestination", value);
export const parsePdfIndexedDiffPdfOutlineItem = (value: unknown): PdfIndexedDiffPdfOutlineItem => validateAgainst<PdfIndexedDiffPdfOutlineItem>(schema, "/$defs/PdfIndexedDiffPdfOutlineItem", value);
export const parsePdfKeyedDiffPdfNamedProperties = (value: unknown): PdfKeyedDiffPdfNamedProperties => validateAgainst<PdfKeyedDiffPdfNamedProperties>(schema, "/$defs/PdfKeyedDiffPdfNamedProperties", value);
export const parsePdfKeyedDiffPdfNamedColorSpace = (value: unknown): PdfKeyedDiffPdfNamedColorSpace => validateAgainst<PdfKeyedDiffPdfNamedColorSpace>(schema, "/$defs/PdfKeyedDiffPdfNamedColorSpace", value);
export const parsePdfKeyedDiffPdfPattern = (value: unknown): PdfKeyedDiffPdfPattern => validateAgainst<PdfKeyedDiffPdfPattern>(schema, "/$defs/PdfKeyedDiffPdfPattern", value);
export const parsePdfKeyedDiffPdfShading = (value: unknown): PdfKeyedDiffPdfShading => validateAgainst<PdfKeyedDiffPdfShading>(schema, "/$defs/PdfKeyedDiffPdfShading", value);
export const parsePdfKeyedDiffPdfExtGState = (value: unknown): PdfKeyedDiffPdfExtGState => validateAgainst<PdfKeyedDiffPdfExtGState>(schema, "/$defs/PdfKeyedDiffPdfExtGState", value);
export const parsePdfKeyedDiffPdfFormXObject = (value: unknown): PdfKeyedDiffPdfFormXObject => validateAgainst<PdfKeyedDiffPdfFormXObject>(schema, "/$defs/PdfKeyedDiffPdfFormXObject", value);
export const parsePdfKeyedDiffPdfImage = (value: unknown): PdfKeyedDiffPdfImage => validateAgainst<PdfKeyedDiffPdfImage>(schema, "/$defs/PdfKeyedDiffPdfImage", value);
export const parsePdfKeyedDiffPdfFont = (value: unknown): PdfKeyedDiffPdfFont => validateAgainst<PdfKeyedDiffPdfFont>(schema, "/$defs/PdfKeyedDiffPdfFont", value);
export const parsePdfPagesDiff = (value: unknown): PdfPagesDiff => validateAgainst<PdfPagesDiff>(schema, "/$defs/PdfPagesDiff", value);
export const parsePdfPageAdded = (value: unknown): PdfPageAdded => validateAgainst<PdfPageAdded>(schema, "/$defs/PdfPageAdded", value);
export const parsePdfPageModified = (value: unknown): PdfPageModified => validateAgainst<PdfPageModified>(schema, "/$defs/PdfPageModified", value);
export const parsePdfPageDiff = (value: unknown): PdfPageDiff => validateAgainst<PdfPageDiff>(schema, "/$defs/PdfPageDiff", value);
export const parsePdfSetf64 = (value: unknown): PdfSetf64 => validateAgainst<PdfSetf64>(schema, "/$defs/PdfSetf64", value);
export const parsePdfSetVec_PdfDictEntry = (value: unknown): PdfSetVec_PdfDictEntry => validateAgainst<PdfSetVec_PdfDictEntry>(schema, "/$defs/PdfSetVec_PdfDictEntry", value);
export const parsePdfSetu32 = (value: unknown): PdfSetu32 => validateAgainst<PdfSetu32>(schema, "/$defs/PdfSetu32", value);
export const parsePdfSetPdfTransparencyGroup = (value: unknown): PdfSetPdfTransparencyGroup => validateAgainst<PdfSetPdfTransparencyGroup>(schema, "/$defs/PdfSetPdfTransparencyGroup", value);
export const parsePdfIndexedDiffPdfAnnotation = (value: unknown): PdfIndexedDiffPdfAnnotation => validateAgainst<PdfIndexedDiffPdfAnnotation>(schema, "/$defs/PdfIndexedDiffPdfAnnotation", value);
export const parsePdfIndexedDiffPdfOp = (value: unknown): PdfIndexedDiffPdfOp => validateAgainst<PdfIndexedDiffPdfOp>(schema, "/$defs/PdfIndexedDiffPdfOp", value);
export const parsePdfSetPdfRect = (value: unknown): PdfSetPdfRect => validateAgainst<PdfSetPdfRect>(schema, "/$defs/PdfSetPdfRect", value);
export const parsePdfPathSegment = (value: unknown): PdfPathSegment => validateAgainst<PdfPathSegment>(schema, "/$defs/PdfPathSegment", value);
export const parsePdfPageBox = (value: unknown): PdfPageBox => validateAgainst<PdfPageBox>(schema, "/$defs/PdfPageBox", value);
export const parsePdfInfoDiff = (value: unknown): PdfInfoDiff => validateAgainst<PdfInfoDiff>(schema, "/$defs/PdfInfoDiff", value);
export const parsePdfSetPdfDate = (value: unknown): PdfSetPdfDate => validateAgainst<PdfSetPdfDate>(schema, "/$defs/PdfSetPdfDate", value);
