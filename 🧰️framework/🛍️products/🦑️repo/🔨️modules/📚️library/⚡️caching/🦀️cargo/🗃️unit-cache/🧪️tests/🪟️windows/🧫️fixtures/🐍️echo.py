"""🐍️ Fixture echoing its arguments as JSON (Windows cargo unit-cache)."""
import json,sys
if len(sys.argv)>1 and sys.argv[1]=="--exit":
    sys.exit(int(sys.argv[2]))
sys.stdout.write(json.dumps(sys.argv[1:],ensure_ascii=False))
