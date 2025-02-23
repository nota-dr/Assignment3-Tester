use chrono::{DateTime, Utc};
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use nix::unistd::mkfifo;
use nix::sys::stat::Mode;

use async_trait::async_trait;
use tests_lib::{check_valgrind_leaks, ProcessOutput, TestAgent};

pub struct Usage;
pub struct ValidateInput;
pub struct OnlyGETMethod;
pub struct PathDoesNotExist;
pub struct TemporaryRedirect;
pub struct Forbidden;
pub struct SearchForIndexHtml;
pub struct ReturnDirContent;
pub struct FileSizeExceedsOSBuffer;
pub struct Deadlock;
pub struct Valgrind;

// pub fn mkdirs_tree(cwd: &PathBuf) {
//     let tree = vec!["dir1/dir2/dir3/dir5", "dir1/dir2/dir4"];

//     for path in tree {
//         let path = cwd.join(path);
//         if !path.exists() {
//             std::fs::create_dir_all(path).unwrap();
//         }
//     }

//     let file_tree = vec![
//         "dir1/dir2/dir3/avi_example.avi",
//         "dir1/dir2/dir3/gif_example.gif",
//         "dir1/dir2/dir3/mp3_example.mp3",
//         "dir1/dir2/dir3/png_example.png",
//         "dir1/dir2/dir3/html_example.html",
//         "dir1/dir2/dir3/jpg_example.jpg",
//         "dir1/dir2/dir3/mov_example.mov",
//                                           // MIME that should not be supported
//                                           // "dir1/dir2/dir4/no_permission",
//                                           // "dir1/dir2/index.html"
//                                           // "dir1/dir2/symlink"
//     ];

//     let urls = vec![
//         "https://file-examples.com/storage/fefbb4ad1367b5dbea839ba/2018/04/file_example_AVI_480_750kB.avi",
//         "https://file-examples.com/storage/fefbb4ad1367b5dbea839ba/2017/10/file_example_GIF_500kB.gif",
//         "https://file-examples.com/storage/fefbb4ad1367b5dbea839ba/2017/11/file_example_MP3_700KB.mp3",
//         "https://file-examples.com/storage/fefbb4ad1367b5dbea839ba/2017/10/file_example_PNG_500kB.png",
//         "https://www.filesampleshub.com/download/code/html/sample2.html",
//         "https://file-examples.com/storage/fefbb4ad1367b5dbea839ba/2017/10/file_example_JPG_100kB.jpg",
//         "https://file-examples.com/storage/fefbb4ad1367b5dbea839ba/2018/04/file_example_MOV_1920_2_2MB.mov"
//     ];

//     for (path, url) in file_tree.iter().zip(urls.iter()) {
//         let path = cwd.join(path);
//         if !path.exists() {
//             let mut response = reqwest::blocking::get(*url).unwrap();
//             let mut file = std::fs::File::create(path).unwrap();
//             response.copy_to(&mut file).unwrap();
//         }
//     }

//     std::fs::copy(
//         cwd.join("dir1/dir2/dir3/avi_example.avi"),
//         cwd.join("dir1/dir2/dir4/no_permission"),
//     )
//     .unwrap();
//     std::fs::copy(
//         cwd.join("dir1/dir2/dir3/html_example.html"),
//         cwd.join("dir1/dir2/index.html"),
//     )
//     .unwrap();
//     // std::fs::set_permissions(
//     //     cwd.join("dir1/dir2/dir4/no_permission"),
//     //     std::fs::Permissions::from_mode(0o277),
//     // )
//     // .unwrap();

//     let threadpool_h = std::env::current_dir()
//         .unwrap()
//         .join("assignment3-tester")
//         .join("resources")
//         .join("threadpool.h");

//     // std::fs::copy(threadpool_h, cwd.join("threadpool.h"))
//     //     .expect("[-] Could not copy threadpool.h");

