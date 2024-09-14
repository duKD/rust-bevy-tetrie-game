use bevy::{color::palettes, prelude::*};
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use crate::{
    board::{Block, HasNextPiece, MainBoard, BLOCK_LENGTH, COL_COUNT, ROW_COUNT},
    common::GameAudios,
    state::{new_block_sprite, Score},
};
use rand::Rng;

// 自动向下移动四格骨牌计时器
#[derive(Debug, Resource)]
pub struct AutoMovePieceDownTimer(pub Timer);

// 控制手动移动频率
#[derive(Debug, Resource)]
pub struct ManuallyMoveTimer(pub Timer);

#[derive(Debug, Resource)]
pub struct RemovePieceComponentTimer(pub Timer);

// 待生成的骨牌队列
#[derive(Debug, Resource)]
pub struct PieceQueue(pub VecDeque<PieceConfig>);

pub fn setup_piece_queue(mut commands: Commands) {
    let mut piece_queue = PieceQueue(VecDeque::new());
    piece_queue.0.extend(random_7_pieces());
    commands.insert_resource(piece_queue);
}

// 展示下一个骨牌
#[derive(Debug, Resource)]
pub struct NextPieceType(pub Option<PieceType>);

#[derive(Debug, Clone)]
pub struct PieceConfig {
    pub piece_type: PieceType,
    pub blocks: [Block; 4],
    pub color: Color,
}

impl PieceConfig {
    pub fn new(piece_type: PieceType, blocks: [Block; 4]) -> Self {
        let color = Color::Srgba(match piece_type {
            PieceType::I => palettes::css::LIGHT_CYAN,
            PieceType::J => palettes::css::BLUE,
            PieceType::L => palettes::css::ORANGE,
            PieceType::O => palettes::css::YELLOW,
            PieceType::S => palettes::css::GREEN,
            PieceType::T => palettes::css::PURPLE,
            PieceType::Z => palettes::css::RED,
        });
        PieceConfig {
            piece_type,
            blocks,
            color,
        }
    }
}

// 四格骨牌
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum PieceType {
    // ####
    I,

    // #
    // ###
    J,

    //   #
    // ###
    L,

    // ##
    // ##
    O,

    //  ##
    // ##
    S,

    //  #
    // ###
    T,

    // ##
    //  ##
    Z,
}

impl PieceType {
    pub const PIECE_AMOUNT: u32 = 7;
}

const SHAPE_I: [[i32; 2]; 4] = [[0, 0], [1, 0], [2, 0], [3, 0]];
const SHAPE_J: [[i32; 2]; 4] = [[0, 1], [0, 0], [1, 0], [2, 0]];
const SHAPE_L: [[i32; 2]; 4] = [[0, 0], [1, 0], [2, 0], [2, 1]];
const SHAPE_O: [[i32; 2]; 4] = [[1, 1], [1, 0], [2, 1], [2, 0]];
const SHAPE_S: [[i32; 2]; 4] = [[1, 0], [2, 0], [1, 1], [2, 1]];
const SHAPE_T: [[i32; 2]; 4] = [[0, 0], [1, 1], [1, 0], [2, 0]];
const SHAPE_Z: [[i32; 2]; 4] = [[0, 1], [1, 1], [1, 0], [2, 0]];

// 可移动方向
#[derive(Component)]
pub struct Movable {
    pub can_down: bool,
    pub can_left: bool,
    pub can_right: bool,
}

pub fn piece_shape(piece_type: PieceType) -> [Block; 4] {
    match piece_type {
        PieceType::I => SHAPE_I.map(|pos| pos.into()),
        PieceType::J => SHAPE_J.map(|pos| pos.into()),
        PieceType::L => SHAPE_L.map(|pos| pos.into()),
        PieceType::O => SHAPE_O.map(|pos| pos.into()),
        PieceType::S => SHAPE_S.map(|pos| pos.into()),
        PieceType::T => SHAPE_T.map(|pos| pos.into()),
        PieceType::Z => SHAPE_Z.map(|pos| pos.into()),
    }
}
// bag7算法实现随机：每次填充7个随机排序的骨牌
pub fn random_7_pieces() -> Vec<PieceConfig> {
    let mut rng = rand::thread_rng();
    let mut piece_type_set = BTreeSet::new();

    loop {
        match rng.gen_range(0..PieceType::PIECE_AMOUNT) {
            0 => {
                piece_type_set.insert(PieceType::I);
            }
            1 => {
                piece_type_set.insert(PieceType::J);
            }
            2 => {
                piece_type_set.insert(PieceType::L);
            }
            3 => {
                piece_type_set.insert(PieceType::O);
            }
            4 => {
                piece_type_set.insert(PieceType::S);
            }
            5 => {
                piece_type_set.insert(PieceType::T);
            }
            6 => {
                piece_type_set.insert(PieceType::Z);
            }
            _ => {
                panic!("Random value is unexpected");
            }
        }
        if piece_type_set.len() == PieceType::PIECE_AMOUNT as usize {
            break;
        }
    }
    piece_type_set
        .iter()
        .map(|piece_type| PieceConfig::new(*piece_type, piece_shape(*piece_type)))
        .collect()
}

