## Protobuf IR for Regular Expressions

Command: make build

Dependencies: protobuf, [Rust protobuf plugin](https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-codegen)

Example of a regular expression should be parsed:

- "\b[A-Z]{1}\b", found in file "example.java"

```
Root { 
    file: "example.java",
    language: "Java"
    expression: Expression {
        raw: "\b[A-Z]{1}\b"
        tokens: [
            Token {
                token: "\b",
                type: 3 (Anchor),
                sub_type: AnchorType {
                    2 = WordBoundary
                }
            },
            Token {
                token: "[",
                type: 8 (CharacterClass),
                sub_type: CharacterClassType {
                    0 = OpenSet
                }
            },
            Token {
                token: "A-Z",
                type: 8 (CharacterClass),
                sub_type: CharacterClassType {
                    4 = RangeSet
                }
            },
            Token {
                token: "]",
                type: 8 (CharacterClass),
                sub_type: CharacterClassType {
                    1 = CloseSet
                }
            },
            Token {
                token: "{1}",
                type: 2 (QuantifierModifier),
                sub_type: QuantifierModifierType {
                    2 = SpecifiedQuantifier
                }
            },
             Token {
                token: "\b",
                type: 3 (Anchor),
                sub_type: AnchorType {
                    2 = WordBoundary
                }
            },
        ]
    },
    expressions: [
        Expression {
            raw = "\b"
            tokens = [
                Token {
                    token: "\b",
                    type: 3 (Anchor),
                    sub_type: AnchorType {
                        2 = WordBoundary
                    }
                }
            ]
        },
        Expression {
            raw = "[A-Z]{1}"
            tokens = [
                Token {
                token: "[",
                type: 8 (CharacterClass),
                sub_type: CharacterClassType {
                    0 = OpenSet
                }
            },
            Token {
                token: "A-Z",
                type: 8 (CharacterClass),
                sub_type: CharacterClassType {
                    4 = RangeSet
                }
            },
            Token {
                token: "]",
                type: 8 (CharacterClass),
                sub_type: CharacterClassType {
                    1 = CloseSet
                }
            },
            Token {
                token: "{1}",
                type: 2 (QuantifierModifier),
                sub_type: QuantifierModifierType {
                    2 = SpecifiedQuantifier
                }
            },
        ],
        },
        Expression {
            raw = "\b"
            tokens = [
                Token {
                    token: "\b",
                    type: 3 (Anchor),
                    sub_type: AnchorType {
                        2 = WordBoundary
                    }
                }
            ]
        },
    ]
}
```