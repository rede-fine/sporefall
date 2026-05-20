from __future__ import annotations

import argparse
import json
import sqlite3
from datetime import datetime, timezone
from http import HTTPStatus
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import Any

DEFAULT_HOST = "127.0.0.1"
DEFAULT_PORT = 8787
DEFAULT_DB_PATH = Path("runtime-data") / "leaderboard.sqlite3"
MAX_ENTRIES = 10
MAX_NAME_LENGTH = 24


def initialize_database(db_path: Path) -> None:
    db_path.parent.mkdir(parents=True, exist_ok=True)
    with sqlite3.connect(db_path) as connection:
        connection.execute(
            """
            CREATE TABLE IF NOT EXISTS leaderboard_entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                player_name TEXT NOT NULL,
                score INTEGER NOT NULL CHECK(score >= 0),
                recorded_at TEXT NOT NULL
            )
            """
        )
        connection.commit()


def fetch_entries(connection: sqlite3.Connection) -> list[dict[str, Any]]:
    rows = connection.execute(
        """
        SELECT id, player_name, score, recorded_at
        FROM leaderboard_entries
        ORDER BY score DESC, recorded_at ASC, id ASC
        LIMIT ?
        """,
        (MAX_ENTRIES,),
    ).fetchall()

    return [
        {
            "id": row["id"],
            "player_name": row["player_name"],
            "score": row["score"],
            "recorded_at": row["recorded_at"],
        }
        for row in rows
    ]


def insert_entry(connection: sqlite3.Connection, player_name: str, score: int) -> dict[str, Any]:
    recorded_at = datetime.now(timezone.utc).isoformat(timespec="seconds").replace("+00:00", "Z")
    cursor = connection.execute(
        """
        INSERT INTO leaderboard_entries (player_name, score, recorded_at)
        VALUES (?, ?, ?)
        """,
        (player_name, score, recorded_at),
    )
    connection.commit()

    inserted_id = cursor.lastrowid
    entries = fetch_entries(connection)
    rank = next((index + 1 for index, entry in enumerate(entries) if entry["id"] == inserted_id), None)

    return {
        "accepted": rank is not None,
        "rank": rank,
        "entries": entries,
    }


def open_database(db_path: Path) -> sqlite3.Connection:
    connection = sqlite3.connect(db_path)
    connection.row_factory = sqlite3.Row
    return connection


class LeaderboardRequestHandler(BaseHTTPRequestHandler):
    db_path: Path

    def log_message(self, format: str, *args: Any) -> None:
        print(f"{self.address_string()} - {format % args}")

    def end_headers(self) -> None:
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "Content-Type")
        self.send_header("Cache-Control", "no-store")
        super().end_headers()

    def do_OPTIONS(self) -> None:
        self.send_response(HTTPStatus.NO_CONTENT)
        self.end_headers()

    def do_GET(self) -> None:
        route = self.path.split("?", 1)[0]
        if route == "/api/health":
            self._write_json(HTTPStatus.OK, {"status": "ok"})
            return

        if route == "/api/leaderboard":
            with open_database(self.db_path) as connection:
                self._write_json(HTTPStatus.OK, {"entries": fetch_entries(connection)})
            return

        self._write_json(HTTPStatus.NOT_FOUND, {"error": f"Unknown route: {route}"})

    def do_POST(self) -> None:
        route = self.path.split("?", 1)[0]
        if route != "/api/leaderboard":
            self._write_json(HTTPStatus.NOT_FOUND, {"error": f"Unknown route: {route}"})
            return

        try:
            payload = self._read_json()
        except ValueError as error:
            self._write_json(HTTPStatus.BAD_REQUEST, {"error": str(error)})
            return

        player_name = str(payload.get("player_name", "")).strip()
        score = payload.get("score")

        if not player_name:
            self._write_json(HTTPStatus.BAD_REQUEST, {"error": "player_name is required"})
            return

        if len(player_name) > MAX_NAME_LENGTH:
            self._write_json(
                HTTPStatus.BAD_REQUEST,
                {"error": f"player_name must be at most {MAX_NAME_LENGTH} characters"},
            )
            return

        if not isinstance(score, int) or score < 0:
            self._write_json(HTTPStatus.BAD_REQUEST, {"error": "score must be a non-negative integer"})
            return

        with open_database(self.db_path) as connection:
            result = insert_entry(connection, player_name, score)
        self._write_json(HTTPStatus.CREATED, result)

    def _read_json(self) -> dict[str, Any]:
        content_length = self.headers.get("Content-Length")
        if content_length is None:
            raise ValueError("Content-Length header is required")

        raw_body = self.rfile.read(int(content_length))
        try:
            payload = json.loads(raw_body.decode("utf-8"))
        except json.JSONDecodeError as error:
            raise ValueError("Request body must be valid JSON") from error

        if not isinstance(payload, dict):
            raise ValueError("Request body must be a JSON object")

        return payload

    def _write_json(self, status: HTTPStatus, payload: dict[str, Any]) -> None:
        body = json.dumps(payload, ensure_ascii=True).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Run the local Sporefall leaderboard service.")
    parser.add_argument("--host", default=DEFAULT_HOST, help=f"Host to bind (default: {DEFAULT_HOST})")
    parser.add_argument("--port", default=DEFAULT_PORT, type=int, help=f"Port to bind (default: {DEFAULT_PORT})")
    parser.add_argument(
        "--db",
        default=str(DEFAULT_DB_PATH),
        help=f"Path to the SQLite database file (default: {DEFAULT_DB_PATH})",
    )
    return parser


def main() -> None:
    arguments = build_parser().parse_args()
    db_path = Path(arguments.db)
    initialize_database(db_path)

    server = ThreadingHTTPServer((arguments.host, arguments.port), LeaderboardRequestHandler)
    server.RequestHandlerClass.db_path = db_path

    print(
        f"Sporefall leaderboard service listening on http://{arguments.host}:{arguments.port} "
        f"using {db_path}"
    )
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nStopping leaderboard service.")
    finally:
        server.server_close()


if __name__ == "__main__":
    main()
