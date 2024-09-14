use bevy::prelude::*;

use crate::{
    common::AppState, piece::{piece_shape, PieceQueue, PieceType}, state::{base_offset, spawn_next_piece_board}
};

// game board宽高
pub const COL_COUNT: u8 = 10;
pub const ROW_COUNT: u8 = 20;

// 正方形方块边长
pub const BLOCK_LENGTH: f32 = 30.0;
// TODO 贴纸圆角
// 正方形方块贴纸边长
pub const BLOCK_STICKER_LENGTH: f32 = 28.0;

// game board 边界厚度
pub const BORDER_THICKNESS: f32 = 10.0;
pub const BORDER_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);

// 方块
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}

impl Block {
    pub fn transform_to(&self) -> Vec2 {
        Vec2 {
            x: (self.x + base_offset.0) as f32 * BLOCK_LENGTH,
            y: (self.y + base_offset.1) as f32 * BLOCK_LENGTH,
        }
    }

    pub fn transform_to_real_pos(&self) -> (i32, i32) {
        (self.x + base_offset.0, self.y + base_offset.1)
    }
}

impl From<[i32; 2]> for Block {
    fn from([x, y]: [i32; 2]) -> Self {
        Block { x, y }
    }
}

// 装方块的容器
#[derive(Component)]
pub struct MainBoard;

#[derive(Debug, Component)]
pub struct NextPieceBoard;

// 展示下一个骨牌
#[derive(Resource)]
pub struct HasNextPiece(pub bool);

pub fn setup_game_board(mut commands: Commands) {
    let border_size: f32 = 1.0;
    let main_board = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(BLOCK_LENGTH * COL_COUNT as f32 + border_size),
                height: Val::Px(BLOCK_LENGTH * ROW_COUNT as f32 + border_size),
                position_type: PositionType::Absolute,
                border: UiRect::all(Val::Px(border_size)),
                overflow:Overflow::clip(),
                ..Default::default()
            },
            border_color: BorderColor(Color::srgb(0.5, 0.5, 1.0)),
            ..default()
        })
        .insert(MainBoard)
        .id();

    let next_piece_board = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(BLOCK_LENGTH * 4 as f32),
                height: Val::Px(BLOCK_LENGTH * 4 as f32),
                position_type: PositionType::Absolute,
                top: Val::Px(80 as f32),
                right: Val::Px(150 as f32),
                ..Default::default()
            },
            ..default()
        })
        .insert(NextPieceBoard)
        .id();

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center, // 在主轴方向上居中
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..default()
        })
        .add_child(main_board)
        .add_child(next_piece_board);
}

pub fn update_next_piece_board(
    mut commands: Commands,
    piece_queue: Res<PieceQueue>,
    nest_piece_board_query: Query<Entity, With<NextPieceBoard>>,
    //  Bevy 中，实体和子实体之间的关系是通过 Children 组件来维护的。如果你想遍历某个实体的所有子实体，你可以使用 Query 来获取 Children 组件
    children_query: Query<&Children>,
    mut has_next_piece: ResMut<HasNextPiece>,
) {
    if !has_next_piece.0 {
        let next_piece = piece_queue.0.front().unwrap();
        let piece_type = next_piece.piece_type;
        let color = next_piece.color;
        let blocks = piece_shape(piece_type);
        let next_piece_board = nest_piece_board_query.single();
        if let Ok(children) = children_query.get(next_piece_board) {
            for child in children.iter() {
                //despawn_recursive 方法用于递归地销毁一个实体及其所有子实体
                commands.entity(*child).despawn_recursive();
            }
        }
        spawn_next_piece_board(&mut commands, next_piece_board, blocks, color);
        has_next_piece.0 = true;
    }
}



// 检测容器是否溢出 游戏结束
pub fn check_game_overs(
    mut app_state: ResMut<NextState<AppState>>,
    board_query: Query<&Block, Without<PieceType>>,
) {
    // 检查是否碰撞边界
    let mut max_block_y = ROW_COUNT as i32;
    for board_block in board_query.iter() {
        let cur = board_block.transform_to_real_pos();
        if cur.1<max_block_y {
            max_block_y = cur.1;
        }
    }


    if max_block_y < 0 {
        app_state.set(AppState::GameOver);
    }
}


#[derive(Component,Debug)]
pub struct QuitButton;


pub fn show_game_over_menu(
    mut commands: Commands,
) {

    commands
    .spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.),
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        },
    ))
    .with_children(|parent| {
        parent
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(400.),
                        height: Val::Px(400.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor::from(Color::srgb(0.1, 0.1, 0.1)),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(200.0),
                                    height: Val::Px(50.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                image: UiImage::default()
                                    .with_color(Color::srgb(0.15, 0.15, 0.15).into()),
                                ..default()
                            },
                            QuitButton
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "GAME OVER",
                                TextStyle {
                                    font_size: 40.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            ));
                        });
               
            });
    });
}