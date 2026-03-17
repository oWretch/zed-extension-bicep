#!/usr/bin/env python3
"""
Bridge script for the Bicep Language Server.

The Zed extension API communicates with language servers via stdin/stdout.
This script creates a TCP socket server, starts the Bicep language server
with the --socket flag (so it connects to that TCP socket), then bridges
Zed's stdin/stdout to the TCP connection. This allows the Bicep language
server to use TCP sockets for LSP communication rather than stdin/stdout,
which may provide better performance.

Usage: bicep-lsp-bridge.py <dotnet_path> [dotnet_args...] <lsp_dll_path>
The script appends --socket <port> to the command before launching it.
"""

import socket
import sys
import threading
import subprocess


def main():
    if len(sys.argv) < 2:
        print(
            "Usage: bicep-lsp-bridge.py <dotnet_args...>",
            file=sys.stderr,
        )
        sys.exit(1)

    # Create a TCP server socket bound to a random free port on loopback
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    server.bind(("127.0.0.1", 0))
    port = server.getsockname()[1]
    server.listen(1)

    # Build the command: append --socket <port> to the provided arguments
    # so the Bicep language server connects back to our TCP socket
    cmd = sys.argv[1:] + ["--socket", str(port)]

    # Start the Bicep language server subprocess
    proc = subprocess.Popen(
        cmd,
        stdin=subprocess.DEVNULL,
        stdout=subprocess.DEVNULL,
        # Inherit stderr so Bicep language server diagnostics are visible
        # in Zed's logs for debugging purposes.
    )

    # Accept the incoming TCP connection from the Bicep language server
    try:
        server.settimeout(30)
        conn, _ = server.accept()
    except socket.timeout:
        print(
            "Timed out waiting for the Bicep language server to connect",
            file=sys.stderr,
        )
        proc.terminate()
        sys.exit(1)
    finally:
        server.close()

    # Forward data from Zed's stdin to the TCP socket (language server input)
    def forward_stdin_to_socket():
        try:
            while True:
                data = sys.stdin.buffer.read(65536)
                if not data:
                    break
                conn.sendall(data)
        except OSError:
            pass
        try:
            conn.shutdown(socket.SHUT_WR)
        except OSError:
            pass

    stdin_thread = threading.Thread(target=forward_stdin_to_socket, daemon=True)
    stdin_thread.start()

    # Forward data from the TCP socket to Zed's stdout (language server output)
    try:
        while True:
            data = conn.recv(65536)
            if not data:
                break
            sys.stdout.buffer.write(data)
            sys.stdout.buffer.flush()
    except OSError:
        pass

    proc.wait()


if __name__ == "__main__":
    main()
