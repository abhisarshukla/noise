use once_cell::sync::OnceCell;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub html_output: Option<PathBuf>,
}

impl AppContext {
    pub fn new(html_output: Option<String>) -> Self {
        Self {
            html_output: html_output.map(PathBuf::from),
        }
    }
}

static APP_CONTEXT: OnceCell<AppContext> = OnceCell::new();

pub fn init_context(context: AppContext) {
    APP_CONTEXT.set(context).expect("Context already initialized");
}

pub fn get_context() -> &'static AppContext {
    APP_CONTEXT.get().expect("Context not initialized")
}
