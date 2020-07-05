const GITHUB_URL: &str = "https://github.com";
const USER_INDICATOR: char = '@';




pub fn construct_github_url(params: &str) -> String {
    if params.is_empty() {
        return GITHUB_URL.to_string();
    }

    if params.starts_with(USER_INDICATOR) {
        return construct_github_profile_url(&params[1..]);
    }

    // Assume its a page url if nothing else
    construct_github_repo_url(&params)
}



fn construct_github_profile_url(profile: &str) -> String {
    format!("{}/{}", GITHUB_URL, profile)
}



fn construct_github_repo_url(repo: &str) -> String {
    let search_url = format!("{}/{}", GITHUB_URL, repo);

    search_url
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_github_url() {
        let fake_query = "";
        assert_eq!(construct_github_url(fake_query), GITHUB_URL);
    }

    #[test]
    fn test_construct_github_url_query() {
        let fake_query = "0x20F/paris";
        assert_eq!(construct_github_url(fake_query), format!("{}/0x20F/paris", GITHUB_URL));
    }

    #[test]
    fn test_construct_github_url_profile() {
        let fake_query = "@water";
        assert_eq!(construct_github_url(fake_query), format!("{}/water", GITHUB_URL));
    }

    #[test]
    fn test_construct_github_profile_url() {
        let fake_profile = "abcde";
        assert_eq!(construct_github_profile_url(fake_profile), format!("{}/abcde", GITHUB_URL));
    }

    #[test]
    fn test_construct_github_repo_url() {
        let fake_query = "0x20F/paris";
        assert_eq!(construct_github_repo_url(fake_query), format!("{}/0x20F/paris", GITHUB_URL));
    }
}