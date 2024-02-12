use fancy_regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::Command;
use std::thread;
use std::time::Duration;

// INFO: hello this is mostly ai generated code xD
fn print_txt(input: &str, font_path: &str) {
    if let Ok(output) = Command::new("figlet")
        .args(&["-w", "200", "-d", "res/", "-f", font_path])
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

// fn print_txt(txt: &str) {
//     let figure = Command::new("figlet")
//         .arg("-t")
//         .arg(txt)
//         .output()
//         .expect(txt);
//     println!(
//         "{}",
//         String::from_utf8_lossy(&figure.stdout).trim().to_string()
//     );
//     // let standard_font = FIGfont::from_file("res/small.flf").unwrap();
//     // let figure = standard_font.convert(txt);
// }

// fn remove_timestamp(line: &str) -> String {
//     let re = Regex::new(r"\[\d+:\.\d+\]").unwrap();
//     let out = re.replace_all(line, "").to_string();
//     println!("TS REMOVED: {}", out);
//     out
// }

fn remove_timestamp(line: &str) -> String {
    // Find the position of '[' and ']'
    if let Some(start) = line.find('[') {
        if let Some(end) = line[start..].find(']') {
            // Construct the new string without the timestamp
            let mut result = String::with_capacity(line.len() - (end + 2));
            result.push_str(&line[..start]);
            result.push_str(&line[start + end + 1..]);

            // println!("TS REMOVED: {}", result);
            return result;
        }
    }

    // Return the original string if brackets are not found
    // println!("TS REMOVED: {}", line);
    line.to_string()
}

// fn get_timestamp(line: &str) -> &str {
//     let re = Regex::new(r"(?<=\[)\d+:\.\d+(?=\])").unwrap();
//     re.find(line).map_or("", |m| m.unwrap().as_str())
// }

fn get_timestamp(line: &str) -> &str {
    // Find the position of '[' and ']'
    if let Some(start) = line.find('[') {
        if let Some(end) = line[start..].find(']') {
            // Extract the content between '[' and ']'
            return &line[start + 1..start + end];
        }
    }

    // Return an empty string if brackets are not found
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
    // println!("WE GOT {} lines", line_count);

    Command::new("clear")
        .status()
        .expect("failed to clear screen");

    let t1 = thread::spawn(|| {
        Command::new("ffplay")
            .args(&["-v", "0", "-nodisp", "-autoexit", "res/music.mp3"])
            .status()
            .expect("failed to execute ffplay");
    });

    for x in 0..line_count {
        Command::new("clear")
            .status()
            .expect("failed to clear screen");

        let ts_next = if x != line_count - 1 {
            // handle the last line
            get_timestamp(&lines[x + 1].as_ref().unwrap())
        } else {
            "00:00.00"
        };

        let line_now = &lines[x];
        let line_now_cleaned = remove_timestamp(line_now.as_ref().unwrap().as_str());
        let ts_now = get_timestamp(line_now.as_ref().unwrap().as_str());
        print_txt(&line_now_cleaned, "small");
        pause_between_timestamps(ts_now, ts_next);
    }

    t1.join().expect("thread panicked");

    Ok(())
}
