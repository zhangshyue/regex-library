# Builtin imports
import typing
import string
import re

# Internal imports
from extraction.FoundExpression import FoundExpression
import root_pb2 as root
from parsing import parserHelper


def parse(fe: typing.Union[str, FoundExpression], exp: root.Expression = None, language: str = None,
in_set: bool = False) -> root.Expression:
    """
    Main parsing function to parse expression into tokens

    :param fe: FoundExpression object
    :param exp: protobuf Expression object
    :param language: language Expression is written in
    :param in_set: whether this expression is within []
    :return: protobuf Expression object
    """
    if isinstance(fe, str):
        expression = fe
    else:
        expression = fe.expression
    if not language and not isinstance(language, int):
        language = fe.language
    if not exp:
        exp = root.Expression()
        exp.raw = expression
    index = 0
    string = ""
    while index < len(expression):
        if not in_set and (expression[index] == "[" or expression[index] == "(" or expression[index] == "{" or
                           (language == "JS" and expression[index] == "/")):
            parserHelper.append_token(exp, string, root.TokenType.Character, string)
            length, is_literal = find_closing(exp, expression[index:], language)
            if not is_literal:
                string = ""
            else:
                del exp.tokens[-1]
                string += expression[index]
            index += length
        elif expression[index] == '\\':  # this is just \, \ within a str represents an escape character
            string = parserHelper.modify_string(exp, string)
            length = parse_backslash(exp, expression[index + 1:], language)
            index += length
        elif not in_set and (expression[index] == "?"): 
            string = parserHelper.modify_string(exp, string)
            parse_question_mark(exp, expression[index:], language)
            index += len(exp.tokens[-1].token)
        elif not in_set and (expression[index] == "*"):
            string = parserHelper.modify_string(exp, string)
            parserHelper.append_token(exp, "*", root.TokenType.QuantifierModifier, root.QuantifierModifierType.Star)
            index += 1 
        elif not in_set and (expression[index] == "|"): # OR
            string = parserHelper.modify_string(exp, string)
            parserHelper.append_token(exp, "|", root.TokenType.QuantifierModifier, root.QuantifierModifierType.AlternationPipe)
            index += 1 
        elif expression[index] == "-":
            if index == 0 or len(expression) == index + 1: # - can be interpreted literally
                parserHelper.modify_string(exp, expression[index])
                index += 1
            else:
                del exp.tokens[-1]
                parserHelper.append_token(exp, expression[index - 1: index + 2], root.TokenType.CharacterClass, root.CharacterClassType.RangeSet)
                index += 2
        elif expression[index] == "^":
            string = parserHelper.modify_string(exp, string)
            if exp.tokens and exp.tokens[-1].token == "[":
                parserHelper.append_token(exp, "^", root.TokenType.CharacterClass, root.CharacterClassType.SetNegation)
            else:
                parserHelper.append_token(exp, "^", root.TokenType.Anchor, root.AnchorType.Beginning)
            index += 1
        elif not in_set and (expression[index] == "."): 
            string = parserHelper.modify_string(exp, string)
            parserHelper.append_token(exp, ".", root.TokenType.CharacterClass, root.CharacterClassType.Dot)
            index += 1
        elif not in_set and (expression[index] == "$"):
            string = parserHelper.modify_string(exp, string)
            length = parse_dollar_sign(exp, expression[index:])
            index += length
        elif not in_set and (expression[index] == "+"):
            string = parserHelper.modify_string(exp, string)
            parserHelper.append_token(exp, "+", root.TokenType.QuantifierModifier, root.QuantifierModifierType.Plus)
            index += 1
        elif in_set: # ex: [\101a-df<?h|b*.]
            parserHelper.modify_string(exp, expression[index])
            index += 1    
        else:
            string += expression[index]
            index += 1
    if string:
        if exp.tokens and exp.tokens[-1].token == "[" and string == expression:
            parserHelper.append_token(exp, expression, root.TokenType.CharacterClass, root.CharacterClassType.InclusiveSet)
        else:
            parserHelper.append_token(exp, string, root.TokenType.Character, string)
    # json_obj = MessageToJson(exp)
    # print(json_obj)
    return exp

