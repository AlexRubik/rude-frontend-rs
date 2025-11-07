# Solana Web App - Rust Frontend Boilerplate

A modern, Rust-based web application for Solana blockchain interactions. Built with Dioxus, this boilerplate provides a clean starting point for building Solana dApps with a native Rust frontend.

## Features

- 🦀 **Pure Rust Frontend** - Built entirely with Rust using the Dioxus framework
- 🔗 **Solana Wallet Integration** - Seamless wallet connection using Solana Wallet Adapter
- 💸 **SOL Transfer** - Built-in functionality for transferring SOL tokens
- 🎨 **Modern UI** - Beautiful, responsive design with Tailwind CSS
- 🚀 **Fast & Efficient** - WebAssembly-powered performance
- 📱 **Responsive** - Works great on desktop and mobile devices

## Project Structure

```
.
├── src/
│   ├── components/       # Reusable UI components
│   │   ├── app_layout.rs
│   │   ├── navbar.rs
│   │   └── wallet_adapter.rs
│   ├── hooks/           # Custom hooks for state management
│   │   └── use_wallet.rs
│   ├── pages/           # Application pages
│   │   ├── home.rs      # SOL transfer page
│   │   ├── about.rs     # About page
│   │   └── not_found.rs
│   ├── main.rs          # Application entry point
│   └── route.rs         # Routing configuration
├── wallet-adapter/      # JavaScript wallet adapter bridge
│   ├── src/
│   │   ├── main.js
│   │   └── styles.css
│   └── package.json
├── public/              # Static assets
├── Cargo.toml          # Rust dependencies
├── Dioxus.toml         # Dioxus configuration
└── vercel.json         # Vercel deployment config
```

## Prerequisites

- **Rust** (latest stable version)
- **Node.js** (v18 or higher)
- **npm** or **yarn**
- **Dioxus CLI** - Install with: `cargo install dioxus-cli`

## Local Development

### 1. Clone the repository

```bash
git clone <your-repo-url>
cd solana-web-app
```

### 2. Install Rust target for WebAssembly

```bash
rustup target add wasm32-unknown-unknown
```

### 3. Install Node dependencies

```bash
npm install
```

### 4. Build the wallet adapter

```bash
cd wallet-adapter
npm install
npm run build
cd ..
```

### 5. Build Tailwind CSS

```bash
npm run build:css
```

### 6. Run the development server

```bash
dx serve --hot-reload
```

The application will be available at `http://localhost:8080`

## Building for Production

To create an optimized production build:

```bash
npm run build
```

This will:
1. Build the Tailwind CSS (minified)
2. Build the wallet adapter JavaScript bundle
3. Compile the Rust code to WebAssembly
4. Output everything to the `dist/` directory

## Deploying to Vercel

This application is configured for easy deployment to Vercel.

### Method 1: Deploy via Vercel CLI

1. Install Vercel CLI:
   ```bash
   npm install -g vercel
   ```

2. Deploy:
   ```bash
   vercel
   ```

### Method 2: Deploy via Vercel Dashboard

1. Push your code to a Git repository (GitHub, GitLab, or Bitbucket)

