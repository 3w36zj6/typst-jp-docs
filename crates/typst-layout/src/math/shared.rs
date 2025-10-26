<<<<<<< HEAD
use ttf_parser::math::MathValue;
use typst_library::foundations::{Style, StyleChain};
use typst_library::layout::{Abs, Em, FixedAlignment, Frame, Point, Size, VAlignment};
use typst_library::math::{EquationElem, MathSize};
use typst_utils::LazyHash;

use super::{LeftRightAlternator, MathContext, MathFragment, MathRun};

macro_rules! scaled {
    ($ctx:expr, $styles:expr, text: $text:ident, display: $display:ident $(,)?) => {
        match typst_library::math::EquationElem::size_in($styles) {
            typst_library::math::MathSize::Display => scaled!($ctx, $styles, $display),
            _ => scaled!($ctx, $styles, $text),
        }
    };
    ($ctx:expr, $styles:expr, $name:ident) => {
        $crate::math::Scaled::scaled(
            $ctx.constants.$name(),
            $ctx,
            typst_library::text::TextElem::size_in($styles),
        )
    };
}

macro_rules! percent {
    ($ctx:expr, $name:ident) => {
        $ctx.constants.$name() as f64 / 100.0
    };
}
=======
use ttf_parser::Tag;
use typst_library::foundations::{Style, StyleChain};
use typst_library::layout::{Abs, Em, FixedAlignment, Frame, Point, Size};
use typst_library::math::{EquationElem, MathSize};
use typst_library::text::{FontFamily, FontFeatures, TextElem};
use typst_utils::{LazyHash, singleton};

use super::{LeftRightAlternator, MathFragment, MathRun};
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

/// How much less high scaled delimiters can be than what they wrap.
pub const DELIM_SHORT_FALL: Em = Em::new(0.1);

<<<<<<< HEAD
/// Converts some unit to an absolute length with the current font & font size.
pub trait Scaled {
    fn scaled(self, ctx: &MathContext, font_size: Abs) -> Abs;
}

impl Scaled for i16 {
    fn scaled(self, ctx: &MathContext, font_size: Abs) -> Abs {
        ctx.font.to_em(self).at(font_size)
    }
}

impl Scaled for u16 {
    fn scaled(self, ctx: &MathContext, font_size: Abs) -> Abs {
        ctx.font.to_em(self).at(font_size)
    }
}

impl Scaled for MathValue<'_> {
    fn scaled(self, ctx: &MathContext, font_size: Abs) -> Abs {
        self.value.scaled(ctx, font_size)
    }
}

