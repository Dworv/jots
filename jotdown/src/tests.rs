use super::*;

#[test]
fn paragraphs_2newlines_end() {
    let input = "This is a paragraph.\n\nThis is another paragraph.\n\nAnother paragraph!\n\n";
    let expected = vec![
        JdElement::Paragraph("This is a paragraph."),
        JdElement::Paragraph("This is another paragraph."),
        JdElement::Paragraph("Another paragraph!"),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn paragraphs_noend() {
    let input = "This is a paragraph.\n\nThis is another paragraph.\n";
    let expected = vec![
        JdElement::Paragraph("This is a paragraph."),
        JdElement::Paragraph("This is another paragraph."),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn paragraph_unterminated() {
    let input = "This is a paragraph.\n\nThis is another paragraph.";
    let expected = vec![
        JdElement::Paragraph("This is a paragraph."),
        JdElement::Paragraph("This is another paragraph."),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn paragraph_after_newlines() {
    let input = "\n\nThis is a paragraph.\n\nThis is another paragraph.";
    let expected = vec![
        JdElement::Paragraph("This is a paragraph."),
        JdElement::Paragraph("This is another paragraph."),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn header() {
    let input = "## This is a header\nThis is a paragraph.";
    let expected = vec![
        JdElement::TitleOrHeading(("This is a header", 2)),
        JdElement::Paragraph("This is a paragraph."),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn titles() {
    let input = "# This is a title\n\n\nThis is a paragraph.\n\n# This is another title\n";
    let expected = vec![
        JdElement::TitleOrHeading(("This is a title", 1)),
        JdElement::Paragraph("This is a paragraph."),
        JdElement::TitleOrHeading(("This is another title", 1)),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn paragraph_startswith_hashtag() {
    let input = "#This is not a header";
    let expected = vec![JdElement::Paragraph("#This is not a header")];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn unordered_list() {
    let input = "- This is a list item\n- This is another list item\n";
    let expected = vec![
        JdElement::UnorderedList(vec!["This is a list item", "This is another list item"]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn paragraph_and_unordered_list() {
    let input = "This is a paragraph.\n\n- This is a list item\n- This is another list item\n";
    let expected = vec![
        JdElement::Paragraph("This is a paragraph."),
        JdElement::UnorderedList(vec!["This is a list item", "This is another list item"]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn unordered_list_and_paragraph() {
    let input = "- This is a list item\n- This is another list item\nThis is a paragraph.";
    let expected = vec![
        JdElement::UnorderedList(vec!["This is a list item", "This is another list item"]),
        JdElement::Paragraph("This is a paragraph."),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn paragraph_startswith_dash() {
    let input = "-This is not a list item";
    let expected = vec![JdElement::Paragraph("-This is not a list item")];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn bullet_in_header() {
    let input = "# This is a header with a bullet - in it\nThis is a paragraph.";
    let expected = vec![
        JdElement::TitleOrHeading(("This is a header with a bullet - in it", 1)),
        JdElement::Paragraph("This is a paragraph."),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn unordered_list_far() {
    let input = "This is a paragraph.\n\n- This is a list item\n- This is another list item\n\nThis is another paragraph.\n\n";
    let expected = vec![
        JdElement::Paragraph("This is a paragraph."),
        JdElement::UnorderedList(vec!["This is a list item", "This is another list item"]),
        JdElement::Paragraph("This is another paragraph."),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn ordered_list() {
    let input = "1. This is a list item\n2. This is another list item\n";
    let expected = vec![
        JdElement::OrderedList(vec!["This is a list item", "This is another list item"]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn two_ordered_lists() {
    let input = "1. This is a list item\n2. This is another list item\n1. This is a list item\n2. This is another list item\n";
    let expected = vec![
        JdElement::OrderedList(vec!["This is a list item", "This is another list item"]),
        JdElement::OrderedList(vec!["This is a list item", "This is another list item"]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn ordered_list_and_paragraph() {
    let input = "1. This is a list item\n2. This is another list item\n\nThis is a paragraph.\n";
    let expected = vec![
        JdElement::OrderedList(vec!["This is a list item", "This is another list item"]),
        JdElement::Paragraph("This is a paragraph."),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn ordered_and_unordered_lists() {
    let input = "1. This is a list item\n2. This is another list item\n\n- This is a list item\n- This is another list item\n";
    let expected = vec![
        JdElement::OrderedList(vec!["This is a list item", "This is another list item"]),
        JdElement::UnorderedList(vec!["This is a list item", "This is another list item"]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn very_long_gaps() {
    let input = "\n\n\n\n\n# This is a very long gap\n\n\n\n\n\n\n\n\n\nI sure hope it doesn't break my parser\n\n\n\n\n\n";
    let expected = vec![
        JdElement::TitleOrHeading(("This is a very long gap", 1)),
        JdElement::Paragraph("I sure hope it doesn't break my parser"),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn no_jotdown() {
    let input = "\n \n  \n\n\t\n\t\t\n\t\t\t";
    let expected = vec![];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn checklist() {
    let input = "- [ ] This is a list item\n- [x] This is another list item\n";
    let expected = vec![
        JdElement::Checklist(vec![
            ("This is a list item", false),
            ("This is another list item", true),
        ]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn checklist_doesnt_eat_unordered() {
    let input = "- [ ] This is a list item\n- [x] This is another list item\n- This is a list item\n- This is another list item\n";
    let expected = vec![
        JdElement::Checklist(vec![
            ("This is a list item", false),
            ("This is another list item", true),
        ]),
        JdElement::UnorderedList(vec!["This is a list item", "This is another list item"]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn unordered_doesnt_eat_checklist() {
    let input = "- This is a list item\n- This is another list item\n- [ ] This is a list item\n- [x] This is another list item\n";
    let expected = vec![
        JdElement::UnorderedList(vec!["This is a list item", "This is another list item"]),
        JdElement::Checklist(vec![
            ("This is a list item", false),
            ("This is another list item", true),
        ]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn checklist_and_paragraph() {
    let input = "- [ ] This is a list item\n- [x] This is another list item\n\nThis is a paragraph.\n";
    let expected = vec![
        JdElement::Checklist(vec![
            ("This is a list item", false),
            ("This is another list item", true),
        ]),
        JdElement::Paragraph("This is a paragraph."),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn checklist_and_ordered_list() {
    let input = "- [ ] This is a list item\n- [x] This is another list item\n\n1. This is a list item\n2. This is another list item\n";
    let expected = vec![
        JdElement::Checklist(vec![
            ("This is a list item", false),
            ("This is another list item", true),
        ]),
        JdElement::OrderedList(vec!["This is a list item", "This is another list item"]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn checklist_and_unordered_list() {
    let input = "- [ ] This is a list item\n- [x] This is another list item\n\n- This is a list item\n- This is another list item\n";
    let expected = vec![
        JdElement::Checklist(vec![
            ("This is a list item", false),
            ("This is another list item", true),
        ]),
        JdElement::UnorderedList(vec!["This is a list item", "This is another list item"]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn checklist_and_checklist() {
    let input = "- [ ] This is a list item\n- [x] This is another list item\n\n- [ ] This is a list item\n- [x] This is another list item\n";
    let expected = vec![
        JdElement::Checklist(vec![
            ("This is a list item", false),
            ("This is another list item", true),
        ]),
        JdElement::Checklist(vec![
            ("This is a list item", false),
            ("This is another list item", true),
        ]),
    ];
    let (_, output) = parse_jotdown(input).unwrap();
    assert_eq!(output, expected);
}