2. Import the project in Vercel:
   - Go to [vercel.com](https://vercel.com)
   - Click "Add New Project"
   - Import your Git repository

3. Configure the project:
   - **Framework Preset:** Other
   - **Root Directory:** `./` (leave as root)
   - **Build Command:** (automatically detected from vercel.json)
   - **Output Directory:** `dist` (automatically detected from vercel.json)
   - **Install Command:** `npm install` (automatically detected from vercel.json)

4. Click "Deploy"

### Important Vercel Configuration

The `vercel.json` file contains all necessary configuration:

```json
{
  "buildCommand": "bash vercel-build.sh",
  "outputDirectory": "dist",
  "framework": null,
  "devCommand": null,
  "installCommand": "npm install"
}
```

**Key Points:**
- The build command executes `vercel-build.sh` which handles the complete build process
- The build script installs Rust, the WASM target, Dioxus CLI, and builds the entire project
- Output directory is set to `dist`
- SPA routing is configured to serve `index.html` for all routes

**Build Script (vercel-build.sh):**
The build logic is extracted to a separate script to comply with Vercel's 256-character
limit on buildCommand. The script:
1. Installs Rust toolchain via rustup
2. Adds wasm32-unknown-unknown target for WebAssembly compilation
3. Installs Dioxus CLI for building the application
4. Installs Node.js dependencies
5. Runs the complete build process

### Vercel Settings Summary

When configuring in Vercel Dashboard:

- ✅ **Root Directory:** `./` (project root)
- ✅ **Build Command:** Automatically detected from `vercel.json`
- ✅ **Output Directory:** `dist`
- ✅ **Install Command:** `npm install`
- ✅ **Framework:** Other/None

## Environment Variables

For production deployments, you may want to configure:

- **RPC_ENDPOINT** - Solana RPC endpoint (defaults to mainnet-beta in code)

You can set these in:
- Vercel: Project Settings → Environment Variables
- Local: Create a `.env` file (not tracked in git)

## Technologies Used

- **[Dioxus](https://dioxuslabs.com/)** - Rust GUI library for web, desktop, and mobile
- **[Solana](https://solana.com/)** - High-performance blockchain
- **[Solana Wallet Adapter](https://github.com/solana-labs/wallet-adapter)** - Wallet integration
- **[Tailwind CSS](https://tailwindcss.com/)** - Utility-first CSS framework
- **[WebAssembly](https://webassembly.org/)** - Binary instruction format for web

## How It Works

### Wallet Connection

The application uses a JavaScript bridge (`wallet-adapter/`) that wraps the Solana Wallet Adapter. The Rust code communicates with this bridge via:

1. **JavaScript → Rust**: Custom events dispatch wallet state changes
2. **Rust → JavaScript**: `eval()` calls JavaScript functions for signing transactions

### SOL Transfer Flow

1. User connects wallet (JavaScript wallet adapter)
2. User enters recipient address and amount
3. Rust code creates a Solana transfer instruction
4. Transaction is serialized and sent to JavaScript for signing
5. User approves in wallet popup
6. Signed transaction is sent to Solana network
7. Confirmation is displayed to user

## Customization

### Changing RPC Endpoint

Edit `wallet-adapter/src/main.js`:

```javascript
const endpoint = "https://api.mainnet-beta.solana.com";
// Change to devnet: "https://api.devnet.solana.com"
// Or your own RPC
```

### Styling

The application uses Tailwind CSS. Modify:
- `tailwind.config.js` - Tailwind configuration
- `input.css` - Custom CSS and animations
- Component classes in `.rs` files

### Adding New Pages

1. Create a new file in `src/pages/`
2. Add the component to `src/pages/mod.rs`
3. Add a route in `src/route.rs`
4. Add navigation link in `src/components/navbar.rs`

## Troubleshooting

### Build Issues

If you encounter build errors:

```bash
# Clean build artifacts
cargo clean
rm -rf target/
rm -rf dist/

# Rebuild
npm run build
```

### Wallet Connection Issues

- Ensure you have a Solana wallet extension installed (Phantom, Solflare, etc.)
- Check browser console for errors
- Try refreshing the page
- Verify the RPC endpoint is accessible

### Vercel Deployment Issues

- Check that `vercel.json` is in the root directory
- Verify build logs in Vercel dashboard
- Ensure all environment variables are set correctly
- Check that the output directory contains `index.html`

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Resources

- [Dioxus Documentation](https://dioxuslabs.com/learn/0.6/)
- [Solana Documentation](https://docs.solana.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Rust Book](https://doc.rust-lang.org/book/)

## Support

For issues and questions:
- Open an issue on GitHub
- Check existing issues and discussions
- Review the documentation

---

Built with ❤️ using Rust and Dioxus
