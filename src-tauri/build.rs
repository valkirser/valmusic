use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rerun-if-env-changed=FFMPEG_PATH");
    println!("cargo:rerun-if-env-changed=PATH");
    println!("cargo:rerun-if-changed=resources/ffmpeg");

    // Last.fm API credentials live in a gitignored `lastfm.keys` next to this file (the repo is
    // public — they must never be tracked). Format: `LIMUSIC_LASTFM_API_KEY=…` and
    // `LIMUSIC_LASTFM_API_SECRET=…`, one per line. Exported as compile-time env for `option_env!`
    // in lastfm.rs; missing file just means the scrobbler reports "not configured".
    println!("cargo:rerun-if-changed=lastfm.keys");
    if let Ok(keys) = std::fs::read_to_string("lastfm.keys") {
        for line in keys.lines() {
            if let Some((k, v)) = line.split_once('=') {
                let (k, v) = (k.trim(), v.trim());
                if k == "LIMUSIC_LASTFM_API_KEY" || k == "LIMUSIC_LASTFM_API_SECRET" {
                    println!("cargo:rustc-env={k}={v}");
                }
            }
        }
    }
    // tauri-build watches tauri.conf.json but not the icons it embeds, so editing a PNG here
    // leaves `generate_context!` emitting the old `default_window_icon` (window, tray, taskbar).
    println!("cargo:rerun-if-changed=icons");

    // Tauri packages this folder as an app resource. Release workflows stage their pinned FFmpeg
    // build here; local builds copy the developer's PATH/FFMPEG_PATH executable here so converted
    // downloads work from the installed app without a separate FFmpeg installation.
    stage_ffmpeg();

    tauri_build::build()
}

fn stage_ffmpeg() {
    let executable = if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" };
    let destination = Path::new("resources/ffmpeg").join(executable);
    if destination.is_file() {
        return;
    }

    let source = env::var_os("FFMPEG_PATH")
        .map(PathBuf::from)
        .filter(|path| path.is_file())
        .or_else(|| {
            env::split_paths(&env::var_os("PATH")?).find_map(|directory| {
                let candidate = directory.join(executable);
                candidate.is_file().then_some(candidate)
            })
        });
    let Some(source) = source else {
        println!("cargo:warning=FFmpeg was not found; converted audio formats will be unavailable in this build");
        return;
    };

    if let Err(error) = fs::create_dir_all("resources/ffmpeg")
        .and_then(|_| fs::copy(&source, &destination).map(|_| ()))
    {
        println!("cargo:warning=Could not bundle FFmpeg from {}: {error}", source.display());
        return;
    }
    println!("cargo:rerun-if-changed={}", source.display());
    println!("cargo:warning=Bundled FFmpeg from {}", source.display());
}
