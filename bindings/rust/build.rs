fn main() {
    let src_dir = std::path::Path::new("src");

    #[cfg(feature = "tree-sitter-c2rust")]
    {
        tree_sitter_wasm_build_tool::compile_parser(
            &src_dir.join("parser.c"),
            &src_dir.join("tree_sitter/parser.h"),
        )
        .unwrap();
    }

    #[cfg(not(feature = "tree-sitter-c2rust"))]
    {
        let mut c_config = cc::Build::new();
        c_config.include(src_dir);
        c_config
            .flag_if_supported("-Wno-unused-parameter")
            .flag_if_supported("-Wno-unused-but-set-variable")
            .flag_if_supported("-Wno-trigraphs");
        #[cfg(target_env = "msvc")]
    c_config.flag("-utf-8");

    let parser_path = src_dir.join("parser.c");
        c_config.file(&parser_path);

        c_config.compile("parser");
        println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());
    }
}
