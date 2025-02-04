use bevy::prelude::*;

pub fn update_cursor_state(
    mut cursor_moved_events: EventReader<CursorMoved>,
    windows: Query<&Window>,
    mut state: ResMut<WindowInteractionState>,
) {
    let window = windows.single();

    for event in cursor_moved_events.read() {
        state.cursor_inside = event.position.x >= 0.0
            && event.position.x <= window.width()
            && event.position.y >= 0.0
            && event.position.y <= window.height();
    }
}

pub fn update_focus_state(
    mut focus_events: EventReader<WindowFocused>,
    mut state: ResMut<WindowInteractionState>,
) {
    for event in focus_events.read() {
        state.window_focused = event.focused;
    }
}