use fancy_regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::Command;
use std::thread;
use std::time::Duration;

fn print_txt(input: &str) {
    if let Ok(output) = Command::new("figlet")
        .args(["-w", "150"])
        .arg(input)
        .output()
    {
        if output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            eprintln!(
                "Figlet command failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    } else {
        eprintln!("Error running figlet.");
    }
}

fn remove_timestamp(line: &str) -> String {
    if let Some(start) = line.find('[') {
        if let Some(end) = line[start..].find(']') {
            let mut result = String::with_capacity(line.len() - (end + 2));
            result.push_str(&line[..start]);
            result.push_str(&line[start + end + 1..]);

            return result;
        }
    }

    line.to_string()
}

fn get_timestamp(line: &str) -> &str {
    if let Some(start) = line.find('[') {
        if let Some(end) = line[start..].find(']') {
            return &line[start + 1..start + end];
        }
    }

    ""
}

fn pause_between_timestamps(timestamp1: &str, timestamp2: &str) {
    let pattern = Regex::new(r"(?:(\d+):)?(\d+\.\d+)").unwrap();
    let captures1 = pattern.captures(timestamp1).unwrap().unwrap();
    let captures2 = pattern.captures(timestamp2).unwrap().unwrap();

    let minutes1 = captures1
        .get(1)
        .map_or(0, |m| m.as_str().parse::<u64>().unwrap());
    let seconds1 = captures1
        .get(2)
        .map_or(0.0, |m| m.as_str().parse::<f64>().unwrap());

    let minutes2 = captures2
        .get(1)
        .map_or(0, |m| m.as_str().parse::<u64>().unwrap());
    let seconds2 = captures2
        .get(2)
        .map_or(0.0, |m| m.as_str().parse::<f64>().unwrap());

    let total_seconds1 = (minutes1 * 60) as f64 + seconds1;
    let total_seconds2 = (minutes2 * 60) as f64 + seconds2;

    let time_to_pause = (total_seconds2 - total_seconds1).abs();
    std::thread::sleep(Duration::from_secs_f64(time_to_pause));
}

fn main() -> io::Result<()> {
    let file = File::open("res/lyrics.txt")?;
    let lines: Vec<_> = io::BufReader::new(file).lines().collect();

    let line_count = lines.len();

    Command::new("clear")
        .status()
        .expect("failed to clear screen");

    let t1 = thread::spawn(|| {
        Command::new("ffplay")
            .args(["-v", "0", "-nodisp", "-autoexit", "res/music.mp3"])
            .status()
            .expect("failed to execute ffplay");
    });

    for x in 0..line_count {
        Command::new("clear")
            .status()
            .expect("failed to clear screen");

        let ts_next = if x != line_count - 1 {
            get_timestamp(lines[x + 1].as_ref().unwrap())
        } else {
            "00:00.00"
        };

        let line_now = &lines[x];
        let line_now_cleaned = remove_timestamp(line_now.as_ref().unwrap().as_str());
        let ts_now = get_timestamp(line_now.as_ref().unwrap().as_str());
        print_txt(&line_now_cleaned);
        pause_between_timestamps(ts_now, ts_next);
    }

    t1.join().expect("thread panicked");

    Ok(())
}
