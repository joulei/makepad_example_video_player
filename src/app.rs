use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;
    import makepad_draw::shader::draw_color::DrawColor;

    GAMING = dep("crate://self/resources/gaming.mp4")    
    //SUNSET = dep("crate://self/resources/sunset.mp4")

    // 60 FPS
    // CAT = dep("crate://self/resources/cat.mp4")

    // WITH AUDIO
    // SINGING = dep("crate://self/resources/singing.mp4")

    App = {{App}} {
        ui: <Window> {
            show_bg: true
            width: Fill,
            height: Fill,
            draw_bg: {color: #0}
 
            body = <View> {
                width: Fill,
                height: Fill,
                flow: Down,
                spacing: 20.0,
                align: {
                    x: 0.5,
                    y: 0.5
                },

                dep_video = <Video> {
                    source: Dependency { path: (GAMING)}
                    height: 185,
                    width: 320,
                    is_looping: true
                    hold_to_pause: true
                    autoplay: true
                }

                // Only works on android < 13. Requires the app to have storage read permissions, which must be 
                // requested at runtime on android > 13, doing so is not yet supported on Makepad.

                // fs_video = <Video> { 
                //     source: Filesystem { path: "/storage/emulated/0/DCIM/Camera/test.mp4"}
                //     height: 185,
                //     width: 320,
                //     is_looping: true
                //     hold_to_pause: true
                //     autoplay: true
                // }

                network_video = <Video> {
                    source: Network { url: "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4"}
                    height: 185,
                    width: 320,
                    is_looping: true
                    hold_to_pause: true
                    autoplay: true
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
        makepad_widgets::live_design(cx);
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