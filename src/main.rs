// "Hello 😊︎ 😐︎ ☹︎ example"
#![windows_subsystem = "windows"]

mod commands;
mod seticon;
mod theme;
mod widgets;

use druid::{piet::Color, AppDelegate, AppLauncher, Command, DelegateCtx, Env, LocalizedString, Target, WindowDesc};
use druid::{Data, Menu, Size, WindowHandle, WindowId};

use seticon::set_icon;

use theme::Theme;
use widgets::window::NPWindowState;

#[derive(Debug)]
pub struct Delegate;
impl AppDelegate<NPWindowState> for Delegate {
    fn event(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _window_id: druid::WindowId,
        event: druid::Event,
        _data: &mut NPWindowState,
        _env: &Env,
    ) -> Option<druid::Event> {
        Some(event)
    }
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        _cmd: &Command,
        _data: &mut NPWindowState,
        _env: &Env,
    ) -> druid::Handled {
        druid::Handled::No
    }
    fn window_added(
        &mut self,
        id: druid::WindowId,
        _handle: WindowHandle,
        _data: &mut NPWindowState,
        _env: &Env,
        _ctx: &mut druid::DelegateCtx,
    ) {
        set_icon(id);
    }
    fn window_removed(
        &mut self,
        _id: druid::WindowId,
        _data: &mut NPWindowState,
        _env: &Env,
        _ctx: &mut druid::DelegateCtx,
    ) {
    }
}

#[allow(unused_assignments, unused_mut)]
fn make_menu<T: Data>(_window: Option<WindowId>, _data: &NPWindowState, _env: &Env) -> Menu<T> {
    let mut base = Menu::empty();
    #[cfg(target_os = "macos")]
    {
        base = base.entry(druid::platform_menus::mac::application::default())
    }
    base
}

fn main() -> anyhow::Result<()> {
    #[cfg(target_os = "windows")]
    {
        use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};
        unsafe {
            AttachConsole(ATTACH_PARENT_PROCESS);
        }
    }

    let app_state = if let Some(filename) = std::env::args().nth(1) {
        NPWindowState::from_file(filename)?
    } else {
        NPWindowState::new()
    };

    let win = WindowDesc::new(widgets::window::NPWindow::build())
        .title(LocalizedString::new("NonePad"))
        .with_min_size(Size::new(500., 500.))
        .menu(make_menu);
    AppLauncher::with_window(win)
        .delegate(Delegate)
        .configure_env(|env, _| {
            let theme = Theme::default();
            env.set(
                druid::theme::WINDOW_BACKGROUND_COLOR,
                Color::from_hex_str(&theme.vscode.colors.editor_background).unwrap(),
            );
            env.set(
                druid::theme::BORDER_DARK,
                Color::from_hex_str(&theme.vscode.colors.panel_border).unwrap(),
            );

            theme.to_env(env);
        })
        .launch(app_state)?;
    Ok(())
}
