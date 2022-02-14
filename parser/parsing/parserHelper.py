# Internal imports
import root_pb2 as root

################ Helper functions for modifying protobuf ################
def append_token(exp, token_str, type, sub_type):
    """
    Create token and append it to tokens of exp

    :param exp: Expression protobuf object
    :param token_str: string of token
    :param type: TokenType
    :param sub_type: token sub_type
    """
    if token_str == "":
        return
    token = root.Token()
    token.token = token_str
    token.type = type
    if type == root.TokenType.Flag:
        token.flag = sub_type
    elif type == root.TokenType.Substitution:
        token.substitution = sub_type
    elif type == root.TokenType.QuantifierModifier:
        token.quantifiermodifier = sub_type 
    elif type == root.TokenType.Anchor:
        token.anchor = sub_type
    elif type == root.TokenType.GroupReference:
        token.groupref = sub_type
    elif type == root.TokenType.Character:
        token.character = sub_type
    elif type == root.TokenType.Lookaround:
        token.lookaround = sub_type
    elif type == root.TokenType.Escape:
        token.escape = sub_type
    else:
        token.characterclass = sub_type
    exp.tokens.append(token)

def modify_string(exp, string):
    if string:
        append_token(exp, string, root.TokenType.Character, string)
    return ""