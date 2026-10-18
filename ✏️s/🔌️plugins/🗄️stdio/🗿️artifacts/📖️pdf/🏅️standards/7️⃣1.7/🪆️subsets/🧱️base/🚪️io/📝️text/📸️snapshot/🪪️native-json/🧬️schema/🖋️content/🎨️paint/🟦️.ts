/** 🎨️ JSON Schema variants of the PDF 1.7 color, painting and marked content operators. */
export const paintOperatorVariants = [
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setStrokeColorSpace"
      },
      "name": {
        "type": "string"
      }
    },
    "required": [
      "op",
      "name"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setFillColorSpace"
      },
      "name": {
        "type": "string"
      }
    },
    "required": [
      "op",
      "name"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setStrokeColor"
      },
      "components": {
        "type": "array",
        "items": {
          "type": "number"
        }
      }
    },
    "required": [
      "op",
      "components"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setStrokeColorN"
      },
      "components": {
        "type": "array",
        "items": {
          "type": "number"
        }
      },
      "pattern": {
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
      "op",
      "components"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setFillColor"
      },
      "components": {
        "type": "array",
        "items": {
          "type": "number"
        }
      }
    },
    "required": [
      "op",
      "components"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setFillColorN"
      },
      "components": {
        "type": "array",
        "items": {
          "type": "number"
        }
      },
      "pattern": {
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
      "op",
      "components"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setStrokeGray"
      },
      "gray": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "gray"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setFillGray"
      },
      "gray": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "gray"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setStrokeRgb"
      },
      "r": {
        "type": "number"
      },
      "g": {
        "type": "number"
      },
      "b": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "r",
      "g",
      "b"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setFillRgb"
      },
      "r": {
        "type": "number"
      },
      "g": {
        "type": "number"
      },
      "b": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "r",
      "g",
      "b"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setStrokeCmyk"
      },
      "c": {
        "type": "number"
      },
      "m": {
        "type": "number"
      },
      "y": {
        "type": "number"
      },
      "k": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "c",
      "m",
      "y",
      "k"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setFillCmyk"
      },
      "c": {
        "type": "number"
      },
      "m": {
        "type": "number"
      },
      "y": {
        "type": "number"
      },
      "k": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "c",
      "m",
      "y",
      "k"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "paintShading"
      },
      "name": {
        "type": "string"
      }
    },
    "required": [
      "op",
      "name"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "paintXObject"
      },
      "name": {
        "type": "string"
      }
    },
    "required": [
      "op",
      "name"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "inlineImage"
      },
      "image": {
        "$ref": "#/$defs/PdfInlineImage"
      }
    },
    "required": [
      "op",
      "image"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "markedContentPoint"
      },
      "tag": {
        "type": "string"
      }
    },
    "required": [
      "op",
      "tag"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "markedContentPointWithProperties"
      },
      "tag": {
        "type": "string"
      },
      "properties": {
        "$ref": "#/$defs/PdfPropertyList"
      }
    },
    "required": [
      "op",
      "tag",
      "properties"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "beginMarkedContent"
      },
      "tag": {
        "type": "string"
      }
    },
    "required": [
      "op",
      "tag"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "beginMarkedContentWithProperties"
      },
      "tag": {
        "type": "string"
      },
      "properties": {
        "$ref": "#/$defs/PdfPropertyList"
      }
    },
    "required": [
      "op",
      "tag",
      "properties"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "endMarkedContent"
      }
    },
    "required": [
      "op"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "beginCompatibility"
      }
    },
    "required": [
      "op"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "endCompatibility"
      }
    },
    "required": [
      "op"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "unknown"
      },
      "operator": {
        "type": "string"
      },
      "operands": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfObject"
        }
      }
    },
    "required": [
      "op",
      "operator",
      "operands"
    ]
  }
] as const;
