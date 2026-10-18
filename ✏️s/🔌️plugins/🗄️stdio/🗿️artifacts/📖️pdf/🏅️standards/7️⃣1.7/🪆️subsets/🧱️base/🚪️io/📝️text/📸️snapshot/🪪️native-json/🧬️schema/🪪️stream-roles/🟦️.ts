/** 🪪️ JSON Schema definitions of admitted stream roles. */
export const streamRoleDefinitions = {
  "ArtifactRef": {
    "type": "object",
    "additionalProperties": false,
    "required": [
      "artifactId",
      "dialect"
    ],
    "properties": {
      "artifactId": {
        "type": "string"
      },
      "dialect": {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "artifactKind",
          "standard",
          "subset"
        ],
        "properties": {
          "artifactKind": {
            "type": "string"
          },
          "standard": {
            "type": "string"
          },
          "subset": {
            "type": "string"
          }
        }
      }
    }
  },
  "PdfStreamRoleValue": {
    "oneOf": [
      {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "kind",
          "content"
        ],
        "properties": {
          "kind": {
            "const": "operators"
          },
          "content": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfOp"
            }
          }
        }
      },
      {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "kind",
          "samples"
        ],
        "properties": {
          "kind": {
            "const": "sampledWords"
          },
          "samples": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 4294967295
            }
          }
        }
      },
      {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "kind",
          "code"
        ],
        "properties": {
          "kind": {
            "const": "calculatorProgram"
          },
          "code": {
            "type": "string"
          }
        }
      },
      {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "kind",
          "mapping"
        ],
        "properties": {
          "kind": {
            "const": "unicodeMap"
          },
          "mapping": {
            "$ref": "#/$defs/PdfToUnicode"
          }
        }
      },
      {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "kind",
          "cmap"
        ],
        "properties": {
          "kind": {
            "const": "characterMap"
          },
          "cmap": {
            "$ref": "#/$defs/PdfEmbeddedCMap"
          }
        }
      },
      {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "kind",
          "program"
        ],
        "properties": {
          "kind": {
            "const": "fontProgram"
          },
          "program": {
            "$ref": "#/$defs/PdfFontProgram"
          }
        }
      },
      {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "kind",
          "image"
        ],
        "properties": {
          "kind": {
            "const": "image"
          },
          "image": {
            "$ref": "#/$defs/PdfImage"
          }
        }
      },
      {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "kind",
          "text"
        ],
        "properties": {
          "kind": {
            "const": "metadataText"
          },
          "text": {
            "type": "string"
          }
        }
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "attachmentBytes"
          },
          "bytes": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 255
            }
          }
        },
        "required": [
          "kind",
          "bytes"
        ],
        "additionalProperties": false
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "paletteComponents"
          },
          "components": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 255
            }
          }
        },
        "required": [
          "kind",
          "components"
        ],
        "additionalProperties": false
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "glyphIds"
          },
          "glyphs": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 65535
            }
          }
        },
        "required": [
          "kind",
          "glyphs"
        ],
        "additionalProperties": false
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "referenceBody"
          },
          "reference": {
            "$ref": "#/$defs/ArtifactRef"
          }
        },
        "required": [
          "kind",
          "reference"
        ],
        "additionalProperties": false
      }
    ]
  }
} as const;
