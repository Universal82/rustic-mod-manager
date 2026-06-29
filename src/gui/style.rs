pub static NO_SHADOW: iced::Shadow = iced::Shadow {
    color: iced::color!(0,0,0),
    offset: iced::Vector {
        x: 0.0,
        y: 0.0
    },
    blur_radius: 0.0,
};

pub static NO_RADIUS: iced::border::Radius = iced::border::Radius {
            top_left: 0.0,
            top_right: 0.0,
            bottom_right: 0.0,
            bottom_left: 0.0,
        };

pub mod light {
    use iced::color;

    const LUMA_OFFSET: u8 = 8;

    pub static BACKGROUND_A: iced::Background = iced::Background::Color(color!(127,127,127));
    pub static BACKGROUND_B: iced::Background = iced::Background::Color(color!(255,255,255));
    pub static BACKGROUND_LIST_A: iced::Background = iced::Background::Color(color!(127-LUMA_OFFSET,127-LUMA_OFFSET,127-LUMA_OFFSET));
    pub static BACKGROUND_LIST_B: iced::Background = iced::Background::Color(color!(127+LUMA_OFFSET,127+LUMA_OFFSET,127+LUMA_OFFSET));
    pub static TEXT_A: iced::Color = color!(255,255,255);
    pub static TEXT_B: iced::Color = color!(64,64,64);
    pub static BORDER_A: iced::Border = iced::Border {
        color: color!(200,200,200),
        width: 1.0,
        radius: super::NO_RADIUS,
    };
    pub static BORDER_B: iced::Border = iced::Border {
        color: color!(200,200,200),
        width: 1.0,
        radius: super::NO_RADIUS
    };

    pub fn button(_: &iced::Theme, status: iced::widget::button::Status) -> iced::widget::button::Style {
        match status {
            iced::widget::button::Status::Pressed => {
                iced::widget::button::Style {
                    background: Some(super::light::BACKGROUND_A),
                    text_color: super::light::TEXT_A,
                    border: super::light::BORDER_A,
                    shadow: super::NO_SHADOW,
                    snap: true,
                }
            }
            _ => {
                iced::widget::button::Style {
                    background: Some(super::light::BACKGROUND_B),
                    text_color: super::light::TEXT_B,
                    border: super::light::BORDER_B,
                    shadow: super::NO_SHADOW,
                    snap: true,
                }
            }
        }
    }

    pub fn container(_: &iced::Theme) -> iced::widget::container::Style {
        iced::widget::container::Style {
            text_color: None,
            background: Some(super::light::BACKGROUND_A),
            border: super::light::BORDER_A,
            shadow: super::NO_SHADOW,
            snap: true,
        }
    }

    pub fn list_container_1(_: &iced::Theme) -> iced::widget::container::Style {
        iced::widget::container::Style {
            text_color: None,
            background: Some(super::light::BACKGROUND_LIST_A),
            border: super::light::BORDER_A,
            shadow: super::NO_SHADOW,
            snap: true,
        }
    }

    pub fn list_container_2(_: &iced::Theme) -> iced::widget::container::Style {
        iced::widget::container::Style {
            text_color: None,
            background: Some(super::light::BACKGROUND_LIST_B),
            border: super::light::BORDER_B,
            shadow: super::NO_SHADOW,
            snap: true,
        }
    }
}