//     let symlink_path = cwd.join("dir1").join("dir2").join("dummy_symlink");

//     if !symlink_path.is_symlink() {
//         std::os::unix::fs::symlink("dummy", symlink_path).unwrap();
//     }
// }

pub fn copy_dir_recursivly(src: &PathBuf, dst: &PathBuf) {
    if !src.is_dir() {
        return;
    }

    if !dst.exists() {
        std::fs::create_dir_all(&dst).unwrap();
    }

    for entry in std::fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap();
        let dst = dst.join(file_name);

        if path.is_dir() {
            copy_dir_recursivly(&path, &dst);
        } else {
            std::fs::copy(&path, &dst)
                .expect(format!("[-] Could not copy {:?}", path).as_str());
        }
    }
}

pub fn create_dir_structure() {
    let cwd = std::env::current_dir().unwrap();
    if cwd.join("testee").join("dir1").exists() {
        return;
    }

    let src = cwd
        .join("assignment3-tester")
        .join("resources")
        .join("dir_structure");
    let dst = cwd.join("testee");
    copy_dir_recursivly(&src, &dst);

    std::fs::set_permissions(
        cwd.join("testee")
            .join("dir1")
            .join("dir2")
            .join("dir4")
            .join("no_permission"),
        std::fs::Permissions::from_mode(0o277),
    )
    .unwrap();

    let threadpool_h = cwd
        .join("assignment3-tester")
        .join("resources")
        .join("threadpool.h");

    let fifo_file = cwd
        .join("testee")
        .join("dir1")
        .join("dir2")
        .join("fifo_file");

    if !fifo_file.exists() {
        let permissions = Mode::S_IRUSR | Mode::S_IWUSR | Mode::S_IRGRP | Mode::S_IWGRP | Mode::S_IROTH | Mode::S_IWOTH;
        mkfifo(&fifo_file, permissions).expect("[-] Could not create fifo file");
    }

    // if !symlink_path.is_symlink() {
    //     std::os::unix::fs::symlink(&threadpool_h, symlink_path).unwrap();
    // }

    std::fs::copy(threadpool_h, cwd.join("testee").join("threadpool.h"))
        .expect("[-] Could not copy threadpool.h");
}

async fn send_local_request(
    port: &str,
    request: &[u8],
    read_timeout: u64,
) -> Result<Vec<u8>, std::io::Error> {
    let address = format!("127.0.0.1:{}", port);
    let mut buffer = Vec::new();
    let mut stream = TcpStream::connect(address).await?;
    stream.write_all(request).await?;
    timeout(
        Duration::from_secs(read_timeout),
        stream.read_to_end(&mut buffer),
    )
    .await??;

    Ok(buffer)
}

fn verify_responses_status(output: &Vec<Vec<u8>>, expected: &str) -> bool {
    let status = String::from("http/1.0 ") + expected;
    let content_length = "content-length: ";
    for raw_res in output {
        let res = String::from_utf8_lossy(raw_res).to_lowercase();

        if !res.contains(&status) {
            return false;
        }

        // verify that the content-length was not copied-pasta
        let content_length_pos =
            res.find(content_length).map(|i| i + content_length.len());

        let content_length_pos = match content_length_pos {
            Some(i) => i,
            None => return false,
        };

        let content_length_value = res[content_length_pos..]
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<usize>()
            .expect("[-] Could not parse content-length");

        let res_parts = res.split("\r\n\r\n").collect::<Vec<&str>>();
        if res_parts.len() < 2 {
            return false;
        }
        
        let raw_body = res_parts[1];

        if raw_body.len() != content_length_value {
            return false;
        }
    }

    true
}

impl TestAgent for Usage {
    fn validate(
        &self,
        _: &Vec<String>,
        _: Option<Vec<Vec<u8>>>,
        result: ProcessOutput,
        _: &PathBuf,
    ) -> bool {
        let expected = "Usage: server";
        let output = [result.stdout, result.stderr].concat();
        let output = String::from_utf8_lossy(&output);
        if output
            .trim()
            .to_lowercase()
            .contains(&expected.to_lowercase())
        {
            return true;
        }
        false
    }
}

