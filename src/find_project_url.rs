use crate::constants;

pub enum ProjectName {
    Dawood,
    Quranicity,
    Ai100,
}

impl ProjectName {
    pub fn to_string(&self) -> &str {
        match self {
            ProjectName::Dawood => constants::DAWOOD_PROJECT_NAME,
            ProjectName::Quranicity => constants::QURANICITY_PROJECT_NAME,
            ProjectName::Ai100 => constants::AI100_PROJECT_NAME,
        }
    }
}

pub fn find_project_url(project_name: &ProjectName) -> String {
    let url = format!(
        "{}{}.json",
        constants::DEFAULT_URL,
        project_name.to_string()
    );
    println!("url is {}", url);
    url
}
