# rust todo mcp

## Usage

```json
{
  "mcpServers": {
    "todo-mcp": {
      "autoApprove": [
        "create_todo",
        "read_todos"
      ],
      "disabled": false,
      "timeout": 60,
      "command": "/workspace/supabase-demo/rs-todo-mcp/target/debug/rs-todo-mcp",
      "args": [],
      "env": {
        "API_URL": "http://localhost:8000/rest/v1/todos",
        "API_KEY": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyAgCiAgICAicm9sZSI6ICJzZXJ2aWNlX3JvbGUiLAogICAgImlzcyI6ICJzdXBhYmFzZS1kZW1vIiwKICAgICJpYXQiOiAxNjQxNzY5MjAwLAogICAgImV4cCI6IDE3OTk1MzU2MDAKfQ.DaYlNEoUrrEn2Ig7tqibS-PHK5vgusbcbo7X36XVt4Q"
      },
      "transportType": "stdio"
    }
  }
}
```