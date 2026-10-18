/** 🖋️ JSON Schema definitions of the PDF 1.7 content stream operands and text. */
import { pathOperatorVariants } from "./🛤️path/🟦️.ts";
import { textOperatorVariants } from "./🔤️text/🟦️.ts";
import { paintOperatorVariants } from "./🎨️paint/🟦️.ts";

export const contentDefinitions = {
  "PdfInlineImage": {
    "type": "object",
    "properties": {
      "width": {
        "type": "integer",
        "minimum": 0
      },
      "height": {
        "type": "integer",
        "minimum": 0
      },
      "bitsPerComponent": {
        "type": "integer",
        "minimum": 0
      },
      "colorSpace": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfColorSpace"
          },
          {
            "type": "null"
          }
        ]
      },
      "imageMask": {
        "type": "boolean"
      },
      "decode": {
        "type": "array",
        "items": {
          "type": "number"
        }
      },
      "interpolate": {
        "type": "boolean"
      },
      "extra": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfDictEntry"
        }
      },
      "body": {
        "$ref": "#/$defs/PdfImageBody"
      }
    },
    "required": [
      "width",
      "height",
      "body"
    ]
  },
  "PdfOp": {
    "oneOf": [
      ...pathOperatorVariants,
      ...textOperatorVariants,
      ...paintOperatorVariants
    ]
  },
  "PdfPropertyList": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "named"
          },
          "name": {
            "type": "string"
          }
        },
        "required": [
          "kind",
          "name"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "inline"
          },
          "entries": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfDictEntry"
            }
          }
        },
        "required": [
          "kind",
          "entries"
        ]
      }
    ]
  },
  "PdfTextArrayItem": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "text"
          },
          "text": {
            "type": "string"
          }
        },
        "required": [
          "kind",
          "text"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "codes"
          },
          "bytes": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0
            }
          }
        },
        "required": [
          "kind",
          "bytes"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "adjust"
          },
          "amount": {
            "type": "number"
          }
        },
        "required": [
          "kind",
          "amount"
        ]
      }
    ]
  },
  "PdfTextString": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "text"
          },
          "text": {
            "type": "string"
          }
        },
        "required": [
          "kind",
          "text"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "codes"
          },
          "bytes": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0
            }
          }
        },
        "required": [
          "kind",
          "bytes"
        ]
      }
    ]
  }
} as const;