#[async_trait]
impl TestAgent for ValidateInput {
    async fn communicate(
        &self,
        read_timeout: u64,
        port: &str,
        _: Option<i32>
    ) -> Result<Vec<Vec<u8>>, std::io::Error> {
        let buffer1 =
            send_local_request(port, b"GET HTTP/1.0\r\n", read_timeout).await?;
        let buffer2 =
            send_local_request(port, b"/ HTTP/1.0\r\n", read_timeout).await?;
        let buffer3 =
            send_local_request(port, b"GET / HTT/1.0\r\n", read_timeout)
                .await?;
        Ok(vec![buffer1, buffer2, buffer3])
    }

    fn validate(
        &self,
        _: &Vec<String>,
        communicate_output: Option<Vec<Vec<u8>>>,
        _: ProcessOutput,
        _: &PathBuf,
    ) -> bool {
        let communicate_output = communicate_output.unwrap();
        verify_responses_status(&communicate_output, "400 bad request")
    }
}

#[async_trait]
impl TestAgent for OnlyGETMethod {
    async fn communicate(
        &self,
        read_timeout: u64,
        port: &str,
        _: Option<i32>
    ) -> Result<Vec<Vec<u8>>, std::io::Error> {
        let buffer =
            send_local_request(port, b"POST / HTTP/1.0\r\n", read_timeout)
                .await?;
        Ok(vec![buffer])
    }

    fn validate(
        &self,
        _: &Vec<String>,
        communicate_output: Option<Vec<Vec<u8>>>,
        _: ProcessOutput,
        _: &PathBuf,
    ) -> bool {
        let communicate_output = communicate_output.unwrap();
        verify_responses_status(&communicate_output, "501 not supported")
    }
}

#[async_trait]
impl TestAgent for PathDoesNotExist {
    async fn communicate(
        &self,
        read_timeout: u64,
        port: &str,
        _: Option<i32>
    ) -> Result<Vec<Vec<u8>>, std::io::Error> {
        let buffer = send_local_request(
            port,
            b"GET /nonexistent HTTP/1.0\r\n",
            read_timeout,
        )
        .await?;
        Ok(vec![buffer])
    }

    fn validate(
        &self,
        _: &Vec<String>,
        communicate_output: Option<Vec<Vec<u8>>>,
        _: ProcessOutput,
        _: &PathBuf,
    ) -> bool {
        let communicate_output = communicate_output.unwrap();
        verify_responses_status(&communicate_output, "404 not found")
    }
}

#[async_trait]
impl TestAgent for TemporaryRedirect {
    async fn communicate(
        &self,
        read_timeout: u64,
        port: &str,
        _: Option<i32>
    ) -> Result<Vec<Vec<u8>>, std::io::Error> {
        let buffer = send_local_request(
            port,
            b"GET /dir1/dir2 HTTP/1.0\r\n",
            read_timeout,
        )
        .await?;
        Ok(vec![buffer])
    }

    fn validate(
        &self,
        _: &Vec<String>,
        communicate_output: Option<Vec<Vec<u8>>>,
        _: ProcessOutput,
        _: &PathBuf,
    ) -> bool {
        let communicate_output = communicate_output.unwrap();
        if !verify_responses_status(&communicate_output, "302 found") {
            return false;
        }

        let communicate_output = communicate_output[0].to_ascii_lowercase();
        let location_header = b"location: /dir1/dir2/\r\n";
        let last_modified = b"last-modified: ";
        let found_location = communicate_output
            .windows(location_header.len())
            .filter(|w| w == location_header)
            .count();
        let found_last_modified = communicate_output
            .windows(last_modified.len())
            .filter(|w| w == last_modified)
            .count();

        if found_last_modified != 0 || found_location != 1 {
            return false;
        }

        true
    }
}

