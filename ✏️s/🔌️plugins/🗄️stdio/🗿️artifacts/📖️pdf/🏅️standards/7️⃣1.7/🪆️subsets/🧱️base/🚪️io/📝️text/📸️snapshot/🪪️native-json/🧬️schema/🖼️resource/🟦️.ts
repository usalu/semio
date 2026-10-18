/** 🖼️ JSON Schema definitions of the PDF 1.7 image XObjects. */
export const imageDefinitions = {
  "PdfImage": {
    "type": "object",
    "properties": {
      "id": {
        "type": "string"
      },
      "width": {
        "type": "integer",
        "minimum": 0
      },
      "height": {
        "type": "integer",
        "minimum": 0
      },
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
      "bitsPerComponent": {
        "type": "integer",
        "minimum": 0
      },
      "imageMask": {
        "type": "boolean"
      },
      "decode": {
        "type": "array",
        "items": {
          "type": "number"
        }
      },
      "interpolate": {
        "type": "boolean"
      },
      "softMask": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "softMaskInData": {
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
      "mask": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfImageMask"
          },
          {
            "type": "null"
          }
        ]
      },
      "matte": {
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
      "intent": {
        "anyOf": [
          {
            "type": "string"
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
      },
      "body": {
        "$ref": "#/$defs/PdfImageBody"
      }
    },
    "required": [
      "id",
      "width",
      "height",
      "body"
    ]
  },
  "PdfImageMask": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "stencil"
          },
          "image": {
            "type": "string"
          }
        },
        "required": [
          "kind",
          "image"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "colorKey"
          },
          "ranges": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0
            }
          }
        },
        "required": [
          "kind",
          "ranges"
        ]
      }
    ]
  },
  "PdfImageBody": {
    "oneOf": [
      {
        "type": "object",
        "required": [
          "kind",
          "values"
        ],
        "properties": {
          "kind": {
            "const": "samples"
          },
          "values": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 65535
            }
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
            "const": "artifact"
          },
          "reference": {
            "$ref": "#/$defs/ArtifactRef"
          }
        },
        "additionalProperties": false
      }
    ]
  }
} as const;
