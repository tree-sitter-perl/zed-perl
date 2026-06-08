[(pod_command) (command) (cut_command)] @keyword

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

(interior_sequence
  (sequence_letter) @punctuation.special
  (#eq? @punctuation.special "E")
  (content) @string.escape)