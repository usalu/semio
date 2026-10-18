/** 🏷️ JSON Schema definitions of the PDF 1.7 annotation kinds. */
export const annotationKindDefinitions = {
  "PdfAnnotationKind": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "text"
          },
          "open": {
            "type": "boolean"
          },
          "icon": {
            "anyOf": [
              {
                "type": "string"
              },
              {
                "type": "null"
              }
            ]
          },
          "state": {
            "anyOf": [
              {
                "type": "string"
              },
              {
                "type": "null"
              }
            ]
          },
          "stateModel": {
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
          "kind",
          "open"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "link"
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
          "highlight": {
            "anyOf": [
              {
                "type": "string"
              },
              {
                "type": "null"
              }
            ]
          },
          "quadPoints": {
            "type": "array",
            "items": {
              "type": "number"
            }
          }
        },
        "required": [
          "kind",
          "quadPoints"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "freeText"
          },
          "defaultAppearance": {
            "type": "string"
          },
          "quadding": {
            "type": "integer",
            "minimum": 0
          },
          "callout": {
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
          "lineEnding": {
            "anyOf": [
              {
                "type": "string"
              },
              {
                "type": "null"
              }
            ]
          },
          "richText": {
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
          "kind",
          "defaultAppearance",
          "quadding"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "line"
          },
          "points": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 4,
            "maxItems": 4
          },
          "lineEndings": {
            "anyOf": [
              {
                "type": "array",
                "items": {
                  "type": "string"
                },
                "minItems": 2,
                "maxItems": 2
              },
              {
                "type": "null"
              }
            ]
          },
          "interiorColor": {
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
          "leaderLength": {
            "anyOf": [
              {
                "type": "number"
              },
              {
                "type": "null"
              }
            ]
          },
          "caption": {
            "type": "boolean"
          }
        },
        "required": [
          "kind",
          "points",
          "caption"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "square"
          },
          "interiorColor": {
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
          "rectDifferences": {
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
            "const": "circle"
          },
          "interiorColor": {
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
          "rectDifferences": {
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
            "const": "polygon"
          },
          "vertices": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "interiorColor": {
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
          }
        },
        "required": [
          "kind",
          "vertices"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "polyLine"
          },
          "vertices": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "lineEndings": {
            "anyOf": [
              {
                "type": "array",
                "items": {
                  "type": "string"
                },
                "minItems": 2,
                "maxItems": 2
              },
              {
                "type": "null"
              }
            ]
          },
          "interiorColor": {
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
          }
        },
        "required": [
          "kind",
          "vertices"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "highlight"
          },
          "quadPoints": {
            "type": "array",
            "items": {
              "type": "number"
            }
          }
        },
        "required": [
          "kind",
          "quadPoints"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "underline"
          },
          "quadPoints": {
            "type": "array",
            "items": {
              "type": "number"
            }
          }
        },
        "required": [
          "kind",
          "quadPoints"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "squiggly"
          },
          "quadPoints": {
            "type": "array",
            "items": {
              "type": "number"
            }
          }
        },
        "required": [
          "kind",
          "quadPoints"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "strikeOut"
          },
          "quadPoints": {
            "type": "array",
            "items": {
              "type": "number"
            }
          }
        },
        "required": [
          "kind",
          "quadPoints"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "stamp"
          },
          "icon": {
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
            "const": "caret"
          },
          "rectDifferences": {
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
          "symbol": {
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
            "const": "ink"
          },
          "paths": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "number"
              }
            }
          }
        },
        "required": [
          "kind",
          "paths"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "popup"
          },
          "parent": {
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
          "open": {
            "type": "boolean"
          }
        },
        "required": [
          "kind",
          "open"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "fileAttachment"
          },
          "file": {
            "$ref": "#/$defs/PdfFileSpecification"
          },
          "icon": {
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
          "kind",
          "file"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "sound"
          },
          "sound": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfDictEntry"
            }
          },
          "icon": {
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
          "kind",
          "sound"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "movie"
          },
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
          "movie": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfDictEntry"
            }
          },
          "activation": {
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
          }
        },
        "required": [
          "kind",
          "movie"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "widget"
          },
          "field": {
            "anyOf": [
              {
                "type": "string"
              },
              {
                "type": "null"
              }
            ]
          },
          "highlight": {
            "anyOf": [
              {
                "type": "string"
              },
              {
                "type": "null"
              }
            ]
          },
          "characteristics": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfDictEntry"
            }
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
          "additionalActions": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfDictEntry"
            }
          }
        },
        "required": [
          "kind",
          "characteristics",
          "additionalActions"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "screen"
          },
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
          "characteristics": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfDictEntry"
            }
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
          "additionalActions": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfDictEntry"
            }
          }
        },
        "required": [
          "kind",
          "characteristics",
          "additionalActions"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "printerMark"
          },
          "markStyle": {
            "anyOf": [
              {
                "type": "string"
              },
              {
                "type": "null"
              }
            ]
          },
          "colorants": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfDictEntry"
            }
          }
        },
        "required": [
          "kind",
          "colorants"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "trapNet"
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
            "const": "watermark"
          },
          "fixedPrint": {
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
            "const": "threeD"
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
            "const": "redact"
          },
          "quadPoints": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "interiorColor": {
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
          "overlayText": {
            "anyOf": [
              {
                "type": "string"
              },
              {
                "type": "null"
              }
            ]
          },
          "repeat": {
            "type": "boolean"
          },
          "defaultAppearance": {
            "anyOf": [
              {
                "type": "string"
              },
              {
                "type": "null"
              }
            ]
          },
          "quadding": {
            "type": "integer",
            "minimum": 0
          }
        },
        "required": [
          "kind",
          "quadPoints",
          "repeat",
          "quadding"
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
  }
} as const;