def find_balanced_closing(opening_char: str, closing_char: str, expression: str) -> int:
    in_escape = False
    closing_needed = 0

    for (idx, ch) in enumerate(expression):
        if in_escape:
            in_escape = False
        else:
            if ch == '\\':
                in_escape = True
            else:
                if ch == closing_char:
                    closing_needed -= 1
                    if closing_needed == 0:
                        return idx
                elif ch == opening_char:
                    closing_needed += 1

    raise ValueError(f"Expected to find {opening_char} and {closing_char} balanced but did not")


################ Helper functions for parsing specific characters ################
def find_closing(exp, expression, language, prev = ''):
    """
    Function for parsing expression that has a closing

    :param exp: protobuf Expression objectregex expression
    :param expression: regex expression
    :param language: language this expression is written
    :param prev: what's before this expression that need to consider
    :return: length of this expression, bool indicating if the opening is literal character
    """
    opening = expression[0]
    closing = ""
    if opening == "[":
        closing = "]"
    elif opening == "{":
        closing = "}"
    elif opening == "<":
        closing = ">"
    elif opening == "(":
        closing = ")"
    if opening == "/":
        end = expression.rfind('/')
        if end == 0:
            raise ValueError('Wrong regex.')
        parserHelper.append_token(exp, "/", root.TokenType.Anchor,\
            root.AnchorType.ForwardSlash)
        parse(expression[1 : end], exp, language)
        parserHelper.append_token(exp, "/", root.TokenType.Anchor,\
            root.AnchorType.ForwardSlash)
        parse_flag(exp, expression[end + 1:])
        return len(expression), False

    if closing:
        # Do a quick search of the expression for the appearances of closing without an escape
        i = find_balanced_closing(opening, closing, expression)
        if expression[i] == closing:
            if closing == "}":
                if prev == "\p":
                    parserHelper.append_token(exp, expression[:i+1], root.TokenType.CharacterClass,\
                        root.CharacterClassType.UnicodePropertyEscape)
                elif prev == "\P":
                    parserHelper.append_token(exp, expression[:i+1], root.TokenType.CharacterClass,\
                        root.CharacterClassType.NotUnicodePropertyEscape)
                elif prev == r"\u":
                    parserHelper.append_token(exp, expression[:i+1], root.TokenType.Escape,\
                        root.EscapeType.UnicodeEscape)
                elif prev == "":
                    if re.match("\d+,\d*$", expression[1:i]) or expression[1:i].isdigit():
                        parserHelper.append_token(exp, expression[:i+1], root.TokenType.QuantifierModifier,\
                            root.QuantifierModifierType.SpecifiedQuantifier)
                        return i + 1, False
                    return 1, True  # { be interpreted literally
                raise ValueError('Wrong regex.')
            elif closing == "]":
                parserHelper.append_token(exp, opening, root.TokenType.CharacterClass,\
                    root.CharacterClassType.OpenSet)
                parse(expression[1: i], exp, language, in_set=True)
                parserHelper.append_token(exp, closing, root.TokenType.CharacterClass,\
                    root.CharacterClassType.CloseSet)
                return i + 1, False
            elif closing == ")":
                parserHelper.append_token(exp, opening, root.TokenType.GroupReference,\
                    root.GroupReferenceType.OpenCapture)
                parse(expression[1: i], exp, language)
                parserHelper.append_token(exp, closing, root.TokenType.GroupReference,\
                    root.GroupReferenceType.CloseCapture)
                return i + 1, False
            elif closing == ">":
                if prev == "?":
                    parserHelper.append_token(exp, '?' + expression[:i + 1], root.TokenType.GroupReference,\
                    root.GroupReferenceType.GroupName)
                    return i + 2, False

    if opening == "{": # { be interpreted literally
        return 1, True
    raise ValueError('Wrong regex.')

def parse_flag(exp, expression):
    """
    Add flag token to Expression protobuf object

    :param expression: expression string
    :param exp: Expression protobuf object
    """
    tokens = expression.split()
    for t in tokens:
        token = root.Token()
        token.type = root.TokenType.Flag
        token.token = t
        if t == "i":
            token.flag = root.FlagType.Ignore
        elif t == "g":
            token.flag = root.FlagType.Global
        elif t == "m":
            token.flag = root.FlagType.Multiline
        elif t == "u":
            token.flag = root.FlagType.Unicode
        elif t == "y":
            token.flag = root.FlagType.Sticky
        else:
            token.flag = root.FlagType.Dotall
        exp.tokens.append(token)

