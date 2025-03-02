# buddai

A personal assistant for helping you with whatever info you might or might not need.

## Prerequisites

1. Rust
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. You need to have an API key for whatever LLM you want to use.
Supported LLMs:
- [Google Gemini](https://aistudio.google.com/apikey)
- [DeepSeek R1](https://platform.deepseek.com/api_keys)

## Installation

```sh
./install.sh

# Export API key for the LLM you want to use
export GOOGLE_API_KEY=your_google_api_key
export DEEPSEEK_API_KEY=your_deepseek_api_key
```

## Usage

```sh
$ buddai 'Linux cmd to find some text in all files and folders in a directory'
[INFO] Gemini says: `grep -r "text" directory`

# Using DeepSeek
$ buddai -a deepseek 'delete everything in a dir'
[INFO] DeepSeek says: `rm -rf /path/to/dir/*`
```
