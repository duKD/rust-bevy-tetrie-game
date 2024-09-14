use std::time::Duration;

use bevy::{prelude::*, window::WindowResolution};
use board::{check_game_overs, setup_game_board, show_game_over_menu, update_next_piece_board, HasNextPiece, MainBoard, BLOCK_LENGTH};
use common::{setup_font_assets_resource, setup_game_audios_resource, AppState, GameAudios, WindowSize};
use menu::click_button;
use piece::{
    auto_generate_new_piece, check_collision, check_full_line, move_piece, remove_piece_component, rotate_piece, setup_piece_queue, AutoMovePieceDownTimer, ManuallyMoveTimer, RemovePieceComponentTimer
};
use state::{setup_post_states_boards, update_scoreboard, Score};

mod board;
mod common;
mod piece;
mod state;
mod menu;
pub fn start() {
    App::new()
        .insert_resource(Score(0))
        .insert_resource(HasNextPiece(false))
        .insert_resource(AutoMovePieceDownTimer(Timer::new(
            Duration::from_millis(500),
            TimerMode::Repeating,
        )))
        .insert_resource(ManuallyMoveTimer(Timer::new(
            Duration::from_millis(100),
            TimerMode::Once,
        )))
        .insert_resource(RemovePieceComponentTimer(Timer::new(
            Duration::from_millis(300),
            TimerMode::Once,
        )))
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_font_assets_resource,
                setup_game_audios_resource,
                setup_game_board,
                setup_piece_queue,
            ),
        )
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetris".to_string(),
                resolution: WindowResolution::new(
                    WindowSize::default().width,
                    WindowSize::default().height,
                ),
                ..default()
            }),
            ..Default::default()
        }))
        .init_state::<AppState>()
        // 这个阶段在 Startup 阶段之后运行，也用于执行初始化任务 可以使用部分预加载的资源
        .add_systems(PostStartup, (setup_post_states_boards,))
        .add_systems(Update, (auto_generate_new_piece, move_piece,rotate_piece).run_if(in_state(AppState::InGame)))
        .add_systems(
            PostUpdate,
            (
                update_next_piece_board,
                check_collision,
                remove_piece_component,
                update_scoreboard,
                check_full_line,
                check_game_overs
            ).run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            OnEnter(AppState::GameOver),
            show_game_over_menu,
        )
        .add_systems(
            Update,
            click_button.run_if(
                in_state(AppState::GameOver)
            ),
        )
        .run();
}

/**
 * 在 Bevy 中，当你使用 Camera2dBundle::default() 创建一个 2D 相机时
 * ，这个相机默认会设置为覆盖整个屏幕。Camera2dBundle 是为 2D 渲染设计的，
 * 它会创建一个正交相机，其视野（viewport）默认设置为匹配窗口的尺寸。
 *
 * 由于 OrthographicProjection 的视野默认设置为窗口的宽度和高度，
 * 所以相机会覆盖整个屏幕。在这种情况下，如果你在世界坐标的原点（0, 0, 0）处放置一个精灵，它应该会出现在屏幕的中心。
 */
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
