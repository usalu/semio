/** 🔤️ JSON Schema definitions of the PDF 1.7 fonts, encodings and character maps. */
export const fontDefinitions = {
  "PdfBaseEncoding": {
    "type": "string",
    "enum": [
      "standard",
      "winAnsi",
      "macRoman",
      "macExpert"
    ]
  },
  "PdfCMap": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "predefined"
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
            "const": "embedded"
          },
          "cmap": {
            "$ref": "#/$defs/PdfEmbeddedCMap"
          }
        },
        "required": [
          "kind",
          "cmap"
        ]
      }
    ]
  },
  "PdfCharProc": {
    "type": "object",
    "properties": {
      "name": {
        "type": "string"
      },
      "content": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfOp"
        }
      }
    },
    "required": [
      "name",
      "content"
    ]
  },
  "PdfCidFont": {
    "type": "object",
    "properties": {
      "trueType": {
        "type": "boolean"
      },
      "baseFont": {
        "type": "string"
      },
      "systemInfo": {
        "$ref": "#/$defs/PdfCidSystemInfo"
      },
      "descriptor": {
        "$ref": "#/$defs/PdfFontDescriptor"
      },
      "defaultWidth": {
        "type": "number"
      },
      "widths": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfCidWidthRun"
        }
      },
      "defaultVertical": {
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
      "verticalMetrics": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfCidVerticalRun"
        }
      },
      "cidToGid": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfCidToGid"
          },
          {
            "type": "null"
          }
        ]
      },
      "program": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfFontProgram"
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
      "trueType",
      "baseFont",
      "descriptor"
    ]
  },
  "PdfCidMapping": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "char"
          },
          "code": {
            "type": "integer",
            "minimum": 0
          },
          "cid": {
            "type": "integer",
            "minimum": 0
          }
        },
        "required": [
          "kind",
          "code",
          "cid"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "range"
          },
          "low": {
            "type": "integer",
            "minimum": 0
          },
          "high": {
            "type": "integer",
            "minimum": 0
          },
          "cid": {
            "type": "integer",
            "minimum": 0
          }
        },
        "required": [
          "kind",
          "low",
          "high",
          "cid"
        ]
      }
    ]
  },
  "PdfCidSystemInfo": {
    "type": "object",
    "properties": {
      "registry": {
        "type": "string"
      },
      "ordering": {
        "type": "string"
      },
      "supplement": {
        "type": "integer",
        "minimum": 0
      }
    },
    "required": [
      "registry",
      "ordering",
      "supplement"
    ]
  },
  "PdfCidToGid": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "identity"
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
            "const": "map"
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
        ]
      }
    ]
  },
  "PdfCidVerticalRun": {
    "type": "object",
    "properties": {
      "startCid": {
        "type": "integer",
        "minimum": 0
      },
      "metrics": {
        "type": "array",
        "items": {
          "type": "array",
          "items": {
            "type": "number"
          },
          "minItems": 3,
          "maxItems": 3
        }
      }
    },
    "required": [
      "startCid",
      "metrics"
    ]
  },
  "PdfCidWidthRun": {
    "type": "object",
    "properties": {
      "startCid": {
        "type": "integer",
        "minimum": 0
      },
      "widths": {
        "type": "array",
        "items": {
          "type": "number"
        }
      }
    },
    "required": [
      "startCid",
      "widths"
    ]
  },
  "PdfCodespaceRange": {
    "type": "object",
    "properties": {
      "byteWidth": {
        "type": "integer",
        "minimum": 0
      },
      "low": {
        "type": "integer",
        "minimum": 0
      },
      "high": {
        "type": "integer",
        "minimum": 0
      }
    },
    "required": [
      "byteWidth",
      "low",
      "high"
    ]
  },
  "PdfEmbeddedCMap": {
    "type": "object",
    "properties": {
      "name": {
        "type": "string"
      },
      "vertical": {
        "type": "boolean"
      },
      "codespace": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfCodespaceRange"
        }
      },
      "mappings": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfCidMapping"
        }
      },
      "useCmap": {
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
      "name"
    ]
  },
  "PdfEncodingDifference": {
    "type": "object",
    "properties": {
      "code": {
        "type": "integer",
        "minimum": 0
      },
      "glyph": {
        "type": "string"
      }
    },
    "required": [
      "code",
      "glyph"
    ]
  },
  "PdfFont": {
    "type": "object",
    "properties": {
      "id": {
        "type": "string"
      },
      "kind": {
        "$ref": "#/$defs/PdfFontKind"
      },
      "toUnicode": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfToUnicode"
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
      "kind"
    ]
  },
  "PdfFontDescriptor": {
    "type": "object",
    "properties": {
      "fontName": {
        "type": "string"
      },
      "flags": {
        "type": "integer",
        "minimum": 0
      },
      "fontBbox": {
        "type": "array",
        "items": {
          "type": "number"
        },
        "minItems": 4,
        "maxItems": 4
      },
      "italicAngle": {
        "type": "number"
      },
      "ascent": {
        "type": "number"
      },
      "descent": {
        "type": "number"
      },
      "capHeight": {
        "type": "number"
      },
      "stemV": {
        "type": "number"
      },
      "stemH": {
        "anyOf": [
          {
            "type": "number"
          },
          {
            "type": "null"
          }
        ]
      },
      "xHeight": {
        "anyOf": [
          {
            "type": "number"
          },
          {
            "type": "null"
          }
        ]
      },
      "leading": {
        "anyOf": [
          {
            "type": "number"
          },
          {
            "type": "null"
          }
        ]
      },
      "avgWidth": {
        "anyOf": [
          {
            "type": "number"
          },
          {
            "type": "null"
          }
        ]
      },
      "maxWidth": {
        "anyOf": [
          {
            "type": "number"
          },
          {
            "type": "null"
          }
        ]
      },
      "missingWidth": {
        "anyOf": [
          {
            "type": "number"
          },
          {
            "type": "null"
          }
        ]
      },
      "fontFamily": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "fontStretch": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "fontWeight": {
        "anyOf": [
          {
            "type": "number"
          },
          {
            "type": "null"
          }
        ]
      },
      "charSet": {
        "anyOf": [
          {
            "type": "string"
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
      "fontName"
    ]
  },
  "PdfFontKind": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "type1"
          },
          "baseFont": {
            "type": "string"
          },
          "encoding": {
            "$ref": "#/$defs/PdfSimpleEncoding"
          },
          "firstChar": {
            "type": "integer",
            "minimum": 0
          },
          "widths": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "descriptor": {
            "anyOf": [
              {
                "$ref": "#/$defs/PdfFontDescriptor"
              },
              {
                "type": "null"
              }
            ]
          },
          "program": {
            "anyOf": [
              {
                "$ref": "#/$defs/PdfFontProgram"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "kind",
          "baseFont",
          "encoding",
          "firstChar",
          "widths"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "trueType"
          },
          "baseFont": {
            "type": "string"
          },
          "encoding": {
            "$ref": "#/$defs/PdfSimpleEncoding"
          },
          "firstChar": {
            "type": "integer",
            "minimum": 0
          },
          "widths": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "descriptor": {
            "anyOf": [
              {
                "$ref": "#/$defs/PdfFontDescriptor"
              },
              {
                "type": "null"
              }
            ]
          },
          "program": {
            "anyOf": [
              {
                "$ref": "#/$defs/PdfFontProgram"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "kind",
          "baseFont",
          "encoding",
          "firstChar",
          "widths"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "type3"
          },
          "fontMatrix": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 6,
            "maxItems": 6
          },
          "fontBbox": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 4,
            "maxItems": 4
          },
          "encoding": {
            "$ref": "#/$defs/PdfSimpleEncoding"
          },
          "firstChar": {
            "type": "integer",
            "minimum": 0
          },
          "widths": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "charProcs": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PdfCharProc"
            }
          },
          "descriptor": {
            "anyOf": [
              {
                "$ref": "#/$defs/PdfFontDescriptor"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "kind",
          "fontMatrix",
          "fontBbox",
          "encoding",
          "firstChar",
          "widths",
          "charProcs"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "type0"
          },
          "baseFont": {
            "type": "string"
          },
          "cmap": {
            "$ref": "#/$defs/PdfCMap"
          },
          "descendant": {
            "$ref": "#/$defs/PdfCidFont"
          }
        },
        "required": [
          "kind",
          "baseFont",
          "cmap",
          "descendant"
        ]
      }
    ]
  },
  "PdfFontProgram": {
    "oneOf": [
      {
        "type": "object",
        "required": [
          "kind",
          "reference"
        ],
        "properties": {
          "kind": {
            "const": "type1"
          },
          "reference": {
            "$ref": "#/$defs/ArtifactRef"
          }
        },
        "additionalProperties": false
      },
      {
        "type": "object",
        "required": [
          "kind",
          "reference"
        ],
        "properties": {
          "kind": {
            "const": "trueType"
          },
          "reference": {
            "$ref": "#/$defs/ArtifactRef"
          }
        },
        "additionalProperties": false
      },
      {
        "type": "object",
        "required": [
          "kind",
          "reference"
        ],
        "properties": {
          "kind": {
            "const": "cff"
          },
          "reference": {
            "$ref": "#/$defs/ArtifactRef"
          }
        },
        "additionalProperties": false
      },
      {
        "type": "object",
        "required": [
          "kind",
          "reference"
        ],
        "properties": {
          "kind": {
            "const": "cidCff"
          },
          "reference": {
            "$ref": "#/$defs/ArtifactRef"
          }
        },
        "additionalProperties": false
      },
      {
        "type": "object",
        "required": [
          "kind",
          "reference"
        ],
        "properties": {
          "kind": {
            "const": "openType"
          },
          "reference": {
            "$ref": "#/$defs/ArtifactRef"
          }
        },
        "additionalProperties": false
      }
    ]
  },
  "PdfSimpleEncoding": {
    "type": "object",
    "properties": {
      "base": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfBaseEncoding"
          },
          {
            "type": "null"
          }
        ]
      },
      "differences": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfEncodingDifference"
        }
      }
    }
  },
  "PdfToUnicode": {
    "type": "object",
    "properties": {
      "byteWidth": {
        "type": "integer",
        "minimum": 0
      },
      "mappings": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfToUnicodeMapping"
        }
      }
    },
    "required": [
      "byteWidth"
    ]
  },
  "PdfToUnicodeMapping": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "char"
          },
          "code": {
            "type": "integer",
            "minimum": 0
          },
          "text": {
            "type": "string"
          }
        },
        "required": [
          "kind",
          "code",
          "text"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "range"
          },
          "low": {
            "type": "integer",
            "minimum": 0
          },
          "high": {
            "type": "integer",
            "minimum": 0
          },
          "text": {
            "type": "string"
          }
        },
        "required": [
          "kind",
          "low",
          "high",
          "text"
        ]
      }
    ]
  }
} as const;
