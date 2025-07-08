use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_help_message() {
    let mut cmd = Command::cargo_bin("vmerger").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "vmerger is a high-performance CLI tool that merges multiple video files into a single file and provides format conversion options. It leverages Rust's system programming capabilities and directly calls external FFmpeg programs to ensure maximum execution efficiency and resource control.",
        ));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("vmerger").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_no_input_files() {
    let mut cmd = Command::cargo_bin("vmerger").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_nonexistent_input_file() {
    let mut cmd = Command::cargo_bin("vmerger").unwrap();
    cmd.arg("nonexistent_file.mp4")
        .assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));
}

#[test]
fn test_ffmpeg_not_available() {
    // This test assumes FFmpeg is not in PATH or renamed
    // Skip if FFmpeg is available
    if Command::new("ffmpeg").arg("-version").output().is_ok() {
        // FFmpeg is available, so we can't test this scenario
        return;
    }

    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.mp4");

    // Create a dummy file
    let mut file = File::create(&test_file).unwrap();
    file.write_all(b"dummy content").unwrap();

    let mut cmd = Command::cargo_bin("vmerger").unwrap();
    cmd.arg(&test_file)
        .assert()
        .failure()
        .stderr(predicate::str::contains("FFmpeg"));
}

#[test]
fn test_verbose_flag() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.mp4");

    // Create a dummy file
    let mut file = File::create(&test_file).unwrap();
    file.write_all(b"dummy content").unwrap();

    let mut cmd = Command::cargo_bin("vmerger").unwrap();
    cmd.arg(&test_file).arg("--verbose").assert().failure(); // Will fail because it's not a real video file and FFmpeg will fail
}

#[test]
fn test_format_flag() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.mp4");

    // Create a dummy file
    let mut file = File::create(&test_file).unwrap();
    file.write_all(b"dummy content").unwrap();

    let mut cmd = Command::cargo_bin("vmerger").unwrap();
    cmd.arg(&test_file).arg("-F").arg("avi").assert().failure(); // Will fail because it's not a real video file
}

#[test]
fn test_output_flag() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.mp4");
    let output_file = temp_dir.path().join("output.mp4");

    // Create a dummy file
    let mut file = File::create(&test_file).unwrap();
    file.write_all(b"dummy content").unwrap();

    let mut cmd = Command::cargo_bin("vmerger").unwrap();
    cmd.arg(&test_file)
        .arg("-O")
        .arg(&output_file)
        .assert()
        .failure(); // Will fail because it's not a real video file
}

#[test]
fn test_multiple_input_files() {
    let temp_dir = TempDir::new().unwrap();
    let test_file1 = temp_dir.path().join("test1.mp4");
    let test_file2 = temp_dir.path().join("test2.mp4");

    // Create dummy files
    let mut file1 = File::create(&test_file1).unwrap();
    file1.write_all(b"dummy content 1").unwrap();

    let mut file2 = File::create(&test_file2).unwrap();
    file2.write_all(b"dummy content 2").unwrap();

    let mut cmd = Command::cargo_bin("vmerger").unwrap();
    cmd.arg(&test_file1).arg(&test_file2).assert().failure(); // Will fail because they're not real video files
}

#[test]
fn test_codec_options() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.mp4");

    // Create a dummy file
    let mut file = File::create(&test_file).unwrap();
    file.write_all(b"dummy content").unwrap();

    let mut cmd = Command::cargo_bin("vmerger").unwrap();
    cmd.arg(&test_file)
        .arg("--video-codec")
        .arg("libx264")
        .arg("--audio-codec")
        .arg("aac")
        .assert()
        .failure(); // Will fail because it's not a real video file
}

#[test]
fn test_quality_option() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.mp4");

    // Create a dummy file
    let mut file = File::create(&test_file).unwrap();
    file.write_all(b"dummy content").unwrap();

    let mut cmd = Command::cargo_bin("vmerger").unwrap();
    cmd.arg(&test_file).arg("-q").arg("1M").assert().failure(); // Will fail because it's not a real video file
}

// Integration test with real video files (requires FFmpeg and test video files)
#[cfg(feature = "integration")]
#[test]
fn test_real_video_merge() {
    // This test requires:
    // 1. FFmpeg to be installed and in PATH
    // 2. Test video files to be present
    // 3. The feature "integration" to be enabled

    let test_video1 = "tests/fixtures/test1.mp4";
    let test_video2 = "tests/fixtures/test2.mp4";

    // Skip if test files don't exist
    if !std::path::Path::new(test_video1).exists() || !std::path::Path::new(test_video2).exists() {
        return;
    }

    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("merged.mp4");

    let mut cmd = Command::cargo_bin("vmerger").unwrap();
    cmd.arg(test_video1)
        .arg(test_video2)
        .arg("-O")
        .arg(&output_file)
        .arg("--verbose")
        .assert()
        .success();

    // Verify output file was created
    assert!(output_file.exists());
    assert!(output_file.metadata().unwrap().len() > 0);
}
