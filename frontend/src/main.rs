use zoon::{named_color::*, println, *};

#[static_ref]
fn playing() -> &'static Mutable<bool> {
    Default::default()
}

fn root() -> impl Element {
    let video_element = Mutable::new(None);
    Column::new()
        .s(Padding::all(20))
        .s(Width::fill().max(600))
        .s(Align::center())
        .s(Gap::both(20))
        // Note: `web_sys` element (aka `DomElement`) cloning is cheap
        // because it clones only a reference to associated Javascript/browser DOM element.
        .item(video(video_element.clone()))
        .item(play_button(video_element))
}

fn video(video_element: Mutable<Option<web_sys::HtmlVideoElement>>) -> impl Element {
    // @TODO Improve `event_handler` API / creating events.
    make_event!(VideoPlay, "play" => web_sys::Event);
    make_event!(VideoPause, "pause" => web_sys::Event);

    // Note: `RawHtmlEl::new(..)`'s default `DomElement` is `web_sys::HtmlElement`.
    // Note/Warning: Video element controls and layout behave very differently on phones.
    RawHtmlEl::<web_sys::HtmlVideoElement>::new("video")
        .attr("controls", "")
        .use_dom_element(|this, dom_element| {
            // Note: Poster may cause video element height changes (e.g. in Chrome at the time of writing).
            dom_element.set_poster(
                "https://00f74ba44b093e4de48cfc48935c55aea42a77bfc9-apidata.googleusercontent.com/download/storage/v1/b/video-mz/o/vlcsnap-2022-08-30-06h28m32s287.png?jk=AFshE3Xq7_lhI8U5NzyEKPdj490_BG1l1Vnt8nIstBJmuoeegfRSHbGi-fy1Rpg8DOE4uL-B5il7BMYzIXu0bXSY7FdJTcyHx0i-5VvHKGUFws_NU7MCr_pydWelORIdrfVj46SDK3P0mLHcA2rpyErM9VWoLfXzQq18oOzZAaNLuUhLH7JlmEi0H4OCQPfXGFIkNgMkkrSzo4aLefKLNXfhcqPVwxA60BFNkz3IjQndnbcx4_crgbcxHG4YdD-9vAZV6EuPXzr1-EdyGgFIrhUSGmR2ZHN__4n5lXF6xCU_JjHux6GAdJd1ocMZhmxJREh5_dPShaGMVHyzSdkiGSu4vfl9-WmUEpJssMKrEpckdvWXdQU0GTb3KhQLBfKlN4u9irPingFADP8YNFgfyCBSuzpzqgQPYuXzS6m2C3dLTM0e2795uKpD4GLgSjia7Iqo51Ngj8Ji8svDkAvgQYZkAiI78mL1LX-p3NShD9MCz66mi0PVral-cPEl7DYK1WnHU7qS1RPAz5I936ULFgQI-ZkmD8Ey5s58GTtWd_UMoyPAQAIkcntzPQU0d6kQGjg-C0ywUzZum_tztiBavwdEeW5EqZQrAdO8JDvm9A4bf6Qcc_xRr6IGcCDQ-OXsk86heWdkQlED65U0GjY6pENMjQHgcyppy0EaNE01TOKPbrzli8Ufy6U2PMXNQpLhSGX8VJHgbvnErSUhOkmlmz7awPQfdBp6xIlrtfSME4VmhJciHau2hgt6UGV7nOQxwWyVcnhU3Ec6E9zuKvKjBV2VZj3debN-I1WmuWKF-8W-ECngrU19hGwGgNNcbaam5Vgb_Q2hnLjOFFaOzZZFVjsY0-_g7aGp_GQIZsoU_5NDoQNX39ea-ZKFDp7HggB50oYHkehfZjOu_4GrI-ZxSah9n8gXdL2siqd-FFVCRNOCYtlvNiUU1fiCOk_Q8YTTa9SEZ4VmrASViUu45LuIuGmneRSy9dKqtqnxml5v4ye7_NTVZmJbwoNvBqZ7GwsC__KvPdFge3YVtHwV_mFlrqFSGz_xC12v2BXtTXChs7hg632ijNB-f8G9xPCIRPD16rLht344f2Q&isca=1",
            );
            dom_element.set_src(
                "https://00f74ba44b109b9b1f6dbeeb1db57a3ec564bf6802-apidata.googleusercontent.com/download/storage/v1/b/video-mz/o/joji_Yukon.mp4?jk=AFshE3UIss_p94CiGkCaK152zisn16LophIJDoOCLdkgmn4pm336xziJt1S_ELrCyKKS62uZQYhWLaAe2Ti0q1fi5ZSACX9rTkPhjbmysgYuMJiYVBaJDGlOdDwahssQVrZjnM7-Dtt1arj9kVW2qtuXzlVA5U3Z0h2nregFUBxccY5LgIGo3yEju3r94pvTuTMU7OmXdxPv5o0pxu--qCUUFmjvH6ZaU6-p92S0BBW2g1lvBNsOywM2tXrz4V5mVEqz0Jrvsy4vW2ZYqbIBNlUq1bYiNp9WqmWObV4mQdXhYdriwzLScY0ZTlyiXJZA9D3lLETUN0w4hqiDrLPkz_SU7yPwx7KkmEM65xB1XHfOyv1KKEaH89XFTAT9BtIYzmztso1UGvxW4j-9EMSN7OTcPasiALcuOJNL0CORRb54QNUhaXVlfDCbb4biEBzJpivBEX8c_DrSjwmPbxjaXtKLKUrCGQNUDptuHBsY2Kce3RvwGOXUQOgqx_7gxUpGh5exGnkBbmEOuY_QoNSbyiP4ldZfnoR7ACGl9VfbdBT-qVvXwmhaXq5ht2mISTv8xCerSTYt_AbrhhFAUMO2rH4ahtyFdA5IZ2vfj8-vHvZhRBrh7Ctq7FDX8z-BXuUpXvChoWS1TFaMNXcUbxK37sLL_d6ESRI4B8RksC8vcnGjnPPGp7EOWFLxYLeUxVhmVdHw3SVnHOkf9_Z7U5gG2m7lK1SWqyxqQVTME5_oiYUQC93-rvzH-6usf6gGnKHfnqwZ9NdmIJeYv4eH2PaaxPaGd1bHM9PrnOUCt2L5qIG8-P4ChOt6OT4xNASBwxPb3TCbHnpCIl480eNLG0rEwntrv8Unb77dr37UXhA8OB63yGEtNhHSJyPkY4N1QI5TwA0bYU3cxegthKRiW0wcgOYZV3sI3o6NNGNd79h-5mubPuIuElRmdIJAGO0k9Ukzeh2HZJT6_EGIVPmZJCIDECwmVyB_li2b29DH5MGN0yMQa0CILsWqbiiUdFfVMCFEriiBMj5dPZoVEgeJuPDAtAKxJhCq407xLFM_dJSwG8i3ObE&isca=1",
            );
            this
        })
        .after_insert(move |dom_element| video_element.set(Some(dom_element)))
        .event_handler(|_: VideoPlay| playing().set_neq(true))
        .event_handler(|_: VideoPause| playing().set_neq(false))
        // Unchecked cast `DomElement` to another type.
        .dom_element_type::<web_sys::HtmlMediaElement>()
}

