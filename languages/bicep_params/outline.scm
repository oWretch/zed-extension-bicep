; Outline queries for Bicep Parameter files
; These queries extract the main constructs for outline/symbol views

; Comments
(comment) @annotation

; Diagnostic comments
(diagnostic_comment) @annotation

; Parameters
(parameter_declaration
  "param" @context
  (identifier) @name) @item

; Variables
(variable_declaration
  "var" @context
  (identifier) @name) @item

; Type declarations
(type_declaration
  "type" @context
  (identifier) @name) @item

; Import functionality (destructuring imports)
(import_functionality
  "import" @context
  (identifier) @name) @item

; Using statements
(using_statement
  "using" @context) @item
