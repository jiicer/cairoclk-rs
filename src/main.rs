/* 
 https://medium.com/journey-to-rust/drawing-in-gtk-in-rust-part-1-4a401eecc4e0
 https://gtk-rs.org/
 https://github.com/gtk-rs/examples/blob/master/src/bin/clock.rs

 */

extern crate chrono;
extern crate gdk;
extern crate gio;
extern crate gtk;

use cairo::Context;
use chrono::{Local, Timelike};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ContainerExt, DrawingArea, WidgetExt};
use std::env;
use std::f64::consts::PI;

fn draw(c: &Context) {
    let sec = Local::now().second() as f64;
    let min = Local::now().minute() as f64;
    let (_, hrs) = Local::now().hour12();
    let hr = hrs as f64;

    c.save();
    c.set_source_rgb(1.0, 1.0, 1.0);
    c.paint();
    c.set_source_rgb(0.0, 0.0, 0.0);
    c.translate(75.0, 75.0);
    c.scale(0.4, 0.4);
    c.rotate(-PI / 2.0);
    c.set_line_width(8.0);
    c.set_line_cap(cairo::LineCap::Round);

    // Hour marks
    c.save();
    for _ in 0..12 {
        c.new_path();
        c.rotate(PI / 6.0);
        c.move_to(100.0, 0.0);
        c.line_to(120.0, 0.0);
        c.stroke();
    }
    c.restore();

    // Minute marks
    c.save();
    c.set_line_width(5.0);
    for i in 0..60 {
        if i % 5 != 0 {
            c.new_path();
            c.move_to(117.0, 0.0);
            c.line_to(120.0, 0.0);
            c.stroke();
        }
        c.rotate(PI / 30.0);
    }
    c.restore();

    c.set_source_rgb(0.0, 0.0, 0.0);

    // write Hours
    c.save();
    c.rotate(hr * (PI / 6.0) + (PI / 360.0) * min + (PI / 21600.0) * sec);
    c.set_line_width(14.0);
    c.new_path();
    c.move_to(-20.0, 0.0);
    c.line_to(80.0, 0.0);
    c.stroke();
    c.restore();

    // write Minutes
    c.save();
    c.rotate((PI / 30.0) * min + (PI / 1800.0) * sec);
    c.set_line_width(10.0);
    c.new_path();
    c.move_to(-28.0, 0.0);
    c.line_to(112.0, 0.0);
    c.stroke();
    c.restore();

    // Write seconds
    c.save();
    c.rotate(sec * PI / 30.0);
    c.set_source_rgb(1.0, 0.0, 0.0);
    //c.strokeStyle = '#D40000';
    //c.fillStyle = '#D40000';
    c.set_line_width(6.0);
    c.new_path();
    c.move_to(-30.0, 0.0);
    c.line_to(83.0, 0.0);
    c.stroke();
    c.new_path();
    c.arc(0.0, 0.0, 10.0, 0.0, PI * 2.0); // anticlockwise = true
    c.fill();
    c.new_path();
    c.arc(95.0, 0.0, 10.0, 0.0, PI * 2.0); // anticlockwise = true
    c.stroke();
    //c.fillStyle = 'rgba(0, 0, 0, 0)';
    c.arc(0.0, 0.0, 3.0, 0.0, PI * 2.0); // anticlockwise = true
    c.fill();
    c.restore();

    c.new_path();
    c.set_line_width(14.0);
    //c.strokeStyle = '#325FA2';
    c.arc(0.0, 0.0, 142.0, 0.0, PI * 2.0); // anticlockwise = true
    c.stroke();

    c.restore();
}

fn app(app: &gtk::Application) {
    let win = gtk::ApplicationWindow::new(app);

    win.set_default_size(500, 500);
    win.set_title("Clock on Cairo Canvas");
    let frame = gtk::Frame::new(None);
    let area = DrawingArea::new();

    area.connect_draw(move |_, c| {
        draw(c);
        gtk::Inhibit(false)
    });
    frame.add(&area);
    win.add(&frame);

    // we are using a closure to capture the label (else we could also use a normal function)
    let tick = move || {
        // we could return glib::Continue(false) to stop our clock after this tick
        area.queue_draw();
        glib::Continue(true)
    };
    // executes the closure once every 0.5s
    gtk::timeout_add(500, tick);

    win.show_all();
}

fn main() {
    let uiapp = gtk::Application::new(None, gio::ApplicationFlags::FLAGS_NONE)
        .expect("Application::new failed");
    uiapp.connect_activate(app);

    uiapp.run(&env::args().collect::<Vec<_>>());
}
