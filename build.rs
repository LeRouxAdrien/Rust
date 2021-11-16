fn main() {
    let mdbook_files = skeptic::markdown_files_of_directory("docs/guides/");
    skeptic::generate_doc_tests(&mdbook_files);
}
