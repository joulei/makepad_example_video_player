use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    VIDEO_1 = dep("crate://self/resources/video_1.mp4")
    // VIDEO_2 = dep("crate://self/resources/video_2.mp4")
    // VIDEO_3 = dep("crate://self/resources/video_3.mp4")
    // VIDEO_4 = dep("crate://self/resources/video_4.mp4")
    // VIDEO_5 = dep("crate://self/resources/video_5.mp4")
    
    App = {{App}} {
        ui: <Window>{
            show_bg: true
            width: 370,
            height: 640,
            draw_bg: {color: #0}
 
            <View>{
                width: 370,
                height: 640,

                <Video> {
                    source: (VIDEO_1)
                    width: 370,
                    height: 640,
                    is_looping: true
                    hold_to_pause: true
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