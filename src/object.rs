use gtk4::{prelude::WidgetExt, Grid};

/// A struct that provides a GTK grid of this objects and some implementations.
pub struct Object {
    /// A grid itselve
    grid: Grid,
}
impl Object {
    /// Returns a new instance of [Self](GridCapacitor)
    pub fn new() -> Self {
        Self {
            grid: Grid::new(),
        }
    }

    /// Returns a [gtk grid](Grid) of `self`
    pub fn get_grid(&self) -> Grid {
        self.grid.clone()
    }

    /// Attaches a new `widget` to a [grid](Object::grid)
    pub fn attach_widget<t: WidgetExt>(&self, widget: t) {
    }
}

/// An object behavior
pub trait ObjectTrait { 
    /// Returns a `Self`'s GTK widget
    fn get_gtk_widget(&self) -> Grid;
}
