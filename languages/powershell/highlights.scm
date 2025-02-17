; Keywords
[
  "begin"
  "break"
  "catch"
  "class"
  ; "clean"
  "continue"
  "data"
  ; "define"
  "do"
  "dynamicparam"
  "else"
  "elseif"
  "end"
  "enum"
  "exit"
  "filter"
  "finally"
  "for"
  "foreach"
  ; "from"
  "function"
  "hidden"
  "if"
  "in"
  "param"
  "process"
  "return"
  "static"
  "switch"
  "throw"
  "trap"
  "try"
  "until"
  ; "using"
  ; "var"
  "while"
] @keyword

; Powershell Workflows
[
  "inlinescript"
  "parallel"
  "sequence"
  "workflow"
] @keyword

"-as" @operator
"-ccontains" @operator
"-ceq" @operator
"-cge" @operator
"-cgt" @operator
"-cle" @operator
"-clike" @operator
"-clt" @operator
"-cmatch" @operator
"-cne" @operator
"-cnotcontains" @operator
"-cnotlike" @operator
"-cnotmatch" @operator
"-contains" @operator
"-creplace" @operator
"-csplit" @operator
"-eq" @operator
"-ge" @operator
"-gt" @operator
"-icontains" @operator
"-ieq" @operator
"-ige" @operator
"-igt" @operator
"-ile" @operator
"-ilike" @operator
"-ilt" @operator
"-imatch" @operator
"-in" @operator
"-ine" @operator
"-inotcontains" @operator
"-inotlike" @operator
"-inotmatch" @operator
"-ireplace" @operator
"-is" @operator
"-isnot" @operator
"-isplit" @operator
"-join" @operator
"-le" @operator
"-like" @operator
"-lt" @operator
"-match" @operator
"-ne" @operator
"-notcontains" @operator
"-notin" @operator
"-notlike" @operator
"-notmatch" @operator
"-replace" @operator
"-shl" @operator
"-shr" @operator
"-split" @operator
"-and" @operator
"-or" @operator
"-xor" @operator
"-band" @operator
"-bor" @operator
"-bxor" @operator
"+" @operator
"-" @operator
"/" @operator
"\\" @operator
"%" @operator
"*" @operator
".." @operator
"-not" @operator


[
  ","
  ";"
] @punctuation.delimiter


(string_literal) @string

(integer_literal) @number
(real_literal) @number

(command
  command_name: (command_name) @function)

(function_statement
  (function_name) @function)

(invokation_expression
  (member_name) @function)

(member_access
  (member_name) @property)

(command_invokation_operator) @operator

(type_spec) @type

(variable) @variable

(comment) @comment

(array_expression) @array

(assignment_expression
  value: (pipeline) @assignvalue)
