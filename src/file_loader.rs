use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::{Reflect, TypeUuid},
    utils::BoxedFuture,
};

/// Each entry contains a rect with the grass height, internally defined by [height, x,z,width,length].
/// It can be loaded from file (see examples)
#[derive(Debug, TypeUuid, serde::Deserialize, Reflect)]
#[uuid = "39a3dc56-aa9c-4543-8640-a018b74b5052"]
pub struct GrassFields(pub Vec<[f32; 5]>);

#[derive(Default)]
pub struct GrassFieldsAssetLoader;
impl AssetLoader for GrassFieldsAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<GrassFields>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["ron", "grass"]
    }
}