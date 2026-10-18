/** 🌅️ JSON Schema definitions of the PDF 1.7 shadings and patterns. */
export const shadingDefinitions = {
  "PdfPattern": {
    "type": "object",
    "properties": {
      "id": {
        "type": "string"
      },
      "matrix": {
        "type": "array",
        "items": {
          "type": "number"
        },
        "minItems": 6,
        "maxItems": 6
      },
      "kind": {
        "$ref": "#/$defs/PdfPatternKind"
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
      "kind"
    ]
  },
  "PdfPatternKind": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "tiling"
          },
          "paintType": {
            "type": "integer",
            "minimum": 0
          },
          "tilingType": {
            "type": "integer",
            "minimum": 0
          },
          "bbox": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 4,
            "maxItems": 4
          },
          "xStep": {
            "type": "number"
          },
          "yStep": {
            "type": "number"
          },
          "content": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfOp"
            }
          }
        },
        "required": [
          "kind",
          "paintType",
          "tilingType",
          "bbox",
          "xStep",
          "yStep",
          "content"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "shading"
          },
          "shading": {
            "type": "string"
          },
          "extGState": {
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
          "shading"
        ]
      }
    ]
  },
  "PdfShading": {
    "type": "object",
    "properties": {
      "id": {
        "type": "string"
      },
      "colorSpace": {
        "$ref": "#/$defs/PdfColorSpace"
      },
      "kind": {
        "$ref": "#/$defs/PdfShadingKind"
      },
      "background": {
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
      "bbox": {
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
      "antiAlias": {
        "type": "boolean"
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
      "colorSpace",
      "kind"
    ]
  },
  "PdfShadingKind": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "functionBased"
          },
          "domain": {
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
          "matrix": {
            "anyOf": [
              {
                "type": "array",
                "items": {
                  "type": "number"
                },
                "minItems": 6,
                "maxItems": 6
              },
              {
                "type": "null"
              }
            ]
          },
          "function": {
            "$ref": "#/$defs/PdfFunction"
          }
        },
        "required": [
          "kind",
          "function"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "axial"
          },
          "coords": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 4,
            "maxItems": 4
          },
          "domain": {
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
          },
          "function": {
            "$ref": "#/$defs/PdfFunction"
          },
          "extend": {
            "type": "array",
            "items": {
              "type": "boolean"
            },
            "minItems": 2,
            "maxItems": 2
          }
        },
        "required": [
          "kind",
          "coords",
          "function",
          "extend"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "radial"
          },
          "coords": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 6,
            "maxItems": 6
          },
          "domain": {
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
          },
          "function": {
            "$ref": "#/$defs/PdfFunction"
          },
          "extend": {
            "type": "array",
            "items": {
              "type": "boolean"
            },
            "minItems": 2,
            "maxItems": 2
          }
        },
        "required": [
          "kind",
          "coords",
          "function",
          "extend"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "mesh"
          },
          "shadingType": {
            "type": "integer",
            "minimum": 0
          },
          "bitsPerCoordinate": {
            "type": "integer",
            "minimum": 0
          },
          "bitsPerComponent": {
            "type": "integer",
            "minimum": 0
          },
          "bitsPerFlag": {
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
          "verticesPerRow": {
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
          "decode": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "function": {
            "anyOf": [
              {
                "$ref": "#/$defs/PdfFunction"
              },
              {
                "type": "null"
              }
            ]
          },
          "reference": {
            "$ref": "#/$defs/ArtifactRef"
          }
        },
        "required": [
          "kind",
          "shadingType",
          "bitsPerCoordinate",
          "bitsPerComponent",
          "decode",
          "reference"
        ]
      }
    ]
  }
} as const;
