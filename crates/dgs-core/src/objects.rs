use crate::color::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PointRef {
    Named(String),
    Coords(f64, f64),
}

impl PointRef {
    pub fn resolve(&self, lookup: &HashMap<String, (f64, f64)>) -> Option<(f64, f64)> {
        match self {
            PointRef::Named(name) => lookup.get(name).copied(),
            PointRef::Coords(x, y) => Some((*x, *y)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GeoObject {
    #[serde(rename = "point")]
    Point {
        name: Option<String>,
        coords: (f64, f64),
        color: Option<Color>,
        size: Option<f64>,
    },
    #[serde(rename = "line")]
    Line {
        from: PointRef,
        to: PointRef,
        color: Option<Color>,
        stroke: Option<f64>,
    },
    #[serde(rename = "circle")]
    Circle {
        center: PointRef,
        radius: f64,
        color: Option<Color>,
        stroke: Option<f64>,
        fill: Option<Color>,
    },
    #[serde(rename = "polygon")]
    Polygon {
        points: Vec<PointRef>,
        color: Option<Color>,
        stroke: Option<f64>,
        fill: Option<Color>,
    },
    #[serde(rename = "ellipse")]
    Ellipse {
        center: PointRef,
        rx: f64,
        ry: f64,
        rotation: Option<f64>,
        color: Option<Color>,
        stroke: Option<f64>,
        fill: Option<Color>,
    },
    #[serde(rename = "arc")]
    Arc {
        center: PointRef,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        color: Option<Color>,
        stroke: Option<f64>,
    },
    #[serde(rename = "semicircle")]
    Semicircle {
        from: PointRef,
        to: PointRef,
        center: Option<PointRef>,
        dir: String,
        color: Option<Color>,
        stroke: Option<f64>,
        fill: Option<Color>,
    },
    #[serde(rename = "curve")]
    Curve {
        expr_str: String,
        t_min: Option<f64>,
        t_max: Option<f64>,
        var_name: String,
        color: Option<Color>,
        stroke: Option<f64>,
    },
    #[serde(rename = "curve_param")]
    CurveParam {
        x_expr: String,
        y_expr: String,
        t_min: Option<f64>,
        t_max: Option<f64>,
        color: Option<Color>,
        stroke: Option<f64>,
    },
    #[serde(rename = "resolved_curve")]
    ResolvedCurve {
        points: Vec<(f64, f64)>,
        color: Option<Color>,
        stroke: Option<f64>,
    },
}

pub fn build_point_lookup(objects: &[GeoObject]) -> HashMap<String, (f64, f64)> {
    let mut lookup = HashMap::new();
    for obj in objects {
        if let GeoObject::Point {
            name: Some(name),
            coords,
            ..
        } = obj
        {
            lookup.insert(name.clone(), *coords);
        }
    }
    lookup
}

pub fn resolve_objects(
    objects: &[GeoObject],
    lookup: &HashMap<String, (f64, f64)>,
) -> Vec<GeoObject> {
    let mut resolved = Vec::with_capacity(objects.len());
    for obj in objects {
        match obj {
            GeoObject::Point { .. } => {
                resolved.push(obj.clone());
            }
            GeoObject::Line {
                from,
                to,
                color,
                stroke,
            } => {
                resolved.push(GeoObject::Line {
                    from: resolve_point_ref(from, lookup),
                    to: resolve_point_ref(to, lookup),
                    color: *color,
                    stroke: *stroke,
                });
            }
            GeoObject::Circle {
                center,
                radius,
                color,
                stroke,
                fill,
            } => {
                resolved.push(GeoObject::Circle {
                    center: resolve_point_ref(center, lookup),
                    radius: *radius,
                    color: *color,
                    stroke: *stroke,
                    fill: *fill,
                });
            }
            GeoObject::Polygon {
                points,
                color,
                stroke,
                fill,
            } => {
                resolved.push(GeoObject::Polygon {
                    points: points
                        .iter()
                        .map(|p| resolve_point_ref(p, lookup))
                        .collect(),
                    color: *color,
                    stroke: *stroke,
                    fill: *fill,
                });
            }
            GeoObject::Ellipse {
                center,
                rx,
                ry,
                rotation,
                color,
                stroke,
                fill,
            } => {
                resolved.push(GeoObject::Ellipse {
                    center: resolve_point_ref(center, lookup),
                    rx: *rx,
                    ry: *ry,
                    rotation: *rotation,
                    color: *color,
                    stroke: *stroke,
                    fill: *fill,
                });
            }
            GeoObject::Arc {
                center,
                radius,
                start_angle,
                end_angle,
                color,
                stroke,
            } => {
                resolved.push(GeoObject::Arc {
                    center: resolve_point_ref(center, lookup),
                    radius: *radius,
                    start_angle: *start_angle,
                    end_angle: *end_angle,
                    color: *color,
                    stroke: *stroke,
                });
            }
            GeoObject::Semicircle {
                from,
                to,
                center,
                dir,
                color,
                stroke,
                fill,
            } => {
                resolved.push(GeoObject::Semicircle {
                    from: resolve_point_ref(from, lookup),
                    to: resolve_point_ref(to, lookup),
                    center: center.as_ref().map(|c| resolve_point_ref(c, lookup)),
                    dir: dir.clone(),
                    color: *color,
                    stroke: *stroke,
                    fill: *fill,
                });
            }
            GeoObject::Curve {
                expr_str,
                t_min,
                t_max,
                var_name,
                color,
                stroke,
            } => {
                let points = evaluate_curve(expr_str, var_name, *t_min, *t_max, lookup);
                resolved.push(GeoObject::ResolvedCurve {
                    points,
                    color: *color,
                    stroke: *stroke,
                });
            }
            GeoObject::CurveParam {
                x_expr,
                y_expr,
                t_min,
                t_max,
                color,
                stroke,
            } => {
                let points = evaluate_parametric(x_expr, y_expr, *t_min, *t_max);
                resolved.push(GeoObject::ResolvedCurve {
                    points,
                    color: *color,
                    stroke: *stroke,
                });
            }
            GeoObject::ResolvedCurve { .. } => {
                resolved.push(obj.clone());
            }
        }
    }
    resolved
}

fn resolve_point_ref(pr: &PointRef, lookup: &HashMap<String, (f64, f64)>) -> PointRef {
    match pr {
        PointRef::Named(name) => match lookup.get(name) {
            Some(coords) => PointRef::Coords(coords.0, coords.1),
            None => PointRef::Named(name.clone()),
        },
        PointRef::Coords(x, y) => PointRef::Coords(*x, *y),
    }
}

fn evaluate_curve(
    expr_str: &str,
    var_name: &str,
    t_min: Option<f64>,
    t_max: Option<f64>,
    _lookup: &HashMap<String, (f64, f64)>,
) -> Vec<(f64, f64)> {
    let expr = match dgs_parser::parse(expr_str) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };

    let default_min = -10.0;
    let default_max = 10.0;
    let min = t_min.unwrap_or(default_min);
    let max = t_max.unwrap_or(default_max);
    let steps = 200;
    let dt = (max - min) / steps as f64;

    let mut points = Vec::with_capacity(steps + 1);

    for i in 0..=steps {
        let t = min + dt * i as f64;
        if let Ok(y) = dgs_parser::eval(&expr, &[(var_name, t)]) {
            if y.is_finite() {
                points.push((t, y));
            }
        }
    }

    points
}

fn evaluate_parametric(
    x_expr_str: &str,
    y_expr_str: &str,
    t_min: Option<f64>,
    t_max: Option<f64>,
) -> Vec<(f64, f64)> {
    let x_expr = match dgs_parser::parse(x_expr_str) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };
    let y_expr = match dgs_parser::parse(y_expr_str) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };

    let default_min = 0.0;
    let default_max = std::f64::consts::TAU;
    let min = t_min.unwrap_or(default_min);
    let max = t_max.unwrap_or(default_max);
    let steps = 200;
    let dt = (max - min) / steps as f64;

    let mut points = Vec::with_capacity(steps + 1);

    for i in 0..=steps {
        let t = min + dt * i as f64;
        let x = dgs_parser::eval(&x_expr, &[("t", t)]);
        let y = dgs_parser::eval(&y_expr, &[("t", t)]);
        if let (Ok(x_val), Ok(y_val)) = (x, y) {
            if x_val.is_finite() && y_val.is_finite() {
                points.push((x_val, y_val));
            }
        }
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_lookup() {
        let objects = vec![GeoObject::Point {
            name: Some("A".to_string()),
            coords: (1.0, 2.0),
            color: None,
            size: None,
        }];
        let lookup = build_point_lookup(&objects);
        assert_eq!(lookup.get("A"), Some(&(1.0, 2.0)));
    }

    #[test]
    fn test_resolve_named_point() {
        let pr = PointRef::Named("A".to_string());
        let mut lookup = HashMap::new();
        lookup.insert("A".to_string(), (3.0, 4.0));
        assert_eq!(pr.resolve(&lookup), Some((3.0, 4.0)));
    }

    #[test]
    fn test_resolve_curve() {
        let points = evaluate_curve("x^2", "x", Some(0.0), Some(2.0), &HashMap::new());
        assert!(!points.is_empty());
        assert!((points[0].1 - 0.0).abs() < 1e-10);
    }
}
