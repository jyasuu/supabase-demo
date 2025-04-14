#!/usr/bin/env python3
import os
import json
import requests
from mcp.server.server import Server
from mcp.server.stdio_transport import StdioServerTransport
from mcp.types import (
    CallToolRequestSchema,
    ErrorCode,
    ListToolsRequestSchema,
    McpError,
)

class TodoServer:
    def __init__(self):
        self.server = Server(
            {
                "name": "todo-mcp-server",
                "version": "0.1.0",
            },
            {
                "capabilities": {
                    "resources": {},
                    "tools": {},
                },
            }
        )

        self.base_url = "http://localhost:8000/rest/v1/todos"
        self.api_key = os.environ.get("SUPABASE_API_KEY")
        self.auth_token = os.environ.get("SUPABASE_AUTH_TOKEN")

        self.setup_tool_handlers()

        self.server.onerror = lambda error: print(f"[MCP Error] {error}")

    def setup_tool_handlers(self):
        self.server.setRequestHandler(ListToolsRequestSchema, self.list_tools)
        self.server.setRequestHandler(CallToolRequestSchema, self.call_tool)

    async def list_tools(self, request):
        return {
            "tools": [
                {
                    "name": "get_todos",
                    "description": "Get todos from the API",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "select": {
                                "type": "string",
                                "description": "Columns to select (e.g., id, task, is_complete)",
                            },
                            "filter": {
                                "type": "string",
                                "description": "Filter conditions (e.g., task=eq.MyTask)",
                            },
                            "range_start": {
                                "type": "integer",
                                "description": "Pagination start range",
                            },
                            "range_end": {
                                "type": "integer",
                                "description": "Pagination end range",
                            },
                        },
                    },
                },
                {
                    "name": "insert_todo",
                    "description": "Insert a new todo item",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "task": {
                                "type": "string",
                                "description": "The task to add",
                            },
                            "user_id": {
                                "type": "string",
                                "description": "The user ID",
                            },
                            "is_complete": {
                                "type": "boolean",
                                "description": "Is the task complete?",
                            },
                        },
                        "required": ["task", "user_id"],
                    },
                },
            ]
        }

    async def call_tool(self, request):
        tool_name = request.params.name
        arguments = request.params.arguments

        if tool_name == "get_todos":
            return await self.get_todos(arguments)
        elif tool_name == "insert_todo":
            return await self.insert_todo(arguments)
        else:
            raise McpError(ErrorCode.MethodNotFound, f"Tool not found: {tool_name}")

    async def get_todos(self, arguments):
        select = arguments.get("select", "*")
        filter_param = arguments.get("filter")
        range_start = arguments.get("range_start")
        range_end = arguments.get("range_end")

        url = f"{self.base_url}?select={select}"
        if filter_param:
            url += f"&{filter_param}"

        headers = {
            "apikey": self.api_key,
            "Authorization": f"Bearer {self.auth_token}",
            "Content-Type": "application/json",
        }

        if range_start is not None and range_end is not None:
            headers["Range"] = f"{range_start}-{range_end}"

        try:
            response = requests.get(url, headers=headers)
            response.raise_for_status()
            return {"content": [{"type": "text", "text": json.dumps(response.json(), indent=2)}]}
        except requests.exceptions.RequestException as e:
            raise McpError(ErrorCode.InternalError, f"API request failed: {e}")

    async def insert_todo(self, arguments):
        task = arguments["task"]
        user_id = arguments["user_id"]
        is_complete = arguments.get("is_complete", False)

        url = self.base_url

        headers = {
            "apikey": self.api_key,
            "Authorization": f"Bearer {self.auth_token}",
            "Content-Type": "application/json",
            "Prefer": "return=minimal",
        }

        data = {"task": task, "user_id": user_id, "is_complete": is_complete}

        try:
            response = requests.post(url, headers=headers, data=json.dumps(data))
            response.raise_for_status()
            return {"content": [{"type": "text", "text": "Todo item inserted successfully"}]}
        except requests.exceptions.RequestException as e:
            raise McpError(ErrorCode.InternalError, f"API request failed: {e}")

    async def run(self):
        transport = StdioServerTransport()
        await self.server.connect(transport)
        print("Todo MCP server running on stdio")


if __name__ == "__main__":
    server = TodoServer()
    server.run()
