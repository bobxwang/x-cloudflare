# x-cloudflare

A minimal [Cloudflare Worker](https://developers.cloudflare.com/workers/) written in Rust using [workers-rs](https://github.com/cloudflare/workers-rs).

## Local Development

```bash
# Install wrangler CLI (requires Node.js)
npm install -g wrangler

# Preview locally
wrangler dev

# Visit http://localhost:8787
```

## Deploy to Cloudflare Workers

### Prerequisites

1. A [Cloudflare account](https://dash.cloudflare.com/sign-up)
2. A [Cloudflare API Token](https://dash.cloudflare.com/profile/api-tokens) with **Edit Cloudflare Workers** permission

### One-time Setup

```bash
# Login to Cloudflare
wrangler login

# Deploy
wrangler deploy
```

### Automatic Deployment via GitHub Actions

1. Push this repo to GitHub
2. In your GitHub repo settings, add a secret `CLOUDFLARE_API_TOKEN` with your [Cloudflare API Token](https://dash.cloudflare.com/profile/api-tokens)
3. Push to `main` — the workflow will automatically build and deploy

## Project Structure

```
├── .github/workflows/deploy.yml  # GitHub Actions CI/CD
├── src/lib.rs                    # Cloudflare Worker entry point
├── wrangler.toml                 # Cloudflare Workers configuration
├── Cargo.toml                    # Rust dependencies
└── .gitignore
```

## Tech Stack

- **Rust** 2024 edition
- **workers-rs** — official Cloudflare Rust SDK for Workers
- **worker-build** — Rust → Wasm build tool for Workers
