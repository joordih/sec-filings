# DOCUMENTACION - NOTAS

## NOTAS:

- ### EXTRACCIÓN DE FILINGS:
    * **10-Q, 8-K**: Estos filings son siempre en formato XBRL y **SIEMPRE** van a tener en el directorio del *
      *accessNumber** un archivo JSON llamado '**MetaLinks.json**' el cual tiene todos los datos, nombre, información y
      demás cosas que se necesitan para obtener toda la información que se puede encontrar.
        * **Tags**: Los tags que se encuentran en la vista de navegador, en los que puedes hacer click para obtener más
          información sobre esa palabra, se encuentran en el json.
        ````json lines
        {
          "$schema": "http://json-schema.org/draft-04/schema#",
          "type": "object",
          "properties": {
            "version": {
              "type": "string"
            },
            "instance": {
              "type": "object",
              "properties": {
                "aal-20250123.htm": {
                  "type": "object",
                  "properties": {
                    "nsprefix": {
                      "type": "string"
                    },
                    "nsuri": {
                      "type": "string"
                    },
                    "dts": {
                      "type": "object",
                      "properties": {
                        "inline": {
                          "type": "object",
                          "properties": {
                            "local": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            }
                          },
                          "required": [
                            "local"
                          ]
                        },
                        "schema": {
                          "type": "object",
                          "properties": {
                            "local": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "remote": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                },
                                {
                                  "type": "string"
                                }
                              ]
                            }
                          },
                          "required": [
                            "local",
                            "remote"
                          ]
                        },
                        "definitionLink": {
                          "type": "object",
                          "properties": {
                            "local": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            }
                          },
                          "required": [
                            "local"
                          ]
                        },
                        "labelLink": {
                          "type": "object",
                          "properties": {
                            "local": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            }
                          },
                          "required": [
                            "local"
                          ]
                        },
                        "presentationLink": {
                          "type": "object",
                          "properties": {
                            "local": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            }
                          },
                          "required": [
                            "local"
                          ]
                        }
                      },
                      "required": [
                        "inline",
                        "schema",
                        "definitionLink",
                        "labelLink",
                        "presentationLink"
                      ]
                    },
                    "keyStandard": {
                      "type": "integer"
                    },
                    "keyCustom": {
                      "type": "integer"
                    },
                    "axisStandard": {
                      "type": "integer"
                    },
                    "axisCustom": {
                      "type": "integer"
                    },
                    "memberStandard": {
                      "type": "integer"
                    },
                    "memberCustom": {
                      "type": "integer"
                    },
                    "hidden": {
                      "type": "object",
                      "properties": {
                        "total": {
                          "type": "integer"
                        },
                        "http://xbrl.sec.gov/dei/2024": {
                          "type": "integer"
                        }
                      },
                      "required": [
                        "total",
                        "http://xbrl.sec.gov/dei/2024"
                      ]
                    },
                    "contextCount": {
                      "type": "integer"
                    },
                    "entityCount": {
                      "type": "integer"
                    },
                    "segmentCount": {
                      "type": "integer"
                    },
                    "elementCount": {
                      "type": "integer"
                    },
                    "unitCount": {
                      "type": "integer"
                    },
                    "baseTaxonomies": {
                      "type": "object",
                      "properties": {
                        "http://xbrl.sec.gov/dei/2024": {
                          "type": "integer"
                        }
                      },
                      "required": [
                        "http://xbrl.sec.gov/dei/2024"
                      ]
                    },
                    "report": {
                      "type": "object",
                      "properties": {
                        "R1": {
                          "type": "object",
                          "properties": {
                            "role": {
                              "type": "string"
                            },
                            "longName": {
                              "type": "string"
                            },
                            "shortName": {
                              "type": "string"
                            },
                            "isDefault": {
                              "type": "string"
                            },
                            "groupType": {
                              "type": "string"
                            },
                            "subGroupType": {
                              "type": "string"
                            },
                            "menuCat": {
                              "type": "string"
                            },
                            "order": {
                              "type": "string"
                            },
                            "firstAnchor": {
                              "type": "object",
                              "properties": {
                                "contextRef": {
                                  "type": "string"
                                },
                                "name": {
                                  "type": "string"
                                },
                                "unitRef": {
                                  "type": "null"
                                },
                                "xsiNil": {
                                  "type": "string"
                                },
                                "lang": {
                                  "type": "string"
                                },
                                "decimals": {
                                  "type": "null"
                                },
                                "ancestors": {
                                  "type": "array",
                                  "items": [
                                    {
                                      "type": "string"
                                    },
                                    {
                                      "type": "string"
                                    },
                                    {
                                      "type": "string"
                                    },
                                    {
                                      "type": "string"
                                    }
                                  ]
                                },
                                "reportCount": {
                                  "type": "integer"
                                },
                                "baseRef": {
                                  "type": "string"
                                },
                                "first": {
                                  "type": "boolean"
                                },
                                "unique": {
                                  "type": "boolean"
                                }
                              },
                              "required": [
                                "contextRef",
                                "name",
                                "unitRef",
                                "xsiNil",
                                "lang",
                                "decimals",
                                "ancestors",
                                "reportCount",
                                "baseRef",
                                "first",
                                "unique"
                              ]
                            },
                            "uniqueAnchor": {
                              "type": "object",
                              "properties": {
                                "contextRef": {
                                  "type": "string"
                                },
                                "name": {
                                  "type": "string"
                                },
                                "unitRef": {
                                  "type": "null"
                                },
                                "xsiNil": {
                                  "type": "string"
                                },
                                "lang": {
                                  "type": "string"
                                },
                                "decimals": {
                                  "type": "null"
                                },
                                "ancestors": {
                                  "type": "array",
                                  "items": [
                                    {
                                      "type": "string"
                                    },
                                    {
                                      "type": "string"
                                    },
                                    {
                                      "type": "string"
                                    },
                                    {
                                      "type": "string"
                                    }
                                  ]
                                },
                                "reportCount": {
                                  "type": "integer"
                                },
                                "baseRef": {
                                  "type": "string"
                                },
                                "first": {
                                  "type": "boolean"
                                },
                                "unique": {
                                  "type": "boolean"
                                }
                              },
                              "required": [
                                "contextRef",
                                "name",
                                "unitRef",
                                "xsiNil",
                                "lang",
                                "decimals",
                                "ancestors",
                                "reportCount",
                                "baseRef",
                                "first",
                                "unique"
                              ]
                            }
                          },
                          "required": [
                            "role",
                            "longName",
                            "shortName",
                            "isDefault",
                            "groupType",
                            "subGroupType",
                            "menuCat",
                            "order",
                            "firstAnchor",
                            "uniqueAnchor"
                          ]
                        }
                      },
                      "required": [
                        "R1"
                      ]
                    },
                    "tag": {
                      "type": "object",
                      "properties": {
                        "dei_AmendmentFlag": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_CityAreaCode": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "us-gaap_ClassOfStockDomain": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "us-gaap_CommonStockMember": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "srt_ConsolidatedEntitiesAxis": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "srt_ConsolidatedEntitiesDomain": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_CoverAbstract": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_DocumentPeriodEndDate": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_DocumentType": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_EntitiesTable": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_EntityAddressAddressLine1": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_EntityAddressCityOrTown": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_EntityAddressPostalZipCode": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_EntityAddressStateOrProvince": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_EntityCentralIndexKey": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_EntityEmergingGrowthCompany": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_EntityFileNumber": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_EntityIncorporationStateCountryCode": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_EntityInformationLineItems": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_EntityRegistrantName": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_EntityTaxIdentificationNumber": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_LocalPhoneNumber": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_PreCommencementIssuerTenderOffer": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_PreCommencementTenderOffer": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_Security12bTitle": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_SecurityExchangeName": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_SolicitingMaterial": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "us-gaap_StatementClassOfStockAxis": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "srt_SubsidiariesMember": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_TradingSymbol": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "us-gaap_WarrantMember": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": {}
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        },
                        "dei_WrittenCommunications": {
                          "type": "object",
                          "properties": {
                            "xbrltype": {
                              "type": "string"
                            },
                            "nsuri": {
                              "type": "string"
                            },
                            "localname": {
                              "type": "string"
                            },
                            "presentation": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            },
                            "lang": {
                              "type": "object",
                              "properties": {
                                "en-us": {
                                  "type": "object",
                                  "properties": {
                                    "role": {
                                      "type": "object",
                                      "properties": {
                                        "terseLabel": {
                                          "type": "string"
                                        },
                                        "label": {
                                          "type": "string"
                                        },
                                        "documentation": {
                                          "type": "string"
                                        }
                                      },
                                      "required": [
                                        "terseLabel",
                                        "label",
                                        "documentation"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "role"
                                  ]
                                }
                              },
                              "required": [
                                "en-us"
                              ]
                            },
                            "auth_ref": {
                              "type": "array",
                              "items": [
                                {
                                  "type": "string"
                                }
                              ]
                            }
                          },
                          "required": [
                            "xbrltype",
                            "nsuri",
                            "localname",
                            "presentation",
                            "lang",
                            "auth_ref"
                          ]
                        }
                      },
                      "required": [
                        "dei_AmendmentFlag",
                        "dei_CityAreaCode",
                        "us-gaap_ClassOfStockDomain",
                        "us-gaap_CommonStockMember",
                        "srt_ConsolidatedEntitiesAxis",
                        "srt_ConsolidatedEntitiesDomain",
                        "dei_CoverAbstract",
                        "dei_DocumentPeriodEndDate",
                        "dei_DocumentType",
                        "dei_EntitiesTable",
                        "dei_EntityAddressAddressLine1",
                        "dei_EntityAddressCityOrTown",
                        "dei_EntityAddressPostalZipCode",
                        "dei_EntityAddressStateOrProvince",
                        "dei_EntityCentralIndexKey",
                        "dei_EntityEmergingGrowthCompany",
                        "dei_EntityFileNumber",
                        "dei_EntityIncorporationStateCountryCode",
                        "dei_EntityInformationLineItems",
                        "dei_EntityRegistrantName",
                        "dei_EntityTaxIdentificationNumber",
                        "dei_LocalPhoneNumber",
                        "dei_PreCommencementIssuerTenderOffer",
                        "dei_PreCommencementTenderOffer",
                        "dei_Security12bTitle",
                        "dei_SecurityExchangeName",
                        "dei_SolicitingMaterial",
                        "us-gaap_StatementClassOfStockAxis",
                        "srt_SubsidiariesMember",
                        "dei_TradingSymbol",
                        "us-gaap_WarrantMember",
                        "dei_WrittenCommunications"
                      ]
                    }
                  },
                  "required": [
                    "nsprefix",
                    "nsuri",
                    "dts",
                    "keyStandard",
                    "keyCustom",
                    "axisStandard",
                    "axisCustom",
                    "memberStandard",
                    "memberCustom",
                    "hidden",
                    "contextCount",
                    "entityCount",
                    "segmentCount",
                    "elementCount",
                    "unitCount",
                    "baseTaxonomies",
                    "report",
                    "tag"
                  ]
                }
              },
              "required": [
                "aal-20250123.htm"
              ]
            },
            "std_ref": {
              "type": "object",
              "properties": {
                "r0": {
                  "type": "object",
                  "properties": {
                    "role": {
                      "type": "string"
                    },
                    "Publisher": {
                      "type": "string"
                    },
                    "Name": {
                      "type": "string"
                    },
                    "Number": {
                      "type": "string"
                    },
                    "Section": {
                      "type": "string"
                    },
                    "Subsection": {
                      "type": "string"
                    }
                  },
                  "required": [
                    "role",
                    "Publisher",
                    "Name",
                    "Number",
                    "Section",
                    "Subsection"
                  ]
                },
                "r1": {
                  "type": "object",
                  "properties": {
                    "role": {
                      "type": "string"
                    },
                    "Publisher": {
                      "type": "string"
                    },
                    "Name": {
                      "type": "string"
                    },
                    "Number": {
                      "type": "string"
                    },
                    "Section": {
                      "type": "string"
                    },
                    "Subsection": {
                      "type": "string"
                    }
                  },
                  "required": [
                    "role",
                    "Publisher",
                    "Name",
                    "Number",
                    "Section",
                    "Subsection"
                  ]
                },
                "r2": {
                  "type": "object",
                  "properties": {
                    "role": {
                      "type": "string"
                    },
                    "Publisher": {
                      "type": "string"
                    },
                    "Name": {
                      "type": "string"
                    },
                    "Number": {
                      "type": "string"
                    },
                    "Section": {
                      "type": "string"
                    },
                    "Subsection": {
                      "type": "string"
                    }
                  },
                  "required": [
                    "role",
                    "Publisher",
                    "Name",
                    "Number",
                    "Section",
                    "Subsection"
                  ]
                },
                "r3": {
                  "type": "object",
                  "properties": {
                    "role": {
                      "type": "string"
                    },
                    "Publisher": {
                      "type": "string"
                    },
                    "Name": {
                      "type": "string"
                    },
                    "Number": {
                      "type": "string"
                    },
                    "Section": {
                      "type": "string"
                    },
                    "Subsection": {
                      "type": "string"
                    }
                  },
                  "required": [
                    "role",
                    "Publisher",
                    "Name",
                    "Number",
                    "Section",
                    "Subsection"
                  ]
                },
                "r4": {
                  "type": "object",
                  "properties": {
                    "role": {
                      "type": "string"
                    },
                    "Publisher": {
                      "type": "string"
                    },
                    "Name": {
                      "type": "string"
                    },
                    "Number": {
                      "type": "string"
                    },
                    "Section": {
                      "type": "string"
                    },
                    "Subsection": {
                      "type": "string"
                    }
                  },
                  "required": [
                    "role",
                    "Publisher",
                    "Name",
                    "Number",
                    "Section",
                    "Subsection"
                  ]
                },
                "r5": {
                  "type": "object",
                  "properties": {
                    "role": {
                      "type": "string"
                    },
                    "Publisher": {
                      "type": "string"
                    },
                    "Name": {
                      "type": "string"
                    },
                    "Number": {
                      "type": "string"
                    },
                    "Section": {
                      "type": "string"
                    },
                    "Subsection": {
                      "type": "string"
                    }
                  },
                  "required": [
                    "role",
                    "Publisher",
                    "Name",
                    "Number",
                    "Section",
                    "Subsection"
                  ]
                },
                "r6": {
                  "type": "object",
                  "properties": {
                    "role": {
                      "type": "string"
                    },
                    "Publisher": {
                      "type": "string"
                    },
                    "Name": {
                      "type": "string"
                    },
                    "Number": {
                      "type": "string"
                    },
                    "Section": {
                      "type": "string"
                    }
                  },
                  "required": [
                    "role",
                    "Publisher",
                    "Name",
                    "Number",
                    "Section"
                  ]
                }
              },
              "required": [
                "r0",
                "r1",
                "r2",
                "r3",
                "r4",
                "r5",
                "r6"
              ]
            }
          },
          "required": [
            "version",
            "instance",
            "std_ref"
          ]
        }
        ````
