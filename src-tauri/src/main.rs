// Prescrire : coquille de bureau (macOS et Windows) autour de l'interface web.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose::STANDARD, Engine};
use std::{fs, path::PathBuf, sync::Mutex, time::UNIX_EPOCH};
use tauri::{Emitter, Manager};

/// Projets .caz demandés à l'ouverture (double-clic dans le Finder ou l'Explorateur).
struct Pending(Mutex<Vec<String>>);

fn is_caz(s: &str) -> bool {
    s.to_lowercase().ends_with(".caz")
}

/// Écriture sûre : fichier temporaire puis renommage, pour ne jamais laisser
/// un fichier à moitié écrit sur le dossier partagé.
fn write_atomic(path: &str, bytes: &[u8]) -> Result<(), String> {
    let p = PathBuf::from(path);
    if let Some(dir) = p.parent() {
        if !dir.as_os_str().is_empty() {
            fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
    }
    let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    let tmp = p.with_file_name(format!(".{}.{}.tmp", name, std::process::id()));
    fs::write(&tmp, bytes).map_err(|e| e.to_string())?;
    fs::rename(&tmp, &p).map_err(|e| {
        let _ = fs::remove_file(&tmp);
        e.to_string()
    })
}

#[tauri::command]
fn read_text(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_text(path: String, content: String) -> Result<(), String> {
    write_atomic(&path, content.as_bytes())
}

#[tauri::command]
fn read_b64(path: String) -> Result<String, String> {
    fs::read(&path).map(|b| STANDARD.encode(b)).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_b64(path: String, data: String) -> Result<(), String> {
    let bytes = STANDARD.decode(data).map_err(|e| e.to_string())?;
    write_atomic(&path, &bytes)
}

/// Date de modification en millisecondes, ou rien si le fichier n'existe pas.
#[tauri::command]
fn mtime(path: String) -> Option<u64> {
    let m = fs::metadata(&path).ok()?.modified().ok()?;
    Some(m.duration_since(UNIX_EPOCH).ok()?.as_millis() as u64)
}

#[tauri::command]
fn remove_file(path: String) -> Result<(), String> {
    match fs::remove_file(&path) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(serde::Serialize)]
struct Dirs {
    config: String,
    data: String,
    documents: Option<String>,
}

#[tauri::command]
fn app_dirs(app: tauri::AppHandle) -> Result<Dirs, String> {
    let p = app.path();
    let config = p.app_config_dir().map_err(|e| e.to_string())?;
    let data = p.app_data_dir().map_err(|e| e.to_string())?;
    let _ = fs::create_dir_all(&config);
    let _ = fs::create_dir_all(&data);
    Ok(Dirs {
        config: config.to_string_lossy().to_string(),
        data: data.to_string_lossy().to_string(),
        documents: p.document_dir().ok().map(|d| d.to_string_lossy().to_string()),
    })
}

/// Nom de la session (affiché dans les verrous et l'historique de la bibliothèque).
#[tauri::command]
fn whoami() -> String {
    std::env::var("USERNAME")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "poste inconnu".into())
}

#[tauri::command]
fn take_pending(state: tauri::State<Pending>) -> Vec<String> {
    std::mem::take(&mut *state.0.lock().unwrap())
}

fn main() {
    let initial: Vec<String> = std::env::args().skip(1).filter(|a| is_caz(a)).collect();

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            // Windows : un double-clic sur un .caz alors que Prescrire est déjà ouvert.
            let files: Vec<String> = argv.into_iter().skip(1).filter(|a| is_caz(a)).collect();
            if !files.is_empty() {
                app.state::<Pending>().0.lock().unwrap().extend(files);
                let _ = app.emit("open-file", ());
            }
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.unminimize();
                let _ = w.set_focus();
            }
        }))
        .plugin(tauri_plugin_dialog::init())
        .manage(Pending(Mutex::new(initial)))
        .invoke_handler(tauri::generate_handler![
            read_text, write_text, read_b64, write_b64, mtime, remove_file, app_dirs, whoami, take_pending
        ])
        .build(tauri::generate_context!())
        .expect("Prescrire n'a pas pu démarrer");

    app.run(|_app, _event| {
        // macOS : un double-clic sur un .caz dans le Finder.
        #[cfg(target_os = "macos")]
        if let tauri::RunEvent::Opened { urls } = &_event {
            let files: Vec<String> = urls
                .iter()
                .filter_map(|u| u.to_file_path().ok())
                .map(|p| p.to_string_lossy().to_string())
                .filter(|p| is_caz(p))
                .collect();
            if !files.is_empty() {
                _app.state::<Pending>().0.lock().unwrap().extend(files);
                let _ = _app.emit("open-file", ());
            }
        }
    });
}
