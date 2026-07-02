; Bicep outline support for Zed editor
; Comments
(comment) @annotation
(directive_statement) @annotation
(disable_next_line_directive) @annotation
(disable_diagnostics_directive) @annotation
(restore_diagnostics_directive) @annotation

; Parameter declarations
(parameter_declaration
    "param" @context
    (identifier) @name) @item

; Variable declarations
(variable_declaration
    "var" @context
    (identifier) @name) @item

; Resource declarations
(resource_declaration
    "resource" @context
    (identifier) @name) @item

; Module declarations
(module_declaration
    "module" @context
    (identifier) @name) @item

; Output declarations
(output_declaration
    "output" @context
    (identifier) @name) @item

; Type declarations
(type_declaration
    "type" @context
    (identifier) @name) @item

; User-defined functions
(user_defined_function
    "func" @context
    name: (identifier) @name) @item

; Metadata declarations
(metadata_declaration
    "metadata" @context
    (identifier) @name) @item

; Test blocks
(test_block
    "test" @context
    (identifier) @name) @item

; Assert statements
(assert_statement
    "assert" @context
    name: (identifier) @name) @item
