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


[
  "-as"
  "-ccontains"
  "-ceq"
  "-cge"
  "-cgt"
  "-cle"
  "-clike"
  "-clt"
  "-cmatch"
  "-cne"
  "-cnotcontains"
  "-cnotlike"
  "-cnotmatch"
  "-contains"
  "-creplace"
  "-csplit"
  "-eq"
  "-ge"
  "-gt"
  "-icontains"
  "-ieq"
  "-ige"
  "-igt"
  "-ile"
  "-ilike"
  "-ilt"
  "-imatch"
  "-in"
  "-ine"
  "-inotcontains"
  "-inotlike"
  "-inotmatch"
  "-ireplace"
  "-is"
  "-isnot"
  "-isplit"
  "-join"
  "-le"
  "-like"
  "-lt"
  "-match"
  "-ne"
  "-notcontains"
  "-notin"
  "-notlike"
  "-notmatch"
  "-replace"
  "-shl"
  "-shr"
  "-split"
  "-and"
  "-or"
  "-xor"
  "-band"
  "-bor"
  "-bxor"
  "+"
  "-"
  "/"
  "\\"
  "%"
  "*"
  ".."
  "-not"
] @operator

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
