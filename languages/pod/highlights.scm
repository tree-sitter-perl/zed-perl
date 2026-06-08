[(pod_command)
 (command)
 (cut_command)
 (begin_command)
 (end_command)
 (for_command)] @keyword

(command_paragraph
  (command) @keyword
  (#match? @keyword "^=head")
  (content) @title)

(command_paragraph
  (command) @keyword
  (#match? @keyword "^=over")
  (content) @number)

(command_paragraph
  (command) @keyword
  (#match? @keyword "^=item")
  (content) @text)

(command_paragraph
  (command) @keyword
  (#match? @keyword "^=encoding")
  (content) @string.special)

(command_paragraph
  (command) @keyword
  (#not-match? @keyword "^=(head|over|item|encoding)")
  (content) @string)

(verbatim_paragraph (content) @embedded)

(begin_paragraph (format_name) @string.special)
(for_paragraph (format_name) @string.special)

(begin_paragraph (data) @embedded)
(for_paragraph (content) @embedded)

(interior_sequence
  (sequence_letter) @punctuation.special
  ["<" ">"] @punctuation.bracket)

(interior_sequence
  (sequence_letter) @punctuation.special
  (#eq? @punctuation.special "B")
  (content) @emphasis.strong)

(interior_sequence
  (sequence_letter) @punctuation.special
  (#eq? @punctuation.special "C")
  (content) @string.special)

(interior_sequence
  (sequence_letter) @punctuation.special
  (#eq? @punctuation.special "F")
  (content) @string.special)

(interior_sequence
  (sequence_letter) @punctuation.special
  (#eq? @punctuation.special "I")
  (content) @emphasis)

(interior_sequence
  (sequence_letter) @punctuation.special
  (#eq? @punctuation.special "L")
  (content) @link)

(interior_sequence
  (sequence_letter) @punctuation.special
  (#eq? @punctuation.special "X")
  (content) @tag)

(escape_sequence
  (sequence_letter) @punctuation.special
  ["<" ">"] @punctuation.bracket
  (content) @string.escape)