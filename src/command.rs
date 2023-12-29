use clap::{Parser, Subcommand};
use std::str::FromStr;
use url::Url;

#[derive(Clone, Debug)]
pub struct ObsWebsocket {
    pub hostname: String,
    pub port: u16,
    pub password: Option<String>,
}

impl FromStr for ObsWebsocket {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Url::parse(s) {
            Ok(unvalidated_websocket) => {
                if unvalidated_websocket.scheme() != "obsws" {
                    return Err(
                        "Invalid URL format, use the format obsws://hostname:port/password",
                    );
                }

                let hostname = unvalidated_websocket.host().unwrap().to_string();

                let port =
                    match unvalidated_websocket.port() {
                        Some(port) => port,
                        None => return Err(
                            "Please specify a port in the format obsws://hostname:port/password",
                        ),
                    };

                let password = match unvalidated_websocket.path() {
                    "" => None,
                    _ => {
                        let mut pass = unvalidated_websocket.path().to_string();
                        // Otherwise the `/` part of the password in the URL is included.
                        let _ = pass.remove(0);
                        Some(pass)
                    }
                };

                Ok(ObsWebsocket {
                    hostname,
                    port,
                    password,
                })
            }
            Err(_) => Err("Invalid URL format, use the format obsws://hostname:port/password"),
        }
    }
}

#[derive(Subcommand, Clone, Debug)]
pub enum Replay {
    Start,
    Stop,
    Toggle,
    Save,
}

#[derive(Subcommand, Clone, Debug)]
pub enum VirtualCamera {
    Start,
    Stop,
    Toggle,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Streaming {
    Start,
    Stop,
    Toggle,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Recording {
    Start,
    Stop,
    Toggle,
}

// Define the SceneAction enum for different scene actions
#[derive(Subcommand)]
pub enum SceneAction {
    Preview {
        scene_name: String,
    },
    Get,
    List,
    Switch {
        scene_name: String,
    },
}

#[derive(Subcommand)]
pub enum StudioModeAction {
    Enable,
    Disable,
    Toggle,
    Status
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long)]
    /// The default websocket URL is `obsws://localhost:4455/secret`
    /// if this argument is not provided
    pub websocket: Option<ObsWebsocket>,
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Info,

    Filter {
        command: String,
        source: String,
        filter: String,
    },

    Scene {
        #[clap(subcommand)]
        action: SceneAction,
    },

    SceneCollection {
        switch_placeholder: String, // NOTE: just for args positioning
        scene_collection_name: String,
    },

    SceneItem {
        command: String,
        scene: String,
        source: String,
    },

    #[clap(subcommand)]
    Streaming(Streaming),

    StudioMode {
        #[clap(subcommand)]
        action: StudioModeAction,
    },

    #[clap(subcommand)]
    Recording(Recording),

    #[clap(subcommand)]
    Replay(Replay),

    ToggleMute {
        device: String,
    },

    #[clap(subcommand)]
    VirtualCamera(VirtualCamera)
}
