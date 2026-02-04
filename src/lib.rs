//! Renderer for Bevy UI using Vello.

use bevy_app::{App, Plugin, PostUpdate};
use bevy_ecs::prelude::*;
use bevy_ui::{BackgroundColor, ComputedNode, Node, UiGlobalTransform, UiSystems};
use vello::{
    Scene,
    kurbo::{Affine, Rect},
    peniko::{Color, Fill},
};

/// A Vello scene rebuilt each frame from Bevy UI nodes.
#[derive(Resource, Default)]
pub struct VelloUiScene {
    pub scene: Scene,
}

pub struct BevyUiRenderVelloPlugin;

impl Plugin for BevyUiRenderVelloPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VelloUiScene>()
            .add_systems(PostUpdate, build_vello_ui_scene.after(UiSystems::Stack));
    }
}

fn build_vello_ui_scene(
    mut vello_scene: ResMut<VelloUiScene>,
    nodes: Query<(&ComputedNode, &UiGlobalTransform, &BackgroundColor), With<Node>>,
) {
    vello_scene.scene.reset();

    let mut items: Vec<_> = nodes
        .iter()
        .filter(|(computed, _, background)| {
            !computed.is_empty() && background.0.to_srgba().alpha > 0.0
        })
        .collect();

    items.sort_by_key(|(computed, _, _)| computed.stack_index);

    for (computed, transform, background) in items {
        let size = computed.size();
        let rect = Rect::from_origin_size((0.0, 0.0), (size.x as f64, size.y as f64));
        let color = background_to_peniko(background);
        let affine = ui_transform_to_kurbo(transform);

        vello_scene
            .scene
            .fill(Fill::NonZero, affine, color, None, &rect);
    }
}

fn ui_transform_to_kurbo(transform: &UiGlobalTransform) -> Affine {
    let cols = transform.affine().to_cols_array();
    Affine::new([
        cols[0] as f64,
        cols[1] as f64,
        cols[2] as f64,
        cols[3] as f64,
        cols[4] as f64,
        cols[5] as f64,
    ])
}

fn background_to_peniko(background: &BackgroundColor) -> Color {
    let srgb = background.0.to_srgba();
    Color::new([srgb.red, srgb.green, srgb.blue, srgb.alpha])
}
