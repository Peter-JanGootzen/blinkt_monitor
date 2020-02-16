use blinkt::Blinkt;

pub type Task = Box<dyn Fn(&mut Blinkt, f32) + 'static>;

mod online_task;
pub use online_task::OnlineTask;
