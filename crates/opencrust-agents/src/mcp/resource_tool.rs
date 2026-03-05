use std::sync::Arc;

use async_trait::async_trait;
use opencrust_common::Result;
use serde_json::Value;

use super::manager::McpManager;
use crate::tools::{Tool, ToolContext, ToolOutput};

/// Exposes MCP resources as an on-demand tool so the LLM can list and read
/// resources without bloating the system prompt.
pub struct McpResourceTool {
    manager: Arc<McpManager>,
}

impl McpResourceTool {
    pub fn new(manager: Arc<McpManager>) -> Self {
        Self { manager }
    }
}

#[async_trait]
impl Tool for McpResourceTool {
    fn name(&self) -> &str {
        "mcp_resources"
    }

    fn description(&self) -> &str {
        "List and read resources from MCP servers. Use action 'list' to see available resources, or action 'read' with a server_name and uri to read a specific resource."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["list", "read"],
                    "description": "Action to perform: 'list' to see available resources, 'read' to fetch a specific resource"
                },
                "server_name": {
                    "type": "string",
                    "description": "Name of the MCP server (required for 'read')"
                },
                "uri": {
                    "type": "string",
                    "description": "URI of the resource to read (required for 'read')"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, _context: &ToolContext, input: Value) -> Result<ToolOutput> {
        let action = input
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("list");

        match action {
            "list" => {
                let all_resources = self.manager.list_all_resources().await;
                if all_resources.is_empty() {
                    return Ok(ToolOutput::success(
                        "No resources available from any MCP server.".to_string(),
                    ));
                }

                let mut output = String::new();
                for (server_name, resources) in &all_resources {
                    output.push_str(&format!("## {server_name}\n"));
                    for r in resources {
                        output.push_str(&format!("- **{}** ({})", r.name, r.uri));
                        if let Some(desc) = &r.description {
                            output.push_str(&format!(" - {desc}"));
                        }
                        if let Some(mime) = &r.mime_type {
                            output.push_str(&format!(" [{mime}]"));
                        }
                        output.push('\n');
                    }
                    output.push('\n');
                }
                Ok(ToolOutput::success(output))
            }
            "read" => {
                let server_name = match input.get("server_name").and_then(|v| v.as_str()) {
                    Some(s) => s,
                    None => {
                        return Ok(ToolOutput::error(
                            "Missing required parameter 'server_name' for read action".to_string(),
                        ));
                    }
                };
                let uri = match input.get("uri").and_then(|v| v.as_str()) {
                    Some(s) => s,
                    None => {
                        return Ok(ToolOutput::error(
                            "Missing required parameter 'uri' for read action".to_string(),
                        ));
                    }
                };

                match self.manager.read_resource(server_name, uri).await {
                    Ok(content) => Ok(ToolOutput::success(content)),
                    Err(e) => Ok(ToolOutput::error(format!("Failed to read resource: {e}"))),
                }
            }
            other => Ok(ToolOutput::error(format!(
                "Unknown action '{other}'. Use 'list' or 'read'."
            ))),
        }
    }
}
