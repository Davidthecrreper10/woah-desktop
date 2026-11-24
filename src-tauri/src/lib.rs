// The WOAH desktop app: one window that shows the live WOAH dashboard website.
// Nothing about the dashboard lives in here, so the app never needs updating when the website changes.
// Links that leave the dashboard (Gmail, Google Sheets, Drive, email addresses) open in the computer's normal browser or mail app.
use tauri::{WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;

const SITE: &str = "https://davidthecrreper10.github.io/woah-dashboard/";
const SITE_HOST: &str = "davidthecrreper10.github.io";

// The website opens most links in a new tab. Inside the app there are no tabs, so a link click or window.open()
// becomes a normal navigation, which on_navigation below sends to the browser when it leaves the dashboard.
const INIT: &str = r#"
(function () {
  function go(u) { try { var url = new URL(u, location.href); location.href = url.href; } catch (e) {} return null; }
  window.open = function (u) { if (u) go(u); return null; };
  document.addEventListener('click', function (e) {
    var a = e.target && e.target.closest ? e.target.closest('a[href]') : null;
    if (!a || e.defaultPrevented) return;
    var href = a.getAttribute('href') || '';
    if (!href || href.charAt(0) === '#' || /^javascript:/i.test(href)) return;
    var t = (a.getAttribute('target') || (document.querySelector('base[target]') || {}).target || '').toLowerCase();
    if (t && t !== '_self' && t !== '_top' && t !== '_parent') { e.preventDefault(); go(a.href); }
  }, true);
})();
"#;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(SITE.parse().expect("site url")))
                .title("WOAH")
                .inner_size(1280.0, 840.0)
                .min_inner_size(380.0, 560.0)
                .initialization_script(INIT)
                .on_navigation(move |url| {
                    let inside = url.host_str() == Some(SITE_HOST)
                        || matches!(url.scheme(), "tauri" | "about" | "data" | "blob");
                    if !inside {
                        let _ = handle.opener().open_url(url.as_str(), None::<&str>);
                    }
                    inside
                })
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running WOAH");
}
