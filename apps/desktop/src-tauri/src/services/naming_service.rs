use chrono::Local;

pub struct FileNamingService;

impl FileNamingService {
    pub fn sanitize(input: &str) -> String {
        input
            .chars()
            .map(|c| match c {
                '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
                x if x.is_control() => '_',
                x => x,
            })
            .collect::<String>()
            .replace(' ', "_")
    }

    pub fn with_suffix(stem: &str, suffix: &str) -> String {
        format!("{}_{}.pdf", Self::sanitize(stem), suffix)
    }

    pub fn template(template: &str, map: &[(&str, &str)]) -> String {
        let mut output = template.to_string();
        for (key, value) in map {
            output = output.replace(&format!("{{{key}}}"), value);
        }
        if output.contains("{date}") {
            output = output.replace("{date}", &Local::now().format("%Y-%m-%d").to_string());
        }
        Self::sanitize(&output)
    }
}

#[cfg(test)]
mod tests {
    use super::FileNamingService;

    #[test]
    fn sanitize_unsafe_chars() {
        assert_eq!(FileNamingService::sanitize("a/b:c"), "a_b_c");
    }
}
