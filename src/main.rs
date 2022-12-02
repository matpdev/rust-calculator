use gtk::prelude::*;

mod constants;
mod functions;
mod gui;

fn main () {
   let application = gtk::Application::new(
      Some("br.com.matpdev.github"),
      Default::default(),
   );
   application.connect_activate(gui::build_ui);
   application.run();
}