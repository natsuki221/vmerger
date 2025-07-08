use anyhow::{Context, Result};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tempfile::NamedTempFile;
use thiserror::Error;

use crate::cli::Cli;

#[derive(Error, Debug)]
pub enum ProcessorError {
    #[error("FFmpeg not found. Please install FFmpeg and ensure it's in your PATH")]
    FfmpegNotFound,
    #[error("FFmpeg execution failed: {0}")]
    FfmpegExecutionFailed(String),
    #[error("File I/O error: {0}")]
    FileIoError(#[from] std::io::Error),
}

pub struct VideoProcessor {
    verbose: bool,
}

impl VideoProcessor {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    /// Check if FFmpeg is available in the system
    pub fn check_ffmpeg_availability(&self) -> Result<()> {
        let output = Command::new("ffmpeg").arg("-version").output().context(
            "Failed to execute FFmpeg. Please ensure FFmpeg is installed and in your PATH",
        )?;

        if !output.status.success() {
            return Err(ProcessorError::FfmpegNotFound.into());
        }

        if self.verbose {
            println!("✓ FFmpeg is available");
        }

        Ok(())
    }

    /// Create a temporary file list for FFmpeg concat demuxer
    fn create_concat_file(&self, input_files: &[PathBuf]) -> Result<NamedTempFile> {
        let mut temp_file = NamedTempFile::new().context("Failed to create temporary file")?;

        for file in input_files {
            let absolute_path = file
                .canonicalize()
                .with_context(|| format!("Failed to get absolute path for: {}", file.display()))?;

            writeln!(temp_file, "file '{}'", absolute_path.display())
                .context("Failed to write to temporary file")?;
        }

        temp_file
            .flush()
            .context("Failed to flush temporary file")?;

        if self.verbose {
            println!(
                "✓ Created temporary concat file: {}",
                temp_file.path().display()
            );
        }

        Ok(temp_file)
    }

    /// Build FFmpeg command for merging videos
    fn build_ffmpeg_command(
        &self,
        cli: &Cli,
        concat_file_path: &PathBuf,
        output_path: &PathBuf,
    ) -> Command {
        let mut cmd = Command::new("ffmpeg");

        // Input arguments
        cmd.arg("-f")
            .arg("concat")
            .arg("-safe")
            .arg("0")
            .arg("-i")
            .arg(concat_file_path);

        // Video codec
        let video_codec = cli.get_video_codec();
        cmd.arg("-c:v").arg(&video_codec);

        // Audio codec
        let audio_codec = cli.get_audio_codec();
        cmd.arg("-c:a").arg(&audio_codec);

        // Video quality/bitrate
        if let Some(ref quality) = cli.video_quality {
            cmd.arg("-b:v").arg(quality);
        }

        // Overwrite output file without asking
        cmd.arg("-y");

        // Output file
        cmd.arg(output_path);

        if self.verbose {
            println!("✓ FFmpeg command: {cmd:?}");
        }

        cmd
    }

    /// Execute FFmpeg command and handle output
    fn execute_ffmpeg_command(&self, mut cmd: Command) -> Result<()> {
        if self.verbose {
            println!("🎬 Starting video merge process...");
        }

        let output = cmd.output().context("Failed to execute FFmpeg command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ProcessorError::FfmpegExecutionFailed(stderr.to_string()).into());
        }

        if self.verbose {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if !stdout.is_empty() {
                println!("FFmpeg stdout:\n{stdout}");
            }
            if !stderr.is_empty() {
                println!("FFmpeg stderr:\n{stderr}");
            }
        }

        Ok(())
    }

    /// Main processing function to merge video files
    pub fn merge_videos(&self, cli: &Cli) -> Result<()> {
        // Validate inputs
        cli.validate_inputs().context("Input validation failed")?;

        // Check FFmpeg availability
        self.check_ffmpeg_availability()
            .context("FFmpeg availability check failed")?;

        // Generate output path
        let output_path = cli
            .generate_output_path()
            .context("Failed to generate output path")?;

        if self.verbose {
            println!("📁 Input files: {:?}", cli.input_files);
            println!("📁 Output file: {}", output_path.display());
            println!("🎥 Video codec: {}", cli.get_video_codec());
            println!("🎵 Audio codec: {}", cli.get_audio_codec());
        }

        // Create temporary concat file
        let concat_file = self
            .create_concat_file(&cli.input_files)
            .context("Failed to create concat file")?;

        let concat_file_path = concat_file.path().to_path_buf();

        // Build and execute FFmpeg command
        let ffmpeg_cmd = self.build_ffmpeg_command(cli, &concat_file_path, &output_path);
        self.execute_ffmpeg_command(ffmpeg_cmd)
            .context("FFmpeg execution failed")?;

        // Verify output file was created
        if !output_path.exists() {
            return Err(anyhow::anyhow!(
                "Output file was not created: {}",
                output_path.display()
            ));
        }

        println!("✅ Video merge completed successfully!");
        println!("📄 Output file: {}", output_path.display());

        // Display output file size
        if let Ok(metadata) = std::fs::metadata(&output_path) {
            let size_mb = metadata.len() as f64 / 1024.0 / 1024.0;
            println!("📊 Output file size: {size_mb:.2} MB");
        }

        Ok(())
    }
}
