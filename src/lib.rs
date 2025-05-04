use find_project_url::ProjectName;
use make_backdoor_request::BackdoorResponse;
use reqwest::Error;

mod constants;
mod find_project_url;
mod make_backdoor_request;

pub async fn make_backdoor_request(project_name: &ProjectName) -> Result<bool, Error> {
    let response = make_backdoor_request::make_backdoor_request(project_name)
        .await
        .unwrap_or(BackdoorResponse { succeed: false });
    Ok(response.succeed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let result = make_backdoor_request(&ProjectName::Dawood).await.unwrap();
        assert_eq!(result, true);
    }
}
