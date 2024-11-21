use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct FileIcon {
    pub icon: char,
    pub color: u32,
}

impl Default for FileIcon {
    fn default() -> Self {
        Self {
            icon: '',
            color: 0xFFFFFF,
        }
    }
}

lazy_static! {
    pub static ref ICONS_BY_EXTENSION: HashMap<&'static str, FileIcon> = HashMap::from([
        (
            "glb",
            FileIcon {
                icon: '󰆧',
                color: 0x87C63E,
            }
        ),
        (
            "gltf",
            FileIcon {
                icon: '󰆧',
                color: 0x87C63E,
            }
        ),
        (
            "png",
            FileIcon {
                icon: '',
                color: 0xA074C4,
            }
        ),
        (
            "rs",
            FileIcon {
                icon: '',
                color: 0xDEA584,
            }
        ),
        (  "mid",
            FileIcon {
                icon: '󰎇',
                color: 0x800080,
            }
        ),
        (  "wav",
            FileIcon {
                icon: '󱑽',
                color: 0xF74231,
            }
        )
    ]);
}

pub const FOLDER_ICON: FileIcon = FileIcon {
    icon: '',
    color: 0xFFFFFF,
};
