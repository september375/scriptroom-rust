# Scriptroom 3.1 — Rust Desktop Edition

This is the existing Literary Edition website packaged in a native Rust/Tauri 2 desktop host. It uses Windows' Microsoft Edge WebView2 runtime instead of bundling Electron's own Chromium. It is not a pure Rust rewrite: HTML/CSS/JS, Supabase's JavaScript client and PDF.js remain responsible for the UI, storage/authentication and PDF rendering. The Rust entry point is in `src-tauri/src/main.rs`.

## Configure before building

Copy your working website's `config.js` into `frontend/`, replacing the included placeholder. The known project URL is already filled in. Use only your Supabase publishable or legacy anon key. Never use a secret/service-role key or your admin password. No new Supabase database, SQL migration, or new admin account is required. The existing database policies still enforce permissions. `supabase-setup.sql` is included for reference only; do not re-run it on a working project.

## Build locally (Windows 10 x64)

1. Install Microsoft C++ Build Tools (or open Visual Studio Installer and modify your existing Visual Studio installation): select Desktop development with C++, MSVC x64/x86 tools and a Windows SDK. https://visualstudio.microsoft.com/visual-cpp-build-tools/
2. Install Rust via https://rustup.rs/ and select the default MSVC toolchain. Reopen CMD. If needed, run `rustup default stable-x86_64-pc-windows-msvc`.
3. Install Microsoft Edge WebView2 Evergreen Runtime if it is not already installed: https://developer.microsoft.com/microsoft-edge/webview2/
4. Install Node.js LTS if needed (the Node CLI is a build tool, not a runtime requirement for the installed app): https://nodejs.org/
5. In CMD, open the extracted folder containing package.json. Run:

```cmd
rustc --version
cargo --version
node --version
npm install
npm start
```

Close the development window or press Ctrl+C in CMD before building. To compile the release EXE and setup installer:

```cmd
npm run build
```

Outputs (default target directory):
- `src-tauri\target\release\scriptroom.exe`
- `src-tauri\target\release\bundle\nsis\Scriptroom_*_x64-setup.exe`

To skip installer creation: `npm run build:exe`. The executable still needs the WebView2 runtime. Rust compilation can use considerable CPU, RAM and disk space. On low-memory PCs, first run `set CARGO_BUILD_JOBS=1` in CMD (it can be slower), or use the CI option below. The installer is unsigned, so Windows SmartScreen may warn.

## Build on GitHub instead (recommended for a 4 GB RAM PC)

Create a separate GitHub repository, e.g. `scriptroom-rust`. Keeping your web repository separate avoids disturbing GitHub Pages. Copy your working config.js into frontend/ before uploading. Only publishable/anon credentials belong in the repo.

Upload the *contents* of the extracted folder, including the hidden `.github/workflows/build-windows.yml` and `.gitignore` (git or GitHub Desktop is easiest for hidden files). Do not upload `node_modules` or `target`. In the repository's Actions tab, select **Build Scriptroom for Windows**, click **Run workflow**, select your branch and click Run workflow. After the job completes, open the run and download the `Scriptroom-Windows-x64` artifact. Extract it to access the application and NSIS setup executable. GitHub Actions availability/quotas depend on your repository/account. The workflow is not run automatically on pushes.

## Notes

- Same literary dark/light theme, search, admin login, PDF upload/delete, fullscreen and Eco/Sharp PDF reader.
- The frontend is embedded in the EXE; changes to your GitHub Pages website do not update it automatically. Rebuild after frontend changes. If you want to deploy website updates, use the files in `frontend/` separately.
- Internet access is needed for Supabase, the Supabase client loaded from esm.sh and PDF.js loaded from jsDelivr. The desktop CSP allows these dependencies and standard supabase.co projects. Custom domains require adjusting CSP in `tauri.conf.json`.
- The original PDF link opens according to the platform WebView's new-window behavior. You can always read PDFs within Scriptroom's own reader.
- No app-specific native IPC commands or filesystem/shell plugins are exposed. Admin privileges remain server-side; public PDFs may be downloaded.
- Tauri avoids bundling a separate Chromium runtime, but still uses Chromium-based WebView2. PDF rendering can still require significant resources. Actual performance depends on your PC.
- Validation performed: archive integrity, JSON/TOML parsing, JavaScript syntax and asset references. Windows compilation and live runtime testing were not available in the creation environment; the source and CI workflow are supplied for building and testing.
