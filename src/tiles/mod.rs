// example from https://bevyengine.org/examples/UI%20(User%20Interface)/ui-texture-atlas/
// use bevy::prelude::*;
//
// fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
// ) {
//     // Camera
//     commands.spawn(Camera2dBundle::default());
//
//     let text_style = TextStyle {
//         font_size: 20.,
//         ..default()
//     };
//
//     let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
//     let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(24.0, 24.0), 7, 1, None, None);
//     let texture_atlas_handle = texture_atlases.add(texture_atlas);
//
//     // root node
//     commands
//         .spawn(NodeBundle {
//             style: Style {
//                 width: Val::Percent(100.0),
//                 height: Val::Percent(100.0),
//                 flex_direction: FlexDirection::Column,
//                 justify_content: JustifyContent::Center,
//                 align_items: AlignItems::Center,
//                 row_gap: Val::Px(text_style.font_size * 2.),
//                 ..default()
//             },
//             ..default()
//         })
//         .with_children(|parent| {
//             parent.spawn(AtlasImageBundle {
//                 style: Style {
//                     width: Val::Px(256.),
//                     height: Val::Px(256.),
//                     ..default()
//                 },
//                 texture_atlas: texture_atlas_handle.into(),
//                 image: UiImage::new(texture_handle),
//                 ..default()
//             });
//             parent.spawn(TextBundle::from_sections([
//                 TextSection::new("press ".to_string(), text_style.clone()),
//                 TextSection::new(
//                     "space".to_string(),
//                     TextStyle {
//                         color: Color::YELLOW,
//                         ..text_style.clone()
//                     },
//                 ),
//                 TextSection::new(" to advance frames".to_string(), text_style),
//             ]));
//         });
// }
