use std::cmp;

use ratatui::layout::{Constraint, Direction, Layout, Rect};

#[allow(dead_code)]
pub fn max_height(rect: Rect, max: u16) -> Rect {
    let after = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(max), Constraint::Min(0)])
        .split(rect)[0];

    after
}

pub fn max_width(rect: Rect, max: u16) -> Rect {
    let after = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Max(max), Constraint::Min(0)])
        .split(rect)[0];

    after
}

pub fn transform(rect: Rect, x: i16, y: i16) -> Rect {
    let after_x: i16 = (rect.x as i16) + x;
    let after_y: i16 = (rect.y as i16) + y;

    Rect {
        x: cmp::max(after_x, 0) as u16,
        y: cmp::max(after_y, 0) as u16,
        width: rect.width,
        height: rect.height,
    }
}

/// Center a child rect inside a container rect
/// The child MUST completely fits within the container
pub fn center_rect_in_container(child: &mut Rect, container: &Rect) {
    if child.width > container.width || child.height > container.height {
        panic!("A child Rect must fit within the container Rect")
    }

    let center_x = container.x + container.width / 2;
    let center_y = container.y + container.height / 2;

    child.x = center_x - child.width / 2;
    child.y = center_y - child.height / 2;
}

#[cfg(test)]
mod max_height_tests {
    use super::*;
    use ratatui::layout::Rect;

    #[test]
    fn bigger_than_max_height() {
        let rect = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };

        let result = max_height(rect, 5);

        assert_eq!(
            result,
            Rect {
                x: 0,
                y: 0,
                width: 10,
                height: 5
            }
        )
    }

    #[test]
    fn smaller_than_max_height() {
        let rect = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };

        let result = max_height(rect, 15);

        assert_eq!(
            result,
            Rect {
                x: 0,
                y: 0,
                width: 10,
                height: 10
            }
        )
    }
}

#[cfg(test)]
mod max_width_tests {
    use super::*;
    use ratatui::layout::Rect;

    #[test]
    fn bigger_than_max_width() {
        let rect = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };

        let result = max_width(rect, 5);

        assert_eq!(
            result,
            Rect {
                x: 0,
                y: 0,
                width: 5,
                height: 10
            }
        )
    }

    #[test]
    fn smaller_than_max_width() {
        let rect = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };

        let result = max_width(rect, 15);

        assert_eq!(
            result,
            Rect {
                x: 0,
                y: 0,
                width: 10,
                height: 10
            }
        )
    }
}

#[cfg(test)]
mod transform_tests {
    use super::*;
    use ratatui::layout::Rect;

    #[test]
    fn normal() {
        let rect = Rect {
            x: 5,
            y: 5,
            width: 10,
            height: 10,
        };

        let result = transform(rect, 2, -2);

        assert_eq!(
            result,
            Rect {
                x: 7,
                y: 3,
                width: 10,
                height: 10
            }
        )
    }

    #[test]
    fn exceed_limit() {
        let rect = Rect {
            x: 2,
            y: 4,
            width: 10,
            height: 10,
        };

        let result = transform(rect, -5, -5);

        assert_eq!(
            result,
            Rect {
                x: 0,
                y: 0,
                width: 10,
                height: 10
            }
        )
    }
}

#[cfg(test)]
mod center_rect_in_container_tests {
    use super::*;
    use ratatui::layout::Rect;

    #[test]
    fn child_fits_within_container() {
        let mut child = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };

        let container = Rect {
            x: 10,
            y: 10,
            width: 20,
            height: 20,
        };

        center_rect_in_container(&mut child, &container);

        assert_eq!(
            child,
            Rect {
                x: 15,
                y: 15,
                width: 10,
                height: 10
            }
        )
    }

    #[test]
    #[should_panic]
    fn child_does_not_fit_within_container() {
        let mut child = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };

        let container = Rect {
            x: 10,
            y: 10,
            width: 8,
            height: 20,
        };

        center_rect_in_container(&mut child, &container);
    }
}
