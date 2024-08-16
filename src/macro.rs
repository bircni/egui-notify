/// todo docs
#[cfg(feature = "macros")]
pub static TOASTS: once_cell::sync::Lazy<egui::mutex::RwLock<crate::Toasts>> =
    once_cell::sync::Lazy::new(|| {
        egui::mutex::RwLock::new(crate::Toasts::new().with_anchor(crate::Anchor::TopRight))
    });

#[macro_export]
#[cfg(feature = "macros")]
/// todo macro
macro_rules! toast{
    (Basic, $($format:expr),+) => {
        $crate::macros::TOASTS.write().basic(format!($($format),+));
    };
    (Info, $($format:expr),+) => {
        $crate::macros::TOASTS.write().info(format!($($format),+));
    };
    (Warning, $($format:expr),+) => {
        $crate::macros::TOASTS.write().warning(format!($($format),+));
    };
    (Error, $($format:expr),+) => {
        $crate::macros::TOASTS.write().error(format!($($format),+));
    };
    (Success, $($format:expr),+) => {
        $crate::macros::TOASTS.write().success(format!($($format),+));
    };
}
