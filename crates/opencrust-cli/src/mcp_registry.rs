/// Known MCP server registry for guided `opencrust mcp add`.
pub struct KnownMcpServer {
    pub id: &'static str,
    pub display_name: &'static str,
    pub description: &'static str,
    pub command: &'static str,
    pub args: &'static [&'static str],
    pub transport: &'static str,
    pub required_env: &'static [EnvRequirement],
    pub setup_instructions: &'static str,
}

pub struct EnvRequirement {
    pub key: &'static str,
    pub description: &'static str,
    pub is_secret: bool,
}

pub static KNOWN_MCP_SERVERS: &[KnownMcpServer] = &[
    KnownMcpServer {
        id: "notion",
        display_name: "Notion",
        description: "Read and write Notion pages, databases, and blocks",
        command: "npx",
        args: &["-y", "@notionhq/notion-mcp-server"],
        transport: "stdio",
        required_env: &[EnvRequirement {
            key: "NOTION_API_TOKEN",
            description: "Go to developers.notion.com > Create integration > Copy the token (starts with secret_...)\nThen share the pages/databases you want accessible with the integration.",
            is_secret: true,
        }],
        setup_instructions: "Requires Node.js installed.",
    },
    KnownMcpServer {
        id: "monday",
        display_name: "Monday.com",
        description: "Manage Monday.com boards, items, and workflows",
        command: "npx",
        args: &["-y", "@mondaycom/monday-mcp-server"],
        transport: "stdio",
        required_env: &[EnvRequirement {
            key: "MONDAY_API_TOKEN",
            description: "Go to monday.com > Avatar > Admin > API > Copy your API token.",
            is_secret: true,
        }],
        setup_instructions: "Requires Node.js installed.",
    },
    KnownMcpServer {
        id: "github",
        display_name: "GitHub",
        description: "Access repos, issues, PRs, and code search",
        command: "npx",
        args: &["-y", "@modelcontextprotocol/server-github"],
        transport: "stdio",
        required_env: &[EnvRequirement {
            key: "GITHUB_PERSONAL_ACCESS_TOKEN",
            description: "Go to github.com/settings/tokens > Generate new token (classic) > Select scopes as needed.",
            is_secret: true,
        }],
        setup_instructions: "Requires Node.js installed.",
    },
    KnownMcpServer {
        id: "filesystem",
        display_name: "Filesystem",
        description: "Read and write files on disk (scoped to allowed paths)",
        command: "npx",
        args: &["-y", "@modelcontextprotocol/server-filesystem"],
        transport: "stdio",
        required_env: &[],
        setup_instructions: "Requires Node.js installed. You will be prompted for allowed directory paths.",
    },
    KnownMcpServer {
        id: "brave-search",
        display_name: "Brave Search",
        description: "Web search via Brave Search API",
        command: "npx",
        args: &["-y", "@modelcontextprotocol/server-brave-search"],
        transport: "stdio",
        required_env: &[EnvRequirement {
            key: "BRAVE_API_KEY",
            description: "Go to brave.com/search/api > Get API Key > Copy your key.",
            is_secret: true,
        }],
        setup_instructions: "Requires Node.js installed.",
    },
    KnownMcpServer {
        id: "postgres",
        display_name: "PostgreSQL",
        description: "Query and inspect PostgreSQL databases",
        command: "npx",
        args: &["-y", "@modelcontextprotocol/server-postgres"],
        transport: "stdio",
        required_env: &[EnvRequirement {
            key: "POSTGRES_CONNECTION_STRING",
            description: "PostgreSQL connection string, e.g. postgresql://user:password@localhost:5432/dbname",
            is_secret: true,
        }],
        setup_instructions: "Requires Node.js installed.",
    },
];

pub fn find_known_server(id: &str) -> Option<&'static KnownMcpServer> {
    KNOWN_MCP_SERVERS.iter().find(|s| s.id == id)
}
