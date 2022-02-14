# Builtin imports
from google.protobuf.json_format import MessageToJson


# Internal imports
# from extraction.extraction import FoundExpression
import root_pb2 as root
import parser


def parse_expression(exp = None):
	"""
    Main parsing function to parse sub expressions of expression

    :param exp: protobuf Expression object
    :return: protobuf Expression object
    """
	# exp = parser.parse(r"\b[A-Z]{1}\b")
	tokens = exp.tokens
	index = 0
	while index < len(tokens):
		if tokens[index].token == "(" or tokens[index].token == "[" or tokens[index].token == "<"\
			or tokens[index].token == "/":
			length, sub_expression = find_closing(tokens[index + 1:], tokens[index].token)
			append_sub_expression(exp, sub_expression, tokens[index: index + length])
			index += length
		elif tokens[index].type == root.TokenType.QuantifierModifier and\
			tokens[index].groupref != root.QuantifierModifierType.AlternationPipe:
			if len(exp.expressions) == 0:
				raise ValueError('Wrong expression.')
			exp.expressions[-1].raw += tokens[index].token
			exp.expressions[-1].tokens.append(tokens[index])
			index += 1
		else:
			append_sub_expression(exp, tokens[index].token, [tokens[index]])
			index += 1

	json_obj = MessageToJson(exp)
	# print(json_obj)
	return exp

################ Helper functions for parsing specific characters ################
def find_closing(tokens, opening):
	"""
    Function for parsing sub_expression that has a closing

    :param tokens: tokens of Expression object
    :param opening: opening character
    """
	if opening == "(":
		closing = ")"
	elif opening == "[":
		closing = "]"
	elif opening == "<":
		closing = ">"
	elif opening == "/":
		closing = "/"
	remain = 0
	sub_expression = opening
	for i in range(len(tokens)):
		if tokens[i].token == opening:
			remain += 1 # edge case like (a(b)c)
		elif tokens[i].token == closing:
			if remain == 0:
				sub_expression += tokens[i].token
				return i + 2, sub_expression
			remain -= 1
		sub_expression += tokens[i].token

################ Helper functions for modifying protobuf ################
def append_sub_expression(exp, sub_expression, tokens):
	"""
    Add a sub expression of an Expression object sub expressions

    :param exp: protobuf Expression object
	:param sub_expression: regex expression
    :param tokens: tokens of sub_expression
    """
	expression = root.Expression()
	expression.raw = sub_expression
	expression.tokens.extend(tokens)
	exp.expressions.append(expression)

# parse_expression()


