#![allow(dead_code)]
pub mod fixtures {
    use std::fs::read_to_string;
    use std::path::PathBuf;

    /// ./fixtures/sample_ruby_file.rb
    pub fn sample_ruby_filename() -> PathBuf {
        fixtures_dir().join("sample_ruby_file.rb")
    }

    pub fn sample_ruby_file_contents() -> String {
        read_to_string(sample_ruby_filename()).unwrap()
    }

    pub fn sample_doc_filename() -> PathBuf {
        fixtures_dir().join("sample_doc.md")
    }

    pub fn sample_doc_contents() -> String {
        read_to_string(sample_doc_filename()).unwrap()
    }

    fn fixtures_dir() -> PathBuf {
        std::env::current_dir()
            .unwrap()
            .join("src/support/fixtures")
    }
}
