; Functions (including filter and workflow keywords)
(function_statement
  ["function" "filter" "workflow"] @context
  (function_name) @name) @item

; Classes
(class_statement
  "class" @context
  (simple_name) @name) @item

; Class methods
(class_method_definition
  (simple_name) @name) @item

; Class properties
(class_property_definition
  (variable) @name) @item

; Enums
(enum_statement
  "enum" @context
  (simple_name) @name) @item

; Enum members
(enum_member
  (simple_name) @name) @item

; Named blocks (begin, process, end, dynamicparam)
(named_block
  (block_name) @name) @item
