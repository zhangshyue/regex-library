"""
Entrypoint for the whole library
"""
# Builtin imports
import argparse
import subprocess
import os
import base64

# Internal imports
from protobuf.python_out.root_pb2 import *
# from output.main import format as output_formatter

PATH = "/usr/local/Cellar/regexAnalyzer/1.0.6/bin"

def run_rust_module(module: str, args: str):
    # Sanity check - make sure the module exists, it's a rust project, and it's been built for release
    assert os.path.isdir(PATH + "/" + module) and \
            os.path.isfile(PATH + f"/{module}/Cargo.toml") and \
           os.path.isdir(PATH + f"/{module}/target/release")
    output = subprocess.check_output([PATH + f"/{module}/target/release/runner", args])
    return output.decode()


_PYTHON_ENTRYPOINTS = {
    "output": "main.py",
    "parser": "main.py",
    "security": "genetic.py",
    "understandability": "understandability.py"
}


def run_python_module(module: str, args: str):
    curr_dir = os.getcwd()
    # Run the process
    module_entrypoint = _PYTHON_ENTRYPOINTS[module]
    output = subprocess.check_output(["python3", PATH + f"/{module}/{module_entrypoint}", args])
    return output.decode()


def extraction(entrypoint):
    output = run_rust_module("extraction", entrypoint)
    return [i for i in output.split("\n") if i]


def parser(expression):
    return run_python_module("parser", expression).split('\n')[0]


def generalizability(expression):
    return run_rust_module("generalizability", expression).split('\n')[0]


def understandability(expression):
    return run_python_module("understandability", expression).split("\n")[0]


def security(expression):
    return run_python_module("security", expression).split("\n")[0]




def run_library(entrypoint: str, run_security: bool):
    """
    Run every step of the pipeline

    """
    modules = [understandability, generalizability]
    if run_security:
        modules.append(security)

    assert os.path.isdir(entrypoint)
    # Run extraction
    extracted_expressions = extraction(entrypoint)

    for expr_raw in extracted_expressions:
        # Parse each expression
        parsed_expression = parser(expr_raw)
        # Create PB types for it
        expr_root = Root()
        expr_root.ParseFromString(base64.b64decode(parsed_expression))
        file_output = FileOutput()
        file_output.root.CopyFrom(expr_root)
        # Run each module component on it
        for module in modules:
            # Execute the module
            encoded_output = module(parsed_expression)
            # Decode its output and add it to the file output
            module_output = Output()
            module_output.ParseFromString(base64.b64decode(encoded_output))
            new_output = file_output.outputs.add()
            new_output.CopyFrom(module_output)
        # output_formatter(file_output)
        expr_raw = base64.b64encode(file_output.SerializeToString()).decode('utf-8')
        output = FileOutput()
        output.ParseFromString(base64.b64decode(expr_raw))
        print(output)



def main():
    """
    Parse command line arguments and run the library
    """
    arg_parser = argparse.ArgumentParser(description="Run the regex library on a directory or file")
    arg_parser.add_argument("entrypoint", type=str, help="The file or directory to run the library on")
    arg_parser.add_argument("--security", action="store_true", help="Run the security component as well")
    args = arg_parser.parse_args()
    entrypoint = args.entrypoint
    run_library(entrypoint, bool(args.security))


if __name__ == "__main__":
    main()