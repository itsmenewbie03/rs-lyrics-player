# LyricsPlayeRS 🎵 [![author/maintainer](https://img.shields.io/badge/by-itsmenewbie03-016eea.svg?logo=github&labelColor=181717&longCache=true&style=flat-square)](https://itsmenewbie03.is-a.dev)
Play song along with lyrics directly in your terminal with **LyricsPlayerRS**.

> [!NOTE]
> This project is meant for personal use only. The code is written when I started learning Rust.

Like this project? **Leave a star**! ⭐⭐⭐⭐⭐

## ⚙️ Requirements

You must have the following installed in your system
- **[Rust 🦀](https://www.rust-lang.org/tools/install)**
- **[FFmpeg](https://ffmpeg.org/download.html)**

## 🛠️ Setup

After you successfully installed all the requirements, clone this repo with
```shell
git clone https://github.com/itsmenewbie03/rs-lyrics-player
cd rs-lyrics-player
```
After cloning perform the following steps
- Add the music you want to play in `res` directory name it `music.mp3`.
- Add the lyrics file that matches the music also in the `res` directory and name it `lyrics.txt`
> [!TIP]
> You can find a lyrics by searching in google `song title lrc`<br>
> e.g., **Sweet Scar lrc**

Usually you will find the lyrics in the following format
```lrc
[ar:Weird Genius]
[ti:Sweet Scar]
[length:03:28.63]
[re:www.megalobiz.com/lrc/maker]
[ve:v1.2.3]
[00:21.56]I can't throw away the pieces that you gave
[00:28.02]Betrayed in a way
[00:30.74]Of a knife stuck on my back
[00:34.25]How could you do this, we've been through this for a while 
```
Remove all the meta tags and add the [00:00.00] right before the first line of lyrics
```lrc
[00:00.00]@itsmenewbie03
[00:21.56]I can't throw away the pieces that you gave
[00:28.02]Betrayed in a way
[00:30.74]Of a knife stuck on my back
[00:34.25]How could you do this, we've been through this for a while 
```
## 🚀 Running

To run execute the command
```shell
cargo run
```

## 🎯 Contributing

Contributions are welcome, create a pull request to this repo and I will review your code. Thank you!

## 🐛 Issues

Please report any issues and bugs by [creating a new issue here](https://github.com/itsmenewbie03/rs-lyrics-player/issues/new/choose), also make sure you're reporting an issue that doesn't exist. Any help to improve the project would be appreciated. Thanks! 🙏✨

## 📌 Note

If you are going to make a video that uses this project, please include the link to this repo in the caption/description.
