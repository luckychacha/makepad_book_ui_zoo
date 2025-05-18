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

    App = {{App}} {
        ui: <Window> {
            body = <View> {
                // <ImageItem> {}
                <ImageRow> {}
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



// Define App struct containing UI and counter
#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef, // UI component reference
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
        // Handle UI events
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

// Define application entry point
app_main!(App);