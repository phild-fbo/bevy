use crate::{ColorMaterial, Rect};
use bevy_asset::{AssetStorage, Handle};
use bevy_render::texture::Texture;
pub use legion::prelude::*;
pub struct Sprite {
    pub scale: f32,
}

impl Default for Sprite {
    fn default() -> Self {
        Sprite { scale: 1.0 }
    }
}

pub fn sprite_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("sprite_system")
        .read_resource::<AssetStorage<ColorMaterial>>()
        .read_resource::<AssetStorage<Texture>>()
        .with_query(
            <(Read<Sprite>, Read<Handle<ColorMaterial>>, Write<Rect>)>::query().filter(
                changed::<Sprite>() | changed::<Rect>() | changed::<Handle<ColorMaterial>>(),
            ),
        )
        .build(|_, world, (materials, textures), query| {
            for (sprite, handle, mut rect) in query.iter_mut(world) {
                let material = materials.get(&handle).unwrap();
                if let Some(texture_handle) = material.texture {
                    let texture = textures.get(&texture_handle).unwrap();
                    let aspect = texture.aspect();
                    *rect.size.x_mut() = texture.width as f32 * sprite.scale;
                    *rect.size.y_mut() = rect.size.x() * aspect;
                }
            }
        })
}