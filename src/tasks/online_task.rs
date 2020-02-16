use blinkt::Blinkt;
use online::online;

pub fn OnlineTask(blinkt: &mut Blinkt, brightness: f32) {
    info!("Running online check task!");
    let is_online = match online(None) {
        Ok(is_online) => is_online,
        Err(_) => false
    };
    match is_online {
        true => blinkt.set_pixel_rgbb(6, 0, 10, 0, brightness),
        false => {
            error!("There is no internet!");
            blinkt.set_pixel_rgbb(6, 10, 0, 0, brightness)
        }
    };
}
