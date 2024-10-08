use anyhow::Result;
use lapce_plugin::{
    psp_types::{
        lsp_types::{
            request::Initialize, DocumentFilter, DocumentSelector, InitializeParams, MessageType,
            Url,
        },
        Request,
    },
    register_plugin, LapcePlugin, VoltEnvironment, PLUGIN_RPC,
};
use serde_json::Value;
#[derive(Default)]
struct State {}

register_plugin!(State);

macro_rules! string {
    ( $x:expr ) => {
        String::from($x)
    };
}

fn initialize(params: InitializeParams) -> Result<()> {
    let document_selector: DocumentSelector = vec![
        DocumentFilter {
            language: Some(string!("css")),
            pattern: Some(string!("**/*.css")),
            scheme: None,
        },
        DocumentFilter {
            language: Some(string!("scss")),
            pattern: Some(string!("**/*.scss")),
            scheme: None,
        },
    ];

    let mut server_args = vec![string!("--stdio")];

    if let Some(options) = params.initialization_options.as_ref() {
        if let Some(volt) = options.get("volt") {
            if let Some(args) = volt.get("serverArgs") {
                if let Some(args) = args.as_array() {
                    if !args.is_empty() {
                        server_args = vec![];
                    }
                    for arg in args {
                        if let Some(arg) = arg.as_str() {
                            server_args.push(string!(arg));
                        }
                    }
                }
            }

            if let Some(server_path) = volt.get("serverPath") {
                if let Some(server_path) = server_path.as_str() {
                    if !server_path.is_empty() {
                        let server_uri = Url::parse(&format!("urn:{}", server_path)).unwrap();
                        PLUGIN_RPC
                            .start_lsp(
                                server_uri,
                                server_args,
                                document_selector,
                                params.initialization_options,
                            )
                            .unwrap();
                        return Ok(());
                    }
                }
            }
        }
    }

    let server_uri = match VoltEnvironment::operating_system().as_deref() {
        Ok("windows") => Url::parse(&format!("urn:{}", "vscode-css-language-server.cmd")).unwrap(),
        _ => Url::parse(&format!("urn:{}", "vscode-css-language-server")).unwrap(),
    };
    PLUGIN_RPC
        .start_lsp(
            server_uri,
            server_args,
            document_selector,
            params.initialization_options,
        )
        .unwrap();

    Ok(())
}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        #[allow(clippy::single_match)]
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                if let Err(e) = initialize(params) {
                    PLUGIN_RPC.stderr(&format!("plugin returned with error: {e}"))
                }
            }
            _ => {}
        }
    }
}
