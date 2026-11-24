# WOAH desktop app

The WOAH dashboard as a normal app for Mac and Windows. It is a small window (made with Tauri) that shows the live
dashboard website, so it never needs updating when the website changes. Links to Gmail, Google Sheets and Drive open
in the computer's normal browser.

**Download:** the newest `.dmg` (Mac) and `-setup.exe` (Windows) are on the **Releases** page of this repo, and
behind the Download buttons on the dashboard's *Get the app* page.

**Not signed:** the app has no paid Apple / Microsoft signature, so the first time it opens:
- Mac: open the .dmg, drag WOAH into Applications, open it once (it is blocked), then System Settings →
  Privacy & Security → scroll down → **Open Anyway**.
- Windows: on "Windows protected your PC" click **More info** → **Run anyway**.

**Making a new version** (only needed if the window, icon or link handling changes):
1. Change the files, and raise `version` in `src-tauri/tauri.conf.json` (e.g. 1.0.0 → 1.0.1).
2. Commit to `main`. GitHub builds both installers by itself (Actions tab, about 10–15 minutes) and adds them to Releases.
3. Update the file names in the Download buttons on the dashboard's `install.html` if the version changed.

Files: `src-tauri/src/lib.rs` (the window and link handling), `src-tauri/tauri.conf.json` (name, version, installer
settings), `icon.png` (the app icon, 1024×1024), `.github/workflows/build.yml` (the GitHub build).
