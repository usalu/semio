/** 📌️ JSON Schema definitions of the PDF 1.7 annotations and appearances. */
export const annotationDefinitions = {
  "PdfAnnotation": {
    "type": "object",
    "properties": {
      "rect": {
        "type": "array",
        "items": {
          "type": "number"
        },
        "minItems": 4,
        "maxItems": 4
      },
      "kind": {
        "$ref": "#/$defs/PdfAnnotationKind"
      },
      "contents": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "name": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "modified": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "flags": {
        "type": "integer",
        "minimum": 0
      },
      "border": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfBorderStyle"
          },
          {
            "type": "null"
          }
        ]
      },
      "color": {
        "type": "array",
        "items": {
          "type": "number"
        }
      },
      "appearance": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfAppearance"
          },
          {
            "type": "null"
          }
        ]
      },
      "appearanceState": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "markup": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfMarkupAnnotation"
          },
          {
            "type": "null"
          }
        ]
      },
      "optionalContent": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "structParent": {
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
      "extra": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfDictEntry"
        }
      }
    },
    "required": [
      "rect",
      "kind"
    ]
  },
  "PdfAppearance": {
    "type": "object",
    "properties": {
      "normal": {
        "$ref": "#/$defs/PdfAppearanceEntry"
      },
      "rollover": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfAppearanceEntry"
          },
          {
            "type": "null"
          }
        ]
      },
      "down": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfAppearanceEntry"
          },
          {
            "type": "null"
          }
        ]
      }
    },
    "required": [
      "normal"
    ]
  },
  "PdfAppearanceEntry": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "single"
          },
          "form": {
            "type": "string"
          }
        },
        "required": [
          "kind",
          "form"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "states"
          },
          "states": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfAppearanceState"
            }
          }
        },
        "required": [
          "kind",
          "states"
        ]
      }
    ]
  },
  "PdfAppearanceState": {
    "type": "object",
    "properties": {
      "state": {
        "type": "string"
      },
      "form": {
        "type": "string"
      }
    },
    "required": [
      "state",
      "form"
    ]
  },
  "PdfBorderStyle": {
    "type": "object",
    "properties": {
      "width": {
        "type": "number"
      },
      "style": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "dash": {
        "anyOf": [
          {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          {
            "type": "null"
          }
        ]
      },
      "radii": {
        "anyOf": [
          {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 2,
            "maxItems": 2
          },
          {
            "type": "null"
          }
        ]
      }
    },
    "required": [
      "width"
    ]
  },
  "PdfMarkupAnnotation": {
    "type": "object",
    "properties": {
      "title": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "popup": {
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
      "opacity": {
        "anyOf": [
          {
            "type": "number"
          },
          {
            "type": "null"
          }
        ]
      },
      "richContents": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "creationDate": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfDate"
          },
          {
            "type": "null"
          }
        ]
      },
      "inReplyTo": {
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
      "subject": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "replyType": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "intent": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      }
    }
  }
} as const;
