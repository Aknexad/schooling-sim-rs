use macroquad::window::Conf;

pub fn default() -> Conf {
    Conf {
        window_title: "schooling sim".to_string(),
        window_width: 650,
        window_height: 650,
        window_resizable: true,
        ..Default::default()
    }
}