/// Styles something as cramped.
pub fn style_cramped() -> LazyHash<Style> {
    EquationElem::set_cramped(true).wrap()
=======
/// Styles something as cramped.
pub fn style_cramped() -> LazyHash<Style> {
    EquationElem::cramped.set(true).wrap()
}

/// Sets flac OpenType feature.
pub fn style_flac() -> LazyHash<Style> {
    TextElem::features
        .set(FontFeatures(vec![(Tag::from_bytes(b"flac"), 1)]))
        .wrap()
}

/// Sets dtls OpenType feature.
pub fn style_dtls() -> LazyHash<Style> {
    TextElem::features
        .set(FontFeatures(vec![(Tag::from_bytes(b"dtls"), 1)]))
        .wrap()
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

/// The style for subscripts in the current style.
pub fn style_for_subscript(styles: StyleChain) -> [LazyHash<Style>; 2] {
<<<<<<< HEAD
    [style_for_superscript(styles), EquationElem::set_cramped(true).wrap()]
=======
    [style_for_superscript(styles), EquationElem::cramped.set(true).wrap()]
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

/// The style for superscripts in the current style.
pub fn style_for_superscript(styles: StyleChain) -> LazyHash<Style> {
<<<<<<< HEAD
    EquationElem::set_size(match EquationElem::size_in(styles) {
        MathSize::Display | MathSize::Text => MathSize::Script,
        MathSize::Script | MathSize::ScriptScript => MathSize::ScriptScript,
    })
    .wrap()
=======
    EquationElem::size
        .set(match styles.get(EquationElem::size) {
            MathSize::Display | MathSize::Text => MathSize::Script,
            MathSize::Script | MathSize::ScriptScript => MathSize::ScriptScript,
        })
        .wrap()
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

/// The style for numerators in the current style.
pub fn style_for_numerator(styles: StyleChain) -> LazyHash<Style> {
<<<<<<< HEAD
    EquationElem::set_size(match EquationElem::size_in(styles) {
        MathSize::Display => MathSize::Text,
        MathSize::Text => MathSize::Script,
        MathSize::Script | MathSize::ScriptScript => MathSize::ScriptScript,
    })
    .wrap()
=======
    EquationElem::size
        .set(match styles.get(EquationElem::size) {
            MathSize::Display => MathSize::Text,
            MathSize::Text => MathSize::Script,
            MathSize::Script | MathSize::ScriptScript => MathSize::ScriptScript,
        })
        .wrap()
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

/// The style for denominators in the current style.
pub fn style_for_denominator(styles: StyleChain) -> [LazyHash<Style>; 2] {
<<<<<<< HEAD
    [style_for_numerator(styles), EquationElem::set_cramped(true).wrap()]
}

/// Styles to add font constants to the style chain.
pub fn style_for_script_scale(ctx: &MathContext) -> LazyHash<Style> {
    EquationElem::set_script_scale((
        ctx.constants.script_percent_scale_down(),
        ctx.constants.script_script_percent_scale_down(),
    ))
    .wrap()
}

/// How a delimieter should be aligned when scaling.
pub fn delimiter_alignment(delimiter: char) -> VAlignment {
    match delimiter {
        '⌜' | '⌝' => VAlignment::Top,
        '⌞' | '⌟' => VAlignment::Bottom,
        _ => VAlignment::Horizon,
    }
=======
    [style_for_numerator(styles), EquationElem::cramped.set(true).wrap()]
}

/// Resolve a prioritized iterator over the font families for math.
pub fn families(styles: StyleChain<'_>) -> impl Iterator<Item = &'_ FontFamily> + Clone {
    let fallbacks = singleton!(Vec<FontFamily>, {
        [
            "new computer modern math",
            "libertinus serif",
            "twitter color emoji",
            "noto color emoji",
            "apple color emoji",
            "segoe ui emoji",
        ]
        .into_iter()
        .map(FontFamily::new)
        .collect()
    });

    let tail = if styles.get(TextElem::fallback) { fallbacks.as_slice() } else { &[] };
    styles.get_ref(TextElem::font).into_iter().chain(tail.iter())
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

/// Stack rows on top of each other.
///
/// Add a `gap` between each row and uses the baseline of the `baseline`-th
/// row for the whole frame. `alternator` controls the left/right alternating
/// alignment behavior of `AlignPointElem` in the rows.
pub fn stack(
    rows: Vec<MathRun>,
    align: FixedAlignment,
    gap: Abs,
    baseline: usize,
    alternator: LeftRightAlternator,
<<<<<<< HEAD
    minimum_ascent_descent: Option<(Abs, Abs)>,
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
) -> Frame {
    let AlignmentResult { points, width } = alignments(&rows);
    let rows: Vec<_> = rows
        .into_iter()
        .map(|row| row.into_line_frame(&points, alternator))
        .collect();

<<<<<<< HEAD
    let padded_height = |height: Abs| {
        height.max(minimum_ascent_descent.map_or(Abs::zero(), |(a, d)| a + d))
    };

    let mut frame = Frame::soft(Size::new(
        width,
        rows.iter().map(|row| padded_height(row.height())).sum::<Abs>()
=======
    let mut frame = Frame::soft(Size::new(
        width,
        rows.iter().map(|row| row.height()).sum::<Abs>()
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            + rows.len().saturating_sub(1) as f64 * gap,
    ));

    let mut y = Abs::zero();
    for (i, row) in rows.into_iter().enumerate() {
        let x = if points.is_empty() {
            align.position(width - row.width())
        } else {
            Abs::zero()
        };
<<<<<<< HEAD
        let ascent_padded_part = minimum_ascent_descent
            .map_or(Abs::zero(), |(a, _)| (a - row.ascent()))
            .max(Abs::zero());
        let pos = Point::new(x, y + ascent_padded_part);
        if i == baseline {
            frame.set_baseline(y + row.baseline() + ascent_padded_part);
        }
        y += padded_height(row.height()) + gap;
=======
        let pos = Point::new(x, y);
        if i == baseline {
            frame.set_baseline(y + row.baseline());
        }
        y += row.height() + gap;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        frame.push_frame(pos, row);
    }

    frame
}

/// Determine the positions of the alignment points, according to the input rows combined.
pub fn alignments(rows: &[MathRun]) -> AlignmentResult {
    let mut widths = Vec::<Abs>::new();

    let mut pending_width = Abs::zero();
    for row in rows {
        let mut width = Abs::zero();
        let mut alignment_index = 0;

        for fragment in row.iter() {
            if matches!(fragment, MathFragment::Align) {
                if alignment_index < widths.len() {
                    widths[alignment_index].set_max(width);
                } else {
                    widths.push(width.max(pending_width));
                }
                width = Abs::zero();
                alignment_index += 1;
            } else {
                width += fragment.width();
            }
        }
        if widths.is_empty() {
            pending_width.set_max(width);
        } else if alignment_index < widths.len() {
            widths[alignment_index].set_max(width);
        } else {
            widths.push(width.max(pending_width));
        }
    }

    let mut points = widths;
    for i in 1..points.len() {
        let prev = points[i - 1];
        points[i] += prev;
    }
    AlignmentResult {
        width: points.last().copied().unwrap_or(pending_width),
        points,
    }
}

pub struct AlignmentResult {
    pub points: Vec<Abs>,
    pub width: Abs,
}
