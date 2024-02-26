# Google Image DL

Download images from Google Image Search.

## Installation

Requires Rust to be installed. You can install it from [rustup.rs](https://rustup.rs/).

```bash
cargo install --git https://github.com/bencevans/google-image-dl
```

## Usage

An API key and a Custom Search Engine ID is required to use this tool. You can get them from the [Google Programmable Search Engine Control Panel](https://programmablesearchengine.google.com/controlpanel/).

```bash
google-image-dl --query="QUERY" \
                --api-key=YOUR_API_KEY \
                --engine-id=YOUR_ENGINE_ID \
                --target=500 \
                --output=./images
```

## License

MIT
