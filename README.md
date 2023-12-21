# rust-ffmpeg-hls
360p, 720p, 1080p를 지원하는 HLS 파일을 생성합니다. 로컬에 있는 FFmpeg를 통해 HLS 파일을 생성합니다.  

## FFmpeg HLS Example
```shell
ffmpeg -i input.mp4 \
-preset veryfast -threads 0 \
-map 0:v:0 -map '0:a:0' -map 0:v:0 -map '0:a:0' -map 0:v:0 -map '0:a:0' \
-c:v libx264 -crf 22 -c:a aac -ar 48000 \
-filter:v:0 scale=-2:360:force_original_aspect_ratio=decrease  -maxrate:v:0 600k -b:a:0 64k \
-filter:v:1 scale=-2:720:force_original_aspect_ratio=decrease  -maxrate:v:1 900k -b:a:1 128k \
-filter:v:2 scale=-2:1080:force_original_aspect_ratio=decrease -maxrate:v:2 900k -b:a:2 128k \
-f hls -hls_time 10 -hls_playlist_type vod -hls_list_size 0 -hls_flags independent_segments \
-var_stream_map "v:0,a:0,name:360p v:1,a:1,name:720p v:2,a:2,name:1080p" \
-master_pl_name master.m3u8 \
-hls_segment_filename "res_%v/file_%03d.ts" "res_%v/index.m3u8"
```
