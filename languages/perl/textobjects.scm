; Perl text objects for Zed.

; Subs and methods (named and anonymous) -> function
(subroutine_declaration_statement
  body: (block) @function.inside) @function.around

(method_declaration_statement
  body: (block) @function.inside) @function.around

(anonymous_subroutine_expression
  body: (block) @function.inside) @function.around

; Block-form packages and classes -> class
(package_statement
  (block) @class.inside) @class.around

(class_statement
  (block) @class.inside) @class.around

; Comments (adjacent line comments group together)
(comment)+ @comment.around
