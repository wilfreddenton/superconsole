use crate::{
    components::{Blank, Dimensions, DrawMode},
    content::LinesExt,
    Component, Line, State,
};

/// The `Padded` [`Component`](Component) wraps its child by padding left, right, above, and below its content.
/// This can be used to shift the content to a different location and ensure that following content comes after a certain distance.
/// It is worth noting that this component will also *truncate* any content that is too long to fit in the given window at draw time.
/// However, components are expected to constrain themselves to the given window, anyway.
///
/// Content is truncated preferentially over padding.
#[derive(Debug)]
pub struct Padded {
    pub child: Box<dyn Component>,
    pub left: usize,
    pub right: usize,
    pub top: usize,
    pub bottom: usize,
}

impl Default for Padded {
    fn default() -> Self {
        Self {
            child: Box::new(Blank),
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        }
    }
}

impl Padded {
    pub fn new(
        child: Box<dyn Component>,
        left: usize,
        right: usize,
        top: usize,
        bottom: usize,
    ) -> Self {
        Self {
            child,
            left,
            right,
            top,
            bottom,
        }
    }
}

impl Component for Padded {
    fn draw_unchecked(
        &self,
        state: &State,
        dimensions: Dimensions,
        mode: DrawMode,
    ) -> anyhow::Result<Vec<Line>> {
        let mut output = self.child.draw(state, dimensions, mode)?;

        // ordering is important:
        // the top and bottom lines need to be padded horizontally too.
        output.pad_lines_top(self.top);
        // cut off enough space at the bottom for the bottom padding
        output.truncate((dimensions.y as usize).saturating_sub(self.bottom));
        output.pad_lines_bottom(self.bottom);

        output.pad_lines_left(self.left);
        // cut off enough space on the right for the right padding
        output.truncate_lines((dimensions.x as usize).saturating_sub(self.right));
        output.pad_lines_right(self.right);

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use derive_more::AsRef;

    use crate::{
        components::{Echo, Padded},
        Component, Dimensions, DrawMode, Line, State,
    };

    #[derive(Debug, AsRef)]
    struct Msg(Vec<Line>);

    #[test]
    fn test_pad_left() {
        let padder = Padded {
            child: Box::new(Echo::<Msg>::new(false)),
            left: 5,
            ..Default::default()
        };
        let mut state = State::new();
        let msg = Msg(vec![
            vec!["hello world"].try_into().unwrap(),
            vec!["ok"].try_into().unwrap(),
            Line::default(),
        ]);
        state.insert(&msg);

        let drawing = padder
            .draw(&state, Dimensions::new(20, 20), DrawMode::Normal)
            .unwrap();
        let expected = vec![
            vec![" ".repeat(5).as_ref(), "hello world"]
                .try_into()
                .unwrap(),
            vec![" ".repeat(5).as_ref(), "ok"].try_into().unwrap(),
            vec![" ".repeat(5)].try_into().unwrap(),
        ];
        assert_eq!(drawing, expected);
    }

    #[test]
    fn test_pad_right() {
        let padder = Padded {
            child: Box::new(Echo::<Msg>::new(false)),
            right: 4,
            ..Default::default()
        };
        let mut state = State::new();
        let msg = Msg(vec![
            vec!["hello world"].try_into().unwrap(),
            vec!["ok"].try_into().unwrap(),
            Line::default(),
        ]);
        state.insert(&msg);

        let drawing = padder
            .draw(&state, Dimensions::new(20, 20), DrawMode::Normal)
            .unwrap();
        let expected = vec![
            vec!["hello world", &" ".repeat(4)].try_into().unwrap(),
            vec!["ok", &" ".repeat(4 + 9)].try_into().unwrap(),
            vec![" ".repeat(4 + 11)].try_into().unwrap(),
        ];
        assert_eq!(drawing, expected);
    }

    #[test]
    fn test_pad_top() {
        let padder = Padded {
            child: Box::new(Echo::<Msg>::new(false)),
            top: 5,
            ..Default::default()
        };
        let mut state = State::new();
        let msg = Msg(vec![
            vec!["hello world"].try_into().unwrap(),
            vec!["ok"].try_into().unwrap(),
            Line::default(),
        ]);
        state.insert(&msg);

        let drawing = padder
            .draw(&state, Dimensions::new(15, 15), DrawMode::Normal)
            .unwrap();
        let expected = vec![
            Line::default(),
            Line::default(),
            Line::default(),
            Line::default(),
            Line::default(),
            vec!["hello world"].try_into().unwrap(),
            vec!["ok"].try_into().unwrap(),
            Line::default(),
        ];

        assert_eq!(drawing, expected);
    }

    #[test]
    fn test_pad_bottom() {
        let padder = Padded {
            child: Box::new(Echo::<Msg>::new(false)),
            bottom: 5,
            ..Default::default()
        };
        let mut state = State::new();
        let msg = Msg(vec![
            vec!["hello world"].try_into().unwrap(),
            vec!["ok"].try_into().unwrap(),
            Line::default(),
        ]);
        state.insert(&msg);

        let drawing = padder
            .draw(&state, Dimensions::new(15, 15), DrawMode::Normal)
            .unwrap();
        let expected = vec![
            vec!["hello world"].try_into().unwrap(),
            vec!["ok"].try_into().unwrap(),
            Line::default(),
            Line::default(),
            Line::default(),
            Line::default(),
            Line::default(),
            Line::default(),
        ];

        assert_eq!(drawing, expected);
    }

    #[test]
    fn test_no_pad() {
        let padder = Padded {
            child: Box::new(Echo::<Msg>::new(false)),
            ..Default::default()
        };
        let mut state = State::new();
        let msg = Msg(vec![
            vec!["hello world"].try_into().unwrap(),
            vec!["ok"].try_into().unwrap(),
            Line::default(),
        ]);
        state.insert(&msg);

        let drawing = padder
            .draw(&state, Dimensions::new(15, 15), DrawMode::Normal)
            .unwrap();
        let expected = vec![
            vec!["hello world"].try_into().unwrap(),
            vec!["ok"].try_into().unwrap(),
            Line::default(),
        ];

        assert_eq!(drawing, expected);
    }

    #[test]
    fn test_truncated() {
        let padder = Padded {
            child: Box::new(Echo::<Msg>::new(false)),
            left: 5,
            right: 3,
            top: 3,
            bottom: 3,
        };
        let mut state = State::new();
        let msg = Msg(vec![
            vec!["hello world"].try_into().unwrap(),
            vec!["ok"].try_into().unwrap(),
            Line::default(),
        ]);
        state.insert(&msg);
        let drawing = padder
            .draw(&state, Dimensions::new(10, 8), DrawMode::Normal)
            .unwrap();
        let expected = vec![
            // 5 rows of padding at the top
            vec![" ".repeat(5), " ".repeat(5)].try_into().unwrap(),
            vec![" ".repeat(5), " ".repeat(5)].try_into().unwrap(),
            vec![" ".repeat(5), " ".repeat(5)].try_into().unwrap(),
            // 2 rows of content, padded on left and right
            vec![" ".repeat(5).as_ref(), "he", &" ".repeat(3)]
                .try_into()
                .unwrap(),
            vec![" ".repeat(5).as_ref(), "ok", &" ".repeat(3)]
                .try_into()
                .unwrap(),
            vec![" ".repeat(5), " ".repeat(5)].try_into().unwrap(),
            vec![" ".repeat(5), " ".repeat(5)].try_into().unwrap(),
            vec![" ".repeat(5), " ".repeat(5)].try_into().unwrap(),
        ];

        assert_eq!(drawing, expected);
    }
}