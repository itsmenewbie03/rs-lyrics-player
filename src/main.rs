use fancy_regex::Regex;
use std::{path::PathBuf, process::Command, str::FromStr, thread, time::Duration};

#[derive(Debug)]
struct Line {
    line: String,
    time: String,
}

// NOTE: I'm pretending to understand this lifetime shit xD
// OK that's it, I'm still now worthy to use References xD
#[derive(Debug)]
struct Lyrics {
    lines: Vec<Line>,
}

impl Line {
    fn diff(&self, next: &Line) -> Duration {
        let cur = self.to_ms();
        let next = next.to_ms();
        Duration::from_secs_f64(next - cur)
    }

    fn to_ms(&self) -> f64 {
        // INFO: format 00:50.00
        let mut parts = self.time.split(&[':']);
        let mins = parts.next().expect("Minutes part not found");
        let mins = mins
            .parse::<f64>()
            .unwrap_or_else(|_| panic!("Failed to parse minutes to float.\nMins: {}", mins))
            * 60_f64;
        let sec = parts.next().expect("Seconds part not found");
        let sec = sec
            .parse::<f64>()
            .expect("Failed to parse seconds to float.");
        mins + sec
    }
}

impl Lyrics {
    fn iter(&self) -> LyricsIterator {
        LyricsIterator {
            lyrics: self,
            index: 0,
        }
    }
    fn from_file(path: std::path::PathBuf) -> Result<Self, String> {
        let content = std::fs::read_to_string(path);
        match content {
            // NOTE: valid line format
            // [00:50.00] @itsmenewbie03
            Ok(lyrics) => {
                let mut lines: Vec<Line> = lyrics
                    .trim()
                    .split("\n")
                    .map(|line| {
                        let re = Regex::new(r"((?<=\[)[0-9:\.]+(?=\]))(\])(.*)").unwrap();
                        let matches = re.captures(line).unwrap().unwrap();
                        let time = matches.get(1).expect("Time is not found!").as_str();
                        let line = matches.get(3).expect("Line is not found!").as_str();
                        Line {
                            time: time.to_owned(),
                            line: line.to_owned(),
                        }
                    })
                    .collect();
                Ok(Lyrics { lines })
            }
            Err(err) => Err(format!("Failed to read lyric file!\nERR: {}", err)),
        }
    }
}

struct LyricsIterator<'a> {
    lyrics: &'a Lyrics,
    index: usize,
}

impl<'a> Iterator for LyricsIterator<'a> {
    type Item = &'a Line;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.lyrics.lines.len() {
            let item = &self.lyrics.lines[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

fn clear() {
    Command::new("clear")
        .status()
        .expect("failed to clear screen");
}

fn play() -> std::thread::JoinHandle<()> {
    thread::spawn(|| {
        Command::new("ffplay")
            .args(["-v", "0", "-nodisp", "-autoexit", "res/music.mp3"])
            .status()
            .expect("failed to execute ffplay");
    })
}
fn main() {
    let lyrics = Lyrics::from_file(PathBuf::from_str("res/lyrics.txt").unwrap())
        .expect("Failed to parse lyrics file.");
    clear();
    let play_handle = play();
    for line in lyrics.iter().collect::<Vec<_>>().windows(2) {
        let cur = line.first().expect("100% RUST BUG!");
        let next = line.last().expect("100% RUST BUG!");
        let diff = cur.diff(next);
        println!("{}", cur.line);
        thread::sleep(diff);
        clear();
    }
    println!("{}", lyrics.lines.last().expect("100% RUST BUG!").line);
    play_handle.join().unwrap();
}