fn play_button(video_element: Mutable<Option<web_sys::HtmlVideoElement>>) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let size = 50;
    Button::new()
        .s(Visible::with_signal(
            video_element.signal_ref(Option::is_some),
        ))
        .s(Background::new().color_signal(hovered_signal.map_bool(|| BLUE_7, || BLUE_9)))
        .s(Align::new().center_x())
        .s(Width::exact(size))
        .s(Height::exact(size))
        .s(RoundedCorners::all_max())
        .s(Borders::all(Border::new().color(BLUE_5).width(2)))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_click(move || {
            video_element.use_ref(|video| {
                let video = video.as_ref().expect_throw("failed to get video element");
                if video.paused() {
                    // See https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play
                    let play_promise = video.play().expect_throw("failed to play video");
                    Task::start(async {
                        JsFuture::from(play_promise)
                            .await
                            .expect_throw("failed to play video");
                        println!("Play button clicked and playing!")
                    })
                } else {
                    video.pause().expect_throw("failed to pause video");
                }
            })
        })
        .label(play_button_icon())
}

fn play_button_icon() -> impl Element {
    macro_rules! make_icon {
        ($name:literal) => {
            $crate::paste! {
                fn [<icon_ $name>]() -> RawSvgEl<web_sys::SvgsvgElement> {
                    // Note: Icons downloaded from https://remixicon.com/.
                    RawSvgEl::from_markup(include_str!(concat!("../icons/", $name, ".svg")))
                        .unwrap_throw()
                        // Set `currentColor` in SVG elements.
                        .style("color", &BLUE_3.to_string())
                }
            }
        };
    }
    // Tip: You can write a `build.rs` script to automatically generate the lines below
    // according to the files in the `icons` folder,
    // see https://doc.rust-lang.org/cargo/reference/build-scripts.html
    make_icon!("play-fill");
    make_icon!("pause-fill");

    El::new().child_signal(playing().signal().map_bool(icon_pause_fill, icon_play_fill))
}

fn main() {
    start_app("app", root);
}
