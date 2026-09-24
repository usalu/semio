import ast
import pathlib
import sys

module = ast.parse(pathlib.Path(sys.argv[1]).read_text())
role = "declaration"
for statement in module.body:
    if isinstance(statement, (ast.Import, ast.ImportFrom)):
        continue
    if isinstance(statement, ast.Assign) and all(isinstance(target, ast.Name) and target.id == "__all__" for target in statement.targets):
        continue
    if isinstance(statement, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
        role = "implementation"
        break
    role = "unresolved"
print(role, end="")
