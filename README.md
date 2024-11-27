# YouTube Chat CLI

## Overview

YouTube Chat is a Rust-based command-line application that allows you to interact with YouTube video content using an AI-powered chat interface. The application fetches video information, including its transcript, and enables you to ask questions about the video using a local AI model.

## Features

- Fetch YouTube video details using the YouTube Data API
- Retrieve video transcripts
- Interactive AI-powered chat interface
- Uses Ollama for local AI model inference
- Supports multiple languages

## Prerequisites

Before running the application, ensure you have the following installed:

- Rust (latest stable version)
- Cargo
- Ollama
- An active YouTube Data API key

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/youtube-chat.git
   cd youtube-chat
   ```

2. Create a `.env` file in the project root and add your YouTube API key:

   ```
   YOUTUBE_API_KEY=your_youtube_api_key_here
   ```

3. Install dependencies:
   ```bash
   cargo build
   ```

## Usage

1. Ensure you have Ollama running with the `llama3` model available.

2. Run the application:

   ```bash
   cargo run
   ```

3. When prompted, enter a YouTube video URL.

4. The application will:

   - Fetch video metadata
   - Retrieve the video transcript
   - Start an interactive chat session about the video

5. Type your questions about the video.

6. Type `exit` to end the chat session.

## Configuration

- The default AI model is set to `llama3` in `lib.rs`
- You can modify the model by changing the `MODEL` constant

## Dependencies

- `ollama-rs`: Ollama API integration
- `reqwest`: HTTP requests
- `tokio`: Async runtime
- `ytranscript`: YouTube transcript retrieval
- `dotenv`: Environment variable management
- `serde_json`: JSON parsing
- `futures`: Async stream handling

## Roadmap

- [ ] Add support for multiple language transcripts
- [ ] Implement more robust error handling
- [ ] Add configurable AI model selection
- [ ] Enhance text wrapping functionality

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
(please please please contribute, this code sucks. this was my first rust project💀)

## License

This project is licensed under the [MIT License](./LICENSE).

## Disclaimer

This project is for educational purposes and relies on third-party APIs and services.
