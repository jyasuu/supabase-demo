
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: i32,
    pub user_id: String,
    pub task: String,
    pub is_complete: bool,
    pub inserted_at: String,
}


pub async fn create_todo(task: &str) -> Result<()> {
    
    let API_URL = {
        match std::env::var("API_URL") {
            Ok(url) => Box::leak(url.into_boxed_str()),
            Err(_) => "http://localhost:8000/rest/v1/todos",
        }
    };
    let API_KEY= {
        match std::env::var("API_KEY") {
            Ok(key) => Box::leak(key.into_boxed_str()),
            Err(_) => "SUPABASE_KEY",
        }
    };
    let client = reqwest::Client::new();
    let response = client
        .post(API_URL)
        .header("apikey", API_KEY)
        .header("Authorization", format!("Bearer {}", API_KEY))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=representation")
        .body(serde_json::json!({ "task": task,"user_id": "d85345e0-eca0-4d11-963e-d143016c33dd" }).to_string())
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(anyhow!("Create todo failed with status: {}", response.status()))
    }
}

pub async fn read_todos() -> Result<Vec<Todo>> {
    
    let API_URL = {
        match std::env::var("API_URL") {
            Ok(url) => Box::leak(url.into_boxed_str()),
            Err(_) => "http://localhost:8000/rest/v1/todos",
        }
    };
    let API_KEY= {
        match std::env::var("API_KEY") {
            Ok(key) => Box::leak(key.into_boxed_str()),
            Err(_) => "SUPABASE_KEY",
        }
    };

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}?select={}", API_URL, "*") )
        .header("apikey", API_KEY)
        .header("Authorization", format!("Bearer {}", API_KEY))
        .header("Prefer", "return=representation")
        .send()
        .await?;

    if response.status().is_success() {
        let todos: Vec<Todo> = response.json().await?;
        Ok(todos)
    } else {
        Err(anyhow!("Read todos failed with status: {}", response.status()))
    }
}

pub async fn update_todo(id: i32, task: &str, is_complete: bool) -> Result<Todo> {
    let API_URL = {
        match std::env::var("API_URL") {
            Ok(url) => Box::leak(url.into_boxed_str()),
            Err(_) => "http://localhost:8000/rest/v1/todos",
        }
    };
    let API_KEY= {
        match std::env::var("API_KEY") {
            Ok(key) => Box::leak(key.into_boxed_str()),
            Err(_) => "SUPABASE_KEY",
        }
    };

    let client = reqwest::Client::new();
    let url = format!("{}?id=eq.{}", API_URL, id);
    let response = client
        .patch(url)
        .header("apikey", API_KEY)
        .header("Authorization", format!("Bearer {}", API_KEY))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=representation")
        .body(serde_json::json!({ "task": task, "is_complete": is_complete }).to_string())
        .send()
        .await?;

    if response.status().is_success() {
        let todo: Todo = response.json().await?;
        Ok(todo)
    } else {
        Err(anyhow!("Update todo failed with status: {}", response.status()))
    }
}

pub async fn delete_todo(id: i32) -> Result<()> {
    
    let API_URL = {
        match std::env::var("API_URL") {
            Ok(url) => Box::leak(url.into_boxed_str()),
            Err(_) => "http://localhost:8000/rest/v1/todos",
        }
    };
    let API_KEY= {
        match std::env::var("API_KEY") {
            Ok(key) => Box::leak(key.into_boxed_str()),
            Err(_) => "SUPABASE_KEY",
        }
    };
    
    let client = reqwest::Client::new();
    let url = format!("{}?id=eq.{}", API_URL, id);
    let response = client
        .delete(url)
        .header("apikey", API_KEY)
        .header("Authorization", format!("Bearer {}", API_KEY))
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(anyhow!("Delete todo failed with status: {}", response.status()))
    }
}

use rmcp::{
    Error as McpError, RoleServer, ServerHandler, const_string, model::*, schemars,
    service::RequestContext, tool,
};
use serde_json::json;


#[derive(Clone)]
pub struct TodoManager {
}
#[tool(tool_box)]
impl TodoManager {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
        }
    }
    

    fn _create_resource_text(&self, uri: &str, name: &str) -> Resource {
        RawResource::new(uri, name.to_string()).no_annotation()
    }

    #[tool(description = "Read all todos")]
    async fn read_todos(&self) -> Result<CallToolResult, McpError> {
        let todos = read_todos().await.unwrap();
        let str = serde_json::to_string(&todos).unwrap();
        
        Ok(CallToolResult::success(vec![Content::text(
            str,
        )]))

    }

    #[tool(description = "Create a todo")]
    async fn create_todo(
        &self,
        #[tool(param)]
        #[schemars(description = "Task description")]
        task: String,
    ) -> Result<CallToolResult, McpError> {
        let res = create_todo(&task).await.unwrap();
        let str = serde_json::to_string(&res).unwrap();

        Ok(CallToolResult::success(vec![Content::text(str)]))
    }

}
const_string!(Echo = "echo");
#[tool(tool_box)]
impl ServerHandler for TodoManager {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides a todo tool that can create and read todos. Use 'read_todos' to get all todos without any filter and use 'create_todo' with specify task description.".to_string()),
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            resources: vec![
                self._create_resource_text("str:////Users/to/some/path/", "cwd"),
                self._create_resource_text("memo://insights", "memo-name"),
            ],
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        ReadResourceRequestParam { uri }: ReadResourceRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        match uri.as_str() {
            "str:////Users/to/some/path/" => {
                let cwd = "/Users/to/some/path/";
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(cwd, uri)],
                })
            }
            "memo://insights" => {
                let memo = "Business Intelligence Memo\n\nAnalysis has revealed 5 key insights ...";
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(memo, uri)],
                })
            }
            _ => Err(McpError::resource_not_found(
                "resource_not_found",
                Some(json!({
                    "uri": uri
                })),
            )),
        }
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        Ok(ListPromptsResult {
            next_cursor: None,
            prompts: vec![Prompt::new(
                "example_prompt",
                Some("This is an example prompt that takes one required argument, message"),
                Some(vec![PromptArgument {
                    name: "message".to_string(),
                    description: Some("A message to put in the prompt".to_string()),
                    required: Some(true),
                }]),
            )],
        })
    }

    async fn get_prompt(
        &self,
        GetPromptRequestParam { name, arguments }: GetPromptRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        match name.as_str() {
            "example_prompt" => {
                let message = arguments
                    .and_then(|json| json.get("message")?.as_str().map(|s| s.to_string()))
                    .ok_or_else(|| {
                        McpError::invalid_params("No message provided to example_prompt", None)
                    })?;

                let prompt =
                    format!("This is an example prompt with your message here: '{message}'");
                Ok(GetPromptResult {
                    description: None,
                    messages: vec![PromptMessage {
                        role: PromptMessageRole::User,
                        content: PromptMessageContent::text(prompt),
                    }],
                })
            }
            _ => Err(McpError::invalid_params("prompt not found", None)),
        }
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(ListResourceTemplatesResult {
            next_cursor: None,
            resource_templates: Vec::new(),
        })
    }
}
