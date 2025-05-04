use crate::find_project_url::{ProjectName, find_project_url};
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BackdoorResponse {
    pub succeed: bool,
}

pub async fn make_backdoor_request(project_name: &ProjectName) -> Result<BackdoorResponse, Error> {
    let url = find_project_url(project_name);
    let response = reqwest::get(url).await?;
    let body = response.json::<BackdoorResponse>().await?;
    Ok(body)
}
