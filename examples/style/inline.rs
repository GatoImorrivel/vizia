use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        Element::new(cx)
            .display(Display::Flex)
            .visibility(Visibility::Visible)
            .opacity(1.0)
            .overflow(Overflow::Visible)
            // .clip_path(Pixels(0.0))
            .backdrop_filter(Filter::Blur(Length::px(10.0)))
            .layout_type(LayoutType::Row)
            .position_type(PositionType::ParentDirected)
            .space(Pixels(30.0))
            .left(Pixels(30.0))
            .right(Pixels(30.0))
            .top(Pixels(30.0))
            .bottom(Pixels(30.0))
            .width(Pixels(50.0))
            .height(Pixels(50.0))
            .border_color(Color::black())
            .border_width(Pixels(4.0))
            .border_corner_shape(BorderCornerShape::Bevel)
            .border_radius(Pixels(20.0))
            .outline_color(Color::blue())
            .outline_offset(Pixels(20.0))
            .outline_width(Pixels(2.0))
            .background_color(Color::rebeccapurple())
            //.font_size(Pixels(20.0))
            .color(Color::moccasin())
            .font_weight(FontWeightKeyword::Bold)
            .font_style(FontStyle::Italic)
            .font_stretch(FontStretch::Condensed)
            // .font_family(FontFamily::Generic(GenericFontFamily::Serif))
            .selection_color(Color::royalblue())
            .caret_color(Color::azure())
            .text_wrap(true)
            .box_shadow(
                BoxShadowBuilder::new()
                    .x_offset(Pixels(10.0))
                    .y_offset(Pixels(10.0))
                    .color(Color::limegreen()),
            )
            // .translate(Translate::new(Pixels(50.0), Pixels(0.0)))
            // .rotate(Angle::Deg(30.0))
            // .scale(Scale::new(Percentage(3.0), 2.0))
            // .transform_origin((Pixels(0.0), Pixels(0.0)))
            .z_index(5)
            .cursor(CursorIcon::Grab);
    })
    .run();
}