import configparser
import json
import re
import subprocess
import sys
from pathlib import Path

CLASS_OF = {"replica": "interactive", "cache": "interactive", "relay": "standard", "compact": "background"}
IONICE = {"best-effort": 2, "idle": 3}


def classes(schema_path):
    return json.loads(Path(schema_path).read_text())["definitions"]["Classes"]["const"]


def check(unit_text, script_text, service, schema_path):
    problems = []
    parser = configparser.ConfigParser(strict=False, interpolation=None, delimiters=("=",))
    parser.optionxform = str
    parser.read_string(unit_text)
    section = parser["Service"]
    for key in ("Nice", "CPUSchedulingPolicy", "IOSchedulingClass", "IOSchedulingPriority", "EnvironmentFile", "WorkingDirectory"):
        if key in section:
            problems.append(f"the fixed unit must not carry {key}")
    if section.get("AmbientCapabilities") != "CAP_SYS_NICE":
        problems.append("AmbientCapabilities=CAP_SYS_NICE required")
    start = section.get("ExecStart", "")
    if not re.fullmatch(r'/bin/sh "[^"]*/entry/service/' + service + r'\.sh"', start):
        problems.append(f"ExecStart must run the launch script of {service}: {start}")
    lines = script_text.splitlines()
    if not lines or lines[0] != "#!/bin/sh":
        problems.append("shebang required")
    if subprocess.run(["sh", "-n"], input=script_text, text=True, capture_output=True).returncode != 0:
        problems.append("sh -n rejects the launch script")
    declared = classes(schema_path)[CLASS_OF[service]]
    if f"renice -n {declared['nice']} -p $$ >/dev/null 2>&1 || :" not in lines:
        problems.append("renice line of the declared class missing")
    io = declared["linux"]
    wanted = f"ionice -c {IONICE[io['ioSchedulingClass']]}" + (f" -n {io['ioSchedulingPriority']}" if "ioSchedulingPriority" in io else "") + " -p $$ >/dev/null 2>&1 || :"
    if wanted not in lines:
        problems.append("ionice line of the declared class missing")
    if (io["cpuSchedulingPolicy"] == "idle") != ("chrt --idle -p 0 $$ >/dev/null 2>&1 || :" in lines):
        problems.append("chrt line must exist exactly for the idle policy")
    if not re.match(r"^(cd .* && )?exec ", lines[-1]):
        problems.append("the last line must exec the service")
    return problems


if __name__ == "__main__":
    if len(sys.argv) == 6 and sys.argv[1] == "check":
        found = check(Path(sys.argv[2]).read_text(), Path(sys.argv[3]).read_text(), sys.argv[4], sys.argv[5])
        print(json.dumps(found))
        sys.exit(1 if found else 0)
    sys.exit(2)
