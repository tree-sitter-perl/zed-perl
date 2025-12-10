(command_paragraph
  (command) @context
  (#match? @context "^=head[1-4]")
  (content) @name)

(command_paragraph
  (command) @context
  (#eq? @context "=item")
  (content) @name)