def parse_backslash(exp, expression, language):
    """
    Determine token that starts with backslash

    :param exp: Expression protobuf object
    :param expression: expression string
    :param language: language this expression is written
    :return token: Token protobuf object
    :return : length of token
    """
    character_classes = ["s", "S", "w", "W", "d", "D"]
    anchors = ["B", "b"]
    escape_reserved = ['+', '*', '?', '^', '$', '\\', '.', '[', ']', '{', '}', '(', ')', '|', '/']
    escape_other = ["t", "n", "v", "f", "r", "0"]

    if expression[0] in character_classes:
        if expression[0] == "s":
            parserHelper.append_token(exp, "\s", root.TokenType.CharacterClass,\
                root.CharacterClassType.Whitespace)
        elif expression[0] == "S":
            parserHelper.append_token(exp, "\S", root.TokenType.CharacterClass,\
                root.CharacterClassType.NotWhitespace)
        elif expression[0] == "w":
            parserHelper.append_token(exp, "\w", root.TokenType.CharacterClass,\
                root.CharacterClassType.Word)
        elif expression[0] == "W":
            parserHelper.append_token(exp, "\W", root.TokenType.CharacterClass,\
                root.CharacterClassType.NotWord)
        elif expression[0] == "d":
            parserHelper.append_token(exp, "\d", root.TokenType.CharacterClass,\
                root.CharacterClassType.Digit)
        else:
            parserHelper.append_token(exp, "\D", root.TokenType.CharacterClass,\
                root.CharacterClassType.NotDigit)
        return 2
    if expression[0] in ["p", "P"]:
        length = find_closing(exp, expression[1:], language, prev = "\\" + expression[0])
        return length
    if expression[0] in anchors:
        if expression[0] == "b":
            parserHelper.append_token(exp, "\b", root.TokenType.Anchor,\
                root.AnchorType.WordBoundary)
        elif expression[0] == "B":
            parserHelper.append_token(exp, "\B", root.TokenType.Anchor,\
                root.AnchorType.NotWordBoundary)
        return 2
    if expression[0] in escape_reserved:
        parserHelper.append_token(exp, "\\" + expression[0], root.TokenType.Escape,\
            root.EscapeType.Reserved)
        return 2
    if expression[0] in escape_other:
        if expression[0] == "t":
            parserHelper.append_token(exp, "\t", root.TokenType.Escape,\
                root.EscapeType.Tab)
        elif expression[0] == "n":
            parserHelper.append_token(exp, "\n", root.TokenType.Escape,\
                root.EscapeType.Tab)
        elif expression[0] == "v":
            parserHelper.append_token(exp, "\v", root.TokenType.Escape,\
                root.EscapeType.Tab)
        elif expression[0] == "f":
            parserHelper.append_token(exp, "\f", root.TokenType.Escape,\
                root.EscapeType.Tab)
        elif expression[0] == "r":
            parserHelper.append_token(exp, "\r", root.TokenType.Escape,\
                root.EscapeType.Tab)
        else:
            parserHelper.append_token(exp, "\n", root.TokenType.Escape,\
                root.EscapeType.Tab)
        return 2
    if len(expression) > 1 and expression[:2] == "cI":
        parserHelper.append_token(exp, "\n", root.TokenType.Escape,\
            root.EscapeType.Control)
        return 3
    if expression[0] == "u" and len(expression) >= 2:
        if expression[1] == "{":
            length = find_closing(exp, expression[1:], language, prev = r"\u") 
            return length
        if len(expression) >= 5 and all(c in string.hexdigits for c in expression[1:5]):
            parserHelper.append_token(exp, "\\" + expression[:5], root.TokenType.Escape,\
                        root.EscapeType.UnicodeEscape)
            return 6
        raise ValueError('Wrong regex.')
    if len(expression) >= 3 and all(c in string.octdigits for c in expression[:3]):
        parserHelper.append_token(exp, "\\" + expression[:3], root.TokenType.Escape,\
                        root.EscapeType.Octal)
        return 4
    elif len(expression) >= 3 and expression[0] == "x" and all(c in string.hexdigits for c in expression[1:3]):
        parserHelper.append_token(exp, "\\" + expression[:3], root.TokenType.Escape,\
                        root.EscapeType.Hexadecimal)
        return 4
    if expression[0].isdigit():
        parserHelper.append_token(exp, "\\" + expression[:0], root.TokenType.GroupReference,\
                        root.GroupReferenceType.NumericReference)
        return 2
    else:
        parserHelper.append_token(exp, "\\" + expression[0], root.TokenType.Character, expression[0])
        return 2

