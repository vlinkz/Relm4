use std::time::Duration;

use gtk::cairo::{Context, Operator};
use gtk::prelude::*;
use relm4::drawing::DrawHandler;
use relm4::{gtk, Component, ComponentParts, ComponentSender, RelmApp, WidgetPlus};

#[derive(Debug)]
enum AppMsg {
    AddPoint((f64, f64)),
    Reset,
    Resize((i32, i32)),
}

#[derive(Debug)]
struct UpdatePointsMsg;

struct AppModel {
    width: f64,
    height: f64,
    points: Vec<Point>,
    handler: DrawHandler,
}

#[relm4::component]
impl Component for AppModel {
    type InitParams = ();
    type Input = AppMsg;
    type Output = ();
    type CommandOutput = UpdatePointsMsg;
    type Widgets = AppWidgets;

    view! {
      gtk::Window {
        set_default_height: 300,
        set_default_width: 600,

        gtk::Box {
          set_orientation: gtk::Orientation::Vertical,
          set_margin_all: 10,
          set_spacing: 10,
          set_hexpand: true,

          gtk::Label {
            set_label: "Left-click to add circles, resize or right-click to reset!",
          },

          #[name = "area"]
          gtk::DrawingArea {
            set_vexpand: true,
            set_hexpand: true,

            add_controller = &gtk::GestureClick::new() {
              set_button: 0,
              connect_pressed[sender] => move |controller, _, x, y| {
                if controller.current_button() == gtk::gdk::BUTTON_SECONDARY {
                    sender.input(AppMsg::Reset);
                } else {
                    sender.input(AppMsg::AddPoint((x, y)));
                }
              }
            },
            connect_resize[sender] => move |_, x, y| {
                sender.input(AppMsg::Resize((x, y)));
            }
          },
        }
      }
    }

    fn update(&mut self, msg: AppMsg, _sender: ComponentSender<Self>) {
        let cx = self.handler.get_context().unwrap();

        match msg {
            AppMsg::AddPoint((x, y)) => {
                self.points.push(Point::new(x, y));
            }
            AppMsg::Resize((x, y)) => {
                self.width = x as f64;
                self.height = y as f64;
            }
            AppMsg::Reset => {
                cx.set_operator(Operator::Clear);
                cx.set_source_rgba(0.0, 0.0, 0.0, 0.0);
                cx.paint().expect("Couldn't fill context");
            }
        }

        draw(&cx, &self.points);
    }

    fn update_cmd(&mut self, _: UpdatePointsMsg, _: ComponentSender<Self>) {
        for point in &mut self.points {
            let Point { x, y, .. } = point;
            if *x < 0.0 {
                point.xs = point.xs.abs();
            } else if *x > self.width {
                point.xs = -point.xs.abs();
            }
            *x = x.clamp(0.0, self.width);
            *x += point.xs;

            if *y < 0.0 {
                point.ys = point.ys.abs();
            } else if *y > self.height {
                point.ys = -point.ys.abs();
            }
            *y = y.clamp(0.0, self.height);
            *y += point.ys;
        }

        let cx = self.handler.get_context().unwrap();
        draw(&cx, &self.points);
    }

    fn init(_params: (), root: &Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let mut model = AppModel {
            width: 100.0,
            height: 100.0,
            points: Vec::new(),
            handler: DrawHandler::new().unwrap(),
        };

        let widgets = view_output!();

        model.handler.init(&widgets.area);

        sender.command(|out, shutdown| {
            shutdown
                .register(async move {
                    loop {
                        tokio::time::sleep(Duration::from_millis(20)).await;
                        out.send(UpdatePointsMsg);
                    }
                })
                .drop_on_shutdown()
        });

        ComponentParts { model, widgets }
    }
}

struct Point {
    x: f64,
    y: f64,
    xs: f64,
    ys: f64,
    color: Color,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        let angle: f64 = rand::random::<f64>() * std::f64::consts::PI * 2.0;
        Point {
            x,
            y,
            xs: angle.sin() * 7.0,
            ys: angle.cos() * 7.0,
            color: Color::random(),
        }
    }
}

struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    fn random() -> Color {
        Color {
            r: rand::random(),
            g: rand::random(),
            b: rand::random(),
        }
    }
}

fn draw(cx: &Context, points: &[Point]) {
    for point in points {
        let Point {
            x,
            y,
            color: Color { r, g, b },
            ..
        } = *point;
        cx.set_source_rgb(r, g, b);
        cx.arc(x, y, 10.0, 0.0, std::f64::consts::PI * 2.0);
        cx.fill().expect("Couldn't fill arc");
    }
}

fn main() {
    let relm = RelmApp::new("relm4.examples.drawing");
    relm.run::<AppModel>(());
}
