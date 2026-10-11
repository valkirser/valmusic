# ValMusic

ValMusic is my fork of [Limusic](https://github.com/SimoHypers/limusic). I made it to add features I
find useful while continuing the original creator's work.

> **Switching back to Limusic:** Some ValMusic changes, such as locally saved song details and
> custom playlist order, may not carry over or look the same in Limusic. Back up playlists you care
> about before editing them in ValMusic.

## Added features

- Edit a song's title, artists, and cover art for a specific playlist. Changes are saved locally
  and update during playback, including in Discord Rich Presence. Choose a primary artist for the
  artist link.
- Download a song, selected songs, or a full playlist. Choose the format and destination folder;
  multiple songs are packaged in a ZIP with download progress. FFmpeg is included for format
  conversion.
- Drag songs into a custom playlist order.
- Create a playlist from an artist's discography, with options to include collaborative albums and
  featured tracks.

## Download

Get ValMusic from [GitHub Releases](https://github.com/valkirser/valmusic/releases/latest).

## Build for Windows

Follow the [Windows build and installer guide](docs/VALMUSIC_BUILD_RELEASE.md).

## Credit and license

ValMusic is based on [Limusic by SimoHypers](https://github.com/SimoHypers/limusic) and is
licensed under [GPL-3.0](LICENSE).
