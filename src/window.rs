use std::{array, collections::HashMap};

use gtk4::{prelude::GtkWindowExt, Application, ApplicationWindow, CenterBox, Grid};
use gtk4_layer_shell::{Edge, LayerShell};

pub enum Alignment {
    Left,
    Center,
    Right,
}

/// A window struct.
pub struct Win {
    /// GTK window
    win: ApplicationWindow,

    /// An array alignment grids.
    alignments: [Grid; 3],
}
impl Win {
    /// Creates a new window with given `app`
    pub fn new(app: Application) -> Self {
        Self {
            win: ApplicationWindow::new(&app),
            alignments: array::from_fn(|_| Grid::new()),
        }
    }

    /// Initializes a `gtk4-layer-shell`, setups alignments and root layout
    pub fn init(&self) {

        // Layer shell
        self.win.init_layer_shell();
        self.win.set_layer(gtk4_layer_shell::Layer::Overlay);
        self.win.auto_exclusive_zone_enable();

        let anchors = [
            ( Edge::Left,   true  ),
            ( Edge::Right,  true  ),
            ( Edge::Top,    true  ),
            ( Edge::Bottom, false ),
        ];

        for (e, v) in anchors {
            self.win.set_anchor(e, v);
        }

        // Root layout
        let root = CenterBox::new();
        root.set_start_widget(  Some(&self.alignments[0]));
        root.set_center_widget( Some(&self.alignments[1]));
        root.set_end_widget(    Some(&self.alignments[2]));
    }

    /// Runs a window
    pub fn run(&self) {
        self.win.present();
    }
}
