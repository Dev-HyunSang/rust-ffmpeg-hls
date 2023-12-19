/* Author: HyunSang Park <me@hyunsang.dev> */
use tokio::process::Command;
use tokio::io::{self, AsyncBufReadExt};

#[tokio::main]
async fn main() {
    let input = "./sample_1h.mp4";
    let mut command = Command::new("ffmpeg");
    
    command.arg("-i").arg(input)
        .arg("-preset").arg("veryfast")
        .arg("-threads").arg("0")
        .arg("-map").arg("0:v:0").arg("-map").arg("0:a:0")
        .arg("-map").arg("0:v:0").arg("-map").arg("0:a:0")
        .arg("-map").arg("0:v:0").arg("-map").arg("0:a:0")
        .arg("-c:v").arg("libx264")
        .arg("-c:v").arg("h264_videotoolbox")
        .arg("-crf").arg("22")
        .arg("-c:a").arg("aac")
        .arg("-ar").arg("48000")
        .arg("-filter:v:0").arg("scale=-2:360:force_original_aspect_ratio=decrease").arg("-maxrate:v:0").arg("600k").arg("-b:a:0").arg("64k")
        .arg("-filter:v:1").arg("scale=-2:720:force_original_aspect_ratio=decrease").arg("-maxrate:v:1").arg("900k").arg("-b:a:1").arg("128k")
        .arg("-filter:v:2").arg("scale=-2:1080:force_original_aspect_ratio=decrease").arg("-maxrate:v:2").arg("900k").arg("-b:a:2").arg("128k")
        .arg("-f").arg("hls")
        .arg("-hls_time").arg("10")
        .arg("-hls_playlist_type").arg("vod")
        .arg("-hls_list_size").arg("0")
        .arg("-hls_flags").arg("independent_segments")
        .arg("-var_stream_map").arg("v:0,a:0,name:360p v:1,a:1,name:720p v:2,a:2,name:1080p")
        .arg("-master_pl_name").arg("master.m3u8")
        .arg("-hls_segment_filename").arg("res_%v/file_%03d.ts")
        .arg("res_%v/index.m3u8");

    let mut child = command.spawn().expect("Failed to spawn command");
    let stdout = child.stdout.take().expect("Failed to get stdout");
    let reader = io::BufReader::new(stdout);
    
    tokio::spawn(async move {
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await.expect("Failed to read line") {
            println!("Output: {}", line);
        }
    });
    
    let status = child.wait().await.expect("Failed to wait for command");
    
    if status.success() {
        println!("Command executed successfully");
    } else {
        println!("Command failed");
    }
}