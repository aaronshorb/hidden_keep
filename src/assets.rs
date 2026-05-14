//! Asset loading for fonts, textures, item images, and character sprite metadata.

use macroquad::prelude::*;
use std::fs;

#[derive(Clone)]
pub struct AnimationFrames {
    pub right: u32,
    pub left: u32,
    pub idle: u32,
}

#[derive(Clone)]
pub struct CharacterAsset {
    pub texture: Texture2D,
    pub name: String,
    pub animation_frames: AnimationFrames,
}

pub struct Assets {
    pub text_font: Font,
    pub background: Texture2D,
    pub board_back: Texture2D,
    pub gem_icon: Texture2D,
    pub pillar_icon: Texture2D,
    pub item_list: Vec<Texture2D>,
    pub characters: [CharacterAsset; 4],
}

impl Assets {
    pub async fn load() -> Self {
        let text_font = load_ttf_font("assets/fonts/Montfaucon.ttf").await.unwrap();

        //textures for background, gameboard and icons
        let background = load_texture("assets/backgrounds/nightsky.png")
            .await
            .unwrap();
        let board_back = load_texture("assets/backgrounds/floor.png").await.unwrap();
        let gem_icon = load_texture("assets/icons/Gem.png").await.unwrap();
        let pillar_icon = load_texture("assets/icons/Pillar.png").await.unwrap();

        //vector of 24 item textures in items directory
        let mut item_list: Vec<Texture2D> = Vec::new();
        let paths = fs::read_dir("assets/items/").unwrap();
        for path in paths {
            let item = path.unwrap().path().display().to_string();
            if item.ends_with(".png") {
                let texture = load_texture(item.as_str()).await.unwrap();
                item_list.push(texture);
            }
        }

        //textures to use for player avatars
        let knight = load_texture("assets/sprites/Knight.png").await.unwrap();
        let skeleton = load_texture("assets/sprites/Skeleton.png").await.unwrap();
        let wizard = load_texture("assets/sprites/Wizard.png").await.unwrap();
        let witch = load_texture("assets/sprites/Witch.png").await.unwrap();

        let characters = [
            CharacterAsset {
                texture: knight,
                name: "KNIGHT".to_string(),
                animation_frames: AnimationFrames {
                    right: 8,
                    left: 8,
                    idle: 4,
                },
            },
            CharacterAsset {
                texture: skeleton,
                name: "SKELETON".to_string(),
                animation_frames: AnimationFrames {
                    right: 7,
                    left: 7,
                    idle: 7,
                },
            },
            CharacterAsset {
                texture: wizard,
                name: "WIZARD".to_string(),
                animation_frames: AnimationFrames {
                    right: 7,
                    left: 7,
                    idle: 8,
                },
            },
            CharacterAsset {
                texture: witch,
                name: "WITCH".to_string(),
                animation_frames: AnimationFrames {
                    right: 7,
                    left: 7,
                    idle: 7,
                },
            },
        ];

        Self {
            text_font,
            background,
            board_back,
            gem_icon,
            pillar_icon,
            item_list,
            characters,
        }
    }
}
