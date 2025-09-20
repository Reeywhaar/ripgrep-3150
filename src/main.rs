fn main() {}

#[cfg(test)]
mod tests {
    use ignore::gitignore::{GitignoreBuilder, gitconfig_excludes_path};

    #[test]
    fn case_with_global() {
        let root = std::env::current_dir().unwrap();
        let mut gitignore_builder = GitignoreBuilder::new(root);
        gitignore_builder.add(".gitignore");
        let gitignore = gitignore_builder.build_global();
        if let Some(err) = gitignore.1 {
            panic!("Failed to parse .gitignore: {err}");
        }
        let gitignore = gitignore.0;
        assert!(
            gitignore.matched("test.txt", false).is_ignore(),
            "test.txt should be ignored according to .gitignore"
        );
        assert!(
            gitignore.matched("test/test.txt", false).is_ignore(),
            "test/test.txt should be ignored according to .gitignore"
        );
    }

    #[test]
    fn case_with_manual_global() {
        let root = std::env::current_dir().unwrap();
        let mut gitignore_builder = GitignoreBuilder::new(root);
        let global_gitignore = gitconfig_excludes_path();
        if let Some(path) = global_gitignore {
            if path.exists() {
                gitignore_builder.add(path).unwrap();
            }
        }
        gitignore_builder.add(".gitignore");
        let gitignore = gitignore_builder.build().unwrap();
        assert!(
            gitignore.matched("test.txt", false).is_ignore(),
            "test.txt should be ignored according to .gitignore"
        );
        assert!(
            gitignore.matched("test/test.txt", false).is_ignore(),
            "test/test.txt should be ignored according to .gitignore"
        );
    }
}
