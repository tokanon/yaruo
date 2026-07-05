use bevy::{
    asset::AssetPlugin,
    prelude::*,
    sprite::Anchor,
    text::{FontSmoothing, LineBreak},
    window::WindowResolution,
};

const ASSET_ROOT: &str = "../../assets";
const AA_FONT_PATH: &str = "fonts/Saitamaar.ttf";
const YARUO_NEUTRAL_AA_ID: &str = "yaruo.neutral";
const YARUO_NEUTRAL_AA: &str = include_str!("../../../assets/aa/yaruo/neutral.txt");

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.07, 0.065, 0.055)))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: ASSET_ROOT.into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "yaruo - AA prototype".into(),
                        resolution: WindowResolution::new(1280, 720)
                            .with_scale_factor_override(1.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, spawn_yaruo)
        .run();
}

fn spawn_yaruo(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Name::new("aa.stage"),
        Sprite::from_color(Color::srgb(0.105, 0.095, 0.075), Vec2::new(760.0, 380.0)),
        Transform::from_xyz(-170.0, 70.0, -1.0),
    ));

    commands.spawn((
        Name::new("aa.stage.left_rule"),
        Sprite::from_color(Color::srgb(0.65, 0.52, 0.32), Vec2::new(6.0, 380.0)),
        Transform::from_xyz(-550.0, 70.0, -0.5),
    ));

    let aa_font = TextFont {
        font: asset_server.load(AA_FONT_PATH).into(),
        font_size: FontSize::Px(28.0),
        ..default()
    }
    .with_font_smoothing(FontSmoothing::None);

    commands.spawn((
        Name::new(format!("aa:{YARUO_NEUTRAL_AA_ID}")),
        Text2d::new(YARUO_NEUTRAL_AA),
        aa_font,
        TextColor(Color::srgb(0.92, 0.92, 0.86)),
        TextLayout::new(Justify::Left, LineBreak::NoWrap),
        Anchor::TOP_LEFT,
        Transform::from_xyz(-520.0, 220.0, 0.0),
    ));
}
