//! Custom macroquad UI skin used by the menu windows and buttons.

use macroquad::{
    color::Color,
    math::RectOffset,
    texture::Image,
    ui::{root_ui, Skin},
};

pub struct GuiStyle {
    pub menu_skin: Skin,
}

impl GuiStyle {
    pub fn new() -> GuiStyle {
        let menu_skin = {
            //styles for ui menu, text, and buttons
            let window_style = root_ui()
                .style_builder()
                .background(
                    Image::from_file_with_format(
                        include_bytes!("../../assets/menu/Menu.png"),
                        None,
                    )
                    .unwrap(),
                )
                .background_margin(RectOffset::new(200.0, 200.0, 200.0, 200.0))
                .margin(RectOffset::new(-153.0, 0.0, -153.0, 0.0))
                .build();

            let label_style = root_ui()
                .style_builder()
                .text_color(Color {
                    r: 0.322,
                    g: 0.2,
                    b: 0.247,
                    a: 1.0,
                })
                .font(include_bytes!("../../assets/fonts/Montfaucon.ttf"))
                .unwrap()
                .font_size(40)
                .build();

            let button_style = root_ui()
                .style_builder()
                .background(
                    Image::from_file_with_format(
                        include_bytes!("../../assets/menu/Button.png"),
                        None,
                    )
                    .unwrap(),
                )
                .background_margin(RectOffset::new(37.0, 37.0, 27.0, 27.0))
                .background_hovered(
                    Image::from_file_with_format(
                        include_bytes!("../../assets/menu/Buttonhovered.png"),
                        None,
                    )
                    .unwrap(),
                )
                .background_clicked(
                    Image::from_file_with_format(
                        include_bytes!("../../assets/menu/Buttonclicked.png"),
                        None,
                    )
                    .unwrap(),
                )
                .font(include_bytes!("../../assets/fonts/Montfaucon.ttf"))
                .unwrap()
                .text_color(Color {
                    r: 0.929,
                    g: 0.918,
                    b: 0.878,
                    a: 1.0,
                })
                .text_color_hovered(Color {
                    r: 0.929,
                    g: 0.918,
                    b: 0.878,
                    a: 1.0,
                })
                .text_color_clicked(Color {
                    r: 0.929,
                    g: 0.918,
                    b: 0.878,
                    a: 1.0,
                })
                .font_size(40)
                .build();

            Skin {
                window_style,
                label_style,
                button_style,
                ..root_ui().default_skin()
            }
        };

        GuiStyle { menu_skin }
    }
}
