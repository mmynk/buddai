# buddai

A personal assistant for helping you with whatever info you might or might not need.

Uses Google Gemini API to query the Gemini AI assistant.

## Installation

```bash
cargo install buddai
echo "GOOGLE_API_KEY=your_google_api_key" > ~/.env
``` 

## Usage

```bash
$ buddai -q 'Linux cmd to find some text in all files and folders in a directory'
Gemini says: `grep -r "text" directory`
```
