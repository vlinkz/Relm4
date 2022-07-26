use gtk::prelude::{
    BoxExt, ButtonExt, GridExt, GtkWindowExt, OrientableExt, ToggleButtonExt, WidgetExt,
};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, SimpleComponent, WidgetPlus};

#[tracker::track]
struct AppModel {
    value: u8,
}

#[derive(Debug)]
enum AppMsg {
    Increment,
    Decrement,
}

struct AppInit {
    counter: u8,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type InitParams = AppInit;
    type Input = AppMsg;
    type Output = ();
    // AppWidgets is generated by the macro
    type Widgets = AppWidgets;

    view! {
        #[root]
        #[name(main_window)]
        gtk::Window {
            set_title: Some("Macro reference example"),
            set_default_width: 300,
            set_default_height: 100,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                append: inc_button = &gtk::Button {
                    set_label: "Increment",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Increment);
                    }
                },

                gtk::Button {
                    set_label: "Decrement",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Decrement);
                    }
                },

                //gtk::Dialog::builder().header_bar(true).build() {
                gtk::Grid {
                    attach[1, 1, 1, 1] = &gtk::Label {
                        // Alternative: #[track = "counter.value % 10 == 0"]
                        #[track(counter.value % 10 == 0)]
                        set_label: &format!("Grid works! ({})", counter.value),
                    }
                },

                /// A conditional widget, wow!
                // Alternative: #[transition = "SlideLeft"]
                #[transition(SlideLeft)]
                append = if counter.value % 2 == 0 {
                    gtk::Label {
                        set_label: "Value is even",
                    }
                } else if counter.value % 3 == 0 {
                    gtk::Label {
                        set_label: "Value is dividable by 3",
                    }
                } else {
                    gtk::Label {
                        set_label: "Value is odd",
                    }
                },

                #[transition = "SlideRight"]
                append: match_stack = match counter.value {
                    (0..=2) => {
                        gtk::Label {
                            set_label: "Value is smaller than 3",
                        }
                    },
                    _ => {
                        gtk::Label {
                            set_label: "Value is higher than 2",
                        }
                    }
                },

                append = &gtk::Label,

                gtk::Label::builder()
                    .label("Builder pattern works!")
                    .selectable(true)
                    .build(),

                gtk::Label::new(Some("Constructors work!")),

                /// Counter label
                gtk::Label {
                    #[watch]
                    set_label: &format!("Counter: {}", counter.value),
                    #[track]
                    set_margin_all: counter.value.into(),
                },

                gtk::ToggleButton {
                    set_label: "Counter is even",
                    #[watch]
                    #[block_signal(toggle_handler)]
                    set_active: counter.value % 2 == 0,

                    connect_toggled[sender] => move |_| {
                        sender.input(AppMsg::Increment);
                    } @toggle_handler,
                },

                #[local]
                local_label -> gtk::Label {
                    set_opacity: 0.7,
                },

                #[local_ref]
                local_ref_label -> gtk::Label {
                    set_opacity: 0.7,
                    set_size_request: (40, 40),
                },
            }
        },
        gtk::Window {
            set_title: Some("Another window"),
            set_default_width: 300,
            set_default_height: 100,
            set_transient_for: Some(&main_window),
            // Empty args
            hide: (),
            #[watch]
            set_visible: counter.value == 42,

            #[name = "my_label_name"]
            gtk::Label {
                set_label: "You made it to 42!",
            }
        }
    }

    // Initialize the UI.
    fn init(
        params: Self::InitParams,
        renamed_root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let counter = AppModel {
            value: params.counter,
            tracker: 0,
        };

        let local_label = gtk::Label::new(Some("local_label"));
        let local_ref_label_value = gtk::Label::new(Some("local_ref_label"));
        let local_ref_label = &local_ref_label_value;
        // Insert the macro code generation here
        let widgets = view_output!();

        ComponentParts {
            model: counter,
            widgets,
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: &ComponentSender<Self>) {
        self.reset();
        match msg {
            AppMsg::Increment => {
                self.set_value(self.value.wrapping_add(1));
            }
            AppMsg::Decrement => {
                self.set_value(self.value.wrapping_sub(1));
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.macro_reference");
    app.run::<AppModel>(AppInit { counter: 0 });
}
