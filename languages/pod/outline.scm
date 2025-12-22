(command_paragraph
  (command) @context
  (#match? @context "^=head[1-4]")
  (content) @name) @item

(command_paragraph
  (command) @context
  (#eq? @context "=item")
  (content) @name) @item