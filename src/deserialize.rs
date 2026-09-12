use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct AtlasData
{
    pub frames: HashMap<String, SpriteData>,
    pub meta: Meta
}

#[derive(Deserialize)]
pub struct Meta
{
    pub image: String,
    pub size: Size
}

#[derive(Deserialize)]
pub struct Size
{
    pub w: u16,
    pub h: u16
}

#[derive(Clone, Copy, Deserialize)]
pub struct SpriteData
{
    pub frame: Rect
}

#[derive(Clone, Copy, Deserialize)]
pub struct Rect
{
    pub h: u16,
    pub w: u16,
    pub x: u16,
    pub y: u16
}

impl AtlasData
{
    fn from_json(path: &str) -> serde_json::Result<AtlasData>
    {
        let json = std::fs::read_to_string(path).unwrap();
        let atlas: AtlasData = serde_json::from_str(&json)?;

        Ok(atlas)
    }

    pub(crate) fn compute_sprites(&self) -> HashMap<String, crate::sprite::Sprite> {
        todo!()
    }
}

#[cfg(test)]
mod tests
{
    use crate::deserialize::AtlasData;

    #[test]
    fn parse_check()
    {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/test_tiles/output/atlas.json");
        let atlas: AtlasData = AtlasData::from_json(&path).expect("Error: failed to create the atlas");

        let bleed = atlas.frames["bleed_dot"];
        assert_eq!(atlas.meta.size.w, 448);
        assert_eq!(bleed.frame.x, 52);
        assert_eq!(bleed.frame.h, 32);
    }
}
