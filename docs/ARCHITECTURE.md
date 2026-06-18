# Insta-Repo Architecture

The scanner operates in a three-stage pipeline to balance speed with deep analysis.

1. **Traversal & Triage (Fast Path):** Utilizes `walkdir` to recursively map the directory. Large binaries (>5MB) are skipped to preserve memory.
2. **Static Heuristics (Regex Engine):** Text-based files are parsed against `rules/signatures.yaml`. This catches 90% of lazy payloads (base64 obfuscation, `curl | bash`, hardcoded wallet paths).
3. **Semantic Analysis (AI Engine - Optional):** Highly obfuscated or metamorphic payloads flagged in Stage 2 are dispatched to a reasoning model (Gemini 1.5 Pro / 3.1 Flash-Lite) via `src/ai_analyzer.rs`. The model decodes the intent and returns a structured threat assessment.
4. **Reporting:**
   Results are pushed to stdout or serialized to JSON via `src/reporter.rs` for CI/CD ingestion.