// 自动生成新的四格骨牌
pub fn auto_generate_new_piece(
    mut commands: Commands,
    query: Query<&PieceType>,
    main_board: Query<Entity, With<MainBoard>>,
    mut piece_queue: ResMut<PieceQueue>,
    mut has_next_piece: ResMut<HasNextPiece>,
) {
    // 获取到
    let main_board_entity = main_board.single();
    if piece_queue.0.len() < PieceType::PIECE_AMOUNT as usize {
        piece_queue.0.extend(random_7_pieces());
    }
    // 如果没有四格骨牌，则生成新的
    if query.is_empty() {
        // 设置 状态值 表示 可以更新下一个待出现 的卡片
        has_next_piece.0 = false;
        let piece_config = piece_queue.0.pop_front().unwrap();
        // 生成新的四格骨牌
        let color = piece_config.color;
        let piece_type: PieceType = piece_config.piece_type.clone();

        for block in piece_config.blocks.iter() {
            let cur = block.transform_to();
            let left = Val::Px(cur.x);
            let top = Val::Px(cur.y);
            commands.entity(main_board_entity).with_children(|parent| {
                parent
                    .spawn(piece_type)
                    .insert(new_block_sprite(color, left, top))
                    .insert(*block)
                    .insert(Movable {
                        can_down: true,
                        can_left: true,
                        can_right: true,
                    });
            });
        }
    }
}

// 自动和手动移动四格骨牌
pub fn move_piece(
    mut commands: Commands,
    game_audios: Res<GameAudios>,
    mut query: Query<(&mut Block, &mut Style, &Movable), With<PieceType>>,
    mut auto_move_timer: ResMut<AutoMovePieceDownTimer>,
    mut manually_move_timer: ResMut<ManuallyMoveTimer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    auto_move_timer.0.tick(time.delta());
    manually_move_timer.0.tick(time.delta());
    let mut reset_manually_move_timer = false;
    let can_manually_move = manually_move_timer.0.finished();
    // 同一桢 手动向下移动后 不能在自动移动
    let can_auto_move = auto_move_timer.0.finished()
        && !(can_manually_move && keyboard_input.pressed(KeyCode::ArrowDown));
    for (mut block, mut style, movable) in &mut query {
        // 手动移动
        if can_manually_move {
            if keyboard_input.pressed(KeyCode::ArrowLeft) && movable.can_left {
                block.x -= 1;
                reset_manually_move_timer = true;
                spawn_drop_audio(&mut commands, &game_audios);
            } else if keyboard_input.pressed(KeyCode::ArrowRight) && movable.can_right {
                block.x += 1;
                reset_manually_move_timer = true;
                spawn_drop_audio(&mut commands, &game_audios);
            } else if keyboard_input.pressed(KeyCode::ArrowDown) && movable.can_down {
                reset_manually_move_timer = true;
                block.y += 1;
                spawn_drop_audio(&mut commands, &game_audios);
            }

            let cur = block.transform_to();
            style.top = Val::Px(cur.y);
            style.left = Val::Px(cur.x);
        }
        // 自动下移
        if can_auto_move && movable.can_down {
            block.y += 1;
            spawn_drop_audio(&mut commands, &game_audios);
            let cur = block.transform_to();
            style.top = Val::Px(cur.y);
            style.left = Val::Px(cur.x);
        }
    }

    if reset_manually_move_timer {
        manually_move_timer.0.reset();
    }
}

