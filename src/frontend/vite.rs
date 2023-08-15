use std::{collections::HashMap, rc::Rc};

use indoc::formatdoc;

use super::StaticFiles;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestEntry {
    file: String,
    css: Option<Vec<String>>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not find dev server information, make sure you are running `pnpm dev`")]
    HrmServerNotFound,
    #[error("Could not find build manifest, make sure you've run `pnpm build`")]
    BuildManifestNotFound,
    #[error("Failed to parse manifest: {0}")]
    FailedToParseManifest(#[from] serde_json::Error),
}

#[derive(Debug)]
pub enum Vite {
    Production {
        manifest: HashMap<String, ManifestEntry>,
    },
    Development {
        dev_server: String,
    },
}

impl Vite {
    pub fn new() -> Result<Self, Error> {
        if cfg!(debug_assertions) {
            let dev_server = StaticFiles::get(".vite-dev")
                .ok_or_else(|| {
                    anyhow::anyhow!(
                    "Could not find dev server information, make sure you are running `pnpm dev`"
                )
                })
                .map_err(|_| Error::HrmServerNotFound)?;

            Ok(Vite::Development {
                dev_server: String::from_utf8_lossy(&dev_server.data).to_string(),
            })
        } else {
            let manifest = StaticFiles::get("manifest.json")
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Could not find build manifest, make sure you've run `pnpm build`"
                    )
                })
                .map_err(|_| Error::BuildManifestNotFound)?;

            Ok(Vite::Production {
                manifest: serde_json::from_str(&String::from_utf8_lossy(&manifest.data))?,
            })
        }
    }

    pub fn asset(&self, path: &str) -> Option<String> {
        match self {
            Vite::Development { dev_server } => Some(format!(
                r#"<script type="module" src="{dev_server}/{path}"></script>"#
            )),
            Vite::Production { manifest } => {
                let entry = manifest.get(path)?;

                let css_imports = entry
                    .css
                    .as_ref()
                    .map(|css_files| {
                        css_files
                            .iter()
                            .map(|css_file| {
                                format!(r#"<link rel="stylesheet" href="/{css_file}" />"#)
                            })
                            .collect::<Rc<_>>()
                            .join("\n")
                    })
                    .unwrap_or_default();

                Some(format!(
                    r#"{css_imports}<script type="module" src="/{}"></script>"#,
                    entry.file
                ))
            }
        }
    }
    pub fn dev_scripts(&self) -> Option<String> {
        let Vite::Development { dev_server } = self else {
            return None;
        };

        Some(formatdoc! {
            r#"
                <script type="module" src="{dev_server}/@vite/client"></script>
                <script type="module">
                    import RefreshRuntime from '{dev_server}/@react-refresh'
                    RefreshRuntime.injectIntoGlobalHook(window)
                    window.$RefreshReg$ = () => {{}}
                    window.$RefreshSig$ = () => (type) => type
                    window.__vite_plugin_react_preamble_installed__ = true
                </script>
            "#
        })
    }
}
