/** 📝️ JSON Schema definitions of the PDF 1.7 interactive forms and optional content. */
export const formDefinitions = {
  "PdfAcroForm": {
    "type": "object",
    "properties": {
      "fields": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfFormField"
        }
      },
      "needAppearances": {
        "type": "boolean"
      },
      "signatureFlags": {
        "type": "integer",
        "minimum": 0
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
      "defaultFonts": {
        "type": "array",
        "items": {
          "type": "string"
        }
      },
      "extra": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfDictEntry"
        }
      }
    }
  },
  "PdfFormField": {
    "type": "object",
    "properties": {
      "name": {
        "type": "string"
      },
      "kind": {
        "$ref": "#/$defs/PdfFormFieldKind"
      },
      "flags": {
        "type": "integer",
        "minimum": 0
      },
      "alternateName": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "mappingName": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
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
      "widgets": {
        "type": "array",
        "items": {
          "type": "array",
          "items": {
            "type": "integer",
            "minimum": 0
          },
          "minItems": 2,
          "maxItems": 2
        }
      },
      "children": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfFormField"
        }
      },
      "additionalActions": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfDictEntry"
        }
      },
      "extra": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfDictEntry"
        }
      }
    },
    "required": [
      "name",
      "kind"
    ]
  },
  "PdfFormFieldKind": {
    "oneOf": [
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "button"
          },
          "value": {
            "anyOf": [
              {
                "type": "string"
              },
              {
                "type": "null"
              }
            ]
          },
          "defaultValue": {
            "anyOf": [
              {
                "type": "string"
              },
              {
                "type": "null"
              }
            ]
          },
          "options": {
            "type": "array",
            "items": {
              "type": "string"
            }
          }
        },
        "required": [
          "kind",
          "options"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "text"
          },
          "value": {
            "anyOf": [
              {
                "type": "string"
              },
              {
                "type": "null"
              }
            ]
          },
          "defaultValue": {
            "anyOf": [
              {
                "type": "string"
              },
              {
                "type": "null"
              }
            ]
          },
          "maxLength": {
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
          "richValue": {
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
            "const": "choice"
          },
          "values": {
            "type": "array",
            "items": {
              "type": "string"
            }
          },
          "defaultValues": {
            "type": "array",
            "items": {
              "type": "string"
            }
          },
          "options": {
            "type": "array",
            "items": {
              "type": "array",
              "items": [
                {
                  "type": "string"
                },
                {
                  "type": "string"
                }
              ],
              "minItems": 2,
              "maxItems": 2
            }
          },
          "topIndex": {
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
          "kind",
          "values",
          "defaultValues",
          "options"
        ]
      },
      {
        "type": "object",
        "properties": {
          "kind": {
            "const": "signature"
          },
          "value": {
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
            "const": "container"
          }
        },
        "required": [
          "kind"
        ]
      }
    ]
  },
  "PdfOptionalContent": {
    "type": "object",
    "properties": {
      "groups": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfOptionalContentGroup"
        }
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
      },
      "baseStateOff": {
        "type": "boolean"
      },
      "on": {
        "type": "array",
        "items": {
          "type": "string"
        }
      },
      "off": {
        "type": "array",
        "items": {
          "type": "string"
        }
      },
      "order": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfObject"
        }
      },
      "extra": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfDictEntry"
        }
      }
    }
  },
  "PdfOptionalContentGroup": {
    "type": "object",
    "properties": {
      "id": {
        "type": "string"
      },
      "name": {
        "type": "string"
      },
      "intent": {
        "type": "array",
        "items": {
          "type": "string"
        }
      },
      "usage": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PdfDictEntry"
        }
      }
    },
    "required": [
      "id",
      "name"
    ]
  }
} as const;
