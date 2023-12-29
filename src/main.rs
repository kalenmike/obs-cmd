mod command;
use command::*;

use clap::Parser;
use obws::{requests::filters::SetEnabled as SetEnabledFilter, Client};
use obws::requests::scene_items::SetEnabled as SetEnabledItem;
use obws::requests::scene_items::Id as IdItem;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let client = match cli.websocket {
        Some(ObsWebsocket {
            hostname,
            port,
            password,
        }) => Client::connect(hostname, port, password).await?,
        None => Client::connect("localhost", 4455, Some("secret")).await?,
    };

    match &cli.command {
        Commands::Scene { action } => match action {
            SceneAction::Preview{ scene_name } => {
                let res = client.scenes().set_current_preview_scene(scene_name).await;
                println!("Switched preview to scene: {:?}", scene_name);
                println!("Result: {:?}", res);
            }
            SceneAction::Get => {
                let active_scene = client.scenes().current_program_scene().await?;

                let studio_scene_result = client.scenes().current_preview_scene().await;
                let studio_scene = match studio_scene_result {
                    Ok(scene) => scene,
                    Err(_) => "".to_string(), // Studio mode is disabled
                };

                let combined_json = json!({
                    "program": active_scene,
                    "preview": studio_scene
                });

                println!("Scenes: {}", combined_json.to_string());
            }
            SceneAction::List => {
                let res = client.scenes().list().await?;
                println!("Scenes: {:?}", res);
            }
            SceneAction::Switch { scene_name } => {
                let res = client.scenes().set_current_program_scene(scene_name).await;
                println!("Switched to scene: {}", scene_name);
                println!("Result: {:?}", res);
            }
        },

        Commands::SceneCollection {
            switch_placeholder,
            scene_collection_name,
        } => {
            // let scene_name = &args[3];
            let res = client.scene_collections().set_current(scene_collection_name).await;
            println!("Set current scene collection: {} {}", switch_placeholder, scene_collection_name);
            println!("Result: {:?}", res);
        }

        Commands::Info => {
            let version = client.general().version().await?;
            println!("Version: {:?}", version);
        }

        Commands::StudioMode { action } => match action {
            StudioModeAction::Enable => {
                let res = client.ui().set_studio_mode_enabled(true).await;
                println!("Enable Studio Mode");
                println!("Result: {:?}", res);
            }
            StudioModeAction::Disable => {
                let res = client.ui().set_studio_mode_enabled(false).await;
                println!("Disable Studio Mode");
                println!("Scene: {:?}", res);
            }
            StudioModeAction::Toggle => {
                let state = client.ui().studio_mode_enabled().await?;
                let res = if state {
                    client.ui().set_studio_mode_enabled(false).await?;
                } else {
                    client.ui().set_studio_mode_enabled(true).await?;
                };
                println!("Toggle Studio Mode: {}", if state { "Disabled" } else { "Enabled" });
                println!("Result: {:?}", res);
            }
            StudioModeAction::Status => {
                let res = client.ui().studio_mode_enabled().await?;
                println!("Studio Mode: {:?}", res);
            }
        },

        Commands::Recording(action) => {
            use Recording::*;
            println!("Recording {:?}", action);

            match action {
                Start => {
                    let res = client.recording().start().await;
                    println!("Recording started");
                    println!("Result: {:?}", res);
                }
                Stop => {
                    let res = client.recording().stop().await;
                    println!("Recording stopped");
                    println!("Result: {:?}", res);
                }
                Toggle => {
                    let res = client.recording().toggle().await;
                    println!("Recording toggled");
                    println!("Result: {:?}", res);
                }
            }
        }

        Commands::Streaming(action) => {
            use Streaming::*;
            println!("Streaming {:?}", action);

            match action {
                Start => {
                    let res = client.streaming().start().await;
                    println!("Streaming started");
                    println!("Result: {:?}", res);
                }
                Stop => {
                    let res = client.streaming().stop().await;
                    println!("Streaming stopped");
                    println!("Result: {:?}", res);
                }
                Toggle => {
                    let res = client.streaming().toggle().await?;
                    println!("Streaming toggled");
                    println!("Result: {:?}", res);
                }
            }
        }

        Commands::VirtualCamera(action) => {
            use VirtualCamera::*;
            println!("VirtualCamera {:?}", action);

            match action {
                Start => {
                    let res = client.virtual_cam().start().await;
                    println!("Result: {:?}", res);
                }
                Stop => {
                    let res = client.virtual_cam().stop().await;
                    println!("Result: {:?}", res);
                }
                Toggle => {
                    let res = client.virtual_cam().toggle().await?;
                    println!("Result: {:?}", res);
                }
            }
        }

        Commands::Replay(action) => {
            use Replay::*;
            println!("Replay {:?}", action);

            match action {
                Start => {
                    let res = client.replay_buffer().start().await;
                    println!("Replay Buffer started");
                    println!("Result: {:?}", res);
                }
                Stop => {
                    let res = client.replay_buffer().stop().await;
                    println!("Replay Buffer stopped");
                    println!("Result: {:?}", res);
                }
                Toggle => {
                    let res = client.replay_buffer().toggle().await?;
                    println!("Replay Buffer toggled");
                    println!("Result: {:?}", res);
                }
                Save => {
                    let res = client.replay_buffer().save().await;
                    println!("Buffer saved");
                    println!("Result: {:?}", res);
                }
            }
        }

        Commands::ToggleMute { device } => {
            println!("Toggling mute on device: {:?}  ", device);

            let res = client.inputs().toggle_mute(device).await;
            println!("Result: {:?}", res);
        }

        Commands::Filter {
            command,
            source,
            filter,
        } => {
            println!("Filter: {:?} {:?} {:?}", command, source, filter);

            let enabled: bool = match command.as_str() {
                "enable" => true,
                "disable" => false,
                "toggle" => !client.filters().get(source, filter).await?.enabled,
                _ => {
                    println!("Invalid filter command: {}", command);
                    return Ok(());
                }
            };
            let res = client
                .filters()
                .set_enabled(SetEnabledFilter {
                    source,
                    filter,
                    enabled,
                })
                .await;
            println!("Result: {:?}", res);
        }

        Commands::SceneItem {
            command,
            scene,
            source,
        } => {
            println!("Scene Item: {:?} {:?} {:?}", command, scene, source);

            // get item_id
            let item_id = client
                          .scene_items()
                          .id(IdItem {
                              scene,
                              source,
                              search_offset: Some(0)
                          })
                          .await?;

            // use item_id in toggle
            let enabled: bool = match command.as_str() {
                "enable" => true,
                "disable" => false,
                "toggle" => !client.scene_items().enabled(scene, item_id).await?,
                _ => {
                    println!("Invalid scene item command: {}", command);
                    return Ok(());
                }
            }; // use item_id in setenabled
            let res = client
                .scene_items()
                .set_enabled(SetEnabledItem {
                    scene,
                    item_id,
                    enabled,
                })
                .await;
            println!("Result: {:?}", res);
        }

    }

    Ok(())
}
