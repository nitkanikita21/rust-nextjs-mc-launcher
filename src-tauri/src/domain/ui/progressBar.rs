use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Clone, Deserialize)]
struct ProgressBarDisplay {
    title: String,
    displayValue: String,
}

#[derive(Clone, Deserialize)]
struct ProgressBarState {
    value: u8,
    display: ProgressBarDisplay,
}

#[derive(Deserialize, Clone)]
struct ProgressBarChangeState {
    barId: String,
    state: ProgressBarState
}

pub struct ProgressBarUiLinker<'a> {
    app_handle: &'a AppHandle,
    id: String,

    state: ProgressBarState,
}

impl ProgressBarUiLinker<'_> {
    pub fn new<'a>(app_handle: &'a AppHandle, id: String) -> Self {
        Self {
            app_handle,
            id,
            state: ProgressBarState {
                value: 0, display: ProgressBarDisplay { title: "".into(), displayValue: "".into() }
            },
        }
    }
}
