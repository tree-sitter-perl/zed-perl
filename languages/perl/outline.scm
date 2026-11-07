(package_statement
  "package" @context
  name: (package) @name
  ) @item

(class_statement
  "class" @context
  name: (package) @name
  ) @item

(phaser_statement
  phase: ["BEGIN" "CHECK" "END" "INIT" "UNITCHECK"] @name
  ) @item

(class_phaser_statement
  phase: "ADJUST" @name
  ) @item

(subroutine_declaration_statement
  "sub" @context
  name: (bareword) @name
  ) @item

(method_declaration_statement
  "method" @context
  name: (bareword) @name
  ) @item
