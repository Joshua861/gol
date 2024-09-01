use crate::prelude::*;
use lazy_static::lazy_static;
use std::{fmt::Display, sync::RwLock};

lazy_static! {
    static ref STATE: RwLock<NotificationState> = RwLock::new(NotificationState::new());
}

fn set_state(new_state: NotificationState) {
    let mut state = STATE.write().unwrap();
    *state = new_state;
}

fn get_state() -> NotificationState {
    STATE.read().unwrap().clone()
}

pub fn notify_error(text: impl Display) {
    send_notification(text, NotificationKind::Error);
}

pub fn notify_info(text: impl Display) {
    send_notification(text, NotificationKind::Info);
}

pub fn notify(text: impl Display) {
    send_notification(text, NotificationKind::Default);
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NotificationKind {
    Error,
    Info,
    Default,
}

#[derive(Clone, Debug)]
struct NotificationState {
    buffer: Vec<Notification>,
}

impl NotificationState {
    fn new() -> Self {
        Self { buffer: Vec::new() }
    }
}

#[derive(Clone, Debug)]
struct Notification {
    text: String,
    kind: NotificationKind,
    timer: f32,
}

pub fn send_notification(text: impl std::fmt::Display, ty: NotificationKind) {
    let mut state = get_state();
    state.buffer.insert(
        0,
        Notification {
            text: text.to_string(),
            kind: ty,
            timer: 0.0,
        },
    );
    set_state(state);
}

pub fn draw_notifications(app: &nannou::App, draw: &nannou::Draw, model: &Model) {
    let mut state = get_state();
    let mut to_remove = Vec::new();

    for (i, notification) in state.buffer.iter_mut().enumerate() {
        let prefix = if notification.kind == NotificationKind::Error {
            "[ERROR]: "
        } else if notification.kind == NotificationKind::Info {
            "[INFO]: "
        } else {
            ""
        };
        draw.text(&format!("{}{}", prefix, notification.text))
            .x(0.)
            .y((-app.window_rect().h() / 2.0) + CONFIG.font_size as f32 * 1.25 * (i + 1) as f32)
            .w(app.window_rect().w() - 30.)
            .h(CONFIG.font_size as f32)
            .right_justify()
            .align_text_bottom()
            .font_size(CONFIG.font_size)
            .font(model.font.clone())
            .color(match notification.kind {
                NotificationKind::Error => CONFIG.error_color.to_srgb(),
                NotificationKind::Info => CONFIG.info_color.to_srgb(),
                NotificationKind::Default => CONFIG.text_color.to_srgb(),
            });

        notification.timer += model.delta_time();

        if notification.timer >= 10.0 {
            to_remove.push(i);
        }
    }

    for i in to_remove.iter().rev() {
        state.buffer.remove(*i);
    }

    set_state(state.clone());
}
