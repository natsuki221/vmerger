use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "vmerger")]
#[command(author = "natsuki221<linnatsuki221@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "A command-line tool for merging video files using FFmpeg")]
#[command(
    long_about = "vmerger is a high-performance CLI tool that merges multiple video files into a single file and provides format conversion options. It leverages Rust's system programming capabilities and directly calls external FFmpeg programs to ensure maximum execution efficiency and resource control."
)]
pub struct Cli {
    /// Input video files to merge
    #[arg(required = true, help = "Input video files to merge")]
    pub input_files: Vec<PathBuf>,

    /// Output format (e.g., mp4, avi, mov, mkv)
    #[arg(
        short = 'F',
        long = "format",
        help = "Output format (e.g., mp4, avi, mov, mkv)"
    )]
    pub output_format: Option<String>,

    /// Output file path
    #[arg(short = 'O', long = "output", help = "Output file path")]
    pub output_path: Option<PathBuf>,

    /// Verbose output
    #[arg(short, long, help = "Enable verbose output")]
    pub verbose: bool,

    /// Codec for video stream
    #[arg(
        long = "video-codec",
        help = "Video codec to use (e.g., libx264, libx265, copy)"
    )]
    pub video_codec: Option<String>,

    /// Codec for audio stream
    #[arg(
        long = "audio-codec",
        help = "Audio codec to use (e.g., aac, mp3, copy)"
    )]
    pub audio_codec: Option<String>,

    /// Quality/bitrate for video
    #[arg(
        short = 'q',
        long = "quality",
        help = "Video quality/bitrate (e.g., 1M, 2000k)"
    )]
    pub video_quality: Option<String>,
}

impl Cli {
    /// Generate output filename based on input files and format
    pub fn generate_output_path(&self) -> anyhow::Result<PathBuf> {
        if let Some(ref output_path) = self.output_path {
            return Ok(output_path.clone());
        }

        let first_input = self
            .input_files
            .first()
            .ok_or_else(|| anyhow::anyhow!("No input files provided"))?;

        let stem = first_input
            .file_stem()
            .ok_or_else(|| anyhow::anyhow!("Invalid input filename"))?
            .to_string_lossy();

        let format = self.output_format.as_deref().unwrap_or("mp4");
        let output_filename = format!("{stem}_merged.{format}");

        Ok(PathBuf::from(output_filename))
    }

    /// Validate input files exist and are accessible
    pub fn validate_inputs(&self) -> anyhow::Result<()> {
        if self.input_files.is_empty() {
            return Err(anyhow::anyhow!("No input files provided"));
        }

        for file in &self.input_files {
            if !file.exists() {
                return Err(anyhow::anyhow!(
                    "Input file does not exist: {}",
                    file.display()
                ));
            }
            if !file.is_file() {
                return Err(anyhow::anyhow!(
                    "Input path is not a file: {}",
                    file.display()
                ));
            }
        }

        Ok(())
    }

    /// Get the appropriate video codec based on user input and output format
    pub fn get_video_codec(&self) -> String {
        if let Some(ref codec) = self.video_codec {
            codec.clone()
        } else if let Some(ref format) = self.output_format {
            match format.to_lowercase().as_str() {
                "mp4" => "libx264".to_string(),
                "mkv" => "libx264".to_string(),
                "avi" => "libxvid".to_string(),
                "mov" => "libx264".to_string(),
                _ => "copy".to_string(),
            }
        } else {
            "copy".to_string()
        }
    }

    /// Get the appropriate audio codec based on user input and output format
    pub fn get_audio_codec(&self) -> String {
        if let Some(ref codec) = self.audio_codec {
            codec.clone()
        } else if let Some(ref format) = self.output_format {
            match format.to_lowercase().as_str() {
                "mp4" => "aac".to_string(),
                "mkv" => "aac".to_string(),
                "avi" => "mp3".to_string(),
                "mov" => "aac".to_string(),
                _ => "copy".to_string(),
            }
        } else {
            "copy".to_string()
        }
    }
}