// 向下移动的音效
fn spawn_drop_audio(commands: &mut Commands, game_audios: &Res<GameAudios>) {
    commands.spawn(AudioBundle {
        source: game_audios.drop.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
}

// 检查碰撞
pub fn check_collision(
    mut piece_query: Query<(&mut Block, &mut Movable), With<PieceType>>,
    board_query: Query<&Block, Without<PieceType>>,
) {
    let mut can_down = true;
    let mut can_left = true;
    let mut can_right = true;

    // 遍历正在移动的方块 检查是否碰撞边界
    for (block, _) in &mut piece_query {
        let cur_pos = block.transform_to_real_pos();
        if cur_pos.0 == 0 {
            // 碰撞左边界
            can_left = false;
        }
        if cur_pos.0 == COL_COUNT as i32 - 1 {
            // 碰撞右边界
            can_right = false;
        }
        if cur_pos.1 == (ROW_COUNT as i32) - 1 {
            // 碰撞下边界
            can_down = false;
        }
    }

    // 遍历正在移动的方块 检查是否碰撞面板方块
    for (block, _) in &piece_query {
        // 遍历 面板已经存在的方块
        for board_block in &board_query {
            let board_y = board_block.y;
            let board_x = board_block.x;
            if board_y == block.y {
                if board_x == block.x - 1 {
                    // 左侧碰撞
                    can_left = false;
                } else if board_x == block.x + 1 {
                    // 右侧碰撞
                    can_right = false;
                }
            }
            if board_x == block.x {
                if block.y > 0 && board_y == block.y + 1 {
                    // 下侧碰撞
                    can_down = false;
                }
                if board_y == 0 {
                    // 底部碰撞
                    can_down = false;
                }
            }
        }
    }

    // 更新Movable
    for (_, mut movable) in &mut piece_query {
        movable.can_left = can_left;
        movable.can_right = can_right;
        movable.can_down = can_down;
    }
}

// 当piece移到底部后，移除piece组件
pub fn remove_piece_component(
    mut commands: Commands,
    q_piece_blocks: Query<(Entity, &Movable), With<PieceType>>,
    mut timer: ResMut<RemovePieceComponentTimer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if !q_piece_blocks.is_empty() && !q_piece_blocks.iter().last().unwrap().1.can_down {
        if !q_piece_blocks.iter().last().unwrap().1.can_down {
            timer.0.tick(time.delta());
        } else {
            // 无法下移时，通过左右移动获得重新下移能力
            timer.0.reset();
        }
    }
    let mut reset_timer = false;
    for (entity, movable) in &q_piece_blocks {
        // 到达底部后，仍可短时间内左右移动
        if !movable.can_down {
            // 当到达底部后，按向下键时，跳过timer直接开始新一个piece
            if timer.0.just_finished() || keyboard_input.pressed(KeyCode::ArrowDown) {
                commands.entity(entity).remove::<PieceType>();
                reset_timer = true;
            }
        }
    }
    if reset_timer {
        timer.0.reset();
    }
}

// 检查是否有成功的行
pub fn check_full_line(
    mut commands: Commands,
    game_audios: Res<GameAudios>,
    mut score: ResMut<Score>,
    mut query: Query<(Entity, &mut Block, &mut Style), Without<PieceType>>,
) {
    let mut y_to_x_set_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (_, block, _) in &query {
        if y_to_x_set_map.contains_key(&block.y) {
            let x_set = y_to_x_set_map.get_mut(&block.y).unwrap();
            x_set.insert(block.x);
        } else {
            let mut x_set = HashSet::new();
            x_set.insert(block.x);
            y_to_x_set_map.insert(block.y, x_set);
        }
    }
    let mut full_lines = Vec::new();
    for (y, x_set) in y_to_x_set_map.iter() {
        if x_set.len() == COL_COUNT as usize {
            full_lines.push(y);
        }
    }
    if full_lines.len() > 0 {
        dbg!(full_lines.len());
        commands.spawn(AudioBundle {
            source: game_audios.line_clear.clone(),
            ..default()
        });
    }
    // 分数增加
    score.0 += match full_lines.len() {
        0 => 0,
        1 => 100,
        2 => 200,
        3 => 400,
        4 => 800,
        _ => 1000,
    };

    // 消除行
    let mut despawn_entities = Vec::new();
    for line_no in full_lines.iter() {
        let line_no = **line_no;
        for (entity, block, _) in &mut query {
            if block.y == line_no {
                despawn_entities.push(entity);
                commands.entity(entity).despawn();
            }
        }
    }
    // 消除行的上面block整体向下移
    full_lines.sort();
    for line_no in full_lines.iter() {
        println!("line_no: {}", line_no);
        for (entity, mut block, mut style) in &mut query {
            if !despawn_entities.contains(&entity) && block.y < **line_no {
                info!("down block: {:?}, line_no: {}", block, line_no);
                block.y += 1;
                let cur = block.transform_to();
                style.top = Val::Px(cur.y);
                style.left = Val::Px(cur.x);
            }
        }
    }
}

pub fn rotate_piece(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_piece: Query<(&mut PieceType, &mut Block, &mut Style)>,
    q_board: Query<&Block, Without<PieceType>>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        let piece_type = match q_piece.iter().next() {
            Some((piece_type, _, _)) => piece_type.clone(),
            None => {
                return;
            }
        };
        let sum_x = q_piece.iter().map(|(_, block, _)| block.x).sum::<i32>();
        let sum_y = q_piece.iter().map(|(_, block, _)| block.y).sum::<i32>();

        let original_blocks: Vec<Block> =
            q_piece.iter().map(|(_, block, _)| block.clone()).collect();
        // 通过矩阵变化实现旋转，可以理解为沿y=x对称后沿y=0对称，然后平移
        for (_, mut block, mut style) in &mut q_piece {
            *block = match piece_type {
                // 微调平移量，使其更自然
                PieceType::O | PieceType::L | PieceType::J => shift_block(
                    [block.y, -block.x].into(),
                    Some(sum_x / 4 - sum_y / 4),
                    Some(sum_x / 4 + sum_y / 4 + 1),
                ),
                _ => shift_block(
                    [block.y, -block.x].into(),
                    Some(sum_x / 4 - sum_y / 4),
                    Some(sum_x / 4 + sum_y / 4),
                ),
            };
            let cur = block.transform_to();
            style.top = Val::Px(cur.y);
            style.left = Val::Px(cur.x);
        }


        // 当出现碰撞时，尝试左右平移最多2格（也可采取旋转后一旦出现碰撞则恢复原样）
        if whether_colliding(&q_piece, &q_board) {
            for (_, mut block, mut style) in &mut q_piece {
                *block = shift_block(block.clone(), Some(-1), None);
                let cur = block.transform_to();
                style.top = Val::Px(cur.y);
                style.left = Val::Px(cur.x);
            }
        }
        if whether_colliding(&q_piece, &q_board) {
            for (_, mut block, mut style) in &mut q_piece {
                *block = shift_block(block.clone(), Some(-1), None);
                let cur = block.transform_to();
                style.top = Val::Px(cur.y);
                style.left = Val::Px(cur.x);
            }
        }
        if whether_colliding(&q_piece, &q_board) {
            for (_, mut block, mut style) in &mut q_piece {
                *block = shift_block(block.clone(), Some(3), None);
                let cur = block.transform_to();
                style.top = Val::Px(cur.y);
                style.left = Val::Px(cur.x);
            }
        }
        if whether_colliding(&q_piece, &q_board) {
            for (_, mut block, mut style) in &mut q_piece {
                *block = shift_block(block.clone(), Some(3), None);
                let cur = block.transform_to();
                style.top = Val::Px(cur.y);
                style.left = Val::Px(cur.x);
            }
        }
        // 恢复旋转前样子
        if whether_colliding(&q_piece, &q_board) {
            let mut index = 0;
            for (_, mut block, mut style) in &mut q_piece {
                *block = original_blocks[index];
                let cur = block.transform_to();
                style.top = Val::Px(cur.y);
                style.left = Val::Px(cur.x);
                index += 1;
            }
        }
    }
}

fn shift_block(mut block: Block, delta_x: Option<i32>, delta_y: Option<i32>) -> Block {
    match delta_x {
        Some(delta) => {
            block.x += delta;
        }
        None => {}
    }
    match delta_y {
        Some(delta) => {
            block.y += delta;
        }
        None => {}
    }
    block
}

// 检测旋转过程中是否发送碰撞
pub fn whether_colliding(
    piece_query: &Query<(&mut PieceType, &mut Block, &mut Style)>,
    board_query: &Query<&Block, Without<PieceType>>,
) -> bool {
    // 检查是否碰撞边界
    for (_, block, _) in piece_query {
        let cur_pos = block.transform_to_real_pos();
        if cur_pos.0 == 0 {
            // 碰撞左边界
            return true;
        }
        if cur_pos.0 == COL_COUNT as i32 - 1 {
            // 碰撞右边界
            return true;
        }
        if cur_pos.1 == (ROW_COUNT as i32) - 1 {
            // 碰撞下边界
            return true;
        }
    }

    // 检查是否碰撞面板方块
    for (_, block, _) in piece_query {
        for board_block in board_query {
            let board_y = board_block.y;
            let board_x = board_block.x;
            if board_y == block.y {
                if board_x == block.x - 1 {
                    // 左侧碰撞
                    return true;
                } else if board_x == block.x + 1 {
                    // 右侧碰撞
                    return true;
                }
            }
            if board_x == block.x {
                if block.y > 0 && board_y == block.y + 1 {
                    // 下侧碰撞
                    return true;
                }
                if board_y == 0 {
                    // 底部碰撞
                    return true;
                }
            }
        }
    }

    return false;
}