#[async_trait]
impl TestAgent for Forbidden {
    async fn communicate(
        &self,
        read_timeout: u64,
        port: &str,
        _: Option<i32>
    ) -> Result<Vec<Vec<u8>>, std::io::Error> {
        let buffer1 = send_local_request(
            port,
            b"GET /dir1/dir2/dir4/no_permission HTTP/1.0\r\n",
            read_timeout,
        )
        .await?;

        let buffer2 = send_local_request(
            port,
            b"GET /dir1/dir2/fifo_file HTTP/1.0\r\n",
            read_timeout,
        )
        .await?;

        Ok(vec![buffer1, buffer2])
    }

    fn validate(
        &self,
        _: &Vec<String>,
        communicate_output: Option<Vec<Vec<u8>>>,
        _: ProcessOutput,
        _: &PathBuf,
    ) -> bool {
        let communicate_output = communicate_output.unwrap();
        verify_responses_status(&communicate_output, "403 forbidden")
    }
}

#[async_trait]
impl TestAgent for SearchForIndexHtml {
    async fn communicate(
        &self,
        read_timeout: u64,
        port: &str,
        _: Option<i32>
    ) -> Result<Vec<Vec<u8>>, std::io::Error> {
        let buffer = send_local_request(
            port,
            b"GET /dir1/dir2/ HTTP/1.0\r\n",
            read_timeout,
        )
        .await?;
        Ok(vec![buffer])
    }

    fn validate(
        &self,
        _: &Vec<String>,
        communicate_output: Option<Vec<Vec<u8>>>,
        _: ProcessOutput,
        cwd: &PathBuf,
    ) -> bool {
        let index_path = cwd.join("").join("dir1/dir2/index.html");

        let mut expected_headers = vec![
            "http/1.0 200 ok",
            "server: webserver",
            "content-type: text/html",
            "connection: close",
        ];

        let index_metadata = std::fs::metadata(&index_path).unwrap();
        let content_length =
            format!("content-length: {}", index_metadata.len());

        let last_modified = index_metadata
            .modified()
            .expect("[-] Could not get last modified time");

        let datetime: DateTime<Utc> = last_modified.into();

        let last_modified = format!(
            "last-modified: {}",
            datetime.format("%a, %d %b %Y %H:%M:%S GMT")
        )
        .to_lowercase();

        expected_headers.push(&content_length);
        expected_headers.push(&last_modified);

        let communicate_output =
            String::from_utf8_lossy(&communicate_output.unwrap()[0])
                .to_lowercase();

        for header in expected_headers {
            if !communicate_output.contains(&header) {
                return false;
            }
        }

        let index_html = std::fs::read_to_string(index_path)
            .expect("[-] Could not read index.html");

        if !communicate_output.contains(index_html.to_lowercase().as_str()) {
            return false;
        }

        true
    }
}

#[async_trait]
impl TestAgent for ReturnDirContent {
    async fn communicate(
        &self,
        read_timeout: u64,
        port: &str,
        _: Option<i32>
    ) -> Result<Vec<Vec<u8>>, std::io::Error> {
        let buffer = send_local_request(
            port,
            b"GET /dir1/dir2/dir3/ HTTP/1.0\r\n",
            read_timeout,
        )
        .await?;
        Ok(vec![buffer])
    }

