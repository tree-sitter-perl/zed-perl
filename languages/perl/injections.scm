; an scm file for nvim-treesitter
((comment) @content
 (#set! language "comment"))
 
((pod) @content
 (#set! language "pod"))

((substitution_regexp
  (replacement) @content
  (substitution_regexp_modifiers) @_modifiers)
    ; match if there's a single `e` in the modifiers list
  (#match? @_modifiers "e")
  (#not-match? @_modifiers "e.*e")
  (#set! language "perl"))
