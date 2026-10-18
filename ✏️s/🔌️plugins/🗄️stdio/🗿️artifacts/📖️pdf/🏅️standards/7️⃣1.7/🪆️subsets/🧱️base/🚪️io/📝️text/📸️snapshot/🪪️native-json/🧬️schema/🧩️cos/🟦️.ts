/** 🧩️ JSON Schema definitions of the PDF 1.7 COS objects, indirect references and stream filters. */
export const cosDefinitions = {
  "ObjRef": {
    "type": "object",
    "properties": {
      "num": {
        "type": "integer",
        "minimum": 0
      },
      "gen": {
        "type": "integer",
        "minimum": 0
      }
    },
    "required": [
      "num",
      "gen"
    ]
  },
  "PdfCcittParameters": {
    "type": "object",
    "properties": {
      "k": {
        "type": "integer"
      },
      "columns": {
        "type": "integer",
        "minimum": 0
      },
      "rows": {
        "type": "integer",
        "minimum": 0
      },
      "blackIs1": {
        "type": "boolean"
      },
      "encodedByteAlign": {
        "type": "boolean"
      },
      "endOfLine": {
        "type": "boolean"
      },
      "endOfBlock": {
        "type": "boolean"
      },
      "damagedRowsBeforeError": {
        "type": "integer",
        "minimum": 0
      }
    }
  },
  "PdfDecimal": {
    "type": "object",
    "properties": {
      "negative": {
        "type": "boolean"
      },
      "coefficient": {
        "type": "string"
      },
      "scale": {
        "type": "integer",
        "minimum": 0
      }
    },
    "required": [
      "negative",
      "coefficient",
      "scale"
    ]
  },
  "PdfDictEntry": {
    "type": "object",
    "properties": {
      "key": {
        "type": "string"
      },
      "value": {
        "$ref": "#/$defs/PdfObject"
      }
    },
    "required": [
      "key",
      "value"
    ]
  },
  "PdfObject": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "null"
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
            "const": "bool"
          },
          "value": {
            "type": "boolean"
          }
        },
        "required": [
          "kind",
          "value"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "int"
          },
          "value": {
            "type": "integer"
          }
        },
        "required": [
          "kind",
          "value"
        ]
      },
      {
        "allOf": [
          {
            "$ref": "#/$defs/PdfDecimal"
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "real"
              }
            },
            "required": [
              "kind"
            ]
          }
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "str"
          },
          "value": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0
            }
          }
        },
        "required": [
          "kind",
          "value"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "name"
          },
          "value": {
            "type": "string"
          }
        },
        "required": [
          "kind",
          "value"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "array"
          },
          "value": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfObject"
            }
          }
        },
        "required": [
          "kind",
          "value"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "dict"
          },
          "value": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfDictEntry"
            }
          }
        },
        "required": [
          "kind",
          "value"
        ]
      },
      {
        "allOf": [
          {
            "$ref": "#/$defs/ObjRef"
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "ref"
              }
            },
            "required": [
              "kind"
            ]
          }
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "stream"
          },
          "dict": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfDictEntry"
            }
          },
          "data": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0
            }
          },
          "filters": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfStreamFilter"
            }
          }
        },
        "required": [
          "kind",
          "dict",
          "data",
          "filters"
        ]
      }
    ]
  },
  "PdfPredictor": {
    "type": "object",
    "properties": {
      "predictor": {
        "type": "integer",
        "minimum": 0
      },
      "colors": {
        "type": "integer",
        "minimum": 0
      },
      "bitsPerComponent": {
        "type": "integer",
        "minimum": 0
      },
      "columns": {
        "type": "integer",
        "minimum": 0
      }
    },
    "required": [
      "predictor",
      "colors",
      "bitsPerComponent",
      "columns"
    ]
  },
  "PdfStreamFilter": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "flate"
          },
          "predictor": {
            "anyOf": [
              {
                "$ref": "#/$defs/PdfPredictor"
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
            "const": "lzw"
          },
          "predictor": {
            "anyOf": [
              {
                "$ref": "#/$defs/PdfPredictor"
              },
              {
                "type": "null"
              }
            ]
          },
          "earlyChange": {
            "type": "boolean"
          }
        },
        "required": [
          "kind",
          "earlyChange"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "asciiHex"
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
            "const": "ascii85"
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
            "const": "runLength"
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
            "const": "dct"
          },
          "colorTransform": {
            "anyOf": [
              {
                "type": "integer",
                "minimum": 0
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
            "const": "jpx"
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
            "const": "ccitt"
          },
          "parameters": {
            "$ref": "#/$defs/PdfCcittParameters"
          }
        },
        "required": [
          "kind",
          "parameters"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "jbig2"
          },
          "globals": {
            "anyOf": [
              {
                "type": "array",
                "items": {
                  "type": "integer",
                  "minimum": 0
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
            "const": "crypt"
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
          }
        },
        "required": [
          "kind"
        ]
      }
    ]
  }
} as const;
