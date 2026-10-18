/** 🎯️ JSON Schema definitions of the PDF 1.7 actions, destinations, outlines and page labels. */
export const navigationDefinitions = {
  "PdfAction": {
    "type": "object",
    "properties": {
      "kind": {
        "$ref": "#/$defs/PdfActionKind"
      },
      "next": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfAction"
        }
      }
    },
    "required": [
      "kind"
    ]
  },
  "PdfActionKind": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "goTo"
          },
          "destination": {
            "$ref": "#/$defs/PdfDestination"
          }
        },
        "required": [
          "kind",
          "destination"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "goToRemote"
          },
          "file": {
            "$ref": "#/$defs/PdfFileSpecification"
          },
          "destination": {
            "$ref": "#/$defs/PdfDestination"
          },
          "newWindow": {
            "anyOf": [
              {
                "type": "boolean"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "kind",
          "file",
          "destination"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "goToEmbedded"
          },
          "destination": {
            "$ref": "#/$defs/PdfDestination"
          },
          "newWindow": {
            "anyOf": [
              {
                "type": "boolean"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "kind",
          "destination"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "launch"
          },
          "file": {
            "$ref": "#/$defs/PdfFileSpecification"
          },
          "newWindow": {
            "anyOf": [
              {
                "type": "boolean"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "kind",
          "file"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "thread"
          },
          "file": {
            "anyOf": [
              {
                "$ref": "#/$defs/PdfFileSpecification"
              },
              {
                "type": "null"
              }
            ]
          },
          "thread": {
            "type": "integer",
            "minimum": 0
          }
        },
        "required": [
          "kind",
          "thread"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "uri"
          },
          "uri": {
            "type": "string"
          },
          "isMap": {
            "type": "boolean"
          }
        },
        "required": [
          "kind",
          "uri",
          "isMap"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "sound"
          },
          "sound": {
            "type": "string"
          },
          "volume": {
            "anyOf": [
              {
                "type": "number"
              },
              {
                "type": "null"
              }
            ]
          },
          "synchronous": {
            "type": "boolean"
          },
          "repeat": {
            "type": "boolean"
          },
          "mix": {
            "type": "boolean"
          }
        },
        "required": [
          "kind",
          "sound",
          "synchronous",
          "repeat",
          "mix"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "movie"
          },
          "annotation": {
            "anyOf": [
              {
                "type": "string"
              },
              {
                "type": "null"
              }
            ]
          },
          "operation": {
            "anyOf": [
              {
                "type": "string"
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
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "hide"
          },
          "annotations": {
            "type": "array",
            "items": {
              "type": "string"
            }
          },
          "hide": {
            "type": "boolean"
          }
        },
        "required": [
          "kind",
          "annotations",
          "hide"
        ]
      },
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
            "const": "submitForm"
          },
          "url": {
            "type": "string"
          },
          "fields": {
            "type": "array",
            "items": {
              "type": "string"
            }
          },
          "flags": {
            "type": "integer",
            "minimum": 0
          }
        },
        "required": [
          "kind",
          "url",
          "fields",
          "flags"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "resetForm"
          },
          "fields": {
            "type": "array",
            "items": {
              "type": "string"
            }
          },
          "flags": {
            "type": "integer",
            "minimum": 0
          }
        },
        "required": [
          "kind",
          "fields",
          "flags"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "importData"
          },
          "file": {
            "$ref": "#/$defs/PdfFileSpecification"
          }
        },
        "required": [
          "kind",
          "file"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "javaScript"
          },
          "script": {
            "type": "string"
          }
        },
        "required": [
          "kind",
          "script"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "setOptionalContentState"
          },
          "states": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfDictEntry"
            }
          },
          "preserveRadioButtons": {
            "type": "boolean"
          }
        },
        "required": [
          "kind",
          "states",
          "preserveRadioButtons"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "rendition"
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
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "transition"
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
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "goTo3dView"
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
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "unknown"
          },
          "subtype": {
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
          "kind",
          "subtype",
          "entries"
        ]
      }
    ]
  },
  "PdfDestination": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "page"
          },
          "page": {
            "type": "integer",
            "minimum": 0
          },
          "fit": {
            "$ref": "#/$defs/PdfDestinationFit"
          }
        },
        "required": [
          "kind",
          "page",
          "fit"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "remotePage"
          },
          "page": {
            "type": "integer",
            "minimum": 0
          },
          "fit": {
            "$ref": "#/$defs/PdfDestinationFit"
          }
        },
        "required": [
          "kind",
          "page",
          "fit"
        ]
      },
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
      }
    ]
  },
  "PdfDestinationFit": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "xyz"
          },
          "left": {
            "anyOf": [
              {
                "type": "number"
              },
              {
                "type": "null"
              }
            ]
          },
          "top": {
            "anyOf": [
              {
                "type": "number"
              },
              {
                "type": "null"
              }
            ]
          },
          "zoom": {
            "anyOf": [
              {
                "type": "number"
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
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "fit"
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
            "const": "fitHorizontal"
          },
          "top": {
            "anyOf": [
              {
                "type": "number"
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
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "fitVertical"
          },
          "left": {
            "anyOf": [
              {
                "type": "number"
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
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "fitRectangle"
          },
          "rect": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 4,
            "maxItems": 4
          }
        },
        "required": [
          "kind",
          "rect"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "fitBoundingBox"
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
            "const": "fitBoundingBoxHorizontal"
          },
          "top": {
            "anyOf": [
              {
                "type": "number"
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
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "fitBoundingBoxVertical"
          },
          "left": {
            "anyOf": [
              {
                "type": "number"
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
  },
  "PdfFileSpecification": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "path"
          },
          "path": {
            "type": "string"
          }
        },
        "required": [
          "kind",
          "path"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "embedded"
          },
          "file": {
            "type": "string"
          }
        },
        "required": [
          "kind",
          "file"
        ]
      }
    ]
  },
  "PdfNamedDestination": {
    "type": "object",
    "properties": {
      "name": {
        "type": "string"
      },
      "destination": {
        "$ref": "#/$defs/PdfDestination"
      }
    },
    "required": [
      "name",
      "destination"
    ]
  },
  "PdfOpenAction": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "destination"
          },
          "destination": {
            "$ref": "#/$defs/PdfDestination"
          }
        },
        "required": [
          "kind",
          "destination"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "action"
          },
          "action": {
            "$ref": "#/$defs/PdfAction"
          }
        },
        "required": [
          "kind",
          "action"
        ]
      }
    ]
  },
  "PdfOutlineItem": {
    "type": "object",
    "properties": {
      "title": {
        "type": "string"
      },
      "destination": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfDestination"
          },
          {
            "type": "null"
          }
        ]
      },
      "action": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfAction"
          },
          {
            "type": "null"
          }
        ]
      },
      "color": {
        "anyOf": [
          {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 3,
            "maxItems": 3
          },
          {
            "type": "null"
          }
        ]
      },
      "italic": {
        "type": "boolean"
      },
      "bold": {
        "type": "boolean"
      },
      "open": {
        "type": "boolean"
      },
      "children": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfOutlineItem"
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
      "title"
    ]
  },
  "PdfPageLabelRange": {
    "type": "object",
    "properties": {
      "startIndex": {
        "type": "integer",
        "minimum": 0
      },
      "style": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfPageLabelStyle"
          },
          {
            "type": "null"
          }
        ]
      },
      "prefix": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "start": {
        "type": "integer",
        "minimum": 0
      }
    },
    "required": [
      "startIndex"
    ]
  },
  "PdfPageLabelStyle": {
    "type": "string",
    "enum": [
      "decimal",
      "romanUpper",
      "romanLower",
      "lettersUpper",
      "lettersLower"
    ]
  }
} as const;
