# Builtin imports
import sys
import base64

# Internal imports
from root_pb2 import *

# External imports
import rich

LINES_SURROUNDING = 5


def retrieve_program_context(root: Root) -> list:
    file_contents = open(root.file)

    lines = list(file_contents.readlines())

    context = []

    if root.line_number > LINES_SURROUNDING:
        start_line = root.line_number - LINES_SURROUNDING
    else:
        start_line = 0

    if len(lines) > root.line_number + LINES_SURROUNDING:
        end_line = root.line_number + LINES_SURROUNDING
    else:
        end_line = len(lines)

    for line_idx in range(start_line, end_line):
        context.append((line_idx, lines[line_idx]))

    return context

def format_token_annotation(root: Root, annotation: Annotation):
    """
    Token annotation will find the specific token in the output that the annotation is referring to,
    and highlight it in the following format

    Ex. expr = (.*) token = * note = "this token"
    output =

    "(.*)"
       ^ "this token"

    :param annotation:
    :return:
    """
    pass

def format_expression_annotation(root: Root, annotation: Annotation):
    pass

def format_entity_annotation(root: Root, annotation: Annotation):
    pass

def format_annotation(root: Root, annotation: Annotation):
    pass

def format(output: FileOutput):
    rich.print(f"Expression [bold magenta]{output.root.expression.raw}[/bold magenta] from [blue]{output.root.file}[/blue] line [yellow]{output.root.line_number}[/yellow]")
    relevant_lines = retrieve_program_context(output.root)
    for line_idx, line in relevant_lines:
        if line_idx == output.root.line_number-1:
            rich.print(f"[bold][red]>[/red][gray]{line_idx}[/gray]{line}[/bold]")
        else:
            rich.print(f"|[gray]{line_idx}[/gray]{line}")


def main():
    expr_raw = sys.argv[1]
    output = FileOutput()
    output.ParseFromString(base64.b64decode(expr_raw))
    print(output)
    #format(output)



if __name__ == "__main__":
    main()