use super::gh;
use super::init;
use clap::{Parser, Subcommand};

pub async fn figure() -> anyhow::Result<String> {
    let cli = Cli::parse();

    let result: anyhow::Result<String> = match cli.command {
        Some(Commands::GlobalDeps {
            token,
            org,
            repos_path,
        }) => gh::get_deps_global(&token, org, repos_path).await,
        Some(Commands::InitGlobal {}) => init::global_example(),
        Some(Commands::Init {}) => init::example(),
        Some(Commands::Markdown) => Ok(clap_markdown::help_markdown::<Cli>()),
        Some(Commands::Check { token, org, repo }) => {
            gh::check_licenses_on(&token, org, repo).await
        }
        Some(Commands::Deps { token, org, repo }) => gh::get_deps(&token, org, repo).await,
        None => Ok("try dy --help for information on how to use dreamy".to_string()),
    };

    result
}

/// dreamy cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about, name = "dy")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
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

    /// [PREVIEW] get all deps of an org
    GlobalDeps {
        #[arg(short, long, env = "GITHUB_TOKEN")]
        token: String,

        #[arg(short, long, env = "DY_ORG")]
        org: Option<String>,

        #[arg(short, long, env = "DY_REPOS_PATH")]
        repos_path: Option<String>,
    },
}
