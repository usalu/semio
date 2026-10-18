/** 🎚️ JSON Schema definitions of the PDF 1.7 graphics state, soft masks and form XObjects. */
export const stateDefinitions = {
  "PdfExtGState": {
    "type": "object",
    "properties": {
      "id": {
        "type": "string"
      },
      "lineWidth": {
        "anyOf": [
          {
            "type": "number"
          },
          {
            "type": "null"
          }
        ]
      },
      "lineCap": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfLineCap"
          },
          {
            "type": "null"
          }
        ]
      },
      "lineJoin": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfLineJoin"
          },
          {
            "type": "null"
          }
        ]
      },
      "miterLimit": {
        "anyOf": [
          {
            "type": "number"
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
            "items": [
              {
                "type": "array",
                "items": {
                  "type": "number"
                }
              },
              {
                "type": "number"
              }
            ],
            "minItems": 2,
            "maxItems": 2
          },
          {
            "type": "null"
          }
        ]
      },
      "renderingIntent": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "overprintStroke": {
        "anyOf": [
          {
            "type": "boolean"
          },
          {
            "type": "null"
          }
        ]
      },
      "overprintFill": {
        "anyOf": [
          {
            "type": "boolean"
          },
          {
            "type": "null"
          }
        ]
      },
      "overprintMode": {
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
      "font": {
        "anyOf": [
          {
            "type": "array",
            "items": [
              {
                "type": "string"
              },
              {
                "type": "number"
              }
            ],
            "minItems": 2,
            "maxItems": 2
          },
          {
            "type": "null"
          }
        ]
      },
      "blendMode": {
        "anyOf": [
          {
            "type": "array",
            "items": {
              "type": "string"
            }
          },
          {
            "type": "null"
          }
        ]
      },
      "softMask": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfSoftMask"
          },
          {
            "type": "null"
          }
        ]
      },
      "strokeAlpha": {
        "anyOf": [
          {
            "type": "number"
          },
          {
            "type": "null"
          }
        ]
      },
      "fillAlpha": {
        "anyOf": [
          {
            "type": "number"
          },
          {
            "type": "null"
          }
        ]
      },
      "alphaIsShape": {
        "anyOf": [
          {
            "type": "boolean"
          },
          {
            "type": "null"
          }
        ]
      },
      "strokeAdjust": {
        "anyOf": [
          {
            "type": "boolean"
          },
          {
            "type": "null"
          }
        ]
      },
      "flatness": {
        "anyOf": [
          {
            "type": "number"
          },
          {
            "type": "null"
          }
        ]
      },
      "smoothness": {
        "anyOf": [
          {
            "type": "number"
          },
          {
            "type": "null"
          }
        ]
      },
      "textKnockout": {
        "anyOf": [
          {
            "type": "boolean"
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
      "id"
    ]
  },
  "PdfFormXObject": {
    "type": "object",
    "properties": {
      "id": {
        "type": "string"
      },
      "bbox": {
        "type": "array",
        "items": {
          "type": "number"
        },
        "minItems": 4,
        "maxItems": 4
      },
      "matrix": {
        "type": "array",
        "items": {
          "type": "number"
        },
        "minItems": 6,
        "maxItems": 6
      },
      "content": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfOp"
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
      "id",
      "bbox"
    ]
  },
  "PdfLineCap": {
    "type": "string",
    "enum": [
      "butt",
      "round",
      "square"
    ]
  },
  "PdfLineJoin": {
    "type": "string",
    "enum": [
      "miter",
      "round",
      "bevel"
    ]
  },
  "PdfSoftMask": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "none"
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
            "const": "alpha"
          },
          "group": {
            "type": "string"
          },
          "transfer": {
            "anyOf": [
              {
                "$ref": "#/$defs/PdfFunction"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "kind",
          "group"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "luminosity"
          },
          "group": {
            "type": "string"
          },
          "backdrop": {
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
          "transfer": {
            "anyOf": [
              {
                "$ref": "#/$defs/PdfFunction"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "kind",
          "group"
        ]
      }
    ]
  },
  "PdfTransparencyGroup": {
    "type": "object",
    "properties": {
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
      "isolated": {
        "type": "boolean"
      },
      "knockout": {
        "type": "boolean"
      }
    }
  }
} as const;
