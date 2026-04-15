# StreamDeck - Unified Entertainment Hub for Windows

StreamDeck is a feature-rich, unified digital entertainment hub built with **Tauri**, **Node.js**, and vanilla web technologies. It aggregates live TV, movies, live sports events, and TV show catalogs using TMDB (The Movie Database) to surface top-trending media content and providing seamless integration through multiple streaming providers.

## Features

- **Live TV & Sports Integration:** Seamlessly aggregates live sporting events and television networks alongside traditional stream sources.
- **Movies & Shows Discovery:** Powered by TMDB API for rich, real-time metadata covering Trending, Now Playing, Top Rated, and localized content (e.g., Trending in India).
- **Embedded Webviews:** Launches content directly in embedded players giving you control over UI interactions, zoom levels, and media continuity.
- **Settings & API Configuration:** Includes a built-in UI for storing your TMDB API keys locally so you never have to hardcode them.

## Technology Stack

- **Tauri** — Blazing fast, lightweight, and secure desktop application framework.
- **Rust** — Powers the low-level backend and desktop capabilities.
- **Vanilla JS/HTML/CSS** — For a pristine, highly-performant frontend unburdened by heavy JS frameworks. 

## Prerequisites

Before building StreamDeck locally, make sure you have the following installed to support Tauri development:

- [Node.js](https://nodejs.org/en/)
- [Rust](https://www.rust-lang.org/tools/install)
- [C++ Build Tools for Windows](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

*Note for Windows users: You can find further details about configuring Tauri for Windows in the [official Tauri prerequisites guide](https://tauri.app/v1/guides/getting-started/prerequisites).*

## Installation & Setup

1. **Clone the repository:**
   ```bash
   git clone <your-repository-url>
   cd streamdeck
   ```

2. **Install frontend dependencies:**
   ```bash
   npm install
   ```

3. **Configure your TMDB API Key:**
   To access live movie and TV data, you need an API key from TMDB. 
   - You can simply launch the app, click the **Settings icon** located in the sidebar or header, and paste your API key in the UI. 
   - Your key is securely saved in your browser's `localStorage` and will not be pushed to your codebase.

4. **Run the application in Development Mode:**
   ```bash
   npm run dev
   ```
   *(This script will transparently execute `tauri dev`, compile the Rust backend, and bundle the frontend.)*

## Building for Production

To compile StreamDeck into a self-contained executable for distribution on Windows:

```bash
npm run tauri build
```

This will output the compiled binaries directly into `src-tauri/target/release/` which you can then distribute.

## Contributing

Pull requests are welcome! If you're contributing code:
1. Ensure API keys or any sensitive information remain out of commits.
2. Use standard coding conventions matching the current application structure.
3. Test layout regressions using `npm run dev`.

## License

This project is licensed under the MIT License.
