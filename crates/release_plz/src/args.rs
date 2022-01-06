use anyhow::anyhow;
use release_plz::GitHub;
use secrecy::SecretString;
use url::Url;

#[derive(clap::Parser, Debug)]
#[clap(about, version, author)]
pub struct CliArgs {
    /// GitHub token
    pub github_token: SecretString,
    /// GitHub repository url
    pub repo_url: Url,
}

impl CliArgs {
    pub fn github(&self) -> anyhow::Result<GitHub> {
        let segments = self
            .repo_url
            .path_segments()
            .map(|c| c.collect::<Vec<_>>())
            .ok_or_else(|| {
                anyhow!(
                    "cannot find github owner and repo from url {}",
                    self.repo_url
                )
            })?;
        let owner = segments
            .get(0)
            .ok_or_else(|| anyhow!("cannot find github owner from url {}", self.repo_url))?
            .to_string();
        let repo = segments
            .get(1)
            .ok_or_else(|| anyhow!("cannot find github repo from url {}", self.repo_url))?
            .to_string();
        Ok(GitHub {
            owner,
            repo,
            token: self.github_token.clone(),
        })
    }
}
