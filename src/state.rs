use bevy::{color::palettes, prelude::*};

use crate::{
    board::{Block, NextPieceBoard, BLOCK_LENGTH, BLOCK_STICKER_LENGTH},
    common::FontTff,
    piece::{piece_shape, NextPieceType, PieceQueue, PieceType},
};

// 计分板长宽
const STATS_BOARD_LENGTH: f32 = 300.0;
const STATS_BOARD_WIDTH: f32 = 50.0;


// 方块初始 出现的偏移量
pub const base_offset:(i32,i32) = (3, -2);
// 分数
#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct Scoreboard;

#[derive(Component)]
pub struct Linesboard;

// 设置 游戏相关信息
pub fn setup_post_states_boards(
    mut commands: Commands,
    font_tff: Res<FontTff>,
    windows: Query<&Window>,
) {
    // 通过窗口大小和棋盘大小计算stats位置
    let window = windows.single();

    // gameboard左上角在窗口上的位置
    let gameboard_left_corner_pos = (
        window.physical_width() as f32 / 2.0 - 5.0 * BLOCK_LENGTH,
        window.physical_height() as f32 / 2.0 - 10.0 * BLOCK_LENGTH,
    );


    // 标题
    commands.spawn(
        TextBundle::from_sections([TextSection::new(
            "俄罗斯方块",
            TextStyle {
                font_size: 40.0,
                color: Color::srgb(0.5, 0.5, 1.0),
                font: font_tff.sim_hei.clone(),
            },
        )])
        .with_style(Style {
            height: Val::Percent(10.),
            width: Val::Percent(100.),
            margin: UiRect::top(Val::Px(10.)),
            ..default()
        })
        .with_text_justify(JustifyText::Center),
    );

    // 分数
    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new(
                    "分数: ",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::srgb(0.5, 0.5, 1.0),
                        font: font_tff.sim_hei.clone(),
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::srgb(1.0, 0.5, 0.5),
                        ..default()
                    },
                ),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(gameboard_left_corner_pos.1),
                left: Val::Px(gameboard_left_corner_pos.0 - STATS_BOARD_LENGTH),
                ..default()
            }),
        )
        .insert(Scoreboard);

    // // 行数
    // commands
    //     .spawn(
    //         TextBundle::from_sections([
    //             TextSection::new(
    //                 "行数: ",
    //                 TextStyle {
    //                     font_size: 40.0,
    //                     color: Color::srgb(0.5, 0.5, 1.0),
    //                     font: font_tff.sim_hei.clone(),
    //                 },
    //             ),
    //             TextSection::new(
    //                 "0",
    //                 TextStyle {
    //                     font_size: 40.0,
    //                     color: Color::srgb(1.0, 0.5, 0.5),
    //                     ..default()
    //                 },
    //             ),
    //         ])
    //         .with_style(Style {
    //             position_type: PositionType::Absolute,
    //             top: Val::Px(gameboard_left_corner_pos.1 + STATS_BOARD_WIDTH),
    //             left: Val::Px(gameboard_left_corner_pos.0 - STATS_BOARD_LENGTH),
    //             ..default()
    //         }),
    //     )
    //     .insert(Linesboard);
}

// 提前显示下一个即将出现的方块
pub fn spawn_next_piece_board(
    commands: &mut Commands,
    next_piece_board: Entity,
    blocks: [Block; 4],
    color: Color,
) {
    for block in blocks.iter() {
        let left = Val::Px(block.x as f32 * BLOCK_LENGTH);
        let top = Val::Px(block.y as f32 * BLOCK_LENGTH);
        let new_block_sprite = commands.spawn(new_block_sprite(color, left, top)).id();
        commands.entity(next_piece_board).add_child(new_block_sprite);
    }
}

pub fn new_block_sprite(color: Color, left: Val, top: Val) -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            top,
            left,
            width: Val::Px(BLOCK_LENGTH),
            height: Val::Px(BLOCK_LENGTH),
            border: UiRect::all(Val::Px(1.0)),
            ..Default::default()
        },
        background_color: BackgroundColor(color),
        border_color: BorderColor(Color::BLACK),
        ..Default::default()
    }
}


pub fn update_scoreboard(score: Res<Score>, mut query: Query<&mut Text, With<Scoreboard>>) {
    let mut text = query.single_mut();
    text.sections[1].value = score.0.to_string();
}