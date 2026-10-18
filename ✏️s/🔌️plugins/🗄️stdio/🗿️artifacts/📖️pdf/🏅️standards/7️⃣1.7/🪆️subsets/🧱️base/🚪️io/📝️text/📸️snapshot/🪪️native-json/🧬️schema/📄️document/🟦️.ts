/** 📄️ JSON Schema definitions of the PDF 1.7 pages and indirect objects. */
export const documentDefinitions = {
  "PdfIndirectObject": {
    "type": "object",
    "properties": {
      "id": {
        "$ref": "#/$defs/ObjRef"
      },
      "value": {
        "$ref": "#/$defs/PdfObject"
      }
    },
    "required": [
      "id",
      "value"
    ]
  },
  "PdfNamedColorSpace": {
    "type": "object",
    "properties": {
      "name": {
        "type": "string"
      },
      "colorSpace": {
        "$ref": "#/$defs/PdfColorSpace"
      }
    },
    "required": [
      "name",
      "colorSpace"
    ]
  },
  "PdfNamedProperties": {
    "type": "object",
    "properties": {
      "name": {
        "type": "string"
      },
      "entries": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfDictEntry"
        }
      }
    },
    "required": [
      "name",
      "entries"
    ]
  },
  "PdfPage": {
    "type": "object",
    "properties": {
      "mediaBox": {
        "type": "array",
        "items": {
          "type": "number"
        },
        "minItems": 4,
        "maxItems": 4
      },
      "cropBox": {
        "anyOf": [
          {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 4,
            "maxItems": 4
          },
          {
            "type": "null"
          }
        ]
      },
      "bleedBox": {
        "anyOf": [
          {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 4,
            "maxItems": 4
          },
          {
            "type": "null"
          }
        ]
      },
      "trimBox": {
        "anyOf": [
          {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 4,
            "maxItems": 4
          },
          {
            "type": "null"
          }
        ]
      },
      "artBox": {
        "anyOf": [
          {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 4,
            "maxItems": 4
          },
          {
            "type": "null"
          }
        ]
      },
      "rotate": {
        "type": "integer"
      },
      "userUnit": {
        "anyOf": [
          {
            "type": "number"
          },
          {
            "type": "null"
          }
        ]
      },
      "content": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfOp"
        }
      },
      "annotations": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfAnnotation"
        }
      },
      "group": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfTransparencyGroup"
          },
          {
            "type": "null"
          }
        ]
      },
      "thumbnail": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "structParents": {
        "anyOf": [
          {
            "type": "integer",
            "minimum": 0
          },
          {
            "type": "null"
          }
        ]
      },
      "transition": {
        "anyOf": [
          {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfDictEntry"
            }
          },
          {
            "type": "null"
          }
        ]
      },
      "duration": {
        "anyOf": [
          {
            "type": "number"
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
      "additionalActions": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfDictEntry"
        }
      },
      "extra": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfDictEntry"
        }
      }
    },
    "required": [
      "mediaBox"
    ]
  }
} as const;