    fn validate(
        &self,
        _: &Vec<String>,
        communicate_output: Option<Vec<Vec<u8>>>,
        _: ProcessOutput,
        cwd: &PathBuf,
    ) -> bool {
        let dir_path = cwd.join("dir1/dir2/dir3");
        let communicate_output =
            String::from_utf8_lossy(&communicate_output.unwrap()[0])
                .to_lowercase();
        let entries =
            std::fs::read_dir(dir_path).expect("[-] Could not read directory");

        if !communicate_output.contains("http/1.0 200 ok") {
            return false;
        }

        for entry in entries {
            let entry = entry.expect("[-] Could not get entry");
            let file_type = entry.file_type().unwrap();
            let file_name = entry.file_name().into_string().unwrap();
            let file_metadata = entry.metadata().unwrap();
            let last_modified: DateTime<Utc> =
                file_metadata.modified().unwrap().into();

            let col_name =
                format!("<td><a href=\"{}\">{}</a></td>", file_name, file_name)
                    .to_lowercase();

            let col_time = format!(
                "<td>{}</td>",
                last_modified.format("%a, %d %b %Y %H:%M:%S GMT")
            )
            .to_lowercase();

            let col_size = if file_type.is_file() {
                format!("<td>{}</td>", file_metadata.len())
            } else {
                String::from("<td></td>")
            }
            .to_lowercase();

            if !communicate_output.contains(&col_name) {
                return false;
            }

            if !communicate_output.contains(&col_time) {
                return false;
            }

            if !communicate_output.contains(&col_size) {
                return false;
            }
        }

        true
    }
}

#[async_trait]
impl TestAgent for FileSizeExceedsOSBuffer {
    async fn communicate(
        &self,
        read_timeout: u64,
        port: &str,
        _: Option<i32>
    ) -> Result<Vec<Vec<u8>>, std::io::Error> {
        let buffer = send_local_request(
            port,
            b"GET /dir1/dir2/dir3/mov_example.mov HTTP/1.0\r\n",
            read_timeout,
        )
        .await?;
        Ok(vec![buffer])
    }

    fn validate(
        &self,
        _: &Vec<String>,
        communicate_output: Option<Vec<Vec<u8>>>,
        _: ProcessOutput,
        cwd: &PathBuf,
    ) -> bool {
        let communicate_output = &communicate_output.unwrap()[0];
        let communicate_output_utf8 =
            String::from_utf8_lossy(&communicate_output).to_lowercase();
        let raw_headers =
            communicate_output_utf8.split("\r\n\r\n").next().unwrap();

        if !raw_headers.contains("http/1.0 200 ok") {
            return false;
        }

        if raw_headers.contains("content-type") {
            return false;
        }

        let mov_path = cwd
            .join("dir1")
            .join("dir2")
            .join("dir3")
            .join("mov_example.mov");

        let mut file = std::fs::File::open(mov_path)
            .expect("[-] Could not open mov_example.mov");

        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer)
            .expect("[-] Could not read mov_example.mov");

        communicate_output
            .windows(buffer.len())
            .filter(|&w| w == &buffer)
            .count()
            == 1
    }
}

#[async_trait]
impl TestAgent for Deadlock {
    async fn communicate(
        &self,
        read_timeout: u64,
        port: &str,
        _: Option<i32>
    ) -> Result<Vec<Vec<u8>>, std::io::Error> {
        let mut tasks = Vec::new();
        for _ in 0..10 {
            let owned_port = port.to_owned();
            let t = tokio::task::spawn(async move {
                send_local_request(
                    &owned_port,
                    b"GET /dir1/dir2/dir3/jpg_example.jpg HTTP/1.0\r\n",
                    read_timeout,
                )
                .await
            });
            tasks.push(t);
        }

        let mut responses = Vec::new();

        let results = futures::future::join_all(tasks).await;
        for res in results {
            let res = res??;
            responses.push(res);
        }

        let mut streams: Vec<tokio::net::TcpStream> = Vec::new();
        for _ in 0..5 {
            let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))
                .await
                .expect("[-] Could not connect to server");

            stream
                .write_all(b"GET /dir1/dir2/dir3/jpg_example.jpg HTTP/1.0\r\n")
                .await
                .expect("[-] Could not write to stream");

