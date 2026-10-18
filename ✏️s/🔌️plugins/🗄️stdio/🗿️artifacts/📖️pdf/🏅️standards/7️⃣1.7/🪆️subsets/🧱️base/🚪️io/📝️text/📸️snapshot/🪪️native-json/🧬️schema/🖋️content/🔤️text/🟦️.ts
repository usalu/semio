/** ✒️ JSON Schema variants of the PDF 1.7 text and glyph width operators. */
export const textOperatorVariants = [
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "beginText"
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
        "const": "endText"
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
        "const": "setCharSpacing"
      },
      "spacing": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "spacing"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setWordSpacing"
      },
      "spacing": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "spacing"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setHorizontalScale"
      },
      "scale": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "scale"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setLeading"
      },
      "leading": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "leading"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setFont"
      },
      "name": {
        "type": "string"
      },
      "size": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "name",
      "size"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setTextRenderingMode"
      },
      "mode": {
        "type": "integer",
        "minimum": 0
      }
    },
    "required": [
      "op",
      "mode"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setTextRise"
      },
      "rise": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "rise"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "moveText"
      },
      "tx": {
        "type": "number"
      },
      "ty": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "tx",
      "ty"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "moveTextSetLeading"
      },
      "tx": {
        "type": "number"
      },
      "ty": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "tx",
      "ty"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setTextMatrix"
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
        "const": "nextLine"
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
        "const": "showText"
      },
      "text": {
        "$ref": "#/$defs/PdfTextString"
      }
    },
    "required": [
      "op",
      "text"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "showTextArray"
      },
      "items": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfTextArrayItem"
        }
      }
    },
    "required": [
      "op",
      "items"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "nextLineShowText"
      },
      "text": {
        "$ref": "#/$defs/PdfTextString"
      }
    },
    "required": [
      "op",
      "text"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "nextLineShowTextSpaced"
      },
      "wordSpacing": {
        "type": "number"
      },
      "charSpacing": {
        "type": "number"
      },
      "text": {
        "$ref": "#/$defs/PdfTextString"
      }
    },
    "required": [
      "op",
      "wordSpacing",
      "charSpacing",
      "text"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setGlyphWidth"
      },
      "wx": {
        "type": "number"
      },
      "wy": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "wx",
      "wy"
    ]
  },
  {
    "type": "object",
    "properties": {
      "op": {
        "const": "setGlyphWidthAndBox"
      },
      "wx": {
        "type": "number"
      },
      "wy": {
        "type": "number"
      },
      "llx": {
        "type": "number"
      },
      "lly": {
        "type": "number"
      },
      "urx": {
        "type": "number"
      },
      "ury": {
        "type": "number"
      }
    },
    "required": [
      "op",
      "wx",
      "wy",
      "llx",
      "lly",
      "urx",
      "ury"
    ]
  }
] as const;