def parse_question_mark(exp, expression, language):
    """
    Function for parsing question mark

    :param exp: protobuf Expression objectregex expression
    :param expression: regex expression
    :param language: language this expression is written
    :return: length of this expression
    """
    if exp.tokens and exp.tokens[-1].token == "(":
        if expression[:2] == "?=":
            parserHelper.append_token(exp, "?=", root.TokenType.Lookaround, root.LookaroundType.PositiveLookahead)
            return 2
        elif expression[:2] == "?!":
            parserHelper.append_token(exp, "?!", root.TokenType.Lookaround, root.LookaroundType.NegativeLookahead)
            return 2
        elif expression[:3] == "?<=":
            parserHelper.append_token(exp, "?<=", root.TokenType.Lookaround, root.LookaroundType.PositiveLookbehind)
            return 3
        elif expression[:3] == "?<!":
            parserHelper.append_token(exp, "?<!", root.TokenType.Lookaround, root.LookaroundType.NegativeLookbehind)
            return 3
        elif expression[:2] == "?:":
            parserHelper.append_token(exp, "?:", root.TokenType.GroupReference, root.GroupReferenceType.NonCapturing)
            return 2
        elif expression[:2] == "?<":
            length = find_closing(exp, expression[1:], language, prev = "?")
            return length
        elif expression[:2] == "?>":
            parserHelper.append_token(exp, "?>", root.TokenType.CharacterClass, root.CharacterClassType.AtomicGroup)
            return 2
        raise ValueError('Wrong regex.')
    if exp.tokens and exp.tokens[-1].token == "+":
        parserHelper.append_token(exp, "?", root.TokenType.QuantifierModifier, root.QuantifierModifierType.Lazy)
    else:
        parserHelper.append_token(exp, "?", root.TokenType.QuantifierModifier, root.QuantifierModifierType.Optional)
    return 1

def parse_dollar_sign(exp, expression):
    """
    Function for parsing dollar sign

    :param exp: protobuf Expression objectregex expression
    :param expression: regex expression
    :return: length of this expression
    """
    if len(expression) > 1:
        if expression[1] == "&":
            parserHelper.append_token(exp, "$&", root.TokenType.Substitution, root.SubstitutionType.Match)
            return 2
        if expression[1] == "`":
            parserHelper.append_token(exp, "$`", root.TokenType.Substitution, root.SubstitutionType.BeforeMatch)
            return 2
        if expression[1] == "'":
            parserHelper.append_token(exp, "$'", root.TokenType.Substitution, root.SubstitutionType.AfterMatch)
            return 2
        if expression[1] == "$":
            parserHelper.append_token(exp, "$$", root.TokenType.Substitution, root.SubstitutionType.EscapedDollar)
            return 2
        if expression[1].isdigit():
            parserHelper.append_token(exp, "$" + expression[1], root.TokenType.Substitution, root.SubstitutionType.Capture)
            return 2
    parserHelper.append_token(exp, "$", root.TokenType.Anchor,root.AnchorType.End)
    return 1



# parse(r"\b[A-Z]{1}\b")
# parse(r"/Chapter (\d+)\.\d*/")
# parse(r"(?<Name>x)")
# parse(r"/\/example\/[a-z]+/i")
# parse(r"(?<=http://)\S+", language="python") # forward slash interpreted different for other languages
# parse(r"(\W|^)(baloney|darn|drat|fooey|gosh\sdarnit|heck)(\W|$)")
# parse(r"(\W|^)stock\s{0,3}tip(s){0,1}(\W|$)")
# parse(r"/[\d\.]/", language="python")
