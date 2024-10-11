use crate::models::display::{Grid, GridCoord, CELL_WIDTH, LINE_HEIGHT};
use leptos::html::Div;
use leptos::*;
use leptos_use::use_resize_observer;

#[component]
pub fn Display(grid: ReadSignal<Grid>, grid_size: RwSignal<GridCoord>) -> impl IntoView {
    let (padding, set_padding) = create_signal((0.0, 0.0));
    let wrapper = create_node_ref::<Div>();

    use_resize_observer(wrapper, move |entries, _observer| {
        let rect = entries[0].content_rect();
        let width = rect.width();
        let height = rect.height();
        let x = width / CELL_WIDTH;
        let y = height / LINE_HEIGHT;
        let x_cells = x.floor() as usize;
        let y_cells = y.floor() as usize;
        grid_size.update(|size| *size = (x_cells, y_cells));
        let x_padding = (width - (x_cells as f64 * CELL_WIDTH)) / 2.0;
        let y_padding = (height - (y_cells as f64 * LINE_HEIGHT)) / 2.0;
        set_padding((x_padding, y_padding));
    });

    view! {
        <div class="wrapper" node_ref=wrapper>
            <div class="display"
                style:padding=move || format!("{}px {}px", padding.get().1, padding.get().0)
            >
                <For each=move || 0..grid_size.get().1 key=|&y| y
                    children=move |y| {
                    view! {
                        <div class="line">
                            <For each=move || 0..grid_size.get().0 key=|&x| x
                                children=move |x| {
                                view! {
                                    <span class="cell">
                                        {move || {
                                            grid.with(|g| {
                                                g.get(&(x, y))
                                                    .map(|cell| cell.char.to_string())
                                                    .unwrap_or_else(|| "".to_string())
                                            })
                                        }}
                                    </span>
                                }
                            }/>
                        </div>
                    }
                }/>
            </div>
        </div>
    }
}