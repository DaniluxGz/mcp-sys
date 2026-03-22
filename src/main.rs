use rmcp::{
    ServerHandler, ServiceExt,
    model::{
        CallToolResult, Content, Implementation,
        ListToolsResult, ProtocolVersion, ServerCapabilities,
        ServerInfo, Tool, PaginatedRequestParams, CallToolRequestParams,
    },
    transport::stdio,
};
use sysinfo::System;


#[derive(Debug, Clone)]
struct McpSys;

impl McpSys {
    fn get_tools() -> Vec<Tool> {
        // Use serde to construct Tool with all fields from JSON — avoids struct field mismatches
        let make_tool = |name: &str, description: &str| -> Tool {
            serde_json::from_value(serde_json::json!({
                "name": name,
                "description": description,
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            }))
            .unwrap()
        };

        vec![
            make_tool("system_stats",   "Get real-time CPU and RAM usage of the system"),
            make_tool("list_processes", "List running processes sorted by RAM usage"),
            make_tool("list_ports",     "List occupied TCP ports and which process is using them"),
        ]
    }

    fn system_stats() -> String {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu_usage: f32 = sys.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>()
            / sys.cpus().len() as f32;

        let total_ram_mb = sys.total_memory() / 1024 / 1024;
        let used_ram_mb  = sys.used_memory()  / 1024 / 1024;
        let ram_percent  = (used_ram_mb as f64 / total_ram_mb as f64) * 100.0;

        format!(
            "System Stats:\nCPU Usage:  {cpu_usage:.1}%\nRAM Usage:  {used_ram_mb} MB / {total_ram_mb} MB ({ram_percent:.1}%)\n"
        )
    }

    fn list_processes() -> String {
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut processes: Vec<_> = sys.processes().values().collect();
        processes.sort_by(|a, b| b.memory().cmp(&a.memory()));

        let mut lines = vec!["Processes (sorted by RAM):".to_string()];
        for proc in processes.iter().take(20) {
            lines.push(format!(
                "  PID {:6} | {:30} | CPU: {:5.1}% | RAM: {} MB",
                proc.pid(),
                proc.name().to_string_lossy(),
                proc.cpu_usage(),
                proc.memory() / 1024 / 1024,
            ));
        }
        lines.join("\n")
    }

    fn list_ports() -> String {
        use std::process::Command;
        let output = Command::new("netstat").args(["-ano"]).output();
        match output {
            Err(e) => format!("Error running netstat: {e}"),
            Ok(out) => {
                let raw = String::from_utf8_lossy(&out.stdout);
                let mut lines = vec!["Listening ports:".to_string()];
                for line in raw.lines() {
                    if line.contains("LISTENING") {
                        lines.push(format!("  {}", line.trim()));
                    }
                }
                if lines.len() == 1 {
                    lines.push("  No LISTENING ports found.".to_string());
                }
                lines.join("\n")
            }
        }
    }
}

impl ServerHandler for McpSys {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation {
                name:    "mcp-sys".into(),
                version: "0.1.0".into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<ListToolsResult, rmcp::ErrorData> {
        Ok(ListToolsResult {
            tools: Self::get_tools(),
            ..Default::default()
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let text = match request.name.as_ref() {
            "system_stats"   => Self::system_stats(),
            "list_processes" => Self::list_processes(),
            "list_ports"     => Self::list_ports(),
            other => format!("Unknown tool: {other}"),
        };
        Ok(CallToolResult::success(vec![Content::text(text)]))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    eprintln!("mcp-sys starting...");

    let transport = stdio();

    eprintln!("transport created, serving...");

    let server = McpSys.serve(transport).await?;

    eprintln!("server running, waiting...");

    // Keep the server alive until it receives a shutdown signal
    server.waiting().await?;

    eprintln!("server shut down cleanly");

    Ok(())
}