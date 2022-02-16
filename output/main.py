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

def format_annotation(annotation: Annotation):
    if annotation.entity and annotation.entity != '':
        rel_entity = annotation.entity.replace('[', '\\[')
    elif annotation.token.token and annotation.token.token != '':
        rel_entity = annotation.token.token
    elif annotation.expression:
        rel_entity = annotation.expression.raw
    else:
        raise ValueError("Empty annotation")
    rich.print(f"[bold]{rel_entity}[/bold]\n^[green][italic]{annotation.note}[/italic][/green]")


def format(output: FileOutput):
    rich.print(f"Expression [bold magenta]{output.root.expression.raw}[/bold magenta] from [blue]{output.root.file}[/blue] line [yellow]{output.root.line_number}[/yellow]")
    relevant_lines = retrieve_program_context(output.root)
    rich_output = ""
    for line_idx, line in relevant_lines:
        if line_idx == output.root.line_number-1:
            rich_output += f"[bold][red]>[/red][gray]{line_idx}[/gray]{line}[/bold]"
        else:
            rich_output += f"|[gray]{line_idx}[/gray]{line}"
    rich.print(rich_output)

    for module_output in output.outputs:
        for annotation in module_output.annotations:
            format_annotation(annotation)

def main():
    expr_raw = sys.argv[1]
    output = FileOutput()
    output.ParseFromString(base64.b64decode(expr_raw))
    format(output)



if __name__ == "__main__":
    main()