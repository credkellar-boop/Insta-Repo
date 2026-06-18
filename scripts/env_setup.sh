#!/bin/bash
if [ ! -f .env ]; then
    echo "GEMINI_API_KEY=your_key_here" > .env
    echo "GEMINI_MODEL=gemini-1.5-pro-latest" >> .env
    echo "[!] .env file created. Please update it with your API key."
else
    echo "[+] .env file already exists."
fi
