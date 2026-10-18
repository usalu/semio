/** 📇️ JSON Schema definitions of the PDF 1.7 document information, encryption and viewer settings. */
export const metadataDefinitions = {
  "PdfDate": {
    "type": "object",
    "properties": {
      "year": {
        "type": "integer"
      },
      "month": {
        "type": "integer",
        "minimum": 0
      },
      "day": {
        "type": "integer",
        "minimum": 0
      },
      "hour": {
        "type": "integer",
        "minimum": 0
      },
      "minute": {
        "type": "integer",
        "minimum": 0
      },
      "second": {
        "type": "integer",
        "minimum": 0
      },
      "offsetMinutes": {
        "anyOf": [
          {
            "type": "integer"
          },
          {
            "type": "null"
          }
        ]
      }
    },
    "required": [
      "year"
    ]
  },
  "PdfEmbeddedFile": {
    "type": "object",
    "properties": {
      "id": {
        "type": "string"
      },
      "fileName": {
        "type": "string"
      },
      "description": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "mimeType": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "data": {
        "type": "array",
        "items": {
          "type": "integer",
          "minimum": 0
        }
      },
      "creationDate": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfDate"
          },
          {
            "type": "null"
          }
        ]
      },
      "modificationDate": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfDate"
          },
          {
            "type": "null"
          }
        ]
      },
      "relationship": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "listed": {
        "type": "boolean"
      }
    },
    "required": [
      "id",
      "fileName",
      "data"
    ]
  },
  "PdfEncryption": {
    "type": "object",
    "properties": {
      "algorithm": {
        "$ref": "#/$defs/PdfEncryptionAlgorithm"
      },
      "permissions": {
        "type": "integer"
      },
      "userPassword": {
        "type": "string"
      },
      "ownerPassword": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "encryptMetadata": {
        "type": "boolean"
      }
    },
    "required": [
      "algorithm"
    ]
  },
  "PdfEncryptionAlgorithm": {
    "type": "string",
    "enum": [
      "rc4_40",
      "rc4_128",
      "aes128",
      "aes256"
    ]
  },
  "PdfInfo": {
    "type": "object",
    "properties": {
      "title": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "author": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "subject": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "keywords": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "creator": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "producer": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "creationDate": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfDate"
          },
          {
            "type": "null"
          }
        ]
      },
      "modificationDate": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfDate"
          },
          {
            "type": "null"
          }
        ]
      },
      "trapped": {
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
    }
  },
  "PdfMarkInfo": {
    "type": "object",
    "properties": {
      "marked": {
        "type": "boolean"
      },
      "userProperties": {
        "type": "boolean"
      },
      "suspects": {
        "type": "boolean"
      }
    }
  },
  "PdfOutputIntent": {
    "type": "object",
    "properties": {
      "subtype": {
        "type": "string"
      },
      "conditionIdentifier": {
        "type": "string"
      },
      "condition": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "registryName": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "info": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "profile": {
        "anyOf": [
          {
            "$ref": "#/$defs/ArtifactRef"
          },
          {
            "type": "null"
          }
        ]
      }
    },
    "required": [
      "subtype",
      "conditionIdentifier"
    ]
  },
  "PdfPageLayout": {
    "type": "string",
    "enum": [
      "singlePage",
      "oneColumn",
      "twoColumnLeft",
      "twoColumnRight",
      "twoPageLeft",
      "twoPageRight"
    ]
  },
  "PdfPageMode": {
    "type": "string",
    "enum": [
      "useNone",
      "useOutlines",
      "useThumbs",
      "fullScreen",
      "useOc",
      "useAttachments"
    ]
  },
  "PdfViewerPreferences": {
    "type": "object",
    "properties": {
      "hideToolbar": {
        "type": "boolean"
      },
      "hideMenubar": {
        "type": "boolean"
      },
      "hideWindowUi": {
        "type": "boolean"
      },
      "fitWindow": {
        "type": "boolean"
      },
      "centerWindow": {
        "type": "boolean"
      },
      "displayDocTitle": {
        "type": "boolean"
      },
      "nonFullScreenPageMode": {
        "anyOf": [
          {
            "$ref": "#/$defs/PdfPageMode"
          },
          {
            "type": "null"
          }
        ]
      },
      "direction": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "viewArea": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "viewClip": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "printArea": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "printClip": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "printScaling": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "duplex": {
        "anyOf": [
          {
            "type": "string"
          },
          {
            "type": "null"
          }
        ]
      },
      "pickTrayByPdfSize": {
        "type": "boolean"
      },
      "printPageRange": {
        "type": "array",
        "items": {
          "type": "integer",
          "minimum": 0
        }
      },
      "numCopies": {
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
    }
  }
} as const;
