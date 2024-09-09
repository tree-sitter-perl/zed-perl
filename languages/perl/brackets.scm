("(" @open ")" @close)
("[" @open "]" @close)
("{" @open "}" @close)
; little annoying, but to get string quotes JUST RIGHT we gotta be specific
("'" @open (_) (replacement) "'" @close)
("'" @open (string_content) "'" @close)
(quoted_regexp "'" @open "'" @close)
