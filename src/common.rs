use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
/// 定义应用程序状态
pub enum AppState {
    #[default]
    InGame,
    GameOver,
}

#[derive(Debug, Resource)]
pub struct GameAudios {
    pub drop: Handle<AudioSource>,
    pub game_over: Handle<AudioSource>,
    pub line_clear: Handle<AudioSource>,
}
#[derive(Debug, Resource)]
pub struct FontTff {
    pub sim_hei: Handle<Font>,
    pub fira_sans_bold: Handle<Font>,
}



// 屏幕尺寸
#[derive(Resource)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

impl Default for WindowSize {
    fn default() -> Self {
        Self {
            width: 1000.0,
            height: 750.0,
        }
    }
}

/// 加载游戏音频资源
///
/// 该函数负责加载游戏过程中使用的音频资源，并将它们插入到游戏资源池中。
///
/// # 参数
/// - `mut command: Commands`: 命令缓冲区，用于插入资源命令。
/// - `asset_server: Res<AssetServer>`: 资源服务器，用于加载音频资源。
pub fn setup_game_audios_resource(mut command: Commands, asset_server: Res<AssetServer>) {
    // 定义并加载游戏音频资源
    let game_audios: GameAudios = GameAudios {
        drop: asset_server.load("sounds/Drop.wav"),
        game_over: asset_server.load("sounds/Gameover.wav"),
        line_clear: asset_server.load("sounds/Lineclear.wav"),
    };

    // 将加载的音频资源插入到命令缓冲区中
    command.insert_resource(game_audios);
}

pub fn setup_font_assets_resource(mut command: Commands, asset_server: Res<AssetServer>) {
    let font_tff: FontTff = FontTff {
        sim_hei: asset_server.load("fonts/SimHei.ttf"),
        fira_sans_bold: asset_server.load("fonts/Fira-Sans-Bold.ttf"),
    };

    command.insert_resource(font_tff);
}

