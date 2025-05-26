use makepad_widgets::*; // Import Makepad Widgets package


// Define live_design macro for declaring UI components and layout
live_design! {
    // import Makepad theme and shaders, and widgets
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    PLACEHOLDER_IMAGE = dep("crate://self/resources/placeholder.jpg");


    ImageItem = <View> {
        width: 256,
        height: 256,

        image = <Image> {
            width: Fill,
            height: Fill,
            source: (PLACEHOLDER_IMAGE),
            fit: Biggest,
        }
    }

    ImageRow = {{ImageRow}} {
        <PortalList> {
            height: 256,
            flow: Right,
            ImageItem = <ImageItem> {}
        }
    }

    ImageGrid = {{ImageGrid}} {
        <PortalList> {
            flow: Down,
            ImageRow = <ImageRow> {}
        }
    }

    App = {{App}} {
        ui: <Window> {
            body = <View> {
                // <ImageItem> {}
                // <ImageRow> {}
                <ImageGrid> {}
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ImageRow {
    #[deref]
    view: View,
}

impl Widget for ImageRow {

    fn draw_walk(
        &mut self, 
        cx: &mut Cx2d, 
        scope: &mut Scope, 
        walk: Walk
    ) -> DrawStep {
        while let Some(item) =self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, 4);
                while let Some(item_idx) = list.next_visible_item(cx) {
                    // log the item index
                    println!("item_idx: {}", item_idx);
                    let item = list.item(cx, item_idx, live_id!(ImageItem));
                    item.draw_all(cx, &mut Scope::empty());
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ImageGrid {
    #[deref]
    view: View,
}

impl Widget for ImageGrid {
    fn draw_walk(
        &mut self, 
        cx: &mut Cx2d, 
        scope: &mut Scope, 
        walk: Walk
    ) -> DrawStep {
        while let Some(item) =self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let state = scope.data.get_mut::<State>().unwrap();
                list.set_item_range(cx, 0, state.num_rows());
                while let Some(item_idx) = list.next_visible_item(cx) {
                    if item_idx >= state.num_rows() {
                        continue;
                    }
                    // log the item index
                    println!("item_idx: {}", item_idx);
                    let item = list.item(cx, item_idx, live_id!(ImageRow));
                    item.draw_all(cx, &mut Scope::empty());
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }
}

pub struct State {
    num_images: usize,
    max_images_per_row: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            num_images: 7,
            max_images_per_row: 4,
        }
    }
}

impl State {
    fn num_images(&self) -> usize {
        self.num_images
    }

    fn num_rows(&self) -> usize {
        self.num_images().div_ceil(self.max_images_per_row)
    }

    fn first_image_idx_for_row(&self, row_idx: usize) -> usize {
        row_idx * self.max_images_per_row
    }

    fn num_images_for_row(&self, row_idx: usize) -> usize {
        let first_image_idx = self.first_image_idx_for_row(row_idx);
        let remaining_images = self.num_images() - first_image_idx;
        self.max_images_per_row.min(remaining_images)
    }
}

// Define App struct containing UI and counter
#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef, // UI component reference
    #[rust]
    state: State,
}

// Implement LiveRegister trait for registering live design
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        // Register Makepad Widgets' live design
        makepad_widgets::live_design(cx);
    }
}

// Implement AppMain trait for handling events
impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        let mut scope = Scope::with_data(&mut self.state);
        // Handle UI events
        self.ui.handle_event(cx, event, &mut scope);
    }
}

// Define application entry point
app_main!(App);