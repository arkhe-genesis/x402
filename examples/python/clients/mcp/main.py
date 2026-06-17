"""MCP Client Example Entry Point.

Routes to either simple or advanced example based on CLI arguments.

Usage:
    python main.py simple           - Run simple example (wrap_mcp_client_with_payment_from_config)
    python main.py advanced         - Run advanced example (x402MCPClient with manual setup)
"""

import sys

from advanced import run_advanced
from simple import run_simple

if __name__ == "__main__":
    mode = sys.argv[1] if len(sys.argv) > 1 else "simple"

    try:
        if mode == "advanced":
            run_advanced()
        else:
            run_simple()
    except Exception as e:
        print(f"Fatal error: {e}", file=sys.stderr)
        sys.exit(1)
