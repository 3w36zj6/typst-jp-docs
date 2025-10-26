use std::fmt::Write;

<<<<<<< HEAD
use typst::foundations::Smart;
use typst::layout::PagedDocument;
use typst::model::DocumentInfo;
use typst::World;
=======
use typst::World;
use typst::foundations::Smart;
use typst::introspection::{Location, Tag};
use typst::layout::{Frame, FrameItem, PagedDocument};
use typst::model::DocumentInfo;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

use crate::collect::Test;
use crate::world::TestWorld;

/// We don't want to panic when there is a failure.
macro_rules! test_eq {
    ($sink:expr, $lhs:expr, $rhs:expr) => {
        if $lhs != $rhs {
            writeln!(&mut $sink, "{:?} != {:?}", $lhs, $rhs).unwrap();
        }
    };
}

/// Run special checks for specific tests for which it is not worth it to create
/// custom annotations.
pub fn check(test: &Test, world: &TestWorld, doc: Option<&PagedDocument>) -> String {
    let mut sink = String::new();
    match test.name.as_str() {
        "document-set-author-date" => {
            let info = info(doc);
            test_eq!(sink, info.author, ["A", "B"]);
            test_eq!(sink, info.date, Smart::Custom(world.today(None)));
        }
        "issue-4065-document-context" => {
            let info = info(doc);
            test_eq!(sink, info.title.as_deref(), Some("Top level"));
        }
        "issue-4769-document-context-conditional" => {
            let info = info(doc);
            test_eq!(sink, info.author, ["Changed"]);
            test_eq!(sink, info.title.as_deref(), Some("Alternative"));
        }
<<<<<<< HEAD
=======
        "tags-grouping" | "tags-textual" => {
            if let Some(doc) = doc {
                if let Err(message) = check_balanced(doc) {
                    sink.push_str(message);
                }
            } else {
                sink.push_str("missing document");
            }
        }
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        _ => {}
    }
    sink
}

/// Extract the document information.
fn info(doc: Option<&PagedDocument>) -> DocumentInfo {
    doc.map(|doc| doc.info.clone()).unwrap_or_default()
}
<<<<<<< HEAD
=======

/// Naive check for whether tags are balanced in the document.
///
/// This is kept minimal for now: It does not handle groups with parents and
/// does not print useful debugging information. This is currently only run for
/// specific tests that are known not to have those. We might want to extend
/// this to the whole test suite in the future. Then we'll need to handle
/// insertions and provide a better debugging experience. However, there are
/// scenarios that are inherently (and correctly) unbalanced and we'd need some
/// way to opt out for those (via something like `large`).
fn check_balanced(doc: &PagedDocument) -> Result<(), &'static str> {
    fn visit(stack: &mut Vec<Location>, frame: &Frame) -> Result<(), &'static str> {
        for (_, item) in frame.items() {
            match item {
                FrameItem::Tag(tag) => match tag {
                    Tag::Start(..) => stack.push(tag.location()),
                    Tag::End(..) => {
                        if stack.pop() != Some(tag.location()) {
                            return Err("tags are unbalanced");
                        }
                    }
                },
                FrameItem::Group(group) => {
                    if group.parent.is_some() {
                        return Err("groups with parents are not supported");
                    }
                    visit(stack, &group.frame)?
                }
                _ => {}
            }
        }
        Ok(())
    }

    let mut stack = Vec::new();
    doc.pages.iter().try_for_each(|page| visit(&mut stack, &page.frame))
}
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
