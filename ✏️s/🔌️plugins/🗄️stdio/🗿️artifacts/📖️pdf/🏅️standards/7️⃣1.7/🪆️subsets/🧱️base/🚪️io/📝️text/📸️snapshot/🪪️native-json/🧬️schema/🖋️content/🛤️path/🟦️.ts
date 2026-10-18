/** 🛤️ JSON Schema variants of the PDF 1.7 graphics state, path and clipping operators. */
export const pathOperatorVariants = [
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setLineWidth"
      },
      "width": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "width"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setLineCap"
      },
      "cap": {
        "$ref": "#/$defs/PdfLineCap"
      }
    },
    "required": [
      "op",
      "cap"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setLineJoin"
      },
      "join": {
        "$ref": "#/$defs/PdfLineJoin"
      }
    },
    "required": [
      "op",
      "join"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setMiterLimit"
      },
      "limit": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "limit"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setDash"
      },
      "array": {
        "type": "array",
        "items": {
          "type": "number"
        }
      },
      "phase": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "array",
      "phase"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setRenderingIntent"
      },
      "intent": {
        "type": "string"
      }
    },
    "required": [
      "op",
      "intent"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setFlatness"
      },
      "flatness": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "flatness"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setExtGState"
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
        "const": "save"
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
        "const": "restore"
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
        "const": "transform"
      },
      "matrix": {
        "type": "array",
        "items": {
          "type": "number"
        },
        "minItems": 6,
        "maxItems": 6
      }
    },
    "required": [
      "op",
      "matrix"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "moveTo"
      },
      "x": {
        "type": "number"
      },
      "y": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "x",
      "y"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "lineTo"
      },
      "x": {
        "type": "number"
      },
      "y": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "x",
      "y"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "curveTo"
      },
      "x1": {
        "type": "number"
      },
      "y1": {
        "type": "number"
      },
      "x2": {
        "type": "number"
      },
      "y2": {
        "type": "number"
      },
      "x3": {
        "type": "number"
      },
      "y3": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "x1",
      "y1",
      "x2",
      "y2",
      "x3",
      "y3"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "curveToInitial"
      },
      "x2": {
        "type": "number"
      },
      "y2": {
        "type": "number"
      },
      "x3": {
        "type": "number"
      },
      "y3": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "x2",
      "y2",
      "x3",
      "y3"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "curveToFinal"
      },
      "x1": {
        "type": "number"
      },
      "y1": {
        "type": "number"
      },
      "x3": {
        "type": "number"
      },
      "y3": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "x1",
      "y1",
      "x3",
      "y3"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "closePath"
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
        "const": "rectangle"
      },
      "x": {
        "type": "number"
      },
      "y": {
        "type": "number"
      },
      "width": {
        "type": "number"
      },
      "height": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "x",
      "y",
      "width",
      "height"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "stroke"
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
        "const": "closeStroke"
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
        "const": "fill"
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
        "const": "fillEvenOdd"
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
        "const": "fillStroke"
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
        "const": "fillStrokeEvenOdd"
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
        "const": "closeFillStroke"
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
        "const": "closeFillStrokeEvenOdd"
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
        "const": "endPath"
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
        "const": "clip"
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
        "const": "clipEvenOdd"
      }
    },
    "required": [
      "op"
    ]
  }
] as const;
