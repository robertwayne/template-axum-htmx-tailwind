use std::process::Command;

fn main() {
    std::fs::create_dir_all("build").expect("failed to create build directory");

    Command::new("bunx")
        .args([
            "@tailwindcss/cli",
            "-i",
            "assets/styles/index.css",
            "-o",
            "build/index.css",
            "--content",
            "./templates/**/*.html,./assets/scripts/**/*.ts",
        ])
        .status()
        .expect("failed to run tailwindcss");

    Command::new("bun")
        .args([
            "build",
            "--minify",
            "--outdir=build",
            "--entry-naming",
            "[name].[hash].[ext]",
            "--asset-naming",
            "[name].[hash].[ext]",
            "./assets/scripts/index.ts",
        ])
        .status()
        .expect("failed to run bun");

    copy_files("public");
}

fn copy_files(dir: &str) {
    for entry in std::fs::read_dir(dir).expect("failed to read dir `public`") {
        let entry = entry.expect("failed to read entry");

        if entry.file_type().unwrap().is_dir() {
            copy_files(entry.path().to_str().unwrap());
        } else {
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            let dest = format!("build/{}", filename);

            std::fs::copy(path, dest).expect("failed to copy file");
        }
    }
}
