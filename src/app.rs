use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    TEST = dep("crate://self/resources/test.mp4")
    
    App = {{App}} {
        ui: <DesktopWindow>{
            show_bg: true
            width: Fill,
            height: Fill
            
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return mix(#7, #3, self.pos.y);
                }
            }
 
            <View>{
                flow: Down,
                align: {
                    x: 0.5,
                    y: 0.5
                
                },

                <Video> {
                    source: (TEST)
                    width: Fill,
                    height: Fill,
                    is_looping: true
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl LiveHook for App {
    fn before_live_design(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}

impl AppMain for App{
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            return self.ui.draw_widget_all(&mut Cx2d::new(cx, event));
        }

        self.ui.handle_widget_event(cx, event);
    }
}