use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=assets");

    std::fs::remove_dir_all("build").unwrap_or_default();

    Command::new("npx")
        .args([
            "tailwindcss",
            "-c",
            "tailwind.config.js",
            "-i",
            "assets/styles/index.css",
            "-o",
            "build/index.css",
            "--minify",
        ])
        .status()
        .expect("failed to run tailwindcss");

    Command::new("esbuild")
        .args([
            "--bundle",
            "--outdir=build",
            "--entry-names=[name].[hash]",
            "assets/scripts/index.ts",
            "build/index.css",
            "--minify",
        ])
        .status()
        .expect("failed to run esbuild");

    std::fs::remove_file("build/index.css").unwrap_or_default();

    for entry in std::fs::read_dir("public").expect("failed to read dir `public`") {
        let entry = entry.expect("failed to read entry");

        let path = entry.path();

        if path.is_file() {
            std::fs::copy(
                path.clone(),
                path.to_str().unwrap().replace("public/", "build/").as_str(),
            )
            .expect("failed to copy file");
        }
    }
}
