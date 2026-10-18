/** 🌈️ JSON Schema definitions of the PDF 1.7 color spaces and functions. */
export const colorDefinitions = {
  "PdfColorSpace": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "deviceGray"
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
            "const": "deviceRgb"
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
            "const": "deviceCmyk"
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
            "const": "calGray"
          },
          "whitePoint": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "blackPoint": {
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
          "gamma": {
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
          "kind",
          "whitePoint"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "calRgb"
          },
          "whitePoint": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "blackPoint": {
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
          "gamma": {
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
          "matrix": {
            "anyOf": [
              {
                "type": "array",
                "items": {
                  "type": "number"
                },
                "minItems": 9,
                "maxItems": 9
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "kind",
          "whitePoint"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "lab"
          },
          "whitePoint": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "blackPoint": {
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
          "range": {
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
          "kind",
          "whitePoint"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "iccBased"
          },
          "components": {
            "type": "integer",
            "minimum": 0
          },
          "profile": {
            "$ref": "#/$defs/ArtifactRef"
          },
          "alternate": {
            "anyOf": [
              {
                "$ref": "#/$defs/PdfColorSpace"
              },
              {
                "type": "null"
              }
            ]
          },
          "range": {
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
          "components",
          "profile"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "indexed"
          },
          "base": {
            "$ref": "#/$defs/PdfColorSpace"
          },
          "hival": {
            "type": "integer",
            "minimum": 0
          },
          "palette": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0
            }
          }
        },
        "required": [
          "kind",
          "base",
          "hival",
          "palette"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "separation"
          },
          "name": {
            "type": "string"
          },
          "alternate": {
            "$ref": "#/$defs/PdfColorSpace"
          },
          "tintTransform": {
            "$ref": "#/$defs/PdfFunction"
          }
        },
        "required": [
          "kind",
          "name",
          "alternate",
          "tintTransform"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "deviceN"
          },
          "names": {
            "type": "array",
            "items": {
              "type": "string"
            }
          },
          "alternate": {
            "$ref": "#/$defs/PdfColorSpace"
          },
          "tintTransform": {
            "$ref": "#/$defs/PdfFunction"
          },
          "attributes": {
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
          "names",
          "alternate",
          "tintTransform"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "pattern"
          },
          "base": {
            "anyOf": [
              {
                "$ref": "#/$defs/PdfColorSpace"
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
  "PdfFunction": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "sampled"
          },
          "domain": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "range": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "size": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0
            }
          },
          "bitsPerSample": {
            "type": "integer",
            "minimum": 0
          },
          "order": {
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
          "encode": {
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
          "decode": {
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
          "samples": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0
            }
          }
        },
        "required": [
          "kind",
          "domain",
          "range",
          "size",
          "bitsPerSample",
          "samples"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "exponential"
          },
          "domain": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "range": {
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
          "c0": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "c1": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "n": {
            "type": "number"
          }
        },
        "required": [
          "kind",
          "domain",
          "c0",
          "c1",
          "n"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "stitching"
          },
          "domain": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "range": {
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
          "functions": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfFunction"
            }
          },
          "bounds": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "encode": {
            "type": "array",
            "items": {
              "type": "number"
            }
          }
        },
        "required": [
          "kind",
          "domain",
          "functions",
          "bounds",
          "encode"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "postScript"
          },
          "domain": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "range": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "code": {
            "type": "string"
          }
        },
        "required": [
          "kind",
          "domain",
          "range",
          "code"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "array"
          },
          "functions": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfFunction"
            }
          }
        },
        "required": [
          "kind",
          "functions"
        ]
      }
    ]
  }
} as const;
