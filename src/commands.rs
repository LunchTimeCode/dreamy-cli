use super::gh;
use super::init;
use clap::{Parser, Subcommand};

pub async fn figure() -> anyhow::Result<String> {
    let cli = Cli::parse();

    let result: anyhow::Result<String> = match cli.command {
        Some(Commands::Init {}) => init::example(),
        Some(Commands::Markdown) => Ok(clap_markdown::help_markdown::<Cli>()),
        Some(Commands::CheckGh {
            token,
            org,
            repo,
            base,
            head,
        }) => gh::check_licenses_on(&token, org, repo, base, head).await,
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

    // [PREVIEW] checks licenses on github
    CheckGh {
        #[arg(short, long, env = "GITHUB_TOKEN")]
        token: String,

        #[arg(short, long, env = "DY_ORG")]
        org: Option<String>,

        #[arg(short, long, env = "DY_REPO")]
        repo: Option<String>,

        #[arg(short, long, env = "DY_BASE")]
        base: Option<String>,

        #[arg(short, long, env = "DY_HEAD")]
        head: Option<String>,
    },
}
