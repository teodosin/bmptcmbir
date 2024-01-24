use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_mod_picking::prelude::*;

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)

        .add_systems(Startup, setup)
        .add_systems(PostStartup, uinode_add_drag)
        .add_systems(PostUpdate, uinode_transform_to_style)
        .add_systems(PostUpdate, update_top_panel_colors)
    ;
    app.run();
}

#[derive(Component)]
struct UiNode(Vec2);

#[derive(Component)]
struct UiTopBar;

fn setup(
    mut commands: Commands,
){
    let position = Vec2::new(20.0, 20.0);
    let size = Vec2::new(100.0, 100.0);
    let col = Color::rgb(0.2, 0.2, 0.2);

    let base_panel = commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                left: Val::Px(position.x),
                top: Val::Px(position.y),
                width: Val::Px(size.x),
                height: Val::Px(size.y),
                border: UiRect {
                    left: Val::Px(2.0),
                    right: Val::Px(2.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(2.0),
                },
                ..Default::default()
            },
            background_color: BackgroundColor::from(Color::rgb(0.1, 0.1, 0.1)),
            border_color: BorderColor::from(col),
            transform: Transform::from_xyz(position.x, position.y, -1.0),
            ..default()
        },
        UiNode(position),
        // Pickable::IGNORE,
    )).id();

    let top_bar = commands.spawn((
        ButtonBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                width: Val::Px(size.x),
                height: Val::Px(12.0),
                align_items: AlignItems::Start,
                align_self: AlignSelf::FlexEnd,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor::from(col),
            ..default()
        },
        UiTopBar,
        PickableBundle::default(),

    )).id();

    commands.entity(base_panel).push_children(&[top_bar]);

    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        },
    ));
}

fn uinode_add_drag(
    mut commands: Commands,
    mut query: Query<Entity, Added<UiNode>>,
){
    for entity in &mut query.iter_mut() {
        commands.entity(entity).insert(
            On::<Pointer<Drag>>::target_component_mut::<UiNode>(|drag, tform| {
                tform.0 += drag.event.delta;
            }),
        );
    }
}


fn uinode_transform_to_style(
    mut nodes: Query<(&UiNode, &mut Style), Changed<UiNode>>,
    window: Query<&Window>,
){
    if nodes.iter_mut().count() == 0 {
        return;
    }

    let viewport = window.single();

    let viewport = &viewport.resolution;
    let viewport = Vec2::new(viewport.physical_width() as f32, viewport.physical_height() as f32);

    for (transform, mut style) in &mut nodes.iter_mut() {
        let mut new_pos = Vec2::new(transform.0.x, transform.0.y);
        if new_pos.x < 0. {
            new_pos.x = 0.;
        }
        if new_pos.y < 0. {
            new_pos.y = 0.;
        }

        let width = style.width.resolve(1.0, Vec2::new(viewport.x as f32, viewport.y as f32));
        let width = width.unwrap();

        let height = style.height.resolve(1.0, Vec2::new(viewport.x as f32, viewport.y as f32));
        let height = height.unwrap();

        if new_pos.x > viewport.x as f32 - width {
            new_pos.x = viewport.x as f32 - width;
        }
        if new_pos.y > viewport.y as f32 - height {
            new_pos.y = viewport.y as f32 - height;
        }
        
        style.left = Val::Px(new_pos.x);
        style.top = Val::Px(new_pos.y);
    }
}


fn update_top_panel_colors(
    mut buttons: Query<(Option<&PickingInteraction>, &mut BackgroundColor), (With<Button>, With<UiTopBar>)>,
) {
    for (interaction, mut button_color) in &mut buttons {
        *button_color = match interaction {
            Some(PickingInteraction::Pressed) => Color::rgb(0.35, 0.75, 0.35),
            Some(PickingInteraction::Hovered) => Color::rgb(0.25, 0.25, 0.25),
            Some(PickingInteraction::None) | None => Color::rgb(0.15, 0.15, 0.15),
        }
        .into();
    }
}