# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

AIG (AI Gauge) is a Rust CLI tool that tracks and reports daily API costs/usage across LLM providers (OpenAI, Anthropic, Google Gemini). It aggregates usage data concurrently and outputs summaries in plain text or JSON.

## Build and Development Commands

テスト・動作確認はすべてDocker上で実行すること。

```bash
docker compose build                              # コンテナビルド
docker compose run aig cargo test                  # 全テスト実行
docker compose run aig cargo test --lib            # ユニットテストのみ
docker compose run aig cargo test test_name        # 単一テスト実行
docker compose run aig cargo clippy                # Lint
docker compose run aig cargo fmt                   # Format
docker compose run aig cargo run                   # CLI実行
docker compose run aig bash                        # コンテナ内シェル
```

## Architecture

**Entry point:** `src/main.rs` — parses CLI args, loads config, spawns concurrent provider fetches via `futures::join_all()`, aggregates results.

**Provider trait pattern:** `src/providers/mod.rs` defines an `async_trait` `Provider` with `name()` and `fetch_today_usage()`. Each provider (Anthropic, OpenAI, Gemini) implements this in its own file under `src/providers/`. Adding a new provider means implementing this trait and wiring it in `main.rs`.

**Configuration:** `src/config.rs` loads from `~/.config/aig/config.toml`. Environment variables (`ANTHROPIC_API_KEY`, `GEMINI_API_KEY`, `OPENAI_API_KEY`) override config file values. Only providers with valid API keys are instantiated.

**CLI:** `src/cli.rs` uses clap derive macros. `--json` flag for JSON output, `list` subcommand for per-model breakdown.

**TLS:** Uses `rustls-tls` (not native OpenSSL) to avoid runtime dependencies in containers.

## Testing

Integration tests in `tests/` use `mockito` for HTTP mocking and `tokio::test` for async. Tests cover CLI parsing, config deserialization, and each provider's success/error paths.
