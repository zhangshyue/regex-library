
# Builtin imports
import unittest
import logging

# Internal imports
import root_pb2
from testing.test_utils import PatternTest
import logconf


class IntermediateSubexpressionTests2(unittest.TestCase,
                              PatternTest):
    patterns = {
        "(?:(^[a-zA-Z].+)_(.+$))": PatternTest.gen_root(root_pb2.Expression(
            raw="(?:(^[a-zA-Z].+)_(.+$))",
            tokens=[
                # Add tokens here
                root_pb2.Token(
                    token=r"(",
                    type=root_pb2.TokenType.GroupReference,
                    groupref=root_pb2.GroupReferenceType.OpenCapture
                ),
                root_pb2.Token(
                    token=r"?:",
                    type=root_pb2.TokenType.GroupReference,
                    groupref=root_pb2.GroupReferenceType.NonCapturing
                ),
                root_pb2.Token(
                    token=r"(",
                    type=root_pb2.TokenType.GroupReference,
                    groupref=root_pb2.GroupReferenceType.OpenCapture
                ),
                root_pb2.Token(
                    token=r"[",
                    type=root_pb2.TokenType.CharacterClass,
                    characterclass=root_pb2.CharacterClassType.OpenSet
                ),
                root_pb2.Token(
                    token="^",
                    type=root_pb2.TokenType.CharacterClass,
                    characterclass=root_pb2.CharacterClassType.SetNegation
                ),
                root_pb2.Token(
                    token="a-z",
                    type=root_pb2.TokenType.CharacterClass,
                    characterclass=root_pb2.CharacterClassType.Range
                ),
                root_pb2.Token(
                    token="A-Z",
                    type=root_pb2.TokenType.CharacterClass,
                    characterclass=root_pb2.CharacterClassType.Range
                ),
                root_pb2.Token(
                    token=r"]",
                    type=root_pb2.TokenType.CharacterClass,
                    characterclass=root_pb2.CharacterClassType.CloseSet
                ),
                root_pb2.Token(
                    token=".",
                    type=root_pb2.TokenType.CharacterClass,
                    character=root_pb2.CharacterClassType.Dot
                ),
                root_pb2.Token(
                    token="+",
                    type=root_pb2.TokenType.QuantifierModifier,
                    quantifiermodifier=root_pb2.QuantifierModifierType.Plus
                ),
                root_pb2.Token(
                    token=")",
                    type=root_pb2.TokenType.GroupReference,
                    groupref=root_pb2.GroupReferenceType.CloseCapture
                ),
                root_pb2.Token(
                    token="_",
                    type=root_pb2.TokenType.Character,
                    character="_"
                ),
                root_pb2.Token(
                    token=r"(",
                    type=root_pb2.TokenType.GroupReference,
                    groupref=root_pb2.GroupReferenceType.OpenCapture
                ),
                root_pb2.Token(
                    token=".",
                    type=root_pb2.TokenType.CharacterClass,
                    character=root_pb2.CharacterClassType.Dot
                ),
                root_pb2.Token(
                    token="+",
                    type=root_pb2.TokenType.QuantifierModifier,
                    quantifiermodifier=root_pb2.QuantifierModifierType.Plus
                ),
                root_pb2.Token(
                    token="$",
                    type=root_pb2.TokenType.Anchor,
                    character=root_pb2.AnchorType.End
                ),
                root_pb2.Token(
                    token=")",
                    type=root_pb2.TokenType.GroupReference,
                    groupref=root_pb2.GroupReferenceType.CloseCapture
                ),
                root_pb2.Token(
                    token=")",
                    type=root_pb2.TokenType.GroupReference,
                    groupref=root_pb2.GroupReferenceType.CloseCapture
                ),
            ],
            expressions=[
                root_pb2.Expression(
                    raw="(?:(^[a-zA-Z].+)_(.+$))",
                    tokens=[
                        root_pb2.Token(
                            token=r"(",
                            type=root_pb2.TokenType.GroupReference,
                            groupref=root_pb2.GroupReferenceType.OpenCapture
                        ),
                        root_pb2.Token(
                            token=r"?:",
                            type=root_pb2.TokenType.GroupReference,
                            groupref=root_pb2.GroupReferenceType.NonCapturing
                        ),
                        root_pb2.Token(
                            token=r"(",
                            type=root_pb2.TokenType.GroupReference,
                            groupref=root_pb2.GroupReferenceType.OpenCapture
                        ),
                        root_pb2.Token(
                            token="^",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.SetNegation
                        ),
                        root_pb2.Token(
                            token=r"[",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.OpenSet
                        ),
                        root_pb2.Token(
                            token="a-z",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.Range
                        ),
                        root_pb2.Token(
                            token="A-Z",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.Range
                        ),
                        root_pb2.Token(
                            token=r"]",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.CloseSet
                        ),
                        root_pb2.Token(
                            token=".",
                            type=root_pb2.TokenType.CharacterClass,
                            character=root_pb2.CharacterClassType.Dot
                        ),
                        root_pb2.Token(
                            token="+",
                            type=root_pb2.TokenType.QuantifierModifier,
                            quantifiermodifier=root_pb2.QuantifierModifierType.Plus
                        ),
                        root_pb2.Token(
                            token=")",
                            type=root_pb2.TokenType.GroupReference,
                            groupref=root_pb2.GroupReferenceType.CloseCapture
                        ),
                        root_pb2.Token(
                            token="_",
                            type=root_pb2.TokenType.Character,
                            character="_"
                        ),
                        root_pb2.Token(
                            token=r"(",
                            type=root_pb2.TokenType.GroupReference,
                            groupref=root_pb2.GroupReferenceType.OpenCapture
                        ),
                        root_pb2.Token(
                            token=".",
                            type=root_pb2.TokenType.CharacterClass,
                            character=root_pb2.CharacterClassType.Dot
                        ),
                        root_pb2.Token(
                            token="+",
                            type=root_pb2.TokenType.QuantifierModifier,
                            quantifiermodifier=root_pb2.QuantifierModifierType.Plus
                        ),
                        root_pb2.Token(
                            token="$",
                            type=root_pb2.TokenType.Anchor,
                            character=root_pb2.AnchorType.End
                        ),
                        root_pb2.Token(
                            token=")",
                            type=root_pb2.TokenType.GroupReference,
                            groupref=root_pb2.GroupReferenceType.CloseCapture
                        ),
                        root_pb2.Token(
                            token=")",
                            type=root_pb2.TokenType.GroupReference,
                            groupref=root_pb2.GroupReferenceType.CloseCapture
                        ),
                    ]
                ),
                root_pb2.Expression(
                    raw="(.+$)",
                    tokens=[
                        root_pb2.Token(
                            token=r"(",
                            type=root_pb2.TokenType.GroupReference,
                            groupref=root_pb2.GroupReferenceType.OpenCapture
                        ),
                        root_pb2.Token(
                            token=".",
                            type=root_pb2.TokenType.CharacterClass,
                            character=root_pb2.CharacterClassType.Dot
                        ),
                        root_pb2.Token(
                            token="+",
                            type=root_pb2.TokenType.QuantifierModifier,
                            quantifiermodifier=root_pb2.QuantifierModifierType.Plus
                        ),
                        root_pb2.Token(
                            token="$",
                            type=root_pb2.TokenType.Anchor,
                            character=root_pb2.AnchorType.End
                        ),
                        root_pb2.Token(
                            token=")",
                            type=root_pb2.TokenType.GroupReference,
                            groupref=root_pb2.GroupReferenceType.CloseCapture
                        ),
                    ]
                ),
                root_pb2.Expression(
                    raw="_",
                    tokens=[
                        root_pb2.Token(
                            token="_",
                            type=root_pb2.TokenType.Character,
                            character="_"
                        ),
                    ]
                ),
                root_pb2.Expression(
                    raw=".+",
                    tokens=[
                        root_pb2.Token(
                            token=".",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.Dot
                        ),
                        root_pb2.Token(
                            token="+",
                            type=root_pb2.TokenType.QuantifierModifier,
                            quantifiermodifier=root_pb2.QuantifierModifierType.Plus
                        ),
                    ]
                ),
                root_pb2.Expression(
                    raw=r"^[a-zA-Z]",
                    tokens=[
                        root_pb2.Token(
                            token="^",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.SetNegation
                        ),
                        root_pb2.Token(
                            token=r"[",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.OpenSet
                        ),
                        root_pb2.Token(
                            token="a-z",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.Range
                        ),
                        root_pb2.Token(
                            token="A-Z",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.Range
                        ),
                        root_pb2.Token(
                            token=r"]",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.CloseSet
                        ),
                    ]
                ),
                root_pb2.Expression(
                    raw=r"(^[a-zA-Z].+)",
                    tokens=[
                        root_pb2.Token(
                            token=r"(",
                            type=root_pb2.TokenType.GroupReference,
                            groupref=root_pb2.GroupReferenceType.OpenCapture
                        ),
                        root_pb2.Token(
                            token="^",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.SetNegation
                        ),
                        root_pb2.Token(
                            token=r"[",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.OpenSet
                        ),
                        root_pb2.Token(
                            token="a-z",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.Range
                        ),
                        root_pb2.Token(
                            token="A-Z",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.Range
                        ),
                        root_pb2.Token(
                            token=r"]",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.CloseSet
                        ),
                        root_pb2.Token(
                            token=".",
                            type=root_pb2.TokenType.CharacterClass,
                            character=root_pb2.CharacterClassType.Dot
                        ),
                        root_pb2.Token(
                            token="+",
                            type=root_pb2.TokenType.QuantifierModifier,
                            quantifiermodifier=root_pb2.QuantifierModifierType.Plus
                        ),
                        root_pb2.Token(
                            token=")",
                            type=root_pb2.TokenType.GroupReference,
                            groupref=root_pb2.GroupReferenceType.CloseCapture
                        ),
                    ]
                ),
            ]
        )),
        r"(?:[^\[\]%:/?#]|%[a-fA-F0-9]{2})*": PatternTest.gen_root(root_pb2.Expression(
            raw=r"(?:[^\[\]%:/?#]|%[a-fA-F0-9]{2})*",
            tokens=[
                # Add tokens here
                root_pb2.Token(
                    token=r"/",
                    type=root_pb2.TokenType.Anchor,
                    anchor=root_pb2.AnchorType.ForwardSlash
                ),
                root_pb2.Token(
                    token=r"(",
                    type=root_pb2.TokenType.GroupReference,
                    groupref=root_pb2.GroupReferenceType.OpenCapture
                ),
                root_pb2.Token(
                    token=r"?:",
                    type=root_pb2.TokenType.GroupReference,
                    groupref=root_pb2.GroupReferenceType.NonCapturing
                ),
                root_pb2.Token(
                    token=r"[",
                    type=root_pb2.TokenType.CharacterClass,
                    characterclass=root_pb2.CharacterClassType.OpenSet
                ),
                root_pb2.Token(
                    token="^",
                    type=root_pb2.TokenType.CharacterClass,
                    characterclass=root_pb2.CharacterClassType.SetNegation
                ),
                root_pb2.Token(
                    token="\[",
                    type=root_pb2.TokenType.Escape,
                    anchor=root_pb2.EscapeType.Reserved
                ),
                root_pb2.Token(
                    token="\]",
                    type=root_pb2.TokenType.Escape,
                    anchor=root_pb2.EscapeType.Reserved
                ),
                root_pb2.Token(
                    token="%",
                    type=root_pb2.TokenType.Character,
                    character="%"
                ),
                root_pb2.Token(
                    token=":",
                    type=root_pb2.TokenType.Character,
                    character=":"
                ),
                root_pb2.Token(
                    token="/",
                    type=root_pb2.TokenType.Character,
                    character="/"
                ),
                root_pb2.Token(
                    token="?",
                    type=root_pb2.TokenType.Character,
                    character="?"
                ),
                root_pb2.Token(
                    token="#",
                    type=root_pb2.TokenType.Character,
                    character="#"
                ),
                root_pb2.Token(
                    token="]",
                    type=root_pb2.TokenType.CharacterClass,
                    characterclass=root_pb2.CharacterClassType.CloseSet
                ),
                root_pb2.Token(
                    token="|",
                    type=root_pb2.TokenType.QuantifierModifier,
                    quantifiermodifier=root_pb2.QuantifierModifierType.AlternationPipe
                ),
                root_pb2.Token(
                    token="%",
                    type=root_pb2.TokenType.Character,
                    character="%"
                ),
                root_pb2.Token(
                    token="[",
                    type=root_pb2.TokenType.CharacterClass,
                    characterclass=root_pb2.CharacterClassType.OpenSet
                ),
                root_pb2.Token(
                    token="a-f",
                    type=root_pb2.TokenType.CharacterClass,
                    characterclass=root_pb2.CharacterClassType.Range
                ),
                root_pb2.Token(
                    token="A-F",
                    type=root_pb2.TokenType.CharacterClass,
                    characterclass=root_pb2.CharacterClassType.Range
                ),
                root_pb2.Token(
                    token="0-9",
                    type=root_pb2.TokenType.CharacterClass,
                    characterclass=root_pb2.CharacterClassType.Range
                ),
                root_pb2.Token(
                    token="{2}",
                    type=root_pb2.TokenType.QuantifierModifier,
                    quantifiermodifier=root_pb2.QuantifierModifierType.SpecifiedQuantifier
                ),
                root_pb2.Token(
                    token=")",
                    type=root_pb2.TokenType.GroupReference,
                    groupref=root_pb2.GroupReferenceType.CloseCapture
                ),
                root_pb2.Token(
                    token="*",
                    type=root_pb2.TokenType.GroupReference,
                    groupref=root_pb2.GroupReferenceType.Star
                ),
            ],
            expressions=[
                root_pb2.Expression(
                    raw=r'(?:[^\[\]%:/?#]|%[a-fA-F0-9]{2})*',
                    tokens=[
                        root_pb2.Token(
                            token=r"/",
                            type=root_pb2.TokenType.Anchor,
                            anchor=root_pb2.AnchorType.ForwardSlash
                        ),
                        root_pb2.Token(
                            token=r"(",
                            type=root_pb2.TokenType.GroupReference,
                            groupref=root_pb2.GroupReferenceType.OpenCapture
                        ),
                        root_pb2.Token(
                            token=r"?:",
                            type=root_pb2.TokenType.GroupReference,
                            groupref=root_pb2.GroupReferenceType.NonCapturing
                        ),
                        root_pb2.Token(
                            token=r"[",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.OpenSet
                        ),
                        root_pb2.Token(
                            token="^",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.SetNegation
                        ),
                        root_pb2.Token(
                            token="\[",
                            type=root_pb2.TokenType.Escape,
                            anchor=root_pb2.EscapeType.Reserved
                        ),
                        root_pb2.Token(
                            token="\]",
                            type=root_pb2.TokenType.Escape,
                            anchor=root_pb2.EscapeType.Reserved
                        ),
                        root_pb2.Token(
                            token="%",
                            type=root_pb2.TokenType.Character,
                            character="%"
                        ),
                        root_pb2.Token(
                            token=":",
                            type=root_pb2.TokenType.Character,
                            character=":"
                        ),
                        root_pb2.Token(
                            token="/",
                            type=root_pb2.TokenType.Character,
                            character="/"
                        ),
                        root_pb2.Token(
                            token="?",
                            type=root_pb2.TokenType.Character,
                            character="?"
                        ),
                        root_pb2.Token(
                            token="#",
                            type=root_pb2.TokenType.Character,
                            character="#"
                        ),
                        root_pb2.Token(
                            token="]",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.CloseSet
                        ),
                        root_pb2.Token(
                            token="|",
                            type=root_pb2.TokenType.QuantifierModifier,
                            quantifiermodifier=root_pb2.QuantifierModifierType.AlternationPipe
                        ),
                        root_pb2.Token(
                            token="%",
                            type=root_pb2.TokenType.Character,
                            character="%"
                        ),
                        root_pb2.Token(
                            token="[",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.OpenSet
                        ),
                        root_pb2.Token(
                            token="a-f",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.Range
                        ),
                        root_pb2.Token(
                            token="A-F",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.Range
                        ),
                        root_pb2.Token(
                            token="0-9",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.Range
                        ),
                        root_pb2.Token(
                            token="{2}",
                            type=root_pb2.TokenType.QuantifierModifier,
                            quantifiermodifier=root_pb2.QuantifierModifierType.SpecifiedQuantifier
                        ),
                        root_pb2.Token(
                            token=")",
                            type=root_pb2.TokenType.GroupReference,
                            groupref=root_pb2.GroupReferenceType.CloseCapture
                        ),
                        root_pb2.Token(
                            token="*",
                            type=root_pb2.TokenType.GroupReference,
                            groupref=root_pb2.GroupReferenceType.Star
                        ),
                    ]
                ),
                root_pb2.Expression(
                    raw='[a-fA-F0-9]{2}',
                    tokens=[
                        root_pb2.Token(
                            token="[",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.OpenSet
                        ),
                        root_pb2.Token(
                            token="a-f",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.Range
                        ),
                        root_pb2.Token(
                            token="A-F",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.Range
                        ),
                        root_pb2.Token(
                            token="0-9",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.Range
                        ),
                        root_pb2.Token(
                            token="{2}",
                            type=root_pb2.TokenType.QuantifierModifier,
                            quantifiermodifier=root_pb2.QuantifierModifierType.SpecifiedQuantifier
                        ),
                    ]
                ),
                root_pb2.Expression(
                    raw='?:[^\[\]%:/?#]|%',
                    tokens=[
                        root_pb2.Token(
                            token=r"?:",
                            type=root_pb2.TokenType.GroupReference,
                            groupref=root_pb2.GroupReferenceType.NonCapturing
                        ),
                        root_pb2.Token(
                            token=r"[",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.OpenSet
                        ),
                        root_pb2.Token(
                            token="^",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.SetNegation
                        ),
                        root_pb2.Token(
                            token="\[",
                            type=root_pb2.TokenType.Escape,
                            anchor=root_pb2.EscapeType.Reserved
                        ),
                        root_pb2.Token(
                            token="\]",
                            type=root_pb2.TokenType.Escape,
                            anchor=root_pb2.EscapeType.Reserved
                        ),
                        root_pb2.Token(
                            token="%",
                            type=root_pb2.TokenType.Character,
                            character="%"
                        ),
                        root_pb2.Token(
                            token=":",
                            type=root_pb2.TokenType.Character,
                            character=":"
                        ),
                        root_pb2.Token(
                            token="/",
                            type=root_pb2.TokenType.Character,
                            character="/"
                        ),
                        root_pb2.Token(
                            token="?",
                            type=root_pb2.TokenType.Character,
                            character="?"
                        ),
                        root_pb2.Token(
                            token="#",
                            type=root_pb2.TokenType.Character,
                            character="#"
                        ),
                        root_pb2.Token(
                            token="]",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.CloseSet
                        ),
                        root_pb2.Token(
                            token="|",
                            type=root_pb2.TokenType.QuantifierModifier,
                            quantifiermodifier=root_pb2.QuantifierModifierType.AlternationPipe
                        ),
                        root_pb2.Token(
                            token="%",
                            type=root_pb2.TokenType.Character,
                            character="%"
                        ),
                    ]
                ),
                root_pb2.Expression(
                    raw='%',
                    tokens=[
                        root_pb2.Token(
                            token="%",
                            type=root_pb2.TokenType.Character,
                            character = "%"
                        ),
                    ]
                ),
                root_pb2.Expression(
                    raw='[^\[\]%:/?#]|%',
                    tokens=[
                        root_pb2.Token(
                            token=r"[",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.OpenSet
                        ),
                        root_pb2.Token(
                            token="^",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.SetNegation
                        ),
                        root_pb2.Token(
                            token="\[",
                            type=root_pb2.TokenType.Escape,
                            anchor=root_pb2.EscapeType.Reserved
                        ),
                        root_pb2.Token(
                            token="\]",
                            type=root_pb2.TokenType.Escape,
                            anchor=root_pb2.EscapeType.Reserved
                        ),
                        root_pb2.Token(
                            token="%",
                            type=root_pb2.TokenType.Character,
                            character="%"
                        ),
                        root_pb2.Token(
                            token=":",
                            type=root_pb2.TokenType.Character,
                            character=":"
                        ),
                        root_pb2.Token(
                            token="/",
                            type=root_pb2.TokenType.Character,
                            character="/"
                        ),
                        root_pb2.Token(
                            token="?",
                            type=root_pb2.TokenType.Character,
                            character="?"
                        ),
                        root_pb2.Token(
                            token="#",
                            type=root_pb2.TokenType.Character,
                            character="#"
                        ),
                        root_pb2.Token(
                            token="]",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.CloseSet
                        ),
                        root_pb2.Token(
                            token="|",
                            type=root_pb2.TokenType.QuantifierModifier,
                            quantifiermodifier=root_pb2.QuantifierModifierType.AlternationPipe
                        ),
                        root_pb2.Token(
                            token="%",
                            type=root_pb2.TokenType.Character,
                            character="%"
                        ),
                    ]
                ),
                root_pb2.Expression(
                    raw='[^\[\]%:/?#]',
                    tokens=[
                        root_pb2.Token(
                            token=r"[",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.OpenSet
                        ),
                        root_pb2.Token(
                            token="^",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.SetNegation
                        ),
                        root_pb2.Token(
                            token="\[",
                            type=root_pb2.TokenType.Escape,
                            anchor=root_pb2.EscapeType.Reserved
                        ),
                        root_pb2.Token(
                            token="\]",
                            type=root_pb2.TokenType.Escape,
                            anchor=root_pb2.EscapeType.Reserved
                        ),
                        root_pb2.Token(
                            token="%",
                            type=root_pb2.TokenType.Character,
                            character="%"
                        ),
                        root_pb2.Token(
                            token=":",
                            type=root_pb2.TokenType.Character,
                            character=":"
                        ),
                        root_pb2.Token(
                            token="/",
                            type=root_pb2.TokenType.Character,
                            character="/"
                        ),
                        root_pb2.Token(
                            token="?",
                            type=root_pb2.TokenType.Character,
                            character="?"
                        ),
                        root_pb2.Token(
                            token="#",
                            type=root_pb2.TokenType.Character,
                            character="#"
                        ),
                        root_pb2.Token(
                            token="]",
                            type=root_pb2.TokenType.CharacterClass,
                            characterclass=root_pb2.CharacterClassType.CloseSet
                        ),
                    ]
                ),
            ]
        ))
    }