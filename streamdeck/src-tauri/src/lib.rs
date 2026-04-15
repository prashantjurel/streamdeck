use tauri::{Manager, Listener};
use std::sync::OnceLock;

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn get_client() -> &'static reqwest::Client {
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert("Accept", "application/json, text/plain, */*".parse().unwrap());
                headers
            })
            .timeout(std::time::Duration::from_secs(30))
            .pool_max_idle_per_host(0) // CRITICAL: Disable pooling to avoid 10054 Connection Reset on Windows
            .build()
            .expect("Failed to build persistent reqwest client")
    })
}

#[derive(serde::Deserialize, Clone, Debug)]
struct ScriptPayload {
    label: String,
    script: String,
}

#[tauri::command]
fn execute_script(app: tauri::AppHandle, label: String, script: String) -> Result<(), String> {
    if let Some(webview) = app.get_webview(&label) {
        webview.eval(&script).map_err(|e| {
            println!("Error evaluating script: {}", e);
            e.to_string()
        })?;
        Ok(())
    } else {
        println!("Webview not found for label: {}", label);
        Err("Webview not found".into())
    }
}

#[tauri::command]
fn get_webview_url(app: tauri::AppHandle, label: String) -> Result<String, String> {
    if let Some(webview) = app.get_webview(&label) {
        Ok(webview.url().map_err(|e| e.to_string())?.to_string())
    } else {
        Err("Webview not found".into())
    }
}

#[tauri::command]
fn get_webview_title(app: tauri::AppHandle, label: String) -> Result<String, String> {
    // Try to get as a WebviewWindow first
    if let Some(window) = app.get_webview_window(&label) {
        return Ok(window.title().map_err(|e| e.to_string())?);
    }
    
    // Fallback for child webviews: return a placeholder to avoid "Window not found" 
    // which triggers the UI fallback logic unnecessarily.
    if let Some(_webview) = app.get_webview(&label) {
        return Ok("Streaming Content".into());
    }

    Err("Webview not found".into())
}

#[tauri::command]
fn set_webview_url(app: tauri::AppHandle, label: String, url: String) -> Result<(), String> {
    if let Some(webview) = app.get_webview(&label) {
        webview.navigate(tauri::Url::parse(&url).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Webview not found".into())
    }
}

#[derive(serde::Deserialize)]
struct FetchOptions {
    url: String,
    method: Option<String>,
    headers: Option<std::collections::HashMap<String, String>>,
    body: Option<String>,
}

#[tauri::command]
async fn native_fetch(options: FetchOptions) -> Result<String, String> {
    let client = get_client();
    
    let method = match options.method.as_deref().unwrap_or("GET").to_uppercase().as_str() {
        "POST" => reqwest::Method::POST,
        "PUT" => reqwest::Method::PUT,
        "DELETE" => reqwest::Method::DELETE,
        _ => reqwest::Method::GET,
    };

    let mut rb = client.request(method, &options.url);

    // Apply headers
    if let Some(headers) = options.headers {
        for (k, v) in headers {
            if let Ok(hk) = reqwest::header::HeaderName::from_bytes(k.as_bytes()) {
                if let Ok(hv) = reqwest::header::HeaderValue::from_str(&v) {
                    rb = rb.header(hk, hv);
                }
            }
        }
    }

    // Apply body
    if let Some(body) = options.body {
        rb = rb.body(body);
    }

    let res = rb.send().await
        .map_err(|e| {
            println!("[NativeFetch] Error sending request: {:?}", e);
            format!("Request error: {:?}", e)
        })?;

    let body = res.text().await
        .map_err(|e| format!("Body read error: {:?}", e))?;

    Ok(body)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .setup(|app| {
            let handle = app.handle().clone();
            app.listen("execute-script-event", move |event: tauri::Event| {
                if let Ok(payload) = serde_json::from_str::<ScriptPayload>(event.payload()) {
                    if let Some(webview) = handle.get_webview(&payload.label) {
                        let _ = webview.eval(&payload.script);
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![execute_script, get_webview_url, get_webview_title, native_fetch, set_webview_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
