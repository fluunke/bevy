use std::path::PathBuf;

use super::{WindowDescriptor, WindowId};
use bevy_math::{IVec2, Vec2};

/// A window event that is sent whenever a window's logical size has changed.
#[derive(Debug, Clone)]
pub struct WindowResized {
    pub id: WindowId,
    /// The new logical width of the window.
    pub width: f32,
    /// The new logical height of the window.
    pub height: f32,
}

/// An event that indicates that a new window should be created.
#[derive(Debug, Clone)]
pub struct CreateWindow {
    pub id: WindowId,
    pub descriptor: WindowDescriptor,
}

/// An event that indicates the window should redraw, even if its control flow is set to `Wait` and
/// there have been no window events.
#[derive(Debug, Clone)]
pub struct RequestRedraw;

/// An event that is sent whenever a new window is created.
///
/// To create a new window, send a [`CreateWindow`] event - this
/// event will be sent in the handler for that event.
#[derive(Debug, Clone)]
pub struct WindowCreated {
    pub id: WindowId,
}

/// An event that is sent whenever the operating systems requests that a window
/// be closed. This will be sent when the close button of the window is pressed.
///
/// If the default [`WindowPlugin`] is used, these events are handled
/// by [closing] the corresponding [`Window`].  
/// To disable this behaviour, set `close_when_requested` on the [`WindowPlugin`]
/// to `false`.
///
/// [`WindowPlugin`]: crate::WindowPlugin
/// [`Window`]: crate::Window
/// [closing]: crate::Window::close
#[derive(Debug, Clone)]
pub struct WindowCloseRequested {
    pub id: WindowId,
}

/// An event that is sent whenever a window is closed. This will be sent by the
/// handler for [`Window::close`].
///
/// [`Window::close`]: crate::Window::close
#[derive(Debug, Clone)]
pub struct WindowClosed {
    pub id: WindowId,
}
/// An event that is sent whenenver the user's cursor moves.
#[derive(Debug, Clone)]
pub struct CursorMoved {
    pub id: WindowId,
    pub position: Vec2,
}
/// An event that is sent whenever the user's cursor enters a window.
#[derive(Debug, Clone)]
pub struct CursorEntered {
    pub id: WindowId,
}
/// An event that is sent whenever the user's cursor leaves a window.
#[derive(Debug, Clone)]
pub struct CursorLeft {
    pub id: WindowId,
}

/// An event that is sent whenever a window receives a character from the OS or underlying system.
#[derive(Debug, Clone)]
pub struct ReceivedCharacter {
    pub id: WindowId,
    pub char: char,
}

/// An event that indicates a window has received or lost focus.
#[derive(Debug, Clone)]
pub struct WindowFocused {
    pub id: WindowId,
    pub focused: bool,
}

/// An event that indicates a window's scale factor has changed.
#[derive(Debug, Clone)]
pub struct WindowScaleFactorChanged {
    pub id: WindowId,
    pub scale_factor: f64,
}
/// An event that indicates a window's OS-reported scale factor has changed.
#[derive(Debug, Clone)]
pub struct WindowBackendScaleFactorChanged {
    pub id: WindowId,
    pub scale_factor: f64,
}

/// Events related to files being dragged and dropped on a window.
#[derive(Debug, Clone)]
pub enum FileDragAndDrop {
    DroppedFile { id: WindowId, path_buf: PathBuf },

    HoveredFile { id: WindowId, path_buf: PathBuf },

    HoveredFileCancelled { id: WindowId },
}

/// An event that is sent when a window is repositioned in physical pixels.
#[derive(Debug, Clone)]
pub struct WindowMoved {
    pub id: WindowId,
    pub position: IVec2,
}
