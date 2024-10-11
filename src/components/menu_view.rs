use std::time::Duration;

use leptos::*;
use log::info;

use crate::{
    components::display::Display,
    models::display::{Cell, Grid, GridCoord},
};

#[component]
pub fn MenuView() -> impl IntoView {
    let (grid, set_grid) = create_signal(Grid::new());
    let grid_size = create_rw_signal((0, 0));

    let (i, set_i) = create_signal(0);

    create_effect(move |_| {
        set_interval(
            move || {
                set_cell(
                    &set_grid,
                    (i(), i()),
                    Cell {
                        char: 'x',
                        background: 0,
                        foreground: 0,
                        bold: false,
                        italic: false,
                    },
                );
                set_i(i() + 1);
            },
            Duration::from_millis(500),
        );
    });

    view! {
        <Display grid=grid grid_size=grid_size />
    }
}

fn set_cell(grid: &WriteSignal<Grid>, coord: GridCoord, cell: Cell) {
    grid.update(|g| {
        g.insert(coord, cell);
    });
}
