pub mod arc;
pub mod axis;
pub mod circle;
pub mod color;
pub mod curve;
pub mod ellipse;
pub mod grid;
pub mod line;
pub mod objects;
pub mod point;
pub mod polygon;
pub mod semicircle;
pub mod svg;
pub mod theme;
pub mod viewport;

use serde::Deserialize;
use theme::Theme;
use viewport::Viewport;

#[derive(Deserialize)]
pub struct CanvasRequest {
    pub viewport: Viewport,
    pub theme: Theme,
    pub grid: bool,
    pub grid_color: Option<color::Color>,
    pub grid_width: Option<f64>,
    pub grid_spacing: Option<f64>,
    pub axes: bool,
    pub axis_color: Option<color::Color>,
    pub axis_width: Option<f64>,
    pub axis_label_size: Option<f64>,
    pub objects: Vec<objects::GeoObject>,
}

pub fn render(req: CanvasRequest) -> String {
    let mut svg = svg::SvgBuilder::new(req.viewport.width, req.viewport.height);

    // 1. Background
    svg.rect(
        0.0,
        0.0,
        req.viewport.width,
        req.viewport.height,
        req.theme.background(),
        None,
    );

    // 2. Grid
    if req.grid {
        grid::render_grid(
            &req.viewport,
            req.grid_color.unwrap_or_else(|| req.theme.grid_color()),
            req.grid_width.unwrap_or(0.5),
            req.grid_spacing.unwrap_or(1.0),
            &mut svg,
        );
    }

    // 3. Build point lookup and resolve objects
    let lookup = objects::build_point_lookup(&req.objects);
    let resolved = objects::resolve_objects(&req.objects, &lookup);

    // 4. Axes
    if req.axes {
        axis::render_axes(
            &req.viewport,
            req.axis_color.unwrap_or_else(|| req.theme.axis_color()),
            req.axis_width.unwrap_or(1.5),
            req.theme.axis_label_color(),
            req.axis_label_size.unwrap_or(10.0),
            &mut svg,
        );
    }

    // 5. Render all objects
    let default_color = req.theme.text_color();
    for obj in &resolved {
        match obj {
            objects::GeoObject::Point {
                coords,
                color,
                size,
                ..
            } => {
                point::render_point(
                    *coords,
                    color.unwrap_or(default_color),
                    size.unwrap_or(point::DEFAULT_POINT_SIZE),
                    &req.viewport,
                    &mut svg,
                );
            }
            objects::GeoObject::Line {
                from,
                to,
                color,
                stroke,
            } => {
                if let (Some(f), Some(t)) = (from.resolve(&lookup), to.resolve(&lookup)) {
                    line::render_line(
                        f,
                        t,
                        color.unwrap_or(default_color),
                        stroke.unwrap_or(line::DEFAULT_STROKE),
                        &req.viewport,
                        &mut svg,
                    );
                }
            }
            objects::GeoObject::Circle {
                center,
                radius,
                color,
                stroke,
                fill,
            } => {
                if let Some(c) = center.resolve(&lookup) {
                    circle::render_circle(
                        c,
                        *radius,
                        color.unwrap_or(default_color),
                        stroke.unwrap_or(1.5),
                        *fill,
                        &req.viewport,
                        &mut svg,
                    );
                }
            }
            objects::GeoObject::Polygon {
                points,
                color,
                stroke,
                fill,
            } => {
                let resolved_points: Vec<(f64, f64)> =
                    points.iter().filter_map(|p| p.resolve(&lookup)).collect();
                if !resolved_points.is_empty() {
                    polygon::render_polygon(
                        &resolved_points,
                        color.unwrap_or(default_color),
                        stroke.unwrap_or(1.5),
                        *fill,
                        &req.viewport,
                        &mut svg,
                    );
                }
            }
            objects::GeoObject::Ellipse {
                center,
                rx,
                ry,
                rotation,
                color,
                stroke,
                fill,
            } => {
                if let Some(c) = center.resolve(&lookup) {
                    ellipse::render_ellipse(
                        c,
                        *rx,
                        *ry,
                        rotation.unwrap_or(0.0),
                        color.unwrap_or(default_color),
                        stroke.unwrap_or(1.5),
                        *fill,
                        &req.viewport,
                        &mut svg,
                    );
                }
            }
            objects::GeoObject::Arc {
                center,
                radius,
                start_angle,
                end_angle,
                color,
                stroke,
            } => {
                if let Some(c) = center.resolve(&lookup) {
                    arc::render_arc(
                        c,
                        *radius,
                        *start_angle,
                        *end_angle,
                        color.unwrap_or(default_color),
                        stroke.unwrap_or(1.5),
                        &req.viewport,
                        &mut svg,
                    );
                }
            }
            objects::GeoObject::Semicircle {
                from,
                to,
                center,
                dir,
                color,
                stroke,
                fill,
            } => {
                if let (Some(f), Some(t)) = (from.resolve(&lookup), to.resolve(&lookup)) {
                    let c = center.as_ref().and_then(|c| c.resolve(&lookup));
                    semicircle::render_semicircle(
                        f,
                        t,
                        c,
                        dir,
                        color.unwrap_or(default_color),
                        stroke.unwrap_or(1.5),
                        *fill,
                        &req.viewport,
                        &mut svg,
                    );
                }
            }
            objects::GeoObject::ResolvedCurve {
                points,
                color,
                stroke,
            } => {
                curve::render_curve(
                    points,
                    color.unwrap_or(default_color),
                    stroke.unwrap_or(1.5),
                    &req.viewport,
                    &mut svg,
                );
            }
            objects::GeoObject::Curve { .. } => {
                // Unresolved curves should have been resolved by resolve_objects
            }
            objects::GeoObject::CurveParam { .. } => {
                // Unresolved parametric curves should have been resolved by resolve_objects
            }
        }
    }

    svg.build()
}