            streams.push(stream);
        }

        for mut stream in streams {
            let mut buffer = Vec::new();
            timeout(
                Duration::from_secs(read_timeout),
                stream.read_to_end(&mut buffer),
            )
            .await??;
            responses.push(buffer);
        }

        Ok(responses)
    }

    fn validate(
        &self,
        _: &Vec<String>,
        communicate_output: Option<Vec<Vec<u8>>>,
        _: ProcessOutput,
        _: &PathBuf,
    ) -> bool {
        let communicate_output = communicate_output.unwrap();
        let expected = "http/1.0 200 ok";
        let found = communicate_output
            .iter()
            .filter(|&output| {
                String::from_utf8_lossy(output)
                    .to_lowercase()
                    .contains(expected)
                    && output.len() > 50 * 1024 // at least 50KB each response
            })
            .count();

        found == 15
    }
}

#[async_trait]
impl TestAgent for Valgrind {
    async fn communicate(
        &self,
        read_timeout: u64,
        port: &str,
        _: Option<i32>
    ) -> Result<Vec<Vec<u8>>, std::io::Error> {
        let buffer1 =
            send_local_request(port, b"GET HTTP/1.0\r\n", read_timeout).await?;

        let buffer2 =
            send_local_request(port, b"POST / HTTP/1.0\r\n", read_timeout)
                .await?;

        let buffer3 = send_local_request(
            port,
            b"GET /nonexistent HTTP/1.0\r\n",
            read_timeout,
        )
        .await?;

        let buffer4 = send_local_request(
            port,
            b"GET /dir1/dir2 HTTP/1.0\r\n",
            read_timeout,
        )
        .await?;

        let buffer5 = send_local_request(
            port,
            b"GET /dir1/dir2/dir4/no_permission HTTP/1.0\r\n",
            read_timeout,
        )
        .await?;

        let buffer6 = send_local_request(
            port,
            b"GET /dir1/dir2/ HTTP/1.0\r\n",
            read_timeout,
        )
        .await?;

        let buffer7 = send_local_request(
            port,
            b"GET /dir1/dir2/dir3/ HTTP/1.0\r\n",
            read_timeout,
        )
        .await?;

        let buffer8 = send_local_request(
            port,
            b"GET /dir1/dir2/dir3/mov_example.mov HTTP/1.0\r\n",
            read_timeout,
        )
        .await?;

        Ok(vec![
            buffer1, buffer2, buffer3, buffer4, buffer5, buffer6, buffer7,
            buffer8,
        ])
    }

    fn validate(
        &self,
        _: &Vec<String>,
        communicate_output: Option<Vec<Vec<u8>>>,
        _: ProcessOutput,
        tests_dir: &PathBuf,
    ) -> bool {
        let communicate_output = communicate_output
            .unwrap()
            .into_iter()
            .map(|output| String::from_utf8_lossy(&output).to_lowercase())
            .collect::<Vec<String>>();

        if !communicate_output[0].contains("bad request") {
            return false;
        }

        if !communicate_output[1].contains("not supported") {
            return false;
        }

        if !communicate_output[2].contains("not found") {
            return false;
        }

        if !communicate_output[3].contains("found") {
            return false;
        }

        if !communicate_output[4].contains("forbidden") {
            return false;
        }

        if !communicate_output[5].contains("ok") {
            return false;
        }

        // if !communicate_output[6].contains("ok")  {
        //     return false;
        // }

        if !communicate_output[7].contains("ok") {
            return false;
        }

        let valgrind_path1 = tests_dir.join("valgrind - Deadlock");
        let valgrind_path2 =
            tests_dir.join("valgrind - Valgrind Path Coverage");

        if !check_valgrind_leaks(&valgrind_path1) {
            return false;
        }

        if !check_valgrind_leaks(&valgrind_path2) {
            return false;
        }

        true
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::hw3_tests::mkdirs_tree;

//     #[test]
//     fn t() {
//         mkdirs_tree(&std::env::current_dir().unwrap().join("testee"));
//     }
// }
