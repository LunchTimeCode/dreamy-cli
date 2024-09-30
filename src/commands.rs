use crate::config::GH_COMMAND;
use crate::server;

use super::gh;
use super::init;
use clap::{Parser, Subcommand};

pub async fn figure() -> anyhow::Result<(String, bool)> {
    let cli = Cli::parse();

    let result: anyhow::Result<String> = match cli.command {
        Some(Commands::GlobalDeps {
            token,
            org,
            repos_path,
            ashtml,
            html_type,
            command,
        }) => match command {
            Some(GlobalSubCommands::Server {
                port,
                schedule,
            }) => {
                server::start_server(
                    port,
                    &token,
                    org,
                    repos_path,
                    ashtml,
                    html_type.unwrap_or(HtmlType::Dependencies),
                    schedule,
                )
                .await
            }
            None => {
                gh::get_deps_global(
                    &token,
                    org,
                    repos_path,
                    ashtml,
                    html_type.unwrap_or(HtmlType::Dependencies),
                )
                .await
            }
        },
        Some(Commands::GhCommand {}) => Ok(GH_COMMAND.to_string()),
        Some(Commands::InitGlobal {}) => init::global_example(),
        Some(Commands::Init {}) => init::example(),
        Some(Commands::Markdown) => Ok(clap_markdown::help_markdown::<Cli>()),
        Some(Commands::Check { token, org, repo }) => {
            gh::check_licenses_on(&token, org, repo).await
        }
        Some(Commands::Deps { token, org, repo }) => gh::get_deps(&token, org, repo).await,
        None => Ok("try dy --help for information on how to use dreamy".to_string()),
    };

    match result {
        Ok(o) => Ok((o, cli.raw)),
        Err(err) => Err(err),
    }
}

/// dreamy cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about, name = "dy")]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    raw: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
#[command(rename_all = "snake_case")]
enum Commands {
    /// [STABLE] print markdown doc of qwit to std out
    Markdown,

    /// [STABLE] creates an example config
    Init {},
    /// [STABLE] creates an global example config
    InitGlobal {},

    /// [PREVIEW] checks licenses on github
    Check {
        #[arg(short, long, env = "GITHUB_TOKEN")]
        token: String,

        #[arg(short, long, env = "DY_ORG")]
        org: Option<String>,

        #[arg(short, long, env = "DY_REPO")]
        repo: Option<String>,
    },
    /// [PREVIEW] get all deps of an repo
    Deps {
        #[arg(short, long, env = "GITHUB_TOKEN")]
        token: String,

        #[arg(short, long, env = "DY_ORG")]
        org: Option<String>,

        #[arg(short, long, env = "DY_REPO")]
        repo: Option<String>,
    },

    /// [STABLE] gh cli command to get all repos
    GhCommand {},

    /// [STABLE] get all deps of an github organisation
    GlobalDeps {
        #[arg(short, long, env = "GITHUB_TOKEN")]
        token: String,

        /// [STABLE] github organisation
        #[arg(short, long, env = "DY_ORG")]
        org: Option<String>,

        /// [STABLE] path to a json file with all repositories to scrape [default: repos.json]
        #[arg(short, long, env = "DY_REPOS_PATH")]
        repos_path: Option<String>,

        /// [PREVIEW] render as html
        #[arg(short, long, default_value_t = false)]
        ashtml: bool,

        /// [PREVIEW] render licenses or dependencies [default: dependencies]
        #[arg(short = 'H', long)]
        html_type: Option<HtmlType>,

        #[command(subcommand)]
        command: Option<GlobalSubCommands>,
    },
}

#[derive(Subcommand, Debug)]
#[command(rename_all = "snake_case")]
enum GlobalSubCommands {
    Server {
        #[arg(short, long, env = "DY_PORT", default_value_t = String::from("3000"))]
        port: String,

        ///Poll github every, seconds: []s, minutes: []m, hours: []h, days: []D, weeks: []W, months: []M. Examples: "30s", "2W", "2h"
        #[arg(short, long, env = "DY_SCHEDULE", default_value_t = String::from("1m"))]
        schedule: String,
    },
}

#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum HtmlType {
    Licenses,
    Dependencies,
}
