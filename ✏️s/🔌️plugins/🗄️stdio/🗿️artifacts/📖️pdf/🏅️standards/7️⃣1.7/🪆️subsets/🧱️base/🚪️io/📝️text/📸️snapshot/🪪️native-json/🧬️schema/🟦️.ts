/** 🔣️ PDF 1.7 native JSON transport schema. */
import { cosDefinitions } from "./🧩️cos/🟦️.ts";
import { documentDefinitions } from "./📄️document/🟦️.ts";
import { metadataDefinitions } from "./📇️metadata/🟦️.ts";
import { navigationDefinitions } from "./🎯️navigation/🟦️.ts";
import { formDefinitions } from "./📝️form/🟦️.ts";
import { colorDefinitions } from "./🌈️color/🟦️.ts";
import { contentDefinitions } from "./🖋️content/🟦️.ts";
import { imageDefinitions } from "./🖼️resource/🟦️.ts";
import { shadingDefinitions } from "./🖼️resource/🌈️shading/🟦️.ts";
import { stateDefinitions } from "./🖼️resource/🎚️state/🟦️.ts";
import { fontDefinitions } from "./🔤️font/🟦️.ts";
import { annotationDefinitions } from "./📌️annotation/🟦️.ts";
import { annotationKindDefinitions } from "./📌️annotation/🏷️kind/🟦️.ts";
import { streamRoleDefinitions } from "./🪪️stream-roles/🟦️.ts";

export const schema={
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json",
  "title": "PdfSnapshot",
  "type": "object",
  "properties": {
    "schema": {
      "type": "string"
    },
    "declaredVersion": {
      "type": "string"
    },
    "pages": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfPage"
      }
    },
    "fonts": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfFont"
      }
    },
    "images": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfImage"
      }
    },
    "forms": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfFormXObject"
      }
    },
    "extGStates": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfExtGState"
      }
    },
    "shadings": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfShading"
      }
    },
    "patterns": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfPattern"
      }
    },
    "colorSpaces": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfNamedColorSpace"
      }
    },
    "properties": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfNamedProperties"
      }
    },
    "outlines": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfOutlineItem"
      }
    },
    "namedDestinations": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfNamedDestination"
      }
    },
    "pageLabels": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfPageLabelRange"
      }
    },
    "embeddedFiles": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfEmbeddedFile"
      }
    },
    "outputIntents": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfOutputIntent"
      }
    },
    "acroForm": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfAcroForm"
        },
        {
          "type": "null"
        }
      ]
    },
    "optionalContent": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfOptionalContent"
        },
        {
          "type": "null"
        }
      ]
    },
    "pageLayout": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfPageLayout"
        },
        {
          "type": "null"
        }
      ]
    },
    "pageMode": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfPageMode"
        },
        {
          "type": "null"
        }
      ]
    },
    "viewerPreferences": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfViewerPreferences"
        },
        {
          "type": "null"
        }
      ]
    },
    "openAction": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfOpenAction"
        },
        {
          "type": "null"
        }
      ]
    },
    "language": {
      "anyOf": [
        {
          "type": "string"
        },
        {
          "type": "null"
        }
      ]
    },
    "markInfo": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfMarkInfo"
        },
        {
          "type": "null"
        }
      ]
    },
    "metadata": {
      "anyOf": [
        {
          "type": "string"
        },
        {
          "type": "null"
        }
      ]
    },
    "documentId": {
      "anyOf": [
        {
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
        },
        {
          "type": "null"
        }
      ]
    },
    "encryption": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfEncryption"
        },
        {
          "type": "null"
        }
      ]
    },
    "info": {
      "$ref": "#/$defs/PdfInfo"
    },
    "catalogExtra": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfDictEntry"
      }
    },
    "objects": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfIndirectObject"
      }
    },
    "trailer": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfDictEntry"
      }
    }
  },
  "required": [
    "schema",
    "declaredVersion",
    "pages",
    "fonts",
    "images",
    "forms",
    "extGStates",
    "shadings",
    "patterns",
    "colorSpaces",
    "properties",
    "outlines",
    "namedDestinations",
    "pageLabels",
    "embeddedFiles",
    "outputIntents",
    "info",
    "catalogExtra",
    "objects",
    "trailer"
  ],
  "$defs": {
    ...cosDefinitions,
    ...documentDefinitions,
    ...metadataDefinitions,
    ...navigationDefinitions,
    ...formDefinitions,
    ...colorDefinitions,
    ...contentDefinitions,
    ...imageDefinitions,
    ...shadingDefinitions,
    ...stateDefinitions,
    ...fontDefinitions,
    ...annotationDefinitions,
    ...annotationKindDefinitions,
    ...streamRoleDefinitions
  }
} as const;
