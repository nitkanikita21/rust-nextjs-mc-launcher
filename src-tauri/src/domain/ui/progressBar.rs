use serde::{Serialize};
use tauri::{AppHandle, Manager, Runtime};

#[derive(Clone, Serialize)]
struct ProgressBarDisplay {
    title: String,
    display_value: String,
}

#[derive(Clone, Serialize)]
struct ProgressBarState {
    value: u8,
    display: ProgressBarDisplay,
}

#[derive(Serialize, Clone)]
struct ProgressBarChangeState {
    bar_id: String,
    state: ProgressBarState,
}

pub struct ProgressBarUiLinker<'a, R: Runtime> {
    app_handle: &'a AppHandle<R>,
    id: String,
    title: String,
}

impl<'a, R: Runtime> ProgressBarUiLinker<'a, R> {
    pub fn new(app_handle: &'a AppHandle<R>, id: String) -> Self {
        Self {
            app_handle,
            id,
            title: "".into(),
        }
    }
    pub fn title(&mut self, title: String) {
        self.title = title;
    }
    pub fn push_value(&self, value: u8, display: String) -> Result<(), anyhow::Error> {
        self.app_handle.emit_all(
            "progress-bar-state-change",
            ProgressBarChangeState {
                bar_id: self.id.clone(),
                state: ProgressBarState {
                    value,
                    display: ProgressBarDisplay {
                        title: self.title.clone(),
                        display_value: display,
                    },
                },
            },
        )?;
        Ok(())
    }
}
