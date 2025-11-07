# Solana Web App

A Solana-focused web application built with Rust and Dioxus, featuring wallet connection and SOL transfer functionality.

## Features

- **Wallet Connection**: Connect your Solana wallet using the wallet adapter
- **SOL Transfer**: Transfer SOL to any Solana wallet address
- **Modern UI**: Built with Tailwind CSS and Dioxus for a responsive, modern interface

## Prerequisites

- Rust (1.83.0 or later)
- Node.js and npm (for building the wallet adapter and Tailwind CSS)
- Dioxus CLI

## Installation

1. Install Dioxus CLI:
```sh
cargo install dioxus-cli
```

2. Install Node.js dependencies:
```sh
npm install
cd wallet-adapter && npm install && cd ..
```

3. Build the wallet adapter:
```sh
cd wallet-adapter && npm run build && cd ..
```

4. Build Tailwind CSS:
```sh
npm run build:css
```

## Development

Run the development server:
```sh
dx serve
```

The app will be available at `http://localhost:8080`

## Building for Production

Build the web app:
```sh
dx build --release
```

The output will be in the `dist` directory.

## Vercel Deployment

To deploy this application to Vercel, follow these steps:

1. **Install Vercel CLI** (if not already installed):
```sh
npm i -g vercel
```

2. **Create a `vercel.json` configuration file** in the root directory:
```json
{
  "buildCommand": "npm run build:css && cd wallet-adapter && npm run build && cd .. && dx build --release",
  "outputDirectory": "dist",
  "installCommand": "npm install && cd wallet-adapter && npm install && cd .. && cargo install dioxus-cli",
  "framework": null
}
```

3. **Configure Vercel Project Settings**:
   - Go to your Vercel project settings
   - Under "Build & Development Settings":
     - **Build Command**: `npm run build:css && cd wallet-adapter && npm run build && cd .. && dx build --release`
     - **Output Directory**: `dist`
     - **Install Command**: `npm install && cd wallet-adapter && npm install && cd .. && cargo install dioxus-cli`
     - **Root Directory**: Leave empty (or set to `/` if needed)

4. **Deploy**:
```sh
vercel
```

Or push to your connected Git repository and Vercel will automatically build and deploy.

### Important Notes for Vercel:

- **Startup Command**: Vercel serves static files from the `dist` directory automatically. No startup command is needed as this is a static site.
- **Build Time**: The build process includes:
  1. Building Tailwind CSS
  2. Building the wallet adapter JavaScript bundle
  3. Building the Rust/Dioxus application to WebAssembly
- **Environment Variables**: If you need to configure RPC endpoints or other settings, add them in Vercel's environment variables section.

### Alternative: Using Vercel's Build Settings UI

If you prefer to configure via the Vercel dashboard:

1. Go to Project Settings → Build & Development Settings
2. Set:
   - **Framework Preset**: Other
   - **Build Command**: `npm run build:css && cd wallet-adapter && npm run build && cd .. && dx build --release`
   - **Output Directory**: `dist`
   - **Install Command**: `npm install && cd wallet-adapter && npm install && cd .. && cargo install dioxus-cli`

## Project Structure

```
.
├── src/
│   ├── components/     # Reusable UI components
│   ├── pages/         # Page components (Home, About)
│   ├── hooks/         # Custom hooks (wallet, gateway)
│   ├── gateway/       # Solana RPC gateway implementation
│   ├── route.rs       # Routing configuration
│   └── main.rs        # Application entry point
├── wallet-adapter/    # React-based wallet adapter
├── public/            # Static assets
└── dist/              # Build output (generated)

```

## License

This project is open source and available under the MIT License.
