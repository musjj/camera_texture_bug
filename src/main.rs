use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_state, update_window_status))
        .add_systems(OnEnter(AppState::Artifacting), setup_artifacting_texture)
        .add_systems(OnEnter(AppState::Clean), setup_clean_texture)
        .run()
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    #[default]
    Artifacting,
    Clean,
}

#[derive(Component)]
#[require(Text2d)]
struct WindowStatus;

fn setup(mut commands: Commands, window: Single<&Window, With<PrimaryWindow>>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text2d::new("Press space to switch sprite"),
        Transform::from_xyz(0.0, 330.0, 0.0),
    ));

    commands.spawn((
        WindowStatus,
        Text2d::new(format!(
            "Window size: {}x{}",
            window.size().x,
            window.size().y
        )),
        Transform::from_xyz(0.0, 250.0, 0.0),
    ));
}

fn setup_artifacting_texture(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text2d::new("Sprite dimension: 480x270"),
        Transform::from_xyz(0.0, 200.0, 0.0),
        StateScoped(AppState::Artifacting),
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("grid_480x270.png")),
        StateScoped(AppState::Artifacting),
    ));
}

fn setup_clean_texture(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text2d::new("Sprite dimension: 512x256"),
        Transform::from_xyz(0.0, 200.0, 0.0),
        StateScoped(AppState::Clean),
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("grid_512x256.png")),
        StateScoped(AppState::Clean),
    ));
}

fn toggle_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match app_state.get() {
            AppState::Artifacting => next_app_state.set(AppState::Clean),
            AppState::Clean => next_app_state.set(AppState::Artifacting),
        }
    }
}

fn update_window_status(
    mut status: Single<&mut Text2d, With<WindowStatus>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    if let Some(resize) = resize_reader.read().last() {
        status.0 = format!("Window size: {}x{}", resize.width, resize.height);
    }
}
